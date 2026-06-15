use std::path::{Path, PathBuf};

pub const WINUI2_VERSION: &str = "2.8.7";
pub const RUNTIME_DLL: &str = "Microsoft.UI.Xaml.dll";
pub const RUNTIME_PRI: &str = "Microsoft.UI.Xaml.pri";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivatableClass {
    pub name: &'static str,
    pub threading_model: &'static str,
}

pub const ACTIVATABLE_CLASSES: &[ActivatableClass] = &[
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.AnimatedVisualPlayerAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.BreadcrumbBarItemAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.ColorPickerSliderAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.ColorSpectrumAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.DropDownButtonAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.ExpanderAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.InfoBarAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.MenuBarAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.MenuBarItemAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.NavigationViewAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.NavigationViewItemAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.NumberBoxAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.PersonPictureAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.PipsPagerAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.ProgressBarAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.ProgressRingAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.RadioButtonsAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.RatingControlAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.RepeaterAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.SplitButtonAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.TabViewAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.TabViewItemAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.TeachingTipAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.ToggleSplitButtonAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.TreeViewItemAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.TreeViewItemDataAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Automation.Peers.TreeViewListAutomationPeer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedIcon",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedIconSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisualPlayer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.BackdropMaterial",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.BitmapIconSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.BreadcrumbBar",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.BreadcrumbBarItem",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ColorPicker",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.CommandBarFlyout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.DropDownButton",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ElementFactory",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ElementFactoryGetArgs",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ElementFactoryRecycleArgs",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Expander",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.FontIconSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.IconSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ImageIcon",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ImageIconSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.InfoBadge",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.InfoBadgeTemplateSettings",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.InfoBar",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.InfoBarClosedEventArgs",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.InfoBarClosingEventArgs",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.InfoBarTemplateSettings",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ItemsRepeater",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ItemsRepeaterScrollHost",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ItemsSourceView",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Layout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.LayoutContext",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.MenuBar",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.MenuBarItem",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.MenuBarItemFlyout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NavigationView",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NavigationViewItem",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NavigationViewItemBase",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NavigationViewItemHeader",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NavigationViewItemInvokedEventArgs",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NavigationViewItemSeparator",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NavigationViewTemplateSettings",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NonVirtualizingLayout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NonVirtualizingLayoutContext",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.NumberBox",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ParallaxView",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.PathIconSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.PersonPicture",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.PipsPager",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.AutoSuggestBoxHelper",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.ColorPickerSlider",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.ColorSpectrum",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.ColumnMajorUniformToLargestGridLayout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.ComboBoxHelper",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBarAutomationProperties",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.CornerRadiusFilterConverter",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.CornerRadiusToThicknessConverter",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.InfoBarPanel",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.MonochromaticOverlayPresenter",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.NavigationViewItemPresenterTemplateSettings",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.Primitives.TabViewListView",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ProgressBar",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ProgressRing",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RadioButtons",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RadioMenuFlyoutItem",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RatingControl",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RatingItemFontInfo",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RatingItemImageInfo",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RatingItemInfo",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RefreshContainer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RefreshVisualizer",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.RevealListViewItemPresenter",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.SplitButton",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.StackLayout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.SwipeControl",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.SwipeItem",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.SwipeItems",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.SymbolIconSource",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TabView",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TabViewItem",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TabViewItemTemplateSettings",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TeachingTip",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TeachingTipTemplateSettings",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TextCommandBarFlyout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.ToggleSplitButton",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TreeView",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TreeViewItem",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TreeViewItemTemplateSettings",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TreeViewList",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TreeViewNode",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.TwoPaneView",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.UniformGridLayout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.VirtualizingLayout",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.VirtualizingLayoutContext",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.WebView2",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Controls.XamlControlsResources",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Media.AcrylicBrush",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Media.RadialGradientBrush",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Media.RevealBackgroundBrush",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Media.RevealBorderBrush",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.Media.RevealBrush",
        threading_model: "both",
    },
    ActivatableClass {
        name: "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider",
        threading_model: "both",
    },
];

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
