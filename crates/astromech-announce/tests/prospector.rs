//! Journal replay tests for the prospector callouts.

use astromech_announce::prospector::{SharedThresholds, Thresholds};
use astromech_announce::*;
use astromech_gamestate::GameState;
use astromech_journal::parse_line;
use std::time::{Duration, Instant};

fn prospected(ts: &str, materials: &[(&str, f64)], motherlode: Option<&str>) -> String {
    let m: Vec<String> = materials.iter().map(|(n, p)| format!(r#"{{"Name":"{n}","Proportion":{p}}}"#)).collect();
    let ml = motherlode.map(|m| format!(r#","MotherlodeMaterial":"{m}""#)).unwrap_or_default();
    format!(
        r#"{{"timestamp":"2026-10-16T{ts}Z","event":"ProspectedAsteroid","Materials":[{}],"Content":"$AsteroidMaterialContent_High;"{ml},"Remaining":100.0}}"#,
        m.join(",")
    )
}

struct Run {
    engine: Engine,
    thresholds: SharedThresholds,
    t0: Instant,
    n: u64,
}

impl Run {
    fn new() -> Self {
        let thresholds = SharedThresholds::default();
        let mut engine = Engine::new(builtin(thresholds.clone()), &Settings::default());
        engine.go_live();
        Self { engine, thresholds, t0: Instant::now(), n: 0 }
    }

    /// Spoken lines for one journal line.
    fn feed(&mut self, line: &str) -> Vec<String> {
        let j = parse_line(line).unwrap();
        self.n += 1;
        let out = self.engine.journal(&j, &GameState::default(), self.t0 + Duration::from_secs(self.n));
        out.into_iter().filter(|r| r.channels.speech).map(|r| r.text).collect()
    }
}

#[test]
fn a_rock_above_threshold_gets_one_short_line() {
    let mut r = Run::new();
    let said = r.feed(&prospected("20:00:00", &[("Platinum", 34.2), ("Bauxite", 40.0)], None));
    assert_eq!(said, ["Platinum 34%."]);
    // Symbols match whatever their case; the next rock seconds later is called out too.
    let said = r.feed(&prospected("20:00:04", &[("lowtemperaturediamond", 26.0), ("painite", 31.5)], None));
    assert_eq!(said, ["Low Temperature Diamonds 26%. Painite 31%."]);
}

#[test]
fn a_rock_below_threshold_gets_nothing() {
    let mut r = Run::new();
    assert!(r.feed(&prospected("20:00:00", &[("Platinum", 19.9), ("Painite", 10.0)], None)).is_empty());
    // Ores that aren't high value are never called out, however rich.
    assert!(r.feed(&prospected("20:00:01", &[("Bauxite", 90.0), ("Gold", 60.0)], None)).is_empty());
}

#[test]
fn a_motherlode_always_gets_a_line() {
    let mut r = Run::new();
    assert_eq!(r.feed(&prospected("20:00:00", &[("Bauxite", 12.0)], Some("Opal"))), ["Motherlode. Void Opals."]);
    // Even with every surface ore turned off, and with a surface ore that is above threshold.
    for o in astromech_announce::prospector::ORES {
        r.thresholds.write().unwrap().set(o.symbol, None).unwrap();
    }
    assert_eq!(
        r.feed(&prospected("20:00:01", &[("Platinum", 50.0)], Some("Alexandrite"))),
        ["Motherlode. Alexandrite."]
    );
    r.thresholds.write().unwrap().reset("platinum");
    assert_eq!(
        r.feed(&prospected("20:00:02", &[("Platinum", 50.0)], Some("Musgravite"))),
        ["Motherlode. Musgravite. Platinum 50%."]
    );
}

#[test]
fn threshold_changes_from_settings_apply_at_once() {
    let mut r = Run::new();
    let rock = prospected("20:00:00", &[("Platinum", 34.0)], None);
    r.thresholds.write().unwrap().set("platinum", Some(40.0)).unwrap();
    assert!(r.feed(&rock).is_empty());
    *r.thresholds.write().unwrap() = Thresholds::default();
    assert_eq!(r.feed(&rock), ["Platinum 34%."]);
}
