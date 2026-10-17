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
//! - Live or replay: one notion for EDDN and callouts (`Tailer::live`). A journal line is live
//!   when the game wrote it while we run: not the startup replay of the current journal, not an
//!   older or already-read journal that becomes newest (re-read as a replay), but every line the
//!   game appends and a journal it starts later. Status.json reads are live once the startup
//!   catch-up (the replay, then the first Status.json read) is done.
//! - Callouts: every journal line and Status.json read also goes to the reaction engine
//!   ([`crate::callouts`]) after the game state has it, and so does every `NavRoute.json` read
//!   (a plotted route); only live ones may fire, and reactions are delivered with no lock held.
//! - EDDN (when switched on): every journal line goes to the [`astromech_eddn::Feed`] in order,
//!   and the companion files to it after their event. Lines replayed at startup are fed as not
//!   live, so they rebuild its header, commander and location but nothing from them is sent; a
//!   journal the game starts while we run (later by name, not opened before) is live from its
//!   `Fileheader`. Any other journal that becomes newest is re-read as a replay. Delivery runs on the
//!   feed's own thread, so the network never holds up this one.
//! - EDSM (when switched on): every journal line also goes to the [`astromech_eddn::edsm::Uploader`],
//!   with the same notion of live; it sends to the commander's EDSM account from its own thread.

use crate::callouts::Callouts;
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
use std::time::{Duration, Instant};

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

/// Is journal `a` later than `b`? Names carry the start time (`Journal.2026-10-16T120000.01.log`,
/// or `Journal.161016120000.01.log` before 2021), so they sort in time order.
fn later(a: &Path, b: &Path) -> bool {
    a.file_name() > b.file_name()
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
    /// Every `Outfitting.json` seen, kept next to `store`'s file.
    outfitting: Option<astromech_station::OutfittingBook>,
    tail: Option<JournalTail>,
    stats: Arc<std::sync::Mutex<Stats>>,
    log: Box<dyn Fn(&str) + Send>,
    /// Companion events whose file was not usable yet; tried again when the game touches the file.
    pending: Vec<Trigger>,
    callouts: Option<Callouts>,
    eddn: Option<astromech_eddn::Feed>,
    edsm: Option<astromech_eddn::edsm::Uploader>,
    /// False while replaying a journal written before we started.
    live: bool,
    /// The startup catch-up is done (until then nothing is live, Status.json included).
    caught_up: bool,
    /// Journals opened this run: opening one again is a replay, never live.
    opened: std::collections::HashSet<PathBuf>,
}

impl Tailer {
    /// Apply journal lines to the state, then read the companion files their events announced.
    fn apply_lines(&mut self, lines: Vec<String>) {
        let mut parsed = Vec::with_capacity(lines.len());
        let mut fired = vec![];
        {
            let mut st = self.stats.lock().unwrap();
            for l in lines {
                match astromech_journal::parse_line(&l) {
                    Ok(j) => {
                        st.journal_lines += 1;
                        self.hub.apply_journal(&j);
                        if let Some(c) = &mut self.callouts {
                            c.engine.set_live(self.caught_up && self.live);
                            fired.extend(self.hub.read(|g| c.engine.journal(&j, g, Instant::now())));
                        }
                        if self.eddn.is_some() || self.edsm.is_some() {
                            if let Ok(raw) = serde_json::from_str(&j.raw_json) {
                                if let Some(f) = &mut self.eddn {
                                    f.journal(&raw, self.live, wall_now());
                                }
                                if let Some(u) = &mut self.edsm {
                                    u.journal(&raw, self.live, wall_now());
                                }
                            }
                        }
                        parsed.push(j);
                    }
                    Err(_) => st.bad_lines += 1,
                }
            }
        }
        // Delivered with no lock held.
        if let (Some(c), false) = (&self.callouts, fired.is_empty()) {
            (c.deliver)(fired);
        }
        // Events first, files after: a slow file never delays the journal state.
        let found = triggers(&parsed);
        if !found.is_empty() {
            self.pending.retain(|p| !found.iter().any(|t| t.kind == p.kind));
            self.load_companions(found);
        }
        self.eddn_nav_route();
    }

