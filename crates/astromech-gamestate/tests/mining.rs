//! A synthetic mining session: refined count and tons per hour.

use astromech_gamestate::mining::MiningRun;
use chrono::DateTime;

#[test]
fn mining_fixture_gives_refined_count_and_tons_per_hour() {
    let mut run = MiningRun::default();
    for l in include_str!("fixtures/mining.log").lines() {
        run.apply(&astromech_journal::parse_line(l).unwrap());
    }
    // Nine MiningRefined lines: seven platinum, two painite. The run starts at the first
    // prospector launch (20:00), not at the earlier dock.
    assert_eq!(run.total(), 9);
    assert_eq!((run.refined["platinum"], run.refined["painite"]), (7, 2));
    assert_eq!(run.started, DateTime::parse_from_rfc3339("2026-10-16T20:00:00Z").ok());
    // 9 t in 30 minutes = 18 t/h.
    let at = DateTime::parse_from_rfc3339("2026-10-16T20:30:00Z").unwrap();
    assert_eq!(run.tons_per_hour(at), Some(18.0));
    // Under a minute in: no rate yet.
    let early = DateTime::parse_from_rfc3339("2026-10-16T20:00:30Z").unwrap();
    assert_eq!(run.tons_per_hour(early), None);
    // Docking (selling) starts over.
    run.apply(
        &astromech_journal::parse_line(
            r#"{"timestamp":"2026-10-16T20:40:00Z","event":"Docked","StationName":"S","StarSystem":"X","MarketID":1}"#,
        )
        .unwrap(),
    );
    assert_eq!((run.total(), run.started), (0, None));
}
