//! Journal + Status.json pipeline. File notifications only (as in S5): no polling loop.
//!
//! - Status.json: every change notification re-reads the file (with retries for half-written
//!   files) and applies the flags.
//! - Journal: the newest `Journal.*.log` is tailed from a byte offset; only complete lines are
//!   parsed. When the game starts a new journal file, the old one is drained and the new one
//!   becomes current. On startup the current file is replayed from the start (to rebuild
//!   state) and the saved position from the store tells us how much of it is new.
//! - Because Windows may not report appends to a file the game keeps open until it flushes
//!   metadata, every Status.json notification (several per second in game) also reads the
//!   journal tail. ⚠ Unverified on the real game how often that is needed.
//! - Companion files (`Market.json`, ...): read after the journal event that announces them,
//!   once (no waiting: that would stall the journal). A file that is missing or older than its
//!   event is logged and not used; it is tried again when the game next writes that file.
//! - EDDN (when switched on): every journal line goes to the [`astromech_eddn::Feed`] in order,
//!   and the companion files to it after their event. Lines replayed at startup are fed as not
//!   live, so they rebuild its header, commander and location but nothing from them is sent; a
//!   journal the game starts while we run is live from its `Fileheader`. Delivery runs on the
//!   feed's own thread, so the network never holds up this one.

use crate::hub::StateHub;
use anyhow::{Context, Result};
use astromech_eddn::worker::wall_now;
use astromech_journal::companion::{load, triggers, CompanionFile, CompanionKind, Trigger};
use notify::{RecursiveMode, Watcher};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::Arc;
use std::time::Duration;

/// Tail one journal file from a byte offset.
pub struct JournalTail {
    pub path: PathBuf,
    pub offset: u64,
    partial: Vec<u8>,
}

impl JournalTail {
    pub fn new(path: PathBuf, offset: u64) -> Self {
        Self { path, offset, partial: vec![] }
    }

    /// Complete new lines since the last call.
    pub fn read_new(&mut self) -> Vec<String> {
        let Ok(mut f) = File::open(&self.path) else { return vec![] };
        let len = f.metadata().map(|m| m.len()).unwrap_or(0);
        if len < self.offset {
            // Truncated or replaced: start over.
            self.offset = 0;
            self.partial.clear();
        }
        if f.seek(SeekFrom::Start(self.offset)).is_err() {
            return vec![];
        }
        let mut buf = vec![];
        if f.read_to_end(&mut buf).is_err() {
            return vec![];
        }
        self.offset += buf.len() as u64;
        self.partial.extend_from_slice(&buf);
        let mut out = vec![];
        while let Some(p) = self.partial.iter().position(|b| *b == b'\n') {
            let line: Vec<u8> = self.partial.drain(..=p).collect();
            let s = String::from_utf8_lossy(&line).trim().to_string();
            if !s.is_empty() {
                out.push(s);
            }
        }
        out
    }

    /// Offset of the last complete line (what we persist).
    pub fn committed(&self) -> u64 {
        self.offset - self.partial.len() as u64
    }
}

fn is_journal(p: &Path) -> bool {
    p.file_name()
        .map(|n| {
            let n = n.to_string_lossy();
            n.starts_with("Journal.") && n.ends_with(".log")
        })
        .unwrap_or(false)
}

/// The journal event and text of a companion file EDDN has a schema for.
fn eddn_file(f: &CompanionFile) -> Option<(&'static str, &str)> {
    Some(match f {
        CompanionFile::Market(m) => ("Market", &m.raw_json),
        CompanionFile::Outfitting(o) => ("Outfitting", &o.raw_json),
        CompanionFile::Shipyard(y) => ("Shipyard", &y.raw_json),
        CompanionFile::FcMaterials(c) => ("FCMaterials", &c.raw_json),
        _ => return None,
    })
}

/// Counters for logs and tests.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Stats {
    pub status_reads: u64,
    pub journal_lines: u64,
    pub bad_lines: u64,
    pub rollovers: u64,
    /// Companion files read and kept / reported as missing, half-written or older than their event.
    pub companions_read: u64,
    pub companions_unused: u64,
}

