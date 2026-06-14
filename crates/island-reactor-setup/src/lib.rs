use std::{env, fs, path::PathBuf};

const APP_MANIFEST: &str = include_str!("../assets/app.manifest");

pub fn embed_manifest() {
    embed_manifest_for("bins");
}

pub fn embed_example_manifest() {
    embed_manifest_for("examples");
}

fn embed_manifest_for(target_kind: &str) {
    let manifest_asset = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("app.manifest");
    println!("cargo:rerun-if-changed={}", manifest_asset.display());

    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let manifest_path = out_dir.join("island-reactor-app.manifest");
    fs::write(&manifest_path, APP_MANIFEST).unwrap_or_else(|err| {
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
