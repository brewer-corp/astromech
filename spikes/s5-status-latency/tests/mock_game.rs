//! End-to-end tests of the measurement loop against a fake game that rewrites Status.json.

use s5_status_latency::binds::Chord;
use s5_status_latency::keys::ScanKey;
use s5_status_latency::platform::mock::MockGame;
use s5_status_latency::platform::Platform;
use s5_status_latency::run::{self, Config, KeyGuard, Session, Stop};
use s5_status_latency::status;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const L: ScanKey = ScanKey { scan: 0x26, extended: false };
const LALT: ScanKey = ScanKey { scan: 0x38, extended: false };

fn cfg(dir: &std::path::Path, modifiers: Vec<(String, ScanKey)>) -> Config {
    Config {
        action: "ShipSpotLightToggle".into(),
        bit: status::FLAG_LIGHTS,
        chord: Chord { source: "Primary".into(), key_name: "Key_L".into(), key: L, modifiers, notes: vec![] },
        status_path: dir.join("Status.json"),
        watch_dir: dir.to_path_buf(),
        timeout: Duration::from_millis(800),
        gap_ms: (50, 80),
        poll_interval: Duration::from_millis(2),
        focus_giveup: Duration::from_millis(300),
        require_no_panel: true,
        context_check: run::context_ship_only,
        quiet: true,
    }
}

#[test]
fn measures_toggle_latency_with_both_methods() {
    let d = tempfile::tempdir().unwrap();
    let game = MockGame::new(d.path().join("Status.json"), L, status::FLAG_LIGHTS, status::FLAG_IN_MAIN_SHIP, Duration::from_millis(40));
    let c = cfg(d.path(), vec![("Key_LeftAlt".into(), LALT)]);
    let cancel = Arc::new(AtomicBool::new(false));
    let mut s = Session::new(&c, &game, cancel);
    std::thread::sleep(Duration::from_millis(100)); // let the watcher settle
    let mut rows = vec![];
    for i in 0..4 {
        let r = s.trial(30).expect("trial");
        assert_eq!(r.expected_on, i % 2 == 0, "trial {i} should toggle");
        rows.push(r);
        s.gap().unwrap();
    }
    for r in &rows {
        let p = r.poll_ms.expect("poll detected");
        let n = r.notify_ms.expect("notify detected");
        assert!((40.0..400.0).contains(&p), "poll {p}");
        assert!((40.0..400.0).contains(&n), "notify {n}");
        assert!(r.actual_hold_ms >= 29.0, "hold {}", r.actual_hold_ms);
    }
    // Back to the original state after an even number of trials.
    assert_eq!(game.current_flags() & status::FLAG_LIGHTS, 0);
    // Only the chosen chord was pressed, modifiers wrap the key, nothing left down.
    let log = game.log.lock().unwrap();
    for chunk in log.chunks(4) {
        assert_eq!(chunk, &[(LALT, false), (L, false), (L, true), (LALT, true)]);
    }
    let csv = d.path().join("out.csv");
    run::write_csv(&csv, "ShipSpotLightToggle", &rows).unwrap();
    let text = std::fs::read_to_string(csv).unwrap();
    assert!(text.starts_with("trial,action,expected_state,poll_ms,notify_ms,missed"));
    assert_eq!(text.lines().count(), 5);
}

#[test]
fn ignored_press_is_a_miss() {
    let d = tempfile::tempdir().unwrap();
    let game = MockGame::new(d.path().join("Status.json"), L, status::FLAG_LIGHTS, status::FLAG_IN_MAIN_SHIP, Duration::from_millis(20));
    game.drop_presses.store(1, Ordering::SeqCst);
    let c = cfg(d.path(), vec![]);
    let mut s = Session::new(&c, &game, Arc::new(AtomicBool::new(false)));
    let r = s.trial(30).unwrap();
    assert!(r.missed());
    let r2 = s.trial(30).unwrap();
    assert!(!r2.missed());
    assert!(r2.expected_on); // state is re-read, so the missed trial does not desync expectations
}

