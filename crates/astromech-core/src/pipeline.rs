//! Journal + Status.json pipeline. Driven by file notifications, plus a read of the journal
//! whenever 250 ms have passed since the last read (`Tailer::on_wake`), because on Windows they can be late or lost.
//!
//! - Status.json: every change notification re-reads the file (with retries for half-written
//!   files) and applies the flags.
//! - Journal: the newest `Journal.*.log` is tailed from a byte offset; only complete lines are
//!   parsed. When the game starts a new journal file, the old one is drained and the new one
//!   becomes current. On startup the current file is replayed from the start (to rebuild
//!   state) and the saved position from the store tells us how much of it is new.
//! - Because Windows may not report appends to a file the game keeps open until it flushes
//!   metadata, the tail is read on every Status.json notification (several per second in game)
//!   and also whenever 250 ms pass since the last read, however the loop was woken
//!   (`Tailer::on_wake`), so a journal line is applied without any file event. ⚠ Unverified on the real game.
//! - Companion files (`Market.json`, ...): read after the journal event that announces them,
//!   once (no waiting: that would stall the journal). A file that is missing or older than its
//!   event is logged and not used; it is tried again when the game next writes that file, and
//!   also after every journal read that follows a Status.json write and once a second when idle,
//!   in case the notification never comes, until the event is older than five minutes.
//! - Live or replay: one notion for EDDN and callouts (`Tailer::live`). A journal line is live
//!   when the game wrote it while we run: not the startup replay of the current journal, not an
//!   older or already-read journal that becomes newest (re-read as a replay, but only up to where
//!   we had read it: what the game wrote since is live), but every line the game appends and a
//!   journal it starts later. Status.json reads are live once the startup
//!   catch-up (the replay, then the first Status.json read) is done.
//! - Callouts: every journal line and Status.json read also goes to the reaction engine
//!   ([`crate::callouts`]) after the game state has it, and so does every `NavRoute.json` read
//!   (a plotted route); only live ones may fire, and reactions are delivered with no lock held.
//! - EDDN (when switched on): every journal line goes to the [`astromech_eddn::Feed`] in order,
//!   and the companion files to it after their event. Lines replayed at startup are fed as not
//!   live, so they rebuild its header, commander and location but nothing from them is sent; a
//!   journal the game starts while we run (later by name, not opened before) is live from its
//!   `Fileheader`. Any other journal that becomes newest is re-read as a replay (up to where it had been read, if it had). Delivery runs on the
//!   feed's own thread, so the network never holds up this one.
//! - EDSM (when switched on): every journal line also goes to the [`astromech_eddn::edsm::Uploader`],
//!   with the same notion of live; it sends to the commander's EDSM account from its own thread.

