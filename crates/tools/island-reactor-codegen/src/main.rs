use std::{
    env, fs,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use xmltree::{Element, EmitterConfig, XMLNode};

const WINUI2_VERSION: &str = "2.8.7";

const XAML_MINIMAL_FILTERS: &[&str] = &[
    "Windows.ApplicationModel.Resources.Core.ResourceManager::{get_Current}",
    "Windows.ApplicationModel.Resources.Core.IResourceManager::{LoadPriFiles}",
    "Windows.Storage.IStorageFile",
    "Windows.Storage.StorageFile::{GetFileFromPathAsync}",
    "Windows.UI.Color",
    "Windows.UI.Text.FontWeight",
    "Windows.UI.Xaml.Application::{CreateInstance, get_Current}",
    "Windows.UI.Xaml.IApplication::{get_Resources, put_Resources, add_UnhandledException}",
    "Windows.UI.Xaml.IApplicationOverrides::*",
    "Windows.UI.Xaml.Controls.AutoSuggestBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IAutoSuggestBox::{get_Text, put_PlaceholderText, put_Header, add_TextChanged, add_SuggestionChosen}",
    "Windows.UI.Xaml.Controls.IAutoSuggestBox2::{add_QuerySubmitted}",
    "Windows.UI.Xaml.Controls.AutoSuggestBoxQuerySubmittedEventArgs",
    "Windows.UI.Xaml.Controls.IAutoSuggestBoxQuerySubmittedEventArgs::{get_QueryText}",
    "Windows.UI.Xaml.Controls.AutoSuggestBoxSuggestionChosenEventArgs",
    "Windows.UI.Xaml.Controls.IAutoSuggestBoxSuggestionChosenEventArgs::{get_SelectedItem}",
    "Windows.UI.Xaml.Controls.AutoSuggestBoxTextChangedEventArgs",
    "Windows.UI.Xaml.Controls.Border::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IBorder::{put_Child, put_Padding, put_Background, put_CornerRadius, put_BorderBrush, put_BorderThickness}",
    "Windows.UI.Xaml.Controls.Button::{CreateInstance}",
    "Windows.UI.Xaml.Controls.CalendarDatePicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ICalendarDatePicker::{put_Header, put_PlaceholderText, put_IsCalendarOpen, put_IsTodayHighlighted}",
    "Windows.UI.Xaml.Controls.CalendarView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ICalendarView::{put_IsTodayHighlighted, put_IsGroupLabelVisible}",
    "Windows.UI.Xaml.Controls.Canvas::{CreateInstance, SetLeft, SetTop, SetZIndex}",
    "Windows.UI.Xaml.Controls.CheckBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ColorChangedEventArgs",
    "Windows.UI.Xaml.Controls.ColorPicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IColorChangedEventArgs::{get_NewColor}",
    "Windows.UI.Xaml.Controls.IColorPicker::{put_Color, add_ColorChanged}",
    "Windows.UI.Xaml.Controls.ColumnDefinition::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IColumnDefinition::{put_Width}",
    "Windows.UI.Xaml.Controls.ColumnDefinitionCollection",
    "Windows.UI.Xaml.Controls.ComboBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IComboBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.CommandBar::{CreateInstance}",
    "Windows.UI.Xaml.Controls.CommandBarDefaultLabelPosition",
    "Windows.UI.Xaml.Controls.ContentControl",
    "Windows.UI.Xaml.Controls.IContentControl::{put_Content}",
    "Windows.UI.Xaml.Controls.ContentDialog::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Control",
    "Windows.UI.Xaml.Controls.IControl::{put_IsEnabled, put_Padding, put_Background, put_Foreground, put_FontSize, put_FontWeight, put_FontFamily}",
    "Windows.UI.Xaml.Controls.DatePicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IDatePicker::{put_Header, put_DayVisible, put_MonthVisible, put_YearVisible}",
    "Windows.UI.Xaml.Controls.DropDownButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.FlipView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Grid::{CreateInstance, SetColumn, SetColumnSpan, SetRow, SetRowSpan}",
    "Windows.UI.Xaml.Controls.IGrid::{get_RowDefinitions, get_ColumnDefinitions}",
    "Windows.UI.Xaml.Controls.IGrid3::{put_RowSpacing, put_ColumnSpacing}",
    "Windows.UI.Xaml.Controls.GridView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.HyperlinkButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IconElement",
    "Windows.UI.Xaml.Controls.Image::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IImage::{put_Source, put_Stretch}",
    "Windows.UI.Xaml.Controls.ItemCollection",
    "Windows.UI.Xaml.Controls.ItemsControl",
    "Windows.UI.Xaml.Controls.IItemsControl::{get_Items}",
    "Windows.UI.Xaml.Controls.ListBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ListView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ListViewSelectionMode",
    "Windows.UI.Xaml.Controls.IListViewBase::{put_SelectionMode}",
    "Windows.UI.Xaml.Controls.MenuBar::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Orientation",
    "Windows.UI.Xaml.Controls.Panel",
    "Windows.UI.Xaml.Controls.IPanel::{get_Children, put_Background}",
    "Windows.UI.Xaml.Controls.PasswordBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IPasswordBox::{get_Password, put_IsPasswordRevealButtonEnabled, add_PasswordChanged}",
    "Windows.UI.Xaml.Controls.IPasswordBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.IPasswordBox3::{put_PasswordRevealMode}",
    "Windows.UI.Xaml.Controls.PasswordRevealMode",
    "Windows.UI.Xaml.Controls.PersonPicture::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IPersonPicture::{put_DisplayName, put_Initials}",
    "Windows.UI.Xaml.Controls.Pivot::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IPivot::{put_SelectedIndex}",
    "Windows.UI.Xaml.Controls.PivotItem::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ProgressBar::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IProgressBar::{put_IsIndeterminate}",
    "Windows.UI.Xaml.Controls.ProgressRing::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IProgressRing::{put_IsActive}",
    "Windows.UI.Xaml.Controls.RadioButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.RatingControl::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRatingControl::{get_Value, put_Value, add_ValueChanged}",
    "Windows.UI.Xaml.Controls.RelativePanel::{CreateInstance, SetAlignBottomWithPanel, SetAlignHorizontalCenterWithPanel, SetAlignLeftWithPanel, SetAlignRightWithPanel, SetAlignTopWithPanel, SetAlignVerticalCenterWithPanel}",
    "Windows.UI.Xaml.Controls.RichEditBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRichEditBox::{put_IsReadOnly}",
    "Windows.UI.Xaml.Controls.IRichEditBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.RichTextBlock::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRichTextBlock::{get_Blocks, put_FontSize, put_FontFamily, put_Foreground}",
    "Windows.UI.Xaml.Controls.RowDefinition::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRowDefinition::{put_Height}",
    "Windows.UI.Xaml.Controls.RowDefinitionCollection",
    "Windows.UI.Xaml.Controls.ScrollBarVisibility",
    "Windows.UI.Xaml.Controls.ScrollViewer::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IScrollViewer::{put_HorizontalScrollBarVisibility, put_VerticalScrollBarVisibility}",
    "Windows.UI.Xaml.Controls.SelectionChangedEventArgs",
    "Windows.UI.Xaml.Controls.SelectionChangedEventHandler::*",
    "Windows.UI.Xaml.Controls.Slider::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ISlider::{put_Orientation, put_StepFrequency}",
    "Windows.UI.Xaml.Controls.ISlider2::{put_Header}",
    "Windows.UI.Xaml.Controls.SplitButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.SplitView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ISplitView::{put_IsPaneOpen, put_OpenPaneLength, put_CompactPaneLength, put_DisplayMode, put_Content, put_Pane}",
    "Windows.UI.Xaml.Controls.SplitViewDisplayMode",
    "Windows.UI.Xaml.Controls.StackPanel::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IStackPanel::{put_Orientation}",
    "Windows.UI.Xaml.Controls.IStackPanel4::{put_Spacing}",
    "Windows.UI.Xaml.Controls.Symbol",
    "Windows.UI.Xaml.Controls.SymbolIcon::{CreateInstanceWithSymbol}",
    "Windows.UI.Xaml.Controls.TextBlock::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITextBlock::{put_Text, put_TextWrapping, put_IsTextSelectionEnabled, put_FontSize, put_FontWeight, put_FontFamily, put_Foreground}",
    "Windows.UI.Xaml.Controls.TextBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITextBox::{get_Text, put_Text, put_TextWrapping, put_AcceptsReturn, add_TextChanged}",
    "Windows.UI.Xaml.Controls.ITextBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.TimePicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITimePicker::{put_Header, put_ClockIdentifier, put_MinuteIncrement}",
    "Windows.UI.Xaml.Controls.ToggleSwitch::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IToggleSwitch::{get_IsOn, put_IsOn, put_OnContent, put_OffContent, add_Toggled}",
    "Windows.UI.Xaml.Controls.ToolTipService::{SetToolTip}",
    "Windows.UI.Xaml.Controls.TreeView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITreeView::{put_SelectionMode}",
    "Windows.UI.Xaml.Controls.TreeViewSelectionMode",
    "Windows.UI.Xaml.Controls.UIElementCollection",
    "Windows.UI.Xaml.Controls.Viewbox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IViewbox::{put_Child, put_Stretch}",
    "Windows.UI.Xaml.Controls.Primitives.ButtonBase",
    "Windows.UI.Xaml.Controls.Primitives.IButtonBase::{add_Click}",
    "Windows.UI.Xaml.Controls.Primitives.FlyoutPlacementMode",
    "Windows.UI.Xaml.Controls.Primitives.RangeBase",
    "Windows.UI.Xaml.Controls.Primitives.IRangeBase::{get_Value, put_Value, put_Minimum, put_Maximum, add_ValueChanged}",
    "Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs",
    "Windows.UI.Xaml.Controls.Primitives.IRangeBaseValueChangedEventArgs::{get_NewValue}",
    "Windows.UI.Xaml.Controls.Primitives.RepeatButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Primitives.IRepeatButton::{put_Delay, put_Interval}",
    "Windows.UI.Xaml.Controls.Primitives.Selector",
    "Windows.UI.Xaml.Controls.Primitives.ISelector::{get_SelectedIndex, put_SelectedIndex, add_SelectionChanged}",
    "Windows.UI.Xaml.Controls.Primitives.ToggleButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Primitives.IToggleButton::{put_IsChecked, add_Checked, add_Unchecked}",
    "Windows.UI.Xaml.CornerRadius",
    "Windows.UI.Xaml.DataTemplate::{CreateInstance}",
    "Windows.UI.Xaml.DependencyObject",
    "Windows.UI.Xaml.DispatcherTimer::{CreateInstance}",
    "Windows.UI.Xaml.IDispatcherTimer::{put_Interval, add_Tick, Start, Stop}",
    "Windows.UI.Xaml.Documents.Block",
    "Windows.UI.Xaml.Documents.BlockCollection",
    "Windows.UI.Xaml.Documents.IParagraph::{get_Inlines}",
    "Windows.UI.Xaml.Documents.Inline",
    "Windows.UI.Xaml.Documents.InlineCollection",
    "Windows.UI.Xaml.Documents.IRun::{put_Text}",
    "Windows.UI.Xaml.Documents.Paragraph::{CreateInstance}",
    "Windows.UI.Xaml.Documents.Run::{CreateInstance}",
    "Windows.UI.Xaml.ElementTheme",
    "Windows.UI.Xaml.FrameworkElement",
    "Windows.UI.Xaml.IFrameworkElement::{put_VerticalAlignment, put_HorizontalAlignment, put_Margin, put_Height, put_Width, put_MinWidth, put_MaxWidth, put_MinHeight, put_MaxHeight, put_Tag, get_Tag}",
    "Windows.UI.Xaml.IFrameworkElement2::{put_RequestedTheme}",
    "Windows.UI.Xaml.IFrameworkElement6::{get_ActualTheme}",
    "Windows.UI.Xaml.GridLength",
    "Windows.UI.Xaml.GridUnitType",
    "Windows.UI.Xaml.HorizontalAlignment",
    "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource::{CreateInstance}",
    "Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource::{put_Content}",
    "Windows.UI.Xaml.Hosting.WindowsXamlManager::{InitializeForCurrentThread}",
    "Windows.UI.Xaml.Interop.TypeName",
    "Windows.UI.Xaml.Markup.IXamlMetadataProvider::*",
    "Windows.UI.Xaml.Markup.IXamlType::*",
    "Windows.UI.Xaml.Markup.XamlReader::{Load}",
    "Windows.UI.Xaml.Markup.XmlnsDefinition",
    "Windows.UI.Xaml.Media.Brush",
    "Windows.UI.Xaml.Media.CompositionTarget::{add_Rendering}",
    "Windows.UI.Xaml.Media.FontFamily::{CreateInstanceWithName}",
    "Windows.UI.Xaml.Media.ImageSource",
    "Windows.UI.Xaml.Media.Imaging.BitmapImage::{CreateInstance}",
    "Windows.UI.Xaml.Media.Imaging.IBitmapImage::{put_UriSource}",
    "Windows.UI.Xaml.Media.SolidColorBrush::{CreateInstance}",
    "Windows.UI.Xaml.Media.ISolidColorBrush::{put_Color}",
    "Windows.UI.Xaml.Media.Stretch",
    "Windows.UI.Xaml.ResourceDictionary::{CreateInstance}",
    "Windows.UI.Xaml.IResourceDictionary::{get_MergedDictionaries}",
    "Windows.UI.Xaml.RoutedEventArgs",
    "Windows.UI.Xaml.RoutedEventHandler::*",
    "Windows.UI.Xaml.Shapes.Ellipse::{CreateInstance}",
    "Windows.UI.Xaml.Shapes.Line::{CreateInstance}",
    "Windows.UI.Xaml.Shapes.ILine::{put_X1, put_Y1, put_X2, put_Y2}",
    "Windows.UI.Xaml.Shapes.Rectangle::{CreateInstance}",
    "Windows.UI.Xaml.Shapes.IRectangle::{put_RadiusX, put_RadiusY}",
    "Windows.UI.Xaml.Shapes.Shape",
    "Windows.UI.Xaml.Shapes.IShape::{put_Fill, put_Stroke, put_StrokeThickness}",
    "Windows.UI.Xaml.TextWrapping",
    "Windows.UI.Xaml.Thickness",
    "Windows.UI.Xaml.UIElement",
    "Windows.UI.Xaml.IUIElement::{put_Opacity}",
    "Windows.UI.Xaml.UnhandledExceptionEventArgs",
    "Windows.UI.Xaml.IUnhandledExceptionEventArgs::{get_Exception, get_Message}",
    "Windows.UI.Xaml.UnhandledExceptionEventHandler",
    "Windows.UI.Xaml.VerticalAlignment",
    "Windows.Win32.Foundation.HWND",
    "Windows.Win32.System.WinRT.Xaml.IDesktopWindowXamlSourceNative::{AttachToWindow, get_WindowHandle}",
];

const MUXC_MINIMAL_FILTERS: &[&str] = &[
    "Microsoft.UI.Xaml.Controls.BreadcrumbBar::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.BreadcrumbBarItem",
    "Microsoft.UI.Xaml.Controls.BreadcrumbBarItemClickedEventArgs",
    "Microsoft.UI.Xaml.Controls.IBreadcrumbBar::{put_ItemsSource, add_ItemClicked}",
    "Microsoft.UI.Xaml.Controls.IBreadcrumbBarItemClickedEventArgs::{get_Index}",
    "Microsoft.UI.Xaml.Controls.Expander::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.ExpanderCollapsedEventArgs",
    "Microsoft.UI.Xaml.Controls.ExpanderExpandingEventArgs",
    "Microsoft.UI.Xaml.Controls.IExpander::{put_Header, put_IsExpanded, add_Expanding, add_Collapsed}",
    "Microsoft.UI.Xaml.Controls.InfoBadge::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.IInfoBadge::{put_Value}",
    "Microsoft.UI.Xaml.Controls.InfoBar::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.InfoBarClosedEventArgs",
    "Microsoft.UI.Xaml.Controls.InfoBarSeverity",
    "Microsoft.UI.Xaml.Controls.IInfoBar::{put_Title, put_Message, put_Severity, put_IsOpen, put_IsClosable, add_Closed}",
    "Microsoft.UI.Xaml.Controls.NavigationView::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.NavigationViewBackButtonVisible",
    "Microsoft.UI.Xaml.Controls.NavigationViewBackRequestedEventArgs",
    "Microsoft.UI.Xaml.Controls.INavigationView::{get_MenuItems, put_SelectedItem, put_IsPaneOpen, put_IsSettingsVisible, put_IsPaneToggleButtonVisible, add_SelectionChanged}",
    "Microsoft.UI.Xaml.Controls.INavigationView2::{put_PaneDisplayMode, put_IsBackButtonVisible, put_IsBackEnabled, put_PaneTitle, add_BackRequested}",
    "Microsoft.UI.Xaml.Controls.INavigationViewItem::{put_Icon}",
    "Microsoft.UI.Xaml.Controls.INavigationViewSelectionChangedEventArgs::{get_SelectedItem}",
    "Microsoft.UI.Xaml.Controls.NavigationViewItem::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.NavigationViewItemBase",
    "Microsoft.UI.Xaml.Controls.NavigationViewItemHeader::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.NavigationViewPaneDisplayMode",
    "Microsoft.UI.Xaml.Controls.NavigationViewSelectionChangedEventArgs",
    "Microsoft.UI.Xaml.Controls.NumberBox::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.NumberBoxValueChangedEventArgs",
    "Microsoft.UI.Xaml.Controls.INumberBox::{put_Header, put_Value, put_Minimum, put_Maximum, add_ValueChanged}",
    "Microsoft.UI.Xaml.Controls.INumberBoxValueChangedEventArgs::{get_NewValue}",
    "Microsoft.UI.Xaml.Controls.RadioButtons::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.IRadioButtons::{put_Header, get_Items, put_SelectedIndex, get_SelectedIndex, put_MaxColumns, add_SelectionChanged}",
    "Microsoft.UI.Xaml.Controls.TabView::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.ITabView::{get_TabItems, put_SelectedIndex, get_SelectedIndex, put_CanReorderTabs, put_IsAddTabButtonVisible, add_SelectionChanged, add_TabCloseRequested, add_AddTabButtonClick}",
    "Microsoft.UI.Xaml.Controls.TabViewItem::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.TabViewTabCloseRequestedEventArgs",
    "Microsoft.UI.Xaml.Controls.ITabViewItem::{put_Header, put_IsClosable}",
    "Microsoft.UI.Xaml.Controls.ITabViewTabCloseRequestedEventArgs::{get_Tab}",
    "Microsoft.UI.Xaml.Controls.TeachingTip::{CreateInstance}",
    "Microsoft.UI.Xaml.Controls.TeachingTipClosedEventArgs",
    "Microsoft.UI.Xaml.Controls.TeachingTipPlacementMode",
    "Microsoft.UI.Xaml.Controls.ITeachingTip::{put_Title, put_Subtitle, put_IsOpen, put_IsLightDismissEnabled, put_PreferredPlacement, put_ActionButtonContent, put_CloseButtonContent, add_Closed, add_ActionButtonClick}",
    "Microsoft.UI.Xaml.Controls.XamlControlsResources::{CreateInstance}",
    "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider::{CreateInstance}",
];

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
    let mut args = vec![
        "--in",
        "default",
        "--out",
        &temp_arg,
        "--implement",
        "Windows.UI.Xaml.IApplicationOverrides",
        "Windows.UI.Xaml.Markup.IXamlMetadataProvider",
        "--minimal",
        "--flat",
        "--filter",
    ];
    args.extend(XAML_MINIMAL_FILTERS);
    args.extend([
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
    run_bindgen(args)?;
    patch_xaml_bindings(&temp)?;
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
    let mut args = vec![
        "--in",
        "default",
        &winmd,
        "--out",
        &temp_arg,
        "--minimal",
        "--flat",
        "--filter",
    ];
    args.extend(MUXC_MINIMAL_FILTERS);
    args.extend([
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
    ]);
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

fn patch_xaml_bindings(path: &Path) -> Result<(), String> {
    let mut code = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    insert_selection_changed_event_handler_ctor(&mut code)?;
    fs::write(path, code).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn insert_selection_changed_event_handler_ctor(code: &mut String) -> Result<(), String> {
    if code.contains("impl SelectionChangedEventHandler {\n    pub fn new<") {
        return Ok(());
    }

    let ctor = r#"struct SelectionChangedEventHandlerResultBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> SelectionChangedEventHandlerResultBox<F>
{
    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SelectionChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            )
            .into()
        }
    }
}
impl SelectionChangedEventHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    {
        let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, _>::new(
            &SelectionChangedEventHandlerResultBox::<F>::VTABLE,
            handler,
        );
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
}
"#;

    for needle in [
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct ListViewSelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct ListViewSelectionMode(pub i32);",
    ] {
        if let Some(index) = code.find(needle) {
            code.insert_str(index, ctor);
            return Ok(());
        }
    }

    Err("failed to patch SelectionChangedEventHandler constructor".to_string())
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
