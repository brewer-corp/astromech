//! Keeps the journal history (`astromech-history`, `history.sqlite` in the data folder)
//! current while the core runs (ARCHITECTURE.md §18.5).
//!
//! One background thread owns the history database. It imports at startup (the first run is
//! the backfill of every journal) and again whenever the journals change, which reads only
//! what is new. Two things start an import:
//! - a change notification for the journal folder (any file: `Status.json` changes all the time
//!   while the game runs). After one it waits [`SETTLE`] and takes every notification that
//!   arrived meanwhile as one import;
//! - every [`RECHECK`] without a notification, the journals' [`JournalVersion`] (how many, and
//!   the newest one's length read through an open handle) differs from the last import's.
//!   Windows doesn't report appends to a file the game holds open (NTFS updates the directory
//!   entry lazily; astromech-txzq), so notifications alone would stop following live play.
//!
//! The game-state pipeline never waits for it: separate thread, separate database file, and
//! nothing here holds a lock the pipeline takes. Progress is in [`status`] (the control API's
//! `status` reply, so the app can show it).

use astromech_history::{History, ImportOptions};
use notify::{RecursiveMode, Watcher};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

/// The history database's file name in the data folder.
pub const DB_FILE: &str = "history.sqlite";
/// How long to gather folder notifications into one import.
pub const SETTLE: Duration = Duration::from_secs(2);
/// How often to look for journal growth that no notification reported.
pub const RECHECK: Duration = Duration::from_secs(2);

/// When the follower imports. [`Default`] is what the core uses.
#[derive(Debug, Clone, Copy)]
pub struct Timing {
    pub settle: Duration,
    pub recheck: Duration,
    /// Watch the folder for change notifications (off: only the recheck).
    pub notify: bool,
}

impl Default for Timing {
    fn default() -> Self {
        Self { settle: SETTLE, recheck: RECHECK, notify: true }
    }
}

/// How many journals there are and how long the newest is, read through an open handle: on
/// Windows the folder listing can lag appends to the file the game holds open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalVersion {
    files: usize,
    newest: Option<PathBuf>,
    len: u64,
}

pub fn journal_version(dir: &std::path::Path) -> JournalVersion {
    let logs = astromech_journal::journal_logs(dir);
    let newest = logs.last().cloned();
    let len =
        newest.as_deref().and_then(|p| std::fs::File::open(p).and_then(|f| f.metadata()).ok()).map_or(0, |m| m.len());
    JournalVersion { files: logs.len(), newest, len }
}

/// What the history thread is doing, for the app.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Progress {
    /// An import is running.
    pub running: bool,
    /// The first import since startup finished: the history is current.
    pub ready: bool,
    /// Of the running (or last) import: files and lines read, bytes read of `bytes_total`.
    pub files: usize,
    pub lines: u64,
    pub bytes: u64,
    pub bytes_total: u64,
    /// Events in the history.
    pub events: u64,
    /// The last import's error, until one succeeds.
    pub error: Option<String>,
}

type Log = Arc<dyn Fn(&str) + Send + Sync>;

/// The history thread and its folder watcher. Dropping it stops both.
pub struct Follower {
    progress: Arc<Mutex<Progress>>,
    poke: Sender<()>,
    _watcher: Option<Mutex<notify::RecommendedWatcher>>,
}

impl Follower {
    /// Start importing `dir` into `db` now and whenever its journals change. Returns at once.
    pub fn start(dir: PathBuf, db: PathBuf, timing: Timing, log: Log) -> Follower {
        let progress = Arc::new(Mutex::new(Progress::default()));
        let (poke, rx) = channel();
        let tx = poke.clone();
        let watcher = if timing.notify {
            notify::recommended_watcher(move |e: notify::Result<notify::Event>| {
                if e.is_ok() {
                    let _ = tx.send(());
                }
            })
            .and_then(|mut w| w.watch(&dir, RecursiveMode::NonRecursive).map(|()| w))
            .map_err(|e| {
                log(&format!("history: not watching {}: {e}; checking for new lines on a timer", dir.display()))
            })
            .ok()
        } else {
            None
        };
        let p = progress.clone();
        let l = log.clone();
        if let Err(e) =
            std::thread::Builder::new().name("history".into()).spawn(move || run(&dir, &db, timing, &p, &rx, &l))
        {
            log(&format!("history: can't start its thread: {e}"));
        }
        Follower { progress, poke, _watcher: watcher.map(Mutex::new) }
    }

