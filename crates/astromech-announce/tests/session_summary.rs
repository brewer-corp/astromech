//! Journal replay tests for the session summary (astromech-oo8y).

mod common;

use astromech_announce::session_summary::{SessionHistory, Totals};
use astromech_announce::*;
use common::Run;
use std::sync::{Arc, Mutex};

const LOAD_GAME: &str =
    r#"{"timestamp":"2026-10-16T19:00:00Z","event":"LoadGame","Commander":"Test","Ship":"krait_mkii","Credits":1}"#;
const MENU: &str = r#"{"timestamp":"2026-10-16T21:00:00Z","event":"Music","MusicTrack":"MainMenu"}"#;
const SHUTDOWN: &str = r#"{"timestamp":"2026-10-16T21:00:05Z","event":"Shutdown"}"#;
const NOW: i64 = 1_792_184_405; // 2026-10-16T21:00:05Z

fn totals(end: &str) -> Totals {
    Totals { end: end.into(), total: 4_200_000, per_hour: Some(1_900_000), jumps: 38, distance_ly: 1203.6, bounties: 6 }
}

const SAID: [&str; 3] =
    ["Earned 4.2 million credits, 1.9 million an hour.", "38 jumps, 1204 light years.", "6 bounties awarded."];

/// A history that holds the session up to `end` (what the follower has imported so far).
struct Fixture(Mutex<Option<Totals>>, Mutex<Vec<(Option<String>, Option<String>)>>);

impl SessionHistory for Fixture {
    fn session(&self, commander: Option<&str>, at: Option<&str>) -> Option<Totals> {
        self.1.lock().unwrap().push((commander.map(Into::into), at.map(Into::into)));
        self.0.lock().unwrap().clone()
    }
}

fn run(history: Option<Totals>) -> (Run, Arc<Fixture>) {
    let r = Run::new();
    let h = Arc::new(Fixture(Mutex::new(history), Mutex::default()));
    r.shared.sessions.set(h.clone());
    (r, h)
}

#[test]
fn shutdown_speaks_the_summary_on_the_next_tick_once() {
    let (mut r, h) = run(Some(totals("2026-10-16T21:00:05Z")));
    assert!(r.spoken(LOAD_GAME).is_empty());
    assert!(r.spoken(SHUTDOWN).is_empty(), "the history is asked on the tick, after the follower had its moment");
    assert_eq!(r.tick(NOW + 1), SAID);
    assert!(r.tick(NOW + 2).is_empty());
    // It asked about the session the commander was in when it ended.
    assert_eq!(h.1.lock().unwrap().as_slice(), [(Some("Test".into()), Some("2026-10-16T21:00:05Z".into()))]);
}

#[test]
fn the_menu_then_the_shutdown_say_it_once() {
    let (mut r, _) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.spoken(LOAD_GAME);
    r.spoken(MENU);
    assert_eq!(r.tick(NOW), SAID);
    r.spoken(SHUTDOWN);
    assert!(r.tick(NOW + 10).is_empty(), "already said for this session");
    // The next login is a new session.
    r.spoken(LOAD_GAME);
    r.spoken(MENU);
    assert_eq!(r.tick(NOW + 20), SAID);
}

#[test]
fn it_waits_for_the_history_to_hold_the_last_line_and_gives_up_after_a_while() {
    let (mut r, h) = run(Some(totals("2026-10-16T20:59:00Z")));
    r.spoken(LOAD_GAME);
    r.spoken(SHUTDOWN);
    assert!(r.tick(NOW).is_empty(), "the history ends before the Shutdown");
    assert!(r.tick(NOW + 2).is_empty());
    *h.0.lock().unwrap() = Some(totals("2026-10-16T21:00:05Z"));
    assert_eq!(r.tick(NOW + 4), SAID);

    // A history that never catches up is dropped, not asked forever.
    let (mut r, h) = run(None);
    r.spoken(LOAD_GAME);
    r.spoken(SHUTDOWN);
    assert!(r.tick(NOW).is_empty());
    assert!(r.tick(NOW + 31).is_empty());
    *h.0.lock().unwrap() = Some(totals("2026-10-16T21:00:05Z"));
    assert!(r.tick(NOW + 40).is_empty());
}

#[test]
fn a_session_with_nothing_to_report_says_nothing() {
    let quiet =
        Totals { total: 0, per_hour: None, jumps: 0, distance_ly: 0.0, bounties: 0, ..totals("2026-10-16T21:00:05Z") };
    let (mut r, _) = run(Some(quiet));
    r.spoken(LOAD_GAME);
    r.spoken(SHUTDOWN);
    assert!(r.tick(NOW).is_empty());
}

