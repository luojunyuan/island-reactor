use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const APP_MANIFEST: &str = include_str!("../assets/app.manifest");

pub fn embed_manifest() {
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    let mux = stage_mux_runtime();
    embed_manifest_for("bins", mux.as_ref());
}

pub fn embed_example_manifest() {
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    let mux = stage_mux_runtime();
    embed_manifest_for("examples", mux.as_ref());
}

fn embed_manifest_for(target_kind: &str, mux: Option<&MuxRegistration>) {
    let manifest_asset = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("app.manifest");
    println!("cargo:rerun-if-changed={}", manifest_asset.display());

    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_path = out_dir.join("island-reactor-app.manifest");
    let manifest = match mux {
        Some(mux) => merged_manifest(mux),
        None => APP_MANIFEST.to_string(),
    };
    fs::write(&manifest_path, manifest).unwrap_or_else(|err| {
        panic!(
            "failed to write island-reactor manifest to {}: {err}",
            manifest_path.display()
        )
    });

    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_abi = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();

    match (target_env.as_str(), target_abi.as_str()) {
        ("msvc", _) => {
            println!("cargo:rustc-link-arg-{target_kind}=/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-{target_kind}=/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        ("gnu", "llvm") => {
            println!("cargo:rustc-link-arg-{target_kind}=-Wl,/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-{target_kind}=-Wl,/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        _ => panic!("unsupported target environment for island-reactor manifest embedding"),
    }
}

fn merged_manifest(mux: &MuxRegistration) -> String {
    let insertion = mux_file_entry(mux);
    if let Some(end) = APP_MANIFEST.rfind("</assembly>") {
        let mut out = String::with_capacity(APP_MANIFEST.len() + insertion.len());
        out.push_str(&APP_MANIFEST[..end]);
        out.push_str(&insertion);
        out.push_str(&APP_MANIFEST[end..]);
        out
    } else {
        APP_MANIFEST.to_string()
    }
}

fn mux_file_entry(mux: &MuxRegistration) -> String {
    let mut out = String::new();
    out.push_str("    <file name=\"");
    out.push_str(&escape_manifest_attr(mux.path));
    out.push_str("\">\n");
    for class in mux.classes {
        out.push_str(
            "        <activatableClass xmlns=\"urn:schemas-microsoft-com:winrt.v1\" name=\"",
        );
        out.push_str(&escape_manifest_attr(class.name));
        out.push_str("\" threadingModel=\"");
        out.push_str(&escape_manifest_attr(class.threading_model));
        out.push_str("\"/>\n");
    }
    out.push_str("    </file>\n");
    out
}

struct MuxRegistration {
    path: &'static str,
    classes: &'static [muxc_bindings::ActivatableClass],
}

fn stage_mux_runtime() -> Option<MuxRegistration> {
    let Ok(out_dir) = env::var("OUT_DIR").map(PathBuf::from) else {
        return None;
    };
    let Some(arch) = target_arch_folder() else {
        println!("cargo:warning=WinUI 2 runtime staging skipped: unsupported target architecture");
        return None;
    };
    let Some(asset_dir) = muxc_bindings::runtime_asset_dir(arch) else {
        println!("cargo:warning=WinUI 2 runtime staging skipped: unsupported target architecture");
        return None;
    };
    let dll = asset_dir.join(muxc_bindings::RUNTIME_DLL);
    let pri = asset_dir.join(muxc_bindings::RUNTIME_PRI);
    println!("cargo:rerun-if-changed={}", dll.display());
    println!("cargo:rerun-if-changed={}", pri.display());
    if !dll.exists() || !pri.exists() {
        println!(
            "cargo:warning=WinUI 2 runtime staging skipped: missing vendored assets in {}",
            asset_dir.display()
        );
        return None;
    }

    if let Some(target_dirs) = target_output_dirs(&out_dir) {
        for target_dir in target_dirs {
            if let Err(err) = copy_runtime_payload(&asset_dir, &target_dir) {
                println!(
                    "cargo:warning=WinUI 2 runtime payload copy failed for {}: {err}",
                    target_dir.display()
                );
            }
        }
    } else {
        println!("cargo:warning=WinUI 2 runtime staging skipped: could not resolve target dir");
    }

    Some(MuxRegistration {
        path: muxc_bindings::RUNTIME_DLL,
        classes: muxc_bindings::ACTIVATABLE_CLASSES,
    })
}

fn target_arch_folder() -> Option<&'static str> {
    match env::var("CARGO_CFG_TARGET_ARCH").ok()?.as_str() {
        "x86_64" => Some("x64"),
        "aarch64" => Some("arm64"),
        _ => None,
    }
}

fn copy_runtime_payload(asset_dir: &Path, target_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(target_dir)?;
    let dll = asset_dir.join(muxc_bindings::RUNTIME_DLL);
    let pri = asset_dir.join(muxc_bindings::RUNTIME_PRI);
    copy_file(&dll, target_dir.join(muxc_bindings::RUNTIME_DLL))?;
    let staged_pri = target_dir.join(muxc_bindings::RUNTIME_PRI);
    copy_file(&pri, &staged_pri)?;
    ensure_app_resources_pri(target_dir, &staged_pri)
}

fn ensure_app_resources_pri(target_dir: &Path, mux_pri: &Path) -> std::io::Result<()> {
    let resources_pri = target_dir.join("resources.pri");

    let Some(makepri) = find_makepri() else {
        println!("cargo:warning=WinUI 2 app PRI generation skipped: could not find makepri.exe");
        return Ok(());
    };

    let work = target_dir.join("island-reactor-pri");
    let input = work.join("input");
    recreate_dir(&work)?;
    fs::create_dir_all(&input)?;
    copy_file(mux_pri, input.join(muxc_bindings::RUNTIME_PRI))?;

    let config = work.join("priconfig.xml");
    fs::write(&config, APP_PRI_CONFIG)?;

    let output = work.join("resources.pri");
    let status = Command::new(&makepri)
        .current_dir(&work)
        .arg("new")
        .arg("/pr")
        .arg(&input)
        .arg("/of")
        .arg(&output)
        .arg("/cf")
        .arg(&config)
        .arg("/o")
        .status()?;
    if !status.success() {
        println!(
            "cargo:warning=WinUI 2 app PRI generation failed with {status}: {}",
            makepri.display()
        );
        return Ok(());
    }

    copy_file(output, resources_pri)?;
    let _ = fs::remove_dir_all(&work);
    Ok(())
}

const APP_PRI_CONFIG: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<resources targetOsVersion="10.0.0" majorVersion="1">
  <packaging>
    <autoResourcePackage qualifier="Scale" />
    <autoResourcePackage qualifier="DXFeatureLevel" />
  </packaging>
  <index root="\" startIndexAt="\">
    <default>
      <qualifier name="Language" value="en-US" />
      <qualifier name="Contrast" value="standard" />
      <qualifier name="Scale" value="200" />
      <qualifier name="HomeRegion" value="001" />
      <qualifier name="TargetSize" value="256" />
      <qualifier name="LayoutDirection" value="LTR" />
      <qualifier name="DXFeatureLevel" value="DX9" />
      <qualifier name="Configuration" value="" />
      <qualifier name="AlternateForm" value="" />
      <qualifier name="Platform" value="UAP" />
    </default>
    <indexer-config type="folder" foldernameAsQualifier="true" filenameAsQualifier="true" qualifierDelimiter="." />
    <indexer-config type="resw" convertDotsToSlashes="true" initialPath="" />
    <indexer-config type="resjson" initialPath="" />
    <indexer-config type="PRI" />
  </index>
</resources>
"#;

fn recreate_dir(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)
}

fn find_makepri() -> Option<PathBuf> {
    if let Ok(path) = env::var("MAKEPRI_EXE") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    let user_profile = env::var("USERPROFILE").ok()?;
    let buildtools = PathBuf::from(&user_profile)
        .join(".nuget")
        .join("packages")
        .join("microsoft.windows.sdk.buildtools");
    if let Some(path) = find_makepri_under(&buildtools) {
        return Some(path);
    }

    let kits = env::var("ProgramFiles(x86)")
        .ok()
        .map(PathBuf::from)?
        .join("Windows Kits")
        .join("10")
        .join("bin");
    find_makepri_under(&kits)
}

fn find_makepri_under(root: &Path) -> Option<PathBuf> {
    if !root.exists() {
        return None;
    }
    let mut candidates = Vec::new();
    collect_makepri(root, &mut candidates).ok()?;
    candidates.sort();
    candidates.pop()
}

fn collect_makepri(root: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_makepri(&path, out)?;
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("makepri.exe"))
            && path
                .parent()
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.eq_ignore_ascii_case("x64"))
        {
            out.push(path);
        }
    }
    Ok(())
}

fn copy_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, dst)?;
    Ok(())
}

fn target_output_dirs(out_dir: &Path) -> Option<Vec<PathBuf>> {
    let mut path = out_dir;
    while let Some(parent) = path.parent() {
        if path.file_name().and_then(|v| v.to_str()) == Some("build") {
            let profile_dir = parent.to_path_buf();
            return Some(vec![profile_dir.clone(), profile_dir.join("examples")]);
        }
        path = parent;
    }
    None
}

fn escape_manifest_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
