//! Crash handling (docs/V1-DESIGN.md §7.7): a panic hook writes the panic and a backtrace to
//! the log and to a crash marker in the app folder. On the next start the tray offers
//! "astromech crashed last time · Save diagnostics"; saving or dismissing moves the marker to
//! `crash-last.txt` (kept for the next diagnostics bundle).

use astromech_proto::Clock;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub const MARKER: &str = "crash.marker";
pub const LAST: &str = "crash-last.txt";

/// Install the panic hook. Panics on any thread are logged; the process then carries on as
/// Rust normally would, except that a panic on the main thread aborts once it is logged and its
/// marker written. The main thread runs Tauri's event loop, which would catch the panic and end
/// the process with an ordinary exit code (101), out of reach of Windows Error Reporting; an
/// abort is a crash to Windows, so it gets a WER report. astromech never restarts itself after a
/// crash (decision astromech-lkwj): the next start offers Save diagnostics and the cause is fixed. A panic in a command's handler,
/// which the command layer catches and answers with an error ([`astromech_api::panic_is_caught`]),
/// is logged but leaves no crash marker: the core kept running. A second panic while that one
/// unwinds aborts the process, so it leaves one. `clock` stamps the marker.
pub fn install(app_dir: &Path, clock: Arc<dyn Clock>) {
    let marker = app_dir.join(MARKER);
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current().name().unwrap_or("unnamed").to_string();
        if astromech_api::panic_is_caught() {
            crate::logging::log(&format!("panic in a command on thread {thread} (answered with an error): {info}"));
            prev(info);
            return;
        }
        let bt = std::backtrace::Backtrace::force_capture();
        let text = format!(
            "astromech-core {} crashed at {}\nthread: {thread}\n{info}\n\nbacktrace:\n{bt}\n",
            env!("CARGO_PKG_VERSION"),
            chrono::DateTime::<chrono::Local>::from(clock.utc()).format("%Y-%m-%d %H:%M:%S %:z"),
        );
        crate::logging::log(&format!("CRASH (panic) on thread {thread}: {info}"));
        let red = astromech_keybinds::report::Redactor::from_env();
        let _ = std::fs::write(&marker, red.apply(&text));
        prev(info);
        if thread == "main" {
            std::process::abort();
        }
    }));
}

/// The crash marker left by the previous run, if any.
pub fn pending(app_dir: &Path) -> Option<String> {
    std::fs::read_to_string(app_dir.join(MARKER)).ok()
}

/// Clear the marker (keeping its text as `crash-last.txt`).
pub fn acknowledge(app_dir: &Path) -> Option<PathBuf> {
    let last = app_dir.join(LAST);
    std::fs::rename(app_dir.join(MARKER), &last).ok().map(|_| last)
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;

    #[test]
    fn marker_round_trip() {
        let d = tempfile::tempdir().unwrap();
        assert!(pending(d.path()).is_none());
        std::fs::write(d.path().join(MARKER), "boom").unwrap();
        assert_eq!(pending(d.path()).as_deref(), Some("boom"));
        assert!(acknowledge(d.path()).is_some());
        assert!(pending(d.path()).is_none());
        assert_eq!(std::fs::read_to_string(d.path().join(LAST)).unwrap(), "boom");
    }
}
