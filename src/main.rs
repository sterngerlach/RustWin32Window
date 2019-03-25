#![windows_subsystem = "windows"]

/* main.rs */

#[cfg(windows)] extern crate winapi;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter;
use std::mem;
use std::ptr;
use std::io::Error;

use winapi::shared::minwindef;
use winapi::shared::windef;

use winapi::um::libloaderapi;
use winapi::um::wingdi;
use winapi::um::winuser;

#[cfg(windows)]
fn convert_to_win32_wide_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide()
                     .chain(iter::once(0))
                     .collect()
}

#[cfg(windows)]
fn register_window_class(class_name: &str) -> Result<(), Error> {
    let class_name = convert_to_win32_wide_string(class_name);

    unsafe {
        let hinst = libloaderapi::GetModuleHandleW(ptr::null_mut());

        let wnd_class = winuser::WNDCLASSW {
            hInstance: hinst,
            lpszClassName: class_name.as_ptr(),
            lpfnWndProc: Some(window_proc),
            style: winuser::CS_HREDRAW | winuser::CS_VREDRAW,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: winuser::LoadIconW(ptr::null_mut(), winuser::IDI_APPLICATION),
            hCursor: winuser::LoadCursorW(ptr::null_mut(), winuser::IDC_ARROW),
            hbrBackground: wingdi::GetStockObject(wingdi::WHITE_BRUSH as i32) as windef::HBRUSH,
            lpszMenuName: ptr::null_mut(),
        };

        if winuser::RegisterClassW(&wnd_class) == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(windows)]
fn create_window(class_name: &str, title: &str) -> Result<windef::HWND, Error> {
    let class_name = convert_to_win32_wide_string(class_name);
    let title = convert_to_win32_wide_string(title);

    unsafe {
        let hinst = libloaderapi::GetModuleHandleW(ptr::null_mut());

        let hwnd = winuser::CreateWindowExW(
            0, class_name.as_ptr(), title.as_ptr(), winuser::WS_OVERLAPPEDWINDOW,
            winuser::CW_USEDEFAULT, winuser::CW_USEDEFAULT, 480, 320,
            ptr::null_mut(), ptr::null_mut(), hinst, ptr::null_mut());

        winuser::ShowWindow(hwnd, winuser::SW_SHOWDEFAULT);
        winuser::UpdateWindow(hwnd);

        if hwnd.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(hwnd)
        }
    }
}

#[cfg(windows)]
fn handle_message(hwnd: windef::HWND) -> bool {
    unsafe {
        let mut msg : winuser::MSG = mem::zeroed();
        let ret = winuser::GetMessageW(&mut msg as *mut winuser::MSG, hwnd, 0, 0);

        if ret > 0 {
            winuser::TranslateMessage(&msg as *const winuser::MSG);
            winuser::DispatchMessageW(&msg as *const winuser::MSG);
            true
        } else {
            false
        }
    }
}

#[cfg(windows)]
extern "system" fn window_proc(
    hwnd: windef::HWND, msg: minwindef::UINT,
    wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    unsafe {
        match msg {
            winuser::WM_DESTROY => {
                winuser::PostQuitMessage(0)
            },
            winuser::WM_PAINT => {
                let mut paint_struct : winuser::PAINTSTRUCT = mem::zeroed();
                let hdc = winuser::BeginPaint(hwnd, &mut paint_struct as *mut winuser::PAINTSTRUCT);
                let text = convert_to_win32_wide_string("Hello, World in Rust");

                wingdi::TextOutW(hdc, 50, 50, text.as_ptr(), (text.len() - 1) as i32);
                winuser::EndPaint(hwnd, &mut paint_struct as *const winuser::PAINTSTRUCT);
                winuser::ReleaseDC(hwnd, hdc);
            },
            _ => ()
        }

        winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

#[cfg(windows)]
fn main() {
    match register_window_class("win32-window") {
        Err(_) => panic!("register_window_class() failed"),
        Ok(_) => ()
    }

    let hwnd = create_window("win32-window", "Win32Window");

    match hwnd {
        Err(_) => panic!("create_window() failed"),
        Ok(_) => ()
    }

    let hwnd = hwnd.unwrap();

    loop {
        if !handle_message(hwnd) {
            break;
        }
    }
}
