//! Rolling log files (docs/V1-DESIGN.md §7.7): `<app dir>/logs/astromech-core.log`, the same
//! lines as the console. A file is rolled over at 10 MB and rolled-over files older than 7 days
//! are deleted. Paths under the user's profile and the Windows user name are redacted.

use astromech_keybinds::report::Redactor;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};

pub const MAX_BYTES: u64 = 10 * 1024 * 1024;
pub const KEEP: Duration = Duration::from_secs(7 * 24 * 3600);
pub const FILE: &str = "astromech-core.log";

struct Sink {
    dir: PathBuf,
    file: Option<File>,
    written: u64,
    max: u64,
    console: bool,
    red: Redactor,
}

static SINK: OnceLock<Mutex<Option<Sink>>> = OnceLock::new();

fn sink() -> &'static Mutex<Option<Sink>> {
    SINK.get_or_init(|| Mutex::new(None))
}

/// Start logging into `dir` (created if needed). `console`: also print each line.
pub fn init(dir: &Path, console: bool) {
    init_with(dir, console, MAX_BYTES, Redactor::from_env());
}

pub fn init_with(dir: &Path, console: bool, max: u64, red: Redactor) {
    let _ = std::fs::create_dir_all(dir);
    prune(dir, KEEP, SystemTime::now());
    let mut s = Sink { dir: dir.to_path_buf(), file: None, written: 0, max, console, red };
    s.open();
    *sink().lock().unwrap_or_else(|e| e.into_inner()) = Some(s);
}

/// The folder logs go to, once [`init`] ran.
pub fn dir() -> Option<PathBuf> {
    sink().lock().ok()?.as_ref().map(|s| s.dir.clone())
}

impl Sink {
    fn open(&mut self) {
        let p = self.dir.join(FILE);
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
        let mut to = self.dir.join(format!("astromech-core.{stamp}.log"));
        let mut n = 1;
        while to.exists() {
            n += 1;
            to = self.dir.join(format!("astromech-core.{stamp}-{n}.log"));
        }
        let _ = std::fs::rename(self.dir.join(FILE), to);
        self.written = 0;
        self.file = OpenOptions::new().create(true).append(true).open(self.dir.join(FILE)).ok();
        prune(&self.dir, KEEP, SystemTime::now());
    }

    fn write(&mut self, line: &str) {
        let line = self.red.apply(line);
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

/// Delete rolled-over logs (`astromech-core.<stamp>.log`) last modified before `now - keep`.
pub fn prune(dir: &Path, keep: Duration, now: SystemTime) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let n = e.file_name().to_string_lossy().to_string();
        if n == FILE || !n.starts_with("astromech-core.") || !n.ends_with(".log") {
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

    #[test]
    fn rolls_over_and_prunes() {
        let d = tempfile::tempdir().unwrap();
        let mut s = Sink {
            dir: d.path().into(),
            file: None,
            written: 0,
            max: 100,
            console: false,
            red: Redactor::new(vec![], Some("Sumeet".into())),
        };
        s.open();
        for i in 0..10 {
            s.write(&format!("line {i} C:\\Users\\Sumeet\\x ............"));
        }
        let names: Vec<String> =
            std::fs::read_dir(d.path()).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into()).collect();
        assert!(names.iter().filter(|n| n.starts_with("astromech-core.")).count() >= 3, "{names:?}");
        let cur = std::fs::read_to_string(d.path().join(FILE)).unwrap();
        assert!(cur.len() <= 100);
        assert!(!cur.contains("Sumeet"), "{cur}");
        // Everything rolled over is "old" a year from now; the live file stays.
        prune(d.path(), KEEP, SystemTime::now() + Duration::from_secs(365 * 24 * 3600));
        let left: Vec<String> =
            std::fs::read_dir(d.path()).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into()).collect();
        assert_eq!(left, vec![FILE.to_string()]);
    }
}
