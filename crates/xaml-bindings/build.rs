fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let bindings = std::path::Path::new(&out_dir).join("xaml_bindings.rs");
    let bindings = bindings.display().to_string();

    let winui2_winmd = winui2_setup::nuget_winmd();
    if let Some(path) = &winui2_winmd {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let mut args = vec!["--in".to_string(), "default".to_string()];
    if let Some(path) = winui2_winmd {
        args.push(path.display().to_string());
    }
    args.extend(
        [
            "--out",
            &bindings,
            "--flat",
            "--filter",
            "Windows.ApplicationModel.Resources.Core.ResourceManager",
            "Windows.Storage.IStorageFile",
            "Windows.Storage.StorageFile",
            "Windows.UI.Xaml",
            "Windows.UI.Input",
            "Windows.UI.Text",
            "Windows.Win32.System.WinRT.Xaml.IDesktopWindowXamlSourceNative",
            "Microsoft.UI.Xaml.Controls.XamlControlsResources",
            "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider",
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
        ]
        .into_iter()
        .map(str::to_string),
    );

    let _warnings = windows_bindgen::bindgen(args);
}
