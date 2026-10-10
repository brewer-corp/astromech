//! OS access behind a trait so the measurement loop can be tested with a mock game.

use crate::keys::ScanKey;

pub const GAME_WINDOW_TITLE: &str = "Elite - Dangerous (CLIENT)";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameIntegrity {
    NotRunning,
    Level(u32),
    /// Could not open the game process/token (usually means it runs elevated).
    Unreadable(String),
}

#[derive(Debug, Clone)]
pub struct IntegrityReport {
    pub own: Option<u32>,
    pub game: GameIntegrity,
}

pub fn level_name(rid: u32) -> &'static str {
    match rid {
        r if r >= 0x4000 => "System",
        r if r >= 0x3000 => "High (elevated)",
        r if r >= 0x2100 => "Medium-Plus",
        r if r >= 0x2000 => "Medium (normal)",
        r if r >= 0x1000 => "Low",
        _ => "Untrusted",
    }
}

impl IntegrityReport {
    /// Human-readable verdict; Err when key presses would likely be blocked or the setup is odd.
    pub fn verdict(&self) -> Result<String, String> {
        let own = match self.own {
            Some(o) => o,
            None => return Ok("integrity check unavailable on this platform".into()),
        };
        match &self.game {
            GameIntegrity::NotRunning => Ok(format!(
                "this tool: {}; game window '{GAME_WINDOW_TITLE}' not found yet",
                level_name(own)
            )),
            GameIntegrity::Unreadable(e) => Err(format!(
                "cannot read the game's integrity level ({e}). This usually means the game runs as \
                 administrator and this tool does not: Windows (UIPI) will silently drop the key presses. \
                 Run both normally (recommended) or both as administrator."
            )),
            GameIntegrity::Level(g) if *g > own => Err(format!(
                "the game runs at {} but this tool at {}: Windows (UIPI) will silently drop the key presses. \
                 Restart the game without 'Run as administrator' (recommended), or run this tool elevated too.",
                level_name(*g),
                level_name(own)
            )),
            GameIntegrity::Level(g) if *g < own => Err(format!(
                "this tool runs at {} but the game at {}. Presses will work, but running elevated is not \
                 needed; please run this tool normally.",
                level_name(own),
                level_name(*g)
            )),
            GameIntegrity::Level(g) => Ok(format!("this tool and the game both run at {} (OK)", level_name(*g))),
        }
    }
}

pub trait Platform: Send + Sync {
    fn foreground_title(&self) -> Option<String>;
    /// Send one key event (down or up) as a hardware scancode.
    fn send_key(&self, key: ScanKey, up: bool) -> Result<(), String>;
    fn integrity(&self) -> IntegrityReport;
    /// Raise the OS timer resolution for accurate sleeps (Windows: timeBeginPeriod(1)).
    fn begin_precise_timing(&self) {}
    fn end_precise_timing(&self) {}
}

#[cfg(windows)]
pub mod win {
    use super::*;
    use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
    use windows_sys::Win32::Media::{timeBeginPeriod, timeEndPeriod};
    use windows_sys::Win32::Security::{
        GetSidSubAuthority, GetSidSubAuthorityCount, GetTokenInformation, TokenIntegrityLevel,
        TOKEN_MANDATORY_LABEL, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{
        GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP,
        KEYEVENTF_SCANCODE,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        FindWindowW, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    };

    pub struct WinPlatform;

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    unsafe fn token_integrity(process: HANDLE) -> Result<u32, String> {
        let mut token: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(process, TOKEN_QUERY, &mut token) == 0 {
            return Err(format!("OpenProcessToken failed, error {}", GetLastError()));
        }
        let mut len = 0u32;
        GetTokenInformation(token, TokenIntegrityLevel, std::ptr::null_mut(), 0, &mut len);
        let mut buf = vec![0u8; len.max(64) as usize];
        let ok = GetTokenInformation(token, TokenIntegrityLevel, buf.as_mut_ptr() as *mut _, buf.len() as u32, &mut len);
        CloseHandle(token);
        if ok == 0 {
            return Err(format!("GetTokenInformation failed, error {}", GetLastError()));
        }
        let label = &*(buf.as_ptr() as *const TOKEN_MANDATORY_LABEL);
        let sid = label.Label.Sid;
        let count = *GetSidSubAuthorityCount(sid) as u32;
        if count == 0 {
            return Err("integrity SID has no sub-authority".into());
        }
        Ok(*GetSidSubAuthority(sid, count - 1))
    }

    impl Platform for WinPlatform {
        fn foreground_title(&self) -> Option<String> {
            unsafe {
                let h = GetForegroundWindow();
                if h.is_null() {
                    return None;
                }
                let mut buf = [0u16; 512];
                let n = GetWindowTextW(h, buf.as_mut_ptr(), buf.len() as i32);
                if n <= 0 {
                    return Some(String::new());
                }
                Some(String::from_utf16_lossy(&buf[..n as usize]))
            }
        }

        fn send_key(&self, key: ScanKey, up: bool) -> Result<(), String> {
            let mut flags = KEYEVENTF_SCANCODE;
            if key.extended {
                flags |= KEYEVENTF_EXTENDEDKEY;
            }
            if up {
                flags |= KEYEVENTF_KEYUP;
            }
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT { wVk: 0, wScan: key.scan, dwFlags: flags, time: 0, dwExtraInfo: 0 },
                },
            };
            let sent = unsafe { SendInput(1, &input, std::mem::size_of::<INPUT>() as i32) };
            if sent == 1 {
                Ok(())
            } else {
                Err(format!(
                    "SendInput inserted {sent} events (error {}). Input may be blocked by UIPI \
                     (game elevated?) or another program.",
                    unsafe { GetLastError() }
                ))
            }
        }

