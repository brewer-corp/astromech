//! EDSM journal upload: the commander's own journal events go to EDSM's Journal API v1, so
//! their EDSM flight log, map and profile keep up without EDMC. Written from EDSM's API docs
//! (<https://www.edsm.net/en/api-journal-v1>, revision of 30 Nov 2022), not from EDMC.
//!
//! - Each event is sent as the game wrote it ("without any modification"), plus the transient
//!   state fields the docs ask for ([`transient`]).
//! - Events on EDSM's discard list (`GET /api-journal-v1/discard`) are never queued once the
//!   list is known, and never sent: the courier fetches the list before its first post (and
//!   again daily, or when EDSM answers 304 "Discarded event") and drops listed events from
//!   every request it builds.
//! - Only the live game: the docs accept only the live version (`fromGameVersion` and
//!   `fromGameBuild` are required; legacy is answered 208), so beta and unknown versions stay
//!   here. Like EDDN, nothing from the startup replay is sent. Unlike EDDN, a live event isn't
//!   checked against its timestamp (see [`Uploader::journal`]).
//! - Only for the commander set in Settings, whose API key is in the OS keyring: the switch
//!   ([`Switch`]) holds the credentials while sharing is on. In another commander's crew
//!   nothing is sent (EDSM keeps no log there, answer 104).
//! - Events wait in an [`Outbox`] (`edsm.sqlite`) without the API key, which is added when the
//!   request is built; they go in batches (the docs allow "a list of message line"), oldest
//!   first, one game session (commander, version, build) per request. An event EDSM has refused
//!   before goes alone, after all the others that are due.

pub mod api;
pub mod transient;

use crate::message::Software;
use crate::outbox::{backoff, Delivery, Outbox, Outcome, Queued, MAX_ATTEMPTS, MIN_RETRY_SECS};
use crate::worker::{Pass, Worker};
use api::Api;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use transient::Transient;

/// The queue and send log in the data folder.
pub const DB_FILE: &str = "edsm.sqlite";
/// The keyring entry for the API key.
pub const KEY_SECRET: &str = "edsm-api-key";
/// Events per request at most.
pub const BATCH: usize = 50;
/// The discard list is fetched again after this long (seconds).
pub const DISCARD_MAX_AGE: i64 = 24 * 3600;
/// Outbox meta key: the newest send-log row id when the credentials last changed. The status
/// doesn't show errors logged up to it as the last error.
pub const ERRORS_CLEARED: &str = "errors_cleared_at";
/// The send-log label for rows about requests (several events) rather than one event.
pub const JOURNAL: &str = "journal";

const MIN_RETRY: Duration = Duration::from_secs(MIN_RETRY_SECS as u64);
/// This many events failing alone in a row, with none stored between, stop the pass.
const OUTAGE_AFTER: u32 = 3;
const IDLE: Duration = Duration::from_secs(300);

type Log = Arc<dyn Fn(&str) + Send + Sync>;

/// The commander name as registered on EDSM and their API key (EDSM: Settings > My API Key).
#[derive(Clone, PartialEq, Eq)]
pub struct Credentials {
    pub commander: String,
    pub api_key: String,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials").field("commander", &self.commander).finish_non_exhaustive()
    }
}

/// The user's EDSM switch: credentials while sharing is on, `None` while off (switched off, no
/// commander name, or no API key). Read on every event and before every request, so a change
/// takes effect at once. Changed credentials also wake the uploader and tell the courier to
/// retry what a wrong name or key held back, without waiting out the backoff.
#[derive(Default)]
pub struct Switch {
    creds: RwLock<Option<Credentials>>,
    /// Counts changes to the credentials (not turning off).
    generation: AtomicU64,
    waker: Mutex<Option<Box<dyn Fn() + Send + Sync>>>,
}

impl std::fmt::Debug for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Switch").field("creds", &self.creds).finish_non_exhaustive()
    }
}