pub struct Pipeline {
    _watcher: notify::RecommendedWatcher,
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    pub stats: Arc<std::sync::Mutex<Stats>>,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

struct Tailer {
    dir: PathBuf,
    hub: Arc<StateHub>,
    store: Option<astromech_store::Store>,
    tail: Option<JournalTail>,
    stats: Arc<std::sync::Mutex<Stats>>,
    log: Box<dyn Fn(&str) + Send>,
    /// Companion events whose file was not usable yet; tried again when the game touches the file.
    pending: Vec<Trigger>,
    eddn: Option<astromech_eddn::Feed>,
    /// False while replaying a journal written before we started.
    live: bool,
}

impl Tailer {
    /// Apply journal lines to the state, then read the companion files their events announced.
    fn apply_lines(&mut self, lines: Vec<String>) {
        let mut parsed = Vec::with_capacity(lines.len());
        {
            let mut st = self.stats.lock().unwrap();
            for l in lines {
                match astromech_journal::parse_line(&l) {
                    Ok(j) => {
                        st.journal_lines += 1;
                        self.hub.apply_journal(&j.event);
                        if let (Some(f), Ok(raw)) = (&mut self.eddn, serde_json::from_str(&j.raw_json)) {
                            f.journal(&raw, self.live, wall_now());
                        }
                        parsed.push(j);
                    }
                    Err(_) => st.bad_lines += 1,
                }
            }
        }
        // Events first, files after: a slow file never delays the journal state.
        let found = triggers(&parsed);
        if !found.is_empty() {
            self.pending.retain(|p| !found.iter().any(|t| t.kind == p.kind));
            self.load_companions(found);
        }
        self.eddn_nav_route();
    }

    /// `NavRoute.json` for a live `NavRoute` event: read after the event and on every change
    /// until the feed has the file the event announced.
    fn eddn_nav_route(&mut self) {
        let Some(f) = self.eddn.as_mut().filter(|f| f.awaiting("NavRoute")) else { return };
        let file = std::fs::read(self.dir.join("NavRoute.json")).ok().and_then(|b| serde_json::from_slice(&b).ok());
        if let Some(file) = file {
            f.companion_file("NavRoute", &file, wall_now());
        }
    }

    fn load_companions(&mut self, found: Vec<Trigger>) {
        let (files, reports) = load(&self.dir, &found);
        {
            let mut st = self.stats.lock().unwrap();
            st.companions_read += files.len() as u64;
            st.companions_unused += reports.len() as u64;
        }
        for f in files {
            if let (Some(feed), Some((event, raw))) = (&mut self.eddn, eddn_file(&f)) {
                if let Ok(file) = serde_json::from_str(raw) {
                    feed.companion_file(event, &file, wall_now());
                }
            }
            self.hub.update(|g| g.apply_companion(f));
        }
        for r in reports {
            (self.log)(&format!("companion file {r}"));
            self.pending.push(Trigger { kind: r.kind, event_ts: r.event_ts });
        }
    }

    /// The game wrote one of these files: try the events that were waiting for it.
    fn on_companion_files(&mut self, kinds: &[CompanionKind]) {
        let waiting: Vec<Trigger> = self.pending.iter().filter(|p| kinds.contains(&p.kind)).cloned().collect();
        if waiting.is_empty() {
            return;
        }
        self.pending.retain(|p| !kinds.contains(&p.kind));
        self.load_companions(waiting);
    }

    fn save(&self) {
        if let (Some(s), Some(t)) = (&self.store, &self.tail) {
            let size = std::fs::metadata(&t.path).map(|m| m.len()).unwrap_or(0);
            let _ = s.set_journal_offset(&t.path.display().to_string(), size, t.committed());
        }
    }

    fn open(&mut self, path: PathBuf, replay: bool) {
        let saved = self.store.as_ref().and_then(|s| s.journal_offset(&path.display().to_string()).ok().flatten());
        // Every journal is read from its first line (the Fileheader). At startup that's a replay
        // of what happened before we started; a file the game starts later is live throughout.
        let mut t = JournalTail::new(path.clone(), 0);
        let lines = t.read_new();
        let n = lines.len();
        self.live = !replay;
        self.apply_lines(lines);
        self.live = true;
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        self.hub.update(|g| g.journal_file = Some(name.clone()));
        match (replay, saved) {
            (true, Some((_, off))) => (self.log)(&format!(
                "journal {name}: replayed {n} lines to rebuild state ({} bytes new since the last run)",
                t.committed().saturating_sub(off)
            )),
            _ => (self.log)(&format!("journal {name}: following ({n} lines so far)")),
        }
        self.tail = Some(t);
        self.save();
    }

