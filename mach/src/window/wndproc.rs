use windows::{
    Win32::{
        Foundation::*, System::Diagnostics::Debug::OutputDebugStringW, UI::WindowsAndMessaging::*,
    },
    core::*,
};

#[allow(non_snake_case)]
pub unsafe extern "system" fn wndproc(
    hwnd: HWND,
    uMsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match uMsg {
        WM_PAINT => {
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, uMsg, wparam, lparam) },
    }
}