impl Switch {
    /// Set the credentials (or turn sharing off). Returns whether sharing was on.
    pub fn set(&self, c: Option<Credentials>) -> bool {
        let new = c.is_some();
        let old = std::mem::replace(&mut *self.creds.write().unwrap(), c.clone());
        if new && old != c {
            self.generation.fetch_add(1, Ordering::SeqCst);
            if let Some(wake) = &*self.waker.lock().unwrap() {
                wake();
            }
        }
        old.is_some()
    }
    pub fn get(&self) -> Option<Credentials> {
        self.creds.read().unwrap().clone()
    }
    /// What to call when the credentials change (the uploader's [`Worker::waker`]).
    pub fn set_waker(&self, wake: impl Fn() + Send + Sync + 'static) {
        *self.waker.lock().unwrap() = Some(Box::new(wake));
    }
    fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }
}

/// EDSM's discard list, shared by the uploader (which stops queuing listed events) and the
/// courier (which fetches it and never sends a listed event).
#[derive(Debug, Default)]
pub struct Discards(RwLock<Option<(HashSet<String>, i64)>>);

impl Discards {
    /// Is `event` discarded? `None` until the list is known.
    pub fn contains(&self, event: &str) -> Option<bool> {
        self.0.read().unwrap().as_ref().map(|(s, _)| s.contains(event))
    }
    pub fn known(&self) -> bool {
        self.0.read().unwrap().is_some()
    }
    /// The list is known and younger than [`DISCARD_MAX_AGE`].
    fn fresh(&self, now: i64) -> bool {
        self.0.read().unwrap().as_ref().is_some_and(|(_, at)| now - at < DISCARD_MAX_AGE)
    }
    fn set(&self, names: Vec<String>, at: i64) {
        *self.0.write().unwrap() = Some((names.into_iter().collect(), at));
    }
    /// Fetch again before the next request (EDSM discarded an event the list didn't name).
    fn expire(&self) {
        if let Some((_, at)) = self.0.write().unwrap().as_mut() {
            *at = i64::MIN / 2;
        }
    }
    /// The list kept from an earlier run.
    fn load(&self, outbox: &Outbox) -> rusqlite::Result<()> {
        if let (Some(list), Some(at)) = (outbox.meta("discard")?, outbox.meta("discard_at")?) {
            if let (Ok(names), Ok(at)) = (serde_json::from_str(&list), at.parse()) {
                self.set(names, at);
            }
        }
        Ok(())
    }
}

/// A queued event: the session it belongs to and the event with its transient fields.
fn queued_body(t: &Transient, commander: &str, message: Value) -> Value {
    json!({
        "commander": commander,
        "gameversion": t.ctx.gameversion,
        "gamebuild": t.ctx.gamebuild,
        "message": message,
    })
}

/// What the core drives: every journal event, in order.
pub struct Uploader {
    state: Transient,
    outbox: Outbox,
    switch: Arc<Switch>,
    discards: Arc<Discards>,
    worker: Option<Worker>,
    /// Why the last live event wasn't queued, so each reason is logged once when it starts.
    held_back: Option<String>,
    log: Log,
}

impl Uploader {
    /// An uploader that only queues (tests, or delivery run elsewhere).
    pub fn new(outbox: Outbox, switch: Arc<Switch>, discards: Arc<Discards>) -> Self {
        Self {
            state: Transient::default(),
            outbox,
            switch,
            discards,
            worker: None,
            held_back: None,
            log: Arc::new(|_| {}),
        }
    }

    /// Open the queue at `db` twice (one connection queues, the worker's delivers) and start
    /// delivering through `api`.
    pub fn start(
        db: &Path,
        software: Software,
        api: Box<dyn Api>,
        switch: Arc<Switch>,
        log: impl Fn(&str) + Send + Sync + 'static,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let log: Log = Arc::new(log);
        let outbox = Outbox::open(db)?.with_label("edsm");
        let discards = Arc::new(Discards::default());
        discards.load(&outbox)?;
        let courier = Courier::new(
            Outbox::open(db)?.with_label("edsm"),
            api,
            software,
            switch.clone(),
            discards.clone(),
            Instant::now(),
        );
        let worker_log = log.clone();
        let worker = Worker::spawn(courier, "EDSM", move |m| worker_log(m))?;
        switch.set_waker(worker.waker());
        Ok(Self { worker: Some(worker), log, ..Self::new(outbox, switch, discards) })
    }

