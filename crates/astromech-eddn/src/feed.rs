//! What the core drives: every journal event in order, `Status.json`, and the companion files
//! the events announce. The feed works out which schemas each one goes to, queues the messages,
//! writes every skip to the send log, and wakes the delivery [`Worker`].
//!
//! Only live data is sent. The core replays the current journal from its `Fileheader` at
//! startup to rebuild state; those events are fed with `live: false`, so the header, flags,
//! commander and last system are right, but nothing from them is queued. EDDN's consumers
//! take a message as what the game shows now, and a catch-up of an hour-old journal (or one
//! from the last session) would publish stale prices and positions as current. A journal the
//! game starts while astromech runs is live from its first line.
//!
//! Companion files (`Market.json`, `NavRoute.json`, ...) are sent only for a live event: the
//! event is remembered until the core hands over its file.
//!
//! A second guard: an event fed as live whose `timestamp` is more than [`MAX_EVENT_AGE_SECS`]
//! old is treated as replayed. The game and astromech share the PC's clock, so a live line is
//! seconds old; an old one means a journal is being re-read (a backup tool touched it, or
//! another client writes the same folder).

use crate::companion::{self, game_timestamp, Unusable};
use crate::events::{self, Built, EventSchemas, Skipped};
use crate::journal;
use crate::message::{SchemaSpec, Software};
use crate::outbox::Outbox;
use crate::transport::Transport;
use crate::worker::{Courier, Worker};
use crate::Sender;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

type Log = Arc<dyn Fn(&str) + Send + Sync>;

/// A live event older than this (by its `timestamp`, against the wall clock) isn't sent.
pub const MAX_EVENT_AGE_SECS: i64 = 5 * 60;

/// Unix seconds for the game's `YYYY-MM-DDTHH:MM:SSZ` timestamp.
fn unix_time(v: &Value) -> Option<i64> {
    let t = game_timestamp(v)?;
    let n = |r: std::ops::Range<usize>| t[r].parse::<i64>().ok();
    let (y, m, d) = (n(0..4)?, n(5..7)?, n(8..10)?);
    let secs = n(11..13)? * 3600 + n(14..16)? * 60 + n(17..19)?;
    // Days from 1970-01-01 to y-m-d (proleptic Gregorian; Howard Hinnant's days_from_civil).
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some((era * 146_097 + doe - 719_468) * 86_400 + secs)
}

/// The journal events whose data is in a companion file, and the schema each feeds.
fn companion_spec(event: &str) -> Option<&'static SchemaSpec> {
    Some(match event {
        "Market" => &companion::COMMODITY,
        "Outfitting" => &companion::OUTFITTING,
        "Shipyard" => &companion::SHIPYARD,
        "NavRoute" => &events::NAV_ROUTE,
        "FCMaterials" => &events::FC_MATERIALS_JOURNAL,
        _ => return None,
    })
}

pub struct Feed {
    sender: Sender,
    events: EventSchemas,
    /// Live companion events waiting for their file, by event name, with when each arrived
    /// (Unix seconds).
    awaiting: HashMap<String, (Value, i64)>,
    /// The last event was live (so a run of signals pending from the replay has been dropped).
    live: bool,
    /// Lines fed as live are arriving too old to send (logged once when this starts and ends).
    stale: bool,
    /// The user's switch. Off: events still keep the context current, but nothing is queued.
    sharing: Arc<AtomicBool>,
    worker: Option<Worker>,
    /// The app log, for what can't go in the send log (the outbox itself failing).
    log: Log,
}

impl Feed {
    /// A feed that only queues (tests, or delivery run elsewhere).
    pub fn new(sender: Sender) -> Self {
        Self {
            sender,
            events: EventSchemas::default(),
            awaiting: HashMap::new(),
            live: false,
            stale: false,
            sharing: Arc::new(AtomicBool::new(true)),
            worker: None,
            log: Arc::new(|_: &str| {}),
        }
    }

