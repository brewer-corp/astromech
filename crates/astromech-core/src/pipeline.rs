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
//!   in case the notification never comes, until the event is older than five minutes. A retry
//!   is as live as the event's line was, however live the journal is by then.
//! - Live or replay: one notion for EDDN and callouts (`Tailer::live`). A journal line is live
//!   when the game wrote it while we run: not the startup replay of the current journal, not what
//!   any other journal held at startup, not an already-read journal that becomes newest (re-read
//!   as a replay, but only up to where we had read it, or its length at startup: what the game
//!   wrote since is live), never a line of a journal the game doesn't
//!   write (`Tailer::classify`: the folder's latest by name stamp, or one whose Fileheader time
//!   agrees with its name's local time, whose lines stamped now are live, so a clock that went
//!   back, a foreign journal stamped later or a late start can't silence it; never a conflict
//!   copy, a copy from another PC's offset, or the old lines of a restored one), but every line the
//!   game appends and a
//!   journal it starts later. Status.json reads are live once the startup
//!   catch-up (the replay, then the first Status.json read) is done.
//! - Callouts: every journal line and Status.json read also goes to the reaction engine
//!   ([`crate::callouts`]) after the game state has it, and so does every `NavRoute.json` read
//!   (a plotted route); only live ones may fire, and reactions are delivered with no lock held.
//! - EDDN (when switched on): every journal line goes to the [`astromech_eddn::Feed`] in order,
//!   and the companion files to it after their event. Lines replayed at startup are fed as not
//!   live, so they rebuild its header, commander and location but nothing from them is sent; a
//!   journal the game starts while we run (the game's by `Tailer::classify`, not in the folder at
//!   startup nor opened before) is live from its `Fileheader`. Any other journal that becomes
//!   newest is re-read as a replay (up to where it had been read, or its length at startup, if
//!   either). Delivery runs on the
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

/// What the pipeline knows about a followed journal (see `Tailer::classify`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    /// The game writes it: everything appended is live.
    Game,
    /// Its header agrees with its name, so the game wrote it here, but it may have been written by
    /// a tool since: only lines stamped now are live.
    Gated,
    /// A restored or copied journal, a conflict copy, another PC's: a replay only.
    Foreign,
}

/// The UTC time (seconds) in a journal's first line, if it is there yet.
fn first_line_time(path: &Path) -> Option<i64> {
    use std::io::BufRead;
    let mut line = String::new();
    std::io::BufReader::new(std::fs::File::open(path).ok()?).read_line(&mut line).ok()?;
    if !line.ends_with('\n') {
        return None; // still being written
    }
    let v: serde_json::Value = serde_json::from_str(line.trim()).ok()?;
    if v.get("event")?.as_str()? != "Fileheader" {
        return None;
    }
    chrono::DateTime::parse_from_rfc3339(v.get("timestamp")?.as_str()?).ok().map(|t| t.timestamp())
}

/// Whether a journal's Fileheader time (UTC seconds) is its name's time read in `tz`. `name_secs`
/// is the name's stamp as `journal_name_stamp` gives it (the name's clock reading taken as UTC).
/// Twice a year a local time is ambiguous (the fall-back hour: accept either reading) or doesn't
/// exist (the spring-forward gap: the game never wrote it, so no).
fn header_agrees_with_name<Tz: chrono::TimeZone>(tz: &Tz, name_secs: i64, header: i64) -> bool {
    let Some(naive) = chrono::DateTime::from_timestamp(name_secs, 0).map(|t| t.naive_utc()) else { return false };
    let near = |t: chrono::DateTime<Tz>| (t.timestamp() - header).abs() <= HEADER_SLACK_SECS;
    match tz.from_local_datetime(&naive) {
        chrono::LocalResult::Single(t) => near(t),
        chrono::LocalResult::Ambiguous(a, b) => near(a) || near(b),
        chrono::LocalResult::None => false,
    }
}

/// The machine's own timezone for [`header_agrees_with_name`].
fn local_header_check() -> Box<dyn Fn(i64, i64) -> bool + Send> {
    Box::new(|name, header| header_agrees_with_name(&chrono::Local, name, header))
}

/// Whether a journal line is stamped within [`FRESH_LINE_SECS`] of now (either side).
fn line_is_fresh(line: &str) -> bool {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { return false };
    let Some(t) =
        v.get("timestamp").and_then(|t| t.as_str()).and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
    else {
        return false;
    };
    (chrono::Utc::now().timestamp() - t.timestamp()).abs() <= FRESH_LINE_SECS
}

/// How long to wait for a file notification before reading the journal anyway.
const IDLE: Duration = Duration::from_millis(250);
/// How far a Fileheader's time may be from its name's local time (names have 1 s resolution).
const HEADER_SLACK_SECS: i64 = 120;
/// How old a line of a [`Class::Gated`] journal may be and still be live.
const FRESH_LINE_SECS: i64 = 120;
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

