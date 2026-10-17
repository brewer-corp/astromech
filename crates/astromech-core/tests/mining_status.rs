//! The mining run in the core's status reply: a synthetic mining session fed through the hub
//! gives the JSON shape the window's Mining page reads. The window's sample
//! (`app/src/lib/fixtures/mining.json`) must be the same JSON.

use astromech_core::hub::StateHub;
use serde_json::Value;

#[test]
fn mining_run_status_shape_matches_the_window_sample() {
    let hub = StateHub::new();
    // Before any mining there is nothing to show.
    assert_eq!(hub.mining_json()["total"], 0);
    assert_eq!(hub.mining_json()["started"], Value::Null);
    for l in include_str!("../../astromech-gamestate/tests/fixtures/mining.log").lines() {
        hub.apply_journal(&astromech_journal::parse_line(l).unwrap());
    }
    let sample: Value = serde_json::from_str(include_str!("../../../app/src/lib/fixtures/mining.json")).unwrap();
    assert_eq!(hub.mining_json(), sample);
    // 9 t from 20:00 to the last line at 20:30.
    assert_eq!(sample["per_hour"], 18.0);
}
