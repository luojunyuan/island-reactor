# Unsupported Reactor Surface

This crate targets `Windows.UI.Xaml` XAML Islands. The upstream Reactor APIs below are omitted because they have no direct native equivalent in that runtime or require native interop that is not yet implemented here.

| Upstream control/API | Omitted gallery page/tag | Reason | Possible future path |
| --- | --- | --- | --- |
| `BreadcrumbBar` | `breadcrumb-bar` | No native `Windows.UI.Xaml.Controls.BreadcrumbBar` control. | Compose a custom breadcrumb from `ItemsControl`-style primitives or add a custom widget. |
| `Expander` | `expander` | No native `Windows.UI.Xaml.Controls.Expander` control. | Compose a `ToggleButton` plus collapsible content. |
| `InfoBadge` | `info-badge` | No native `Windows.UI.Xaml.Controls.InfoBadge` control. | Compose a small badge from `Border` and `TextBlock`. |
| `InfoBar` | `info-bar` | No native `Windows.UI.Xaml.Controls.InfoBar` control. | Compose a dismissible banner from supported controls. |
| `NumberBox` | `number-box` | No native `Windows.UI.Xaml.Controls.NumberBox` control. | Compose `TextBox` with numeric parsing, validation, and optional step buttons. |
| `RadioButtons` | RadioButton page adapted | No native grouped `RadioButtons` control. | Use grouped `RadioButton` controls. |
| `ScrollView` | `scroll-view` | No native `Windows.UI.Xaml.Controls.ScrollView` control. | Use `ScrollViewer`, which is supported. |
| `SelectorBar` | `selector-bar` | No native `Windows.UI.Xaml.Controls.SelectorBar` control. | Compose a selector from `Pivot`, `RadioButton`, or `ListView`. |
| `TabView` | `tab-view` | No native `Windows.UI.Xaml.Controls.TabView` control. | Compose tabs from `Pivot` or custom headers and content. |
| `TeachingTip` | `teaching-tip` | No native `Windows.UI.Xaml.Controls.TeachingTip` control. | Compose a custom tip with `Flyout`/popup-style primitives when available. |
| `TitleBar` | `title-bar`; gallery shell startup | Custom title bar APIs are not provided by `Windows.UI.Xaml` Islands. | Use ordinary in-client controls or implement Win32 non-client customization separately. |
| `SurfaceImageSource` | None | Direct2D surface interop interfaces are not generated for this crate. | Add hand-written COM bindings and lifetime management for the required native surface interfaces. |
| `SwapChainPanel` native interop helpers | None; upstream swap-chain sample omitted | Swap-chain native interop interfaces and composition scale events are not exposed by the generated `Windows.UI.Xaml` bindings. | Add hand-written COM bindings for swap-chain interop and a focused DirectX sample. |
| `App::backdrop`, `set_backdrop`, `Backdrop::Mica`, `Backdrop::MicaAlt`, `Backdrop::Acrylic` | `materials` | Window backdrop materials are outside the `Windows.UI.Xaml` Islands surface. | Implement a separate Win32/DWM backdrop helper if needed. |
| `NavViewItem::child` / nested navigation menu items | Gallery shell nested navigation omitted | `NavigationViewItem.MenuItems` is not available in `Windows.UI.Xaml`. | Flatten navigation items or build a custom nested navigation surface. |
| `ProgressRing` determinate value/range APIs | ProgressRing page adapted | `Windows.UI.Xaml.Controls.ProgressRing` exposes active indeterminate display, not determinate value/range properties. | Compose a custom determinate circular progress visual if required. |
