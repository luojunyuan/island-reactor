use std::{
    env, fs,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use xmltree::{Element, EmitterConfig, XMLNode};

const WINUI2_VERSION: &str = "2.8.7";

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("generate-bindings") if args.next().is_none() => {
            if let Err(err) = generate_bindings() {
                eprintln!("island-reactor-codegen: {err}");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: cargo run -p island-reactor-codegen -- generate-bindings");
            std::process::exit(2);
        }
    }
}

fn generate_bindings() -> Result<(), String> {
    let root = workspace_root()?;
    let package = winui2_package_dir()?;
    let winmd = extract_muxc_metadata_winmd(&root, &package)?;

    generate_xaml_bindings(&root)?;
    generate_muxc_bindings(&root, &winmd)?;
    vendor_muxc_runtime(&root, &package)?;
    Ok(())
}

fn generate_xaml_bindings(root: &Path) -> Result<(), String> {
    let out = root.join("crates/xaml-bindings/src/bindings.rs");
    let temp = root
        .join("target")
        .join("island-reactor-codegen")
        .join("xaml-bindings.rs");
    let temp_arg = path_arg(&temp);
    let args = vec![
        "--in",
        "default",
        "--out",
        &temp_arg,
        "--flat",
        "--filter",
        "Windows.ApplicationModel.Resources.Core.ResourceManager",
        "Windows.Storage.IStorageFile",
        "Windows.Storage.StorageFile",
        "Windows.UI.Composition.ICompositionAnimationBase",
        "Windows.UI.Core.CoreDispatcher",
        "Windows.UI.Xaml",
        "Windows.UI.Composition.AnimationPropertyInfo",
        "Windows.UI.UIContext",
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
    ];
    run_bindgen(args)?;
    copy_if_changed(&temp, &out)
}