    /// One journal event (raw JSON), in order. `live` is false while replaying a journal that
    /// was written before astromech started; `now` is Unix seconds.
    ///
    /// No age guard on live events, unlike EDDN's feed (`feed::MAX_EVENT_AGE_SECS`, astromech-st8o).
    /// EDDN's consumers take a message as what the game shows now, so an old line re-read as live
    /// would publish stale prices and positions. EDSM keeps the commander's own history: an old
    /// event is still true, EDSM takes past events by design (it imports old journal files), and
    /// one it already has is answered 101-103 and counted as sent. A re-read journal costs
    /// requests, not wrong data. The core already reads a journal written before it started (or
    /// an older one touched later) as a replay, which is never sent.
    pub fn journal(&mut self, ev: &Value, live: bool, now: i64) {
        let entry = self.state.apply(ev);
        let Some(creds) = self.switch.get().filter(|_| live) else { return };
        let name = ev.get("event").and_then(Value::as_str).unwrap_or_default();
        if self.discards.contains(name) == Some(true) {
            return;
        }
        let held = match (&self.state.ctx.commander, &self.state.ctx.gameversion) {
            (None, _) => Some("no commander yet (no LoadGame)".to_owned()),
            (Some(c), _) if !c.eq_ignore_ascii_case(&creds.commander) => {
                Some(format!("playing as {c}, not {} (Settings → Data sharing)", creds.commander))
            }
            (_, None) => Some("no game version yet (no Fileheader)".to_owned()),
            _ if self.state.ctx.is_beta() => Some("beta or legacy game: EDSM takes only the live game".to_owned()),
            _ if self.state.crew_member() => Some("in another commander's crew".to_owned()),
            _ => None,
        };
        if held != self.held_back {
            if let Some(why) = &held {
                if let Err(e) = self.outbox.log(now, name, Outcome::Dropped, &format!("not sent: {why}")) {
                    (self.log)(&format!("EDSM outbox: {e}"));
                }
            }
            self.held_back = held.clone();
        }
        if held.is_some() {
            return;
        }
        let cmdr = self.state.ctx.commander.clone().unwrap_or_default();
        let body = queued_body(&self.state, &cmdr, entry);
        match self.outbox.enqueue(name, body.to_string().as_bytes(), now) {
            Ok(()) => {
                if let Some(w) = &self.worker {
                    w.wake();
                }
            }
            Err(e) => (self.log)(&format!("EDSM outbox: {e}")),
        }
    }

    pub fn outbox(&self) -> &Outbox {
        &self.outbox
    }
}

/// EDSM's answer to one request.
#[derive(Debug, PartialEq)]
enum Answer {
    /// One code and message per event, in order.
    Each(Vec<(i64, String)>),
    /// Nothing was stored, but the events may be later: the site is unreachable or busy (a
    /// transport error, 408, 429), or the commander name or API key is wrong (fixed in
    /// Settings). Every event waits, however long it takes.
    Hold(String),
    /// Nothing was stored, and sending the same again won't change that.
    Reject(String),
    /// The request failed without an answer per event: an HTTP status such as 413 or 5xx, a
    /// request-level 5xx code, an answer that isn't readable, or one that doesn't cover every
    /// event. A batch is split in halves on the next tries, down to single events; an event
    /// that fails alone the same way every time is given up after [`MAX_ATTEMPTS`] instead of
    /// blocking the queue.
    Refused(String),
}

/// Read EDSM's answer to a request of `n` events (codes from the Journal API v1 docs).
fn answer(r: Result<crate::transport::Response, String>, n: usize) -> Answer {
    let r = match r {
        Ok(r) => r,
        Err(e) => return Answer::Hold(e),
    };
    let v = match api::json_answer(&r) {
        Ok(v) => v,
        // Too many requests or timed out: wait. Any other status (5xx included, which one bad
        // event can cause), or a body that isn't JSON: refused.
        Err(e) if r.status == 408 || r.status == 429 => return Answer::Hold(e),
        Err(e) => return Answer::Refused(e),
    };
    match api::code(&v) {
        (100, _) => {
            let each: Vec<(i64, String)> =
                v["events"].as_array().map(Vec::as_slice).unwrap_or_default().iter().map(api::code).collect();
            if each.len() == n {
                Answer::Each(each)
            } else {
                Answer::Refused(format!("answer for {} of {n} events", each.len()))
            }
        }
        // 201-203: commander name or API key missing or unknown.
        (c @ 201..=203, msg) => Answer::Hold(format!("{c} {msg}")),
        // EDSM's own failure with the request.
        (c @ 500.., msg) => Answer::Refused(format!("{c} {msg}")),
        (c, msg) => Answer::Reject(format!("{c} {msg}")),
    }
}

