use std::path::{Path, PathBuf};

pub const WINUI2_VERSION: &str = "2.8.7";
pub const RUNTIME_DLL: &str = "Microsoft.UI.Xaml.dll";
pub const RUNTIME_PRI: &str = "Microsoft.UI.Xaml.pri";

pub fn runtime_asset_dir(arch: &str) -> Option<PathBuf> {
    match arch {
        "x64" | "arm64" => Some(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("runtime")
                .join(arch),
        ),
        _ => None,
    }
}