fn generate_muxc_bindings(root: &Path, winmd: &Path) -> Result<(), String> {
    let out = root.join("crates/muxc-bindings/src/bindings.rs");
    let temp = root
        .join("target")
        .join("island-reactor-codegen")
        .join("muxc-bindings.rs");
    let temp_arg = path_arg(&temp);
    let winmd = path_arg(winmd);
    let args = vec![
        "--in",
        "default",
        &winmd,
        "--out",
        &temp_arg,
        "--flat",
        "--filter",
        "Microsoft.UI.Xaml.Controls.BreadcrumbBar",
        "Microsoft.UI.Xaml.Controls.BreadcrumbBarItem",
        "Microsoft.UI.Xaml.Controls.BreadcrumbBarItemClickedEventArgs",
        "Microsoft.UI.Xaml.Controls.ColorChangedEventArgs",
        "Microsoft.UI.Xaml.Controls.ColorPicker",
        "Microsoft.UI.Xaml.Controls.ColorPickerHsvChannel",
        "Microsoft.UI.Xaml.Controls.ColorSpectrumComponents",
        "Microsoft.UI.Xaml.Controls.ColorSpectrumShape",
        "Microsoft.UI.Xaml.Controls.CommandBarFlyout",
        "Microsoft.UI.Xaml.Controls.DropDownButton",
        "Microsoft.UI.Xaml.Controls.ExpandDirection",
        "Microsoft.UI.Xaml.Controls.Expander",
        "Microsoft.UI.Xaml.Controls.ExpanderCollapsedEventArgs",
        "Microsoft.UI.Xaml.Controls.ExpanderExpandingEventArgs",
        "Microsoft.UI.Xaml.Controls.ExpanderTemplateSettings",
        "Microsoft.UI.Xaml.Controls.InfoBadge",
        "Microsoft.UI.Xaml.Controls.InfoBadgeTemplateSettings",
        "Microsoft.UI.Xaml.Controls.InfoBar",
        "Microsoft.UI.Xaml.Controls.InfoBarClosedEventArgs",
        "Microsoft.UI.Xaml.Controls.InfoBarCloseReason",
        "Microsoft.UI.Xaml.Controls.InfoBarClosingEventArgs",
        "Microsoft.UI.Xaml.Controls.InfoBarSeverity",
        "Microsoft.UI.Xaml.Controls.InfoBarTemplateSettings",
        "Microsoft.UI.Xaml.Controls.MenuBar",
        "Microsoft.UI.Xaml.Controls.MenuBarItem",
        "Microsoft.UI.Xaml.Controls.MenuBarItemFlyout",
        "Microsoft.UI.Xaml.Controls.NavigationView",
        "Microsoft.UI.Xaml.Controls.NavigationViewBackButtonVisible",
        "Microsoft.UI.Xaml.Controls.NavigationViewBackRequestedEventArgs",
        "Microsoft.UI.Xaml.Controls.NavigationViewDisplayMode",
        "Microsoft.UI.Xaml.Controls.NavigationViewDisplayModeChangedEventArgs",
        "Microsoft.UI.Xaml.Controls.NavigationViewItem",
        "Microsoft.UI.Xaml.Controls.NavigationViewItemBase",
        "Microsoft.UI.Xaml.Controls.NavigationViewItemCollapsedEventArgs",
        "Microsoft.UI.Xaml.Controls.NavigationViewItemExpandingEventArgs",
        "Microsoft.UI.Xaml.Controls.NavigationViewItemHeader",
        "Microsoft.UI.Xaml.Controls.NavigationViewItemInvokedEventArgs",
        "Microsoft.UI.Xaml.Controls.NavigationViewItemSeparator",
        "Microsoft.UI.Xaml.Controls.NavigationViewOverflowLabelMode",
        "Microsoft.UI.Xaml.Controls.NavigationViewPaneClosingEventArgs",
        "Microsoft.UI.Xaml.Controls.NavigationViewPaneDisplayMode",
        "Microsoft.UI.Xaml.Controls.NavigationViewSelectionChangedEventArgs",
        "Microsoft.UI.Xaml.Controls.NavigationViewSelectionFollowsFocus",
        "Microsoft.UI.Xaml.Controls.NavigationViewShoulderNavigationEnabled",
        "Microsoft.UI.Xaml.Controls.NavigationViewTemplateSettings",
        "Microsoft.UI.Xaml.Controls.NumberBox",
        "Microsoft.UI.Xaml.Controls.NumberBoxSpinButtonPlacementMode",
        "Microsoft.UI.Xaml.Controls.NumberBoxValidationMode",
        "Microsoft.UI.Xaml.Controls.NumberBoxValueChangedEventArgs",
        "Microsoft.UI.Xaml.Controls.PersonPicture",
        "Microsoft.UI.Xaml.Controls.PersonPictureTemplateSettings",
        "Microsoft.UI.Xaml.Controls.ProgressBar",
        "Microsoft.UI.Xaml.Controls.ProgressBarTemplateSettings",
        "Microsoft.UI.Xaml.Controls.ProgressRing",
        "Microsoft.UI.Xaml.Controls.ProgressRingTemplateSettings",
        "Microsoft.UI.Xaml.Controls.RadioButtons",
        "Microsoft.UI.Xaml.Controls.RatingControl",
        "Microsoft.UI.Xaml.Controls.RatingItemFontInfo",
        "Microsoft.UI.Xaml.Controls.RatingItemImageInfo",
        "Microsoft.UI.Xaml.Controls.RatingItemInfo",
        "Microsoft.UI.Xaml.Controls.RefreshContainer",
        "Microsoft.UI.Xaml.Controls.RefreshVisualizer",
        "Microsoft.UI.Xaml.Controls.SplitButton",
        "Microsoft.UI.Xaml.Controls.SplitButtonClickEventArgs",
        "Microsoft.UI.Xaml.Controls.TabView",
        "Microsoft.UI.Xaml.Controls.TabViewCloseButtonOverlayMode",
        "Microsoft.UI.Xaml.Controls.TabViewItem",
        "Microsoft.UI.Xaml.Controls.TabViewItemTemplateSettings",
        "Microsoft.UI.Xaml.Controls.TabViewTabCloseRequestedEventArgs",
        "Microsoft.UI.Xaml.Controls.TabViewWidthMode",
        "Microsoft.UI.Xaml.Controls.TeachingTip",
        "Microsoft.UI.Xaml.Controls.TeachingTipClosedEventArgs",
        "Microsoft.UI.Xaml.Controls.TeachingTipCloseReason",
        "Microsoft.UI.Xaml.Controls.TeachingTipClosingEventArgs",
        "Microsoft.UI.Xaml.Controls.TeachingTipHeroContentPlacementMode",
        "Microsoft.UI.Xaml.Controls.TeachingTipPlacementMode",
        "Microsoft.UI.Xaml.Controls.TeachingTipTailVisibility",
        "Microsoft.UI.Xaml.Controls.TeachingTipTemplateSettings",
        "Microsoft.UI.Xaml.Controls.ToggleSplitButton",
        "Microsoft.UI.Xaml.Controls.ToggleSplitButtonIsCheckedChangedEventArgs",
        "Microsoft.UI.Xaml.Controls.TreeView",
        "Microsoft.UI.Xaml.Controls.TreeViewCollapsedEventArgs",
        "Microsoft.UI.Xaml.Controls.TreeViewExpandingEventArgs",
        "Microsoft.UI.Xaml.Controls.TreeViewItem",
        "Microsoft.UI.Xaml.Controls.TreeViewItemInvokedEventArgs",
        "Microsoft.UI.Xaml.Controls.TreeViewItemTemplateSettings",
        "Microsoft.UI.Xaml.Controls.TreeViewList",
        "Microsoft.UI.Xaml.Controls.TreeViewNode",
        "Microsoft.UI.Xaml.Controls.TreeViewSelectionMode",
        "Microsoft.UI.Xaml.Controls.XamlControlsResources",
        "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider",
        "--reference",
        "xaml_bindings,flat,Windows.UI",
        "--reference",
        "xaml_bindings,flat,Windows.UI.Xaml",
        "--reference",
        "xaml_bindings,flat,Windows.UI.Input",
        "--reference",
        "xaml_bindings,flat,Windows.UI.Text",
        "--reference",
        "windows,skip-root,Windows.ApplicationModel.Resources.Core",
        "--reference",
        "windows_collections,flat,Windows.Foundation.Collections",
        "--reference",
        "windows,skip-root,Windows.Foundation.Numerics.Quaternion",
        "--reference",
        "windows_numerics,flat,Windows.Foundation.Numerics",
        "--reference",
        "windows,skip-root,Windows.Foundation",
    ];
    run_bindgen(args)?;
    copy_if_changed(&temp, &out)
}