/// Delivery: one pass at a time on the worker thread.
pub struct Courier {
    outbox: Outbox,
    api: Box<dyn Api>,
    software: Software,
    switch: Arc<Switch>,
    discards: Arc<Discards>,
    /// No retry before this instant: a minute after the last failure (or the start).
    hold_until: Instant,
    stop: Arc<AtomicBool>,
    /// The switch's generation when the queue was last released.
    seen: u64,
    /// Events per request at most: halved after a failed request, doubled after each pass of
    /// a sent one, back to [`BATCH`].
    cap: usize,
}

impl Courier {
    pub fn new(
        outbox: Outbox,
        api: Box<dyn Api>,
        software: Software,
        switch: Arc<Switch>,
        discards: Arc<Discards>,
        started: Instant,
    ) -> Self {
        let seen = switch.generation();
        Self {
            outbox,
            api,
            software,
            switch,
            discards,
            hold_until: started + MIN_RETRY,
            stop: Arc::default(),
            seen,
            cap: BATCH,
        }
    }

    pub fn outbox(&self) -> &Outbox {
        &self.outbox
    }

    /// Fetch the discard list if it's missing or old. False if there is none to go by.
    fn discard_list(&mut self, wall: i64) -> rusqlite::Result<bool> {
        if self.discards.fresh(wall) {
            return Ok(true);
        }
        match api::discard_list(self.api.as_ref()) {
            Ok(names) => {
                self.outbox.set_meta("discard", &serde_json::to_string(&names).unwrap_or_default())?;
                self.outbox.set_meta("discard_at", &wall.to_string())?;
                self.discards.set(names, wall);
                Ok(true)
            }
            Err(e) => {
                let known = self.discards.known();
                if !known {
                    self.outbox.log(wall, "discard", Outcome::Retry, &format!("discard list: {e}"))?;
                }
                Ok(known)
            }
        }
    }

    /// Drop queued events that can't go with `creds`: unreadable, another commander's, or
    /// on the discard list. Returns the rest.
    fn sendable(&self, due: Vec<Queued>, creds: &Credentials, wall: i64) -> rusqlite::Result<Vec<(Queued, Value)>> {
        let mut out = vec![];
        for q in due {
            let body: Value = serde_json::from_slice(&q.body).unwrap_or(Value::Null);
            let cmdr = body["commander"].as_str().unwrap_or_default();
            if !cmdr.eq_ignore_ascii_case(&creds.commander) {
                self.outbox.take(q.id)?;
                let why = format!("queued for {cmdr:?}, but Settings has {}", creds.commander);
                self.outbox.log(wall, &q.schema_ref, Outcome::Discarded, &why)?;
            } else if self.discards.contains(&q.schema_ref) == Some(true) {
                self.outbox.take(q.id)?;
            } else {
                out.push((q, body));
            }
        }
        Ok(out)
    }

    /// The request for one session's events (the docs' parameters; the API key only here).
    fn request(&self, creds: &Credentials, batch: &[(Queued, Value)]) -> Value {
        let first = &batch[0].1;
        json!({
            "commanderName": creds.commander,
            "apiKey": creds.api_key,
            "fromSoftware": self.software.name,
            "fromSoftwareVersion": self.software.version,
            "fromGameVersion": first["gameversion"],
            "fromGameBuild": first["gamebuild"],
            "message": batch.iter().map(|(_, b)| b["message"].clone()).collect::<Vec<_>>(),
        })
    }

