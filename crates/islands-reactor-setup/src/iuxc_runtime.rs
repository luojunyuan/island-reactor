use std::path::{Path, PathBuf};

pub const ISLANDS_UI_XAML_VERSION: &str = "0.1.0-local";
pub const CONTROLS_DLL: &str = "Islands.UI.Xaml.Controls.dll";
pub const CONTROLS_PRI: &str = "Islands.UI.Xaml.Controls.pri";
pub const CONTROLS_WINMD: &str = "Islands.UI.Xaml.Controls.winmd";
pub const AUTOMATION_WINMD: &str = "Islands.UI.Xaml.Automation.winmd";
pub const RUNTIME_FILES: &[&str] = &[CONTROLS_DLL, CONTROLS_PRI, CONTROLS_WINMD, AUTOMATION_WINMD];

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
