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

fn run_of(lines: &[&str]) -> MiningRun {
    let mut run = MiningRun::default();
    for l in lines {
        run.apply(&astromech_journal::parse_line(l).unwrap());
    }
    run
}

#[test]
fn only_prospector_and_collector_limpets_start_a_run() {
    for kind in ["Repair", "FuelTransfer", "Hatchbreaker", "Recon", "Research", "Decontamination"] {
        let line = format!(r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"LaunchDrone","Type":"{kind}"}}"#);
        let run = run_of(&[&line]);
        assert_eq!(run.started, None, "{kind} limpet started a run");
    }
    for kind in ["Prospector", "Collection"] {
        let line = format!(r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"LaunchDrone","Type":"{kind}"}}"#);
        assert!(run_of(&[&line]).started.is_some(), "{kind} limpet didn't start a run");
    }
}

#[test]
fn rate_runs_to_the_latest_journal_line_not_the_last_refine() {
    // 1 t refined a minute in; a later non-mining line at 20:10 is the newest time seen,
    // so the rate is 1 t over 10 minutes (6 t/h), not 1 t over one minute (60 t/h).
    let run = run_of(&[
        r#"{"timestamp":"2026-10-16T20:00:00Z","event":"LaunchDrone","Type":"Prospector"}"#,
        r#"{"timestamp":"2026-10-16T20:01:00Z","event":"MiningRefined","Type":"$platinum_name;"}"#,
        r#"{"timestamp":"2026-10-16T20:10:00Z","event":"Music","MusicTrack":"Exploration"}"#,
    ]);
    assert_eq!(run.last, DateTime::parse_from_rfc3339("2026-10-16T20:10:00Z").ok());
    assert_eq!(run.per_hour(), Some(6.0));
}