    fn on_journal_activity(&mut self) {
        let newest = astromech_journal::newest_journal(&self.dir);
        if let Some(t) = &mut self.tail {
            let lines = t.read_new();
            self.apply_lines(lines);
        }
        if let Some(n) = newest {
            if self.tail.as_ref().map(|t| t.path != n).unwrap_or(true) {
                if self.tail.is_some() {
                    self.stats.lock().unwrap().rollovers += 1;
                }
                self.flush_eddn();
                self.open(n, false);
            }
        }
        self.save();
    }

    fn on_status(&mut self) {
        let p = self.dir.join("Status.json");
        if let Some(s) = astromech_status::read_status_retry(&p, 5) {
            self.stats.lock().unwrap().status_reads += 1;
            if let Some(f) = &mut self.eddn {
                f.status(&serde_json::json!({ "BodyName": s.body_name }));
            }
            self.hub.apply_status(&s);
        }
        self.on_companions();
    }

    fn on_companions(&mut self) {
        if let Ok(b) = std::fs::read(self.dir.join("Cargo.json")) {
            if let Some(c) = astromech_journal::parse_cargo(&b) {
                self.hub.update(|g| g.apply_cargo(&c));
            }
        }
        if let Ok(b) = std::fs::read(self.dir.join("NavRoute.json")) {
            if let Some(r) = astromech_journal::parse_nav_route(&b) {
                self.hub.update(|g| g.apply_route(&r));
            }
        }
        self.eddn_nav_route();
    }

    /// End of a journal, or shutdown: send a pending signal run.
    fn flush_eddn(&mut self) {
        if let Some(f) = &mut self.eddn {
            f.flush(wall_now());
        }
    }
}

/// Start watching `dir`. `db` is the store file for read positions (None: don't persist).
pub fn start(
    dir: &Path,
    hub: Arc<StateHub>,
    db: Option<&Path>,
    log: impl Fn(&str) + Send + 'static,
) -> Result<Pipeline> {
    start_with_eddn(dir, hub, db, log, None)
}

/// The EDDN feed, if `[eddn] enabled` (off by default). Debug builds send to the `/test`
/// schemas whatever the config says. A feed that can't start is logged and left off.
pub fn eddn_feed(cfg: &crate::config::Config, data_dir: &Path) -> Option<astromech_eddn::Feed> {
    if !cfg.eddn.enabled {
        return None;
    }
    let test = cfg.eddn.test || cfg!(debug_assertions);
    let version = env!("CARGO_PKG_VERSION");
    let software = astromech_eddn::Software { name: "astromech".into(), version: version.into() };
    let transport = astromech_eddn::HttpTransport::new(astromech_eddn::UPLOAD_URL, &format!("astromech/{version}"));
    let db = data_dir.join(astromech_eddn::DB_FILE);
    match astromech_eddn::Feed::start(&db, software, test, Box::new(transport), crate::logging::log) {
        Ok(f) => {
            crate::logging::log(&format!("EDDN: sharing live data{}", if test { " (test schemas)" } else { "" }));
            Some(f)
        }
        Err(e) => {
            crate::logging::log(&format!("EDDN off: {} {e}", db.display()));
            None
        }
    }
}

/// [`start`], also feeding EDDN.
pub fn start_with_eddn(
    dir: &Path,
    hub: Arc<StateHub>,
    db: Option<&Path>,
    log: impl Fn(&str) + Send + 'static,
    eddn: Option<astromech_eddn::Feed>,
) -> Result<Pipeline> {
    let (tx, rx) = channel::<notify::Result<notify::Event>>();
    let mut watcher = notify::recommended_watcher(tx).context("creating file watcher")?;
    watcher.watch(dir, RecursiveMode::NonRecursive).with_context(|| format!("watching {}", dir.display()))?;
    let store = match db {
        Some(p) => Some(astromech_store::Store::open(p).with_context(|| format!("opening {}", p.display()))?),
        None => None,
    };
    let stats = Arc::new(std::sync::Mutex::new(Stats::default()));
    let mut t = Tailer {
        dir: dir.to_path_buf(),
        hub,
        store,
        tail: None,
        stats: stats.clone(),
        log: Box::new(log),
        pending: vec![],
        eddn,
        live: true,
    };
    if let Some(n) = astromech_journal::newest_journal(dir) {
        t.open(n, true);
    } else {
        (t.log)("no journal file yet; waiting for the game to create one");
    }
    t.on_status();
    let stop = Arc::new(AtomicBool::new(false));
    let stop2 = stop.clone();
    let thread = std::thread::Builder::new()
        .name("pipeline".into())
        .spawn(move || {
            while !stop2.load(Ordering::SeqCst) {
                // The timeout only lets us notice `stop`; no file is read on timeout.
                let ev = match rx.recv_timeout(Duration::from_millis(250)) {
                    Ok(Ok(e)) => e,
                    Ok(Err(_)) => continue,
                    Err(RecvTimeoutError::Timeout) => continue,
                    Err(RecvTimeoutError::Disconnected) => break,
                };
                let mut status = false;
                let mut journal = false;
                let mut companions: Vec<CompanionKind> = vec![];
                let mut classify = |paths: &[PathBuf]| {
                    for p in paths {
                        let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                        if name == "Status.json" || name == "Cargo.json" || name == "NavRoute.json" {
                            status = true;
                        } else if let Some(k) = CompanionKind::from_file_name(&name) {
                            if !companions.contains(&k) {
                                companions.push(k);
                            }
                        } else if is_journal(p) {
                            journal = true;
                        }
                    }
                };
                classify(&ev.paths);
                // Coalesce a burst of notifications.
                while let Ok(Ok(e)) = rx.try_recv() {
                    classify(&e.paths);
                }
                if status {
                    t.on_status();
                }
                if journal || status {
                    t.on_journal_activity();
                }
                if !companions.is_empty() {
                    t.on_companion_files(&companions);
                }
            }
            t.flush_eddn();
        })
        .context("starting pipeline thread")?;
    Ok(Pipeline { _watcher: watcher, stop, thread: Some(thread), stats })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn tail_keeps_partial_lines() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("Journal.2026-10-10T120000.01.log");
        let mut f = std::fs::File::create(&p).unwrap();
        write!(f, "{{\"timestamp\":\"t\",\"event\":\"A\"}}\r\n{{\"timestamp\":\"t\",\"ev").unwrap();
        f.flush().unwrap();
        let mut t = JournalTail::new(p.clone(), 0);
        assert_eq!(t.read_new().len(), 1);
        let c = t.committed();
        write!(f, "ent\":\"B\"}}\r\n").unwrap();
        f.flush().unwrap();
        let l = t.read_new();
        assert_eq!(l, vec!["{\"timestamp\":\"t\",\"event\":\"B\"}".to_string()]);
        assert!(t.committed() > c);
    }

