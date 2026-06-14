fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let bindings = std::path::Path::new(&out_dir).join("xaml_bindings.rs");
    let bindings = bindings.display().to_string();

    let _warnings = windows_bindgen::bindgen([
        "--out",
        &bindings,
        "--no-allow",
        "--filter",
        "Windows.UI.Xaml.DependencyObject",
        "Windows.UI.Xaml.FrameworkElement",
        "Windows.UI.Xaml.HorizontalAlignment",
        "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource",
        "Windows.UI.Xaml.Hosting.WindowsXamlManager",
        "Windows.UI.Xaml.TextAlignment",
        "Windows.UI.Xaml.Controls.TextBlock",
        "Windows.UI.Xaml.UIElement",
        "Windows.Win32.System.WinRT.Xaml.IDesktopWindowXamlSourceNative",
        "--reference",
        "windows,skip-root,Windows.Foundation",
        "--reference",
        "windows,skip-root,Windows.Foundation.Collections",
        "--reference",
        "windows,skip-root,Windows.Foundation.Numerics",
        "--reference",
        "windows,skip-root,Windows.Win32.Foundation",
        "--reference",
        "windows,skip-root,Windows.Win32.UI.WindowsAndMessaging",
    ]);
}
