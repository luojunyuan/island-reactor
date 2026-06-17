use std::{env, fs, path::PathBuf};

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    // This crate is a library and does not need build-time setup for downstream
    // apps. The build script exists so Cargo's generated Windows test harness
    // (`islands_reactor_setup-*.exe`) carries an explicit asInvoker manifest;
    // otherwise Windows may treat the word "setup" as an installer and require
    // elevation when running `cargo test`.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest = out_dir.join("islands-reactor-setup-tests.manifest");
    fs::write(&manifest, TEST_MANIFEST).unwrap_or_else(|err| {
        panic!(
            "failed to write islands-reactor-setup test manifest to {}: {err}",
            manifest.display()
        )
    });

    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_abi = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();
    match (target_env.as_str(), target_abi.as_str()) {
        ("msvc", _) => {
            println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
            println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
        }
        ("gnu", "llvm") => {
            println!("cargo:rustc-link-arg=-Wl,/MANIFEST:EMBED");
            println!(
                "cargo:rustc-link-arg=-Wl,/MANIFESTINPUT:{}",
                manifest.display()
            );
        }
        _ => panic!("unsupported target environment for islands-reactor-setup test manifest"),
    }
}

const TEST_MANIFEST: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <assemblyIdentity name="IslandReactor.SetupTests" type="win32" version="0.1.0.0"/>
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
            </requestedPrivileges>
        </security>
    </trustInfo>
</assembly>
"#;