    fn tailer(dir: &Path) -> (Tailer, Arc<StateHub>, Arc<std::sync::Mutex<Vec<String>>>) {
        let hub = Arc::new(StateHub::new());
        let logs = Arc::new(std::sync::Mutex::new(vec![]));
        let l2 = logs.clone();
        let t = Tailer {
            dir: dir.to_path_buf(),
            hub: hub.clone(),
            store: None,
            tail: None,
            stats: Arc::new(std::sync::Mutex::new(Stats::default())),
            log: Box::new(move |m| l2.lock().unwrap().push(m.to_string())),
            pending: vec![],
            eddn: None,
            live: true,
        };
        (t, hub, logs)
    }

    const MARKET_EVENT: &str =
        r#"{"timestamp":"2026-10-10T20:00:05Z","event":"Market","MarketID":7,"StationName":"S","StarSystem":"Sys"}"#;
    const MARKET_FILE: &str = r#"{"timestamp":"2026-10-10T20:00:05Z","event":"Market","MarketID":7,"StationName":"S","StarSystem":"Sys","Items":[]}"#;

    #[test]
    fn companion_file_is_read_after_its_event() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("Market.json"), MARKET_FILE).unwrap();
        let (mut t, hub, logs) = tailer(d.path());
        t.apply_lines(vec![MARKET_EVENT.into()]);
        assert_eq!(hub.snapshot().companions.market.unwrap().market_id, Some(7));
        assert!(logs.lock().unwrap().is_empty());
        assert_eq!(t.stats.lock().unwrap().companions_read, 1);
    }

    #[test]
    fn missing_or_stale_file_is_logged_then_used_once_the_game_writes_it() {
        let d = tempfile::tempdir().unwrap();
        let (mut t, hub, logs) = tailer(d.path());
        t.apply_lines(vec![MARKET_EVENT.into()]);
        assert!(hub.snapshot().companions.market.is_none());
        assert!(logs.lock().unwrap()[0].contains("Market.json") && logs.lock().unwrap()[0].contains("missing"));
        // Last visit's file: reported again, still not used.
        let old = MARKET_FILE.replace("20:00:05", "19:00:05");
        std::fs::write(d.path().join("Market.json"), old).unwrap();
        t.on_companion_files(&[CompanionKind::Market]);
        assert!(hub.snapshot().companions.market.is_none());
        assert!(logs.lock().unwrap()[1].contains("older"));
        // The game finishes the write.
        std::fs::write(d.path().join("Market.json"), MARKET_FILE).unwrap();
        t.on_companion_files(&[CompanionKind::Market]);
        assert!(hub.snapshot().companions.market.is_some());
        assert!(t.pending.is_empty());
        assert_eq!(t.stats.lock().unwrap().companions_unused, 2);
    }

    // EDDN: the feed sees every line from each journal's Fileheader, but only live lines and
    // their companion files are queued.

    const SESSION: &str = concat!(
        r#"{"timestamp":"2026-10-16T11:00:00Z","event":"Fileheader","part":1,"gameversion":"4.0.0.1904","build":"r1 "}"#,
        "\n",
        r#"{"timestamp":"2026-10-16T11:00:01Z","event":"LoadGame","Commander":"Jameson","FID":"F1","Horizons":true,"Odyssey":true}"#,
        "\n",
        r#"{"timestamp":"2026-10-16T11:00:02Z","event":"Location","StarSystem":"Sys","SystemAddress":5,"StarPos":[1.0,2.0,3.0]}"#,
        "\n",
    );

    fn docked(ts: &str) -> String {
        format!(
            r#"{{"timestamp":"{ts}","event":"Docked","StationName":"S","MarketID":7,"StarSystem":"Sys","SystemAddress":5}}"#
        )
    }

    fn eddn_tailer(dir: &Path) -> (Tailer, PathBuf) {
        let db = dir.join("eddn.sqlite");
        let sender = astromech_eddn::Sender::new(
            astromech_eddn::Outbox::open(&db).unwrap(),
            astromech_eddn::Software { name: "astromech".into(), version: "test".into() },
            true,
        );
        let (mut t, _, _) = tailer(dir);
        t.eddn = Some(astromech_eddn::Feed::new(sender));
        (t, db)
    }

    /// Messages queued, and the send log's details (oldest first).
    fn eddn_state(db: &Path) -> (usize, Vec<String>) {
        let o = astromech_eddn::Outbox::open(db).unwrap();
        (o.pending().unwrap(), o.recent(100).unwrap().into_iter().rev().map(|e| e.detail).collect())
    }

    fn append(p: &Path, text: &str) {
        let mut f = std::fs::OpenOptions::new().append(true).open(p).unwrap();
        writeln!(f, "{text}").unwrap();
    }

    #[test]
    fn eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it() {
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join("Journal.2026-10-16T110000.01.log");
        std::fs::write(&j, format!("{SESSION}{}\n{MARKET_EVENT}\n", docked("2026-10-16T11:00:03Z"))).unwrap();
        std::fs::write(d.path().join("Market.json"), MARKET_FILE).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j.clone(), true);
        assert_eq!(eddn_state(&db), (0, vec![]), "nothing from the catch-up, companion file included");

        append(&j, &docked("2026-10-16T11:05:00Z"));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (1, vec![]), "the live Docked");

        // A live Market event reaches the feed with its file: commodity/3 is queued.
        append(&j, MARKET_EVENT);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (2, vec![]));

        // The game starts a new journal: live from its Fileheader, which is replayed into the
        // feed, so its Location is sent with the new header.
        let j2 = d.path().join("Journal.2026-10-16T120000.01.log");
        std::fs::write(&j2, SESSION.replace("r1 ", "r2 ")).unwrap();
        let later = std::time::SystemTime::now() + Duration::from_secs(5);
        std::fs::File::options().write(true).open(&j2).unwrap().set_modified(later).unwrap();
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 3);
        assert_eq!(t.tail.as_ref().unwrap().path, j2);
    }
}