/// A companion event waiting for its file, and whether its journal line was live: a retry after
/// the live half has begun must not make a replayed dock's Market.json live (astromech-01c0).
#[derive(Debug, Clone)]
struct Pending {
    trigger: Trigger,
    live: bool,
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
    pending: Vec<Pending>,
    callouts: Option<Callouts>,
    eddn: Option<astromech_eddn::Feed>,
    edsm: Option<astromech_eddn::edsm::Uploader>,
    /// False while replaying a journal written before we started, and for any journal that
    /// isn't the folder's latest by name.
    live: bool,
    /// The startup catch-up is done (until then nothing is live, Status.json included).
    caught_up: bool,
    /// Journals opened this run or already in the folder at startup, with how far we had read
    /// when we left each, or its length at startup (`None` while followed). Opening one again
    /// replays up to that point; what the game wrote since is live.
    opened: std::collections::HashMap<PathBuf, Option<u64>>,
    /// When `on_idle` last looked for a new journal.
    last_scan: Instant,
    /// When the journal tail was last read, for any reason.
    last_read: Instant,
    /// Name stamp of the session the game is writing now (see `classify`); a later part of it
    /// (same stamp) is the game's too.
    game_session: Option<i64>,
    /// Name stamps of sessions the game has left for a newer one: never the game's again, so a
    /// tool appending to an earlier journal can't make its lines live (astromech-r3s7).
    retired: std::collections::HashSet<i64>,
    /// Journals already logged as followed without being the folder's latest.
    noted: std::collections::HashSet<PathBuf>,
    /// Whether a journal's Fileheader time agrees with its name's local time, once it could be read.
    header_agrees: std::collections::HashMap<PathBuf, bool>,
    /// Whether a Fileheader time (UTC seconds) is a journal name's local time: `(name, header)`.
    /// The machine's own timezone; tests set another.
    header_check: Box<dyn Fn(i64, i64) -> bool + Send>,
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
            self.pending.retain(|p| !found.iter().any(|t| t.kind == p.trigger.kind));
            self.load_companions(found, true, self.caught_up && self.live);
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
    /// quiet (it would repeat on every Status.json write); the event stays pending. `live`: the
    /// events' journal lines were live (only then may a Market.json fire a callout).
    fn load_companions(&mut self, found: Vec<Trigger>, report: bool, live: bool) {
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
                c.engine.set_live(live);
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
            self.pending.push(Pending { trigger: Trigger { kind: r.kind, event_ts: r.event_ts }, live });
        }
    }

    /// Try `waiting` again, each as live as the line that announced it.
    fn reload(&mut self, waiting: Vec<Pending>, report: bool) {
        let (live, replayed): (Vec<Pending>, Vec<Pending>) = waiting.into_iter().partition(|p| p.live);
        for (batch, live) in [(replayed, false), (live, true)] {
            if !batch.is_empty() {
                self.load_companions(batch.into_iter().map(|p| p.trigger).collect(), report, live);
            }
        }
    }

    /// The game wrote one of these files: try the events that were waiting for it.
    fn on_companion_files(&mut self, kinds: &[CompanionKind]) {
        let (waiting, rest): (Vec<Pending>, Vec<Pending>) =
            std::mem::take(&mut self.pending).into_iter().partition(|p| kinds.contains(&p.trigger.kind));
        self.pending = rest;
        self.reload(waiting, true);
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
        let (waiting, stale): (Vec<Pending>, Vec<Pending>) =
            std::mem::take(&mut self.pending).into_iter().partition(|p| {
                chrono::DateTime::parse_from_rfc3339(&p.trigger.event_ts)
                    .map_or(true, |t| (now - t.to_utc()).num_seconds() <= astromech_eddn::feed::MAX_EVENT_AGE_SECS)
            });
        for Pending { trigger: p, .. } in stale {
            (self.log)(&format!("companion file {}: gave up waiting for the {} event", p.kind.file_name(), p.event_ts));
        }
        self.reload(waiting, false);
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
        let known = self.opened.get(&path).copied().flatten().filter(|_| replay);
        let (replayed, live) = match known {
            Some(read) => (t.read_to(read), t.read_new()),
            None => (t.read_new(), vec![]),
        };
        let n = replayed.len() + live.len();
        self.opened.insert(path.clone(), None);
        // Only the game's own journal is live (see `classify`), including what lands in it after
        // we open it: a backup or sync tool may create a file and then write it, and we can
        // follow it in between.
        let class = self.classify(&path);
        self.note_if_not_latest(&path, class);
        self.apply_class(&path, class, replayed, !replay, known.is_none());
        self.apply_class(&path, class, live, true, true);
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

    /// Whether `path` is a journal the game itself writes, as opposed to one a backup or sync tool
    /// restored, touched or copied. The game's are:
    /// - [`Class::Game`], whose appended lines are all live: the session the game is writing, which
    ///   is the folder's latest by name stamp the first time we see it (the common case, and all
    ///   there is at startup), and a later part of it (same stamp). Starting a newer session leaves
    ///   the earlier one for good (`retired`): it is then judged like any other journal;
    /// - [`Class::Gated`], whose fresh lines are live: any other journal whose Fileheader agrees
    ///   with its name. The game names a journal with its local start time (checked against real
    ///   journals: the stamp is the header's UTC time shifted by the machine's offset), so a
    ///   header that matches the name was written by the game on this machine, however the name
    ///   ranks against the others (the clock went back for daylight saving, another PC with a later
    ///   or skewed clock synced the folder, the app started or resumed long after the session
    ///   began: astromech-36g1, astromech-t16r). A restored or copied journal can agree too, so
    ///   only its lines stamped within [`FRESH_LINE_SECS`] of now are live: what a tool writes late
    ///   is old.
    ///
    /// Anything else, including a conflict copy (a name that isn't a stamp) and a copy from a PC
    /// with another offset (header and name disagree), is [`Class::Foreign`]: a replay only.
    fn classify(&mut self, path: &Path) -> Class {
        let Some((stamp, _)) = astromech_journal::journal_name_stamp(path) else { return Class::Foreign };
        // The session the game is writing: no directory listing.
        if self.game_session == Some(stamp) {
            return Class::Game;
        }
        let latest = !self.retired.contains(&stamp)
            && astromech_journal::latest_journal(&self.dir).is_some_and(|l| l.file_name() == path.file_name());
        if latest {
            self.set_game_session(stamp);
            Class::Game
        } else if self.header_agrees_with_name(path, stamp) {
            Class::Gated
        } else {
            Class::Foreign
        }
    }

    /// The game is writing the session stamped `stamp`: the one before it is left for good.
    fn set_game_session(&mut self, stamp: i64) {
        if let Some(old) = self.game_session.filter(|old| *old != stamp) {
            self.retired.insert(old);
        }
        self.retired.remove(&stamp);
        self.game_session = Some(stamp);
    }

    /// Whether the Fileheader's UTC time is the name's local time. Remembered once the header
    /// could be read (a journal may be followed while still empty).
    fn header_agrees_with_name(&mut self, path: &Path, stamp: i64) -> bool {
        if let Some(known) = self.header_agrees.get(path) {
            return *known;
        }
        let Some(header) = first_line_time(path) else { return false };
        let agrees = (self.header_check)(stamp, header);
        self.header_agrees.insert(path.to_path_buf(), agrees);
        agrees
    }

    /// Apply `lines` of the journal at `path` by its class. `game_live`: a [`Class::Game`] batch is
    /// live (not at the startup replay). `may_be_live`: a [`Class::Gated`] batch may hold fresh
    /// lines (not at startup, nor for lines read before).
    fn apply_class(&mut self, path: &Path, class: Class, lines: Vec<String>, game_live: bool, may_be_live: bool) {
        match class {
            Class::Game => {
                self.live = game_live;
                self.apply_lines(lines);
            }
            Class::Foreign => {
                self.live = false;
                self.apply_lines(lines);
            }
            Class::Gated if self.caught_up && may_be_live => {
                // Each line by its own time, in order: the stale ones are the replay, the fresh
                // ones live. Runs of equal freshness are applied together.
                let mut run: Vec<String> = vec![];
                let mut run_fresh = false;
                for line in lines {
                    let fresh = line_is_fresh(&line);
                    if fresh != run_fresh && !run.is_empty() {
                        self.apply_gated_run(path, std::mem::take(&mut run), run_fresh);
                    }
                    run_fresh = fresh;
                    run.push(line);
                }
                if !run.is_empty() {
                    self.apply_gated_run(path, run, run_fresh);
                }
            }
            Class::Gated => {
                self.live = false;
                self.apply_lines(lines);
            }
        }
    }

    fn apply_gated_run(&mut self, path: &Path, lines: Vec<String>, fresh: bool) {
        if fresh {
            // The game is writing it: a later part of this session is too.
            if let Some((stamp, _)) = astromech_journal::journal_name_stamp(path) {
                self.set_game_session(stamp);
            }
        }
        self.live = fresh;
        self.apply_lines(lines);
    }

    /// Log once when the journal we follow isn't the folder's latest by name: whether it is live
    /// anyway, or is a restored or copied one that is only replayed.
    fn note_if_not_latest(&mut self, path: &Path, class: Class) {
        let latest = astromech_journal::latest_journal(&self.dir);
        if latest.as_deref().is_some_and(|l| l.file_name() == path.file_name())
            || !self.noted.insert(path.to_path_buf())
        {
            return;
        }
        let name = |p: &Path| p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        (self.log)(&format!(
            "journal {} isn't the latest by name ({}): {}",
            name(path),
            latest.as_deref().map(name).unwrap_or_default(),
            match class {
                Class::Game => "followed as the game's own",
                Class::Gated => "followed as the game's own (its header matches its name); fresh lines are live",
                Class::Foreign => "followed as a replay only",
            }
        ));
    }

    /// Read what the game appended to the journal we follow, without waiting to be told.
    fn read_tail(&mut self) -> bool {
        self.last_read = Instant::now();
        let Some(t) = &mut self.tail else { return false };
        let (path, lines) = (t.path.clone(), t.read_new());
        let any = !lines.is_empty();
        if any {
            // Judged on every batch: a journal followed while still empty has no header yet.
            let class = self.classify(&path);
            self.apply_class(&path, class, lines, true, true);
        }
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
                // that is new this run (not opened, not in the folder at startup) and the game's
                // by its name stamp (`classify`, both naming schemes) is live from its first line.
                let live = !self.opened.contains_key(&n) && (self.tail.is_none() || self.classify(&n) == Class::Game);
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
        self.note_startup_lengths();
        self.on_status();
        self.caught_up = true;
    }

    /// Everything in the folder at startup is history: a journal followed later is a replay up to
    /// its length now, and only what the game appends afterwards is live. The catch-up follows the
    /// newest by modified time, which a sync tool can make an older journal, so the game's current
    /// one may only be followed once the game next writes it (astromech-nrhv). The game's latest
    /// journal is measured through an open handle: on Windows a directory listing lags appends to
    /// a file the game holds open (astromech-txzq). The others are closed, so the listing is right.
    fn note_startup_lengths(&mut self) {
        let Ok(entries) = std::fs::read_dir(&self.dir) else { return };
        let latest = astromech_journal::latest_journal(&self.dir);
        for e in entries.flatten() {
            let path = e.path();
            if !is_journal(&path) {
                continue;
            }
            let len = if latest.as_ref() == Some(&path) {
                std::fs::File::open(&path).and_then(|f| f.metadata())
            } else {
                e.metadata()
            };
            if let Ok(m) = len {
                // The one the catch-up follows is in already (`None`: followed).
                self.opened.entry(path).or_insert(Some(m.len()));
            }
        }
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
        game_session: None,
        retired: Default::default(),
        noted: Default::default(),
        header_agrees: Default::default(),
        header_check: local_header_check(),
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
            game_session: None,
            retired: Default::default(),
            noted: Default::default(),
            header_agrees: Default::default(),
            header_check: local_header_check(),
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

    /// astromech-nrhv: a sync tool touched an older journal just before we started, so the
    /// startup catch-up follows it. The game's current journal (later by name) was already there:
    /// what it held at startup is history, and only what the game appends afterwards is live.
    #[test]
    fn the_games_journal_is_a_replay_up_to_startup_after_a_catch_up_on_a_touched_older_one() {
        let d = tempfile::tempdir().unwrap();
        let (older, current) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        // Recent timestamps, so only the replay rule keeps them from being sent or spoken.
        std::fs::write(&older, format!("{}{}\n", session(&ts(600), "r1 "), docked(&ts(500)))).unwrap();
        std::fs::write(&current, format!("{}{}\n{}", session(&ts(60), "r2 "), docked(&ts(40)), scanned_at(&ts(30))))
            .unwrap();
        touch(&older, 5);
        let (mut t, db) = eddn_tailer(d.path());
        let fired = Arc::new(std::sync::Mutex::new(vec![]));
        let f2 = fired.clone();
        t.callouts = Some(Callouts {
            engine: astromech_announce::Engine::new(vec![Box::new(ScanRule)], &Default::default()),
            deliver: Box::new(move |rs| f2.lock().unwrap().extend(rs.into_iter().map(|r| r.text))),
        });
        t.catch_up();
        assert_eq!(t.tail.as_ref().unwrap().path, older, "startup follows the newest by modified time");

        // The game writes its current journal.
        append(&current, &format!("{}\n{}", docked(&ts(0)), scanned_at(&ts(0)).trim_end()));
        touch(&current, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, current);
        assert_eq!(eddn_state(&db).0, 1, "only the appended Docked, not the history written before startup");
        assert_eq!(*fired.lock().unwrap(), ["Cargo scan."], "only the appended scan");
    }

    /// astromech-5gxp: a returning player's newest journal is old-scheme (`Journal.YYMMDDHHMMSS`),
    /// whose name sorts after the new scheme's byte by byte. The journal the game starts is still
    /// the later one by its stamp, so it is live from its Fileheader.
    #[test]
    fn a_new_scheme_journal_started_after_an_old_scheme_one_is_live() {
        let d = tempfile::tempdir().unwrap();
        let old = d.path().join("Journal.221014080000.01.log");
        std::fs::write(&old, session(&ts(3600), "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        assert_eq!(t.tail.as_ref().unwrap().path, old);

        let new = d.path().join(game_name(0));
        std::fs::write(&new, format!("{}{}\n", session(&ts(1), "r2 "), docked(&ts(0)))).unwrap();
        touch(&new, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, new);
        // The new header's Location and the Docked.
        assert_eq!(eddn_state(&db).0, 2, "the opening lines of the game's new journal are live");
    }

    /// A `Scanned` line stamped `ts` (full RFC 3339).
    fn scanned_at(ts: &str) -> String {
        format!("{{\"timestamp\":\"{ts}\",\"event\":\"Scanned\",\"ScanType\":\"Cargo\"}}\n")
    }

    /// astromech-0d7l: a sync tool restoring an older journal creates it, then writes it. We may
    /// follow it in between (its creation makes it newest), but what lands in it afterwards is no
    /// more live than what was there: the game only writes its latest journal.
    #[test]
    fn an_older_journal_written_after_we_follow_it_is_still_a_replay() {
        let d = tempfile::tempdir().unwrap();
        let (j1, j2) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        std::fs::write(&j2, session(&ts(10), "r2 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j2.clone(), true);

        std::fs::write(&j1, "").unwrap();
        touch(&j1, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j1, "following the (still empty) older journal");
        append(&j1, &format!("{}{}", session(&ts(30), "r1 "), docked(&ts(0))));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (0, vec![]), "lines written to an older journal are never live");

        // Followed, left and written again: past the read point is still not the game's.
        touch(&j2, 20);
        t.on_journal_activity();
        append(&j1, &docked(&ts(0)));
        touch(&j1, 30);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j1);
        assert_eq!(eddn_state(&db), (0, vec![]), "nor when we come back to it");

        // The game's journal is still live once it's newest again.
        append(&j2, &docked(&ts(0)));
        touch(&j2, 40);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1);
    }

    /// A sync tool's conflict copy (a name that isn't a journal stamp) is never the game's
    /// journal, so it is never live, and it doesn't stop the game's own journal being live.
    #[test]
    fn a_conflict_copy_is_never_live_and_does_not_silence_the_game() {
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join("Journal.2026-10-16T120000.01.log");
        let copy = d.path().join("Journal.2026-10-16T120000.01-DESKTOP-ABC.log");
        std::fs::write(&j, session(&ts(10), "r2 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.open(j.clone(), true);

        std::fs::write(&copy, "").unwrap();
        touch(&copy, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, copy);
        append(&copy, &format!("{}{}", session(&ts(5), "r2 "), docked(&ts(0))));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (0, vec![]), "the copy is not the game's");

        append(&j, &docked(&ts(0)));
        touch(&j, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j);
        assert_eq!(eddn_state(&db).0, 1, "the game's journal is live with the copy in the folder");
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

    /// "Now" as the game would stamp a journal that starts now: local time, in the name's format.
    fn game_name(minutes_from_now: i64) -> String {
        let t = chrono::Local::now().naive_local() + chrono::Duration::minutes(minutes_from_now);
        format!("Journal.{}.01.log", t.format("%Y-%m-%dT%H%M%S"))
    }

    /// astromech-36g1, shape 1: the clock went back (daylight saving, travel), so the game's new
    /// session journal is named earlier than the previous one in the folder. It was created after
    /// we started and is stamped now, so it is the game's and its lines are live.
    #[test]
    fn a_new_session_named_earlier_than_the_followed_one_is_live() {
        let d = tempfile::tempdir().unwrap();
        let before = d.path().join(game_name(60));
        std::fs::write(&before, session(&ts(3600), "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        assert_eq!(t.tail.as_ref().unwrap().path, before);

        let new = d.path().join(game_name(0));
        std::fs::write(&new, "").unwrap();
        touch(&new, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, new, "the game's new journal is followed");
        append(&new, &format!("{}{}", session(&ts(1), "r2 "), docked(&ts(0))));
        t.on_journal_activity();
        // The new header's Location and the Docked: both are live.
        assert_eq!(eddn_state(&db).0, 2, "named earlier than the previous journal, but the game's own");
    }

    /// astromech-36g1, shape 2: another PC (later timezone, skewed clock) syncs a journal stamped
    /// later than the game's current one into the folder. Once the game writes again, its lines
    /// are live even though it is no longer the latest by name.
    #[test]
    fn the_games_journal_stays_live_beside_a_foreign_one_stamped_later() {
        let d = tempfile::tempdir().unwrap();
        let (own, foreign) = (d.path().join(game_name(0)), d.path().join(game_name(180)));
        std::fs::write(&own, session(&ts(10), "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();

        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        touch(&foreign, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, foreign);
        append(&own, &docked(&ts(0)));
        touch(&own, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, own);
        assert_eq!(eddn_state(&db).0, 1, "the game's line is live beside a journal stamped later");
    }

    /// The same shapes with a restored or copied journal rather than the game's: an old stamp, a
    /// conflict copy and a later part of a session we don't know are not the game's.
    #[test]
    fn a_journal_stamped_long_ago_is_not_taken_for_the_games() {
        let d = tempfile::tempdir().unwrap();
        let current = d.path().join(game_name(0));
        std::fs::write(&current, session(&ts(10), "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        let restored = d.path().join(game_name(-600));
        std::fs::write(&restored, "").unwrap();
        touch(&restored, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, restored);
        append(&restored, &format!("{}{}", session(&ts(30), "r0 "), docked(&ts(0))));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (0, vec![]));
    }

    /// A followed journal that isn't the folder's latest is logged once, whichever way it goes.
    #[test]
    fn following_a_journal_that_is_not_the_latest_is_logged_once() {
        let d = tempfile::tempdir().unwrap();
        let (current, restored) = (d.path().join(game_name(0)), d.path().join(game_name(-600)));
        std::fs::write(&current, session(&ts(10), "r1 ")).unwrap();
        std::fs::write(&restored, session(&ts(30), "r0 ")).unwrap();
        let (mut t, _, logs) = tailer(d.path());
        t.open(current.clone(), true);
        let count = |logs: &Arc<std::sync::Mutex<Vec<String>>>| {
            logs.lock().unwrap().iter().filter(|l| l.contains("isn't the latest")).count()
        };
        assert_eq!(count(&logs), 0, "the latest is not remarked on");
        for n in 1..=3 {
            touch(&restored, 10 * n);
            t.on_journal_activity();
            touch(&current, 10 * n + 5);
            t.on_journal_activity();
        }
        assert_eq!(count(&logs), 1);
        assert!(logs.lock().unwrap().iter().any(|l| l.contains("replay only")));
    }

    /// astromech-t16r, case 1: the app starts while the game runs and a foreign journal stamped
    /// later is the newest in the folder. The game's own journal (started half an hour ago,
    /// neither the latest by name nor stamped now) is the game's by its header: the Fileheader's
    /// UTC time is its name's local time. Its new lines are live; what it held is the replay.
    #[test]
    fn the_game_running_before_we_start_is_live_beside_a_later_foreign_journal() {
        let d = tempfile::tempdir().unwrap();
        let (own, foreign) = (d.path().join(game_name(-30)), d.path().join(game_name(180)));
        std::fs::write(&own, session(&ts(1800), "r1 ")).unwrap();
        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        touch(&foreign, 10);
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        assert_eq!(t.tail.as_ref().unwrap().path, foreign, "startup follows the newest by modified time");

        append(&own, &docked(&ts(0)));
        touch(&own, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, own);
        assert_eq!(eddn_state(&db).0, 1, "the game's new line is live; its older lines are not");
    }

    /// astromech-t16r, case 2: the game's journal is first seen long after it started (sleep and
    /// resume), and isn't the latest by name.
    #[test]
    fn the_games_journal_first_seen_long_after_it_started_is_live() {
        let d = tempfile::tempdir().unwrap();
        let foreign = d.path().join(game_name(180));
        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();

        let own = d.path().join(game_name(-45));
        std::fs::write(&own, format!("{}{}\n", session(&ts(2700), "r1 "), docked(&ts(0)))).unwrap();
        touch(&own, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, own);
        assert_eq!(eddn_state(&db).0, 1, "only the fresh line is live");
    }

    /// A restored journal of this machine (name and header agree, both old) written late carries
    /// old lines: replay, however its header matches its name. A copy with another PC's offset
    /// (header and name disagree) is not the game's either.
    #[test]
    fn a_restored_journal_written_late_stays_a_replay_whatever_its_header_says() {
        let d = tempfile::tempdir().unwrap();
        let current = d.path().join(game_name(0));
        std::fs::write(&current, session(&ts(10), "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();

        let two_days = 2 * 24 * 3600;
        let restored = d.path().join(game_name(-2 * 24 * 60));
        std::fs::write(&restored, "").unwrap();
        touch(&restored, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, restored);
        append(&restored, &format!("{}{}", session(&ts(two_days), "r0 "), docked(&ts(two_days - 60))));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (0, vec![]), "old lines are a replay");
        assert!(!t.live, "nothing fresh in it");
        // Recent enough for EDDN's own age guard, but not stamped now: a tool wrote it late.
        append(&restored, &docked(&ts(200)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (0, vec![]), "a line from minutes ago is not live either");

        let abroad = d.path().join(game_name(-3 * 60));
        std::fs::write(&abroad, "").unwrap();
        touch(&abroad, 20);
        t.on_journal_activity();
        // Another PC five hours from us: its header (UTC) doesn't match its local-time name.
        append(&abroad, &format!("{}{}", session(&ts(60), "r5 "), docked(&ts(0))));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db), (0, vec![]), "a journal whose header and name disagree is not the game's");
    }

    /// A later part of a session the game is writing (same stamp, header written when the part
    /// began) is the game's too, though its header doesn't match the name.
    #[test]
    fn a_later_part_of_a_session_judged_the_games_is_live() {
        let d = tempfile::tempdir().unwrap();
        let foreign = d.path().join(game_name(180));
        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        let own = d.path().join(game_name(-45));
        std::fs::write(&own, format!("{}{}\n", session(&ts(2700), "r1 "), docked(&ts(0)))).unwrap();
        touch(&own, 10);
        t.on_journal_activity();
        let sent = eddn_state(&db).0;
        assert_eq!(sent, 1);

        let part2 = PathBuf::from(own.to_string_lossy().replace(".01.log", ".02.log"));
        std::fs::write(&part2, session(&ts(5), "r1 ").replace("\"part\":1", "\"part\":2")).unwrap();
        touch(&part2, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, part2);
        append(&part2, &docked(&ts(0)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, sent + 2, "the next part's header Location and Docked are live");
    }

    /// The game's journal followed while still empty (no header yet), then written: judged on its
    /// first batch, whose fresh lines are live.
    #[test]
    fn the_games_journal_followed_while_empty_is_live_once_written() {
        let d = tempfile::tempdir().unwrap();
        let foreign = d.path().join(game_name(180));
        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        let own = d.path().join(game_name(-30));
        std::fs::write(&own, "").unwrap();
        touch(&own, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, own);
        append(&own, &format!("{}{}", session(&ts(1800), "r1 "), docked(&ts(0))));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1);
    }

    /// Lines are judged one by one within a batch: a stale line (or one that doesn't parse) after a
    /// fresh one is not live. Once a fresh line has been seen the journal is the game's, so lines
    /// of later batches are live by design (see `a_gated_batch_...` for the order within one).
    #[test]
    fn gated_lines_are_judged_one_by_one() {
        let d = tempfile::tempdir().unwrap();
        let foreign = d.path().join(game_name(180));
        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        let own = d.path().join(game_name(-30));
        std::fs::write(&own, session(&ts(1800), "r1 ")).unwrap();
        touch(&own, 10);
        t.on_journal_activity();
        let base = eddn_state(&db).0;
        append(&own, &docked(&ts(0)));
        append(&own, &docked(&ts(200)));
        append(
            &own,
            r#"{"timestamp":"garbage","event":"Docked","StationName":"S","MarketID":9,"StarSystem":"Sys","SystemAddress":5}"#,
        );
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, base + 1, "only the fresh line is live");
    }

    /// astromech-t16r review: the daylight-saving branches of the header-vs-name check, in the
    /// timezone of the machine that matters most during the alpha (the US fall-back, 1 Nov 2026).
    #[test]
    fn the_header_agrees_with_the_name_across_daylight_saving() {
        use chrono::TimeZone;
        let ny = chrono_tz::America::New_York;
        let name = |y, mo, d, h, mi| {
            chrono::NaiveDate::from_ymd_opt(y, mo, d).unwrap().and_hms_opt(h, mi, 0).unwrap().and_utc().timestamp()
        };
        let utc = |y, mo, d, h, mi| chrono::Utc.with_ymd_and_hms(y, mo, d, h, mi, 0).unwrap().timestamp();

        // Fall-back: 01:30 happens twice, 05:30Z (EDT) and 06:30Z (EST). Either is the game's.
        let fall = name(2026, 11, 1, 1, 30);
        assert!(header_agrees_with_name(&ny, fall, utc(2026, 11, 1, 5, 30)), "first 01:30, EDT");
        assert!(header_agrees_with_name(&ny, fall, utc(2026, 11, 1, 6, 30)), "second 01:30, EST");
        assert!(!header_agrees_with_name(&ny, fall, utc(2026, 11, 1, 7, 30)), "an hour past the second");
        assert!(!header_agrees_with_name(&ny, fall, utc(2026, 11, 1, 4, 30)), "an hour before the first");

        // A session begun under EDT before the switch still agrees (judged by the date's own offset,
        // not today's), and one begun after it agrees under EST.
        assert!(header_agrees_with_name(&ny, name(2026, 11, 1, 0, 30), utc(2026, 11, 1, 4, 30)));
        assert!(header_agrees_with_name(&ny, name(2026, 11, 1, 2, 30), utc(2026, 11, 1, 7, 30)));
        assert!(
            !header_agrees_with_name(&ny, name(2026, 11, 1, 0, 30), utc(2026, 11, 1, 5, 30)),
            "EST offset on an EDT hour"
        );

        // Spring-forward: 02:30 on 14 Mar 2027 doesn't exist in New York.
        let gap = name(2027, 3, 14, 2, 30);
        assert!(!header_agrees_with_name(&ny, gap, utc(2027, 3, 14, 6, 30)), "read as EST");
        assert!(!header_agrees_with_name(&ny, gap, utc(2027, 3, 14, 7, 30)), "read as EDT");

        // A copy from a PC one hour west (CDT, UTC-5): its name is Chicago time.
        assert!(!header_agrees_with_name(&ny, name(2026, 10, 17, 12, 0), utc(2026, 10, 17, 17, 0)));
        assert!(header_agrees_with_name(&ny, name(2026, 10, 17, 12, 0), utc(2026, 10, 17, 16, 0)), "same offset");
    }

    /// One batch of a gated journal: a line 200 s old (inside EDDN's own 300 s age guard, so only
    /// our rule keeps it out) followed by a fresh one. Only the fresh one is live.
    #[test]
    fn a_gated_batch_with_a_stale_line_then_a_fresh_one_makes_only_the_fresh_one_live() {
        let d = tempfile::tempdir().unwrap();
        let foreign = d.path().join(game_name(180));
        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        let own = d.path().join(game_name(-30));
        std::fs::write(&own, session(&ts(1800), "r1 ")).unwrap();
        touch(&own, 10);
        t.on_journal_activity();
        let base = eddn_state(&db).0;
        append(&own, &docked(&ts(200)));
        append(&own, &docked(&ts(0)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, base + 1, "the 200 s old line is a replay, the fresh one is live");
    }

    /// The startup replay is silent, even for a journal that is only the game's by its header and
    /// holds lines stamped now.
    #[test]
    fn the_startup_replay_of_a_gated_journal_with_fresh_lines_is_silent() {
        let d = tempfile::tempdir().unwrap();
        let (own, foreign) = (d.path().join(game_name(-30)), d.path().join(game_name(180)));
        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        std::fs::write(&own, format!("{}{}\n", session(&ts(1800), "r1 "), docked(&ts(0)))).unwrap();
        touch(&own, 10);
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        assert_eq!(t.tail.as_ref().unwrap().path, own, "startup follows the newest by modified time");
        assert_eq!(eddn_state(&db), (0, vec![]), "nothing from the startup replay");
        append(&own, &docked(&ts(0)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1, "but what it writes next is live");
    }

    /// astromech-782v: the game starts twice in the repeated hour after the clocks go back (US,
    /// 1 Nov 2026). The second journal's name (01:10) sorts before the first's (01:30), but it was
    /// written later and its header (06:10Z) is its name read as EST: its new lines are live. A
    /// touched older journal from before the switch (00:30 EDT) stays a replay.
    #[test]
    fn a_journal_started_in_the_repeated_hour_after_the_clocks_go_back_is_live() {
        let d = tempfile::tempdir().unwrap();
        let first = d.path().join("Journal.2026-11-01T013000.01.log");
        let second = d.path().join("Journal.2026-11-01T011000.01.log");
        let older = d.path().join("Journal.2026-11-01T003000.01.log");
        // Header (UTC) of each: 01:30 EDT, 01:10 EST (the second 01:00-02:00), 00:30 EDT.
        std::fs::write(&first, session("2026-11-01T05:30:00Z", "r1 ")).unwrap();
        std::fs::write(&older, session("2026-11-01T04:30:00Z", "r0 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.header_check = Box::new(|name, header| header_agrees_with_name(&chrono_tz::America::New_York, name, header));
        touch(&first, 5);
        t.catch_up();
        assert_eq!(t.tail.as_ref().unwrap().path, first);

        // The game restarts after the clocks went back.
        std::fs::write(&second, "").unwrap();
        touch(&second, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, second);
        append(&second, &format!("{}{}", session("2026-11-01T06:10:00Z", "r2 "), docked(&ts(0))));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1, "the new session's fresh line is live though its name sorts first");

        // A tool touches the older journal and writes old lines into it: a replay.
        std::fs::write(&older, session("2026-11-01T04:30:00Z", "r0 ")).unwrap();
        touch(&older, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, older);
        append(&older, &docked("2026-11-01T04:40:00Z"));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1, "nothing live from the older journal");
    }

    /// The same two journals at startup: the app starts while the game runs in the one named
    /// 01:10 (written after 01:30's), so it follows that one by modified time. Its fresh lines are
    /// live, though it is not the latest by name.
    #[test]
    fn the_app_starting_in_the_repeated_hour_follows_the_games_journal_live() {
        let d = tempfile::tempdir().unwrap();
        let first = d.path().join("Journal.2026-11-01T013000.01.log");
        let second = d.path().join("Journal.2026-11-01T011000.01.log");
        std::fs::write(&first, session("2026-11-01T05:30:00Z", "r1 ")).unwrap();
        std::fs::write(&second, session("2026-11-01T06:10:00Z", "r2 ")).unwrap();
        touch(&second, 10);
        let (mut t, db) = eddn_tailer(d.path());
        t.header_check = Box::new(|name, header| header_agrees_with_name(&chrono_tz::America::New_York, name, header));
        t.catch_up();
        assert_eq!(t.tail.as_ref().unwrap().path, second);
        assert_eq!(eddn_state(&db), (0, vec![]), "the startup replay is silent");
        append(&second, &docked(&ts(0)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1);
    }

    /// "Market." on every fresh `Market.json` (the engine decides once per dock).
    struct MarketRule;
    impl astromech_announce::Rule for MarketRule {
        fn id(&self) -> &'static str {
            "market"
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
            matches!(input, Input::Market(_)).then(|| Callout {
                text: "Market.".into(),
                priority: Priority::Normal,
                channels: Channels::SPEECH,
            })
        }
    }

    /// astromech-bh0c: a reopened journal is split into a replayed half and a live half. A Docked
    /// at the same market in each: the replayed one reads Market.json with live=false and must not
    /// use up the dock's one callout, so the live one reaches the callouts.
    #[test]
    fn a_dock_in_the_live_half_of_a_reopened_journal_still_gets_its_market_callout() {
        let d = tempfile::tempdir().unwrap();
        let (j1, j2) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        std::fs::write(&j1, session(&ts(30), "r1 ")).unwrap();
        std::fs::write(&j2, format!("{}{}\n{}\n", session(&ts(10), "r2 "), docked(&ts(5)), market_event(&ts(5), 7)))
            .unwrap();
        std::fs::write(d.path().join("Market.json"), market_file(&ts(5), 7)).unwrap();
        touch(&j2, 5);
        let (mut t, _, _) = tailer(d.path());
        let fired = Arc::new(std::sync::Mutex::new(vec![]));
        let f2 = fired.clone();
        t.callouts = Some(Callouts {
            engine: astromech_announce::Engine::new(vec![Box::new(MarketRule)], &Default::default()),
            deliver: Box::new(move |rs| f2.lock().unwrap().extend(rs.into_iter().map(|r| r.text))),
        });
        t.catch_up();
        assert!(fired.lock().unwrap().is_empty(), "the startup replay is silent");

        // A touched older journal is followed (a replay); meanwhile the game docks again at the
        // same market and writes a new Market.json.
        touch(&j1, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j1);
        std::fs::write(d.path().join("Market.json"), market_file(&ts(0), 7)).unwrap();
        append(&j2, &docked(&ts(0)));
        append(&j2, &market_event(&ts(0), 7));
        touch(&j2, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, j2);
        assert_eq!(*fired.lock().unwrap(), ["Market."], "the live dock's market callout");
    }

    /// astromech-01c0: a Market event whose file wasn't there yet stays pending, and is retried
    /// as it was read. One from the startup replay is retried after the live half has begun and
    /// must stay silent (an old dock); one from a live line still speaks.
    #[test]
    fn a_pending_market_event_is_retried_as_live_only_if_its_line_was() {
        let d = tempfile::tempdir().unwrap();
        let j = d.path().join(game_name(0));
        std::fs::write(&j, format!("{}{}\n{}\n", session(&ts(60), "r1 "), docked(&ts(30)), market_event(&ts(30), 7)))
            .unwrap();
        let (mut t, _, _) = tailer(d.path());
        let fired = Arc::new(std::sync::Mutex::new(vec![]));
        let f2 = fired.clone();
        t.callouts = Some(Callouts {
            engine: astromech_announce::Engine::new(vec![Box::new(MarketRule)], &Default::default()),
            deliver: Box::new(move |rs| f2.lock().unwrap().extend(rs.into_iter().map(|r| r.text))),
        });
        t.catch_up();
        assert_eq!(t.pending.len(), 1, "no Market.json yet for the replayed dock");

        // The live half begins, then the replayed dock's file turns up.
        append(&j, &format!(r#"{{"timestamp":"{}","event":"Music","MusicTrack":"Docking"}}"#, ts(0)));
        t.on_journal_activity();
        assert!(t.live);
        std::fs::write(d.path().join("Market.json"), market_file(&ts(30), 7)).unwrap();
        t.on_companion_files(&[CompanionKind::Market]);
        assert_eq!(t.hub.snapshot().companions.market.unwrap().market_id, Some(7), "the file is used");
        assert!(fired.lock().unwrap().is_empty(), "the replayed dock stays silent: {:?}", fired.lock().unwrap());

        // A live dock whose file comes late still gets its callout, from the idle retry.
        std::fs::remove_file(d.path().join("Market.json")).unwrap();
        append(&j, &docked(&ts(0)).replace("\"MarketID\":7", "\"MarketID\":8"));
        append(&j, &market_event(&ts(0), 8));
        t.on_journal_activity();
        assert_eq!(t.pending.len(), 1);
        std::fs::write(d.path().join("Market.json"), market_file(&ts(0), 8)).unwrap();
        t.retry_pending();
        assert_eq!(*fired.lock().unwrap(), ["Market."]);
    }

    /// astromech-r3s7: once the game has started a newer session, the earlier journal is no longer
    /// the game's, though it was and though nothing changed in the folder's names: a tool that
    /// appends to it and touches it is a replay (EDDN's own 300 s age guard is why the line is
    /// 200 s old: only our rule keeps it out).
    #[test]
    fn an_earlier_session_is_not_live_once_a_newer_one_has_started() {
        let d = tempfile::tempdir().unwrap();
        let (a, b) =
            (d.path().join("Journal.2026-10-16T110000.01.log"), d.path().join("Journal.2026-10-16T120000.01.log"));
        std::fs::write(&a, session(&ts(60), "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();
        append(&a, &docked(&ts(0)));
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, 1, "the game's own line is live");

        // The game starts its next session.
        std::fs::write(&b, session(&ts(5), "r2 ")).unwrap();
        touch(&b, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, b);
        let sent = eddn_state(&db).0;

        // A tool appends an older line to the first session's journal and touches it.
        append(&a, &docked(&ts(200)));
        touch(&a, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, a);
        assert_eq!(eddn_state(&db).0, sent, "nothing live from a session the game has left");

        // The game's current journal is live again when it is written.
        append(&b, &docked(&ts(0)));
        touch(&b, 30);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, sent + 1);
    }

    /// astromech-r3s7, the repeated hour after a fall-back: the game's second session (01:10) sorts
    /// before the first (01:30), which is therefore still the latest by name. Once the second is
    /// the game's, a tool appending to the first is a replay, though it is the latest by name.
    #[test]
    fn a_touched_first_journal_of_the_repeated_hour_is_not_live_once_the_second_is() {
        let d = tempfile::tempdir().unwrap();
        let first = d.path().join("Journal.2026-11-01T013000.01.log");
        let second = d.path().join("Journal.2026-11-01T011000.01.log");
        std::fs::write(&first, session("2026-11-01T05:30:00Z", "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.header_check = Box::new(|name, header| header_agrees_with_name(&chrono_tz::America::New_York, name, header));
        touch(&first, 5);
        t.catch_up();

        std::fs::write(&second, "").unwrap();
        touch(&second, 10);
        t.on_journal_activity();
        append(&second, &format!("{}{}", session("2026-11-01T06:10:00Z", "r2 "), docked(&ts(0))));
        t.on_journal_activity();
        let sent = eddn_state(&db).0;
        assert_eq!(sent, 1, "the second session is live");

        append(&first, &docked(&ts(200)));
        touch(&first, 20);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, first);
        assert_eq!(eddn_state(&db).0, sent, "the first journal is the latest by name but not the game's now");

        append(&second, &docked(&ts(0)));
        touch(&second, 30);
        t.on_journal_activity();
        assert_eq!(eddn_state(&db).0, sent + 1, "the game's journal is still live");
    }

    /// A foreign journal stamped later and touched takes the session while the game is quiet, but
    /// the game's own next fresh line takes it back, and the foreign one is then a replay however
    /// often it is touched again.
    #[test]
    fn the_games_journal_takes_the_session_back_from_a_touched_foreign_one() {
        let d = tempfile::tempdir().unwrap();
        let (own, foreign) = (d.path().join(game_name(0)), d.path().join(game_name(180)));
        std::fs::write(&own, session(&ts(10), "r1 ")).unwrap();
        let (mut t, db) = eddn_tailer(d.path());
        t.catch_up();

        std::fs::write(&foreign, session(&ts(20000), "r9 ")).unwrap();
        touch(&foreign, 10);
        t.on_journal_activity();
        assert_eq!(t.tail.as_ref().unwrap().path, foreign);

        append(&own, &docked(&ts(0)));
        touch(&own, 20);
        t.on_journal_activity();
        let sent = eddn_state(&db).0;
        assert_eq!(t.tail.as_ref().unwrap().path, own);
        assert!(sent >= 1, "the game's fresh line is live");

        for n in 1..=2u64 {
            let before = eddn_state(&db).0;
            append(&foreign, &docked(&ts(200)));
            touch(&foreign, 20 + 10 * n);
            t.on_journal_activity();
            assert_eq!(t.tail.as_ref().unwrap().path, foreign);
            assert_eq!(eddn_state(&db).0, before, "touch {n}: the foreign journal is a replay now");
            append(&own, &docked(&ts(0)));
            touch(&own, 25 + 10 * n);
            t.on_journal_activity();
            assert_eq!(eddn_state(&db).0, before + 1, "the game's journal is live again");
        }
    }
}
