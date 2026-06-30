use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[allow(dead_code)]
mod iuxc_runtime;
#[allow(dead_code)]
mod muxc_runtime;

const APP_MANIFEST: &str = include_str!("../assets/app.manifest");

pub fn embed_manifest() {
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    let manifest_asset = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("app.manifest");
    println!("cargo:rerun-if-changed={}", manifest_asset.display());

    stage_runtime();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_path = out_dir.join("islands-reactor-app.manifest");
    fs::write(&manifest_path, APP_MANIFEST).unwrap_or_else(|err| {
        panic!(
            "failed to write islands-reactor manifest to {}: {err}",
            manifest_path.display()
        )
    });

    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_abi = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();

    match (target_env.as_str(), target_abi.as_str()) {
        ("msvc", _) => {
            println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        ("gnu", "llvm") => {
            println!("cargo:rustc-link-arg-bins=-Wl,/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg-bins=-Wl,/MANIFESTINPUT:{}",
                manifest_path.display()
            );
        }
        _ => panic!("unsupported target environment for islands-reactor manifest embedding"),
    }
}

fn stage_runtime() {
    let Ok(out_dir) = env::var("OUT_DIR").map(PathBuf::from) else {
        return;
    };
    let Some(arch) = target_arch_folder() else {
        println!("cargo:warning=WinUI 2 runtime staging skipped: unsupported target architecture");
        return;
    };
    let Some(asset_dir) = muxc_runtime::runtime_asset_dir(arch) else {
        println!("cargo:warning=WinUI 2 runtime staging skipped: unsupported target architecture");
        return;
    };
    let dll = asset_dir.join(muxc_runtime::RUNTIME_DLL);
    let pri = asset_dir.join(muxc_runtime::RUNTIME_PRI);
    println!("cargo:rerun-if-changed={}", dll.display());
    println!("cargo:rerun-if-changed={}", pri.display());
    if !dll.exists() || !pri.exists() {
        println!(
            "cargo:warning=WinUI 2 runtime staging skipped: missing vendored assets in {}",
            asset_dir.display()
        );
        return;
    }

    let iuxc_asset_dir = iuxc_runtime::runtime_asset_dir(arch);
    if let Some(iuxc_asset_dir) = iuxc_asset_dir.as_ref() {
        for file in [iuxc_runtime::CONTROLS_DLL, iuxc_runtime::CONTROLS_PRI] {
            println!(
                "cargo:rerun-if-changed={}",
                iuxc_asset_dir.join(file).display()
            );
        }
    }

    if let Some(target_dir) = target_output_dir(&out_dir) {
        if let Err(err) = copy_runtime_payload(&asset_dir, iuxc_asset_dir.as_deref(), &target_dir) {
            println!(
                "cargo:warning=XAML runtime payload copy failed for {}: {err}",
                target_dir.display()
            );
        }
    } else {
        println!("cargo:warning=XAML runtime staging skipped: could not resolve target dir");
    }
}

fn target_arch_folder() -> Option<&'static str> {
    match env::var("CARGO_CFG_TARGET_ARCH").ok()?.as_str() {
        "x86_64" => Some("x64"),
        "aarch64" => Some("arm64"),
        _ => None,
    }
}

fn copy_runtime_payload(
    muxc_asset_dir: &Path,
    iuxc_asset_dir: Option<&Path>,
    target_dir: &Path,
) -> std::io::Result<()> {
    fs::create_dir_all(target_dir)?;
    let _stage_lock = RuntimeStageLock::acquire(target_dir)?;
    let dll = muxc_asset_dir.join(muxc_runtime::RUNTIME_DLL);
    let pri = muxc_asset_dir.join(muxc_runtime::RUNTIME_PRI);
    copy_file_if_missing_or_stale(&dll, target_dir.join(muxc_runtime::RUNTIME_DLL))?;

    let iuxc_pri = if let Some(iuxc_asset_dir) = iuxc_asset_dir {
        let iuxc_dll = iuxc_asset_dir.join(iuxc_runtime::CONTROLS_DLL);
        if iuxc_dll.exists() {
            copy_file_if_missing_or_stale(&iuxc_dll, target_dir.join(iuxc_runtime::CONTROLS_DLL))?;
        } else {
            println!(
                "cargo:warning=Islands.UI.Xaml runtime asset missing: {}",
                iuxc_dll.display()
            );
        }

        let iuxc_pri = iuxc_asset_dir.join(iuxc_runtime::CONTROLS_PRI);
        if iuxc_pri.exists() {
            Some(iuxc_pri)
        } else {
            println!(
                "cargo:warning=Islands.UI.Xaml runtime asset missing: {}",
                iuxc_pri.display()
            );
            None
        }
    } else {
        None
    };

    if target_dir.join(muxc_runtime::RUNTIME_PRI).exists() {
        fs::remove_file(target_dir.join(muxc_runtime::RUNTIME_PRI))?;
    }
    ensure_app_resources_pri(target_dir, &pri, iuxc_pri.as_deref())
}

