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
    r.api.answer(JOURNAL_URL, Ok(Response { status: 503, body: "busy".into() }));
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
    assert_eq!(r.courier.outbox().meta(ERRORS_CLEARED).unwrap(), Some((NOW + 6).to_string()));
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

#[test]
fn a_refused_batch_is_given_up_after_the_attempt_limit_and_later_events_go() {
    let mut r = rig(true);
    r.discards.set(vec![], NOW);
    feed(&mut r, &session()[..5], true);
    let queued = r.courier.outbox().pending().unwrap();
    assert_eq!(queued, 4);
    let hour = |i: i64| Duration::from_secs(4000 * i as u64);
    for i in 0..MAX_ATTEMPTS - 1 {
        r.api.answer(JOURNAL_URL, Ok(Response { status: 413, body: "too large".into() }));
        let d = r.courier.pass(NOW + 4000 * i, r.t0 + hour(i)).unwrap();
        assert!(d.stopped && d.retrying == queued, "try {i}: {d:?}");
        assert_eq!(r.courier.outbox().pending().unwrap(), queued);
    }
    // A later event joins the queue; the old batch is refused one last time.
    let last = MAX_ATTEMPTS - 1;
    r.api.answer(JOURNAL_URL, Ok(Response { status: 413, body: "too large".into() }));
    let d = r.courier.pass(NOW + 4000 * last, r.t0 + hour(last)).unwrap();
    assert_eq!((d.gave_up, d.retrying), (queued, 0));
    assert_eq!(r.courier.outbox().pending().unwrap(), 0);
    let row = &r.courier.outbox().recent(1).unwrap()[0];
    assert_eq!(row.outcome, "gave_up");
    assert!(row.detail.contains("refused 8 times") && row.detail.contains("HTTP 413: too large"), "{}", row.detail);

    let later = NOW + 4000 * last + 1;
    r.up.journal(&json!({"event":"Scan"}), true, later);
    r.api.ok(JOURNAL_URL, all_ok(1));
    let d = r.courier.pass(later, r.t0 + hour(last) + Duration::from_secs(1)).unwrap();
    assert_eq!(d.sent, 1, "the queue is not blocked: {d:?}");
}

#[test]
fn what_counts_as_refused_and_what_is_only_held() {
    let ok = |status: u16, body: &str| Ok(Response { status, body: body.into() });
    assert!(matches!(answer(ok(413, "big"), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(400, ""), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(200, "<html>"), 2), Answer::Refused(_)));
    assert!(matches!(answer(ok(200, &all_ok(1).to_string()), 2), Answer::Refused(_)), "wrong length");
    for held in [ok(503, "x"), ok(429, "x"), ok(408, "x"), Err("timed out".into())] {
        assert!(matches!(answer(held, 2), Answer::Hold(_)));
    }
    assert!(matches!(answer(ok(200, r#"{"msgnum":203,"msg":"x"}"#), 2), Answer::Hold(_)));
}