    pub fn progress(&self) -> Progress {
        self.progress.lock().unwrap().clone()
    }

    /// Import again (after the settle time), as if the folder changed.
    pub fn poke(&self) {
        let _ = self.poke.send(());
    }
}

fn run(dir: &std::path::Path, db: &std::path::Path, timing: Timing, p: &Mutex<Progress>, rx: &Receiver<()>, log: &Log) {
    let mut history: Option<History> = None;
    let mut first = true;
    loop {
        // Running from here: opening can migrate and rebuild the ledgers, which takes seconds.
        p.lock().unwrap().running = true;
        if history.is_none() {
            match History::open(db) {
                Ok(h) => history = Some(h),
                Err(e) => {
                    log(&format!("history: can't open {}: {e}", db.display()));
                    let mut g = p.lock().unwrap();
                    g.running = false;
                    g.error = Some(e.to_string());
                }
            }
        }
        // Taken before the import: lines written while it runs make the next recheck import.
        let mut version = journal_version(dir);
        if let Some(h) = history.as_mut() {
            let r = h.import_with(dir, &ImportOptions::default(), |r| {
                let mut g = p.lock().unwrap();
                (g.files, g.lines, g.bytes, g.bytes_total) = (r.files, r.lines, r.bytes, r.bytes_total);
            });
            let events = h.event_count().unwrap_or_default();
            let mut g = p.lock().unwrap();
            g.running = false;
            g.events = events;
            match r {
                Ok(r) => {
                    if first {
                        log(&format!(
                            "history: {} new lines from {} journal files imported; {events} events in {}",
                            r.lines,
                            r.files,
                            db.display()
                        ));
                    }
                    for name in &r.shrunk {
                        log(&format!("history: {name} is shorter than what was imported from it; skipped"));
                    }
                    first = false;
                    g.ready = true;
                    g.error = None;
                }
                Err(e) => {
                    log(&format!("history: import failed: {e}"));
                    g.error = Some(e.to_string());
                }
            }
        }
        // Wait for a notification (then gather what else arrives into one import) or for the
        // journals to grow without one.
        loop {
            match rx.recv_timeout(timing.recheck) {
                Ok(()) => {
                    std::thread::sleep(timing.settle);
                    while rx.try_recv().is_ok() {}
                    break;
                }
                Err(RecvTimeoutError::Timeout) => {
                    let now = journal_version(dir);
                    if now != version {
                        break;
                    }
                    version = now;
                }
                Err(RecvTimeoutError::Disconnected) => return,
            }
        }
    }
}

static CURRENT: OnceLock<Follower> = OnceLock::new();

/// Start the core's one history follower (at core startup).
pub fn start(dir: PathBuf, data_dir: PathBuf, log: Log) {
    let _ = std::fs::create_dir_all(&data_dir);
    let _ = CURRENT.set(Follower::start(dir, data_dir.join(DB_FILE), Timing::default(), log));
}

