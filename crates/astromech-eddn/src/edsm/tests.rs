//! The EDSM upload against a fake EDSM: what goes into each request, and how answers settle it.

use super::api::tests::{Call, FakeApi};
use super::api::{DISCARD_URL, JOURNAL_URL};
use super::*;
use crate::testutil::fixture;
use crate::transport::Response;
use serde_json::json;

const NOW: i64 = 1_800_000_000;

fn creds() -> Credentials {
    Credentials { commander: "Jameson".into(), api_key: "s3cret-key".into() }
}

fn software() -> Software {
    Software { name: "astromech".into(), version: "0.1.5".into() }
}

/// The synthetic session in `tests/fixtures/edsm/session.jsonl`.
fn session() -> Vec<Value> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/edsm/session.jsonl");
    std::fs::read_to_string(path).unwrap().lines().map(|l| serde_json::from_str(l).unwrap()).collect()
}

/// EDSM's discard list as served on 17 Oct 2026 (`GET /api-journal-v1/discard`).
fn discard_list() -> Value {
    fixture("edsm/discard.json")
}

fn name(ev: &Value) -> &str {
    ev["event"].as_str().unwrap()
}

/// An uploader and a courier on one queue file, sharing the switch and the discard list.
struct Rig {
    _dir: tempfile::TempDir,
    up: Uploader,
    courier: Courier,
    api: FakeApi,
    switch: Arc<Switch>,
    discards: Arc<Discards>,
    t0: Instant,
}

fn rig(on: bool) -> Rig {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join(DB_FILE);
    let switch = Arc::new(Switch::default());
    switch.set(on.then(creds));
    let discards = Arc::new(Discards::default());
    let api = FakeApi::default();
    let up = Uploader::new(Outbox::open(&db).unwrap().with_label("edsm"), switch.clone(), discards.clone());
    let t0 = Instant::now();
    let courier = Courier::new(
        Outbox::open(&db).unwrap().with_label("edsm"),
        Box::new(api.clone()),
        software(),
        switch.clone(),
        discards.clone(),
        t0,
    );
    Rig { _dir: dir, up, courier, api, switch, discards, t0 }
}

/// EDSM accepting every event of a request of `n`.
fn all_ok(n: usize) -> Value {
    json!({"msgnum":100,"msg":"OK","events": vec![json!({"msgnum":100,"msg":"Message stored"}); n]})
}

fn feed(r: &mut Rig, events: &[Value], live: bool) {
    for ev in events {
        r.up.journal(ev, live, NOW);
    }
}

