//! Start with Windows (docs/V1-DESIGN.md §7.5, §9): the HKCU `Run` value `astromech` →
//! `"<dir>\astromech-core.exe" --background`. No admin. On by default: the first time the core
//! runs from an install it turns autostart on and records that in `core-state.json`, so turning
//! it off later (tray menu now, setup and Settings later) sticks.

use serde::{Deserialize, Serialize};
use std::path::Path;

pub const VALUE_NAME: &str = "astromech";
pub const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

/// The command line written to the Run key.
pub fn command_line(exe: &Path) -> String {
    format!("\"{}\" --background", exe.display())
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CoreState {
    /// The default (on) has been applied once; from then on only the user changes it.
    pub autostart_default_applied: bool,
}

impl CoreState {
    pub fn load(app_dir: &Path) -> Self {
        std::fs::read_to_string(app_dir.join("core-state.json"))
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_default()
    }
    pub fn save(&self, app_dir: &Path) {
        let _ = std::fs::create_dir_all(app_dir);
        let _ = std::fs::write(app_dir.join("core-state.json"), serde_json::to_string_pretty(self).unwrap_or_default());
    }
}

/// Apply the on-by-default rule once. Returns what it did, for the log.
pub fn apply_default(app_dir: &Path, exe: &Path) -> Option<String> {
    let mut st = CoreState::load(app_dir);
    if st.autostart_default_applied {
        // Keep the path current if the user moved or reinstalled the program.
        if is_enabled() && get().as_deref() != Some(command_line(exe).as_str()) {
            return Some(match set(true, exe) {
                Ok(()) => "autostart: updated the program path".into(),
                Err(e) => format!("autostart: couldn't update the program path: {e}"),
            });
        }
        return None;
    }
    let r = set(true, exe);
    st.autostart_default_applied = true;
    st.save(app_dir);
    Some(match r {
        Ok(()) => "autostart: on (default); turn it off with \"Start with Windows\" in the tray menu".into(),
        Err(e) => format!("autostart: couldn't turn it on: {e}"),
    })
}

pub fn is_enabled() -> bool {
    get().is_some()
}

#[cfg(windows)]
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// The current Run value, if any.
#[cfg(windows)]
pub fn get() -> Option<String> {
    use windows_sys::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_SZ};
    let (key, name) = (wide(RUN_KEY), wide(VALUE_NAME));
    let mut buf = vec![0u16; 2048];
    let mut len = (buf.len() * 2) as u32;
    let r = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            key.as_ptr(),
            name.as_ptr(),
            RRF_RT_REG_SZ,
            std::ptr::null_mut(),
            buf.as_mut_ptr().cast(),
            &mut len,
        )
    };
    if r != 0 {
        return None;
    }
    let n = (len as usize / 2).saturating_sub(1);
    Some(String::from_utf16_lossy(&buf[..n.min(buf.len())]))
}

#[cfg(windows)]
pub fn set(on: bool, exe: &Path) -> Result<(), String> {
    use windows_sys::Win32::System::Registry::{RegDeleteKeyValueW, RegSetKeyValueW, HKEY_CURRENT_USER, REG_SZ};
    let (key, name) = (wide(RUN_KEY), wide(VALUE_NAME));
    let r = unsafe {
        if on {
            let v = wide(&command_line(exe));
            RegSetKeyValueW(
                HKEY_CURRENT_USER,
                key.as_ptr(),
                name.as_ptr(),
                REG_SZ,
                v.as_ptr().cast(),
                (v.len() * 2) as u32,
            )
        } else {
            let r = RegDeleteKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr());
            if r == windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND {
                0
            } else {
                r
            }
        }
    };
    if r == 0 {
        Ok(())
    } else {
        Err(format!("registry error {r}"))
    }
}

#[cfg(not(windows))]
pub fn get() -> Option<String> {
    None
}

#[cfg(not(windows))]
pub fn set(_on: bool, _exe: &Path) -> Result<(), String> {
    Err("start with Windows is only available on Windows".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_quotes_the_path() {
        let p = Path::new(r"C:\Users\x\AppData\Local\Programs\astromech\astromech-core.exe");
        assert_eq!(command_line(p), r#""C:\Users\x\AppData\Local\Programs\astromech\astromech-core.exe" --background"#);
    }

    #[test]
    fn state_round_trip() {
        let d = tempfile::tempdir().unwrap();
        assert!(!CoreState::load(d.path()).autostart_default_applied);
        CoreState { autostart_default_applied: true }.save(d.path());
        assert!(CoreState::load(d.path()).autostart_default_applied);
    }
}
