# Reactor Surface Status

This crate targets `Windows.UI.Xaml` XAML Islands plus the WinUI 2 (`Microsoft.UI.Xaml`) AppX metadata/runtime from `microsoft.ui.xaml` 2.8.7.

The public surface should be the `island-reactor` wrapper API. The checked-in XAML and MUXC bindings are private modules inside `crates/island-reactor/src` and are not exposed as standalone crates.

## Filled With WinUI 2 MUXC

These upstream Reactor controls are now represented by wrappers backed by `Microsoft.UI.Xaml.Controls` from the WinUI 2 AppX winmd:

| Control/API | Gallery tag/page | Notes |
| --- | --- | --- |
| `BreadcrumbBar` | `breadcrumb-bar` | Uses MUXC `BreadcrumbBar` and string `ItemsSource`. |
| `Expander` | `expander` | Uses MUXC `Expander`; supports string or element header and content. |
| `InfoBadge` | `info-badge` | Uses MUXC `InfoBadge`; supports dot and numeric badges. |
| `InfoBar` | `info-bar` | Uses MUXC `InfoBar`; supports severity, open state, closable state, and close event. |
| `NumberBox` | `number-box` | Uses MUXC `NumberBox`; supports value, range, header, enabled state, and value-changed event. |
| `RadioButtons` | RadioButton page sample | Uses MUXC grouped `RadioButtons`; included as a sample on the RadioButton page. |
| `TabView` | `tab-view` | Uses MUXC `TabView`/`TabViewItem`; supports selection, add-tab button, and close metadata. |
| `TeachingTip` | `teaching-tip` | Uses MUXC `TeachingTip`; supports title, subtitle, open state, close/action buttons, and events. |
| `App::backdrop`, `set_backdrop`, `Backdrop::Mica`, `Backdrop::MicaAlt`, `Backdrop::Acrylic` | `materials` | Uses the Xaml-Islands-Cpp-style Win32/DWM backdrop path on Windows 11 22H2 or newer. |

## Adapted Surface

| Upstream control/API | Gallery tag/page | Local behavior | Reason |
| --- | --- | --- | --- |
| `ScrollView` | `scroll-view` | Gallery page is backed by `Windows.UI.Xaml.Controls.ScrollViewer`. | The WinUI 2 AppX winmd used for checked-in bindings does not expose `Microsoft.UI.Xaml.Controls.ScrollView`. The runtime DLL and .NET projection contain ScrollView symbols, but `windows-bindgen` currently fails on the projection metadata, so the binding source stays AppX-winmd-only. |
| Reactor `Backdrop` implementation | `materials` | All host windows use `WS_EX_NOREDIRECTIONBITMAP`; backdrop windows also use `DWMWA_SYSTEMBACKDROP_TYPE`, matching the `Xaml-Islands-Cpp` DWM path instead of upstream Reactor's WinAppSDK `SystemBackdrop` objects. | This project hosts `Windows.UI.Xaml` Islands with WinUI 2 runtime assets, not WinUI 3/Windows App SDK `Window`. Use `App::backdrop(...)` for windows that need Mica/Acrylic root material from startup; `set_backdrop` switches the installed DWM backdrop at runtime. |

## Still Unsupported Or Intentionally Different

| Upstream control/API | Omitted gallery page/tag | Reason | Possible future path |
| --- | --- | --- | --- |
| `SelectorBar` | `selector-bar` | Not exposed by the WinUI 2 AppX winmd used by this project. | Keep omitted or compose from `Pivot`, `RadioButtons`, or `ListView`. |
| `TitleBar` | `title-bar`; gallery shell startup | Custom title bar APIs are outside the current XAML Islands wrapper surface. | Implement separate Win32 non-client/titlebar integration. |
| `SurfaceImageSource` | None | Direct2D surface interop interfaces are not generated for this crate. | Add hand-written COM bindings and lifetime management for the required native surface interfaces. |
| `SwapChainPanel` native interop helpers | None; upstream swap-chain sample omitted | Swap-chain native interop interfaces and composition scale events are not exposed by the checked-in bindings. | Add hand-written COM bindings for swap-chain interop and a focused DirectX sample. |
| `NavViewItem::child` / nested navigation menu items | Gallery shell nested navigation omitted | `NavigationViewItem.MenuItems` is not currently wrapped. | Add a typed wrapper over MUXC nested menu APIs or keep the gallery navigation flat. |
| `ProgressRing` determinate value/range APIs | ProgressRing page adapted | `Windows.UI.Xaml.Controls.ProgressRing` exposes active indeterminate display, not determinate value/range properties. | Use MUXC `ProgressRing` range members if needed, or compose a custom determinate circular progress visual. |

## Binding Policy

Normal `cargo build` does not run `windows-bindgen` or contact NuGet. Regeneration is explicit:

```powershell
cargo run -p island-reactor-codegen -- generate-bindings
```

The codegen tool currently extracts `Microsoft.UI.Xaml.winmd` from the x64 WinUI 2 AppX and treats that file as the authoritative MUXC metadata source. It also copies the matching x64/arm64 AppX runtime payloads. It should not switch to the .NET projection metadata until that path can be generated reproducibly.

The workspace root is only a workspace. The main library lives in `crates/island-reactor`; applications use `island-reactor-setup` from `[build-dependencies]` so their build scripts can embed the manifest, stage the WinUI 2 DLL, and produce the app `resources.pri`.
