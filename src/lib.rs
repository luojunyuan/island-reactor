use std::{
    ffi::c_void,
    sync::atomic::{AtomicPtr, Ordering},
};

use windows::{
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        System::{
            LibraryLoader::{GetModuleHandleW, GetProcAddress},
            WinRT::{RO_INIT_SINGLETHREADED, RoInitialize},
        },
        UI::WindowsAndMessaging::{
            CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DispatchMessageW,
            GWLP_USERDATA, GetClientRect, GetMessageW, HMENU, IDC_ARROW, LoadCursorW, MSG,
            PostQuitMessage, RegisterClassW, SW_HIDE, SW_SHOW, SWP_NOACTIVATE, SWP_NOZORDER,
            SWP_SHOWWINDOW, SendMessageW, SetWindowLongPtrW, SetWindowPos, ShowWindow,
            TranslateMessage, WINDOW_EX_STYLE, WM_DESTROY, WM_NCCREATE, WM_SIZE, WNDCLASSW,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
    core::{BOOL, HSTRING, Interface, PCSTR, PCWSTR},
};

pub use windows::core::{Error, Result};
pub use xaml_bindings::Windows::UI::Xaml::{Controls::Orientation, TextWrapping};

use xaml_bindings::{
    Windows::{
        UI::{
            Text::FontWeight,
            Xaml::{
                Controls::{StackPanel as XamlStackPanel, TextBlock as XamlTextBlock},
                Hosting::{DesktopWindowXamlSource, WindowsXamlManager},
                UIElement,
            },
        },
        Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative,
    },
    interop::{Application, CoreWindow, ICoreWindowInterop, IFrameworkApplicationPrivate},
};

static XAML_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static CORE_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

pub struct App {
    title: Option<String>,
    inner_size: Option<WindowSize>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            title: None,
            inner_size: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn inner_size(mut self, width: f64, height: f64) -> Self {
        self.inner_size = Some(WindowSize { width, height });
        self
    }

    pub fn render<F>(self, f: F) -> Result<()>
    where
        F: Fn(&mut RenderCx) -> Element,
    {
        let mut cx = RenderCx::new();
        let root = f(&mut cx);
        run_island_app(
            self.title.unwrap_or_else(|| "island-reactor".to_string()),
            self.inner_size,
            root,
        )
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RenderCx {
    _private: (),
}

impl RenderCx {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn for_test() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    TextBlock(TextBlock),
    StackPanel(StackPanel),
    Empty,
}

impl Default for Element {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<TextBlock> for Element {
    fn from(value: TextBlock) -> Self {
        Self::TextBlock(value)
    }
}

impl From<StackPanel> for Element {
    fn from(value: StackPanel) -> Self {
        Self::StackPanel(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextBlock {
    pub modifiers: Modifiers,
    pub text: String,
    pub font_size: Option<f64>,
    pub font_weight: Option<u16>,
    pub text_wrapping: TextWrapping,
}

impl TextBlock {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            modifiers: Modifiers::default(),
            text: content.into(),
            font_size: None,
            font_weight: None,
            text_wrapping: TextWrapping::NoWrap,
        }
    }

    pub fn bold(mut self) -> Self {
        self.font_weight = Some(700);
        self
    }

    pub fn semibold(mut self) -> Self {
        self.font_weight = Some(600);
        self
    }

    pub fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn font_size(mut self, value: f64) -> Self {
        self.font_size = Some(value);
        self
    }

    pub fn wrap(mut self) -> Self {
        self.text_wrapping = TextWrapping::Wrap;
        self
    }
}

pub fn text_block(content: impl Into<String>) -> TextBlock {
    TextBlock::new(content)
}

#[derive(Clone, Debug, PartialEq)]
pub struct StackPanel {
    pub modifiers: Modifiers,
    pub orientation: Orientation,
    pub spacing: f64,
    pub children: Vec<Element>,
}

impl StackPanel {
    pub fn vertical() -> Self {
        Self {
            modifiers: Modifiers::default(),
            orientation: Orientation::Vertical,
            spacing: 0.0,
            children: Vec::new(),
        }
    }

    pub fn horizontal() -> Self {
        Self {
            modifiers: Modifiers::default(),
            orientation: Orientation::Horizontal,
            spacing: 0.0,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, value: f64) -> Self {
        self.spacing = value;
        self
    }
}

pub fn vstack(children: impl IntoElements) -> StackPanel {
    let mut stack = StackPanel::vertical();
    stack.children = children.into_elements();
    stack
}

pub fn hstack(children: impl IntoElements) -> StackPanel {
    let mut stack = StackPanel::horizontal();
    stack.children = children.into_elements();
    stack
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Modifiers {
    pub margin: Option<Thickness>,
    pub padding: Option<Thickness>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Thickness {
    pub const fn uniform(value: f64) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    pub const fn xy(x: f64, y: f64) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }
}

impl From<f64> for Thickness {
    fn from(value: f64) -> Self {
        Self::uniform(value)
    }
}

impl From<Thickness> for xaml_bindings::Windows::UI::Xaml::Thickness {
    fn from(value: Thickness) -> Self {
        Self {
            Left: value.left,
            Top: value.top,
            Right: value.right,
            Bottom: value.bottom,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct WindowSize {
    pub width: f64,
    pub height: f64,
}

pub trait ElementExt: Sized {
    fn modifiers_mut(&mut self) -> &mut Modifiers;

    fn margin(mut self, value: impl Into<Thickness>) -> Self {
        self.modifiers_mut().margin = Some(value.into());
        self
    }

    fn padding(mut self, value: impl Into<Thickness>) -> Self {
        self.modifiers_mut().padding = Some(value.into());
        self
    }
}

impl ElementExt for TextBlock {
    fn modifiers_mut(&mut self) -> &mut Modifiers {
        &mut self.modifiers
    }
}

impl ElementExt for StackPanel {
    fn modifiers_mut(&mut self) -> &mut Modifiers {
        &mut self.modifiers
    }
}

pub trait IntoElements {
    fn into_elements(self) -> Vec<Element>;
}

impl IntoElements for Vec<Element> {
    fn into_elements(self) -> Vec<Element> {
        self
    }
}

impl IntoElements for () {
    fn into_elements(self) -> Vec<Element> {
        Vec::new()
    }
}

impl<T: Into<Element>, const N: usize> IntoElements for [T; N] {
    fn into_elements(self) -> Vec<Element> {
        self.into_iter().map(Into::into).collect()
    }
}

macro_rules! impl_into_elements_for_tuple {
    ($($idx:tt : $ty:ident),+ $(,)?) => {
        impl<$($ty: Into<Element>),+> IntoElements for ($($ty,)+) {
            fn into_elements(self) -> Vec<Element> {
                vec![$(self.$idx.into()),+]
            }
        }
    };
}

impl_into_elements_for_tuple!(0: A);
impl_into_elements_for_tuple!(0: A, 1: B);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L);

fn run_island_app(title: String, inner_size: Option<WindowSize>, root: Element) -> Result<()> {
    unsafe {
        RoInitialize(RO_INIT_SINGLETHREADED)?;
    }

    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;
    initialize_core_window_handle();

    let hwnd = create_main_window(&title, inner_size)?;

    let xaml_source = DesktopWindowXamlSource::new()?;
    let native_source: IDesktopWindowXamlSourceNative = xaml_source.cast()?;

    unsafe {
        native_source.AttachToWindow(hwnd)?;
    }

    let xaml_hwnd = unsafe { native_source.WindowHandle()? };
    XAML_HWND.store(xaml_hwnd.0, Ordering::Relaxed);
    enable_resize_layout_synchronization(hwnd, true);

    let content = render_element(&root)?;
    xaml_source.SetContent(&content)?;

    resize_xaml_island(hwnd);

    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
    }

    message_loop()
}

fn render_element(element: &Element) -> Result<UIElement> {
    match element {
        Element::TextBlock(text) => render_text_block(text),
        Element::StackPanel(stack) => render_stack_panel(stack),
        Element::Empty => {
            let panel = XamlStackPanel::new()?;
            panel.cast()
        }
    }
}

fn render_text_block(text: &TextBlock) -> Result<UIElement> {
    let control = XamlTextBlock::new()?;
    control.SetText(&HSTRING::from(text.text.as_str()))?;
    control.SetTextWrapping(text.text_wrapping)?;

    if let Some(value) = text.font_size {
        control.SetFontSize(value)?;
    }
    if let Some(weight) = text.font_weight {
        control.SetFontWeight(FontWeight { Weight: weight })?;
    }
    if let Some(margin) = text.modifiers.margin {
        control.SetMargin(margin.into())?;
    }

    control.cast()
}

fn render_stack_panel(stack: &StackPanel) -> Result<UIElement> {
    let control = XamlStackPanel::new()?;
    control.SetOrientation(stack.orientation)?;
    control.SetSpacing(stack.spacing)?;

    if let Some(margin) = stack.modifiers.margin {
        control.SetMargin(margin.into())?;
    }
    if let Some(padding) = stack.modifiers.padding {
        control.SetPadding(padding.into())?;
    }

    let children = control.Children()?;
    for child in &stack.children {
        if matches!(child, Element::Empty) {
            continue;
        }
        let child = render_element(child)?;
        children.Append(&child)?;
    }

    control.cast()
}

fn create_main_window(title: &str, inner_size: Option<WindowSize>) -> Result<HWND> {
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

        if RegisterClassW(&window_class) == 0 {
            return Err(Error::from_thread());
        }

        let (width, height) = inner_size
            .map(|size| (size.width.round() as i32, size.height.round() as i32))
            .unwrap_or((900, 560));

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

fn message_loop() -> Result<()> {
    unsafe {
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        Ok(())
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
            let xaml_hwnd = HWND(raw_xaml_hwnd);

            let _ = SetWindowPos(
                xaml_hwnd,
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
    let Ok(core_window) = CoreWindow::GetForCurrentThread() else {
        return;
    };

    let Ok(core_window_interop) = core_window.cast::<ICoreWindowInterop>() else {
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
    let Ok(application) = Application::Current() else {
        return;
    };

    let Ok(framework_app_private) = application.cast::<IFrameworkApplicationPrivate>() else {
        return;
    };

    unsafe {
        let _ = framework_app_private.SetSynchronizationWindow(hwnd);
    }
}

fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
