use std::{env, fs, path::PathBuf};

const APP_MANIFEST: &str = include_str!("../assets/app.manifest");

pub fn embed_manifest() {
    embed_manifest_for("bins", None);
}

pub fn embed_example_manifest() {
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    let mux = winui2_setup::stage_example_runtime();
    embed_manifest_for("examples", mux.as_ref());
}

fn embed_manifest_for(target_kind: &str, mux: Option<&winui2_setup::MuxRegistration>) {
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

fn merged_manifest(mux: &winui2_setup::MuxRegistration) -> String {
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

fn mux_file_entry(mux: &winui2_setup::MuxRegistration) -> String {
    let mut out = String::new();
    out.push_str("    <file name=\"");
    out.push_str(&winui2_setup::escape_manifest_attr(&mux.path));
    out.push_str("\">\n");
    for class in &mux.classes {
        out.push_str(
            "        <activatableClass xmlns=\"urn:schemas-microsoft-com:winrt.v1\" name=\"",
        );
        out.push_str(&winui2_setup::escape_manifest_attr(&class.name));
        out.push_str("\" threadingModel=\"");
        out.push_str(&winui2_setup::escape_manifest_attr(&class.threading_model));
        out.push_str("\"/>\n");
    }
    out.push_str("    </file>\n");
    out
}
