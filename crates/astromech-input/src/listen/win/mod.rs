//! Windows-only: Raw Input listener, XInput poller, foreground window title. All read-only.

pub mod rawinput;
pub mod xinput;

pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub enum Msg {
    Device(crate::listen::model::DeviceMeta),
    Sample(crate::listen::model::Sample),
}

pub fn foreground_title() -> String {
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
    unsafe {
        let h = GetForegroundWindow();
        if h.is_null() {
            return String::new();
        }
        let mut buf = [0u16; 512];
        let n = GetWindowTextW(h, buf.as_mut_ptr(), buf.len() as i32);
        String::from_utf16_lossy(&buf[..n.max(0) as usize])
    }
}
