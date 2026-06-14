#![windows_subsystem = "windows"]

use std::{
    ffi::c_void,
    sync::atomic::{AtomicPtr, Ordering},
};

use windows::{
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        System::{
            LibraryLoader::GetModuleHandleW,
            WinRT::{RO_INIT_SINGLETHREADED, RoInitialize},
        },
        UI::WindowsAndMessaging::{
            CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DispatchMessageW,
            GWLP_USERDATA, GetClientRect, GetMessageW, HMENU, IDC_ARROW, LoadCursorW, MSG,
            PostQuitMessage, RegisterClassW, SW_SHOW, SWP_NOACTIVATE, SWP_NOZORDER, SWP_SHOWWINDOW,
            SetWindowLongPtrW, SetWindowPos, ShowWindow, TranslateMessage, WINDOW_EX_STYLE,
            WM_DESTROY, WM_NCCREATE, WM_SIZE, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
    core::{Error, HSTRING, Interface, PCWSTR, Result},
};

use xaml_bindings::Windows::{
    UI::Xaml::{
        Controls::TextBlock,
        Hosting::{DesktopWindowXamlSource, WindowsXamlManager},
        UIElement,
    },
    Win32::System::WinRT::Xaml::IDesktopWindowXamlSourceNative,
};

static XAML_HWND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

fn main() -> Result<()> {
    unsafe {
        RoInitialize(RO_INIT_SINGLETHREADED)?;
    }

    let _xaml_manager = WindowsXamlManager::InitializeForCurrentThread()?;
    let hwnd = create_main_window()?;

    let xaml_source = DesktopWindowXamlSource::new()?;
    let native_source: IDesktopWindowXamlSourceNative = xaml_source.cast()?;

    unsafe {
        native_source.AttachToWindow(hwnd)?;
    }

    let xaml_hwnd = unsafe { native_source.WindowHandle()? };
    XAML_HWND.store(xaml_hwnd.0, Ordering::Relaxed);

    let text = TextBlock::new()?;
    text.SetText(&HSTRING::from("Hello from Rust + windows-rs XAML Islands"))?;
    text.SetFontSize(28.0)?;

    let content: UIElement = text.cast()?;
    xaml_source.SetContent(&content)?;

    resize_xaml_island(hwnd);

    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
    }

    message_loop()
}

fn create_main_window() -> Result<HWND> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(PCWSTR::null())?.into();

        let class_name = wide_null("IslandReactor_Window");
        let title = wide_null("island-reactor");
        let class_name = PCWSTR(class_name.as_ptr());

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

        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            PCWSTR(title.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            900,
            560,
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

fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
