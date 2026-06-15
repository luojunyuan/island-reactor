use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

pub const WINUI2_VERSION: &str = "2.8.7";

pub fn nuget_winmd() -> Option<PathBuf> {
    let package = package_dir()?;
    let winmd = package
        .join("lib")
        .join("uap10.0")
        .join("Microsoft.UI.Xaml.winmd");
    winmd.exists().then_some(winmd)
}

pub fn stage_example_runtime() -> Option<MuxRegistration> {
    let Ok(out_dir) = env::var("OUT_DIR").map(PathBuf::from) else {
        return None;
    };
    let Some(package) = package_dir() else {
        println!(
            "cargo:warning=WinUI 2 runtime staging skipped: NuGet package microsoft.ui.xaml/{WINUI2_VERSION} was not found"
        );
        return None;
    };
    let Some(arch) = target_arch_folder() else {
        println!("cargo:warning=WinUI 2 runtime staging skipped: unsupported target architecture");
        return None;
    };
    let appx = package
        .join("tools")
        .join("AppX")
        .join(arch)
        .join("Release")
        .join("Microsoft.UI.Xaml.2.8.appx");
    println!("cargo:rerun-if-changed={}", appx.display());
    if !appx.exists() {
        println!(
            "cargo:warning=WinUI 2 runtime staging skipped: missing {}",
            appx.display()
        );
        return None;
    }

    let stage = out_dir.join("winui2-stage");
    if let Err(err) = recreate_dir(&stage).and_then(|_| expand_appx(&appx, &stage)) {
        println!("cargo:warning=WinUI 2 AppX expansion failed: {err}");
        return None;
    }

    let manifest = match fs::read_to_string(stage.join("AppxManifest.xml")) {
        Ok(value) => value,
        Err(err) => {
            println!("cargo:warning=WinUI 2 manifest read failed: {err}");
            return None;
        }
    };
    let registration = mux_registration_from_manifest(&manifest);
    if registration.classes.is_empty() {
        println!("cargo:warning=WinUI 2 manifest has no activatable classes");
        return None;
    }

    if let Some(target_dirs) = target_output_dirs(&out_dir) {
        for target_dir in target_dirs {
            if let Err(err) = copy_runtime_payload(&stage, &target_dir, &registration.path) {
                println!(
                    "cargo:warning=WinUI 2 runtime payload copy failed for {}: {err}",
                    target_dir.display()
                );
            }
        }
    } else {
        println!("cargo:warning=WinUI 2 runtime staging skipped: could not resolve target dir");
    }

    Some(registration)
}

pub struct MuxRegistration {
    pub path: String,
    pub classes: Vec<ActivatableClass>,
}

pub struct ActivatableClass {
    pub name: String,
    pub threading_model: String,
}

fn package_dir() -> Option<PathBuf> {
    if let Ok(root) = env::var("WINUI2_NUGET_DIR") {
        let root = PathBuf::from(root);
        if root.exists() {
            return Some(root);
        }
    }

    let package = nuget_root()?.join("microsoft.ui.xaml").join(WINUI2_VERSION);
    package.exists().then_some(package)
}

fn nuget_root() -> Option<PathBuf> {
    if let Ok(root) = env::var("NUGET_PACKAGES") {
        return Some(PathBuf::from(root));
    }
    let user_profile = env::var("USERPROFILE").ok()?;
    Some(PathBuf::from(user_profile).join(".nuget").join("packages"))
}

fn target_arch_folder() -> Option<&'static str> {
    match env::var("CARGO_CFG_TARGET_ARCH").ok()?.as_str() {
        "x86" => Some("x86"),
        "x86_64" => Some("x64"),
        "aarch64" => Some("arm64"),
        "arm" => Some("arm"),
        _ => None,
    }
}

fn recreate_dir(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)
}

fn expand_appx(appx: &Path, stage: &Path) -> std::io::Result<()> {
    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(format!(
            "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force",
            ps_escape(appx),
            ps_escape(stage)
        ))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "Expand-Archive exited with {status}"
        )))
    }
}

fn copy_runtime_payload(stage: &Path, target_dir: &Path, dll_name: &str) -> std::io::Result<()> {
    fs::create_dir_all(target_dir)?;
    copy_file(stage.join(dll_name), target_dir.join(dll_name))?;
    copy_file(
        stage.join("Microsoft.UI.Xaml.winmd"),
        target_dir.join("Microsoft.UI.Xaml.winmd"),
    )?;
    let mux_pri = target_dir.join("Microsoft.UI.Xaml.pri");
    copy_file(stage.join("resources.pri"), &mux_pri)?;
    ensure_app_resources_pri(target_dir, &mux_pri)?;

    let assets = stage.join("Microsoft.UI.Xaml").join("Assets");
    if assets.exists() {
        copy_dir_recursive(
            &assets,
            &target_dir.join("Microsoft.UI.Xaml").join("Assets"),
        )?;
    }

    Ok(())
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
    copy_file(mux_pri, input.join("Microsoft.UI.Xaml.pri"))?;

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

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            copy_file(src_path, dst_path)?;
        }
    }
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

fn mux_registration_from_manifest(manifest: &str) -> MuxRegistration {
    let path = tag_body(manifest, "Path").unwrap_or_else(|| "Microsoft.UI.Xaml.dll".to_string());
    let classes = manifest
        .match_indices("<ActivatableClass")
        .filter_map(|(start, _)| {
            let end = manifest[start..].find('>')?;
            let tag = &manifest[start..start + end + 1];
            let name = attr_value(tag, "ActivatableClassId")?;
            let threading_model =
                attr_value(tag, "ThreadingModel").unwrap_or_else(|| "both".to_string());
            Some(ActivatableClass {
                name,
                threading_model: threading_model.to_ascii_lowercase(),
            })
        })
        .collect();

    MuxRegistration { path, classes }
}

fn tag_body(input: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = input.find(&open)? + open.len();
    let end = input[start..].find(&close)? + start;
    Some(input[start..end].trim().to_string())
}

fn attr_value(tag: &str, name: &str) -> Option<String> {
    let pattern = format!("{name}=\"");
    let start = tag.find(&pattern)? + pattern.len();
    let end = tag[start..].find('"')? + start;
    Some(xml_escape_attr(&tag[start..end]))
}

fn xml_escape_attr(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

pub fn escape_manifest_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn ps_escape(path: &Path) -> String {
    path.display().to_string().replace('\'', "''")
}
