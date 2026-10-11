//! Game detection (docs/V1-DESIGN.md §7.6): poll the process list every 3 s for
//! `EliteDangerous64.exe` (the game) and `EDLaunch.exe` (the launcher, for pre-warming). The
//! core pre-warms (goes active) when the launcher opens, becomes active when the game runs and goes idle again once it has been gone for two
//! polls in a row (so a brief miss doesn't unload the models). A new or growing `Journal.*.log`
//! is logged as the confirming second signal.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub const GAME_EXE: &str = "EliteDangerous64.exe";
pub const LAUNCHER_EXE: &str = "EDLaunch.exe";
pub const POLL: Duration = Duration::from_secs(3);
/// Polls in a row without the game before going idle.
pub const MISSES_TO_IDLE: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Game {
    NotRunning,
    Launcher,
    Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    LauncherOpened,
    GameStarted,
    GameExited,
    /// The launcher closed without the game having started.
    LauncherClosed,
}

/// The pure state machine: feed it each poll's process names.
#[derive(Debug)]
pub struct Watcher {
    pub game: Game,
    misses: u32,
}

impl Default for Watcher {
    fn default() -> Self {
        Self { game: Game::NotRunning, misses: 0 }
    }
}

fn has(names: &[String], exe: &str) -> bool {
    names.iter().any(|n| n.eq_ignore_ascii_case(exe))
}

impl Watcher {
    pub fn poll(&mut self, names: &[String]) -> Option<Change> {
        let game = has(names, GAME_EXE);
        let launcher = has(names, LAUNCHER_EXE);
        match (self.game, game) {
            (Game::Running, true) => {
                self.misses = 0;
                None
            }
            (Game::Running, false) => {
                self.misses += 1;
                if self.misses < MISSES_TO_IDLE {
                    return None;
                }
                self.misses = 0;
                self.game = if launcher { Game::Launcher } else { Game::NotRunning };
                Some(Change::GameExited)
            }
            (_, true) => {
                self.game = Game::Running;
                self.misses = 0;
                Some(Change::GameStarted)
            }
            (Game::NotRunning, false) if launcher => {
                self.game = Game::Launcher;
                Some(Change::LauncherOpened)
            }
            (Game::Launcher, false) if !launcher => {
                self.game = Game::NotRunning;
                Some(Change::LauncherClosed)
            }
            _ => None,
        }
    }
}

/// Names of running processes (exe file names).
pub fn process_names() -> Vec<String> {
    imp::process_names()
}

#[cfg(windows)]
mod imp {
    use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
    };

    pub fn process_names() -> Vec<String> {
        let mut out = vec![];
        unsafe {
            let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snap == INVALID_HANDLE_VALUE {
                return out;
            }
            let mut e: PROCESSENTRY32W = std::mem::zeroed();
            e.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
            let mut ok = Process32FirstW(snap, &mut e) != 0;
            while ok {
                let len = e.szExeFile.iter().position(|&c| c == 0).unwrap_or(e.szExeFile.len());
                out.push(String::from_utf16_lossy(&e.szExeFile[..len]));
                ok = Process32NextW(snap, &mut e) != 0;
            }
            CloseHandle(snap);
        }
        out
    }
}

#[cfg(not(windows))]
mod imp {
    /// Linux (development only): `/proc/<pid>/comm`, which Wine sets to the exe name.
    pub fn process_names() -> Vec<String> {
        let Ok(rd) = std::fs::read_dir("/proc") else { return vec![] };
        rd.flatten()
            .filter(|e| e.file_name().to_string_lossy().chars().all(|c| c.is_ascii_digit()))
            .filter_map(|e| std::fs::read_to_string(e.path().join("comm")).ok())
            .map(|s| s.trim().to_string())
            .collect()
    }
}

/// The newest `Journal.*.log` in `dir` and when it last changed.
pub fn newest_journal(dir: &Path) -> Option<(PathBuf, SystemTime)> {
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.starts_with("Journal.") && n.ends_with(".log")
        })
        .filter_map(|e| Some((e.path(), e.metadata().ok()?.modified().ok()?)))
        .max_by_key(|(_, t)| *t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(n: &[&str]) -> Vec<String> {
        n.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn launcher_game_exit() {
        let mut w = Watcher::default();
        assert_eq!(w.poll(&v(&["explorer.exe"])), None);
        assert_eq!(w.poll(&v(&["EDLaunch.exe"])), Some(Change::LauncherOpened));
        assert_eq!(w.poll(&v(&["EDLaunch.exe"])), None);
        assert_eq!(w.poll(&v(&["EDLaunch.exe", "elitedangerous64.exe"])), Some(Change::GameStarted));
        assert_eq!(w.game, Game::Running);
        // One missed poll is tolerated.
        assert_eq!(w.poll(&v(&["EDLaunch.exe"])), None);
        assert_eq!(w.poll(&v(&["EDLaunch.exe", "EliteDangerous64.exe"])), None);
        assert_eq!(w.poll(&v(&[])), None);
        assert_eq!(w.poll(&v(&[])), Some(Change::GameExited));
        assert_eq!(w.game, Game::NotRunning);
    }

    #[test]
    fn game_without_launcher() {
        let mut w = Watcher::default();
        assert_eq!(w.poll(&v(&["EliteDangerous64.exe"])), Some(Change::GameStarted));
        assert_eq!(w.poll(&v(&["EDLaunch.exe"])), None);
        assert_eq!(w.poll(&v(&["EDLaunch.exe"])), Some(Change::GameExited));
        assert_eq!(w.game, Game::Launcher);
        assert_eq!(w.poll(&v(&[])), Some(Change::LauncherClosed));
        assert_eq!(w.game, Game::NotRunning);
    }

    #[test]
    fn lists_processes() {
        // At least this test process.
        assert!(!process_names().is_empty());
    }

    #[test]
    fn finds_newest_journal() {
        let d = tempfile::tempdir().unwrap();
        assert!(newest_journal(d.path()).is_none());
        std::fs::write(d.path().join("Journal.2026-10-10T120000.01.log"), "a").unwrap();
        std::fs::write(d.path().join("Status.json"), "{}").unwrap();
        assert!(newest_journal(d.path()).unwrap().0.ends_with("Journal.2026-10-10T120000.01.log"));
    }
}