/// The core's history progress, once [`start`] ran.
pub fn status() -> Option<Progress> {
    CURRENT.get().map(Follower::progress)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hub::StateHub;
    use std::io::Write;
    use std::path::Path;
    use std::time::Instant;

    fn fixtures() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../astromech-history/tests/fixtures")
    }

    fn journal_dir() -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        for p in astromech_journal::journal_logs(&fixtures()) {
            std::fs::copy(&p, d.path().join(p.file_name().unwrap())).unwrap();
        }
        d
    }

    fn wait_for(what: &str, mut ok: impl FnMut() -> bool) {
        let end = Instant::now() + Duration::from_secs(20);
        while !ok() {
            assert!(Instant::now() < end, "timed out waiting for {what}");
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn quiet() -> Log {
        Arc::new(|_| {})
    }

    fn fast(notify: bool) -> Timing {
        Timing { settle: Duration::from_millis(50), recheck: Duration::from_millis(50), notify }
    }

    /// The Windows case (astromech-txzq): the game appends to the journal it holds open and no
    /// change notification arrives. With notifications off, the recheck still imports the line.
    #[test]
    fn follows_appends_without_notifications() {
        let j = journal_dir();
        let data = tempfile::tempdir().unwrap();
        let db = data.path().join(DB_FILE);
        let f = Follower::start(j.path().to_path_buf(), db.clone(), fast(false), quiet());
        wait_for("the startup import", || f.progress().ready);
        let newest = astromech_journal::newest_journal(j.path()).unwrap();
        let mut w = std::fs::OpenOptions::new().append(true).open(newest).unwrap();
        w.write_all(
            b"{ \"timestamp\":\"2026-09-03T10:40:00Z\", \"event\":\"FSDJump\", \"StarSystem\":\"Ross 128\", \"SystemAddress\":3, \"JumpDist\":9.1 }\r\n",
        )
        .unwrap();
        w.flush().unwrap();
        let reader = History::open(&db).unwrap();
        wait_for("the appended jump, with the writer still open", || {
            reader.visits("F0000001").unwrap().iter().any(|v| v.star_system == "Ross 128")
        });
        // A new journal file is picked up the same way.
        let next = j.path().join("Journal.2026-09-04T100000.01.log");
        std::fs::write(&next, "{ \"timestamp\":\"2026-09-04T10:00:00Z\", \"event\":\"Fileheader\", \"part\":1, \"gameversion\":\"4.1.0.100\" }\r\n").unwrap();
        wait_for("the new file", || f.progress().events == 54);
        drop(w);
    }

    #[test]
    fn imports_at_startup_then_follows_new_lines() {
        let j = journal_dir();
        let data = tempfile::tempdir().unwrap();
        let db = data.path().join(DB_FILE);
        let f = Follower::start(j.path().to_path_buf(), db.clone(), fast(true), quiet());
        wait_for("the startup import", || f.progress().ready);
        let p = f.progress();
        assert_eq!((p.events, p.error.clone(), p.running), (52, None, false));
        assert_eq!(p.bytes, p.bytes_total);

        // The game writes a jump into the newest journal.
        let newest = astromech_journal::newest_journal(j.path()).unwrap();
        let mut w = std::fs::OpenOptions::new().append(true).open(newest).unwrap();
        w.write_all(
            b"{ \"timestamp\":\"2026-09-03T10:40:00Z\", \"event\":\"FSDJump\", \"StarSystem\":\"Ross 128\", \"SystemAddress\":3, \"JumpDist\":9.1 }\r\n",
        )
        .unwrap();
        w.flush().unwrap();
        let reader = History::open(&db).unwrap();
        wait_for("the new jump in the visits ledger", || {
            reader.visits("F0000001").unwrap().iter().any(|v| v.star_system == "Ross 128")
        });
        wait_for("the progress to count it", || f.progress().events == 53);
    }

    /// A backfill that can't make progress (another connection holds the history database)
    /// doesn't hold up starting the follower, nor the game-state pipeline on the same folder.
    #[test]
    fn a_stuck_backfill_never_blocks_the_pipeline() {
        let j = journal_dir();
        let data = tempfile::tempdir().unwrap();
        let db = data.path().join(DB_FILE);
        drop(History::open(&db).unwrap());
        let lock = rusqlite_lock(&db);

        let t = Instant::now();
        let f = Follower::start(j.path().to_path_buf(), db.clone(), fast(true), quiet());
        assert!(t.elapsed() < Duration::from_millis(500), "Follower::start took {:?}", t.elapsed());
        wait_for("the import to start (and wait on the lock)", || f.progress().running);
        let t = Instant::now();
        let hub = Arc::new(StateHub::new());
        let _pipe = crate::pipeline::start(j.path(), hub.clone(), None, |_| {}, None).unwrap();
        assert!(t.elapsed() < Duration::from_secs(1), "pipeline::start took {:?}", t.elapsed());
        let g = hub.snapshot();
        assert_eq!(g.commander.as_deref(), Some("Fixture"));
        assert_eq!(g.system.as_deref(), Some("Alpha Centauri"));
        let p = f.progress();
        assert!(p.running && !p.ready, "the history import is still stuck: {p:?}");

        drop(lock);
        f.poke();
        wait_for("the import once the database is free", || f.progress().ready);
        assert_eq!(f.progress().events, 52);
    }

    /// An open write transaction on `db`: other writers wait, then fail with "busy".
    fn rusqlite_lock(db: &Path) -> astromech_store::rusqlite::Connection {
        let c = astromech_store::rusqlite::Connection::open(db).unwrap();
        c.execute_batch("BEGIN EXCLUSIVE").unwrap();
        c
    }
}
