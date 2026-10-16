//! astromech-txzq: material changes appended to the journal the game is still writing must reach
//! the Materials page within 2 s. The app window caches the inventory and recomputes it when
//! [`journal_version`] changes, then the page polls every second, so each append has to change
//! the version at once and the recompute has to stay well under a second.

use astromech_materials::{from_journal_dir, journal_version, Inventory};
use std::io::Write;
use std::time::{Duration, Instant};

/// Recompute budget: 2 s minus the page's 1 s poll.
const BUDGET: Duration = Duration::from_secs(1);

fn count(inv: &Inventory, id: &str) -> Option<i64> {
    inv.kinds.iter().flat_map(|k| &k.groups).flat_map(|g| &g.cells).find(|c| c.id == id).map(|c| c.count)
}

#[test]
fn appends_to_an_open_journal_show_up_within_budget() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("Journal.2026-10-16T120000.01.log");
    // Held open for the whole test, as the game holds the journal for the whole session.
    let mut game = std::fs::OpenOptions::new().create(true).append(true).open(&path).unwrap();
    writeln!(game, r#"{{ "timestamp":"2026-10-16T12:00:00Z", "event":"Fileheader", "part":1 }}"#).unwrap();
    writeln!(
        game,
        r#"{{ "timestamp":"2026-10-16T12:00:01Z", "event":"Materials", "Raw":[{{"Name":"iron","Count":50}},{{"Name":"vanadium","Count":10}}], "Manufactured":[], "Encoded":[] }}"#
    )
    .unwrap();
    game.flush().unwrap();
    let mut version = journal_version(d.path());
    assert_eq!(count(&from_journal_dir(d.path()), "iron"), Some(50));

    let events = [
        (
            r#"{ "timestamp":"2026-10-16T12:01:00Z", "event":"MaterialCollected", "Category":"Raw", "Name":"iron", "Count":3 }"#,
            53,
        ),
        (
            r#"{ "timestamp":"2026-10-16T12:02:00Z", "event":"MaterialDiscarded", "Category":"Raw", "Name":"iron", "Count":1 }"#,
            52,
        ),
        (
            r#"{ "timestamp":"2026-10-16T12:03:00Z", "event":"MaterialTrade", "MarketID":1, "TraderType":"raw", "Paid":{"Material":"iron","Category":"Raw","Quantity":6}, "Received":{"Material":"vanadium","Category":"Raw","Quantity":1} }"#,
            46,
        ),
        (
            r#"{ "timestamp":"2026-10-16T12:04:00Z", "event":"EngineerCraft", "Engineer":"Felicity Farseer", "BlueprintName":"FSD_LongRange", "Level":1, "Ingredients":[{"Name":"iron","Count":2}] }"#,
            44,
        ),
        (
            r#"{ "timestamp":"2026-10-16T12:05:00Z", "event":"Synthesis", "Name":"FSD Injection Basic", "Materials":[{"Name":"iron","Count":1}] }"#,
            43,
        ),
        (
            r#"{ "timestamp":"2026-10-16T12:06:00Z", "event":"TechnologyBroker", "BrokerType":"guardian", "Materials":[{"Name":"iron","Count":3,"Category":"Raw"}] }"#,
            40,
        ),
        (
            r#"{ "timestamp":"2026-10-16T12:07:00Z", "event":"MissionCompleted", "MissionID":1, "MaterialsReward":[{"Name":"iron","Category":"Raw","Count":5}] }"#,
            45,
        ),
        (
            r#"{ "timestamp":"2026-10-16T12:08:00Z", "event":"Materials", "Raw":[{"Name":"iron","Count":77}], "Manufactured":[], "Encoded":[] }"#,
            77,
        ),
    ];
    for (line, want) in events {
        writeln!(game, "{line}").unwrap();
        game.flush().unwrap();
        let t0 = Instant::now();
        let now = journal_version(d.path());
        assert_ne!(now, version, "journal_version missed an append to the open journal: {line}");
        version = now;
        assert_eq!(count(&from_journal_dir(d.path()), "iron"), Some(want), "{line}");
        assert!(t0.elapsed() < BUDGET, "recompute took {:?}: {line}", t0.elapsed());
    }
}

#[test]
fn version_follows_a_new_journal_file() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("Journal.2026-10-16T120000.01.log"), "{}\n").unwrap();
    let a = journal_version(d.path());
    std::fs::write(d.path().join("Journal.2026-10-16T130000.01.log"), "{}\n").unwrap();
    assert_ne!(journal_version(d.path()), a);
}