#[test]
fn fixture_session_requests_exclude_everything_on_the_discard_list() {
    let list: HashSet<String> = serde_json::from_value(discard_list()).unwrap();
    let events = session();
    let (kept, dropped): (Vec<&Value>, Vec<&Value>) = events.iter().partition(|e| !list.contains(name(e)));
    assert!(dropped.len() >= 8 && kept.len() >= 8, "the fixture has both kinds");

    // The list isn't known yet when the events arrive (first run): everything is queued, and
    // the courier fetches the list before its first request and leaves the listed ones out.
    // (All but the Fileheader, which comes before the game says who is playing.)
    let mut r = rig(true);
    feed(&mut r, &events, true);
    assert_eq!(r.up.outbox().pending().unwrap(), events.len() - 1);
    r.api.ok(DISCARD_URL, discard_list()).ok(JOURNAL_URL, all_ok(kept.len()));
    let d = r.courier.pass(NOW, r.t0).unwrap();
    assert_eq!((d.sent, d.rejected, d.retrying), (kept.len(), 0, 0));
    assert_eq!(r.courier.outbox().pending().unwrap(), 0, "the discarded ones are dropped, not left queued");

    let calls = r.api.calls.lock().unwrap().clone();
    assert!(matches!(&calls[0], Call::Get(u, _) if u == DISCARD_URL), "the list comes before any post");
    let posts = r.api.posts();
    assert_eq!(posts.len(), 1, "one session, one batch");
    let req = &posts[0];
    let sent: Vec<&str> = req["message"].as_array().unwrap().iter().map(name).collect();
    assert!(sent.iter().all(|e| !list.contains(*e)), "a discarded event was sent: {sent:?}");
    assert_eq!(sent, kept.iter().map(|e| name(e)).collect::<Vec<_>>(), "everything else, in journal order");

    // The documented parameters, with the key from the switch and the session's version.
    assert_eq!(
        (&req["commanderName"], &req["apiKey"], &req["fromSoftware"], &req["fromSoftwareVersion"]),
        (&json!("Jameson"), &json!("s3cret-key"), &json!("astromech"), &json!("0.1.5"))
    );
    assert_eq!((&req["fromGameVersion"], &req["fromGameBuild"]), (&json!("4.0.0.1904"), &json!("r307371/r0 ")));

    // Each event as the game wrote it, plus the transient state at that moment.
    let buy = &req["message"][sent.iter().position(|e| *e == "MarketBuy").unwrap()];
    let original = events.iter().find(|e| name(e) == "MarketBuy").unwrap();
    for (k, v) in original.as_object().unwrap() {
        assert_eq!(&buy[k], v, "{k} changed");
    }
    assert_eq!(buy["_systemName"], "Shinrarta Dezhra");
    assert_eq!(buy["_systemAddress"], 3932277478106u64);
    assert_eq!(buy["_stationName"], "Jameson Memorial");
    assert_eq!(buy["_marketId"], 128666762);
    assert_eq!(buy["_shipId"], 7);
    assert_eq!(buy["_systemCoordinates"], json!([55.71875, 17.59375, 27.15625]));
    let sell = &req["message"][sent.iter().position(|e| *e == "MarketSell").unwrap()];
    assert_eq!((&sell["_systemName"], &sell["_stationName"]), (&json!("Sol"), &json!("Abraham Lincoln")));

    // Sent rows in the log; the list is kept for the next run.
    let log = r.courier.outbox().recent(5).unwrap();
    assert_eq!((log[0].outcome.as_str(), log[0].detail.as_str()), ("sent", "10 events"));
    let kept_list: Vec<String> = serde_json::from_str(&r.courier.outbox().meta("discard").unwrap().unwrap()).unwrap();
    assert_eq!(kept_list.len(), list.len());
}

#[test]
fn once_the_list_is_known_discarded_events_are_not_even_queued() {
    let mut r = rig(true);
    r.discards.set(serde_json::from_value(discard_list()).unwrap(), NOW);
    feed(&mut r, &session(), true);
    assert_eq!(r.up.outbox().pending().unwrap(), 10);
    let queued = r.up.outbox().due(NOW, true, 100).unwrap();
    let names: Vec<&str> = queued.iter().map(|q| q.schema_ref.as_str()).collect();
    assert!(!names.contains(&"Music") && !names.contains(&"ReceiveText") && !names.contains(&"Fileheader"));
    // The queue never holds the API key: it's added when a request is built.
    for q in &queued {
        assert!(!String::from_utf8_lossy(&q.body).contains("s3cret-key"));
    }
    assert!(r.up.outbox().recent(10).unwrap().is_empty(), "nothing held back, nothing logged");
}