    /// The clock, for the rules that remind by time (mission expiry).
    fn on_tick(&mut self) {
        if let Some(c) = &mut self.callouts {
            c.engine.set_live(self.caught_up);
            let fired = self.hub.read(|g| c.engine.tick(chrono::Utc::now().timestamp(), g, Instant::now()));
            if !fired.is_empty() {
                (c.deliver)(fired);
            }
        }
    }

    /// `NavRoute.json` for a live `NavRoute` event: read after the event and whenever the game
    /// writes the file, until the feed has the file the event announced.
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
            // The game overwrites this file at every visit and the journal doesn't hold its
            // module list: this is the only chance to keep it.
            if let (CompanionFile::Outfitting(o), Some(book)) = (&f, &mut self.outfitting) {
                if let Err(e) = book.record(o) {
                    (self.log)(&format!("outfitting: can't keep {} ({}): {e}", o.station_name, o.timestamp));
                }
            }
            if let (Some(feed), Some((event, raw))) = (&mut self.eddn, eddn_file(&f)) {
                if let Ok(file) = serde_json::from_str(raw) {
                    feed.companion_file(event, &file, wall_now());
                }
            }
            let is_market = matches!(f, CompanionFile::Market(_));
            self.hub.update(|g| g.apply_companion(f));
            // A fresh Market.json is an event for the callouts (once per dock; the engine decides).
            if let (true, Some(c)) = (is_market, &mut self.callouts) {
                c.engine.set_live(self.caught_up && self.live);
                let fired = self.hub.read(|g| {
                    let m = g.companions.market.clone()?;
                    Some(c.engine.market(&m, g, Instant::now()))
                });
                if let Some(fired) = fired.filter(|f| !f.is_empty()) {
                    (c.deliver)(fired);
                }
            }
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
        self.opened.insert(path.clone());
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
                // Newest by mtime isn't always the game's new journal: a backup or sync tool can
                // touch an old one, or another client can write the same folder. Only a journal
                // not opened yet this run and later by name than the current one is live.
                let live = !self.opened.contains(&n) && self.tail.as_ref().is_none_or(|t| later(&n, &t.path));
                self.open(n, !live);
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
            if let Some(c) = &mut self.callouts {
                c.engine.set_live(self.caught_up);
                let fired = self.hub.read(|g| c.engine.status(&s, g, Instant::now()));
                (c.deliver)(fired);
            }
        }
        self.on_companions();
    }

    /// Startup: replay the current journal to rebuild state, read Status.json, then go live.
    fn catch_up(&mut self) {
        if let Some(n) = astromech_journal::newest_journal(&self.dir) {
            self.open(n, true);
        } else {
            (self.log)("no journal file yet; waiting for the game to create one");
        }
        self.on_status();
        self.caught_up = true;
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
                // A new route is an event for the callouts (the engine ignores the same one again).
                if let Some(c) = &mut self.callouts {
                    c.engine.set_live(self.caught_up);
                    let fired = self.hub.read(|g| c.engine.route(&r, g, Instant::now()));
                    if !fired.is_empty() {
                        (c.deliver)(fired);
                    }
                }
            }
        }
    }

    /// End of a journal, or shutdown: send a pending signal run.
    fn flush_eddn(&mut self) {
        if let Some(f) = &mut self.eddn {
            f.flush(wall_now());
        }
    }
}

/// Start watching `dir`. `db` is the store file for read positions (None: don't persist).
/// `callouts`: the reaction engine and where its reactions go (None: no callouts).
pub fn start(
    dir: &Path,
    hub: Arc<StateHub>,
    db: Option<&Path>,
    log: impl Fn(&str) + Send + 'static,
    callouts: Option<Callouts>,
) -> Result<Pipeline> {
    start_with_eddn(dir, hub, db, log, None, None, callouts)
}

