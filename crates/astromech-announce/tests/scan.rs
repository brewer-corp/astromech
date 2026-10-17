//! Journal replay tests for the being-scanned callout.

use astromech_announce::*;
use astromech_gamestate::GameState;
use astromech_journal::{parse_cargo, parse_line};
use std::time::{Duration, Instant};

const CLEAN_CARGO: &[u8] = br#"{"timestamp":"2026-10-16T20:00:00Z","event":"Cargo","Vessel":"Ship","Count":8,"Inventory":[{"Name":"gold","Name_Localised":"Gold","Count":8,"Stolen":0}]}"#;
const STOLEN_CARGO: &[u8] = br#"{"timestamp":"2026-10-16T20:00:00Z","event":"Cargo","Vessel":"Ship","Count":8,"Inventory":[{"Name":"gold","Name_Localised":"Gold","Count":6,"Stolen":0},{"Name":"progenitorcells","Name_Localised":"Progenitor Cells","Count":2,"Stolen":2}]}"#;

fn scanned(ts: &str, kind: &str) -> String {
    format!(r#"{{"timestamp":"2026-10-16T{ts}Z","event":"Scanned","ScanType":"{kind}"}}"#)
}

struct Run {
    engine: Engine,
    state: GameState,
    t0: Instant,
    n: u64,
}

impl Run {
    /// `cooldown_s`: the scan rule's cooldown for this run.
    fn new(cargo: &[u8], cooldown_s: Option<f64>) -> Self {
        let settings = Settings([("scan".to_string(), RuleSettings { cooldown_s, ..Default::default() })].into());
        let mut engine = Engine::new(builtin(Default::default()), &settings);
        engine.set_live(true);
        let mut state = GameState::default();
        state.apply_cargo(&parse_cargo(cargo).unwrap());
        Self { engine, state, t0: Instant::now(), n: 0 }
    }

    /// (text, spoken, priority) for one journal line, `secs` after the start.
    fn feed_at(&mut self, line: &str, secs: u64) -> Vec<(String, bool, Priority)> {
        let j = parse_line(line).unwrap();
        self.state.apply_line(&j);
        let out = self.engine.journal(&j, &self.state, self.t0 + Duration::from_secs(secs));
        out.into_iter().map(|r| (r.text, r.channels.speech, r.priority)).collect()
    }

    fn feed(&mut self, line: &str) -> Vec<(String, bool, Priority)> {
        self.n += 1;
        self.feed_at(line, self.n)
    }

    fn spoken(&mut self, line: &str) -> Vec<String> {
        self.feed(line).into_iter().filter(|o| o.1).map(|o| o.0).collect()
    }
}

#[test]
fn each_scan_type_gives_its_line() {
    let mut r = Run::new(CLEAN_CARGO, Some(0.0));
    for (kind, line) in [
        ("Cargo", "Cargo scan."),
        ("Crime", "Crime scan."),
        ("Cabin", "Cabin scan."),
        ("Data", "Data scan."),
        ("Xeno", "Scan."),
    ] {
        assert_eq!(r.spoken(&scanned("20:00:00", kind)), [line], "{kind}");
    }
}

#[test]
fn the_stolen_goods_suffix_only_with_stolen_cargo() {
    let mut clean = Run::new(CLEAN_CARGO, Some(0.0));
    assert_eq!(clean.spoken(&scanned("20:00:00", "Cargo")), ["Cargo scan."]);
    let mut stolen = Run::new(STOLEN_CARGO, Some(0.0));
    assert_eq!(stolen.spoken(&scanned("20:00:00", "Cargo")), ["Cargo scan. Stolen goods aboard."]);
    // Only a cargo scan finds cargo.
    assert_eq!(stolen.spoken(&scanned("20:00:01", "Crime")), ["Crime scan."]);
    // Selling or jettisoning the stolen goods (a new Cargo.json) drops the suffix.
    stolen.state.apply_cargo(&parse_cargo(CLEAN_CARGO).unwrap());
    assert_eq!(stolen.spoken(&scanned("20:00:02", "Cargo")), ["Cargo scan."]);
}

#[test]
fn stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown() {
    let mut r = Run::new(STOLEN_CARGO, None);
    let granted = r#"{"timestamp":"2026-10-16T20:00:00Z","event":"DockingGranted","StationName":"S","LandingPad":7,"MarketID":1}"#;
    r.feed_at(granted, 0);
    let out = r.feed_at(&scanned("20:00:05", "Cargo"), 5);
    assert_eq!(out, [("Cargo scan. Stolen goods aboard.".to_string(), true, Priority::Urgent)]);
    // A second ship scans within 30 s: nothing.
    assert!(r.feed_at(&scanned("20:00:20", "Cargo"), 20).is_empty());
    // A clean ship's scan during docking is shown, not spoken.
    let mut clean = Run::new(CLEAN_CARGO, None);
    clean.feed_at(granted, 0);
    assert_eq!(clean.feed_at(&scanned("20:00:05", "Cargo"), 5), [("Cargo scan.".to_string(), false, Priority::Normal)]);
}

#[test]
fn a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line() {
    // On approach: station security runs a crime scan (shown, not spoken, during docking),
    // then a cargo scan seconds later finds the stolen goods.
    let mut r = Run::new(STOLEN_CARGO, None);
    let granted = r#"{"timestamp":"2026-10-16T20:00:00Z","event":"DockingGranted","StationName":"S","LandingPad":7,"MarketID":1}"#;
    r.feed_at(granted, 0);
    assert_eq!(r.feed_at(&scanned("20:00:03", "Crime"), 3), [("Crime scan.".to_string(), false, Priority::Normal)]);
    assert_eq!(
        r.feed_at(&scanned("20:00:06", "Cargo"), 6),
        [("Cargo scan. Stolen goods aboard.".to_string(), true, Priority::Urgent)]
    );
    // The urgent line starts its own cooldown: a lower-priority scan right after stays quiet.
    assert!(r.feed_at(&scanned("20:00:09", "Crime"), 9).is_empty());
}
