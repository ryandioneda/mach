use windows::{
    Win32::{
        Foundation::{LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

struct Hook {
    hook_type: WINDOWS_HOOK_ID,
    hhook: HHOOK,
}

impl Hook {
    pub fn new(hook_type: WINDOWS_HOOK_ID) -> windows::core::Result<Self> {
        //calls in install_hook, pass
        let callback: HOOKPROC = Some(ll_keyboard_proc);
        let hhook = Self::install_hook(hook_type.0, callback)
            .expect("Fatal: Installing keyboard hook failed");
        Ok(Hook { hook_type, hhook })
    }

    fn install_hook(nfiltertype: i32, pfnfilterproc: HOOKPROC) -> windows::core::Result<HHOOK> {
        let hook_result = unsafe { SetWindowsHookW(nfiltertype, pfnfilterproc) };

        if hook_result.is_invalid() {
            Err(Error::from_win32())
        } else {
            Ok(hook_result)
        }
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn ll_keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        //converts lparam isize to raw pointer and then dereferences (allowing field access)
        let kb_ptr = lparam.0 as *const KBDLLHOOKSTRUCT;
        let kb = unsafe { &*kb_ptr };
    }

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}