fn run_bindgen(args: Vec<&str>) -> Result<(), String> {
    let _warnings = windows_bindgen::bindgen(args);
    Ok(())
}

fn vendor_muxc_runtime(root: &Path, package: &Path) -> Result<(), String> {
    let mut registration = None;
    for arch in ["x64", "arm64"] {
        let appx = muxc_appx(package, arch);
        require_file(&appx, &format!("WinUI 2 {arch} AppX"))?;

        let work = root
            .join("target")
            .join("island-reactor-codegen")
            .join(format!("muxc-{arch}"));
        recreate_dir(&work)?;
        expand_appx(&appx, &work)?;

        let runtime = root.join("crates/muxc-bindings/runtime").join(arch);
        fs::create_dir_all(&runtime).map_err(|err| {
            format!(
                "failed to create runtime asset dir {}: {err}",
                runtime.display()
            )
        })?;
        copy_file(
            &work.join("Microsoft.UI.Xaml.dll"),
            &runtime.join("Microsoft.UI.Xaml.dll"),
        )?;
        trim_mux_pri(
            &work.join("resources.pri"),
            &runtime.join("Microsoft.UI.Xaml.pri"),
        )?;

        if registration.is_none() {
            registration = Some(parse_mux_registration(&work.join("AppxManifest.xml"))?);
        }
    }

    write_runtime_rs(
        &root.join("crates/muxc-bindings/src/runtime.rs"),
        &registration.unwrap_or_default(),
    )
}

