//! Crash handling (docs/V1-DESIGN.md §7.7): a panic hook writes the panic and a backtrace to
//! the log and to a crash marker in the app folder. On the next start the tray offers
//! "astromech crashed last time · Save diagnostics"; saving or dismissing moves the marker to
//! `crash-last.txt` (kept for the next diagnostics bundle).

use std::path::{Path, PathBuf};

pub const MARKER: &str = "crash.marker";
pub const LAST: &str = "crash-last.txt";

/// Install the panic hook. Panics on any thread are logged; the process then carries on as
/// Rust normally would (a panic on the main thread ends it).
pub fn install(app_dir: &Path) {
    let marker = app_dir.join(MARKER);
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current().name().unwrap_or("unnamed").to_string();
        let bt = std::backtrace::Backtrace::force_capture();
        let text = format!(
            "astromech-core {} crashed at {}\nthread: {thread}\n{info}\n\nbacktrace:\n{bt}\n",
            env!("CARGO_PKG_VERSION"),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
        );
        crate::logging::log(&format!("CRASH (panic) on thread {thread}: {info}"));
        let red = astromech_keybinds::report::Redactor::from_env();
        let _ = std::fs::write(&marker, red.apply(&text));
        prev(info);
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