    /// Open the outbox at `db` twice (one connection queues, the worker's delivers) and start
    /// delivering over `transport`. `test` sends to the `/test` schemas. `sharing` is the user's
    /// switch, read on every event and before every post.
    pub fn start(
        db: &Path,
        software: Software,
        test: bool,
        transport: Box<dyn Transport + Send>,
        sharing: Arc<AtomicBool>,
        log: impl Fn(&str) + Send + Sync + 'static,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let log: Log = Arc::new(log);
        let sender = Sender::new(Outbox::open(db)?, software, test);
        let courier = Courier::new(Outbox::open(db)?, transport, Instant::now(), sharing.clone());
        let worker_log = log.clone();
        let worker = Worker::spawn(courier, "EDDN", move |m| worker_log(m))?;
        Ok(Self { worker: Some(worker), log, sharing, ..Self::new(sender) })
    }

    /// One journal event (raw JSON), in order. `live` is false while replaying a journal that
    /// was written before astromech started; `now` is Unix seconds.
    pub fn journal(&mut self, ev: &Value, live: bool, now: i64) {
        // Observe before routing: an Odyssey client writes a system's signals before the
        // FSDJump that arrives there, and the batch must be checked against the new system.
        self.sender.observe(ev);
        let name = ev.get("event").and_then(Value::as_str).unwrap_or_default();
        // Switched off, live lines are handled like a replay: nothing queued or logged.
        let live = live && self.sharing.load(Ordering::SeqCst);
        let age = unix_time(ev).map(|t| now - t);
        let stale = live && age.is_some_and(|a| a > MAX_EVENT_AGE_SECS);
        if live && stale != self.stale {
            // Once per change, so a clock that disagrees with the game leaves a trace without
            // a line per event.
            self.stale = stale;
            (self.log)(&if stale {
                format!("EDDN: live journal line {} min old, treating as replay", age.unwrap_or(0) / 60)
            } else {
                "EDDN: live journal lines are current again, sending".to_owned()
            });
        }
        if !live || stale {
            // Keep the schemas' own state (the body for codexentry) but send nothing.
            let _ = self.events.on_event(ev, &self.sender.ctx);
            self.awaiting.remove(name);
            self.live = false;
            return;
        }
        if !self.live {
            // A signal run that began in the replay isn't sent.
            let _ = self.events.flush(&self.sender.ctx);
            self.live = true;
        }
        let mut built = self.events.on_event(ev, &self.sender.ctx);
        built.extend(journal::message(ev));
        if companion_spec(name).is_some() {
            self.awaiting.insert(name.to_owned(), (ev.clone(), now));
        }
        self.queue(built, now);
    }

    /// Every `Status.json` the game writes.
    pub fn status(&mut self, status: &Value) {
        self.events.observe_status(status);
    }

    /// Is a live `event` (`Market`, `NavRoute`, ...) waiting for its file?
    pub fn awaiting(&self, event: &str) -> bool {
        self.awaiting.contains_key(event)
    }

    /// The companion file for a live `event`, as read. Ignored unless that event is waiting. A
    /// file older than the event is the last one, not this event's: the event keeps waiting
    /// for the game to write the new one. A file that comes more than [`MAX_EVENT_AGE_SECS`]
    /// after its event arrived (sharing was off when the right one came) is no longer live: the
    /// event is dropped. Anything else unusable is logged and the event dropped.
    pub fn companion_file(&mut self, event: &str, file: &Value, now: i64) {
        let (Some((ev, arrived)), Some(spec)) = (self.awaiting.get(event), companion_spec(event)) else {
            return;
        };
        if now - arrived > MAX_EVENT_AGE_SECS {
            self.awaiting.remove(event);
            return;
        }
        if let (Some(f), Some(e)) = (game_timestamp(file), game_timestamp(ev)) {
            if f < e {
                return;
            }
        }
        let (ev, _) = self.awaiting.remove(event).expect("checked above");
        let built: Result<_, Unusable> = match event {
            "NavRoute" => events::navroute(&ev, file),
            "FCMaterials" => events::fcmaterials(&ev, file),
            _ => companion::message(&ev, file),
        };
        self.queue(vec![built.map_err(|u| Skipped { spec, reason: u.to_string() })], now);
    }

