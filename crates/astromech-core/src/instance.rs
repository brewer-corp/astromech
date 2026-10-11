//! Single instance (docs/V1-DESIGN.md §7.7): the named mutex `Local\astromech-core`. A second
//! start asks the first one (over the control API) to open the window, then exits.

pub const MUTEX_NAME: &str = r"Local\astromech-core";

/// Held for the life of the process.
pub struct Guard {
    #[cfg(windows)]
    handle: windows_sys::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
impl Drop for Guard {
    fn drop(&mut self) {
        unsafe { windows_sys::Win32::Foundation::CloseHandle(self.handle) };
    }
}

/// `Some(guard)` if this is the only core; `None` if another one already runs.
#[cfg(windows)]
pub fn acquire(_app_dir: &std::path::Path) -> Option<Guard> {
    use windows_sys::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows_sys::Win32::System::Threading::CreateMutexW;
    let name: Vec<u16> = MUTEX_NAME.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let h = CreateMutexW(std::ptr::null(), 0, name.as_ptr());
        if h.is_null() {
            return Some(Guard { handle: h });
        }
        if GetLastError() == ERROR_ALREADY_EXISTS {
            windows_sys::Win32::Foundation::CloseHandle(h);
            return None;
        }
        Some(Guard { handle: h })
    }
}

/// Elsewhere (development only): a live control endpoint means another core runs.
#[cfg(not(windows))]
pub fn acquire(app_dir: &std::path::Path) -> Option<Guard> {
    match crate::control::call(app_dir, "status", None) {
        Ok(v) if v["ok"] == true => None,
        _ => Some(Guard {}),
    }
}
