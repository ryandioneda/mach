use super::wndproc::wndproc;
use std::ffi::OsStr;
use std::ffi::c_void;
use std::os::windows::ffi::OsStrExt;

use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::{CreateSolidBrush, DeleteObject, HBRUSH, UpdateWindow},
        System::{Diagnostics::Debug::OutputDebugStringW, LibraryLoader::GetModuleHandleW},
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const RGB_BLACK: COLORREF = COLORREF(0x00000000);

pub struct Window {
    hwnd: HWND,
    hbrush: HBRUSH,
    hinstance: HINSTANCE,
}

impl Window {
    pub fn new(window_name: &str) -> windows::core::Result<Self> {
        let hinstance = get_module_handle().expect("Fatal: GetModuleHandleW failed");
        let hbrush = create_brush(RGB_BLACK);
        let wc = Self::build_wndclass(Some(wndproc), hinstance, hbrush);

        let x = (get_screen_width() - WIDTH) / 2;
        let y = (get_screen_height() - HEIGHT) / 2;

        match Self::register_window(&wc) {
            Ok(atom) => atom,
            Err(error) => {
                unsafe {
                    DeleteObject(hbrush);
                }
                return Err(error);
            }
        };

        let hwnd: HWND = Self::create_window(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            WS_POPUP,
            window_name,
            x,
            y,
            WIDTH,
            HEIGHT,
            None,
            None,
            Some(hinstance),
            None,
        )?;

        if hwnd.0 == 0 {
            unsafe {
                DeleteObject(hbrush);
                UnregisterClassW(w!("MachWindowClass"), hinstance);
            }
            return Err(Error::from_win32());
        }

        Ok(Window {
            hwnd,
            hbrush,
            hinstance,
        })
    }

    pub fn show_window(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
            UpdateWindow(self.hwnd);
        }
    }

    fn build_wndclass(wndproc: WNDPROC, hinstance: HINSTANCE, hbrush: HBRUSH) -> WNDCLASSW {
        WNDCLASSW {
            style: Default::default(),
            lpfnWndProc: wndproc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: Default::default(),
            hCursor: Default::default(),
            hbrBackground: hbrush,
            lpszMenuName: PCWSTR::null(),
            lpszClassName: w!("MachWindowClass"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_window(
        dwexstyle: WINDOW_EX_STYLE,
        dwstyle: WINDOW_STYLE,
        window_name: &str,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
        hwndparent: Option<HWND>,
        hmenu: Option<HMENU>,
        hinstance: Option<HINSTANCE>,
        lpparam: Option<*const c_void>,
    ) -> Result<HWND> {
        let window_name_w: Vec<u16> = convert_to_wstring(window_name);

        let hwnd = unsafe {
            CreateWindowExW(
                dwexstyle,
                w!("MachWindowClass"),
                PCWSTR(window_name_w.as_ptr()),
                dwstyle,
                x,
                y,
                n_width,
                n_height,
                hwndparent.unwrap_or_default(),
                hmenu.unwrap_or_default(),
                hinstance.unwrap_or_default(),
                lpparam,
            )
        };

        if hwnd.0 == 0 {
            return Err(Error::from_win32());
        }
        Ok(hwnd)
    }

    fn register_window(wndclass: &WNDCLASSW) -> windows::core::Result<u16> {
        let register_result = unsafe { RegisterClassW(wndclass) };
        if register_result == 0 {
            Err(Error::from_win32())
        } else {
            Ok(register_result)
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.hwnd);
            DeleteObject(self.hbrush);
            UnregisterClassW(w!("MachWindowClass"), self.hinstance);
        }
    }
}

// helper functions
fn create_brush(color: COLORREF) -> HBRUSH {
    unsafe { CreateSolidBrush(color) }
}

fn convert_to_wstring(string: &str) -> Vec<u16> {
    string.encode_utf16().chain(Some(0)).collect()
}
fn get_screen_width() -> i32 {
    const DEFAULT_WIDTH: i32 = 1024;
    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    if width == 0 { DEFAULT_WIDTH } else { width }
}

fn get_screen_height() -> i32 {
    const DEFAULT_HEIGHT: i32 = 768;
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    if height == 0 { DEFAULT_HEIGHT } else { height }
}

fn get_module_handle() -> windows::core::Result<HINSTANCE> {
    let hmod_res = unsafe { GetModuleHandleW(None) };
    match hmod_res {
        Ok(hmod) => Ok(hmod.into()),
        Err(err) => Err(err),
    }
}

fn debug_log(msg: &str) {
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(Some(0)).collect();

    unsafe {
        OutputDebugStringW(PWSTR(wide.as_ptr() as _));
    }
}
