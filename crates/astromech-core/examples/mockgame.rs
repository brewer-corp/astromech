//! Windows-only mock of the game's input + Status.json surface, for smoke-testing the real exe
//! (e.g. under Wine). It opens a window titled "Elite - Dangerous (CLIENT)", and when it receives
//! WM_KEYDOWN for the target scancode it toggles a Flags bit in Status.json after a delay.
//! Every key event it receives is appended to keys.log next to Status.json.
//!
//! Extended from the S5 mock for the gear-down slice: a delay *range* (S5 measured 0-1000 ms),
//! dropping the first N presses (to exercise the retry), and a journal file so the core's
//! pipeline has something to follow.
//!
//! Usage: mockgame <journal_dir> <scancode_hex> <extended 0|1> <bit> <delay_ms|min-max> <run_seconds> [drop_presses]

#[cfg(not(windows))]
fn main() {
    eprintln!("mockgame is Windows-only");
}

#[cfg(windows)]
fn main() {
    win::main();
}

#[cfg(windows)]
mod win {
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, Instant};
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

    struct State {
        dir: PathBuf,
        scan: u32,
        ext: u32,
        bit: u64,
        delay: (u64, u64),
        drop: Mutex<u32>,
        flags: Mutex<u64>,
    }
    static STATE: OnceLock<State> = OnceLock::new();

    fn status_json(flags: u64) -> String {
        format!("{{ \"timestamp\":\"2026-10-08T22:40:00Z\", \"event\":\"Status\", \"Flags\":{flags}, \"Flags2\":0, \"GuiFocus\":0 }}\r\n")
    }

    fn log(line: &str) {
        let s = STATE.get().unwrap();
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(s.dir.join("keys.log")).unwrap();
        writeln!(f, "{line}").unwrap();
    }

    unsafe extern "system" fn wndproc(h: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
        match msg {
            WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => {
                let scan = ((l >> 16) & 0xFF) as u32;
                let ext = ((l >> 24) & 1) as u32;
                let up = msg == WM_KEYUP || msg == WM_SYSKEYUP;
                let repeat = (l >> 30) & 1 == 1 && !up;
                log(&format!("{} scan={scan:02X} ext={ext} repeat={repeat}", if up { "up" } else { "down" }));
                let s = STATE.get().unwrap();
                if !up && !repeat && scan == s.scan && ext == s.ext {
                    {
                        let mut d = s.drop.lock().unwrap();
                        if *d > 0 {
                            *d -= 1;
                            log("dropped this press (simulated miss)");
                            return 0;
                        }
                    }
                    std::thread::spawn(|| {
                        let s = STATE.get().unwrap();
                        let (lo, hi) = s.delay;
                        let nanos =
                            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos()
                                as u64;
                        let ms = lo + if hi > lo { nanos % (hi - lo + 1) } else { 0 };
                        log(&format!("toggling bit after {ms} ms"));
                        std::thread::sleep(Duration::from_millis(ms));
                        let mut f = s.flags.lock().unwrap();
                        *f ^= s.bit;
                        std::fs::write(s.dir.join("Status.json"), status_json(*f)).unwrap();
                    });
                }
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(h, msg, w, l),
        }
    }

    pub fn main() {
        let a: Vec<String> = std::env::args().collect();
        let dir = PathBuf::from(&a[1]);
        let st = State {
            dir: dir.clone(),
            scan: u32::from_str_radix(a[2].trim_start_matches("0x"), 16).unwrap(),
            ext: a[3].parse().unwrap(),
            bit: a[4].parse().unwrap(),
            delay: match a[5].split_once('-') {
                Some((x, y)) => (x.parse().unwrap(), y.parse().unwrap()),
                None => (a[5].parse().unwrap(), a[5].parse().unwrap()),
            },
            drop: Mutex::new(a.get(7).map(|v| v.parse().unwrap()).unwrap_or(0)),
            flags: Mutex::new(1 << 24),
        };
        let run_for = Duration::from_secs(a[6].parse().unwrap());
        std::fs::write(dir.join("Status.json"), status_json(1 << 24)).unwrap();
        std::fs::write(
            dir.join("Journal.2026-10-10T200000.01.log"),
            "{ \"timestamp\":\"2026-10-10T20:00:00Z\", \"event\":\"Fileheader\", \"part\":1, \"gameversion\":\"4.2.0\", \"build\":\"mock\" }\r\n\
             { \"timestamp\":\"2026-10-10T20:00:05Z\", \"event\":\"LoadGame\", \"Commander\":\"Mock\", \"Ship\":\"krait_mkii\" }\r\n\
             { \"timestamp\":\"2026-10-10T20:00:06Z\", \"event\":\"Location\", \"StarSystem\":\"Shinrarta Dezhra\", \"Docked\":false }\r\n",
        )
        .unwrap();
        let _ = std::fs::remove_file(dir.join("keys.log"));
        let _ = STATE.set(st);
        unsafe {
            let inst = GetModuleHandleW(std::ptr::null());
            let class: Vec<u16> = "S5MockGame\0".encode_utf16().collect();
            let title: Vec<u16> = "Elite - Dangerous (CLIENT)\0".encode_utf16().collect();
            let wc = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: inst,
                hIcon: std::ptr::null_mut(),
                hCursor: std::ptr::null_mut(),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null(),
                lpszClassName: class.as_ptr(),
            };
            RegisterClassW(&wc);
            let hwnd = CreateWindowExW(
                0,
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                0,
                0,
                400,
                300,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                inst,
                std::ptr::null(),
            );
            SetForegroundWindow(hwnd);
            log("ready");
            let start = Instant::now();
            let mut msg: MSG = std::mem::zeroed();
            while start.elapsed() < run_for {
                while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
