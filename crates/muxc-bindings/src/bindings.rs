windows_core::imp::define_interface!(
    INavigationView,
    INavigationView_Vtbl,
    0xb00eb54c_9174_5d84_9678_56c98016e689
);
impl windows_core::RuntimeType for INavigationView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.INavigationView");
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CompactModeThresholdWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetCompactModeThresholdWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ExpandedModeThresholdWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetExpandedModeThresholdWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub FooterMenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FooterMenuItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFooterMenuItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneFooter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPaneFooter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub HeaderTemplate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetHeaderTemplate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    DisplayMode: usize,
    pub IsSettingsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSettingsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AlwaysShowHeader:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAlwaysShowHeader:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CompactPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetCompactPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub OpenPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetOpenPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub PaneToggleButtonStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPaneToggleButtonStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMenuItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SettingsItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AutoSuggestBox: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetAutoSuggestBox: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemTemplate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMenuItemTemplate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemTemplateSelector: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMenuItemTemplateSelector: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemContainerStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMenuItemContainerStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemContainerStyleSelector: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMenuItemContainerStyleSelector: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemFromContainer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ContainerFromMenuItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ItemInvoked: usize,
    pub RemoveItemInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DisplayModeChanged: usize,
    pub RemoveDisplayModeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub IsTitleBarAutoPaddingEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsTitleBarAutoPaddingEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationView2,
    INavigationView2_Vtbl,
    0x5ba2eefc_3736_5e42_ac56_9d0be5523d40
);
impl windows_core::RuntimeType for INavigationView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.INavigationView2");
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBackButtonVisible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut NavigationViewBackButtonVisible,
    ) -> windows_core::HRESULT,
    pub SetIsBackButtonVisible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewBackButtonVisible,
    ) -> windows_core::HRESULT,
    pub IsBackEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsBackEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PaneTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPaneTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BackRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveBackRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PaneClosed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePaneClosed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PaneClosing: usize,
    pub RemovePaneClosing:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PaneOpened: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePaneOpened:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PaneOpening: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePaneOpening:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PaneDisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut NavigationViewPaneDisplayMode,
    ) -> windows_core::HRESULT,
    pub SetPaneDisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewPaneDisplayMode,
    ) -> windows_core::HRESULT,
    pub PaneHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPaneHeader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneCustomContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPaneCustomContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ContentOverlay: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContentOverlay: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsPaneVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPaneVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    SelectionFollowsFocus: usize,
    SetSelectionFollowsFocus: usize,
    TemplateSettings: usize,
    ShoulderNavigationEnabled: usize,
    SetShoulderNavigationEnabled: usize,
    OverflowLabelMode: usize,
    SetOverflowLabelMode: usize,
    Expanding: usize,
    pub RemoveExpanding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Collapsed: usize,
    pub RemoveCollapsed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Expand: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewBackRequestedEventArgs,
    INavigationViewBackRequestedEventArgs_Vtbl,
    0xae752207_bd1b_5afa_a872_e9bbaeea0ede
);
impl windows_core::RuntimeType for INavigationViewBackRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewBackRequestedEventArgs",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewBackRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewFactory,
    INavigationViewFactory_Vtbl,
    0xffea1ada_9232_5507_a320_ed2fadbe6127
);
impl windows_core::RuntimeType for INavigationViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewFactory",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItem,
    INavigationViewItem_Vtbl,
    0x143324cb_bb4c_5261_ad98_a31b4b57a0cc
);
impl windows_core::RuntimeType for INavigationViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItem",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Icon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CompactPaneLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItem2,
    INavigationViewItem2_Vtbl,
    0x2d5bd889_9dac_5675_b254_68226f077a61
);
impl windows_core::RuntimeType for INavigationViewItem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItem2",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectsOnInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSelectsOnInvoked:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub HasUnrealizedChildren:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHasUnrealizedChildren:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsChildSelected:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsChildSelected:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMenuItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItem3,
    INavigationViewItem3_Vtbl,
    0xc6aa3120_d888_5c32_8bb7_490ec91b1aec
);
impl windows_core::RuntimeType for INavigationViewItem3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItem3",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItem3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    InfoBadge: usize,
    SetInfoBadge: usize,
}
windows_core::imp::define_interface!(
    INavigationViewItemBase,
    INavigationViewItemBase_Vtbl,
    0x33586494_af48_513f_be4d_f645e8c89005
);
impl windows_core::RuntimeType for INavigationViewItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemBase",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewItemBase2,
    INavigationViewItemBase2_Vtbl,
    0xd94ee683_d437_5523_9c5c_11d4804e471e
);
impl windows_core::RuntimeType for INavigationViewItemBase2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemBase2",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemBase2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSelected:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSelected:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItemBaseFactory,
    INavigationViewItemBaseFactory_Vtbl,
    0x31b9d7b1_7c38_5916_99c6_c71f6b012b1b
);
impl windows_core::RuntimeType for INavigationViewItemBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemBaseFactory",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewItemBaseStatics,
    INavigationViewItemBaseStatics_Vtbl,
    0x02b75190_e038_5233_9d7d_a60099deb58b
);
impl windows_core::RuntimeType for INavigationViewItemBaseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemBaseStatics",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemBaseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSelectedProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItemFactory,
    INavigationViewItemFactory_Vtbl,
    0xde60a001_9385_5535_80e1_2b68f4bfde26
);
impl windows_core::RuntimeType for INavigationViewItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemFactory",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItemHeader,
    INavigationViewItemHeader_Vtbl,
    0x432bc062_45bc_57ef_a2d3_11851a56a882
);
impl windows_core::RuntimeType for INavigationViewItemHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemHeader",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemHeader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewItemHeaderFactory,
    INavigationViewItemHeaderFactory_Vtbl,
    0x6a5447cd_2918_5fe3_899b_93d6961285e6
);
impl windows_core::RuntimeType for INavigationViewItemHeaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemHeaderFactory",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemHeaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItemStatics,
    INavigationViewItemStatics_Vtbl,
    0x00336d86_c1c5_55bf_a20d_a9f4512363ef
);
impl windows_core::RuntimeType for INavigationViewItemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemStatics",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IconProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CompactPaneLengthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItemStatics2,
    INavigationViewItemStatics2_Vtbl,
    0x3f9f2a50_0737_5f45_8084_90fd628fdb13
);
impl windows_core::RuntimeType for INavigationViewItemStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemStatics2",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectsOnInvokedProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsExpandedProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub HasUnrealizedChildrenProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsChildSelectedProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemsProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemsSourceProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewItemStatics3,
    INavigationViewItemStatics3_Vtbl,
    0xa1524411_3ed8_55d3_904a_0e8f2e87907e
);
impl windows_core::RuntimeType for INavigationViewItemStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewItemStatics3",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InfoBadgeProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewSelectionChangedEventArgs,
    INavigationViewSelectionChangedEventArgs_Vtbl,
    0x14a064a5_c79d_5f63_ac6e_1c313fe63566
);
impl windows_core::RuntimeType for INavigationViewSelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewSelectionChangedEventArgs",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewSelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsSettingsSelected:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewSelectionChangedEventArgs2,
    INavigationViewSelectionChangedEventArgs2_Vtbl,
    0x38daeb2b_5363_51e3_bed4_653641b56336
);
impl windows_core::RuntimeType for INavigationViewSelectionChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewSelectionChangedEventArgs2",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewSelectionChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedItemContainer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RecommendedNavigationTransitionInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewStatics,
    INavigationViewStatics_Vtbl,
    0x21de4b8b_3809_5e91_8506_68222bef82e8
);
impl windows_core::RuntimeType for INavigationViewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewStatics",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPaneOpenProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CompactModeThresholdWidthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ExpandedModeThresholdWidthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FooterMenuItemsProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FooterMenuItemsSourceProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneFooterProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub HeaderProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub HeaderTemplateProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DisplayModeProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsSettingsVisibleProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsPaneToggleButtonVisibleProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AlwaysShowHeaderProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CompactPaneLengthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OpenPaneLengthProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneToggleButtonStyleProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemsProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemsSourceProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SettingsItemProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AutoSuggestBoxProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemTemplateProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemTemplateSelectorProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemContainerStyleProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuItemContainerStyleSelectorProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub IsTitleBarAutoPaddingEnabledProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationViewStatics2,
    INavigationViewStatics2_Vtbl,
    0x23d94681_6ebc_583b_a770_a4475a7441cf
);
impl windows_core::RuntimeType for INavigationViewStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.INavigationViewStatics2",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBackButtonVisibleProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsBackEnabledProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneTitleProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneDisplayModeProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneHeaderProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PaneCustomContentProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ContentOverlayProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsPaneVisibleProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SelectionFollowsFocusProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TemplateSettingsProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ShoulderNavigationEnabledProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OverflowLabelModeProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlControlsResources,
    IXamlControlsResources_Vtbl,
    0x0e35a094_868e_5fbe_a92e_2e224a781dd5
);
impl windows_core::RuntimeType for IXamlControlsResources {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.IXamlControlsResources",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsResources_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IXamlControlsResources3,
    IXamlControlsResources3_Vtbl,
    0x3ba1468f_22ab_520b_8d17_9cbb3eee950c
);
impl windows_core::RuntimeType for IXamlControlsResources3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.IXamlControlsResources3",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsResources3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ControlsResourcesVersion: usize,
    SetControlsResourcesVersion: usize,
}
windows_core::imp::define_interface!(
    IXamlControlsResourcesStatics,
    IXamlControlsResourcesStatics_Vtbl,
    0x8fb9eb9b_850a_5225_96cf_a72036043342
);
impl windows_core::RuntimeType for IXamlControlsResourcesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.IXamlControlsResourcesStatics",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsResourcesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnsureRevealLights: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlControlsResourcesStatics3,
    IXamlControlsResourcesStatics3_Vtbl,
    0x50356110_0b91_52ee_984e_562aff981962
);
impl windows_core::RuntimeType for IXamlControlsResourcesStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.IXamlControlsResourcesStatics3",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsResourcesStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ControlsResourcesVersionProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlControlsXamlMetaDataProvider,
    IXamlControlsXamlMetaDataProvider_Vtbl,
    0x17fa3f58_3472_5aa2_a0f8_1ab8a519573d
);
impl windows_core::RuntimeType for IXamlControlsXamlMetaDataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.XamlTypeInfo.IXamlControlsXamlMetaDataProvider",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IXamlControlsXamlMetaDataProviderStatics,
    IXamlControlsXamlMetaDataProviderStatics_Vtbl,
    0x2d7eb3fd_ecdb_5084_b7e0_12f9598381ef
);
impl windows_core::RuntimeType for IXamlControlsXamlMetaDataProviderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.XamlTypeInfo.IXamlControlsXamlMetaDataProviderStatics",
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProviderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationView,
    xaml_bindings::IAnimationObject,
    xaml_bindings::IVisualElement,
    xaml_bindings::ContentControl,
    xaml_bindings::Control,
    xaml_bindings::FrameworkElement,
    xaml_bindings::UIElement,
    xaml_bindings::DependencyObject
);
impl NavigationView {
    pub fn PopulatePropertyInfo<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        propertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IAnimationObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfo)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                propertyinfo.param().abi(),
            )
            .ok()
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContent)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplate(&self) -> windows_core::Result<xaml_bindings::DataTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateSelector(
        &self,
    ) -> windows_core::Result<xaml_bindings::DataTemplateSelector> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplateSelector<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTransitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTransitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(
        &self,
        oldcontent: P0,
        newcontent: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentChanged)(
                windows_core::Interface::as_raw(this),
                oldcontent.param().abi(),
                newcontent.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateChanged<P0, P1>(
        &self,
        oldcontenttemplate: P0,
        newcontenttemplate: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
        P1: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplate.param().abi(),
                newcontenttemplate.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateSelectorChanged<P0, P1>(
        &self,
        oldcontenttemplateselector: P0,
        newcontenttemplateselector: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
        P1: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateSelectorChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplateselector.param().abi(),
                newcontenttemplateselector.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontFamily(&self) -> windows_core::Result<xaml_bindings::FontFamily> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontFamily)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> windows_core::Result<xaml_bindings::FontWeight> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: xaml_bindings::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontWeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> windows_core::Result<xaml_bindings::FontStyle> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: xaml_bindings::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStyle)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> windows_core::Result<xaml_bindings::FontStretch> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: xaml_bindings::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStretch)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCharacterSpacing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Foreground(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetForeground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTabStop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabIndex)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabNavigation(&self) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Template(&self) -> windows_core::Result<xaml_bindings::ControlTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Template)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ControlTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Padding(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPadding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Background(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BorderBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> windows_core::Result<xaml_bindings::FocusState> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyPropertyChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Focus(&self, value: xaml_bindings::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(
                windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RequiresPointer(&self) -> windows_core::Result<xaml_bindings::RequiresPointer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiresPointer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequiresPointer(
        &self,
        value: xaml_bindings::RequiresPointer,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequiresPointer)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeft)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRight)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUp)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDown)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ElementSoundMode(&self) -> windows_core::Result<xaml_bindings::ElementSoundMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementSoundMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: xaml_bindings::ElementSoundMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetElementSoundMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusEngaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusEngagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusEngaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FocusDisengaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusDisengagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusDisengaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusDisengaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngagement)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<xaml_bindings::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(
        &self,
        value: xaml_bindings::BackgroundSizing,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackgroundSizing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CornerRadius(&self) -> windows_core::Result<xaml_bindings::CornerRadius> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CornerRadius)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCornerRadius(&self, value: xaml_bindings::CornerRadius) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCornerRadius)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerEntered)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerPressed)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerMoved)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerReleased)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerExited)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCanceled)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDoubleTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnHolding)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnRightTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationDelta)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnGotFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnLostFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragEnter<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragEnter)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragLeave<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragLeave)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragOver<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragOver)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDrop<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDrop)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnCharacterReceived)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(
        &self,
        childname: &windows_core::HSTRING,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(childname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ClearValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadLocalValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAnimationBaseValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<xaml_bindings::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::DependencyPropertyChangedCallback>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Triggers(&self) -> windows_core::Result<xaml_bindings::TriggerCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Triggers)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<xaml_bindings::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetResources)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Style(&self) -> windows_core::Result<xaml_bindings::Style> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Style>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetStyle)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlowDirection(&self) -> windows_core::Result<xaml_bindings::FlowDirection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlowDirection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFlowDirection(
        &self,
        value: xaml_bindings::FlowDirection,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFlowDirection)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn SizeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::SizeChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LayoutUpdated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutUpdated)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBinding<P0, P1>(&self, dp: P0, binding: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::BindingBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBinding)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                binding.param().abi(),
            )
            .ok()
        }
    }
    pub fn RequestedTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequestedTheme(
        &self,
        value: xaml_bindings::ElementTheme,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequestedTheme)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DataContextChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::DataContextChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContextChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetBindingExpression<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<xaml_bindings::BindingExpression>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBindingExpression)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loading)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ActualTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualThemeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn EffectiveViewportChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::EffectiveViewportChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn MeasureOverride(
        &self,
        availablesize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(
                windows_core::Interface::as_raw(this),
                availablesize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ArrangeOverride(
        &self,
        finalsize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(
                windows_core::Interface::as_raw(this),
                finalsize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn IsPaneOpen(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPaneOpen)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPaneOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CompactModeThresholdWidth(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompactModeThresholdWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCompactModeThresholdWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCompactModeThresholdWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ExpandedModeThresholdWidth(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpandedModeThresholdWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExpandedModeThresholdWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetExpandedModeThresholdWidth)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn FooterMenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FooterMenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FooterMenuItemsSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FooterMenuItemsSource)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFooterMenuItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFooterMenuItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn PaneFooter(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PaneFooter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPaneFooter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPaneFooter)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Header(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Header)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeader)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn HeaderTemplate(&self) -> windows_core::Result<xaml_bindings::DataTemplate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HeaderTemplate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetHeaderTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetHeaderTemplate)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsSettingsVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSettingsVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsSettingsVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSettingsVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsPaneToggleButtonVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn AlwaysShowHeader(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlwaysShowHeader)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAlwaysShowHeader(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAlwaysShowHeader)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CompactPaneLength(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompactPaneLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCompactPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCompactPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn OpenPaneLength(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenPaneLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpenPaneLength(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOpenPaneLength)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn PaneToggleButtonStyle(&self) -> windows_core::Result<xaml_bindings::Style> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PaneToggleButtonStyle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPaneToggleButtonStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Style>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPaneToggleButtonStyle)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectedItem)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MenuItemsSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItemsSource)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMenuItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMenuItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn SettingsItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SettingsItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoSuggestBox(&self) -> windows_core::Result<xaml_bindings::AutoSuggestBox> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoSuggestBox)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAutoSuggestBox<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::AutoSuggestBox>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetAutoSuggestBox)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MenuItemTemplate(&self) -> windows_core::Result<xaml_bindings::DataTemplate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItemTemplate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMenuItemTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMenuItemTemplate)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MenuItemTemplateSelector(
        &self,
    ) -> windows_core::Result<xaml_bindings::DataTemplateSelector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItemTemplateSelector)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMenuItemTemplateSelector<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMenuItemTemplateSelector)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MenuItemContainerStyle(&self) -> windows_core::Result<xaml_bindings::Style> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItemContainerStyle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMenuItemContainerStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Style>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMenuItemContainerStyle)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MenuItemContainerStyleSelector(
        &self,
    ) -> windows_core::Result<xaml_bindings::StyleSelector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItemContainerStyleSelector)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMenuItemContainerStyleSelector<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::StyleSelector>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMenuItemContainerStyleSelector)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MenuItemFromContainer<P0>(
        &self,
        container: P0,
    ) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuItemFromContainer)(
                windows_core::Interface::as_raw(self),
                container.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContainerFromMenuItem<P0>(
        &self,
        item: P0,
    ) -> windows_core::Result<xaml_bindings::DependencyObject>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainerFromMenuItem)(
                windows_core::Interface::as_raw(self),
                item.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectionChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    NavigationView,
                    NavigationViewSelectionChangedEventArgs,
                >,
            >,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveSelectionChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveSelectionChanged)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveItemInvoked(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveItemInvoked)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveDisplayModeChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveDisplayModeChanged)(
                windows_core::Interface::as_raw(self),
                token,
            )
            .ok()
        }
    }
    pub fn IsTitleBarAutoPaddingEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTitleBarAutoPaddingEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTitleBarAutoPaddingEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsTitleBarAutoPaddingEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsBackButtonVisible(&self) -> windows_core::Result<NavigationViewBackButtonVisible> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBackButtonVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsBackButtonVisible(
        &self,
        value: NavigationViewBackButtonVisible,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsBackButtonVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsBackEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBackEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsBackEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsBackEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PaneTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneTitle)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPaneTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPaneTitle)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn BackRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    NavigationView,
                    NavigationViewBackRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveBackRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBackRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PaneClosed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<NavigationView, windows_core::IInspectable>,
            >,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneClosed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePaneClosed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePaneClosed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemovePaneClosing(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePaneClosing)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PaneOpened<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<NavigationView, windows_core::IInspectable>,
            >,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneOpened)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePaneOpened(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePaneOpened)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PaneOpening<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<NavigationView, windows_core::IInspectable>,
            >,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneOpening)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePaneOpening(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePaneOpening)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PaneDisplayMode(&self) -> windows_core::Result<NavigationViewPaneDisplayMode> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneDisplayMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPaneDisplayMode(
        &self,
        value: NavigationViewPaneDisplayMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPaneDisplayMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PaneHeader(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneHeader)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPaneHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPaneHeader)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn PaneCustomContent(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneCustomContent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPaneCustomContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPaneCustomContent)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentOverlay(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentOverlay)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentOverlay<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentOverlay)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsPaneVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaneVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsPaneVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsPaneVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RemoveExpanding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveExpanding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveCollapsed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCollapsed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Expand<P0>(&self, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<NavigationViewItem>,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Expand)(
                windows_core::Interface::as_raw(this),
                item.param().abi(),
            )
            .ok()
        }
    }
    pub fn Collapse<P0>(&self, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<NavigationViewItem>,
    {
        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Collapse)(
                windows_core::Interface::as_raw(this),
                item.param().abi(),
            )
            .ok()
        }
    }
    pub fn new() -> windows_core::Result<NavigationView> {
        Self::INavigationViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<NavigationView>
    where
        T: windows_core::Compose,
    {
        Self::INavigationViewFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn IsPaneOpenProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaneOpenProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CompactModeThresholdWidthProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompactModeThresholdWidthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ExpandedModeThresholdWidthProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpandedModeThresholdWidthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FooterMenuItemsProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FooterMenuItemsProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FooterMenuItemsSourceProperty() -> windows_core::Result<xaml_bindings::DependencyProperty>
    {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FooterMenuItemsSourceProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PaneFooterProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneFooterProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeaderProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeaderTemplateProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderTemplateProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DisplayModeProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayModeProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSettingsVisibleProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSettingsVisibleProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsPaneToggleButtonVisibleProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaneToggleButtonVisibleProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AlwaysShowHeaderProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlwaysShowHeaderProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CompactPaneLengthProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompactPaneLengthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OpenPaneLengthProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenPaneLengthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PaneToggleButtonStyleProperty() -> windows_core::Result<xaml_bindings::DependencyProperty>
    {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneToggleButtonStyleProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemsProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemsProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemsSourceProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemsSourceProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SelectedItemProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedItemProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SettingsItemProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsItemProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AutoSuggestBoxProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoSuggestBoxProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemTemplateProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemTemplateProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemTemplateSelectorProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemTemplateSelectorProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemContainerStyleProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemContainerStyleProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemContainerStyleSelectorProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemContainerStyleSelectorProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsTitleBarAutoPaddingEnabledProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTitleBarAutoPaddingEnabledProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsBackButtonVisibleProperty() -> windows_core::Result<xaml_bindings::DependencyProperty>
    {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBackButtonVisibleProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsBackEnabledProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBackEnabledProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PaneTitleProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneTitleProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PaneDisplayModeProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneDisplayModeProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PaneHeaderProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneHeaderProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PaneCustomContentProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaneCustomContentProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ContentOverlayProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentOverlayProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsPaneVisibleProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaneVisibleProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SelectionFollowsFocusProperty() -> windows_core::Result<xaml_bindings::DependencyProperty>
    {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionFollowsFocusProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TemplateSettingsProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TemplateSettingsProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShoulderNavigationEnabledProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShoulderNavigationEnabledProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OverflowLabelModeProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OverflowLabelModeProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Clip(&self) -> windows_core::Result<xaml_bindings::RectangleGeometry> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clip)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetClip<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RectangleGeometry>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetClip)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransform(&self) -> windows_core::Result<xaml_bindings::Transform> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransform)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRenderTransform<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransform)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Projection(&self) -> windows_core::Result<xaml_bindings::Projection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Projection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProjection<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Projection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetProjection)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(
        &self,
        value: windows::Foundation::Point,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Visibility(&self) -> windows_core::Result<xaml_bindings::Visibility> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: xaml_bindings::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVisibility)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CacheMode(&self) -> windows_core::Result<xaml_bindings::CacheMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCacheMode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CacheMode>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCacheMode)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ManipulationMode(&self) -> windows_core::Result<xaml_bindings::ManipulationModes> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetManipulationMode(
        &self,
        value: xaml_bindings::ManipulationModes,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetManipulationMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointerCaptures(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<xaml_bindings::Pointer>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragEnter<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragEnter)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragLeave<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragLeave)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragOver<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragOver)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Drop<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Drop)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Tapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::TappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DoubleTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Holding<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::HoldingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Holding)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RightTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationInertiaStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationDelta<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationDelta)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Measure)(
                windows_core::Interface::as_raw(this),
                availablesize,
            )
            .ok()
        }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Arrange)(
                windows_core::Interface::as_raw(this),
                finalrect,
            )
            .ok()
        }
    }
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCapture)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformToVisual<P0>(
        &self,
        visual: P0,
    ) -> windows_core::Result<xaml_bindings::GeneralTransform>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformToVisual)(
                windows_core::Interface::as_raw(this),
                visual.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<xaml_bindings::XamlRoot> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXamlRoot)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn UIContext(&self) -> windows_core::Result<xaml_bindings::UIContext> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Shadow(&self) -> windows_core::Result<xaml_bindings::Shadow> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shadow)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetShadow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Shadow>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetShadow)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CompositeMode(&self) -> windows_core::Result<xaml_bindings::ElementCompositeMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositeMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCompositeMode(
        &self,
        value: xaml_bindings::ElementCompositeMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompositeMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Transform3D(&self) -> windows_core::Result<xaml_bindings::Transform3D> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform3D)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransform3D<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform3D>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransform3D)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DragStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DragStartingEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DropCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DropCompletedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DropCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextFlyout(&self) -> windows_core::Result<xaml_bindings::FlyoutBase> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextFlyout)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContextFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FlyoutBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContextFlyout)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ContextRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ContextRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::RoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayDismissed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayDismissedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyInvoked<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyInvokedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Lights(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::XamlLight>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<xaml_bindings::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipPlacementMode(
        &self,
        value: xaml_bindings::KeyTipPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: xaml_bindings::XYFocusKeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> windows_core::Result<xaml_bindings::ElementHighContrastAdjustment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: xaml_bindings::ElementHighContrastAdjustment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabFocusNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabFocusNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GettingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::GettingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GettingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LosingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::LosingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LosingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn NoFocusCandidateFound<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::NoFocusCandidateFoundEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewOptions>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoViewWithOptions)(
                windows_core::Interface::as_raw(this),
                options.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAccelerators(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::KeyboardAccelerator>>
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CharacterReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::CharacterReceivedRoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceived)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ProcessKeyboardAccelerators<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ProcessKeyboardAcceleratorEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: xaml_bindings::KeyboardAcceleratorPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BringIntoViewRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::BringIntoViewRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OpacityTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpacityTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOpacityTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacityTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranslationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTranslationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRotationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetScaleTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScaleTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn StartAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StopAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCreateAutomationPeer(&self) -> windows_core::Result<xaml_bindings::AutomationPeer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnCreateAutomationPeer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn FindSubElementsForTouchTargeting(
        &self,
        point: windows::Foundation::Point,
        boundingrect: windows::Foundation::Rect,
    ) -> windows_core::Result<
        windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>,
    > {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(
                windows_core::Interface::as_raw(this),
                point,
                boundingrect,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<xaml_bindings::DependencyObject>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn PopulatePropertyInfoOverride<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        animationpropertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                animationpropertyinfo.param().abi(),
            )
            .ok()
        }
    }
    fn INavigationViewFactory<R, F: FnOnce(&INavigationViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn INavigationViewStatics<R, F: FnOnce(&INavigationViewStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn INavigationViewStatics2<
        R,
        F: FnOnce(&INavigationViewStatics2) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewStatics2> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NavigationView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationView>();
}
unsafe impl windows_core::Interface for NavigationView {
    type Vtable = <INavigationView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NavigationView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationView";
}
unsafe impl Send for NavigationView {}
unsafe impl Sync for NavigationView {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NavigationViewBackButtonVisible(pub i32);
impl NavigationViewBackButtonVisible {
    pub const Collapsed: Self = Self(0);
    pub const Visible: Self = Self(1);
    pub const Auto: Self = Self(2);
}
impl windows_core::TypeKind for NavigationViewBackButtonVisible {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NavigationViewBackButtonVisible {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.NavigationViewBackButtonVisible;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.NavigationViewBackButtonVisible",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewBackRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewBackRequestedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for NavigationViewBackRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewBackRequestedEventArgs>();
}
unsafe impl windows_core::Interface for NavigationViewBackRequestedEventArgs {
    type Vtable = <INavigationViewBackRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INavigationViewBackRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NavigationViewBackRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewBackRequestedEventArgs";
}
unsafe impl Send for NavigationViewBackRequestedEventArgs {}
unsafe impl Sync for NavigationViewBackRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationViewItem,
    xaml_bindings::IAnimationObject,
    xaml_bindings::IVisualElement,
    NavigationViewItemBase,
    xaml_bindings::ContentControl,
    xaml_bindings::Control,
    xaml_bindings::FrameworkElement,
    xaml_bindings::UIElement,
    xaml_bindings::DependencyObject
);
impl NavigationViewItem {
    pub fn PopulatePropertyInfo<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        propertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IAnimationObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfo)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                propertyinfo.param().abi(),
            )
            .ok()
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContent)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplate(&self) -> windows_core::Result<xaml_bindings::DataTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateSelector(
        &self,
    ) -> windows_core::Result<xaml_bindings::DataTemplateSelector> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplateSelector<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTransitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTransitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(
        &self,
        oldcontent: P0,
        newcontent: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentChanged)(
                windows_core::Interface::as_raw(this),
                oldcontent.param().abi(),
                newcontent.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateChanged<P0, P1>(
        &self,
        oldcontenttemplate: P0,
        newcontenttemplate: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
        P1: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplate.param().abi(),
                newcontenttemplate.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateSelectorChanged<P0, P1>(
        &self,
        oldcontenttemplateselector: P0,
        newcontenttemplateselector: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
        P1: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateSelectorChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplateselector.param().abi(),
                newcontenttemplateselector.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontFamily(&self) -> windows_core::Result<xaml_bindings::FontFamily> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontFamily)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> windows_core::Result<xaml_bindings::FontWeight> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: xaml_bindings::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontWeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> windows_core::Result<xaml_bindings::FontStyle> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: xaml_bindings::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStyle)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> windows_core::Result<xaml_bindings::FontStretch> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: xaml_bindings::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStretch)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCharacterSpacing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Foreground(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetForeground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTabStop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabIndex)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabNavigation(&self) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Template(&self) -> windows_core::Result<xaml_bindings::ControlTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Template)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ControlTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Padding(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPadding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Background(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BorderBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> windows_core::Result<xaml_bindings::FocusState> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyPropertyChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Focus(&self, value: xaml_bindings::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(
                windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RequiresPointer(&self) -> windows_core::Result<xaml_bindings::RequiresPointer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiresPointer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequiresPointer(
        &self,
        value: xaml_bindings::RequiresPointer,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequiresPointer)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeft)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRight)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUp)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDown)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ElementSoundMode(&self) -> windows_core::Result<xaml_bindings::ElementSoundMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementSoundMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: xaml_bindings::ElementSoundMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetElementSoundMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusEngaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusEngagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusEngaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FocusDisengaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusDisengagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusDisengaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusDisengaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngagement)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<xaml_bindings::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(
        &self,
        value: xaml_bindings::BackgroundSizing,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackgroundSizing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CornerRadius(&self) -> windows_core::Result<xaml_bindings::CornerRadius> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CornerRadius)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCornerRadius(&self, value: xaml_bindings::CornerRadius) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCornerRadius)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerEntered)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerPressed)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerMoved)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerReleased)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerExited)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCanceled)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDoubleTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnHolding)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnRightTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationDelta)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnGotFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnLostFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragEnter<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragEnter)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragLeave<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragLeave)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragOver<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragOver)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDrop<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDrop)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnCharacterReceived)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(
        &self,
        childname: &windows_core::HSTRING,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(childname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ClearValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadLocalValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAnimationBaseValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<xaml_bindings::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::DependencyPropertyChangedCallback>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Triggers(&self) -> windows_core::Result<xaml_bindings::TriggerCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Triggers)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<xaml_bindings::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetResources)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Style(&self) -> windows_core::Result<xaml_bindings::Style> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Style>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetStyle)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlowDirection(&self) -> windows_core::Result<xaml_bindings::FlowDirection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlowDirection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFlowDirection(
        &self,
        value: xaml_bindings::FlowDirection,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFlowDirection)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn SizeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::SizeChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LayoutUpdated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutUpdated)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBinding<P0, P1>(&self, dp: P0, binding: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::BindingBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBinding)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                binding.param().abi(),
            )
            .ok()
        }
    }
    pub fn RequestedTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequestedTheme(
        &self,
        value: xaml_bindings::ElementTheme,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequestedTheme)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DataContextChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::DataContextChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContextChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetBindingExpression<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<xaml_bindings::BindingExpression>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBindingExpression)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loading)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ActualTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualThemeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn EffectiveViewportChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::EffectiveViewportChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn MeasureOverride(
        &self,
        availablesize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(
                windows_core::Interface::as_raw(this),
                availablesize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ArrangeOverride(
        &self,
        finalsize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(
                windows_core::Interface::as_raw(this),
                finalsize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Icon(&self) -> windows_core::Result<xaml_bindings::IconElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Icon)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CompactPaneLength(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompactPaneLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SelectsOnInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectsOnInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSelectsOnInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetSelectsOnInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsExpanded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExpanded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsExpanded(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsExpanded)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HasUnrealizedChildren(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasUnrealizedChildren)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHasUnrealizedChildren(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHasUnrealizedChildren)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsChildSelected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsChildSelected)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsChildSelected(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsChildSelected)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItems)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MenuItemsSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemsSource)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMenuItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMenuItemsSource)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsSelected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationViewItemBase2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSelected)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationViewItemBase2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsSelected)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn new() -> windows_core::Result<NavigationViewItem> {
        Self::INavigationViewItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<NavigationViewItem>
    where
        T: windows_core::Compose,
    {
        Self::INavigationViewItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn IconProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CompactPaneLengthProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompactPaneLengthProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SelectsOnInvokedProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectsOnInvokedProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsExpandedProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExpandedProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HasUnrealizedChildrenProperty() -> windows_core::Result<xaml_bindings::DependencyProperty>
    {
        Self::INavigationViewItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasUnrealizedChildrenProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsChildSelectedProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsChildSelectedProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemsProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemsProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MenuItemsSourceProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MenuItemsSourceProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InfoBadgeProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InfoBadgeProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Clip(&self) -> windows_core::Result<xaml_bindings::RectangleGeometry> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clip)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetClip<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RectangleGeometry>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetClip)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransform(&self) -> windows_core::Result<xaml_bindings::Transform> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransform)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRenderTransform<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransform)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Projection(&self) -> windows_core::Result<xaml_bindings::Projection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Projection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProjection<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Projection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetProjection)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(
        &self,
        value: windows::Foundation::Point,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Visibility(&self) -> windows_core::Result<xaml_bindings::Visibility> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: xaml_bindings::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVisibility)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CacheMode(&self) -> windows_core::Result<xaml_bindings::CacheMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCacheMode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CacheMode>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCacheMode)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ManipulationMode(&self) -> windows_core::Result<xaml_bindings::ManipulationModes> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetManipulationMode(
        &self,
        value: xaml_bindings::ManipulationModes,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetManipulationMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointerCaptures(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<xaml_bindings::Pointer>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragEnter<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragEnter)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragLeave<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragLeave)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragOver<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragOver)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Drop<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Drop)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Tapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::TappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DoubleTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Holding<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::HoldingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Holding)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RightTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationInertiaStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationDelta<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationDelta)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Measure)(
                windows_core::Interface::as_raw(this),
                availablesize,
            )
            .ok()
        }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Arrange)(
                windows_core::Interface::as_raw(this),
                finalrect,
            )
            .ok()
        }
    }
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCapture)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformToVisual<P0>(
        &self,
        visual: P0,
    ) -> windows_core::Result<xaml_bindings::GeneralTransform>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformToVisual)(
                windows_core::Interface::as_raw(this),
                visual.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<xaml_bindings::XamlRoot> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXamlRoot)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn UIContext(&self) -> windows_core::Result<xaml_bindings::UIContext> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Shadow(&self) -> windows_core::Result<xaml_bindings::Shadow> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shadow)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetShadow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Shadow>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetShadow)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CompositeMode(&self) -> windows_core::Result<xaml_bindings::ElementCompositeMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositeMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCompositeMode(
        &self,
        value: xaml_bindings::ElementCompositeMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompositeMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Transform3D(&self) -> windows_core::Result<xaml_bindings::Transform3D> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform3D)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransform3D<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform3D>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransform3D)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DragStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DragStartingEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DropCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DropCompletedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DropCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextFlyout(&self) -> windows_core::Result<xaml_bindings::FlyoutBase> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextFlyout)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContextFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FlyoutBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContextFlyout)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ContextRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ContextRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::RoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayDismissed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayDismissedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyInvoked<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyInvokedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Lights(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::XamlLight>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<xaml_bindings::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipPlacementMode(
        &self,
        value: xaml_bindings::KeyTipPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: xaml_bindings::XYFocusKeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> windows_core::Result<xaml_bindings::ElementHighContrastAdjustment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: xaml_bindings::ElementHighContrastAdjustment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabFocusNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabFocusNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GettingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::GettingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GettingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LosingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::LosingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LosingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn NoFocusCandidateFound<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::NoFocusCandidateFoundEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewOptions>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoViewWithOptions)(
                windows_core::Interface::as_raw(this),
                options.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAccelerators(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::KeyboardAccelerator>>
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CharacterReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::CharacterReceivedRoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceived)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ProcessKeyboardAccelerators<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ProcessKeyboardAcceleratorEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: xaml_bindings::KeyboardAcceleratorPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BringIntoViewRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::BringIntoViewRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OpacityTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpacityTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOpacityTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacityTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranslationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTranslationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRotationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetScaleTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScaleTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn StartAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StopAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCreateAutomationPeer(&self) -> windows_core::Result<xaml_bindings::AutomationPeer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnCreateAutomationPeer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn FindSubElementsForTouchTargeting(
        &self,
        point: windows::Foundation::Point,
        boundingrect: windows::Foundation::Rect,
    ) -> windows_core::Result<
        windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>,
    > {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(
                windows_core::Interface::as_raw(this),
                point,
                boundingrect,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<xaml_bindings::DependencyObject>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn PopulatePropertyInfoOverride<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        animationpropertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                animationpropertyinfo.param().abi(),
            )
            .ok()
        }
    }
    fn INavigationViewItemFactory<
        R,
        F: FnOnce(&INavigationViewItemFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItem,
            INavigationViewItemFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn INavigationViewItemStatics<
        R,
        F: FnOnce(&INavigationViewItemStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItem,
            INavigationViewItemStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn INavigationViewItemStatics2<
        R,
        F: FnOnce(&INavigationViewItemStatics2) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItem,
            INavigationViewItemStatics2,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn INavigationViewItemStatics3<
        R,
        F: FnOnce(&INavigationViewItemStatics3) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItem,
            INavigationViewItemStatics3,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NavigationViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItem>();
}
unsafe impl windows_core::Interface for NavigationViewItem {
    type Vtable = <INavigationViewItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NavigationViewItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItem";
}
unsafe impl Send for NavigationViewItem {}
unsafe impl Sync for NavigationViewItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItemBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewItemBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationViewItemBase,
    xaml_bindings::IAnimationObject,
    xaml_bindings::IVisualElement,
    xaml_bindings::ContentControl,
    xaml_bindings::Control,
    xaml_bindings::FrameworkElement,
    xaml_bindings::UIElement,
    xaml_bindings::DependencyObject
);
impl NavigationViewItemBase {
    pub fn PopulatePropertyInfo<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        propertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IAnimationObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfo)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                propertyinfo.param().abi(),
            )
            .ok()
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContent)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplate(&self) -> windows_core::Result<xaml_bindings::DataTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateSelector(
        &self,
    ) -> windows_core::Result<xaml_bindings::DataTemplateSelector> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplateSelector<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTransitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTransitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(
        &self,
        oldcontent: P0,
        newcontent: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentChanged)(
                windows_core::Interface::as_raw(this),
                oldcontent.param().abi(),
                newcontent.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateChanged<P0, P1>(
        &self,
        oldcontenttemplate: P0,
        newcontenttemplate: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
        P1: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplate.param().abi(),
                newcontenttemplate.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateSelectorChanged<P0, P1>(
        &self,
        oldcontenttemplateselector: P0,
        newcontenttemplateselector: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
        P1: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateSelectorChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplateselector.param().abi(),
                newcontenttemplateselector.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontFamily(&self) -> windows_core::Result<xaml_bindings::FontFamily> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontFamily)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> windows_core::Result<xaml_bindings::FontWeight> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: xaml_bindings::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontWeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> windows_core::Result<xaml_bindings::FontStyle> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: xaml_bindings::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStyle)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> windows_core::Result<xaml_bindings::FontStretch> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: xaml_bindings::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStretch)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCharacterSpacing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Foreground(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetForeground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTabStop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabIndex)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabNavigation(&self) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Template(&self) -> windows_core::Result<xaml_bindings::ControlTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Template)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ControlTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Padding(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPadding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Background(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BorderBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> windows_core::Result<xaml_bindings::FocusState> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyPropertyChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Focus(&self, value: xaml_bindings::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(
                windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RequiresPointer(&self) -> windows_core::Result<xaml_bindings::RequiresPointer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiresPointer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequiresPointer(
        &self,
        value: xaml_bindings::RequiresPointer,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequiresPointer)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeft)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRight)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUp)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDown)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ElementSoundMode(&self) -> windows_core::Result<xaml_bindings::ElementSoundMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementSoundMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: xaml_bindings::ElementSoundMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetElementSoundMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusEngaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusEngagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusEngaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FocusDisengaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusDisengagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusDisengaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusDisengaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngagement)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<xaml_bindings::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(
        &self,
        value: xaml_bindings::BackgroundSizing,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackgroundSizing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CornerRadius(&self) -> windows_core::Result<xaml_bindings::CornerRadius> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CornerRadius)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCornerRadius(&self, value: xaml_bindings::CornerRadius) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCornerRadius)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerEntered)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerPressed)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerMoved)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerReleased)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerExited)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCanceled)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDoubleTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnHolding)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnRightTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationDelta)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnGotFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnLostFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragEnter<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragEnter)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragLeave<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragLeave)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragOver<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragOver)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDrop<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDrop)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnCharacterReceived)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(
        &self,
        childname: &windows_core::HSTRING,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(childname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ClearValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadLocalValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAnimationBaseValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<xaml_bindings::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::DependencyPropertyChangedCallback>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Triggers(&self) -> windows_core::Result<xaml_bindings::TriggerCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Triggers)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<xaml_bindings::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetResources)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Style(&self) -> windows_core::Result<xaml_bindings::Style> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Style>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetStyle)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlowDirection(&self) -> windows_core::Result<xaml_bindings::FlowDirection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlowDirection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFlowDirection(
        &self,
        value: xaml_bindings::FlowDirection,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFlowDirection)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn SizeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::SizeChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LayoutUpdated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutUpdated)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBinding<P0, P1>(&self, dp: P0, binding: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::BindingBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBinding)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                binding.param().abi(),
            )
            .ok()
        }
    }
    pub fn RequestedTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequestedTheme(
        &self,
        value: xaml_bindings::ElementTheme,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequestedTheme)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DataContextChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::DataContextChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContextChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetBindingExpression<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<xaml_bindings::BindingExpression>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBindingExpression)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loading)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ActualTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualThemeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn EffectiveViewportChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::EffectiveViewportChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn MeasureOverride(
        &self,
        availablesize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(
                windows_core::Interface::as_raw(this),
                availablesize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ArrangeOverride(
        &self,
        finalsize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(
                windows_core::Interface::as_raw(this),
                finalsize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn IsSelected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationViewItemBase2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSelected)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationViewItemBase2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsSelected)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSelectedProperty() -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::INavigationViewItemBaseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSelectedProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Clip(&self) -> windows_core::Result<xaml_bindings::RectangleGeometry> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clip)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetClip<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RectangleGeometry>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetClip)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransform(&self) -> windows_core::Result<xaml_bindings::Transform> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransform)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRenderTransform<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransform)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Projection(&self) -> windows_core::Result<xaml_bindings::Projection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Projection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProjection<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Projection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetProjection)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(
        &self,
        value: windows::Foundation::Point,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Visibility(&self) -> windows_core::Result<xaml_bindings::Visibility> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: xaml_bindings::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVisibility)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CacheMode(&self) -> windows_core::Result<xaml_bindings::CacheMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCacheMode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CacheMode>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCacheMode)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ManipulationMode(&self) -> windows_core::Result<xaml_bindings::ManipulationModes> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetManipulationMode(
        &self,
        value: xaml_bindings::ManipulationModes,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetManipulationMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointerCaptures(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<xaml_bindings::Pointer>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragEnter<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragEnter)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragLeave<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragLeave)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragOver<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragOver)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Drop<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Drop)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Tapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::TappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DoubleTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Holding<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::HoldingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Holding)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RightTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationInertiaStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationDelta<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationDelta)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Measure)(
                windows_core::Interface::as_raw(this),
                availablesize,
            )
            .ok()
        }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Arrange)(
                windows_core::Interface::as_raw(this),
                finalrect,
            )
            .ok()
        }
    }
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCapture)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformToVisual<P0>(
        &self,
        visual: P0,
    ) -> windows_core::Result<xaml_bindings::GeneralTransform>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformToVisual)(
                windows_core::Interface::as_raw(this),
                visual.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<xaml_bindings::XamlRoot> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXamlRoot)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn UIContext(&self) -> windows_core::Result<xaml_bindings::UIContext> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Shadow(&self) -> windows_core::Result<xaml_bindings::Shadow> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shadow)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetShadow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Shadow>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetShadow)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CompositeMode(&self) -> windows_core::Result<xaml_bindings::ElementCompositeMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositeMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCompositeMode(
        &self,
        value: xaml_bindings::ElementCompositeMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompositeMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Transform3D(&self) -> windows_core::Result<xaml_bindings::Transform3D> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform3D)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransform3D<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform3D>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransform3D)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DragStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DragStartingEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DropCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DropCompletedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DropCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextFlyout(&self) -> windows_core::Result<xaml_bindings::FlyoutBase> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextFlyout)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContextFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FlyoutBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContextFlyout)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ContextRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ContextRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::RoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayDismissed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayDismissedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyInvoked<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyInvokedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Lights(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::XamlLight>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<xaml_bindings::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipPlacementMode(
        &self,
        value: xaml_bindings::KeyTipPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: xaml_bindings::XYFocusKeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> windows_core::Result<xaml_bindings::ElementHighContrastAdjustment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: xaml_bindings::ElementHighContrastAdjustment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabFocusNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabFocusNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GettingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::GettingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GettingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LosingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::LosingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LosingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn NoFocusCandidateFound<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::NoFocusCandidateFoundEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewOptions>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoViewWithOptions)(
                windows_core::Interface::as_raw(this),
                options.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAccelerators(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::KeyboardAccelerator>>
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CharacterReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::CharacterReceivedRoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceived)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ProcessKeyboardAccelerators<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ProcessKeyboardAcceleratorEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: xaml_bindings::KeyboardAcceleratorPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BringIntoViewRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::BringIntoViewRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OpacityTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpacityTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOpacityTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacityTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranslationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTranslationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRotationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetScaleTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScaleTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn StartAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StopAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCreateAutomationPeer(&self) -> windows_core::Result<xaml_bindings::AutomationPeer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnCreateAutomationPeer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn FindSubElementsForTouchTargeting(
        &self,
        point: windows::Foundation::Point,
        boundingrect: windows::Foundation::Rect,
    ) -> windows_core::Result<
        windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>,
    > {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(
                windows_core::Interface::as_raw(this),
                point,
                boundingrect,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<xaml_bindings::DependencyObject>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn PopulatePropertyInfoOverride<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        animationpropertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                animationpropertyinfo.param().abi(),
            )
            .ok()
        }
    }
    fn INavigationViewItemBaseStatics<
        R,
        F: FnOnce(&INavigationViewItemBaseStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItemBase,
            INavigationViewItemBaseStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NavigationViewItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItemBase>();
}
unsafe impl windows_core::Interface for NavigationViewItemBase {
    type Vtable = <INavigationViewItemBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItemBase as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NavigationViewItemBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItemBase";
}
unsafe impl Send for NavigationViewItemBase {}
unsafe impl Sync for NavigationViewItemBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItemHeader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewItemHeader,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    NavigationViewItemHeader,
    xaml_bindings::IAnimationObject,
    xaml_bindings::IVisualElement,
    NavigationViewItemBase,
    xaml_bindings::ContentControl,
    xaml_bindings::Control,
    xaml_bindings::FrameworkElement,
    xaml_bindings::UIElement,
    xaml_bindings::DependencyObject
);
impl NavigationViewItemHeader {
    pub fn PopulatePropertyInfo<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        propertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IAnimationObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfo)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                propertyinfo.param().abi(),
            )
            .ok()
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContent)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplate(&self) -> windows_core::Result<xaml_bindings::DataTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateSelector(
        &self,
    ) -> windows_core::Result<xaml_bindings::DataTemplateSelector> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTemplateSelector<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTemplateSelector)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTransitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTransitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContentTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<xaml_bindings::UIElement> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(
        &self,
        oldcontent: P0,
        newcontent: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentChanged)(
                windows_core::Interface::as_raw(this),
                oldcontent.param().abi(),
                newcontent.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateChanged<P0, P1>(
        &self,
        oldcontenttemplate: P0,
        newcontenttemplate: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplate>,
        P1: windows_core::Param<xaml_bindings::DataTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplate.param().abi(),
                newcontenttemplate.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnContentTemplateSelectorChanged<P0, P1>(
        &self,
        oldcontenttemplateselector: P0,
        newcontenttemplateselector: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DataTemplateSelector>,
        P1: windows_core::Param<xaml_bindings::DataTemplateSelector>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IContentControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnContentTemplateSelectorChanged)(
                windows_core::Interface::as_raw(this),
                oldcontenttemplateselector.param().abi(),
                newcontenttemplateselector.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontSize)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontFamily(&self) -> windows_core::Result<xaml_bindings::FontFamily> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontFamily)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> windows_core::Result<xaml_bindings::FontWeight> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: xaml_bindings::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontWeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> windows_core::Result<xaml_bindings::FontStyle> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: xaml_bindings::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStyle)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> windows_core::Result<xaml_bindings::FontStretch> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: xaml_bindings::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFontStretch)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCharacterSpacing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Foreground(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetForeground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTabStop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabIndex)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabNavigation(&self) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Template(&self) -> windows_core::Result<xaml_bindings::ControlTemplate> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Template)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTemplate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ControlTemplate>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTemplate)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Padding(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetPadding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalContentAlignment(
        &self,
    ) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Background(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackground)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BorderBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBorderBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> windows_core::Result<xaml_bindings::FocusState> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyPropertyChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Focus(&self, value: xaml_bindings::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(
                windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsFocusEngaged)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RequiresPointer(&self) -> windows_core::Result<xaml_bindings::RequiresPointer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiresPointer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequiresPointer(
        &self,
        value: xaml_bindings::RequiresPointer,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequiresPointer)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeft)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRight)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUp)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDown)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ElementSoundMode(&self) -> windows_core::Result<xaml_bindings::ElementSoundMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementSoundMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: xaml_bindings::ElementSoundMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetElementSoundMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusEngaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusEngagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusEngaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusEngaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FocusDisengaged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::Control,
                    xaml_bindings::FocusDisengagedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusDisengaged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveFocusDisengaged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusDisengaged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveFocusEngagement)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<xaml_bindings::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(
        &self,
        value: xaml_bindings::BackgroundSizing,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBackgroundSizing)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CornerRadius(&self) -> windows_core::Result<xaml_bindings::CornerRadius> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CornerRadius)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCornerRadius(&self, value: xaml_bindings::CornerRadius) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControl7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCornerRadius)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerEntered)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerPressed)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerMoved)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerReleased)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerExited)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerCanceled)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDoubleTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnHolding)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnRightTapped)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationStarted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationDelta)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnGotFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnLostFocus)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragEnter<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragEnter)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragLeave<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragLeave)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDragOver<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDragOver)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnDrop<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DragEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDrop)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnPreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlOverrides6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnCharacterReceived)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(
        &self,
        childname: &windows_core::HSTRING,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(childname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ClearValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadLocalValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAnimationBaseValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<xaml_bindings::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::DependencyPropertyChangedCallback>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Triggers(&self) -> windows_core::Result<xaml_bindings::TriggerCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Triggers)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<xaml_bindings::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetResources)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTag)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetLanguage)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxWidth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMaxHeight)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<xaml_bindings::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: xaml_bindings::HorizontalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHorizontalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<xaml_bindings::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: xaml_bindings::VerticalAlignment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVerticalAlignment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: xaml_bindings::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetDataContext)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Style(&self) -> windows_core::Result<xaml_bindings::Style> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Style>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetStyle)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlowDirection(&self) -> windows_core::Result<xaml_bindings::FlowDirection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlowDirection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFlowDirection(
        &self,
        value: xaml_bindings::FlowDirection,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFlowDirection)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unloaded)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveUnloaded(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveUnloaded)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn SizeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::SizeChangedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveSizeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveSizeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LayoutUpdated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutUpdated)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLayoutUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBinding<P0, P1>(&self, dp: P0, binding: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::BindingBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetBinding)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                binding.param().abi(),
            )
            .ok()
        }
    }
    pub fn RequestedTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequestedTheme(
        &self,
        value: xaml_bindings::ElementTheme,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRequestedTheme)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DataContextChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::DataContextChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContextChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDataContextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetBindingExpression<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<xaml_bindings::BindingExpression>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBindingExpression)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Loading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loading)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLoading)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualMargin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<xaml_bindings::Thickness> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(
        &self,
        value: xaml_bindings::Thickness,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<xaml_bindings::Brush> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Brush>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ActualTheme(&self) -> windows_core::Result<xaml_bindings::ElementTheme> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualTheme)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualThemeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    windows_core::IInspectable,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveActualThemeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement6>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn EffectiveViewportChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::FrameworkElement,
                    xaml_bindings::EffectiveViewportChangedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IFrameworkElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn MeasureOverride(
        &self,
        availablesize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(
                windows_core::Interface::as_raw(this),
                availablesize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ArrangeOverride(
        &self,
        finalsize: windows::Foundation::Size,
    ) -> windows_core::Result<windows::Foundation::Size> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(
                windows_core::Interface::as_raw(this),
                finalsize,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnApplyTemplate)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &windows_core::HSTRING,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementOverrides2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this =
            &windows_core::Interface::cast::<xaml_bindings::IFrameworkElementProtected7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateViewport)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn IsSelected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INavigationViewItemBase2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSelected)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INavigationViewItemBase2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsSelected)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn new() -> windows_core::Result<NavigationViewItemHeader> {
        Self::INavigationViewItemHeaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<NavigationViewItemHeader>
    where
        T: windows_core::Compose,
    {
        Self::INavigationViewItemHeaderFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAllowDrop)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacity)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Clip(&self) -> windows_core::Result<xaml_bindings::RectangleGeometry> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clip)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetClip<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RectangleGeometry>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetClip)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransform(&self) -> windows_core::Result<xaml_bindings::Transform> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransform)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRenderTransform<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransform)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Projection(&self) -> windows_core::Result<xaml_bindings::Projection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Projection)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProjection<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Projection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetProjection)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(
        &self,
        value: windows::Foundation::Point,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Visibility(&self) -> windows_core::Result<xaml_bindings::Visibility> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: xaml_bindings::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetVisibility)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transitions(&self) -> windows_core::Result<xaml_bindings::TransitionCollection> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transitions)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransitions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::TransitionCollection>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransitions)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CacheMode(&self) -> windows_core::Result<xaml_bindings::CacheMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCacheMode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::CacheMode>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCacheMode)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ManipulationMode(&self) -> windows_core::Result<xaml_bindings::ManipulationModes> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetManipulationMode(
        &self,
        value: xaml_bindings::ManipulationModes,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetManipulationMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointerCaptures(
        &self,
    ) -> windows_core::Result<windows_collections::IVectorView<xaml_bindings::Pointer>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGotFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLostFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragEnter<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragEnter)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragEnter(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragEnter)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragLeave<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragLeave)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragLeave(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragLeave)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragOver<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragOver)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragOver(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragOver)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Drop<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DragEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Drop)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDrop(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDrop)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerPressed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerMoved)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerReleased)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerEntered)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerExited)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::PointerEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Tapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::TappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DoubleTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DoubleTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDoubleTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Holding<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::HoldingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Holding)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveHolding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHolding)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RightTapped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::RightTappedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightTapped)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveRightTapped)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationInertiaStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationInertiaStartingEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationStarted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationStartedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationDelta<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationDeltaEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationDelta)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationDelta(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ManipulationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::ManipulationCompletedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Measure)(
                windows_core::Interface::as_raw(this),
                availablesize,
            )
            .ok()
        }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Arrange)(
                windows_core::Interface::as_raw(this),
                finalrect,
            )
            .ok()
        }
    }
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Pointer>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCapture)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).AddHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::RoutedEvent>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveHandler)(
                windows_core::Interface::as_raw(this),
                routedevent.param().abi(),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformToVisual<P0>(
        &self,
        visual: P0,
    ) -> windows_core::Result<xaml_bindings::GeneralTransform>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformToVisual)(
                windows_core::Interface::as_raw(this),
                visual.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateMeasure)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).InvalidateArrange)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<xaml_bindings::XamlRoot> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXamlRoot)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn UIContext(&self) -> windows_core::Result<xaml_bindings::UIContext> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Shadow(&self) -> windows_core::Result<xaml_bindings::Shadow> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shadow)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetShadow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Shadow>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement10>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetShadow)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CompositeMode(&self) -> windows_core::Result<xaml_bindings::ElementCompositeMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositeMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCompositeMode(
        &self,
        value: xaml_bindings::ElementCompositeMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompositeMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Transform3D(&self) -> windows_core::Result<xaml_bindings::Transform3D> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform3D)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransform3D<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Transform3D>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransform3D)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanDrag)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DragStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DragStartingEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragStarting)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDragStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDragStarting)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DropCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::DropCompletedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DropCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveDropCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement3>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveDropCompleted)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextFlyout(&self) -> windows_core::Result<xaml_bindings::FlyoutBase> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextFlyout)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContextFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::FlyoutBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetContextFlyout)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetAccessKey)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ContextRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ContextRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextCanceled<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::RoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContextCanceled)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveContextCanceled(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveContextCanceled)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyDisplayDismissed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyDisplayDismissedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AccessKeyInvoked<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::AccessKeyInvokedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement4>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Lights(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::XamlLight>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<xaml_bindings::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipPlacementMode(
        &self,
        value: xaml_bindings::KeyTipPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: xaml_bindings::XYFocusKeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> windows_core::Result<xaml_bindings::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: xaml_bindings::XYFocusNavigationStrategy,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> windows_core::Result<xaml_bindings::ElementHighContrastAdjustment> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: xaml_bindings::ElementHighContrastAdjustment,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetHighContrastAdjustment)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabFocusNavigation(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTabFocusNavigation(
        &self,
        value: xaml_bindings::KeyboardNavigationMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTabFocusNavigation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GettingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::GettingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GettingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveGettingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveGettingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LosingFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::LosingFocusEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LosingFocus)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveLosingFocus(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveLosingFocus)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn NoFocusCandidateFound<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::NoFocusCandidateFoundEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveNoFocusCandidateFound(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoView)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewOptions>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement5>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartBringIntoViewWithOptions)(
                windows_core::Interface::as_raw(this),
                options.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAccelerators(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::KeyboardAccelerator>>
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CharacterReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::CharacterReceivedRoutedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceived)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ProcessKeyboardAccelerators<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::ProcessKeyboardAcceleratorEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyDown<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyDown(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PreviewKeyUp<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::KeyEventHandler>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemovePreviewKeyUp(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> windows_core::Result<xaml_bindings::DependencyObject> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> windows_core::Result<xaml_bindings::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: xaml_bindings::KeyboardAcceleratorPlacementMode,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BringIntoViewRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<
                windows::Foundation::TypedEventHandler<
                    xaml_bindings::UIElement,
                    xaml_bindings::BringIntoViewRequestedEventArgs,
                >,
            >,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveBringIntoViewRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OpacityTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpacityTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOpacityTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetOpacityTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranslationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTranslationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTranslationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotation)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationTransition(&self) -> windows_core::Result<xaml_bindings::ScalarTransition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRotationTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ScalarTransition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScale)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> windows_core::Result<xaml_bindings::Vector3Transition> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleTransition)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetScaleTransition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::Vector3Transition>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetScaleTransition)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: windows_numerics::Matrix4x4,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetTransformMatrix)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCenterPoint)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetRotationAxis)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn StartAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StartAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElement9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).StopAnimation)(
                windows_core::Interface::as_raw(this),
                animation.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnCreateAutomationPeer(&self) -> windows_core::Result<xaml_bindings::AutomationPeer> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnCreateAutomationPeer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn FindSubElementsForTouchTargeting(
        &self,
        point: windows::Foundation::Point,
        boundingrect: windows::Foundation::Rect,
    ) -> windows_core::Result<
        windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>,
    > {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(
                windows_core::Interface::as_raw(this),
                point,
                boundingrect,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<xaml_bindings::DependencyObject>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides7>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(
                windows_core::Interface::as_raw(this),
                args.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides8>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(
                windows_core::Interface::as_raw(this),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn PopulatePropertyInfoOverride<P1>(
        &self,
        propertyname: &windows_core::HSTRING,
        animationpropertyinfo: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<xaml_bindings::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IUIElementOverrides9>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(propertyname),
                animationpropertyinfo.param().abi(),
            )
            .ok()
        }
    }
    fn INavigationViewItemHeaderFactory<
        R,
        F: FnOnce(&INavigationViewItemHeaderFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            NavigationViewItemHeader,
            INavigationViewItemHeaderFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NavigationViewItemHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItemHeader>();
}
unsafe impl windows_core::Interface for NavigationViewItemHeader {
    type Vtable = <INavigationViewItemHeader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItemHeader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NavigationViewItemHeader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItemHeader";
}
unsafe impl Send for NavigationViewItemHeader {}
unsafe impl Sync for NavigationViewItemHeader {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NavigationViewPaneDisplayMode(pub i32);
impl NavigationViewPaneDisplayMode {
    pub const Auto: Self = Self(0);
    pub const Left: Self = Self(1);
    pub const Top: Self = Self(2);
    pub const LeftCompact: Self = Self(3);
    pub const LeftMinimal: Self = Self(4);
}
impl windows_core::TypeKind for NavigationViewPaneDisplayMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NavigationViewPaneDisplayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.NavigationViewPaneDisplayMode;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Microsoft.UI.Xaml.Controls.NavigationViewPaneDisplayMode",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewSelectionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewSelectionChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl NavigationViewSelectionChangedEventArgs {
    pub fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSettingsSelected(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSettingsSelected)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SelectedItemContainer(&self) -> windows_core::Result<NavigationViewItemBase> {
        let this =
            &windows_core::Interface::cast::<INavigationViewSelectionChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedItemContainer)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecommendedNavigationTransitionInfo(
        &self,
    ) -> windows_core::Result<xaml_bindings::NavigationTransitionInfo> {
        let this =
            &windows_core::Interface::cast::<INavigationViewSelectionChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecommendedNavigationTransitionInfo)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NavigationViewSelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        INavigationViewSelectionChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for NavigationViewSelectionChangedEventArgs {
    type Vtable = <INavigationViewSelectionChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INavigationViewSelectionChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NavigationViewSelectionChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewSelectionChangedEventArgs";
}
unsafe impl Send for NavigationViewSelectionChangedEventArgs {}
unsafe impl Sync for NavigationViewSelectionChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlControlsResources(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlControlsResources,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(XamlControlsResources, windows_collections::IIterable < windows_collections::IKeyValuePair < windows_core::IInspectable, windows_core::IInspectable > >, windows_collections::IMap < windows_core::IInspectable, windows_core::IInspectable >, xaml_bindings::ResourceDictionary, xaml_bindings::DependencyObject);
impl XamlControlsResources {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlControlsResources,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ClearValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<P0>(&self, dp: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadLocalValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAnimationBaseValue)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<xaml_bindings::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
        P1: windows_core::Param<xaml_bindings::DependencyPropertyChangedCallback>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::DependencyProperty>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IDependencyObject2>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                windows_core::Interface::as_raw(this),
                dp.param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn First(
        &self,
    ) -> windows_core::Result<
        windows_collections::IIterator<
            windows_collections::IKeyValuePair<
                windows_core::IInspectable,
                windows_core::IInspectable,
            >,
        >,
    > {
        let this = &windows_core::Interface::cast::<
            windows_collections::IIterable<
                windows_collections::IKeyValuePair<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<
            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<
            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<
            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetView(
        &self,
    ) -> windows_core::Result<
        windows_collections::IMapView<windows_core::IInspectable, windows_core::IInspectable>,
    > {
        let this = &windows_core::Interface::cast::<
            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<
            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                value.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<
            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Remove)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<
            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Source(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IResourceDictionary>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<xaml_bindings::IResourceDictionary>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetSource)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MergedDictionaries(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<xaml_bindings::ResourceDictionary>> {
        let this = &windows_core::Interface::cast::<xaml_bindings::IResourceDictionary>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MergedDictionaries)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThemeDictionaries(
        &self,
    ) -> windows_core::Result<
        windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
    > {
        let this = &windows_core::Interface::cast::<xaml_bindings::IResourceDictionary>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThemeDictionaries)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnsureRevealLights<P0>(element: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<xaml_bindings::UIElement>,
    {
        Self::IXamlControlsResourcesStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).EnsureRevealLights)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
            )
            .ok()
        })
    }
    pub fn ControlsResourcesVersionProperty()
    -> windows_core::Result<xaml_bindings::DependencyProperty> {
        Self::IXamlControlsResourcesStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlsResourcesVersionProperty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IXamlControlsResourcesStatics<
        R,
        F: FnOnce(&IXamlControlsResourcesStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlControlsResources,
            IXamlControlsResourcesStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IXamlControlsResourcesStatics3<
        R,
        F: FnOnce(&IXamlControlsResourcesStatics3) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlControlsResources,
            IXamlControlsResourcesStatics3,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlControlsResources {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlControlsResources>();
}
unsafe impl windows_core::Interface for XamlControlsResources {
    type Vtable = <IXamlControlsResources as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlControlsResources as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XamlControlsResources {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.XamlControlsResources";
}
unsafe impl Send for XamlControlsResources {}
unsafe impl Sync for XamlControlsResources {}
impl IntoIterator for XamlControlsResources {
    type Item =
        windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &XamlControlsResources {
    type Item =
        windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlControlsXamlMetaDataProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlControlsXamlMetaDataProvider,
    windows_core::IUnknown,
    windows_core::IInspectable,
    xaml_bindings::IXamlMetadataProvider
);
impl XamlControlsXamlMetaDataProvider {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Initialize() -> windows_core::Result<()> {
        Self::IXamlControlsXamlMetaDataProviderStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).Initialize)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        })
    }
    pub fn GetXamlType(
        &self,
        r#type: &xaml_bindings::TypeName,
    ) -> windows_core::Result<xaml_bindings::IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXamlType)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(r#type),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXamlTypeByFullName(
        &self,
        fullname: &windows_core::HSTRING,
    ) -> windows_core::Result<xaml_bindings::IXamlType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXamlTypeByFullName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(fullname),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> windows_core::Result<windows_core::Array<xaml_bindings::XmlnsDefinition>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetXmlnsDefinitions)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<xaml_bindings::XmlnsDefinition>::set_abi_len(
                    core::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    fn IXamlControlsXamlMetaDataProviderStatics<
        R,
        F: FnOnce(&IXamlControlsXamlMetaDataProviderStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            IXamlControlsXamlMetaDataProviderStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, xaml_bindings::IXamlMetadataProvider>();
}
unsafe impl windows_core::Interface for XamlControlsXamlMetaDataProvider {
    type Vtable = <xaml_bindings::IXamlMetadataProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <xaml_bindings::IXamlMetadataProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
unsafe impl Send for XamlControlsXamlMetaDataProvider {}
unsafe impl Sync for XamlControlsXamlMetaDataProvider {}