    fn deliver(&mut self, creds: &Credentials, wall: i64, mono: Instant, d: &mut Delivery) -> rusqlite::Result<()> {
        let retries = mono >= self.hold_until;
        // Events that failed alone in a row in this pass, with nothing stored in between.
        let mut failed_alone = 0;
        loop {
            if self.stop.load(Ordering::SeqCst) || self.switch.get().as_ref() != Some(creds) {
                return Ok(());
            }
            let due = self.outbox.due(wall, retries, self.cap)?;
            if due.is_empty() {
                return Ok(());
            }
            let mut rows = self.sendable(due, creds, wall)?;
            if rows.is_empty() {
                continue;
            }
            // One game session per request: fromGameVersion and fromGameBuild are per request.
            let session = |b: &Value| (b["gameversion"].clone(), b["gamebuild"].clone());
            let first = session(&rows[0].1);
            let n = rows.iter().take_while(|(_, b)| session(b) == first).count();
            // An event that already failed alone is known to be refused: it goes by itself, never
            // back through 50, 25, 13, ... with the good events beside it.
            let n = match rows[..n].iter().position(|(q, _)| q.refusals > 0) {
                Some(0) => 1,
                Some(i) => i,
                None => n,
            };
            rows.truncate(n);
            let body = serde_json::to_vec(&self.request(creds, &rows)).expect("JSON");
            match answer(self.api.post_json(api::JOURNAL_URL, &body), n) {
                Answer::Each(codes) => {
                    let stored_before = d.sent;
                    self.each(&rows, codes, wall, d)?;
                    self.cap = (self.cap * 2).min(BATCH);
                    if d.sent > stored_before {
                        failed_alone = 0;
                    }
                }
                Answer::Reject(why) => {
                    for (q, _) in &rows {
                        self.outbox.take(q.id)?;
                    }
                    self.outbox.log(wall, JOURNAL, Outcome::Rejected, &format!("{n} events: {why}"))?;
                    d.rejected += n;
                }
                Answer::Refused(why) if n > 1 => {
                    // Nothing says which event EDSM chokes on: try the halves, and count no try
                    // against any event until one is alone.
                    self.cap = n.div_ceil(2);
                    self.outbox.hold_due(wall, wall + MIN_RETRY_SECS)?;
                    self.outbox.log(
                        wall,
                        JOURNAL,
                        Outcome::Retry,
                        &format!("{why}; {n} events, halves in {MIN_RETRY_SECS} s"),
                    )?;
                    d.retrying += n;
                    d.stopped = true;
                    self.hold_until = mono + MIN_RETRY;
                    return Ok(());
                }
                Answer::Refused(why) => {
                    // One event fails alone. Only refusals count here: tries lost to outages or
                    // a wrong key don't.
                    let q = &rows[0].0;
                    if q.refusals + 1 >= MAX_ATTEMPTS {
                        self.outbox.take(q.id)?;
                        let why = format!("{} refused {MAX_ATTEMPTS} times; last: {why}", q.schema_ref);
                        self.outbox.log(wall, JOURNAL, Outcome::GaveUp, &why)?;
                        d.gave_up += 1;
                        continue;
                    }
                    let w = backoff(q.refusals + 1);
                    self.outbox.refuse_at(q.id, q.attempts + 1, q.refusals + 1, wall + w)?;
                    self.outbox.log(wall, JOURNAL, Outcome::Retry, &format!("{why}; retry in {w} s"))?;
                    d.retrying += 1;
                    // The queue behind it moves on. Several events failing alone in a row, with
                    // nothing stored in between, look like an outage, not a few bad events: wait,
                    // as long as the last one's back-off. Events refused before come after every
                    // other due event (`Outbox::due`), so however many poisoned events stop a
                    // pass, the good ones due with them have been tried first; only other
                    // refused events wait. In an outage the never-refused events fail first and
                    // stop the pass, so refused ones are tried (and use up refusals) only once
                    // every other queued event has failed too.
                    failed_alone += 1;
                    if failed_alone >= OUTAGE_AFTER {
                        self.outbox.hold_due(wall, wall + w)?;
                        d.stopped = true;
                        self.hold_until = mono + MIN_RETRY;
                        return Ok(());
                    }
                }
                Answer::Hold(why) => {
                    let attempts = rows[0].0.attempts + 1;
                    let wait = backoff(attempts);
                    for (q, _) in &rows {
                        self.outbox.retry_at(q.id, q.attempts + 1, wall + wait)?;
                    }
                    self.outbox.hold_due(wall, wall + wait)?;
                    self.outbox.log(wall, JOURNAL, Outcome::Retry, &format!("{why}; retry in {wait} s"))?;
                    d.retrying += n;
                    d.stopped = true;
                    self.hold_until = mono + MIN_RETRY;
                    return Ok(());
                }
            }
        }
    }