    /// Send a pending signal run now: at the end of a journal, and on shutdown.
    pub fn flush(&mut self, now: i64) {
        if self.live {
            let run = self.events.flush(&self.sender.ctx);
            self.queue(run.into_iter().map(Ok).collect(), now);
        }
    }

    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    fn queue(&mut self, built: Vec<Built>, now: i64) {
        // Every path to the outbox comes through here: switched off, nothing is queued or
        // logged, whatever was pending (a signal run, a companion file) when it was switched.
        if !self.sharing.load(Ordering::SeqCst) {
            return;
        }
        let mut queued = false;
        for b in built {
            let r = match b {
                Ok((spec, msg)) => self.sender.submit(spec, msg, now).map(|r| queued |= r.is_ok()),
                Err(s) => self.sender.skip(s.spec, &s.reason, now),
            };
            // The outbox is unwritable (disk full, locked for longer than the busy timeout):
            // this message is lost, which EDDN prefers to a late or wrong one.
            if let Err(e) = r {
                (self.log)(&format!("EDDN outbox: {e}"));
            }
        }
        if queued {
            if let Some(w) = &self.worker {
                w.wake();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::gunzip;
    use crate::outbox::tests::{status, FakeTransport};
    use crate::testutil::{fixture, software, SHINRARTA, SHINRARTA_POS};
    use crate::transport::Response;
    use crate::worker::wall_now;
    use serde_json::json;
    use std::sync::mpsc::{channel, Receiver, Sender as Tx};
    use std::sync::Mutex;
    use std::time::Duration;

    const T0: &str = "2026-10-16T11:59:00Z";

    /// The game's timestamp for Unix seconds `t` (inverse of `unix_time`, for tests).
    fn game_ts(t: i64) -> String {
        let (days, secs) = (t.div_euclid(86_400), t.rem_euclid(86_400));
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = yoe + era * 400 + i64::from(m <= 2);
        format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", secs / 3600, secs / 60 % 60, secs % 60)
    }

    #[test]
    fn game_timestamps_convert_to_unix_time() {
        assert_eq!(unix_time(&json!({"timestamp":"1970-01-01T00:00:00Z"})), Some(0));
        assert_eq!(unix_time(&json!({"timestamp":"2026-10-16T12:00:00Z"})), Some(1_792_152_000));
        assert_eq!(unix_time(&json!({"timestamp":"2024-02-29T23:59:59Z"})), Some(1_709_251_199));
        assert_eq!(unix_time(&json!({"timestamp":"16 Oct"})), None);
        for t in [0, 951_782_400, 1_709_251_199, 1_792_152_000] {
            assert_eq!(unix_time(&json!({ "timestamp": game_ts(t) })), Some(t));
        }
    }

    #[test]
    fn a_live_event_minutes_old_is_not_sent() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        let now = unix_time(&json!({"timestamp":"2026-10-16T12:05:00Z"})).unwrap();
        // A line from a journal re-read as live: an hour old.
        f.journal(&docked("2026-10-16T11:05:00Z"), true, now);
        f.journal(&market_event("2026-10-16T11:05:01Z"), true, now);
        assert!(!f.awaiting("Market"));
        assert_eq!(f.sender().outbox().pending().unwrap(), 0);
        assert!(log(&f).is_empty());
        // Just inside the limit is sent.
        f.journal(&docked(&game_ts(now - MAX_EVENT_AGE_SECS)), true, now);
        assert_eq!(f.sender().outbox().pending().unwrap(), 1);
    }

    fn feed() -> Feed {
        Feed::new(Sender::new(Outbox::in_memory().unwrap(), software(), false))
    }

    /// A session's opening lines: header, commander, location.
    fn session(build: &str, cmdr: &str) -> Vec<Value> {
        vec![
            json!({"timestamp":T0,"event":"Fileheader","part":1,"gameversion":"4.0.0.1904","build":build}),
            json!({"timestamp":T0,"event":"Commander","Name":cmdr,"FID":"F1"}),
            json!({"timestamp":T0,"event":"LoadGame","Commander":cmdr,"FID":"F1","Horizons":true,"Odyssey":true}),
            location(),
        ]
    }

    fn location() -> Value {
        json!({"timestamp":T0,"event":"Location","StarSystem":"Shinrarta Dezhra","SystemAddress":SHINRARTA,
            "StarPos":SHINRARTA_POS,"Body":"Shinrarta Dezhra A","BodyID":1,"BodyType":"Star"})
    }

    fn docked(ts: &str) -> Value {
        json!({"timestamp":ts,"event":"Docked","StationName":"Jameson Memorial","MarketID":128666762u64,
            "StarSystem":"Shinrarta Dezhra","SystemAddress":SHINRARTA,"StationType":"Orbis"})
    }

    fn market_event(ts: &str) -> Value {
        json!({"timestamp":ts,"event":"Market","MarketID":128666762u64,"StationName":"Jameson Memorial",
            "StarSystem":"Shinrarta Dezhra"})
    }

    fn signal(ts: &str) -> Value {
        json!({"timestamp":ts,"event":"FSSSignalDiscovered","SystemAddress":SHINRARTA,"SignalName":"Beacon"})
    }

    fn feed_all(f: &mut Feed, evs: &[Value], live: bool) {
        for ev in evs {
            f.journal(ev, live, 100);
        }
    }

    /// Deliver everything queued and return the uploads, oldest first.
    fn sent(f: &Feed) -> Vec<Value> {
        let n = f.sender().outbox().pending().unwrap();
        let t = FakeTransport::new((0..n).map(|_| status(200)));
        f.sender().outbox().deliver_due(&t, 100, true, &crate::outbox::tests::no_stop).unwrap();
        let ups = t.posted.borrow().iter().map(|b| gunzip(b)).collect();
        ups
    }

    fn schemas(ups: &[Value]) -> Vec<String> {
        ups.iter().map(|u| u["$schemaRef"].as_str().unwrap().rsplit("schemas/").next().unwrap().to_owned()).collect()
    }

    fn log(f: &Feed) -> Vec<(String, String)> {
        let mut l: Vec<_> =
            f.sender().outbox().recent(100).unwrap().into_iter().map(|e| (e.outcome, e.detail)).collect();
        l.reverse();
        l
    }

    #[test]
    fn a_replayed_journal_sets_the_context_but_sends_nothing() {
        let mut f = feed();
        let mut replay = session("r1 ", "Jameson");
        replay.extend([docked("2026-10-16T12:00:00Z"), market_event("2026-10-16T12:00:01Z")]);
        replay.push(json!({"timestamp":"2026-10-16T12:00:02Z","event":"Scan","SystemAddress":SHINRARTA}));
        feed_all(&mut f, &replay, false);
        f.companion_file("Market", &fixture("Market.json"), 100);
        assert_eq!(f.sender().outbox().pending().unwrap(), 0, "nothing from the catch-up is queued");
        assert!(log(&f).is_empty(), "and nothing is logged as skipped either");

        // The first live line is sent with the header, flags and commander from the replay.
        f.journal(&docked("2026-10-16T12:05:00Z"), true, 100);
        let ups = sent(&f);
        assert_eq!(schemas(&ups), ["journal/1"]);
        assert_eq!(ups[0]["header"]["uploaderID"], "Jameson");
        assert_eq!(
            (&ups[0]["header"]["gameversion"], &ups[0]["header"]["gamebuild"]),
            (&json!("4.0.0.1904"), &json!("r1 "))
        );
        assert_eq!((&ups[0]["message"]["horizons"], &ups[0]["message"]["odyssey"]), (&json!(true), &json!(true)));
        assert_eq!(ups[0]["message"]["StarPos"], json!(SHINRARTA_POS), "augmented from the replayed Location");
    }

    #[test]
    fn a_new_journal_is_read_from_its_fileheader() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        // The game starts a new session while astromech runs: that journal is live throughout.
        feed_all(&mut f, &session("r2 ", "Other"), true);
        f.journal(&docked("2026-10-16T13:00:00Z"), true, 100);
        let ups = sent(&f);
        assert_eq!(schemas(&ups), ["journal/1", "journal/1"], "Location, then Docked");
        for u in &ups {
            assert_eq!((&u["header"]["uploaderID"], &u["header"]["gamebuild"]), (&json!("Other"), &json!("r2 ")));
        }
    }

    #[test]
    fn a_live_journal_without_its_fileheader_sends_nothing() {
        let mut f = feed();
        f.journal(&docked("2026-10-16T12:05:00Z"), true, 100);
        assert_eq!(f.sender().outbox().pending().unwrap(), 0);
        assert_eq!(log(&f), [("dropped".into(), "game version not known yet".into())]);
    }

    #[test]
    fn companion_files_are_sent_only_for_live_events() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        f.journal(&market_event("2026-10-16T12:00:00Z"), false, 100);
        assert!(!f.awaiting("Market"));
        f.companion_file("Market", &fixture("Market.json"), 100);
        assert_eq!(f.sender().outbox().pending().unwrap(), 0);

        f.journal(&market_event("2026-10-16T12:00:00Z"), true, 100);
        assert!(f.awaiting("Market"));
        // Last visit's file: the event keeps waiting for the new one, and nothing is logged.
        let mut old = fixture("Market.json");
        old["timestamp"] = json!("2026-10-16T11:00:00Z");
        f.companion_file("Market", &old, 100);
        assert!(f.awaiting("Market"));
        assert!(log(&f).is_empty());
        f.companion_file("Market", &fixture("Market.json"), 100);
        assert!(!f.awaiting("Market"));
        // The same file again (the game rewrote it) sends nothing more.
        f.companion_file("Market", &fixture("Market.json"), 100);
        assert_eq!(schemas(&sent(&f)), ["commodity/3"]);
    }