fn trim_mux_pri(input: &Path, output: &Path) -> Result<(), String> {
    require_file(input, "WinUI 2 resources.pri")?;
    let makepri = find_makepri().ok_or_else(|| {
        "cannot find makepri.exe; install Microsoft.Windows.SDK.BuildTools or set MAKEPRI_EXE"
            .to_string()
    })?;

    let work = output
        .parent()
        .ok_or_else(|| format!("output PRI has no parent: {}", output.display()))?
        .join("_trim-pri");
    recreate_dir(&work)?;
    let input_name = work.join("resources.pri");
    copy_file(input, &input_name)?;

    run_command(
        Command::new(&makepri)
            .current_dir(&work)
            .arg("dump")
            .arg("/if")
            .arg(&input_name)
            .arg("/dt")
            .arg("detailed")
            .arg("/o"),
        "makepri dump",
    )?;

    let dump = work.join("resources.pri.xml");
    require_file(&dump, "makepri dump XML")?;
    trim_xbf_nodes(&dump)?;

    let config = work.join("mux_priconfig.xml");
    fs::write(&config, MUX_PRI_CONFIG)
        .map_err(|err| format!("failed to write {}: {err}", config.display()))?;

    let out_name = output
        .file_name()
        .ok_or_else(|| format!("output PRI has no file name: {}", output.display()))?;
    run_command(
        Command::new(&makepri)
            .current_dir(&work)
            .arg("new")
            .arg("/pr")
            .arg(".")
            .arg("/of")
            .arg(out_name)
            .arg("/cf")
            .arg("mux_priconfig.xml")
            .arg("/in")
            .arg("Microsoft.UI.Xaml.2.8")
            .arg("/o"),
        "makepri new",
    )?;

    copy_file(&work.join(out_name), output)?;
    let _ = fs::remove_dir_all(&work);
    Ok(())
}

fn trim_xbf_nodes(path: &Path) -> Result<(), String> {
    let file =
        fs::File::open(path).map_err(|err| format!("failed to open {}: {err}", path.display()))?;
    let mut xml = Element::parse(BufReader::new(file))
        .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
    trim_xbf_element(&mut xml);
    let file = fs::File::create(path)
        .map_err(|err| format!("failed to rewrite {}: {err}", path.display()))?;
    xml.write_with_config(
        BufWriter::new(file),
        EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(true),
    )
    .map_err(|err| format!("failed to save {}: {err}", path.display()))
}

fn trim_xbf_element(element: &mut Element) {
    if local_name(&element.name) == "NamedResource"
        && let Some(name) = element.attributes.get("name")
        && name.ends_with(".xbf")
        && should_strip_xbf(name)
    {
        set_candidate_base64(element, "IA==");
    }

    for child in &mut element.children {
        if let XMLNode::Element(element) = child {
            trim_xbf_element(element);
        }
    }
}

fn should_strip_xbf(name: &str) -> bool {
    if name.contains("scale-100") {
        return true;
    }
    for key in [
        "compact", "Compact", "v1", "rs2", "rs3", "rs4", "rs5", "19h1",
    ] {
        if name.contains(key) {
            return true;
        }
    }
    !name.contains("21h1")
}

fn set_candidate_base64(element: &mut Element, value: &str) {
    for child in &mut element.children {
        let XMLNode::Element(element) = child else {
            continue;
        };
        if local_name(&element.name) == "Base64Value" {
            element.children.clear();
            element.children.push(XMLNode::Text(value.to_string()));
        } else {
            set_candidate_base64(element, value);
        }
    }
}