use crate::callouts::Callouts;
use crate::hub::StateHub;
use anyhow::{Context, Result};
use astromech_eddn::worker::wall_now;
use astromech_journal::companion::{load, triggers, CompanionFile, CompanionKind, Trigger};
use astromech_journal::JournalTail;
use notify::{RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// How long to wait for a file notification before reading the journal anyway.
const IDLE: Duration = Duration::from_millis(250);
/// How often an idle pipeline looks for a new journal file.
const SCAN_EVERY: Duration = Duration::from_secs(1);

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
    /// Journals opened this run, with how far we had read when we left each (`None` while
    /// followed). Opening one again replays up to that point; what the game wrote since is live.
    opened: std::collections::HashMap<PathBuf, Option<u64>>,
    /// When `on_idle` last looked for a new journal.
    last_scan: Instant,
    /// When the journal tail was last read, for any reason.
    last_read: Instant,
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
            self.load_companions(found, true);
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

    /// `report`: log and count the files that couldn't be used. A retry that fails again stays
    /// quiet (it would repeat on every Status.json write); the event stays pending.
    fn load_companions(&mut self, found: Vec<Trigger>, report: bool) {
        let (files, reports) = load(&self.dir, &found);
        {
            let mut st = self.stats.lock().unwrap();
            st.companions_read += files.len() as u64;
            if report {
                st.companions_unused += reports.len() as u64;
            }
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
            if report {
                (self.log)(&format!("companion file {r}"));
            }
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
        self.load_companions(waiting, true);
    }

    /// Try every waiting event again, whether or not the watcher reported its file: Windows can
    /// drop the notification for a file the game rewrites (astromech-ogq4). Run after the journal
    /// read, so a newer event's file is never taken for an older event's. An event older than
    /// [`astromech_eddn::feed::MAX_EVENT_AGE_SECS`] is given up on (once, logged): its file isn't coming.
    fn retry_pending(&mut self) {
        if self.pending.is_empty() {
            return;
        }
        let now = chrono::Utc::now();
        let (waiting, stale): (Vec<Trigger>, Vec<Trigger>) =
            std::mem::take(&mut self.pending).into_iter().partition(|p| {
                chrono::DateTime::parse_from_rfc3339(&p.event_ts)
                    .map_or(true, |t| (now - t.to_utc()).num_seconds() <= astromech_eddn::feed::MAX_EVENT_AGE_SECS)
            });
        for p in stale {
            (self.log)(&format!("companion file {}: gave up waiting for the {} event", p.kind.file_name(), p.event_ts));
        }
        self.load_companions(waiting, false);
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
        // A journal we followed before is a replay only up to where we had read it.
        let (replayed, live) = match self.opened.get(&path).copied().flatten().filter(|_| replay) {
            Some(read) => (t.read_to(read), t.read_new()),
            None => (t.read_new(), vec![]),
        };
        let n = replayed.len() + live.len();
        self.opened.insert(path.clone(), None);
        self.live = !replay;
        self.apply_lines(replayed);
        self.live = true;
        self.apply_lines(live);
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

    /// Read what the game appended to the journal we follow, without waiting to be told.
    fn read_tail(&mut self) -> bool {
        self.last_read = Instant::now();
        let Some(t) = &mut self.tail else { return false };
        let lines = t.read_new();
        let any = !lines.is_empty();
        self.apply_lines(lines);
        any
    }

    fn on_journal_activity(&mut self) {
        self.read_tail();
        self.follow_newest();
        self.retry_pending();
        self.save();
    }

    /// The loop woke for some reason (a notification, or the timeout): read the journal if that
    /// hasn't happened for [`IDLE`], so notifications that name no journal can't starve it.
    fn on_wake(&mut self) {
        if self.last_read.elapsed() >= IDLE {
            self.on_idle();
        }
    }

    /// Nothing was reported for a while. File notifications are not a clock: on Windows the ones
    /// for an open, growing journal can arrive late or never (astromech-mh3m), so read the tail
    /// anyway (an open and a read at the offset, nothing when nothing was appended) and look for
    /// a new journal once a second.
    fn on_idle(&mut self) {
        if self.read_tail() {
            self.save();
        }
        if self.last_scan.elapsed() >= SCAN_EVERY {
            self.last_scan = Instant::now();
            self.follow_newest();
            self.retry_pending();
        }
    }

    /// Switch to the game's newest journal when it isn't the one we follow.
    fn follow_newest(&mut self) {
        let newest = astromech_journal::newest_journal(&self.dir);
        if let Some(n) = newest {
            if self.tail.as_ref().map(|t| t.path != n).unwrap_or(true) {
                if self.tail.is_some() {
                    self.stats.lock().unwrap().rollovers += 1;
                }
                self.flush_eddn();
                if let Some(t) = &self.tail {
                    self.opened.insert(t.path.clone(), Some(t.committed()));
                }
                // Newest by mtime isn't always the game's new journal: a backup or sync tool can
                // touch an old one, or another client can write the same folder. Only a journal
                // not opened yet this run and later by name than the current one is live.
                let live = !self.opened.contains_key(&n) && self.tail.as_ref().is_none_or(|t| later(&n, &t.path));
                self.open(n, !live);
            }
        }
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

/// The pipeline thread: react to file notifications, and read the journal when none arrive.
fn spawn_loop(
    mut t: Tailer,
    rx: std::sync::mpsc::Receiver<notify::Result<notify::Event>>,
    stop2: Arc<AtomicBool>,
) -> Result<std::thread::JoinHandle<()>> {
    std::thread::Builder::new()
        .name("pipeline".into())
        .spawn(move || {
            let mut last_tick = Instant::now();
            while !stop2.load(Ordering::SeqCst) {
                if last_tick.elapsed() >= Duration::from_secs(1) {
                    last_tick = Instant::now();
                    t.on_tick();
                }
                // Whatever woke us, read the journal if it hasn't been for IDLE: with no
                // notification at all the timeout below is what wakes us.
                t.on_wake();
                let ev = match rx.recv_timeout(IDLE) {
                    Ok(Ok(e)) => e,
                    Ok(Err(_)) | Err(RecvTimeoutError::Timeout) => continue,
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
                // Coalesce a burst of notifications, but not so long that the journal waits for it.
                while let Ok(Ok(e)) = rx.try_recv() {
                    classify(&e.paths);
                    if t.last_read.elapsed() >= IDLE {
                        break;
                    }
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
        .context("starting pipeline thread")
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
        last_scan: Instant::now(),
        last_read: Instant::now(),
    };
    t.catch_up();
    let stop = Arc::new(AtomicBool::new(false));
    let thread = spawn_loop(t, rx, stop.clone())?;
    Ok(Pipeline { _watcher: watcher, stop, thread: Some(thread), stats })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

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
            last_scan: Instant::now(),
            last_read: Instant::now(),
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

    /// astromech-mh3m: no notification of any kind (not for the journal, not Status.json,
    /// Cargo.json or NavRoute.json) must be needed to apply a line appended to the open journal.
    #[test]
    fn a_line_appended_to_the_open_journal_is_applied_when_idle() {
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join("Journal.2026-10-16T110000.01.log");
        std::fs::write(&j, format!("{}\n", session(&ts(120), "r1 "))).unwrap();
        let (mut t, hub, _) = tailer(d.path());
        t.open(j.clone(), true);
        t.caught_up = true;
        assert_eq!(hub.snapshot().system.as_deref(), Some("Sys"));
        let jump = r#"{"timestamp":"2026-10-16T11:30:00Z","event":"FSDJump","StarSystem":"Sol","SystemAddress":10477373803,"StarPos":[0,0,0],"JumpDist":8.5}"#;
        append(&j, jump);
        t.on_idle();
        assert_eq!(hub.snapshot().system.as_deref(), Some("Sol"));
        // Nothing appended: idle reads change nothing.
        t.on_idle();
        assert_eq!(hub.snapshot().system.as_deref(), Some("Sol"));
    }

    /// astromech-mh3m: through the real loop, with a watcher that never reports anything, a line
    /// appended to the open journal still reaches the state (well within the generous deadline).
    #[test]
    fn the_pipeline_loop_reads_the_journal_without_any_notification() {
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join("Journal.2026-10-16T110000.01.log");
        std::fs::write(&j, format!("{}\n", session(&ts(120), "r1 "))).unwrap();
        let (mut t, hub, _) = tailer(d.path());
        t.catch_up();
        let (_silent_watcher, rx) = std::sync::mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let thread = spawn_loop(t, rx, stop.clone()).unwrap();
        append(
            &j,
            r#"{"timestamp":"2026-10-16T11:30:00Z","event":"FSDJump","StarSystem":"Sol","SystemAddress":10477373803,"StarPos":[0,0,0],"JumpDist":8.5}"#,
        );
        let deadline = Instant::now() + Duration::from_secs(5);
        while hub.snapshot().system.as_deref() != Some("Sol") && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        stop.store(true, Ordering::SeqCst);
        thread.join().unwrap();
        assert_eq!(hub.snapshot().system.as_deref(), Some("Sol"), "the loop never read the appended line");
    }

    /// astromech-mosx: a steady stream of notifications that name no journal (companion files,
    /// other files, watcher errors) must not keep the loop from reading the journal.
    #[test]
    fn a_stream_of_unrelated_notifications_does_not_starve_the_journal_read() {
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join("Journal.2026-10-16T110000.01.log");
        std::fs::write(&j, format!("{}\n", session(&ts(120), "r1 "))).unwrap();
        let (mut t, hub, _) = tailer(d.path());
        t.catch_up();
        let (tx, rx) = std::sync::mpsc::channel();
        let flood_stop = Arc::new(AtomicBool::new(false));
        let (fs, other) = (flood_stop.clone(), d.path().join("Unrelated.txt"));
        let flood = std::thread::spawn(move || {
            while !fs.load(Ordering::SeqCst) {
                let e = notify::Event::new(notify::EventKind::Any).add_path(other.clone());
                if tx.send(Ok(e)).is_err() {
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        });
        let stop = Arc::new(AtomicBool::new(false));
        let thread = spawn_loop(t, rx, stop.clone()).unwrap();
        append(
            &j,
            r#"{"timestamp":"2026-10-16T11:30:00Z","event":"FSDJump","StarSystem":"Sol","SystemAddress":10477373803,"StarPos":[0,0,0],"JumpDist":8.5}"#,
        );
        let started = Instant::now();
        while hub.snapshot().system.as_deref() != Some("Sol") && started.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(10));
        }
        let waited = started.elapsed();
        flood_stop.store(true, Ordering::SeqCst);
        stop.store(true, Ordering::SeqCst);
        thread.join().unwrap();
        flood.join().unwrap();
        assert_eq!(hub.snapshot().system.as_deref(), Some("Sol"), "the flood starved the journal read");
        assert!(waited < Duration::from_secs(1), "took {waited:?}");
    }

    #[test]
    fn an_idle_pipeline_follows_a_new_journal_within_a_second() {
        let d = tempfile::tempdir().unwrap();
        let (j1, j2) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        std::fs::write(&j1, format!("{}\n", session(&ts(120), "r1 "))).unwrap();
        let (mut t, _, _) = tailer(d.path());
        t.open(j1, true);
        t.last_scan = Instant::now();
        std::fs::write(&j2, format!("{}\n", session(&ts(1), "r2 "))).unwrap();
        touch(&j2, 5);
        t.on_idle();
        assert_ne!(t.tail.as_ref().unwrap().path, j2, "the scan is throttled to once a second");
        t.last_scan = Instant::now() - SCAN_EVERY;
        t.on_idle();
        assert_eq!(t.tail.as_ref().unwrap().path, j2);
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

    /// astromech-ogq4: the game rewrites Backpack.json with each BackpackChange and writes no
    /// Backpack event, so the snapshot has to follow the change event.
    #[test]
    fn backpack_json_is_read_again_after_a_backpack_change() {
        let d = tempfile::tempdir().unwrap();
        let file = |ts: &str, name: &str| {
            format!(
                r#"{{"timestamp":"{ts}","event":"Backpack","Items":[{{"Name":"{name}","Count":1}}],"Components":[],"Consumables":[],"Data":[]}}"#
            )
        };
        let event = |ts: &str, name: &str| match name {
            "Backpack" => format!(r#"{{"timestamp":"{ts}","event":"Backpack"}}"#),
            _ => format!(
                r#"{{"timestamp":"{ts}","event":"BackpackChange","Added":[{{"Name":"x","Type":"Item","Count":1}}]}}"#
            ),
        };
        let items = |hub: &StateHub| {
            hub.snapshot().companions.backpack.map(|b| b.items.iter().map(|i| i.name.clone()).collect::<Vec<_>>())
        };
        let (mut t, hub, _) = tailer(d.path());
        std::fs::write(d.path().join("Backpack.json"), file("2026-10-10T20:00:00Z", "energycell")).unwrap();
        t.apply_lines(vec![event("2026-10-10T20:00:00Z", "Backpack")]);
        assert_eq!(items(&hub), Some(vec!["energycell".to_string()]));

        std::fs::write(d.path().join("Backpack.json"), file("2026-10-10T20:05:00Z", "bypass")).unwrap();
        t.apply_lines(vec![event("2026-10-10T20:05:00Z", "BackpackChange")]);
        assert_eq!(items(&hub), Some(vec!["bypass".to_string()]));
    }

    fn market_event(ts: &str, id: i64) -> String {
        format!(r#"{{"timestamp":"{ts}","event":"Market","MarketID":{id},"StationName":"S{id}","StarSystem":"Sys"}}"#)
    }
    fn market_file(ts: &str, id: i64) -> String {
        format!(
            r#"{{"timestamp":"{ts}","event":"Market","MarketID":{id},"StationName":"S{id}","StarSystem":"Sys","Items":[]}}"#
        )
    }

    /// astromech-ogq4: a pending companion event must not depend on the watcher reporting its
    /// file; the journal read that follows a Status.json write tries it again, quietly.
    #[test]
    fn a_pending_companion_event_is_loaded_on_the_next_status_read() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("Status.json"), r#"{"timestamp":"t","event":"Status","Flags":16842765}"#).unwrap();
        let (mut t, hub, logs) = tailer(d.path());
        let now = ts(0);
        t.apply_lines(vec![market_event(&now, 7)]);
        for _ in 0..2 {
            t.on_status();
            t.on_journal_activity();
        }
        assert!(hub.snapshot().companions.market.is_none());
        assert_eq!(t.pending.len(), 1);
        assert_eq!(logs.lock().unwrap().len(), 1, "retries from the status read don't repeat the report");
        assert_eq!(t.stats.lock().unwrap().companions_unused, 1);

        // The game finishes the write; no notification for Market.json reaches us.
        std::fs::write(d.path().join("Market.json"), market_file(&now, 7)).unwrap();
        t.on_status();
        t.on_journal_activity();
        assert_eq!(hub.snapshot().companions.market.unwrap().market_id, Some(7));
        assert!(t.pending.is_empty());
    }

    /// Docked and idle there is no Status.json write either: the idle scan retries too.
    #[test]
    fn a_pending_companion_event_is_loaded_when_idle() {
        let d = tempfile::tempdir().unwrap();
        let (mut t, hub, _) = tailer(d.path());
        let now = ts(0);
        t.apply_lines(vec![market_event(&now, 7)]);
        std::fs::write(d.path().join("Market.json"), market_file(&now, 7)).unwrap();
        t.on_idle();
        assert!(hub.snapshot().companions.market.is_none(), "the retry is throttled to the scan rate");
        t.last_scan = Instant::now() - SCAN_EVERY;
        t.on_idle();
        assert_eq!(hub.snapshot().companions.market.unwrap().market_id, Some(7));
    }

    /// A retry runs after the journal read: the station-8 file must not be taken for the
    /// station-7 event, and is read once for the station-8 event.
    #[test]
    fn a_retry_never_runs_ahead_of_the_journal_lines_of_its_batch() {
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join("Journal.2026-10-16T110000.01.log");
        std::fs::write(&j, format!("{}\n", session(&ts(120), "r1 "))).unwrap();
        std::fs::write(d.path().join("Status.json"), r#"{"timestamp":"t","event":"Status","Flags":16842765}"#).unwrap();
        let (mut t, hub, _) = tailer(d.path());
        t.open(j.clone(), true);
        let (t7, t8) = (ts(30), ts(0));
        append(&j, &market_event(&t7, 7));
        t.on_journal_activity();
        assert_eq!(t.pending.len(), 1);
        let read_before = t.stats.lock().unwrap().companions_read;

        // Station 8: its file and its journal line arrive together with a Status.json write.
        std::fs::write(d.path().join("Market.json"), market_file(&t8, 8)).unwrap();
        append(&j, &market_event(&t8, 8));
        t.on_status();
        t.on_journal_activity();
        assert_eq!(hub.snapshot().companions.market.unwrap().market_id, Some(8));
        assert_eq!(t.stats.lock().unwrap().companions_read - read_before, 1, "the file was read twice");
        assert!(t.pending.is_empty());
    }

    #[test]
    fn a_pending_event_whose_file_never_comes_is_given_up_once() {
        let d = tempfile::tempdir().unwrap();
        let (mut t, _, logs) = tailer(d.path());
        t.apply_lines(vec![market_event(&ts(10 * 60), 7)]);
        assert_eq!(t.pending.len(), 1);
        let before = logs.lock().unwrap().len();
        t.retry_pending();
        t.retry_pending();
        assert!(t.pending.is_empty());
        let log = logs.lock().unwrap();
        assert_eq!(log.len() - before, 1);
        assert!(log.last().unwrap().contains("gave up"));
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

    /// astromech-7uj3: what the game wrote to the current journal while a touched older one was
    /// followed is live when we come back to it; only what we had read before is the replay.
    #[test]
    fn a_reopened_journal_replays_only_what_was_read_before() {
        let d = tempfile::tempdir().unwrap();
        let (j1, j2) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        std::fs::write(&j1, format!("{}{}\n", session(&ts(30), "r1 "), docked(&ts(20)))).unwrap();
        std::fs::write(&j2, session(&ts(10), "r2 ")).unwrap();
        touch(&j2, 5);
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j2.clone(), true);

        // A backup tool touches the older journal; we follow it (a replay).
        touch(&j1, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j1);
        assert_eq!(eddn_state(&db), (0, vec![]));

        // The game writes to its journal meanwhile (nothing reaches us), then the next event
        // makes it newest again.
        append(&j2, &docked(&ts(0)));
        touch(&j2, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j2);
        assert_eq!(eddn_state(&db).0, 1, "the line written while another journal was followed is live");
    }

    /// Leaving and coming back to the same journal more than once keeps the split exact: every
    /// line is queued once, in the batch it belongs to.
    #[test]
    fn a_journal_left_twice_is_split_at_the_latest_read_point() {
        let d = tempfile::tempdir().unwrap();
        let (j1, j2) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        std::fs::write(&j1, session(&ts(30), "r1 ")).unwrap();
        std::fs::write(&j2, session(&ts(10), "r2 ")).unwrap();
        touch(&j2, 5);
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j2.clone(), true);
        let mut clock = 10;
        let mut go = |t: &mut Tailer, to: &PathBuf| {
            clock += 10;
            touch(to, clock);
            t.on_journal_activity();
            assert_eq!(&t.tail.as_ref().unwrap().path, to);
        };
        let dock = |id: i64| docked(&ts(0)).replace("\"MarketID\":7", &format!("\"MarketID\":{id}"));

        go(&mut t, &j1);
        append(&j2, &dock(8));
        go(&mut t, &j2);
        assert_eq!(eddn_state(&db).0, 1);
        go(&mut t, &j1);
        go(&mut t, &j2);
        assert_eq!(eddn_state(&db).0, 1, "nothing new: nothing sent again");
        go(&mut t, &j1);
        append(&j2, &dock(9));
        go(&mut t, &j2);
        assert_eq!(eddn_state(&db).0, 2, "only the new line is live");
    }

    /// A journal followed while still empty, left, and written to meanwhile: nothing had been read
    /// from it (committed offset 0), so all of it is live when we come back, not a full replay.
    #[test]
    fn a_journal_left_while_empty_is_live_when_reopened() {
        let d = tempfile::tempdir().unwrap();
        let (j1, j2) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        std::fs::write(&j1, session(&ts(30), "r1 ")).unwrap();
        std::fs::write(&j2, "").unwrap();
        touch(&j2, 5);
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j2.clone(), true);
        touch(&j1, 10);
        t.on_journal_activity();
        append(&j2, &format!("{}{}", session(&ts(10), "r2 "), docked(&ts(0))));
        touch(&j2, 20);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 2);
    }
}
