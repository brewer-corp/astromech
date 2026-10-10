//! Glue: the Windows S7 environment, PTT source for S9, integrity check, Status.json reader.

use crate::input::{is_escape, Binding, InputEvent};
use crate::s7::{Cue, Env, StatusSnap};
use crate::voice::{PttEvent, PttSource};
use crate::ConsoleUi;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
use windows_sys::Win32::Security::{GetSidSubAuthority, GetSidSubAuthorityCount, GetTokenInformation, TokenIntegrityLevel, TOKEN_MANDATORY_LABEL, TOKEN_QUERY};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId};

pub struct WinEnv {
    pub rx: Receiver<InputEvent>,
    pub ui: ConsoleUi,
    pub status_path: PathBuf,
    pub sound: bool,
    pub print_cues: bool,
}

impl Env for WinEnv {
    fn recv(&self, timeout: Duration) -> Option<InputEvent> {
        self.rx.recv_timeout(timeout).ok()
    }
    fn cue(&self, c: Cue) {
        if self.print_cues {
            println!("[cue] {c:?}");
        }
        if self.sound {
            super::sound::play_cue(c);
        }
    }
    fn foreground(&self) -> String {
        super::rawinput::foreground_title()
    }
    fn status(&self) -> Option<StatusSnap> {
        read_status(&self.status_path)
    }
    fn out(&self, s: &str) {
        self.ui.out_line(s);
    }
    fn ask(&self, q: &str) -> String {
        self.ui.ask_line(q)
    }
}

pub fn read_status(p: &std::path::Path) -> Option<StatusSnap> {
    let t = std::fs::read_to_string(p).ok()?;
    let v: serde_json::Value = serde_json::from_str(&t).ok()?;
    let key = ["Flags", "Flags2", "GuiFocus", "FireGroup", "Pips", "LegalState", "Cargo"]
        .iter()
        .map(|k| v.get(*k).map(|x| x.to_string()).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("|");
    Some(StatusSnap { key })
}

/// PTT for the S9 mic test: the learned binding (either API), or Space if none was learned.
pub struct WinPtt<'a> {
    pub env: &'a WinEnv,
    pub binding: Option<Binding>,
}

impl WinPtt<'_> {
    fn is_ptt(&self, ev: &InputEvent) -> bool {
        match &self.binding {
            Some(b) => b.matches_any(ev),
            None => matches!(ev.control, crate::input::Control::Key { vkey: 0x20, .. }),
        }
    }
}

impl PttSource for WinPtt<'_> {
    fn describe(&self) -> String {
        match &self.binding {
            Some(b) => b.primary.describe(),
            None => "Space bar (no PTT learned in S7)".into(),
        }
    }
    fn next(&self, timeout: Duration) -> Option<PttEvent> {
        let end = std::time::Instant::now() + timeout;
        loop {
            let left = end.saturating_duration_since(std::time::Instant::now());
            if left.is_zero() {
                return None;
            }
            let ev = self.env.rx.recv_timeout(left).ok()?;
            if ev.down && is_escape(&ev.control) {
                return Some(PttEvent::Cancel);
            }
            if !self.is_ptt(&ev) {
                continue;
            }
            // Keyboard auto-repeat sends repeated "down"s; the caller ignores extras.
            return Some(if ev.down { PttEvent::Down(ev.at) } else { PttEvent::Up(ev.at) });
        }
    }
    fn drain(&self) {
        while self.env.rx.try_recv().is_ok() {}
    }
}

pub fn flush_console_input() {
    use windows_sys::Win32::System::Console::{FlushConsoleInputBuffer, GetStdHandle, STD_INPUT_HANDLE};
    unsafe {
        FlushConsoleInputBuffer(GetStdHandle(STD_INPUT_HANDLE));
    }
}

fn level_name(rid: u32) -> &'static str {
    match rid {
        r if r >= 0x4000 => "System",
        r if r >= 0x3000 => "High (administrator)",
        r if r >= 0x2000 => "Medium (normal)",
        r if r >= 0x1000 => "Low",
        _ => "Untrusted",
    }
}

unsafe fn token_integrity(process: HANDLE) -> Result<u32, String> {
    let mut token: HANDLE = std::ptr::null_mut();
    if OpenProcessToken(process, TOKEN_QUERY, &mut token) == 0 {
        return Err(format!("OpenProcessToken failed, error {}", GetLastError()));
    }
    let mut len = 0u32;
    GetTokenInformation(token, TokenIntegrityLevel, std::ptr::null_mut(), 0, &mut len);
    let mut buf = vec![0u64; (len.max(64) as usize).div_ceil(8)];
    let ok = GetTokenInformation(token, TokenIntegrityLevel, buf.as_mut_ptr() as *mut _, (buf.len() * 8) as u32, &mut len);
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

/// One-line description of this tool's and the game's integrity levels (informational: it is
/// not documented whether UIPI affects Raw Input delivery, so S7 records it).
pub fn integrity_note(game_title: &str) -> String {
    unsafe {
        let own = token_integrity(GetCurrentProcess()).map(level_name).unwrap_or("unknown");
        let t = super::wide(game_title);
        let hwnd = FindWindowW(std::ptr::null(), t.as_ptr());
        if hwnd.is_null() {
            return format!("this tool: {own}; game window not found yet");
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, &mut pid);
        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if h.is_null() {
            return format!("this tool: {own}; game: unreadable (probably running as administrator)");
        }
        let g = token_integrity(h).map(level_name).unwrap_or("unknown");
        CloseHandle(h);
        format!("this tool: {own}; game: {g}")
    }
}

pub fn status_json_path(journal_dir: Option<&str>) -> PathBuf {
    if let Some(d) = journal_dir {
        return PathBuf::from(d).join("Status.json");
    }
    let home = std::env::var_os("USERPROFILE").map(PathBuf::from).unwrap_or_default();
    home.join("Saved Games").join("Frontier Developments").join("Elite Dangerous").join("Status.json")
}