fn parse_mux_registration(manifest: &Path) -> Result<Vec<ActivatableClass>, String> {
    let file = fs::File::open(manifest)
        .map_err(|err| format!("failed to open {}: {err}", manifest.display()))?;
    let xml = Element::parse(BufReader::new(file))
        .map_err(|err| format!("failed to parse {}: {err}", manifest.display()))?;
    let server = find_element(&xml, "InProcessServer")
        .ok_or_else(|| format!("missing InProcessServer in {}", manifest.display()))?;
    let mut classes = Vec::new();
    collect_activatable_classes(server, &mut classes);
    if classes.is_empty() {
        return Err(format!(
            "no ActivatableClass entries found in {}",
            manifest.display()
        ));
    }
    classes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(classes)
}

fn collect_activatable_classes(element: &Element, out: &mut Vec<ActivatableClass>) {
    if local_name(&element.name) == "ActivatableClass"
        && let Some(name) = element.attributes.get("ActivatableClassId")
    {
        let threading_model = element
            .attributes
            .get("ThreadingModel")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_else(|| "both".to_string());
        out.push(ActivatableClass {
            name: name.clone(),
            threading_model,
        });
    }
    for child in &element.children {
        if let XMLNode::Element(element) = child {
            collect_activatable_classes(element, out);
        }
    }
}

fn find_element<'a>(element: &'a Element, name: &str) -> Option<&'a Element> {
    if local_name(&element.name) == name {
        return Some(element);
    }
    for child in &element.children {
        if let XMLNode::Element(element) = child
            && let Some(found) = find_element(element, name)
        {
            return Some(found);
        }
    }
    None
}

#[derive(Default)]
struct ActivatableClass {
    name: String,
    threading_model: String,
}

fn write_runtime_rs(path: &Path, classes: &[ActivatableClass]) -> Result<(), String> {
    let mut code = String::new();
    code.push_str("use std::path::{Path, PathBuf};\n\n");
    code.push_str(&format!(
        "pub const WINUI2_VERSION: &str = {:?};\n",
        WINUI2_VERSION
    ));
    code.push_str("pub const RUNTIME_DLL: &str = \"Microsoft.UI.Xaml.dll\";\n");
    code.push_str("pub const RUNTIME_PRI: &str = \"Microsoft.UI.Xaml.pri\";\n\n");
    code.push_str("#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n");
    code.push_str("pub struct ActivatableClass {\n");
    code.push_str("    pub name: &'static str,\n");
    code.push_str("    pub threading_model: &'static str,\n");
    code.push_str("}\n\n");
    code.push_str("pub const ACTIVATABLE_CLASSES: &[ActivatableClass] = &[\n");
    for class in classes {
        code.push_str("    ActivatableClass { name: ");
        code.push_str(&format!("{:?}", class.name));
        code.push_str(", threading_model: ");
        code.push_str(&format!("{:?}", class.threading_model));
        code.push_str(" },\n");
    }
    code.push_str("];\n\n");
    code.push_str("pub fn runtime_asset_dir(arch: &str) -> Option<PathBuf> {\n");
    code.push_str("    match arch {\n");
    code.push_str(
        "        \"x64\" | \"arm64\" => Some(Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"runtime\").join(arch)),\n",
    );
    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    write_if_changed(path, &rustfmt(&code))
}

fn write_if_changed(path: &Path, content: &str) -> Result<(), String> {
    if let Ok(existing) = fs::read_to_string(path)
        && existing == content
    {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(path, content).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn rustfmt(code: &str) -> String {
    let Ok(mut child) = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return code.to_string();
    };
    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(code.as_bytes());
    }
    match child.wait_with_output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).unwrap_or_else(|_| code.to_string())
        }
        _ => code.to_string(),
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("could not resolve workspace root from {}", dir.display()))
}