    #[test]
    fn an_unusable_companion_file_is_logged() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        f.journal(&market_event("2026-10-16T12:00:00Z"), true, 100);
        let mut other = fixture("Market.json");
        other["MarketID"] = json!(1);
        f.companion_file("Market", &other, 100);
        assert_eq!(f.sender().outbox().pending().unwrap(), 0);
        let l = f.sender().outbox().recent(1).unwrap();
        assert_eq!(
            (l[0].outcome.as_str(), l[0].schema_ref.as_str()),
            ("dropped", "https://eddn.edcd.io/schemas/commodity/3")
        );
        assert!(l[0].detail.contains("MarketID"), "{}", l[0].detail);
        assert!(!f.awaiting("Market"));
    }

    #[test]
    fn outfitting_is_sent_live_and_not_during_catch_up() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        let outfitting = json!({"timestamp":"2026-10-16T12:01:00Z","event":"Outfitting","MarketID":128666762u64,
            "StationName":"Jameson Memorial","StarSystem":"Shinrarta Dezhra"});
        f.journal(&outfitting, false, 100);
        assert!(!f.awaiting("Outfitting"));
        f.companion_file("Outfitting", &fixture("Outfitting.json"), 100);
        assert_eq!(f.sender().outbox().pending().unwrap(), 0, "nothing from the catch-up is queued");
        assert!(log(&f).is_empty());

        f.journal(&outfitting, true, 100);
        assert!(f.awaiting("Outfitting"));
        f.companion_file("Outfitting", &fixture("Outfitting.json"), 100);
        let ups = sent(&f);
        assert_eq!(schemas(&ups), ["outfitting/2"]);
        assert_eq!(
            ups[0]["message"]["modules"],
            json!(["hpt_chafflauncher_tiny", "int_engine_size3_class5_fast", "sidewinder_armour_grade1"]),
            "the fixture's PowerPlay and tech-broker modules are not sent"
        );
        assert!(log(&f).iter().all(|(outcome, _)| outcome == "sent"), "{:?}", log(&f));
    }

    #[test]
    fn navroute_waits_for_the_file_its_event_announced() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        let ts = "2026-10-16T12:10:00Z";
        f.journal(&json!({"timestamp":ts,"event":"NavRoute"}), true, 100);
        let route = json!([{"StarSystem":"Shinrarta Dezhra","SystemAddress":SHINRARTA,"StarPos":SHINRARTA_POS,"StarClass":"K"}]);
        f.companion_file(
            "NavRoute",
            &json!({"timestamp":"2026-10-16T11:00:00Z","event":"NavRoute","Route":route}),
            100,
        );
        assert!(f.awaiting("NavRoute"), "the last plot's file");
        f.companion_file("NavRoute", &json!({"timestamp":ts,"event":"NavRoute","Route":route}), 100);
        assert_eq!(schemas(&sent(&f)), ["navroute/1"]);
    }

    #[test]
    fn skipped_events_are_logged() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        f.journal(&json!({"timestamp":"2026-10-16T12:00:00Z","event":"Scan","SystemAddress":SHINRARTA}), true, 100);
        assert_eq!(log(&f), [("dropped".into(), "Scan event has no StarSystem".into())]);
    }

    #[test]
    fn a_signal_run_from_the_replay_is_not_sent_and_a_live_one_is_flushed() {
        let mut f = feed();
        let mut replay = session("r1 ", "Jameson");
        replay.push(signal("2026-10-16T12:00:00Z"));
        feed_all(&mut f, &replay, false);
        f.journal(&signal("2026-10-16T12:01:00Z"), true, 100);
        f.flush(100);
        let ups = sent(&f);
        assert_eq!(schemas(&ups), ["fsssignaldiscovered/1"]);
        let ts: Vec<&Value> =
            ups[0]["message"]["signals"].as_array().unwrap().iter().map(|s| &s["timestamp"]).collect();
        assert_eq!(ts, [&json!("2026-10-16T12:01:00Z")]);
    }

    /// Posts block until the test lets them go.
    struct Gate {
        started: Mutex<Tx<()>>,
        release: Mutex<Receiver<()>>,
    }
    impl Transport for Gate {
        fn post(&self, _: &[u8]) -> Result<Response, String> {
            self.started.lock().unwrap().send(()).unwrap();
            // Bounded, so a failing test can't hang on the worker's join.
            self.release.lock().unwrap().recv_timeout(Duration::from_secs(10)).map_err(|e| e.to_string())?;
            Ok(Response { status: 200, body: "OK".into() })
        }
    }

    #[test]
    fn queuing_never_waits_for_a_post_in_progress() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("eddn.sqlite");
        let (started_tx, started) = channel();
        let (release, release_rx) = channel();
        let gate = Gate { started: Mutex::new(started_tx), release: Mutex::new(release_rx) };
        let mut f =
            Feed::start(&db, software(), true, Box::new(gate), Arc::new(AtomicBool::new(true)), |_| {}).unwrap();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        // Real time: the worker gives up on messages queued long ago by its own clock.
        let now = wall_now();
        f.journal(&docked(&game_ts(now)), true, now);
        started.recv_timeout(Duration::from_secs(10)).expect("the worker posts the first message");

        // The post is stuck. Observing and queuing still complete at once.
        let t = Instant::now();
        f.journal(&docked(&game_ts(now + 1)), true, now);
        f.journal(&docked(&game_ts(now + 2)), true, now);
        assert!(t.elapsed() < Duration::from_secs(2), "{:?}", t.elapsed());
        assert!(started.try_recv().is_err(), "still on the first post");
        assert_eq!(Outbox::open(&db).unwrap().pending().unwrap(), 3);

        release.send(()).unwrap();
        for _ in 0..2 {
            started.recv_timeout(Duration::from_secs(10)).expect("the next post");
            release.send(()).unwrap();
        }
        // The worker isn't joined on drop, so wait for the last post to be recorded.
        let o = Outbox::open(&db).unwrap();
        let deadline = Instant::now() + Duration::from_secs(10);
        while o.pending().unwrap() > 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        drop(f);
        assert_eq!(o.pending().unwrap(), 0);
        assert!(o.recent(10).unwrap().iter().all(|e| e.outcome == "sent" && e.schema_ref.ends_with("/test")));
    }

    #[test]
    fn dropping_the_feed_does_not_wait_for_a_post_in_progress() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("eddn.sqlite");
        let (started_tx, started) = channel();
        let (release, release_rx) = channel();
        let gate = Gate { started: Mutex::new(started_tx), release: Mutex::new(release_rx) };
        // Two messages due at once, so the second is in the same pass as the held post.
        let queued = Outbox::open(&db).unwrap();
        queued.enqueue("https://eddn.edcd.io/schemas/journal/1/test", b"one", wall_now()).unwrap();
        queued.enqueue("https://eddn.edcd.io/schemas/journal/1/test", b"two", wall_now()).unwrap();
        let f = Feed::start(&db, software(), true, Box::new(gate), Arc::new(AtomicBool::new(true)), |_| {}).unwrap();
        started.recv_timeout(Duration::from_secs(10)).expect("the worker posts");

        let t = Instant::now();
        drop(f);
        assert!(t.elapsed() < Duration::from_secs(1), "{:?}", t.elapsed());
        // The held post finishes; the thread stops before posting the second message.
        release.send(()).unwrap();
        assert!(started.recv_timeout(Duration::from_secs(2)).is_err(), "no post after the stop");
        assert_eq!(queued.pending().unwrap(), 1, "the second waits for the next run");
    }

    #[test]
    fn a_run_of_stale_lines_is_logged_once_and_so_is_recovery() {
        let mut f = feed();
        let lines = Arc::new(Mutex::new(Vec::<String>::new()));
        let l2 = lines.clone();
        f.log = Arc::new(move |m: &str| l2.lock().unwrap().push(m.to_owned()));
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        let now = unix_time(&json!({"timestamp":"2026-10-16T12:05:00Z"})).unwrap();
        for m in 0..5 {
            f.journal(&docked(&game_ts(now - 3600 + m)), true, now);
        }
        assert_eq!(*lines.lock().unwrap(), ["EDDN: live journal line 60 min old, treating as replay"]);
        f.journal(&docked(&game_ts(now)), true, now);
        f.journal(&docked(&game_ts(now)), true, now);
        assert_eq!(lines.lock().unwrap().len(), 2);
        assert_eq!(lines.lock().unwrap()[1], "EDDN: live journal lines are current again, sending");
        assert_eq!(f.sender().outbox().pending().unwrap(), 2);
    }

    #[test]
    fn switched_off_nothing_is_queued_and_switching_on_resumes_with_the_right_context() {
        let mut f = feed();
        f.sharing.store(false, Ordering::SeqCst);
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        f.journal(&docked("2026-10-16T12:05:00Z"), true, 100);
        f.journal(&market_event("2026-10-16T12:05:01Z"), true, 100);
        f.journal(&json!({"timestamp":"2026-10-16T12:05:02Z","event":"Scan","SystemAddress":SHINRARTA}), true, 100);
        assert!(!f.awaiting("Market"));
        assert_eq!(f.sender().outbox().pending().unwrap(), 0, "the outbox stays empty");
        assert!(log(&f).is_empty(), "and nothing is logged");

        f.sharing.store(true, Ordering::SeqCst);
        f.journal(&docked("2026-10-16T12:06:00Z"), true, 100);
        let ups = sent(&f);
        assert_eq!(schemas(&ups), ["journal/1"]);
        assert_eq!(ups[0]["header"]["uploaderID"], "Jameson");
    }

    #[test]
    fn a_companion_file_coming_minutes_after_its_event_is_not_sent() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        let now = unix_time(&json!({"timestamp":"2026-10-16T12:00:00Z"})).unwrap();
        f.journal(&market_event("2026-10-16T12:00:00Z"), true, now);
        // Sharing goes off and back on while the event waits; its file comes minutes later.
        f.sharing.store(false, Ordering::SeqCst);
        f.sharing.store(true, Ordering::SeqCst);
        assert!(f.awaiting("Market"));
        f.companion_file("Market", &fixture("Market.json"), now + MAX_EVENT_AGE_SECS + 1);
        assert!(!f.awaiting("Market"));
        assert_eq!(f.sender().outbox().pending().unwrap(), 0, "a stale Market is not sent");
        assert!(log(&f).is_empty());

        // An event accepted near the live-age limit still gets its file, which comes seconds
        // after it arrived.
        let now = unix_time(&json!({"timestamp":"2026-10-16T12:04:50Z"})).unwrap();
        f.journal(&market_event("2026-10-16T12:00:00Z"), true, now);
        f.companion_file("Market", &fixture("Market.json"), now + 15);
        assert!(!f.awaiting("Market"));
        assert_eq!(f.sender().outbox().pending().unwrap(), 1, "queued");
    }

    #[test]
    fn switched_off_a_pending_run_or_companion_file_queues_nothing() {
        let mut f = feed();
        feed_all(&mut f, &session("r1 ", "Jameson"), false);
        f.journal(&market_event("2026-10-16T12:00:00Z"), true, 100);
        f.journal(&signal("2026-10-16T12:01:00Z"), true, 100);
        assert!(f.awaiting("Market"));
        assert_eq!(f.sender().outbox().pending().unwrap(), 0, "the run is pending, not queued");
        f.sharing.store(false, Ordering::SeqCst);
        f.flush(100);
        f.companion_file("Market", &fixture("Market.json"), 100);
        assert_eq!(f.sender().outbox().pending().unwrap(), 0, "the outbox stays empty");
        assert!(log(&f).is_empty());
    }
}
