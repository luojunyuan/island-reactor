use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicPtr, Ordering};

use windows::{
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
        },
        System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
        UI::{
            HiDpi::GetDpiForWindow,
            WindowsAndMessaging::{
                CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DispatchMessageW,
                GWLP_USERDATA, GetClientRect, GetMessageW, HMENU, IDC_ARROW, LoadCursorW, MSG,
                PostQuitMessage, RegisterClassW, SW_HIDE, SW_SHOW, SWP_NOACTIVATE, SWP_NOZORDER,
                SWP_SHOWWINDOW, SendMessageW, SetWindowLongPtrW, SetWindowPos, ShowWindow,
                TranslateMessage, WINDOW_EX_STYLE, WM_DESTROY, WM_NCCREATE, WM_SIZE, WNDCLASSW,
                WS_OVERLAPPEDWINDOW, WS_VISIBLE,
            },
        },
    },
    core::{BOOL, PCSTR, PCWSTR},
};
use windows_core::Interface;

use crate::bindings::*;
use crate::core::*;

use super::app_shim::create_island_application;
use super::{WinUIBackend, WinUIDispatcher};

static XAML_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static CORE_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

thread_local! {
    static ROOT_FRAMEWORK_ELEMENT: RefCell<Option<FrameworkElement>> = const { RefCell::new(None) };
    static PENDING_THEME: Cell<Option<ElementTheme>> = const { Cell::new(None) };
    static WINUI2_RESOURCES_INSTALLED: Cell<bool> = const { Cell::new(false) };
    static UNHANDLED_EXCEPTION_HANDLER: RefCell<Option<UnhandledExceptionEventHandler>> =
        const { RefCell::new(None) };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestedTheme {
    Default,
    Light,
    Dark,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum PresenterKind {
    #[default]
    Default,
    FullScreen,
    CompactOverlay,
}

pub fn set_requested_theme(theme: RequestedTheme) {
    let element_theme = match theme {
        RequestedTheme::Light => ElementTheme::Light,
        RequestedTheme::Dark => ElementTheme::Dark,
        RequestedTheme::Default => ElementTheme::Default,
    };
    ROOT_FRAMEWORK_ELEMENT.with(|cell| {
        if let Some(fe) = cell.borrow().as_ref() {
            let _ = fe.SetRequestedTheme(element_theme);
            update_color_scheme_from(fe);
        } else {
            PENDING_THEME.with(|pending| pending.set(Some(element_theme)));
        }
    });
}

pub fn set_backdrop(_backdrop: Option<()>) {}

pub fn install_winui2_resources() -> windows_core::Result<()> {
    if WINUI2_RESOURCES_INSTALLED.with(Cell::get) {
        return Ok(());
    }

    if !current_exe_sibling("Microsoft.UI.Xaml.dll")?.exists() {
        crate::diagnostics::emit(
            "island_reactor: WinUI 2 runtime DLL not found; skipping resource install\n",
        );
        return Ok(());
    }

    if let Err(err) = load_winui2_pri() {
        crate::diagnostics::emit(&format!(
            "island_reactor: WinUI 2 PRI load failed: {err:?}\n"
        ));
        return Err(err);
    }

    install_xaml_controls_resources()?;
    WINUI2_RESOURCES_INSTALLED.with(|installed| installed.set(true));
    Ok(())
}

fn install_xaml_controls_resources() -> windows_core::Result<()> {
    let app = Application::Current()?;
    let resources = match app.Resources() {
        Ok(resources) => resources,
        Err(_) => {
            let resources = ResourceDictionary::new()?;
            app.SetResources(&resources)?;
            resources
        }
    };
    let dictionaries = resources.MergedDictionaries()?;
    let mux_resources = create_xaml_controls_resources().map_err(|err| {
        crate::diagnostics::emit(&format!(
            "island_reactor: XamlControlsResources creation failed: {err:?}\n"
        ));
        err
    })?;
    let mux_resources = mux_resources.cast::<ResourceDictionary>().map_err(|err| {
        crate::diagnostics::emit(&format!(
            "island_reactor: XamlControlsResources dictionary cast failed: {err:?}\n"
        ));
        err
    })?;
    dictionaries.Append(&mux_resources).map_err(|err| {
        crate::diagnostics::emit(&format!(
            "island_reactor: XamlControlsResources append failed: {err:?}\n"
        ));
        err
    })?;
    Ok(())
}

fn try_install_winui2_resources() {
    if let Err(err) = install_winui2_resources() {
        crate::diagnostics::emit(&format!(
            "island_reactor: WinUI 2 resources were not installed: {err:?}\n"
        ));
    }
}

fn create_xaml_controls_resources() -> windows_core::Result<ResourceDictionary> {
    const XAML: &str = r#"<ResourceDictionary
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:muxc="using:Microsoft.UI.Xaml.Controls">
    <ResourceDictionary.MergedDictionaries>
        <muxc:XamlControlsResources />
    </ResourceDictionary.MergedDictionaries>
</ResourceDictionary>"#;

    match XamlReader::Load(&windows_core::HSTRING::from(XAML))
        .and_then(|value| value.cast::<ResourceDictionary>())
    {
        Ok(resources) => Ok(resources),
        Err(err) => {
            crate::diagnostics::emit(&format!(
                "island_reactor: XamlReader WinUI 2 resource load failed, falling back to activation: {err:?}\n"
            ));
            XamlControlsResources::new()?.cast()
        }
    }
}

fn load_winui2_pri() -> windows_core::Result<()> {
    let pri_path = current_exe_sibling("Microsoft.UI.Xaml.pri")?;
    if !pri_path.exists() {
        crate::diagnostics::emit(&format!(
            "island_reactor: WinUI 2 PRI not found at {}\n",
            pri_path.display()
        ));
        return Ok(());
    }

    let path = windows_core::HSTRING::from(pri_path.display().to_string());
    let pri_file = StorageFile::GetFileFromPathAsync(&path)
        .map_err(|err| {
            crate::diagnostics::emit(&format!(
                "island_reactor: StorageFile::GetFileFromPathAsync failed for {}: {err:?}\n",
                pri_path.display()
            ));
            err
        })?
        .join()
        .map_err(|err| {
            crate::diagnostics::emit(&format!(
                "island_reactor: StorageFile::GetFileFromPathAsync join failed for {}: {err:?}\n",
                pri_path.display()
            ));
            err
        })?;
    let pri_file: IStorageFile = pri_file.cast()?;
    let files = windows_collections::IVector::<IStorageFile>::from(vec![Some(pri_file)]);
    let resource_manager = ResourceManager::Current().map_err(|err| {
        crate::diagnostics::emit(&format!(
            "island_reactor: ResourceManager::Current failed: {err:?}\n"
        ));
        err
    })?;
    resource_manager.LoadPriFiles(&files).map_err(|err| {
        crate::diagnostics::emit(&format!(
            "island_reactor: ResourceManager::LoadPriFiles failed for {}: {err:?}\n",
            pri_path.display()
        ));
        err
    })?;
    Ok(())
}

fn current_exe_sibling(file_name: &str) -> windows_core::Result<PathBuf> {
    let mut path = std::env::current_exe().map_err(|err| {
        windows_core::Error::new(windows_core::HRESULT(0x80004005u32 as i32), err.to_string())
    })?;
    path.set_file_name(file_name);
    Ok(path)
}

fn install_xaml_exception_logging() {
    let Ok(app) = Application::Current() else {
        return;
    };
    UNHANDLED_EXCEPTION_HANDLER.with(|slot| {
        if slot.borrow().is_some() {
            return;
        }
        let handler = UnhandledExceptionEventHandler::new(|_, args| {
            if let Some(args) = args.as_ref() {
                let hr = args.Exception().map(|v| v.0).unwrap_or_default();
                let message = args
                    .Message()
                    .map(|v| v.to_string_lossy())
                    .unwrap_or_default();
                crate::diagnostics::emit(&format!(
                    "island_reactor: XAML unhandled exception 0x{hr:08X}: {message}\n"
                ));
            }
            Ok(())
        });
        if app.UnhandledException(&handler).is_ok() {
            *slot.borrow_mut() = Some(handler);
        }
    });
}

pub struct ReactorHost {
    render_host: RenderHost<WinUIBackend, WinUIDispatcher>,
    hwnd: HWND,
    _application: Application,
    _xaml_manager: WindowsXamlManager,
    _xaml_source: DesktopWindowXamlSource,
    presenter: Cell<PresenterKind>,
}

impl ReactorHost {
    pub fn new(title: impl AsRef<str>, root: Box<dyn Component>) -> windows_core::Result<Self> {
        Self::new_with(title, root, |_| {})
    }

    pub fn new_with<F>(
        title: impl AsRef<str>,
        root: Box<dyn Component>,
        configure: F,
    ) -> windows_core::Result<Self>
    where
        F: FnOnce(&mut crate::core::reconciler::Reconciler<WinUIBackend>),
    {
        Self::new_with_window_options(title, None, InnerConstraints::default(), root, configure)
    }

    pub fn new_with_window_options<F>(
        title: impl AsRef<str>,
        size: Option<crate::core::Size>,
        _constraints: InnerConstraints,
        root: Box<dyn Component>,
        configure: F,
    ) -> windows_core::Result<Self>
    where
        F: FnOnce(&mut crate::core::reconciler::Reconciler<WinUIBackend>),
    {
        let application = match Application::Current() {
            Ok(application) => application,
            Err(_) => create_island_application()?,
        };
        let xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;
        install_xaml_exception_logging();
        try_install_winui2_resources();
        initialize_core_window_handle();

        let hwnd = create_main_window(title.as_ref(), size)?;
        let xaml_source = DesktopWindowXamlSource::new()?;
        let native_source: IDesktopWindowXamlSourceNative = xaml_source.cast()?;
        unsafe {
            native_source.AttachToWindow(hwnd)?;
        }
        let xaml_hwnd = unsafe { native_source.WindowHandle()? };
        XAML_HWND.store(xaml_hwnd.0, Ordering::Relaxed);
        enable_resize_layout_synchronization(hwnd, true);
        resize_xaml_island(hwnd);

        let dispatcher = WinUIDispatcher::for_current_thread()?;
        let render_host = RenderHost::new(WinUIBackend::new(), root, dispatcher);
        render_host.set_inner_size(client_size_dips(hwnd));
        render_host.set_dpi(current_dpi(hwnd));
        render_host.with_reconciler_mut(configure);

        let source_for_post_render = xaml_source.clone();
        let render_for_post_render = render_host.clone_inner();
        let last_attached: Rc<Cell<Option<ControlId>>> = Rc::new(Cell::new(None));
        let last_for_hook = Rc::clone(&last_attached);
        render_host.set_post_render(move |new_id| {
            if last_for_hook.get() == new_id {
                return;
            }
            if let Some(rid) = new_id {
                if let Some(ui) = render_for_post_render.with_backend(|b| b.get_ui_element(rid))
                    && let Ok(ui_element) = ui.cast::<UIElement>()
                {
                    let _ = source_for_post_render.SetContent(&ui_element);
                    last_for_hook.set(Some(rid));
                    if let Ok(fe) = ui_element.cast::<FrameworkElement>() {
                        ROOT_FRAMEWORK_ELEMENT.with(|cell| {
                            *cell.borrow_mut() = Some(fe.clone());
                        });
                        if let Some(theme) = PENDING_THEME.with(|pending| pending.take()) {
                            let _ = fe.SetRequestedTheme(theme);
                        }
                        update_color_scheme_from(&fe);
                    }
                }
            } else {
                last_for_hook.set(None);
            }
        });

        render_host.kick();

        Ok(Self {
            render_host,
            hwnd,
            _application: application,
            _xaml_manager: xaml_manager,
            _xaml_source: xaml_source,
            presenter: Cell::new(PresenterKind::Default),
        })
    }

    pub fn set_presenter(&self, kind: PresenterKind) {
        self.presenter.set(kind);
    }

    pub fn activate(&self) -> windows_core::Result<()> {
        let _ = self.presenter.get();
        unsafe {
            let _ = ShowWindow(self.hwnd, SW_SHOW);
        }
        Ok(())
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn stats(&self) -> RenderStats {
        self.render_host.stats()
    }

    pub fn set_render_complete<F>(&self, f: F)
    where
        F: Fn(f64, f64, f64) + 'static,
    {
        self.render_host.set_render_complete(f);
    }
}

impl<B: Backend + 'static, D: Dispatcher + 'static> RenderHost<B, D> {
    pub fn with_backend<R>(&self, f: impl FnOnce(&B) -> R) -> R {
        self.with_reconciler(|r| f(&r.backend))
    }
}

pub(crate) fn message_loop() -> windows_core::Result<()> {
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}

fn create_main_window(
    title: &str,
    inner_size: Option<crate::core::Size>,
) -> windows_core::Result<HWND> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(PCWSTR::null())?.into();

        let class_name_buf = wide_null("IslandReactor_Window");
        let title_buf = wide_null(title);
        let class_name = PCWSTR(class_name_buf.as_ptr());

        let window_class = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: class_name,
            lpfnWndProc: Some(wnd_proc),
            ..Default::default()
        };
        let _ = RegisterClassW(&window_class);

        let (width, height) = inner_size
            .map(|size| (size.width.round() as i32, size.height.round() as i32))
            .unwrap_or_else(|| {
                let default = default_display_size();
                (default.width.round() as i32, default.height.round() as i32)
            });

        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            PCWSTR(title_buf.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None::<HMENU>,
            Some(instance),
            None,
        )
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_NCCREATE => unsafe {
            let create_struct = lparam.0 as *const CREATESTRUCTW;
            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                (*create_struct).lpCreateParams as isize,
            );
            DefWindowProcW(hwnd, message, wparam, lparam)
        },
        WM_SIZE => {
            resize_xaml_island(hwnd);
            send_size_to_core_window(wparam, lparam);
            set_synchronization_window(hwnd);
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}

fn resize_xaml_island(parent: HWND) {
    let raw_xaml_hwnd = XAML_HWND.load(Ordering::Relaxed);
    if raw_xaml_hwnd.is_null() {
        return;
    }
    unsafe {
        let mut rect = RECT::default();
        if GetClientRect(parent, &mut rect).is_ok() {
            let _ = SetWindowPos(
                HWND(raw_xaml_hwnd),
                None,
                0,
                0,
                rect.right - rect.left,
                rect.bottom - rect.top,
                SWP_NOZORDER | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
        }
    }
}

fn initialize_core_window_handle() {
    let Ok(core_window) = xaml_bindings::interop::CoreWindow::GetForCurrentThread() else {
        return;
    };
    let Ok(core_window_interop) = core_window.cast::<xaml_bindings::interop::ICoreWindowInterop>()
    else {
        return;
    };
    let Ok(core_hwnd) = (unsafe { core_window_interop.GetWindowHandle() }) else {
        return;
    };
    CORE_HWND.store(core_hwnd.0, Ordering::Relaxed);
    unsafe {
        let _ = ShowWindow(core_hwnd, SW_HIDE);
    }
}

fn enable_resize_layout_synchronization(hwnd: HWND, enabled: bool) {
    type EnableResizeLayoutSynchronization = unsafe extern "system" fn(HWND, BOOL);
    unsafe {
        let user32 = wide_null("user32.dll");
        let Ok(user32) = GetModuleHandleW(PCWSTR(user32.as_ptr())) else {
            return;
        };
        let Some(proc) = GetProcAddress(user32, PCSTR(2615usize as *const u8)) else {
            return;
        };
        let enable_resize_layout_synchronization: EnableResizeLayoutSynchronization =
            std::mem::transmute(proc);
        enable_resize_layout_synchronization(hwnd, BOOL::from(enabled));
    }
}

fn send_size_to_core_window(wparam: WPARAM, lparam: LPARAM) {
    let raw_core_hwnd = CORE_HWND.load(Ordering::Relaxed);
    if raw_core_hwnd.is_null() {
        return;
    }
    unsafe {
        let _ = SendMessageW(HWND(raw_core_hwnd), WM_SIZE, Some(wparam), Some(lparam));
    }
}

fn set_synchronization_window(hwnd: HWND) {
    let Ok(application) = xaml_bindings::interop::Application::Current() else {
        return;
    };
    let Ok(framework_app_private) =
        application.cast::<xaml_bindings::interop::IFrameworkApplicationPrivate>()
    else {
        return;
    };
    unsafe {
        let _ = framework_app_private.SetSynchronizationWindow(hwnd);
    }
}

fn client_size_dips(hwnd: HWND) -> crate::core::Size {
    unsafe {
        let mut rect = RECT::default();
        let dpi = current_dpi(hwnd) as f64;
        if GetClientRect(hwnd, &mut rect).is_ok() {
            crate::core::Size {
                width: (rect.right - rect.left) as f64 * 96.0 / dpi,
                height: (rect.bottom - rect.top) as f64 * 96.0 / dpi,
            }
        } else {
            crate::core::Size::default()
        }
    }
}

fn current_dpi(hwnd: HWND) -> u32 {
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    if dpi == 0 { 96 } else { dpi }
}

fn default_display_size() -> crate::core::Size {
    unsafe {
        let monitor = MonitorFromWindow(HWND::default(), MONITOR_DEFAULTTONEAREST);
        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if GetMonitorInfoW(monitor, &mut monitor_info).as_bool() {
            let work = monitor_info.rcWork;
            crate::core::Size {
                width: (work.right - work.left) as f64 / 2.0,
                height: (work.bottom - work.top) as f64 / 2.0,
            }
        } else {
            crate::core::Size {
                width: 900.0,
                height: 560.0,
            }
        }
    }
}

fn update_color_scheme_from(fe: &FrameworkElement) {
    if let Ok(theme) = fe.ActualTheme() {
        let scheme = match theme {
            ElementTheme::Dark => crate::core::theme::ColorScheme::Dark,
            _ => crate::core::theme::ColorScheme::Light,
        };
        crate::core::theme::set_current_color_scheme(scheme);
    }
}

fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