#[test]
fn a_list_from_an_earlier_run_is_used_until_it_is_a_day_old() {
    let r = rig(true);
    r.courier.outbox().set_meta("discard", r#"["Music"]"#).unwrap();
    r.courier.outbox().set_meta("discard_at", &(NOW - 60).to_string()).unwrap();
    let d = Discards::default();
    d.load(r.courier.outbox()).unwrap();
    assert_eq!((d.contains("Music"), d.contains("FSDJump")), (Some(true), Some(false)));
    assert!(d.fresh(NOW) && !d.fresh(NOW - 60 + DISCARD_MAX_AGE));
}

#[test]
fn without_a_discard_list_nothing_is_sent() {
    let mut r = rig(true);
    feed(&mut r, &session(), true);
    r.api.answer(DISCARD_URL, Err("connection refused".into()));
    let d = r.courier.pass(NOW, r.t0).unwrap();
    assert!(d.stopped && d.sent == 0);
    assert!(r.api.posts().is_empty());
    assert_eq!(r.courier.outbox().pending().unwrap(), 19, "kept for when the list can be read");
    assert_eq!(r.courier.outbox().recent(1).unwrap()[0].detail, "discard list: connection refused");
}

#[test]
fn off_or_without_credentials_nothing_is_queued_and_the_queue_is_discarded() {
    let mut r = rig(false);
    feed(&mut r, &session(), true);
    assert_eq!(r.up.outbox().pending().unwrap(), 0);
    // Queued while on, then switched off: nothing is posted, and the queue goes.
    r.switch.set(Some(creds()));
    feed(&mut r, &session()[9..10], true);
    assert_eq!(r.up.outbox().pending().unwrap(), 1);
    r.switch.set(None);
    assert_eq!(r.courier.pass(NOW, r.t0).unwrap().gave_up, 1);
    assert!(r.api.calls.lock().unwrap().is_empty());
    let row = &r.courier.outbox().recent(1).unwrap()[0];
    assert_eq!((row.schema_ref.as_str(), row.detail.as_str()), ("edsm", "switched off: 1 queued discarded"));
}

#[test]
fn the_startup_replay_is_not_sent() {
    let mut r = rig(true);
    feed(&mut r, &session(), false);
    assert_eq!(r.up.outbox().pending().unwrap(), 0);
    // But it set the state: a live event after it carries the system.
    r.up.journal(
        &json!({"timestamp":"2026-10-16T12:08:00Z","event":"Undocked","StationName":"Abraham Lincoln"}),
        true,
        NOW,
    );
    let q = &r.up.outbox().due(NOW, true, 1).unwrap()[0];
    let body: Value = serde_json::from_slice(&q.body).unwrap();
    assert_eq!(body["message"]["_systemName"], "Sol");
    assert_eq!(body["commander"], "Jameson");
}

#[test]
fn another_commanders_events_are_held_back_and_logged_once() {
    let mut r = rig(true);
    r.switch.set(Some(Credentials { commander: "Someone Else".into(), api_key: "k".into() }));
    feed(&mut r, &session(), true);
    assert_eq!(r.up.outbox().pending().unwrap(), 0);
    let log = r.up.outbox().recent(10).unwrap();
    let held: Vec<&str> = log.iter().map(|l| l.detail.as_str()).collect();
    assert!(held.contains(&"not sent: playing as Jameson, not Someone Else (Settings → Data sharing)"), "{held:?}");
    assert_eq!(held.iter().filter(|d| d.contains("playing as")).count(), 1);
    // The name is matched without regard to case.
    r.switch.set(Some(Credentials { commander: "JAMESON".into(), api_key: "k".into() }));
    feed(&mut r, &session()[9..10], true);
    assert_eq!(r.up.outbox().pending().unwrap(), 1);
}

#[test]
fn beta_and_crew_sessions_are_not_sent() {
    let mut r = rig(true);
    let mut events = session();
    events[0]["gameversion"] = json!("4.1.0.100 BETA");
    feed(&mut r, &events, true);
    assert_eq!(r.up.outbox().pending().unwrap(), 0);
    let mut r = rig(true);
    feed(&mut r, &session()[..7], true);
    let before = r.up.outbox().pending().unwrap();
    r.up.journal(&json!({"event":"JoinACrew","Captain":"Someone Else"}), true, NOW);
    r.up.journal(&json!({"event":"MarketBuy","Type":"gold","Count":1}), true, NOW);
    assert_eq!(r.up.outbox().pending().unwrap(), before);
    r.up.journal(&json!({"event":"QuitACrew","Captain":"Someone Else"}), true, NOW);
    assert_eq!(r.up.outbox().pending().unwrap(), before + 1, "QuitACrew itself goes");
}

#[test]
fn a_wrong_commander_or_key_keeps_the_events_for_later() {
    let mut r = rig(true);
    r.discards.set(serde_json::from_value(discard_list()).unwrap(), NOW);
    feed(&mut r, &session(), true);
    r.api.ok(JOURNAL_URL, json!({"msgnum":203,"msg":"Commander name/API Key not found","events":[]}));
    let d = r.courier.pass(NOW, r.t0).unwrap();
    assert!(d.stopped && d.retrying == 10);
    assert_eq!(r.courier.outbox().pending().unwrap(), 10);
    assert!(r.courier.outbox().due(NOW, true, 1).unwrap().is_empty(), "nothing is due until the retry");
    let row = &r.courier.outbox().recent(1).unwrap()[0];
    assert_eq!(
        (row.outcome.as_str(), row.detail.as_str()),
        ("retry", "203 Commander name/API Key not found; retry in 60 s")
    );
    // EDSM unreachable: the same.
    let mut r = rig(true);
    r.discards.set(vec![], NOW);
    feed(&mut r, &session()[..5], true);
    r.api.answer(JOURNAL_URL, Err("connection refused".into()));
    assert!(r.courier.pass(NOW, r.t0).unwrap().stopped);
    assert_eq!(r.courier.outbox().pending().unwrap(), 4, "all but the Fileheader");
}

#[test]
fn each_event_is_settled_by_its_own_answer() {
    let mut r = rig(true);
    r.discards.set(vec![], NOW);
    for e in ["FSDJump", "Docked", "Scan", "Bogus", "Materials"] {
        r.up.journal(
            &json!({"event":"LoadGame","Commander":"Jameson","gameversion":"4.0.0.1904","build":"r1"}),
            true,
            NOW,
        );
        r.up.journal(&json!({"timestamp":"2026-10-16T12:00:00Z","event":e}), true, NOW);
    }
    // Only the five non-LoadGame events matter here: answer LoadGame with 100 too.
    let codes = [
        (100, "OK"),
        (100, "Message stored"),
        (100, "OK"),
        (101, "Message already stored"),
        (100, "OK"),
        (500, "Exception: boom"),
        (100, "OK"),
        (304, "Discarded event"),
        (100, "OK"),
        (451, "System probably non existant"),
    ];
    let events: Vec<Value> = codes.iter().map(|(c, m)| json!({"msgnum":c,"msg":m})).collect();
    r.api.ok(JOURNAL_URL, json!({"msgnum":100,"msg":"OK","events":events}));
    let d = r.courier.pass(NOW, r.t0).unwrap();
    assert_eq!((d.sent, d.retrying, d.rejected), (7, 1, 1));
    let left = r.courier.outbox().due(NOW + 3600, true, 10).unwrap();
    assert_eq!(left.iter().map(|q| q.schema_ref.as_str()).collect::<Vec<_>>(), ["Scan"], "only the 500 waits");
    assert!(!r.discards.fresh(NOW), "a 304 means the list is out of date: fetched again");
    let log: Vec<String> = r
        .courier
        .outbox()
        .recent(10)
        .unwrap()
        .iter()
        .map(|l| format!("{} {} {}", l.schema_ref, l.outcome, l.detail))
        .collect();
    assert!(log.contains(&"Materials rejected 451 System probably non existant".to_owned()), "{log:?}");
    assert!(log.contains(&"Bogus dropped 304 Discarded event".to_owned()), "{log:?}");
}

#[test]
fn a_request_holds_one_game_session_and_at_most_a_batch() {
    let mut r = rig(true);
    r.discards.set(vec![], NOW);
    let start = |v: &str| {
        vec![
            json!({"event":"Fileheader","part":1,"gameversion":v,"build":"r1"}),
            json!({"event":"LoadGame","Commander":"Jameson"}),
        ]
    };
    feed(&mut r, &start("4.0.0.1904"), true);
    for _ in 0..BATCH + 5 {
        r.up.journal(&json!({"event":"Scan"}), true, NOW);
    }
    feed(&mut r, &start("4.0.0.2000"), true);
    // LoadGame and 55 scans, then the next session's LoadGame (each Fileheader comes before
    // the game says who is playing, so it isn't queued).
    r.api.ok(JOURNAL_URL, all_ok(BATCH)).ok(JOURNAL_URL, all_ok(6)).ok(JOURNAL_URL, all_ok(1));
    assert_eq!(r.courier.pass(NOW, r.t0).unwrap().sent, BATCH + 7);
    let posts = r.api.posts();
    let sizes: Vec<usize> = posts.iter().map(|p| p["message"].as_array().unwrap().len()).collect();
    assert_eq!(sizes, [BATCH, 6, 1]);
    assert_eq!(posts[1]["fromGameVersion"], "4.0.0.1904");
    assert_eq!(posts[2]["fromGameVersion"], "4.0.0.2000");
}

#[test]
fn events_queued_for_another_commander_are_not_sent_with_this_key() {
    let mut r = rig(true);
    r.discards.set(vec![], NOW);
    feed(&mut r, &session()[..7], true);
    r.switch.set(Some(Credentials { commander: "Someone Else".into(), api_key: "k".into() }));
    let d = r.courier.pass(NOW, r.t0).unwrap();
    assert_eq!(d, Delivery::default());
    assert!(r.api.posts().is_empty());
    assert_eq!(r.courier.outbox().pending().unwrap(), 0);
}

#[test]
fn a_corrected_key_sends_what_a_wrong_one_held_back_at_once() {
    let mut r = rig(true);
    r.discards.set(serde_json::from_value(discard_list()).unwrap(), NOW);
    feed(&mut r, &session(), true);
    r.api.ok(JOURNAL_URL, json!({"msgnum":203,"msg":"Commander name/API Key not found","events":[]}));
    assert!(r.courier.pass(NOW, r.t0).unwrap().stopped);
    // Without a change, the held events wait out their backoff (and the minute on the clock).
    assert_eq!(r.courier.pass(NOW + 5, r.t0 + Duration::from_secs(5)).unwrap(), Delivery::default());

    let woken = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let w = woken.clone();
    r.switch.set_waker(move || {
        w.fetch_add(1, Ordering::SeqCst);
    });
    r.switch.set(Some(creds())); // saved again, nothing changed
    assert_eq!(woken.load(Ordering::SeqCst), 0);
    r.switch.set(Some(Credentials { commander: "Jameson".into(), api_key: "fixed-key".into() }));
    assert_eq!(woken.load(Ordering::SeqCst), 1, "the uploader is woken");

    r.api.ok(JOURNAL_URL, all_ok(10));
    let d = r.courier.pass(NOW + 6, r.t0 + Duration::from_secs(6)).unwrap();
    assert_eq!(d.sent, 10, "{d:?}");
    assert_eq!(r.api.posts().last().unwrap()["apiKey"], "fixed-key");
    assert_eq!(r.courier.outbox().pending().unwrap(), 0);
    let mark: i64 = r.courier.outbox().meta(ERRORS_CLEARED).unwrap().unwrap().parse().unwrap();
    assert!(
        r.courier.outbox().recent_of_after(&[Outcome::Retry], mark, 1).unwrap().is_empty(),
        "the old 203 is hidden"
    );
}

#[test]
fn a_key_that_is_still_wrong_shows_its_error_from_the_same_pass() {
    let mut r = rig(true);
    r.discards.set(vec![], NOW);
    feed(&mut r, &session()[..5], true);
    r.switch.set(Some(Credentials { commander: "Jameson".into(), api_key: "typo".into() }));
    r.api.ok(JOURNAL_URL, json!({"msgnum":203,"msg":"Commander name/API Key not found","events":[]}));
    assert!(r.courier.pass(NOW, r.t0).unwrap().stopped);
    let mark: i64 = r.courier.outbox().meta(ERRORS_CLEARED).unwrap().unwrap().parse().unwrap();
    let shown = r.courier.outbox().recent_of_after(&[Outcome::Retry], mark, 1).unwrap();
    assert_eq!(shown.len(), 1, "the 203 is logged in the same second as the change but after it");
    assert!(shown[0].detail.starts_with("203"), "{}", shown[0].detail);
}

#[test]
fn outages_do_not_use_up_a_messages_refusals() {
    let mut r = rig(true);
    r.discards.set(vec![], NOW);
    // One event, so a refusal lands on it alone.
    feed(&mut r, &session()[..2], true);
    assert_eq!(r.courier.outbox().pending().unwrap(), 1);
    let at = |i: i64| (NOW + 4000 * i, r.t0 + Duration::from_secs(4000 * i as u64));
    for i in 0..MAX_ATTEMPTS - 1 {
        let outage =
            if i % 2 == 0 { Ok(Response { status: 429, body: "slow down".into() }) } else { Err("timed out".into()) };
        r.api.answer(JOURNAL_URL, outage);
        let (wall, mono) = at(i);
        let d = r.courier.pass(wall, mono).unwrap();
        assert_eq!((d.gave_up, d.retrying), (0, 1), "outage {i}: {d:?}");
    }
    // Seven outages have gone by: the event still has all its refusals, so it takes the full
    // MAX_ATTEMPTS of them to give it up.
    for i in 0..MAX_ATTEMPTS {
        r.api.answer(JOURNAL_URL, Ok(Response { status: 403, body: "<html>Cloudflare</html>".into() }));
        let (wall, mono) = at(MAX_ATTEMPTS + i);
        let d = r.courier.pass(wall, mono).unwrap();
        let last = i == MAX_ATTEMPTS - 1;
        assert_eq!((d.gave_up, d.retrying), (last as usize, !last as usize), "refusal {i}: {d:?}");
    }
    assert_eq!(r.courier.outbox().pending().unwrap(), 0);
}

#[test]
fn turning_sharing_off_does_not_wake_or_release_anything() {
    let r = rig(true);
    let woken = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let w = woken.clone();
    r.switch.set_waker(move || {
        w.fetch_add(1, Ordering::SeqCst);
    });
    r.switch.set(None);
    assert_eq!(woken.load(Ordering::SeqCst), 0);
}

/// An EDSM that fails the whole request (HTTP `status`) whenever it holds a "poisoned" event
/// (one named `Poison`) or more than `max` events, and otherwise stores everything.
#[derive(Clone)]
struct Choosy {
    status: u16,
    max: usize,
    sizes: Arc<Mutex<Vec<usize>>>,
    stored: Arc<Mutex<Vec<String>>>,
}

impl Choosy {
    fn new(status: u16, max: usize) -> Self {
        Self { status, max, sizes: Arc::default(), stored: Arc::default() }
    }
}

impl api::Api for Choosy {
    fn get(&self, _: &str, _: &[(&str, &str)]) -> Result<Response, String> {
        Ok(Response { status: 200, body: "[]".into() })
    }
    fn post_json(&self, _: &str, body: &[u8]) -> Result<Response, String> {
        let req: Value = serde_json::from_slice(body).unwrap();
        let events = req["message"].as_array().unwrap();
        self.sizes.lock().unwrap().push(events.len());
        if events.len() > self.max || events.iter().any(|e| e["event"] == "Poison") {
            return Ok(Response { status: self.status, body: "no".into() });
        }
        self.stored.lock().unwrap().extend(events.iter().map(|e| e["n"].to_string()));
        Ok(Response { status: 200, body: all_ok(events.len()).to_string() })
    }
}

/// A rig on `api` with `count` events queued, `Poison` (if any) at index `poison`.
fn choosy_rig(api: Choosy, count: usize, poison: Option<usize>) -> Rig {
    choosy_rig_with(api, count, poison.as_slice())
}

/// A rig on `api` with `count` events queued, `Poison` at each index in `poisons`.
fn choosy_rig_with(api: Choosy, count: usize, poisons: &[usize]) -> Rig {
    let mut r = rig(true);
    r.courier.api = Box::new(api);
    r.discards.set(vec![], NOW);
    // The Commander event queues too, with no `n`.
    feed(&mut r, &session()[..2], true);
    for n in 0..count {
        let event = if poisons.contains(&n) { "Poison" } else { "Scan" };
        r.up.journal(&json!({"event": event, "n": n}), true, NOW);
    }
    r
}

/// Run passes a minute apart until the queue is empty (or `limit` passes); the total counts.
fn run_until_empty(r: &mut Rig, limit: u64) -> Delivery {
    let mut total = Delivery::default();
    for i in 1..=limit {
        let d = r.courier.pass(NOW + 4000 * i as i64, r.t0 + Duration::from_secs(4000 * i)).unwrap();
        total.sent += d.sent;
        total.gave_up += d.gave_up;
        if r.courier.outbox().pending().unwrap() == 0 {
            break;
        }
    }
    total
}

#[test]
fn a_poisoned_event_costs_only_itself() {
    let api = Choosy::new(500, BATCH);
    let mut r = choosy_rig(api.clone(), 30, Some(17));
    assert_eq!(r.courier.outbox().pending().unwrap(), 31);
    let d = run_until_empty(&mut r, 100);
    assert_eq!((d.sent, d.gave_up), (30, 1), "{d:?}");
    let mut stored: Vec<i64> =
        api.stored.lock().unwrap().iter().filter(|n| *n != "null").map(|n| n.parse().unwrap()).collect();
    stored.sort();
    assert_eq!(stored, (0..30).filter(|n| *n != 17).collect::<Vec<_>>(), "everything but the poisoned one");
    let sizes = api.sizes.lock().unwrap().clone();
    assert_eq!(sizes[0], 31, "the whole batch goes first");
    assert_eq!(sizes[1], 16, "then the halves");
    let row = r.courier.outbox().recent_of(&[Outcome::GaveUp], 5).unwrap();
    assert_eq!(row.len(), 1);
    assert!(
        row[0].detail.contains("Poison refused 8 times") && row[0].detail.contains("HTTP 500"),
        "{}",
        row[0].detail
    );
}

#[test]
fn the_queue_behind_a_poisoned_event_moves_while_it_waits() {
    let api = Choosy::new(500, BATCH);
    let mut r = choosy_rig(api.clone(), 6, Some(0));
    // Passes until the poisoned event is alone and has failed once; it has not used up its tries.
    for i in 1..=8 {
        r.courier.pass(NOW + 100 * i, r.t0 + Duration::from_secs(100 * i as u64)).unwrap();
    }
    assert_eq!(r.courier.outbox().pending().unwrap(), 1, "only the poisoned event is left");
    assert_eq!(api.stored.lock().unwrap().len(), 6);
}

/// astromech-ix2p: two poisoned events do not hold the queue. After the batches have been split
/// down to them, each fails alone; the good events behind and between them keep going, pass
/// after pass, instead of waiting out the second one's back-off.
#[test]
fn two_poisoned_events_do_not_hold_the_queue() {
    let api = Choosy::new(500, BATCH);
    // Twelve events, the first two poisoned.
    let mut r = choosy_rig_with(api.clone(), 12, &[0, 1]);
    for i in 1..=20 {
        r.courier.pass(NOW + 100 * i, r.t0 + Duration::from_secs(100 * i as u64)).unwrap();
    }
    assert_eq!(r.courier.outbox().pending().unwrap(), 2, "only the two poisoned events are left");
    assert_eq!(api.stored.lock().unwrap().iter().filter(|n| *n != "null").count(), 10);
}

/// A known-poisoned event goes alone, not back through 50, 25, 13, ... with the events beside it:
/// once it has failed alone, the next pass in which it is due sends it by itself and the good
/// events in one request, though the cap has grown back to a full batch.
#[test]
fn a_known_poisoned_event_is_not_sent_back_through_the_halving() {
    let api = Choosy::new(500, BATCH);
    let mut r = choosy_rig_with(api.clone(), 40, &[5]);
    // Until the poisoned event has failed alone once.
    for i in 1..=12 {
        r.courier.pass(NOW + 100 * i, r.t0 + Duration::from_secs(100 * i as u64)).unwrap();
    }
    assert_eq!(r.courier.outbox().pending().unwrap(), 1, "only the poisoned event is left");
    // New events arrive; later the poisoned one is due again, beside them.
    for n in 100..140 {
        r.up.journal(&json!({"event": "Scan", "n": n}), true, NOW + 5000);
    }
    let before = api.sizes.lock().unwrap().len();
    r.courier.pass(NOW + 6000, r.t0 + Duration::from_secs(6000)).unwrap();
    let sizes = api.sizes.lock().unwrap()[before..].to_vec();
    assert_eq!(r.courier.outbox().pending().unwrap(), 1, "the good events went in this one pass: {sizes:?}");
    assert!(sizes.iter().all(|n| *n == 1 || *n == 40), "no batch holds the poisoned event with others: {sizes:?}");
}

#[test]
fn a_413_batch_is_split_the_same_way() {
    let api = Choosy::new(413, 3);
    let mut r = choosy_rig(api.clone(), 8, None);
    let d = run_until_empty(&mut r, 20);
    assert_eq!((d.sent, d.gave_up), (9, 0), "{d:?}");
    let sizes = api.sizes.lock().unwrap().clone();
    assert_eq!(sizes[..2], [9, 5], "the refused batch is halved");
    assert!(sizes[2..].iter().any(|n| *n <= 3), "{sizes:?}");
    assert_eq!(api.stored.lock().unwrap().len(), 9);
}

#[test]
fn a_single_event_that_keeps_failing_is_given_up_and_later_events_go() {
    let api = Choosy::new(413, 0);
    let mut r = choosy_rig(api.clone(), 0, None);
    r.up.journal(&json!({"event": "Scan"}), true, NOW);
    r.courier.outbox().take(r.courier.outbox().due(NOW, true, 1).unwrap()[0].id).unwrap();
    let hour = |i: i64| Duration::from_secs(4000 * i as u64);
    for i in 0..MAX_ATTEMPTS - 1 {
        let d = r.courier.pass(NOW + 4000 * i, r.t0 + hour(i)).unwrap();
        assert_eq!((d.gave_up, d.retrying), (0, 1), "try {i}: {d:?}");
    }
    let last = MAX_ATTEMPTS - 1;
    let d = r.courier.pass(NOW + 4000 * last, r.t0 + hour(last)).unwrap();
    assert_eq!((d.gave_up, d.retrying), (1, 0));
    assert_eq!(r.courier.outbox().pending().unwrap(), 0);

    let later = NOW + 4000 * last + 1;
    r.up.journal(&json!({"event":"Scan"}), true, later);
    r.courier.api = Box::new(Choosy::new(200, BATCH));
    let d = r.courier.pass(later, r.t0 + hour(last) + Duration::from_secs(1)).unwrap();
    assert_eq!(d.sent, 1, "the queue is not blocked: {d:?}");
}

#[test]
fn an_outage_of_5xx_answers_gives_up_a_few_events_not_the_queue() {
    let api = Choosy::new(503, 0);
    let mut r = choosy_rig(api, 20, None);
    let queued = r.courier.outbox().pending().unwrap();
    assert_eq!(queued, 21);
    let mut gave_up = 0;
    for i in 1..=20u64 {
        gave_up += r.courier.pass(NOW + 4000 * i as i64, r.t0 + Duration::from_secs(4000 * i)).unwrap().gave_up;
    }
    // Without the outage guard every one of the 21 goes; with it only the few that fail alone
    // first (OUTAGE_AFTER of them a pass, so that two poisoned events side by side don't look like one).
    assert!((2..=6).contains(&gave_up), "gave up {gave_up} of {queued}");
    assert_eq!(r.courier.outbox().pending().unwrap(), queued - gave_up);
}

#[test]
fn what_counts_as_refused_and_what_is_only_held() {
    let ok = |status: u16, body: &str| Ok(Response { status, body: body.into() });
    assert!(matches!(answer(ok(413, "big"), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(400, ""), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(200, "<html>"), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(200, &all_ok(1).to_string()), 2), Answer::Refused(_)), "wrong length");
    for held in [ok(429, "x"), ok(408, "x"), Err("timed out".into())] {
        assert!(matches!(answer(held, 2), Answer::Hold(_)));
    }
    assert!(matches!(answer(ok(200, r#"{"msgnum":203,"msg":"x"}"#), 2), Answer::Hold(_)));
    // Request-level 5xx, HTTP or code, is a failure of the request: split, then given up.
    assert!(matches!(answer(ok(500, "x"), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(503, "x"), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(200, r#"{"msgnum":500,"msg":"x"}"#), 2), Answer::Refused(_)));
}