fn ensure_app_resources_pri(
    target_dir: &Path,
    mux_pri: &Path,
    iuxc_pri: Option<&Path>,
) -> std::io::Result<()> {
    let resources_pri = target_dir.join("resources.pri");
    if app_resources_pri_is_current(&resources_pri, mux_pri, iuxc_pri)? {
        return Ok(());
    }

    let Some(makepri) = find_makepri() else {
        println!("cargo:warning=XAML app PRI generation skipped: could not find makepri.exe");
        copy_file(mux_pri, target_dir.join(muxc_runtime::RUNTIME_PRI))?;
        if let Some(iuxc_pri) = iuxc_pri {
            copy_file(iuxc_pri, target_dir.join(iuxc_runtime::CONTROLS_PRI))?;
        }
        return Ok(());
    };

    let work = target_dir.join("islands-reactor-pri");
    let input = work.join("input");
    recreate_dir(&work)?;
    fs::create_dir_all(&input)?;

    let config = work.join("priconfig.xml");
    fs::write(&config, APP_PRI_CONFIG)?;

    let output = work.join("resources.pri");
    copy_file(mux_pri, input.join(muxc_runtime::RUNTIME_PRI))?;
    if let Some(iuxc_pri) = iuxc_pri {
        copy_file(iuxc_pri, input.join(iuxc_runtime::CONTROLS_PRI))?;
    }

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
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        println!(
            "cargo:warning=XAML app PRI generation failed with {status}: {}",
            makepri.display()
        );
        return Ok(());
    }

    copy_file(output, resources_pri)?;
    let _ = fs::remove_dir_all(&work);
    Ok(())
}

struct RuntimeStageLock {
    path: PathBuf,
}

impl RuntimeStageLock {
    fn acquire(target_dir: &Path) -> io::Result<Self> {
        let path = target_dir.join(".islands-reactor-runtime.lock");
        let started = Instant::now();
        loop {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(mut file) => {
                    use io::Write as _;
                    let _ = writeln!(file, "{}", process::id());
                    return Ok(Self { path });
                }
                Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                    if started.elapsed() > Duration::from_secs(120) {
                        return Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            format!("timed out waiting for {}", path.display()),
                        ));
                    }
                    thread::sleep(Duration::from_millis(50));
                }
                Err(err) => return Err(err),
            }
        }
    }
}

impl Drop for RuntimeStageLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn app_resources_pri_is_current(
    resources_pri: &Path,
    mux_pri: &Path,
    iuxc_pri: Option<&Path>,
) -> io::Result<bool> {
    let resources = match fs::metadata(resources_pri) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err),
    };
    if resources.len() == 0 {
        return Ok(false);
    }

    let Ok(resources_modified) = resources.modified() else {
        return Ok(false);
    };
    let setup_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("lib.rs");
    let mut inputs = vec![mux_pri, setup_src.as_path()];
    if let Some(iuxc_pri) = iuxc_pri {
        inputs.push(iuxc_pri);
    }
    for input in inputs {
        let Ok(input_modified) = fs::metadata(input).and_then(|metadata| metadata.modified())
        else {
            return Ok(false);
        };
        if resources_modified < input_modified {
            return Ok(false);
        }
    }
    Ok(true)
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

fn copy_file_if_missing_or_stale(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    if let (Ok(src_metadata), Ok(dst_metadata)) = (fs::metadata(src), fs::metadata(dst)) {
        let same_len = src_metadata.len() == dst_metadata.len();
        let dst_is_fresh = match (src_metadata.modified(), dst_metadata.modified()) {
            (Ok(src_modified), Ok(dst_modified)) => dst_modified >= src_modified,
            _ => false,
        };
        if same_len && dst_is_fresh {
            return Ok(());
        }
    }
    copy_file(src, dst)
}

fn target_output_dir(out_dir: &Path) -> Option<PathBuf> {
    let mut path = out_dir;
    while let Some(parent) = path.parent() {
        if path.file_name().and_then(|v| v.to_str()) == Some("build") {
            return Some(parent.to_path_buf());
        }
        path = parent;
    }
    None
}
