//! # astromech-log
//!
//! Rolling log files (docs/V1-DESIGN.md §7.7), one per process in the same folder:
//! `astromech-core.log` for the core and `astromech-shell.log` for the app window. A file is
//! rolled over at 10 MB (`<name>.<stamp>.log`) and rolled-over files older than 7 days are
//! deleted. Each line goes through the redaction the process gives [`init`] (the user's profile
//! paths and Windows user name).

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};

pub const MAX_BYTES: u64 = 10 * 1024 * 1024;
pub const KEEP: Duration = Duration::from_secs(7 * 24 * 3600);
/// The core's log name (`astromech-core.log`).
pub const CORE: &str = "astromech-core";
/// The app window's log name (`astromech-shell.log`).
pub const SHELL: &str = "astromech-shell";

/// Rewrites a line before it is written, e.g. to hide the user's name.
pub type Redact = Box<dyn Fn(&str) -> String + Send>;

struct Sink {
    dir: PathBuf,
    name: &'static str,
    file: Option<File>,
    written: u64,
    max: u64,
    console: bool,
    redact: Redact,
}

static SINK: OnceLock<Mutex<Option<Sink>>> = OnceLock::new();

fn sink() -> &'static Mutex<Option<Sink>> {
    SINK.get_or_init(|| Mutex::new(None))
}

/// The live log file `name` writes to in `dir`.
pub fn file(dir: &Path, name: &str) -> PathBuf {
    dir.join(format!("{name}.log"))
}

/// Start logging this process's lines into `dir/<name>.log` (`dir` created if needed).
/// `console`: also print each line.
pub fn init(dir: &Path, name: &'static str, console: bool, redact: Redact) {
    init_with(dir, name, console, MAX_BYTES, redact);
}

pub fn init_with(dir: &Path, name: &'static str, console: bool, max: u64, redact: Redact) {
    let _ = std::fs::create_dir_all(dir);
    prune(dir, name, KEEP, SystemTime::now());
    let mut s = Sink { dir: dir.to_path_buf(), name, file: None, written: 0, max, console, redact };
    s.open();
    *sink().lock().unwrap_or_else(|e| e.into_inner()) = Some(s);
}

/// The folder logs go to, once [`init`] ran.
pub fn dir() -> Option<PathBuf> {
    sink().lock().ok()?.as_ref().map(|s| s.dir.clone())
}

impl Sink {
    fn open(&mut self) {
        let p = file(&self.dir, self.name);
        self.written = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
        if self.written >= self.max {
            self.roll();
            return;
        }
        self.file = OpenOptions::new().create(true).append(true).open(&p).ok();
    }

    fn roll(&mut self) {
        self.file = None;
        let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S%.3f");
        let mut to = self.dir.join(format!("{}.{stamp}.log", self.name));
        let mut n = 1;
        while to.exists() {
            n += 1;
            to = self.dir.join(format!("{}.{stamp}-{n}.log", self.name));
        }
        let live = file(&self.dir, self.name);
        let _ = std::fs::rename(&live, to);
        self.written = 0;
        self.file = OpenOptions::new().create(true).append(true).open(&live).ok();
        prune(&self.dir, self.name, KEEP, SystemTime::now());
    }

    fn write(&mut self, line: &str) {
        let line = (self.redact)(line);
        if self.console {
            println!("{line}");
        }
        if self.written + line.len() as u64 + 1 > self.max {
            self.roll();
        }
        if let Some(f) = self.file.as_mut() {
            if writeln!(f, "{line}").is_ok() {
                self.written += line.len() as u64 + 1;
            }
        }
    }
}

/// Delete `name`'s rolled-over logs (`<name>.<stamp>.log`) last modified before `now - keep`.
pub fn prune(dir: &Path, name: &str, keep: Duration, now: SystemTime) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let (live, prefix) = (format!("{name}.log"), format!("{name}."));
    for e in rd.flatten() {
        let n = e.file_name().to_string_lossy().to_string();
        if n == live || !n.starts_with(&prefix) || !n.ends_with(".log") {
            continue;
        }
        let old = e.metadata().and_then(|m| m.modified()).map(|t| now.duration_since(t).unwrap_or_default() > keep);
        if old.unwrap_or(false) {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

/// Log one line with a timestamp. Before [`init`], prints only.
pub fn log(s: &str) {
    let line = format!("{}  {s}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    let mut g = sink().lock().unwrap_or_else(|e| e.into_inner());
    match g.as_mut() {
        Some(k) => k.write(&line),
        None => println!("{line}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> =
            std::fs::read_dir(dir).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into()).collect();
        v.sort();
        v
    }

    #[test]
    fn rolls_over_and_prunes() {
        let d = tempfile::tempdir().unwrap();
        let mut s = Sink {
            dir: d.path().into(),
            name: CORE,
            file: None,
            written: 0,
            max: 100,
            console: false,
            redact: Box::new(|l: &str| l.replace("Sumeet", "<user>")),
        };
        s.open();
        for i in 0..10 {
            s.write(&format!("line {i} C:\\Users\\Sumeet\\x ............"));
        }
        let all = names(d.path());
        assert!(all.iter().filter(|n| n.starts_with("astromech-core.")).count() >= 3, "{all:?}");
        let cur = std::fs::read_to_string(file(d.path(), CORE)).unwrap();
        assert!(cur.len() <= 100);
        assert!(!cur.contains("Sumeet"), "{cur}");
        // The window's log in the same folder is not the core's to prune.
        std::fs::write(d.path().join("astromech-shell.20260101-000000.000.log"), "x").unwrap();
        // Everything rolled over is "old" a year from now; the live file stays.
        prune(d.path(), CORE, KEEP, SystemTime::now() + Duration::from_secs(365 * 24 * 3600));
        assert_eq!(names(d.path()), ["astromech-core.log", "astromech-shell.20260101-000000.000.log"]);
    }

    /// The app window's lines (e.g. a remote-data cache that won't open) land in its own file
    /// next to the core's, redacted.
    #[test]
    fn the_shells_lines_land_in_its_log_file() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(file(d.path(), CORE), "core line\n").unwrap();
        init(d.path(), SHELL, false, Box::new(|l: &str| l.replace("Sumeet", "<user>")));
        assert_eq!(dir().as_deref(), Some(d.path()));
        log("remote data cache not opened, answers won't be shared: C:\\Users\\Sumeet\\x locked");
        let shell = std::fs::read_to_string(d.path().join("astromech-shell.log")).unwrap();
        assert!(
            shell.ends_with("  remote data cache not opened, answers won't be shared: C:\\Users\\<user>\\x locked\n"),
            "{shell}"
        );
        assert_eq!(std::fs::read_to_string(file(d.path(), CORE)).unwrap(), "core line\n");
    }
}