/// The user's EDDN switch (`[eddn] enabled`, Settings → Data sharing), read by the feed on every
/// event and by its worker before every post, so switching it takes effect at once. The core
/// sets it from the config at start and on every edit.
pub static EDDN_SHARING: std::sync::LazyLock<Arc<AtomicBool>> = std::sync::LazyLock::new(Arc::default);

/// Whether EDDN messages go to the `/test` schemas: when the config says so, and always in a
/// debug build.
pub fn eddn_test(cfg: &crate::config::Config) -> bool {
    cfg.eddn.test || cfg!(debug_assertions)
}

/// The EDDN feed. It runs even while sharing is off, so the header, commander and location are
/// right the moment it's switched on; off, it queues nothing and its worker posts nothing. A
/// feed that can't start is logged and left out.
pub fn eddn_feed(cfg: &crate::config::Config, data_dir: &Path) -> Option<astromech_eddn::Feed> {
    let test = eddn_test(cfg);
    let version = env!("CARGO_PKG_VERSION");
    let software = astromech_eddn::Software { name: "astromech".into(), version: version.into() };
    let transport = astromech_eddn::HttpTransport::new(astromech_eddn::UPLOAD_URL, &format!("astromech/{version}"));
    let db = data_dir.join(astromech_eddn::DB_FILE);
    let sharing = EDDN_SHARING.clone();
    let on = sharing.load(Ordering::SeqCst);
    match astromech_eddn::Feed::start(&db, software, test, Box::new(transport), sharing, crate::logging::log) {
        Ok(f) => {
            let state = if on { "sharing live data" } else { "off (Settings → Data sharing)" };
            crate::logging::log(&format!("EDDN: {state}{}", if test { ", test schemas" } else { "" }));
            Some(f)
        }
        Err(e) => {
            crate::logging::log(&format!("EDDN off: {} {e}", db.display()));
            None
        }
    }
}

/// The user's EDSM switch (`[edsm]` in the config, the API key from the OS keyring): the
/// credentials while uploading is on. Read by the uploader on every event and by its worker
/// before every request. The core sets it at start and on every edit.
pub static EDSM: std::sync::LazyLock<Arc<astromech_eddn::edsm::Switch>> = std::sync::LazyLock::new(Arc::default);

/// The EDSM uploader. Like EDDN's feed it runs while sharing is off, so the commander, system
/// and ship are right the moment it's switched on. One that can't start is logged and left out.
pub fn edsm_uploader(data_dir: &Path) -> Option<astromech_eddn::edsm::Uploader> {
    use astromech_eddn::edsm;
    let version = env!("CARGO_PKG_VERSION");
    let software = astromech_eddn::Software { name: "astromech".into(), version: version.into() };
    let api = edsm::api::HttpApi::new(&format!("astromech/{version}"));
    let db = data_dir.join(edsm::DB_FILE);
    let on = EDSM.get().is_some();
    match edsm::Uploader::start(&db, software, Box::new(api), EDSM.clone(), crate::logging::log) {
        Ok(u) => {
            crate::logging::log(if on {
                "EDSM: uploading live journal"
            } else {
                "EDSM: off (Settings → Data sharing)"
            });
            Some(u)
        }
        Err(e) => {
            crate::logging::log(&format!("EDSM off: {} {e}", db.display()));
            None
        }
    }
}