fn winui2_package_dir() -> Result<PathBuf, String> {
    if let Ok(root) = env::var("WINUI2_NUGET_DIR") {
        let root = PathBuf::from(root);
        if root.exists() {
            return Ok(root);
        }
        return Err(format!(
            "WINUI2_NUGET_DIR points to a missing directory: {}",
            root.display()
        ));
    }

    let root = if let Ok(root) = env::var("NUGET_PACKAGES") {
        PathBuf::from(root)
    } else {
        let profile = env::var("USERPROFILE").map_err(|_| "USERPROFILE is not set".to_string())?;
        PathBuf::from(profile).join(".nuget").join("packages")
    };
    let package = root.join("microsoft.ui.xaml").join(WINUI2_VERSION);
    if package.exists() {
        Ok(package)
    } else {
        Err(format!(
            "missing NuGet package microsoft.ui.xaml/{WINUI2_VERSION}; expected {}",
            package.display()
        ))
    }
}

fn extract_muxc_metadata_winmd(root: &Path, package: &Path) -> Result<PathBuf, String> {
    let appx = muxc_appx(package, "x64");
    require_file(&appx, "WinUI 2 x64 AppX")?;

    let work = root
        .join("target")
        .join("island-reactor-codegen")
        .join("muxc-metadata-x64");
    recreate_dir(&work)?;
    expand_appx(&appx, &work)?;

    let winmd = work.join("Microsoft.UI.Xaml.winmd");
    require_file(&winmd, "WinUI 2 AppX winmd")?;
    Ok(winmd)
}

fn muxc_appx(package: &Path, arch: &str) -> PathBuf {
    package
        .join("tools")
        .join("AppX")
        .join(arch)
        .join("Release")
        .join("Microsoft.UI.Xaml.2.8.appx")
}

fn expand_appx(appx: &Path, stage: &Path) -> Result<(), String> {
    run_command(
        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(format!(
                "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force",
                ps_escape(appx),
                ps_escape(stage)
            )),
        "Expand-Archive",
    )
}

fn recreate_dir(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|err| format!("failed to remove {}: {err}", path.display()))?;
    }
    fs::create_dir_all(path).map_err(|err| format!("failed to create {}: {err}", path.display()))
}

fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "source file")?;
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::copy(src, dst).map(|_| ()).map_err(|err| {
        format!(
            "failed to copy {} to {}: {err}",
            src.display(),
            dst.display()
        )
    })
}

fn copy_if_changed(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "generated bindings")?;
    if let (Ok(existing), Ok(generated)) = (fs::read(dst), fs::read(src))
        && existing == generated
    {
        return Ok(());
    }
    copy_file(src, dst)
}

fn require_file(path: &Path, label: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("{label} not found: {}", path.display()))
    }
}

fn run_command(command: &mut Command, label: &str) -> Result<(), String> {
    let output = command
        .output()
        .map_err(|err| format!("{label} failed to start: {err}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "{label} exited with {}{}\n{}",
            output.status,
            if stdout.trim().is_empty() {
                String::new()
            } else {
                format!("\nstdout:\n{stdout}")
            },
            stderr
        ))
    }
}

fn find_makepri() -> Option<PathBuf> {
    if let Ok(path) = env::var("MAKEPRI_EXE") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }
    let profile = env::var("USERPROFILE").ok()?;
    let build_tools = PathBuf::from(&profile)
        .join(".nuget")
        .join("packages")
        .join("microsoft.windows.sdk.buildtools");
    if let Some(path) = find_makepri_under(&build_tools) {
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

fn path_arg(path: &Path) -> String {
    path.display().to_string()
}

fn local_name(name: &str) -> &str {
    name.rsplit_once(':').map_or(name, |(_, name)| name)
}

fn ps_escape(path: &Path) -> String {
    path.display().to_string().replace('\'', "''")
}

const MUX_PRI_CONFIG: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<resources targetOsVersion="10.0.0" majorVersion="1">
  <packaging>
    <autoResourcePackage qualifier="Scale" />
    <autoResourcePackage qualifier="DXFeatureLevel" />
  </packaging>
  <index startIndexAt="resources.pri.xml" root="">
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
    <indexer-config type="priinfo" emitStrings="true" emitPaths="true" emitEmbeddedData="true" />
  </index>
</resources>
"#;
