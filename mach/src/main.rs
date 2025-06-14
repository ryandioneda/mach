mod window;
use window::Window;

use windows::{
    Win32::{System::Diagnostics::Debug::OutputDebugStringW, UI::WindowsAndMessaging::*},
    core::*,
};

fn main() -> Result<()> {
    let win = Window::new("Mach Overlay Window")?;
    win.show_window();

    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}