    /// Settle each event of a request by its own answer code.
    fn each(
        &mut self,
        rows: &[(Queued, Value)],
        codes: Vec<(i64, String)>,
        wall: i64,
        d: &mut Delivery,
    ) -> rusqlite::Result<()> {
        let mut sent = 0;
        for ((q, _), (c, msg)) in rows.iter().zip(codes) {
            let event = q.schema_ref.as_str();
            match c {
                // Stored, or already stored (101-103), or a crew session EDSM doesn't log (104).
                100..=199 => {
                    self.outbox.take(q.id)?;
                    sent += 1;
                }
                // Discarded: the list we have is out of date.
                304 => {
                    self.outbox.take(q.id)?;
                    self.discards.expire();
                    self.outbox.log(wall, event, Outcome::Dropped, &format!("{c} {msg}"))?;
                }
                // EDSM's own failure with this event: try it again later.
                500.. if q.attempts + 1 < MAX_ATTEMPTS => {
                    let wait = backoff(q.attempts + 1);
                    self.outbox.retry_at(q.id, q.attempts + 1, wall + wait)?;
                    self.outbox.log(wall, event, Outcome::Retry, &format!("{c} {msg}; retry in {wait} s"))?;
                    d.retrying += 1;
                }
                500.. => {
                    self.outbox.take(q.id)?;
                    self.outbox.log(
                        wall,
                        event,
                        Outcome::GaveUp,
                        &format!("after {} tries; last: {c} {msg}", q.attempts + 1),
                    )?;
                    d.gave_up += 1;
                }
                _ => {
                    self.outbox.take(q.id)?;
                    self.outbox.log(wall, event, Outcome::Rejected, &format!("{c} {msg}"))?;
                    d.rejected += 1;
                }
            }
        }
        if sent > 0 {
            self.outbox.log(wall, JOURNAL, Outcome::Sent, &format!("{sent} events"))?;
            d.sent += sent;
        }
        Ok(())
    }
}

impl Pass for Courier {
    /// Switched off, it posts nothing and discards the queue instead.
    fn pass(&mut self, wall: i64, mono: Instant) -> rusqlite::Result<Delivery> {
        let Some(creds) = self.switch.get() else {
            let n = self.outbox.discard_all(wall)?;
            return Ok(Delivery { gave_up: n, ..Default::default() });
        };
        let mut d = Delivery::default();
        // The name or key was changed in Settings: what a wrong one held back goes now, and the
        // error it caused is no longer news.
        let generation = self.switch.generation();
        if generation != self.seen {
            self.seen = generation;
            self.outbox.make_due(wall)?;
            self.outbox.set_meta(ERRORS_CLEARED, &self.outbox.last_log_id()?.to_string())?;
            self.hold_until = mono;
        }
        if self.outbox.pending()? == 0 {
            return Ok(d);
        }
        if mono < self.hold_until && self.outbox.due(wall, false, 1)?.is_empty() {
            return Ok(d);
        }
        if !self.discard_list(wall)? {
            // No list to go by: nothing is sent until there is one.
            self.hold_until = mono + MIN_RETRY;
            d.stopped = true;
            return Ok(d);
        }
        self.deliver(&creds, wall, mono, &mut d)?;
        Ok(d)
    }

    fn wait(&self, wall: i64, mono: Instant) -> Duration {
        match self.outbox.next_due() {
            Ok(Some(due)) => {
                let by_wall = Duration::from_secs(due.saturating_sub(wall).max(0) as u64);
                by_wall.max(self.hold_until.saturating_duration_since(mono)).clamp(Duration::from_secs(1), IDLE)
            }
            Ok(None) => IDLE,
            Err(_) => MIN_RETRY,
        }
    }

    fn stop_flag(&self) -> Arc<AtomicBool> {
        self.stop.clone()
    }
}

#[cfg(test)]
mod tests;