/// [`start`], also feeding EDDN and EDSM.
pub fn start_with_eddn(
    dir: &Path,
    hub: Arc<StateHub>,
    db: Option<&Path>,
    log: impl Fn(&str) + Send + 'static,
    eddn: Option<astromech_eddn::Feed>,
    edsm: Option<astromech_eddn::edsm::Uploader>,
    callouts: Option<Callouts>,
) -> Result<Pipeline> {
    let (tx, rx) = channel::<notify::Result<notify::Event>>();
    let mut watcher = notify::recommended_watcher(tx).context("creating file watcher")?;
    watcher.watch(dir, RecursiveMode::NonRecursive).with_context(|| format!("watching {}", dir.display()))?;
    let store = match db {
        Some(p) => Some(astromech_store::Store::open(p).with_context(|| format!("opening {}", p.display()))?),
        None => None,
    };
    // Next to the store: `outfitting.sqlite`. A book that won't open costs the history of
    // outfitting, not the pipeline.
    let outfitting = db.and_then(Path::parent).and_then(|d| {
        astromech_station::OutfittingBook::open(&d.join(astromech_station::DB_FILE))
            .map_err(|e| log(&format!("outfitting: can't open the book, not keeping Outfitting.json: {e}")))
            .ok()
    });
    let stats = Arc::new(std::sync::Mutex::new(Stats::default()));
    let mut t = Tailer {
        dir: dir.to_path_buf(),
        hub,
        store,
        outfitting,
        tail: None,
        stats: stats.clone(),
        log: Box::new(log),
        pending: vec![],
        callouts,
        eddn,
        edsm,
        live: true,
        caught_up: false,
        opened: Default::default(),
    };
    t.catch_up();
    let stop = Arc::new(AtomicBool::new(false));
    let stop2 = stop.clone();
    let thread = std::thread::Builder::new()
        .name("pipeline".into())
        .spawn(move || {
            let mut last_tick = Instant::now();
            while !stop2.load(Ordering::SeqCst) {
                if last_tick.elapsed() >= Duration::from_secs(1) {
                    last_tick = Instant::now();
                    t.on_tick();
                }
                // The timeout lets us notice `stop` and tick; no file is read on timeout.
                let ev = match rx.recv_timeout(Duration::from_millis(250)) {
                    Ok(Ok(e)) => e,
                    Ok(Err(_)) => continue,
                    Err(RecvTimeoutError::Timeout) => continue,
                    Err(RecvTimeoutError::Disconnected) => break,
                };
                let mut status = false;
                let mut journal = false;
                let mut companions: Vec<CompanionKind> = vec![];
                let mut nav_route = false;
                let mut classify = |paths: &[PathBuf]| {
                    for p in paths {
                        let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                        if name == "Status.json" || name == "Cargo.json" || name == "NavRoute.json" {
                            status = true;
                            nav_route |= name == "NavRoute.json";
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
                if nav_route {
                    t.eddn_nav_route();
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
            outfitting: None,
            tail: None,
            stats: Arc::new(std::sync::Mutex::new(Stats::default())),
            log: Box::new(move |m| l2.lock().unwrap().push(m.to_string())),
            pending: vec![],
            callouts: None,
            eddn: None,
            edsm: None,
            live: true,
            caught_up: false,
            opened: Default::default(),
        };
        (t, hub, logs)
    }

    /// "Cargo scan." on every `Scanned`, no cooldown.
    struct ScanRule;
    impl astromech_announce::Rule for ScanRule {
        fn id(&self) -> &'static str {
            "scan"
        }
        fn cooldown(&self) -> Duration {
            Duration::ZERO
        }
        fn react(
            &self,
            input: &astromech_announce::Input,
            _: &astromech_gamestate::GameState,
        ) -> Option<astromech_announce::Callout> {
            use astromech_announce::*;
            matches!(input, Input::Journal(l) if l.name == "Scanned").then(|| Callout {
                text: "Cargo scan.".into(),
                priority: Priority::Normal,
                channels: Channels::SPEECH,
            })
        }
    }

    fn scanned(ts: &str) -> String {
        format!("{{\"timestamp\":\"2026-10-16T{ts}Z\",\"event\":\"Scanned\",\"ScanType\":\"Cargo\"}}\r\n")
    }

    #[test]
    fn callouts_are_silent_for_the_startup_catch_up_and_live_after_it() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("Journal.2026-10-16T195000.01.log");
        std::fs::write(&p, scanned("19:50:00") + &scanned("19:55:00")).unwrap();
        // A half-written line at the end of the catch-up belongs to the live part.
        let mut f = std::fs::OpenOptions::new().append(true).open(&p).unwrap();
        write!(f, "{{\"timestamp\":\"2026-10-16T20:00:00Z\",\"event\":\"Scan").unwrap();
        std::fs::write(d.path().join("Status.json"), r#"{"timestamp":"t","event":"Status","Flags":16842765}"#).unwrap();
        let (mut t, hub, _) = tailer(d.path());
        let fired = Arc::new(std::sync::Mutex::new(vec![]));
        let f2 = fired.clone();
        t.callouts = Some(Callouts {
            engine: astromech_announce::Engine::new(vec![Box::new(ScanRule)], &Default::default()),
            deliver: Box::new(move |rs| f2.lock().unwrap().extend(rs.into_iter().map(|r| r.text))),
        });
        // Delivery happens with the stats lock released (a slow sink must not hold it).
        let stats = t.stats.clone();
        let unlocked = Arc::new(std::sync::Mutex::new(vec![]));
        let u2 = unlocked.clone();
        let inner = std::mem::replace(&mut t.callouts.as_mut().unwrap().deliver, Box::new(|_| {}));
        t.callouts.as_mut().unwrap().deliver = Box::new(move |rs| {
            u2.lock().unwrap().push(stats.try_lock().is_ok());
            inner(rs)
        });
        t.catch_up();
        assert_eq!(t.stats.lock().unwrap().journal_lines, 2);
        assert!(hub.snapshot().status_seen);
        assert!(fired.lock().unwrap().is_empty(), "{:?}", fired.lock().unwrap());
        // Live: the rest of the half line, then a new line.
        write!(f, "ned\",\"ScanType\":\"Cargo\"}}\r\n{}", scanned("20:00:05")).unwrap();
        f.flush().unwrap();
        t.on_journal_activity();
        assert_eq!(*fired.lock().unwrap(), ["Cargo scan.", "Cargo scan."]);
        // A journal file the game starts later is live from its first line.
        std::fs::write(d.path().join("Journal.2026-10-16T201000.01.log"), scanned("20:10:00")).unwrap();
        t.on_journal_activity();
        assert_eq!(fired.lock().unwrap().len(), 3);
        // The same notion of live as EDDN: an older journal that becomes newest (a sync tool
        // touched it) is re-read as a replay, so none of its lines fire again. The touch is what
        // a sync tool does: its own handle, closed, then a later modified time. Writing through
        // the game's still-open handle `f` doesn't work on Windows: NTFS updates the directory
        // entry (which `newest_journal`'s listing reads) only when a handle closes. The explicit
        // time also keeps the order from depending on timestamp granularity.
        drop(f);
        {
            let mut g = std::fs::OpenOptions::new().append(true).open(&p).unwrap();
            writeln!(g, "{{\"timestamp\":\"2026-10-16T20:11:00Z\",\"event\":\"Music\"}}").unwrap();
        }
        let later = std::time::SystemTime::now() + Duration::from_secs(3600);
        std::fs::OpenOptions::new().write(true).open(&p).unwrap().set_modified(later).unwrap();
        t.on_journal_activity();
        assert_eq!(astromech_journal::newest_journal(d.path()), Some(p.clone()));
        let got = fired.lock().unwrap().clone();
        assert_eq!(got.len(), 3, "{got:?}");
        let u = unlocked.lock().unwrap();
        assert!(!u.is_empty() && u.iter().all(|ok| *ok), "{u:?}");
    }

    #[test]
    fn a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read() {
        let d = tempfile::tempdir().unwrap();
        let route = |hops: &[&str]| {
            let h: Vec<String> = hops
                .iter()
                .enumerate()
                .map(|(i, c)| format!(r#"{{"StarSystem":"S{i}","StarPos":[{}.0,0,0],"StarClass":"{c}"}}"#, i * 10))
                .collect();
            format!(r#"{{"timestamp":"t","event":"NavRoute","Route":[{}]}}"#, h.join(","))
        };
        std::fs::write(d.path().join("NavRoute.json"), route(&["K", "M"])).unwrap();
        std::fs::write(d.path().join("Status.json"), r#"{"timestamp":"t","event":"Status","Flags":16842765}"#).unwrap();
        let (mut t, hub, _) = tailer(d.path());
        let fired = Arc::new(std::sync::Mutex::new(vec![]));
        let f2 = fired.clone();
        t.callouts = Some(Callouts {
            engine: astromech_announce::Engine::new(
                vec![Box::new(astromech_announce::route::Route)],
                &Default::default(),
            ),
            deliver: Box::new(move |rs| f2.lock().unwrap().extend(rs.into_iter().map(|r| r.text))),
        });
        t.catch_up();
        assert_eq!(hub.snapshot().route_hops.len(), 2);
        // Every Status.json read reads NavRoute.json again: the startup route is not news.
        t.on_status();
        assert!(fired.lock().unwrap().is_empty(), "{:?}", fired.lock().unwrap());
        std::fs::write(d.path().join("NavRoute.json"), route(&["K", "DA", "M"])).unwrap();
        t.on_status();
        t.on_status();
        assert_eq!(*fired.lock().unwrap(), ["Route: 2 jumps, 20 ly.", "White dwarf at jump 1."]);
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

    #[test]
    fn outfitting_file_is_kept_when_fresh_and_only_then() {
        let d = tempfile::tempdir().unwrap();
        let event = r#"{"timestamp":"2026-10-10T20:00:06Z","event":"Outfitting","MarketID":7,"StationName":"S","StarSystem":"Sys"}"#;
        let file = r#"{"timestamp":"2026-10-10T20:00:06Z","event":"Outfitting","MarketID":7,"StationName":"S","StarSystem":"Sys","Items":[{"id":1,"Name":"int_hyperdrive_size5_class5","BuyPrice":9}]}"#;
        let (mut t, _hub, _logs) = tailer(d.path());
        t.outfitting = Some(astromech_station::OutfittingBook::in_memory().unwrap());
        // Last visit's file: not this event's, so not kept.
        std::fs::write(d.path().join("Outfitting.json"), file.replace("20:00:06", "19:00:06")).unwrap();
        t.apply_lines(vec![event.into()]);
        assert_eq!(t.outfitting.as_ref().unwrap().counts().unwrap(), (0, 0));
        // The game finishes the write.
        std::fs::write(d.path().join("Outfitting.json"), file).unwrap();
        t.on_companion_files(&[CompanionKind::Outfitting]);
        let book = t.outfitting.as_ref().unwrap();
        assert_eq!(book.counts().unwrap(), (1, 1));
        assert_eq!(book.find("5a frame shift", 5).unwrap()[0].station_name, "S");
    }

    // EDDN: the feed sees every line from each journal's Fileheader, but only live lines and
    // their companion files are queued.

    /// A session's opening lines, stamped `ts`.
    fn session(ts: &str, build: &str) -> String {
        [
            format!(r#"{{"timestamp":"{ts}","event":"Fileheader","part":1,"gameversion":"4.0.0.1904","build":"{build}"}}"#),
            format!(r#"{{"timestamp":"{ts}","event":"LoadGame","Commander":"Jameson","FID":"F1","Horizons":true,"Odyssey":true}}"#),
            format!(r#"{{"timestamp":"{ts}","event":"Location","StarSystem":"Sys","SystemAddress":5,"StarPos":[1.0,2.0,3.0]}}"#),
            String::new(),
        ]
        .join("\n")
    }

    /// The game's timestamp `secs_ago` seconds before now (live lines must be recent).
    fn ts(secs_ago: i64) -> String {
        (chrono::Utc::now() - chrono::Duration::seconds(secs_ago)).format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    fn touch(p: &Path, secs_from_now: u64) {
        let t = std::time::SystemTime::now() + Duration::from_secs(secs_from_now);
        std::fs::File::options().write(true).open(p).unwrap().set_modified(t).unwrap();
    }

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
        let market_event = MARKET_EVENT.replace("2026-10-10T20:00:05Z", &ts(60));
        let market_file = MARKET_FILE.replace("2026-10-10T20:00:05Z", &ts(60));
        std::fs::write(&j, format!("{}{}\n{market_event}\n", session(&ts(120), "r1 "), docked(&ts(90)))).unwrap();
        std::fs::write(d.path().join("Market.json"), &market_file).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j.clone(), true);
        assert_eq!(eddn_state(&db), (0, vec![]), "nothing from the catch-up, companion file included");

        append(&j, &docked(&ts(0)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (1, vec![]), "the live Docked");

        // A live Market event reaches the feed with its file: commodity/3 is queued.
        let market_event = MARKET_EVENT.replace("2026-10-10T20:00:05Z", &ts(0));
        std::fs::write(d.path().join("Market.json"), MARKET_FILE.replace("2026-10-10T20:00:05Z", &ts(0))).unwrap();
        append(&j, &market_event);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (2, vec![]));

        // The game starts a new journal: live from its Fileheader, which is replayed into the
        // feed, so its Location is sent with the new header.
        let j2 = d.path().join("Journal.2026-10-16T120000.01.log");
        std::fs::write(&j2, session(&ts(0), "r2 ")).unwrap();
        touch(&j2, 5);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 3);
        assert_eq!(t.tail.as_ref().unwrap().path, j2);
    }

    #[test]
    fn edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay() {
        use astromech_eddn::edsm::{Credentials, Discards, Switch, Uploader};
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join("Journal.2026-10-16T110000.01.log");
        std::fs::write(&j, format!("{}{}\n", session(&ts(120), "r1 "), docked(&ts(90)))).unwrap();
        let db = d.path().join(astromech_eddn::edsm::DB_FILE);
        let switch = Arc::new(Switch::default());
        switch.set(Some(Credentials { commander: "Jameson".into(), api_key: "k".into() }));
        let (mut t, _, _) = tailer(d.path());
        t.edsm = Some(Uploader::new(astromech_eddn::Outbox::open(&db).unwrap(), switch, Arc::new(Discards::default())));
        t.open(j.clone(), true);
        let queue = astromech_eddn::Outbox::open(&db).unwrap();
        assert_eq!(queue.pending().unwrap(), 0, "nothing from the catch-up");

        append(&j, r#"{"timestamp":"2026-10-16T11:30:00Z","event":"MarketBuy","Type":"gold","Count":1}"#);
        t.on_journal_activity();
        let q = queue.due(i64::MAX, true, 10).unwrap();
        assert_eq!(q.len(), 1, "the live line, however old its timestamp (EDSM keeps history)");
        let body: serde_json::Value = serde_json::from_slice(&q[0].body).unwrap();
        assert_eq!((&body["message"]["_systemName"], &body["message"]["_stationName"]), (&"Sys".into(), &"S".into()));
    }

    #[test]
    fn an_older_journal_touched_later_is_read_as_a_replay() {
        let d = tempfile::tempdir().unwrap();
        let (j1, j2) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        // Recent timestamps, so only the file choice keeps them from being sent.
        std::fs::write(&j1, format!("{}{}\n", session(&ts(30), "r1 "), docked(&ts(20)))).unwrap();
        std::fs::write(&j2, session(&ts(10), "r2 ")).unwrap();
        touch(&j2, 5);
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j2.clone(), true);

        // A backup tool rewrites the older journal: newest by mtime, but earlier by name.
        touch(&j1, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j1);
        assert_eq!(eddn_state(&db), (0, vec![]), "an older journal is never live");

        // Back to the newer one (seen already this run): also a replay.
        touch(&j2, 20);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (0, vec![]));
        // Lines the game appends to it are live again.
        append(&j2, &docked(&ts(0)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1);
    }
}
