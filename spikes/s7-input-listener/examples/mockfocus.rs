//! Windows-only test harness (not shipped): opens a window titled "Elite - Dangerous (CLIENT)",
//! takes focus, then injects synthetic keyboard/mouse input with SendInput so the listener can be
//! smoke-tested under Wine. Midway it renames itself to test the "not focused" path.
//!
//! Usage: mockfocus <start_delay_s> <run_s>

#[cfg(not(windows))]
fn main() {
    eprintln!("mockfocus is Windows-only");
}

#[cfg(windows)]
fn main() {
    use std::time::{Duration, Instant};
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

    unsafe extern "system" fn wndproc(h: HWND, m: u32, w: WPARAM, l: LPARAM) -> LRESULT {
        DefWindowProcW(h, m, w, l)
    }
    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
    unsafe fn key(vk: u16, scan: u16, up: bool) {
        let mut i: INPUT = std::mem::zeroed();
        i.r#type = INPUT_KEYBOARD;
        i.Anonymous.ki = KEYBDINPUT { wVk: vk, wScan: scan, dwFlags: if up { KEYEVENTF_KEYUP } else { 0 }, time: 0, dwExtraInfo: 0 };
        SendInput(1, &i, std::mem::size_of::<INPUT>() as i32);
    }
    unsafe fn mouse(flags: u32) {
        let mut i: INPUT = std::mem::zeroed();
        i.r#type = INPUT_MOUSE;
        i.Anonymous.mi = MOUSEINPUT { dx: 0, dy: 0, mouseData: 0, dwFlags: flags, time: 0, dwExtraInfo: 0 };
        SendInput(1, &i, std::mem::size_of::<INPUT>() as i32);
    }
    let a: Vec<String> = std::env::args().collect();
    let delay: u64 = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(3);
    let run: u64 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
    unsafe {
        let inst = GetModuleHandleW(std::ptr::null());
        let class = wide("S7MockElite");
        let wc = WNDCLASSW { lpfnWndProc: Some(wndproc), hInstance: inst, lpszClassName: class.as_ptr(), ..std::mem::zeroed() };
        RegisterClassW(&wc);
        let title = wide("Elite - Dangerous (CLIENT)");
        let hwnd = CreateWindowExW(0, class.as_ptr(), title.as_ptr(), WS_OVERLAPPEDWINDOW | WS_VISIBLE, 100, 100, 400, 300, std::ptr::null_mut(), std::ptr::null_mut(), inst, std::ptr::null());
        SetForegroundWindow(hwnd);
        let start = Instant::now();
        let pump = |until: Duration| {
            let mut msg: MSG = std::mem::zeroed();
            while start.elapsed() < until {
                while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
                std::thread::sleep(Duration::from_millis(5));
            }
        };
        let mut t = delay * 1000;
        let mut at = |dt: u64| {
            t += dt;
            pump(Duration::from_millis(t));
        };
        at(0);
        eprintln!("mockfocus: injecting (focused as Elite)");
        key(0x41, 0x1E, false); // A
        at(80);
        key(0x41, 0x1E, false); // auto-repeat style second down
        at(80);
        key(0x41, 0x1E, true);
        at(150);
        key(0x20, 0x39, false); // Space
        at(120);
        key(0x20, 0x39, true);
        at(150);
        key(0x4C, 0x26, false); // L (bound to LandingGearToggle in the test preset)
        at(100);
        key(0x4C, 0x26, true);
        at(150);
        SetCursorPos(250, 250); // inside the mock window, so the click doesn't steal focus
        at(50);
        mouse(MOUSEEVENTF_LEFTDOWN);
        at(80);
        mouse(MOUSEEVENTF_LEFTUP);
        at(500);
        let other = wide("Mock other window");
        SetWindowTextW(hwnd, other.as_ptr());
        eprintln!("mockfocus: renamed (not Elite)");
        at(500);
        key(0x42, 0x30, false); // B
        at(100);
        key(0x42, 0x30, true);
        at(500);
        SetWindowTextW(hwnd, title.as_ptr());
        eprintln!("mockfocus: back to Elite title");
        at(500);
        key(0x43, 0x2E, false); // C down, never released (tests "down without up")
        pump(Duration::from_secs(run));
        key(0x43, 0x2E, true);
        pump(Duration::from_secs(run) + Duration::from_millis(200));
    }
}
