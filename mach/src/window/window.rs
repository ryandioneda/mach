use super::wndproc::wndproc;
use std::{convert, ffi::c_void};

use windows::{
    Win32::{
        Foundation::*,
        Graphics::Gdi::UpdateWindow,
        System::{Diagnostics::Debug::OutputDebugStringW, LibraryLoader::GetModuleHandleW},
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

pub struct Window {
    hwnd: HWND,
}

impl Window {
    pub fn new(window_name: &str) -> Result<Self> {
        let hinstance = get_module_handle().expect("Fatal: GetModuleHandleW failed");
        let wc = Self::build_wndclass(Some(wndproc), hinstance);

        let x = (get_screen_width() - WIDTH) / 2;
        let y = (get_screen_height() - HEIGHT) / 2;

        Self::register_window(&wc).expect("Fatal: RegisterClassW failed");

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
            return Err(Error::from_win32());
        }

        Ok(Self { hwnd })
    }

    pub fn show_window(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
            UpdateWindow(self.hwnd);
        }
    }

    fn build_wndclass(wndproc: WNDPROC, hinstance: HINSTANCE) -> WNDCLASSW {
        WNDCLASSW {
            style: Default::default(),
            lpfnWndProc: wndproc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: Default::default(),
            hCursor: Default::default(),
            hbrBackground: Default::default(),
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
        unsafe {
            OutputDebugStringW(w!("Created Window!"));
        }

        if hwnd.0 == 0 {
            return Err(Error::from_win32());
        }
        Ok(hwnd)
    }

    fn register_window(wndclass: &WNDCLASSW) -> std::result::Result<(), Error> {
        let register_result = unsafe { RegisterClassW(wndclass) };
        if register_result == 0 {
            Err(Error::from_win32())
        } else {
            Ok(())
        }
    }
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

fn get_module_handle() -> std::result::Result<HINSTANCE, Error> {
    let hmod_res = unsafe { GetModuleHandleW(None) };
    match hmod_res {
        Ok(hmod) => Ok(hmod.into()),
        Err(err) => Err(err),
    }
}
