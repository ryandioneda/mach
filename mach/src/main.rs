use windows:: {
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    unsafe {
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
    }
}
