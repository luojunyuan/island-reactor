#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreadcrumbBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BreadcrumbBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl BreadcrumbBar {
    pub fn new() -> windows_core::Result<BreadcrumbBar> {
        Self::IBreadcrumbBarFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<BreadcrumbBar>
    where
        T: windows_core::Compose,
    {
        Self::IBreadcrumbBarFactory(|this| unsafe {
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
    fn IBreadcrumbBarFactory<R, F: FnOnce(&IBreadcrumbBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BreadcrumbBar, IBreadcrumbBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BreadcrumbBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBreadcrumbBar>();
}
unsafe impl windows_core::Interface for BreadcrumbBar {
    type Vtable = <IBreadcrumbBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBreadcrumbBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for BreadcrumbBar {
    type Target = IBreadcrumbBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BreadcrumbBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.BreadcrumbBar";
}
unsafe impl Send for BreadcrumbBar {}
unsafe impl Sync for BreadcrumbBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreadcrumbBarItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BreadcrumbBarItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for BreadcrumbBarItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBreadcrumbBarItem>();
}
unsafe impl windows_core::Interface for BreadcrumbBarItem {
    type Vtable = <IBreadcrumbBarItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBreadcrumbBarItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for BreadcrumbBarItem {
    type Target = IBreadcrumbBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BreadcrumbBarItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.BreadcrumbBarItem";
}
unsafe impl Send for BreadcrumbBarItem {}
unsafe impl Sync for BreadcrumbBarItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreadcrumbBarItemClickedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BreadcrumbBarItemClickedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for BreadcrumbBarItemClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBreadcrumbBarItemClickedEventArgs>();
}
unsafe impl windows_core::Interface for BreadcrumbBarItemClickedEventArgs {
    type Vtable = <IBreadcrumbBarItemClickedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IBreadcrumbBarItemClickedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for BreadcrumbBarItemClickedEventArgs {
    type Target = IBreadcrumbBarItemClickedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BreadcrumbBarItemClickedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.BreadcrumbBarItemClickedEventArgs";
}
unsafe impl Send for BreadcrumbBarItemClickedEventArgs {}
unsafe impl Sync for BreadcrumbBarItemClickedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expander(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Expander,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Expander {
    pub fn new() -> windows_core::Result<Expander> {
        Self::IExpanderFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<Expander>
    where
        T: windows_core::Compose,
    {
        Self::IExpanderFactory(|this| unsafe {
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
    fn IExpanderFactory<R, F: FnOnce(&IExpanderFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Expander, IExpanderFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Expander {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExpander>();
}
unsafe impl windows_core::Interface for Expander {
    type Vtable = <IExpander as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExpander as windows_core::Interface>::IID;
}
impl core::ops::Deref for Expander {
    type Target = IExpander;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Expander {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Expander";
}
unsafe impl Send for Expander {}
unsafe impl Sync for Expander {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpanderCollapsedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ExpanderCollapsedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ExpanderCollapsedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExpanderCollapsedEventArgs>();
}
unsafe impl windows_core::Interface for ExpanderCollapsedEventArgs {
    type Vtable = <IExpanderCollapsedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExpanderCollapsedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ExpanderCollapsedEventArgs {
    type Target = IExpanderCollapsedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ExpanderCollapsedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ExpanderCollapsedEventArgs";
}
unsafe impl Send for ExpanderCollapsedEventArgs {}
unsafe impl Sync for ExpanderCollapsedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpanderExpandingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ExpanderExpandingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ExpanderExpandingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExpanderExpandingEventArgs>();
}
unsafe impl windows_core::Interface for ExpanderExpandingEventArgs {
    type Vtable = <IExpanderExpandingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExpanderExpandingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ExpanderExpandingEventArgs {
    type Target = IExpanderExpandingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ExpanderExpandingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ExpanderExpandingEventArgs";
}
unsafe impl Send for ExpanderExpandingEventArgs {}
unsafe impl Sync for ExpanderExpandingEventArgs {}
windows_core::imp::define_interface!(
    IBreadcrumbBar,
    IBreadcrumbBar_Vtbl,
    0x2e47b7d6_5fbd_54c7_b0b1_ceff4a19c744
);
impl windows_core::RuntimeType for IBreadcrumbBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBreadcrumbBar {
    pub fn put_ItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_ItemsSource)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_ItemClicked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<BreadcrumbBar>,
                windows_core::Ref<BreadcrumbBarItemClickedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            BreadcrumbBar,
            BreadcrumbBarItemClickedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ItemClicked)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ItemClicked,
            ))
        }
    }
}
#[repr(C)]
pub struct IBreadcrumbBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ItemsSource: usize,
    pub put_ItemsSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ItemTemplate: usize,
    put_ItemTemplate: usize,
    pub add_ItemClicked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ItemClicked:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBreadcrumbBarFactory,
    IBreadcrumbBarFactory_Vtbl,
    0xd5b6a6d9_3148_5cbc_a6ae_0f44cde41952
);
impl windows_core::RuntimeType for IBreadcrumbBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBreadcrumbBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBreadcrumbBarItem,
    IBreadcrumbBarItem_Vtbl,
    0x34582de4_6bef_5ba0_86ca_7cc1a3db37ee
);
impl windows_core::RuntimeType for IBreadcrumbBarItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBreadcrumbBarItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IBreadcrumbBarItemClickedEventArgs,
    IBreadcrumbBarItemClickedEventArgs_Vtbl,
    0x1ceea503_365e_580d_bcd4_e9ad0248f6b5
);
impl windows_core::RuntimeType for IBreadcrumbBarItemClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBreadcrumbBarItemClickedEventArgs {
    pub fn get_Index(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Index)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IBreadcrumbBarItemClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Index:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    get_Item: usize,
}
windows_core::imp::define_interface!(
    IExpander,
    IExpander_Vtbl,
    0xcd1d4ede_8d39_5bef_a735_9fdf6038e86b
);
impl windows_core::RuntimeType for IExpander {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IExpander {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsExpanded(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsExpanded)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Expanding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Expander>, windows_core::Ref<ExpanderExpandingEventArgs>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<Expander, ExpanderExpandingEventArgs>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Expanding)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Expanding,
            ))
        }
    }
    pub fn add_Collapsed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Expander>, windows_core::Ref<ExpanderCollapsedEventArgs>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<Expander, ExpanderCollapsedEventArgs>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Collapsed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Collapsed,
            ))
        }
    }
}
#[repr(C)]
pub struct IExpander_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_HeaderTemplateSelector: usize,
    put_HeaderTemplateSelector: usize,
    get_IsExpanded: usize,
    pub put_IsExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_ExpandDirection: usize,
    put_ExpandDirection: usize,
    pub add_Expanding: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Expanding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub add_Collapsed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Collapsed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IExpanderCollapsedEventArgs,
    IExpanderCollapsedEventArgs_Vtbl,
    0x968a6870_7426_535e_a526_279e6eedecd0
);
impl windows_core::RuntimeType for IExpanderCollapsedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExpanderCollapsedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IExpanderExpandingEventArgs,
    IExpanderExpandingEventArgs_Vtbl,
    0x433f2e36_19e7_579c_b4ce_9ce5d510d001
);
impl windows_core::RuntimeType for IExpanderExpandingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExpanderExpandingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IExpanderFactory,
    IExpanderFactory_Vtbl,
    0x51a5afc2_b16d_516e_83ae_5a10476b13af
);
impl windows_core::RuntimeType for IExpanderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExpanderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInfoBadge,
    IInfoBadge_Vtbl,
    0x82104d7f_03d4_5ea4_872e_f9ecab758601
);
impl windows_core::RuntimeType for IInfoBadge {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInfoBadge {
    pub fn put_Value(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IInfoBadge_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Value: usize,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_IconSource: usize,
    put_IconSource: usize,
    get_TemplateSettings: usize,
}
windows_core::imp::define_interface!(
    IInfoBadgeFactory,
    IInfoBadgeFactory_Vtbl,
    0xfb498205_2de0_5986_8aec_2c46ac235087
);
impl windows_core::RuntimeType for IInfoBadgeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInfoBadgeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInfoBar,
    IInfoBar_Vtbl,
    0x273ffde8_9324_55b7_9ffe_7d995a8af56b
);
impl windows_core::RuntimeType for IInfoBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInfoBar {
    pub fn put_IsOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Message(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Message)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Severity(&self, value: InfoBarSeverity) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Severity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsClosable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsClosable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<InfoBar>, windows_core::Ref<InfoBarClosedEventArgs>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<InfoBar, InfoBarClosedEventArgs>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Closed,
            ))
        }
    }
}
#[repr(C)]
pub struct IInfoBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsOpen: usize,
    pub put_IsOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Message: usize,
    pub put_Message: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Severity: usize,
    pub put_Severity:
        unsafe extern "system" fn(*mut core::ffi::c_void, InfoBarSeverity) -> windows_core::HRESULT,
    get_IconSource: usize,
    put_IconSource: usize,
    get_IsIconVisible: usize,
    put_IsIconVisible: usize,
    get_IsClosable: usize,
    pub put_IsClosable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_CloseButtonStyle: usize,
    put_CloseButtonStyle: usize,
    get_CloseButtonCommand: usize,
    put_CloseButtonCommand: usize,
    get_CloseButtonCommandParameter: usize,
    put_CloseButtonCommandParameter: usize,
    get_ActionButton: usize,
    put_ActionButton: usize,
    get_Content: usize,
    put_Content: usize,
    get_ContentTemplate: usize,
    put_ContentTemplate: usize,
    get_TemplateSettings: usize,
    add_CloseButtonClick: usize,
    remove_CloseButtonClick: usize,
    add_Closing: usize,
    remove_Closing: usize,
    pub add_Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Closed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInfoBarClosedEventArgs,
    IInfoBarClosedEventArgs_Vtbl,
    0x593af0b3_bded_53da_8f56_80ed3c64322c
);
impl windows_core::RuntimeType for IInfoBarClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInfoBarClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Reason: usize,
}
windows_core::imp::define_interface!(
    IInfoBarFactory,
    IInfoBarFactory_Vtbl,
    0x60618a60_9be7_5df5_be0d_933d34ddb44c
);
impl windows_core::RuntimeType for IInfoBarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInfoBarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INavigationView,
    INavigationView_Vtbl,
    0xb00eb54c_9174_5d84_9678_56c98016e689
);
impl windows_core::RuntimeType for INavigationView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationView {
    pub fn put_IsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsSettingsVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsSettingsVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsPaneToggleButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_SelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedItem)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn get_MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<NavigationView>,
                windows_core::Ref<NavigationViewSelectionChangedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            NavigationView,
            NavigationViewSelectionChangedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct INavigationView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsPaneOpen: usize,
    pub put_IsPaneOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_CompactModeThresholdWidth: usize,
    put_CompactModeThresholdWidth: usize,
    get_ExpandedModeThresholdWidth: usize,
    put_ExpandedModeThresholdWidth: usize,
    get_FooterMenuItems: usize,
    get_FooterMenuItemsSource: usize,
    put_FooterMenuItemsSource: usize,
    get_PaneFooter: usize,
    put_PaneFooter: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_DisplayMode: usize,
    get_IsSettingsVisible: usize,
    pub put_IsSettingsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_IsPaneToggleButtonVisible: usize,
    pub put_IsPaneToggleButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AlwaysShowHeader: usize,
    put_AlwaysShowHeader: usize,
    get_CompactPaneLength: usize,
    put_CompactPaneLength: usize,
    get_OpenPaneLength: usize,
    put_OpenPaneLength: usize,
    get_PaneToggleButtonStyle: usize,
    put_PaneToggleButtonStyle: usize,
    get_SelectedItem: usize,
    pub put_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MenuItemsSource: usize,
    put_MenuItemsSource: usize,
    get_SettingsItem: usize,
    get_AutoSuggestBox: usize,
    put_AutoSuggestBox: usize,
    get_MenuItemTemplate: usize,
    put_MenuItemTemplate: usize,
    get_MenuItemTemplateSelector: usize,
    put_MenuItemTemplateSelector: usize,
    get_MenuItemContainerStyle: usize,
    put_MenuItemContainerStyle: usize,
    get_MenuItemContainerStyleSelector: usize,
    put_MenuItemContainerStyleSelector: usize,
    MenuItemFromContainer: usize,
    ContainerFromMenuItem: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_ItemInvoked: usize,
    remove_ItemInvoked: usize,
    add_DisplayModeChanged: usize,
    remove_DisplayModeChanged: usize,
    get_IsTitleBarAutoPaddingEnabled: usize,
    put_IsTitleBarAutoPaddingEnabled: usize,
}
windows_core::imp::define_interface!(
    INavigationView2,
    INavigationView2_Vtbl,
    0x5ba2eefc_3736_5e42_ac56_9d0be5523d40
);
impl windows_core::RuntimeType for INavigationView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationView2 {
    pub fn put_IsBackButtonVisible(
        &self,
        value: NavigationViewBackButtonVisible,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_IsBackEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsBackEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PaneTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PaneTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn add_BackRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<NavigationView>,
                windows_core::Ref<NavigationViewBackRequestedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            NavigationView,
            NavigationViewBackRequestedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_BackRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_BackRequested,
            ))
        }
    }
    pub fn put_PaneDisplayMode(
        &self,
        value: NavigationViewPaneDisplayMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PaneDisplayMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct INavigationView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_IsBackButtonVisible: usize,
    pub put_IsBackButtonVisible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewBackButtonVisible,
    ) -> windows_core::HRESULT,
    get_IsBackEnabled: usize,
    pub put_IsBackEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_PaneTitle: usize,
    pub put_PaneTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub add_BackRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_BackRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_PaneClosed: usize,
    remove_PaneClosed: usize,
    add_PaneClosing: usize,
    remove_PaneClosing: usize,
    add_PaneOpened: usize,
    remove_PaneOpened: usize,
    add_PaneOpening: usize,
    remove_PaneOpening: usize,
    get_PaneDisplayMode: usize,
    pub put_PaneDisplayMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NavigationViewPaneDisplayMode,
    ) -> windows_core::HRESULT,
    get_PaneHeader: usize,
    put_PaneHeader: usize,
    get_PaneCustomContent: usize,
    put_PaneCustomContent: usize,
    get_ContentOverlay: usize,
    put_ContentOverlay: usize,
    get_IsPaneVisible: usize,
    put_IsPaneVisible: usize,
    get_SelectionFollowsFocus: usize,
    put_SelectionFollowsFocus: usize,
    get_TemplateSettings: usize,
    get_ShoulderNavigationEnabled: usize,
    put_ShoulderNavigationEnabled: usize,
    get_OverflowLabelMode: usize,
    put_OverflowLabelMode: usize,
    add_Expanding: usize,
    remove_Expanding: usize,
    add_Collapsed: usize,
    remove_Collapsed: usize,
    Expand: usize,
    Collapse: usize,
}
windows_core::imp::define_interface!(
    INavigationViewBackRequestedEventArgs,
    INavigationViewBackRequestedEventArgs_Vtbl,
    0xae752207_bd1b_5afa_a872_e9bbaeea0ede
);
impl windows_core::RuntimeType for INavigationViewBackRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
}
#[repr(C)]
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
}
impl INavigationViewItem {
    pub fn put_Icon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<crate::bindings::IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Icon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct INavigationViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Icon: usize,
    pub put_Icon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CompactPaneLength: usize,
}
windows_core::imp::define_interface!(
    INavigationViewItem2,
    INavigationViewItem2_Vtbl,
    0x2d5bd889_9dac_5675_b254_68226f077a61
);
impl windows_core::RuntimeType for INavigationViewItem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationViewItem2 {
    pub fn get_MenuItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MenuItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct INavigationViewItem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_SelectsOnInvoked: usize,
    put_SelectsOnInvoked: usize,
    get_IsExpanded: usize,
    put_IsExpanded: usize,
    get_HasUnrealizedChildren: usize,
    put_HasUnrealizedChildren: usize,
    get_IsChildSelected: usize,
    put_IsChildSelected: usize,
    pub get_MenuItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_MenuItemsSource: usize,
    put_MenuItemsSource: usize,
}
windows_core::imp::define_interface!(
    INavigationViewItemBase,
    INavigationViewItemBase_Vtbl,
    0x33586494_af48_513f_be4d_f645e8c89005
);
impl windows_core::RuntimeType for INavigationViewItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INavigationViewItemBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    INavigationViewItemFactory,
    INavigationViewItemFactory_Vtbl,
    0xde60a001_9385_5535_80e1_2b68f4bfde26
);
impl windows_core::RuntimeType for INavigationViewItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
}
#[repr(C)]
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
}
#[repr(C)]
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
    INavigationViewSelectionChangedEventArgs,
    INavigationViewSelectionChangedEventArgs_Vtbl,
    0x14a064a5_c79d_5f63_ac6e_1c313fe63566
);
impl windows_core::RuntimeType for INavigationViewSelectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INavigationViewSelectionChangedEventArgs {
    pub fn get_SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedItem)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct INavigationViewSelectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_SelectedItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsSettingsSelected: usize,
}
windows_core::imp::define_interface!(
    INumberBox,
    INumberBox_Vtbl,
    0x22c43a67_d393_56a9_801a_2dea91877de6
);
impl windows_core::RuntimeType for INumberBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INumberBox {
    pub fn put_Minimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Minimum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Maximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Maximum)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Value(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn add_ValueChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<NumberBox>, windows_core::Ref<NumberBoxValueChangedEventArgs>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            NumberBox,
            NumberBoxValueChangedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ValueChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ValueChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct INumberBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Minimum: usize,
    pub put_Minimum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Maximum: usize,
    pub put_Maximum:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_Value: usize,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    get_SmallChange: usize,
    put_SmallChange: usize,
    get_LargeChange: usize,
    put_LargeChange: usize,
    get_Text: usize,
    put_Text: usize,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_PlaceholderText: usize,
    put_PlaceholderText: usize,
    get_SelectionFlyout: usize,
    put_SelectionFlyout: usize,
    get_SelectionHighlightColor: usize,
    put_SelectionHighlightColor: usize,
    get_TextReadingOrder: usize,
    put_TextReadingOrder: usize,
    get_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    put_PreventKeyboardDisplayOnProgrammaticFocus: usize,
    get_Description: usize,
    put_Description: usize,
    get_ValidationMode: usize,
    put_ValidationMode: usize,
    get_SpinButtonPlacementMode: usize,
    put_SpinButtonPlacementMode: usize,
    get_IsWrapEnabled: usize,
    put_IsWrapEnabled: usize,
    get_AcceptsExpression: usize,
    put_AcceptsExpression: usize,
    get_NumberFormatter: usize,
    put_NumberFormatter: usize,
    pub add_ValueChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ValueChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INumberBoxFactory,
    INumberBoxFactory_Vtbl,
    0x6b81f3cb_45a4_5d19_9bbb_a9fe4656ac4d
);
impl windows_core::RuntimeType for INumberBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INumberBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INumberBoxValueChangedEventArgs,
    INumberBoxValueChangedEventArgs_Vtbl,
    0xc66cf16e_7c8a_532e_9d23_058c1c98dd50
);
impl windows_core::RuntimeType for INumberBoxValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INumberBoxValueChangedEventArgs {
    pub fn get_NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NewValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct INumberBoxValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_OldValue: usize,
    pub get_NewValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRadioButtons,
    IRadioButtons_Vtbl,
    0xdd372d64_b740_5739_8d24_b6decc91f408
);
impl windows_core::RuntimeType for IRadioButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRadioButtons {
    pub fn get_Items(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<crate::bindings::SelectionChangedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <crate::bindings::SelectionChangedEventHandler>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
    pub fn put_MaxColumns(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_MaxColumns)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRadioButtons_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_ItemsSource: usize,
    put_ItemsSource: usize,
    pub get_Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ItemTemplate: usize,
    put_ItemTemplate: usize,
    ContainerFromIndex: usize,
    pub get_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_SelectedItem: usize,
    put_SelectedItem: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    get_MaxColumns: usize,
    pub put_MaxColumns:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
}
windows_core::imp::define_interface!(
    IRadioButtonsFactory,
    IRadioButtonsFactory_Vtbl,
    0x2cf95efb_a7a2_5d85_8ead_ea222baa3c55
);
impl windows_core::RuntimeType for IRadioButtonsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadioButtonsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITabView,
    ITabView_Vtbl,
    0x6aa787ab_5a30_5ea2_be5b_aed868381756
);
impl windows_core::RuntimeType for ITabView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITabView {
    pub fn put_IsAddTabButtonVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsAddTabButtonVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_TabCloseRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TabView>, windows_core::Ref<TabViewTabCloseRequestedEventArgs>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            TabView,
            TabViewTabCloseRequestedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_TabCloseRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_TabCloseRequested,
            ))
        }
    }
    pub fn add_AddTabButtonClick<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TabView>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler =
            <windows::Foundation::TypedEventHandler<TabView, windows_core::IInspectable>>::new(
                move |a0, a1| {
                    handler(a0, a1);
                    Ok(())
                },
            );
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_AddTabButtonClick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_AddTabButtonClick,
            ))
        }
    }
    pub fn get_TabItems(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TabItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn put_CanReorderTabs(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_CanReorderTabs)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn get_SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_SelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_SelectedIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<crate::bindings::SelectionChangedEventArgs>,
            ) + Send
            + 'static,
    {
        let handler = <crate::bindings::SelectionChangedEventHandler>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_SelectionChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct ITabView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_TabWidthMode: usize,
    put_TabWidthMode: usize,
    get_CloseButtonOverlayMode: usize,
    put_CloseButtonOverlayMode: usize,
    get_TabStripHeader: usize,
    put_TabStripHeader: usize,
    get_TabStripHeaderTemplate: usize,
    put_TabStripHeaderTemplate: usize,
    get_TabStripFooter: usize,
    put_TabStripFooter: usize,
    get_TabStripFooterTemplate: usize,
    put_TabStripFooterTemplate: usize,
    get_IsAddTabButtonVisible: usize,
    pub put_IsAddTabButtonVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AddTabButtonCommand: usize,
    put_AddTabButtonCommand: usize,
    get_AddTabButtonCommandParameter: usize,
    put_AddTabButtonCommandParameter: usize,
    pub add_TabCloseRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_TabCloseRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_TabDroppedOutside: usize,
    remove_TabDroppedOutside: usize,
    pub add_AddTabButtonClick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_AddTabButtonClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_TabItemsChanged: usize,
    remove_TabItemsChanged: usize,
    get_TabItemsSource: usize,
    put_TabItemsSource: usize,
    pub get_TabItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_TabItemTemplate: usize,
    put_TabItemTemplate: usize,
    get_TabItemTemplateSelector: usize,
    put_TabItemTemplateSelector: usize,
    get_CanDragTabs: usize,
    put_CanDragTabs: usize,
    get_CanReorderTabs: usize,
    pub put_CanReorderTabs:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_AllowDropTabs: usize,
    put_AllowDropTabs: usize,
    pub get_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SelectedIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    get_SelectedItem: usize,
    put_SelectedItem: usize,
    ContainerFromItem: usize,
    ContainerFromIndex: usize,
    pub add_SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_SelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_TabDragStarting: usize,
    remove_TabDragStarting: usize,
    add_TabDragCompleted: usize,
    remove_TabDragCompleted: usize,
    add_TabStripDragOver: usize,
    remove_TabStripDragOver: usize,
    add_TabStripDrop: usize,
    remove_TabStripDrop: usize,
}
windows_core::imp::define_interface!(
    ITabViewFactory,
    ITabViewFactory_Vtbl,
    0xe7e83685_eedf_5106_9429_884435ab166b
);
impl windows_core::RuntimeType for ITabViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITabViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITabViewItem,
    ITabViewItem_Vtbl,
    0x291f3e98_4f17_5021_94f0_6a5b304312b6
);
impl windows_core::RuntimeType for ITabViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITabViewItem {
    pub fn put_Header<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_Header)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsClosable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsClosable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITabViewItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Header: usize,
    pub put_Header: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_HeaderTemplate: usize,
    put_HeaderTemplate: usize,
    get_IconSource: usize,
    put_IconSource: usize,
    get_IsClosable: usize,
    pub put_IsClosable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_TabViewTemplateSettings: usize,
    add_CloseRequested: usize,
    remove_CloseRequested: usize,
}
windows_core::imp::define_interface!(
    ITabViewItemFactory,
    ITabViewItemFactory_Vtbl,
    0xb64c2423_7e56_5d41_8a84_1ee28f9826a4
);
impl windows_core::RuntimeType for ITabViewItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITabViewItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITabViewTabCloseRequestedEventArgs,
    ITabViewTabCloseRequestedEventArgs_Vtbl,
    0xd56ab9b2_e264_5c7e_a1cb_e41a16a6c6c6
);
impl windows_core::RuntimeType for ITabViewTabCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITabViewTabCloseRequestedEventArgs {
    pub fn get_Tab(&self) -> windows_core::Result<TabViewItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Tab)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ITabViewTabCloseRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Item: usize,
    pub get_Tab: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITeachingTip,
    ITeachingTip_Vtbl,
    0x5b200440_4fcc_5fdb_b418_b5797b0874ad
);
impl windows_core::RuntimeType for ITeachingTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITeachingTip {
    pub fn put_Title(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Title)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_Subtitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Subtitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn put_IsOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_ActionButtonContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_ActionButtonContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_CloseButtonContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).put_CloseButtonContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn put_IsLightDismissEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_IsLightDismissEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn put_PreferredPlacement(
        &self,
        value: TeachingTipPlacementMode,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_PreferredPlacement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn add_ActionButtonClick<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TeachingTip>, windows_core::Ref<windows_core::IInspectable>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            TeachingTip,
            windows_core::IInspectable,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_ActionButtonClick)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_ActionButtonClick,
            ))
        }
    }
    pub fn add_Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<TeachingTip>, windows_core::Ref<TeachingTipClosedEventArgs>)
            + Send
            + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<
            TeachingTip,
            TeachingTipClosedEventArgs,
        >>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).add_Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).remove_Closed,
            ))
        }
    }
}
#[repr(C)]
pub struct ITeachingTip_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Title: usize,
    pub put_Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_Subtitle: usize,
    pub put_Subtitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_IsOpen: usize,
    pub put_IsOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_Target: usize,
    put_Target: usize,
    get_TailVisibility: usize,
    put_TailVisibility: usize,
    get_ActionButtonContent: usize,
    pub put_ActionButtonContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_ActionButtonStyle: usize,
    put_ActionButtonStyle: usize,
    get_ActionButtonCommand: usize,
    put_ActionButtonCommand: usize,
    get_ActionButtonCommandParameter: usize,
    put_ActionButtonCommandParameter: usize,
    get_CloseButtonContent: usize,
    pub put_CloseButtonContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    get_CloseButtonStyle: usize,
    put_CloseButtonStyle: usize,
    get_CloseButtonCommand: usize,
    put_CloseButtonCommand: usize,
    get_CloseButtonCommandParameter: usize,
    put_CloseButtonCommandParameter: usize,
    get_PlacementMargin: usize,
    put_PlacementMargin: usize,
    get_ShouldConstrainToRootBounds: usize,
    put_ShouldConstrainToRootBounds: usize,
    get_IsLightDismissEnabled: usize,
    pub put_IsLightDismissEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    get_PreferredPlacement: usize,
    pub put_PreferredPlacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TeachingTipPlacementMode,
    ) -> windows_core::HRESULT,
    get_HeroContentPlacement: usize,
    put_HeroContentPlacement: usize,
    get_HeroContent: usize,
    put_HeroContent: usize,
    get_IconSource: usize,
    put_IconSource: usize,
    get_TemplateSettings: usize,
    pub add_ActionButtonClick: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_ActionButtonClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    add_CloseButtonClick: usize,
    remove_CloseButtonClick: usize,
    add_Closing: usize,
    remove_Closing: usize,
    pub add_Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub remove_Closed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITeachingTipClosedEventArgs,
    ITeachingTipClosedEventArgs_Vtbl,
    0x2536f506_4038_59db_9e35_a9252fb5adb2
);
impl windows_core::RuntimeType for ITeachingTipClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITeachingTipClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    get_Reason: usize,
}
windows_core::imp::define_interface!(
    ITeachingTipFactory,
    ITeachingTipFactory_Vtbl,
    0xa3ecd47d_2972_5d19_a62e_ddfbc5e1ad57
);
impl windows_core::RuntimeType for ITeachingTipFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITeachingTipFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
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
}
#[repr(C)]
pub struct IXamlControlsResources_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoBadge(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InfoBadge,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl InfoBadge {
    pub fn new() -> windows_core::Result<InfoBadge> {
        Self::IInfoBadgeFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<InfoBadge>
    where
        T: windows_core::Compose,
    {
        Self::IInfoBadgeFactory(|this| unsafe {
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
    fn IInfoBadgeFactory<R, F: FnOnce(&IInfoBadgeFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InfoBadge, IInfoBadgeFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InfoBadge {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInfoBadge>();
}
unsafe impl windows_core::Interface for InfoBadge {
    type Vtable = <IInfoBadge as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInfoBadge as windows_core::Interface>::IID;
}
impl core::ops::Deref for InfoBadge {
    type Target = IInfoBadge;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InfoBadge {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.InfoBadge";
}
unsafe impl Send for InfoBadge {}
unsafe impl Sync for InfoBadge {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InfoBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl InfoBar {
    pub fn new() -> windows_core::Result<InfoBar> {
        Self::IInfoBarFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<InfoBar>
    where
        T: windows_core::Compose,
    {
        Self::IInfoBarFactory(|this| unsafe {
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
    fn IInfoBarFactory<R, F: FnOnce(&IInfoBarFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InfoBar, IInfoBarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InfoBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInfoBar>();
}
unsafe impl windows_core::Interface for InfoBar {
    type Vtable = <IInfoBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInfoBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for InfoBar {
    type Target = IInfoBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InfoBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.InfoBar";
}
unsafe impl Send for InfoBar {}
unsafe impl Sync for InfoBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoBarClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InfoBarClosedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for InfoBarClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInfoBarClosedEventArgs>();
}
unsafe impl windows_core::Interface for InfoBarClosedEventArgs {
    type Vtable = <IInfoBarClosedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInfoBarClosedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for InfoBarClosedEventArgs {
    type Target = IInfoBarClosedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InfoBarClosedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.InfoBarClosedEventArgs";
}
unsafe impl Send for InfoBarClosedEventArgs {}
unsafe impl Sync for InfoBarClosedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InfoBarSeverity(pub i32);
impl InfoBarSeverity {
    pub const Informational: Self = Self(0);
    pub const Success: Self = Self(1);
    pub const Warning: Self = Self(2);
    pub const Error: Self = Self(3);
}
impl windows_core::TypeKind for InfoBarSeverity {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InfoBarSeverity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.InfoBarSeverity;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl NavigationView {
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
    fn INavigationViewFactory<R, F: FnOnce(&INavigationViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewFactory> =
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
impl core::ops::Deref for NavigationView {
    type Target = INavigationView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl core::ops::Deref for NavigationViewBackRequestedEventArgs {
    type Target = INavigationViewBackRequestedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
windows_core::imp::required_hierarchy!(NavigationViewItem, NavigationViewItemBase);
impl NavigationViewItem {
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
}
impl windows_core::RuntimeType for NavigationViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItem>();
}
unsafe impl windows_core::Interface for NavigationViewItem {
    type Vtable = <INavigationViewItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationViewItem {
    type Target = INavigationViewItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for NavigationViewItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItemBase>();
}
unsafe impl windows_core::Interface for NavigationViewItemBase {
    type Vtable = <INavigationViewItemBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INavigationViewItemBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for NavigationViewItemBase {
    type Target = INavigationViewItemBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
windows_core::imp::required_hierarchy!(NavigationViewItemHeader, NavigationViewItemBase);
impl NavigationViewItemHeader {
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
impl core::ops::Deref for NavigationViewItemHeader {
    type Target = INavigationViewItemHeader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewSelectionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NavigationViewSelectionChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
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
impl core::ops::Deref for NavigationViewSelectionChangedEventArgs {
    type Target = INavigationViewSelectionChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NavigationViewSelectionChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewSelectionChangedEventArgs";
}
unsafe impl Send for NavigationViewSelectionChangedEventArgs {}
unsafe impl Sync for NavigationViewSelectionChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NumberBox,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl NumberBox {
    pub fn new() -> windows_core::Result<NumberBox> {
        Self::INumberBoxFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<NumberBox>
    where
        T: windows_core::Compose,
    {
        Self::INumberBoxFactory(|this| unsafe {
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
    fn INumberBoxFactory<R, F: FnOnce(&INumberBoxFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NumberBox, INumberBoxFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NumberBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INumberBox>();
}
unsafe impl windows_core::Interface for NumberBox {
    type Vtable = <INumberBox as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumberBox as windows_core::Interface>::IID;
}
impl core::ops::Deref for NumberBox {
    type Target = INumberBox;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NumberBox {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NumberBox";
}
unsafe impl Send for NumberBox {}
unsafe impl Sync for NumberBox {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberBoxValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NumberBoxValueChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for NumberBoxValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INumberBoxValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for NumberBoxValueChangedEventArgs {
    type Vtable = <INumberBoxValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INumberBoxValueChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for NumberBoxValueChangedEventArgs {
    type Target = INumberBoxValueChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NumberBoxValueChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NumberBoxValueChangedEventArgs";
}
unsafe impl Send for NumberBoxValueChangedEventArgs {}
unsafe impl Sync for NumberBoxValueChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioButtons(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RadioButtons,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl RadioButtons {
    pub fn new() -> windows_core::Result<RadioButtons> {
        Self::IRadioButtonsFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<RadioButtons>
    where
        T: windows_core::Compose,
    {
        Self::IRadioButtonsFactory(|this| unsafe {
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
    fn IRadioButtonsFactory<R, F: FnOnce(&IRadioButtonsFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadioButtons, IRadioButtonsFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadioButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRadioButtons>();
}
unsafe impl windows_core::Interface for RadioButtons {
    type Vtable = <IRadioButtons as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRadioButtons as windows_core::Interface>::IID;
}
impl core::ops::Deref for RadioButtons {
    type Target = IRadioButtons;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RadioButtons {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.RadioButtons";
}
unsafe impl Send for RadioButtons {}
unsafe impl Sync for RadioButtons {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TabView,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TabView {
    pub fn new() -> windows_core::Result<TabView> {
        Self::ITabViewFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<TabView>
    where
        T: windows_core::Compose,
    {
        Self::ITabViewFactory(|this| unsafe {
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
    fn ITabViewFactory<R, F: FnOnce(&ITabViewFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TabView, ITabViewFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TabView {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITabView>();
}
unsafe impl windows_core::Interface for TabView {
    type Vtable = <ITabView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITabView as windows_core::Interface>::IID;
}
impl core::ops::Deref for TabView {
    type Target = ITabView;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TabView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TabView";
}
unsafe impl Send for TabView {}
unsafe impl Sync for TabView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabViewItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TabViewItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TabViewItem {
    pub fn new() -> windows_core::Result<TabViewItem> {
        Self::ITabViewItemFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<TabViewItem>
    where
        T: windows_core::Compose,
    {
        Self::ITabViewItemFactory(|this| unsafe {
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
    fn ITabViewItemFactory<R, F: FnOnce(&ITabViewItemFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TabViewItem, ITabViewItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TabViewItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITabViewItem>();
}
unsafe impl windows_core::Interface for TabViewItem {
    type Vtable = <ITabViewItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITabViewItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for TabViewItem {
    type Target = ITabViewItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TabViewItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TabViewItem";
}
unsafe impl Send for TabViewItem {}
unsafe impl Sync for TabViewItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabViewTabCloseRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TabViewTabCloseRequestedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for TabViewTabCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITabViewTabCloseRequestedEventArgs>();
}
unsafe impl windows_core::Interface for TabViewTabCloseRequestedEventArgs {
    type Vtable = <ITabViewTabCloseRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ITabViewTabCloseRequestedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TabViewTabCloseRequestedEventArgs {
    type Target = ITabViewTabCloseRequestedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TabViewTabCloseRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TabViewTabCloseRequestedEventArgs";
}
unsafe impl Send for TabViewTabCloseRequestedEventArgs {}
unsafe impl Sync for TabViewTabCloseRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeachingTip(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TeachingTip,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl TeachingTip {
    pub fn new() -> windows_core::Result<TeachingTip> {
        Self::ITeachingTipFactory(|this| unsafe {
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
    pub fn compose<T>(compose: T) -> windows_core::Result<TeachingTip>
    where
        T: windows_core::Compose,
    {
        Self::ITeachingTipFactory(|this| unsafe {
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
    fn ITeachingTipFactory<R, F: FnOnce(&ITeachingTipFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TeachingTip, ITeachingTipFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TeachingTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITeachingTip>();
}
unsafe impl windows_core::Interface for TeachingTip {
    type Vtable = <ITeachingTip as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITeachingTip as windows_core::Interface>::IID;
}
impl core::ops::Deref for TeachingTip {
    type Target = ITeachingTip;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TeachingTip {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TeachingTip";
}
unsafe impl Send for TeachingTip {}
unsafe impl Sync for TeachingTip {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeachingTipClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TeachingTipClosedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for TeachingTipClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITeachingTipClosedEventArgs>();
}
unsafe impl windows_core::Interface for TeachingTipClosedEventArgs {
    type Vtable = <ITeachingTipClosedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITeachingTipClosedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TeachingTipClosedEventArgs {
    type Target = ITeachingTipClosedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TeachingTipClosedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TeachingTipClosedEventArgs";
}
unsafe impl Send for TeachingTipClosedEventArgs {}
unsafe impl Sync for TeachingTipClosedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TeachingTipPlacementMode(pub i32);
impl TeachingTipPlacementMode {
    pub const Auto: Self = Self(0);
    pub const Top: Self = Self(1);
    pub const Bottom: Self = Self(2);
    pub const Left: Self = Self(3);
    pub const Right: Self = Self(4);
    pub const TopRight: Self = Self(5);
    pub const TopLeft: Self = Self(6);
    pub const BottomRight: Self = Self(7);
    pub const BottomLeft: Self = Self(8);
    pub const LeftTop: Self = Self(9);
    pub const LeftBottom: Self = Self(10);
    pub const RightTop: Self = Self(11);
    pub const RightBottom: Self = Self(12);
    pub const Center: Self = Self(13);
}
impl windows_core::TypeKind for TeachingTipPlacementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TeachingTipPlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.TeachingTipPlacementMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlControlsResources(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlControlsResources,
    windows_core::IUnknown,
    windows_core::IInspectable
);
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
}
impl windows_core::RuntimeType for XamlControlsResources {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlControlsResources>();
}
unsafe impl windows_core::Interface for XamlControlsResources {
    type Vtable = <IXamlControlsResources as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlControlsResources as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlControlsResources {
    type Target = IXamlControlsResources;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlControlsResources {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.XamlControlsResources";
}
unsafe impl Send for XamlControlsResources {}
unsafe impl Sync for XamlControlsResources {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlControlsXamlMetaDataProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlControlsXamlMetaDataProvider,
    windows_core::IUnknown,
    windows_core::IInspectable,
    crate::bindings::IXamlMetadataProvider
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
}
impl windows_core::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, crate::bindings::IXamlMetadataProvider>();
}
unsafe impl windows_core::Interface for XamlControlsXamlMetaDataProvider {
    type Vtable = <crate::bindings::IXamlMetadataProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <crate::bindings::IXamlMetadataProvider as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlControlsXamlMetaDataProvider {
    type Target = crate::bindings::IXamlMetadataProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
unsafe impl Send for XamlControlsXamlMetaDataProvider {}
unsafe impl Sync for XamlControlsXamlMetaDataProvider {}