#[test]
fn the_catch_up_and_a_missing_history_are_silent() {
    let (mut r, _) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.engine.set_live(false);
    r.spoken(LOAD_GAME);
    r.spoken(SHUTDOWN);
    r.engine.set_live(true);
    assert!(r.tick(NOW).is_empty(), "a shutdown in the replayed journal is history");
    // No history plugged in.
    let mut r = Run::new();
    r.spoken(LOAD_GAME);
    r.spoken(SHUTDOWN);
    assert!(r.tick(NOW).is_empty());
}

#[test]
fn the_rule_can_be_switched_off() {
    let settings: Settings = toml::from_str("[session_summary]\nenabled = false\n").unwrap();
    let mut r = Run::with(Shared::default(), &settings);
    r.shared.sessions.set(Arc::new(Fixture(Mutex::new(Some(totals("2026-10-16T21:00:05Z"))), Mutex::default())));
    r.spoken(LOAD_GAME);
    r.spoken(SHUTDOWN);
    assert!(r.tick(NOW).is_empty());
}

#[test]
fn a_menu_before_the_login_of_a_fresh_launch_says_nothing() {
    // The history holds an earlier session of this commander, and the game state still names
    // him from the journal's `Commander` line when `MainMenu` comes before `LoadGame`.
    let (mut r, h) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.spoken(r#"{"timestamp":"2026-10-17T09:00:00Z","event":"Commander","Name":"Test","FID":"F1"}"#);
    r.state.commander = Some("Test".into());
    r.spoken(r#"{"timestamp":"2026-10-17T09:00:01Z","event":"Music","MusicTrack":"MainMenu"}"#);
    r.spoken(r#"{"timestamp":"2026-10-17T09:00:02Z","event":"Shutdown"}"#);
    assert!(r.tick(NOW + 40_000).is_empty());
    assert!(h.1.lock().unwrap().is_empty(), "nothing was even asked");
    // The login arms it: the session that follows ends with a summary.
    r.spoken(LOAD_GAME);
    r.spoken(MENU);
    assert_eq!(r.tick(NOW), SAID);
}

#[test]
fn a_start_in_the_middle_of_a_session_summarises_it_at_the_live_shutdown() {
    // astromech starts after the login: the catch-up holds the `LoadGame`, only the end is live.
    let (mut r, _) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.engine.set_live(false);
    r.spoken(LOAD_GAME);
    r.engine.set_live(true);
    assert!(r.tick(NOW - 5).is_empty());
    r.spoken(SHUTDOWN);
    assert_eq!(r.tick(NOW), SAID);
    assert!(r.tick(NOW + 1).is_empty(), "once");

    // Same when the end is the first live thing, with no tick before it.
    let (mut r, _) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.engine.set_live(false);
    r.spoken(LOAD_GAME);
    r.engine.set_live(true);
    r.spoken(SHUTDOWN);
    assert_eq!(r.tick(NOW), SAID);
}

#[test]
fn a_session_that_ended_before_astromech_started_is_not_summarised_again() {
    let (mut r, _) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.engine.set_live(false);
    r.spoken(LOAD_GAME);
    r.spoken(MENU);
    r.engine.set_live(true);
    r.spoken(SHUTDOWN);
    assert!(r.tick(NOW).is_empty());
}

#[test]
fn a_catch_up_that_ends_in_the_shutdown_leaves_nothing_to_summarise() {
    let (mut r, _) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.engine.set_live(false);
    r.spoken(LOAD_GAME);
    r.spoken(SHUTDOWN);
    r.engine.set_live(true);
    r.spoken(MENU);
    assert!(r.tick(NOW).is_empty());
}

#[test]
fn a_crashed_session_is_not_summarised_by_the_next_launch() {
    // The catch-up holds a session that never logged out (a crash), then the next run's file.
    let (mut r, h) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.engine.set_live(false);
    r.spoken(LOAD_GAME);
    r.spoken(r#"{"timestamp":"2026-10-17T09:00:00Z","event":"Fileheader","part":1,"gameversion":"4.0","build":"b"}"#);
    r.engine.set_live(true);
    r.spoken(r#"{"timestamp":"2026-10-17T09:00:01Z","event":"Music","MusicTrack":"MainMenu"}"#);
    assert!(r.tick(NOW + 40_000).is_empty());
    assert!(h.1.lock().unwrap().is_empty(), "nothing was even asked");

    // A rollover to part 2 is the same session.
    let (mut r, _) = run(Some(totals("2026-10-16T21:00:05Z")));
    r.engine.set_live(false);
    r.spoken(LOAD_GAME);
    r.spoken(r#"{"timestamp":"2026-10-16T20:00:00Z","event":"Fileheader","part":2,"gameversion":"4.0","build":"b"}"#);
    r.engine.set_live(true);
    r.spoken(SHUTDOWN);
    assert_eq!(r.tick(NOW), SAID);
}