#[test]
fn never_presses_without_focus_and_gives_up() {
    let d = tempfile::tempdir().unwrap();
    let game = MockGame::new(d.path().join("Status.json"), L, status::FLAG_LIGHTS, status::FLAG_IN_MAIN_SHIP, Duration::from_millis(20));
    game.focused.store(false, Ordering::SeqCst);
    let c = cfg(d.path(), vec![]);
    let mut s = Session::new(&c, &game, Arc::new(AtomicBool::new(false)));
    let e = s.trial(30).unwrap_err();
    assert!(matches!(e, Stop::FocusLost));
    assert!(game.log.lock().unwrap().is_empty(), "pressed a key without focus");
}

#[test]
fn waits_while_a_panel_is_open() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("Status.json");
    let game = MockGame::new(path.clone(), L, status::FLAG_LIGHTS, status::FLAG_IN_MAIN_SHIP, Duration::from_millis(20));
    let json = status::parse_status(s5_status_latency::platform::mock::status_json(status::FLAG_IN_MAIN_SHIP).as_bytes()).unwrap();
    assert_eq!(json.gui_focus, Some(0));
    std::fs::write(&path, s5_status_latency::platform::mock::status_json(status::FLAG_IN_MAIN_SHIP).replace("\"GuiFocus\":0", "\"GuiFocus\":6")).unwrap();
    let c = cfg(d.path(), vec![]);
    let mut s = Session::new(&c, &game, Arc::new(AtomicBool::new(false)));
    assert!(matches!(s.trial(30).unwrap_err(), Stop::FocusLost));
    assert!(game.log.lock().unwrap().is_empty());
}

#[test]
fn refuses_outside_ship_and_gear_rules() {
    assert!(run::context_ship_only(0).is_err());
    assert!(run::context_ship_only(status::FLAG_IN_MAIN_SHIP).is_ok());
    let ship = status::FLAG_IN_MAIN_SHIP;
    assert!(run::context_gear(ship).is_ok());
    assert!(run::context_gear(ship | status::FLAG_DOCKED).is_err());
    assert!(run::context_gear(ship | status::FLAG_LANDED).is_err());
    assert!(run::context_gear(ship | status::FLAG_SUPERCRUISE).is_err());
    assert!(run::context_ship_or_srv(status::FLAG_IN_SRV).is_ok());
}

#[test]
fn cancel_stops_before_pressing() {
    let d = tempfile::tempdir().unwrap();
    let game = MockGame::new(d.path().join("Status.json"), L, status::FLAG_LIGHTS, status::FLAG_IN_MAIN_SHIP, Duration::from_millis(20));
    let c = cfg(d.path(), vec![]);
    let cancel = Arc::new(AtomicBool::new(true));
    let mut s = Session::new(&c, &game, cancel);
    assert!(matches!(s.trial(30).unwrap_err(), Stop::Cancelled));
    assert!(game.log.lock().unwrap().is_empty());
}

#[test]
fn key_guard_releases_held_keys_on_drop() {
    let d = tempfile::tempdir().unwrap();
    let game = MockGame::new(d.path().join("Status.json"), L, 0, 0, Duration::from_millis(1));
    {
        let mut g = KeyGuard::new(&game as &dyn Platform);
        g.down(LALT).unwrap();
        g.down(L).unwrap();
        // simulated early exit: guard dropped with keys down
    }
    let log = game.log.lock().unwrap();
    assert_eq!(*log, vec![(LALT, false), (L, false), (L, true), (LALT, true)]);
}

#[test]
fn status_parsing_tolerates_partial_and_menu_files() {
    assert!(status::parse_status(b"{ \"timestamp\":\"x\", \"event\":\"Status\", \"Fla").is_none());
    assert!(status::parse_status(b"{ \"timestamp\":\"2026-10-08T22:40:00Z\", \"event\":\"Status\" }").is_none());
    let s = status::parse_status(b"{\"Flags\":16842765,\"GuiFocus\":0}").unwrap();
    assert_eq!(s.flags, 16842765);
}

#[test]
fn fileheader_build_is_read() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(
        d.path().join("Journal.2026-10-08T183000.01.log"),
        "{ \"timestamp\":\"2026-10-08T22:30:00Z\", \"event\":\"Fileheader\", \"part\":1, \"language\":\"English/UK\", \"Odyssey\":true, \"gameversion\":\"4.2.1.0\", \"build\":\"r999999/r0 \" }\r\n",
    )
    .unwrap();
    let b = status::latest_fileheader(d.path()).unwrap();
    assert_eq!(b.gameversion.as_deref(), Some("4.2.1.0"));
    assert_eq!(b.build.as_deref(), Some("r999999/r0"));
}