        fn integrity(&self) -> IntegrityReport {
            unsafe {
                let own = token_integrity(GetCurrentProcess()).ok();
                let title = wide(GAME_WINDOW_TITLE);
                let hwnd = FindWindowW(std::ptr::null(), title.as_ptr());
                if hwnd.is_null() {
                    return IntegrityReport { own, game: GameIntegrity::NotRunning };
                }
                let mut pid = 0u32;
                GetWindowThreadProcessId(hwnd, &mut pid);
                let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                if h.is_null() {
                    return IntegrityReport {
                        own,
                        game: GameIntegrity::Unreadable(format!("OpenProcess(pid {pid}) failed, error {}", GetLastError())),
                    };
                }
                let g = token_integrity(h);
                CloseHandle(h);
                IntegrityReport {
                    own,
                    game: match g {
                        Ok(l) => GameIntegrity::Level(l),
                        Err(e) => GameIntegrity::Unreadable(e),
                    },
                }
            }
        }

        fn begin_precise_timing(&self) {
            unsafe { timeBeginPeriod(1) };
        }
        fn end_precise_timing(&self) {
            unsafe { timeEndPeriod(1) };
        }
    }
}

/// Platform used where key injection is impossible (non-Windows dry runs).
pub struct NoInputPlatform;

impl Platform for NoInputPlatform {
    fn foreground_title(&self) -> Option<String> {
        None
    }
    fn send_key(&self, _key: ScanKey, _up: bool) -> Result<(), String> {
        Err("key injection is only available on Windows".into())
    }
    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: None, game: GameIntegrity::NotRunning }
    }
}

/// A fake game for tests: when the target key goes down, it rewrites Status.json with the bit
/// toggled after `delay`, first writing a truncated file to exercise partial-write handling.
pub mod mock {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    pub struct MockGame {
        pub status_path: PathBuf,
        pub target: ScanKey,
        pub bit: u64,
        pub delay: Duration,
        pub flags: Arc<Mutex<u64>>,
        pub focused: AtomicBool,
        /// Ignore this many presses (to simulate misses).
        pub drop_presses: AtomicU32,
        pub log: Mutex<Vec<(ScanKey, bool)>>,
    }

    impl MockGame {
        pub fn new(status_path: PathBuf, target: ScanKey, bit: u64, initial_flags: u64, delay: Duration) -> Self {
            let m = Self {
                status_path,
                target,
                bit,
                delay,
                flags: Arc::new(Mutex::new(initial_flags)),
                focused: AtomicBool::new(true),
                drop_presses: AtomicU32::new(0),
                log: Mutex::new(vec![]),
            };
            write_status(&m.status_path, initial_flags, false);
            m
        }
        pub fn current_flags(&self) -> u64 {
            *self.flags.lock().unwrap()
        }
    }

    pub fn status_json(flags: u64) -> String {
        format!(
            "{{ \"timestamp\":\"2026-10-08T22:40:00Z\", \"event\":\"Status\", \"Flags\":{flags}, \"Flags2\":0, \
             \"Pips\":[4,4,4], \"FireGroup\":0, \"GuiFocus\":0, \"Fuel\":{{ \"FuelMain\":32.0, \"FuelReservoir\":0.63 }}, \
             \"Cargo\":0.0, \"LegalState\":\"Clean\", \"Balance\":1000 }}\r\n"
        )
    }

    pub fn write_status(path: &std::path::Path, flags: u64, partial_first: bool) {
        let full = status_json(flags);
        if partial_first {
            let _ = std::fs::write(path, &full.as_bytes()[..full.len() / 2]);
            std::thread::sleep(Duration::from_millis(3));
        }
        let _ = std::fs::write(path, full);
    }

    impl Platform for MockGame {
        fn foreground_title(&self) -> Option<String> {
            Some(if self.focused.load(Ordering::SeqCst) { GAME_WINDOW_TITLE.into() } else { "Notepad".into() })
        }
        fn send_key(&self, key: ScanKey, up: bool) -> Result<(), String> {
            self.log.lock().unwrap().push((key, up));
            if key == self.target && !up {
                if self.drop_presses.load(Ordering::SeqCst) > 0 {
                    self.drop_presses.fetch_sub(1, Ordering::SeqCst);
                    return Ok(());
                }
                let flags = self.flags.clone();
                let path = self.status_path.clone();
                let bit = self.bit;
                let delay = self.delay;
                std::thread::spawn(move || {
                    std::thread::sleep(delay);
                    let mut f = flags.lock().unwrap();
                    *f ^= bit;
                    write_status(&path, *f, true);
                });
            }
            Ok(())
        }
        fn integrity(&self) -> IntegrityReport {
            IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
        }
    }
}
