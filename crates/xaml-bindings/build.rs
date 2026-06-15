fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let bindings = std::path::Path::new(&out_dir).join("xaml_bindings.rs");
    let bindings = bindings.display().to_string();

    let _warnings = windows_bindgen::bindgen([
        "--out",
        &bindings,
        "--flat",
        "--filter",
        "Windows.UI.Xaml",
        "Windows.UI.Input",
        "Windows.UI.Text",
        "Windows.Win32.System.WinRT.Xaml.IDesktopWindowXamlSourceNative",
        "--reference",
        "windows,skip-root,Windows.ApplicationModel.Core",
        "--reference",
        "windows,skip-root,Windows.Foundation.Collections.IPropertySet",
        "--reference",
        "windows,skip-root,Windows.Foundation.Collections.PropertySet",
        "--reference",
        "windows,skip-root,Windows.Foundation.Collections.ValueSet",
        "--reference",
        "windows_collections,flat,Windows.Foundation.Collections",
        "--reference",
        "windows,skip-root,Windows.Foundation.Numerics.Quaternion",
        "--reference",
        "windows_numerics,flat,Windows.Foundation.Numerics",
        "--reference",
        "windows,skip-root,Windows.Foundation",
        "--reference",
        "windows,skip-root,Windows.Win32.Foundation",
        "--reference",
        "windows,skip-root,Windows.Win32.UI.WindowsAndMessaging",
    ]);
}
