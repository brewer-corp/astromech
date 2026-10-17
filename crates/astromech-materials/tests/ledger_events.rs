//! astromech-ldiz: the lines the ledger replay reads and the lines a page watches for are one
//! list, `LEDGER_EVENTS`. These tests fail when an event the ledger reacts to is missing from it,
//! and when the replay reads lines the list doesn't name.

use astromech_materials::{ledger::events_from_last_snapshot, replay_lines, LEDGER_EVENTS};

/// One sample line for every event the ledger changes on (see `Ledger::apply` and the parser's
/// `MaterialDelta` events).
const SAMPLES: &[&str] = &[
    r#"{ "timestamp":"2026-10-16T12:00:01Z", "event":"Materials", "Raw":[{"Name":"iron","Count":50}], "Manufactured":[], "Encoded":[] }"#,
    r#"{ "timestamp":"2026-10-16T12:00:01Z", "event": "Materials", "Raw":[{"Name":"iron","Count":50}], "Manufactured":[], "Encoded":[] }"#,
    r#"{ "timestamp":"2026-10-16T12:01:00Z", "event":"MaterialCollected", "Category":"Raw", "Name":"iron", "Count":3 }"#,
    r#"{ "timestamp":"2026-10-16T12:02:00Z", "event":"MaterialDiscarded", "Category":"Raw", "Name":"iron", "Count":1 }"#,
    r#"{ "timestamp":"2026-10-16T12:03:00Z", "event":"MaterialTrade", "MarketID":1, "TraderType":"raw", "Paid":{"Material":"iron","Category":"Raw","Quantity":6}, "Received":{"Material":"vanadium","Category":"Raw","Quantity":1} }"#,
    r#"{ "timestamp":"2026-10-16T12:04:00Z", "event":"EngineerCraft", "Engineer":"Felicity Farseer", "BlueprintName":"FSD_LongRange", "Level":1, "Ingredients":[{"Name":"iron","Count":2}] }"#,
    r#"{ "timestamp":"2026-10-16T12:05:00Z", "event":"Synthesis", "Name":"FSD Injection Basic", "Materials":[{"Name":"iron","Count":1}] }"#,
    r#"{ "timestamp":"2026-10-16T12:06:00Z", "event":"TechnologyBroker", "BrokerType":"guardian", "Materials":[{"Name":"iron","Count":3,"Category":"Raw"}] }"#,
    r#"{ "timestamp":"2026-10-16T12:07:00Z", "event":"MissionCompleted", "MissionID":1, "MaterialsReward":[{"Name":"iron","Category":"Raw","Count":5}] }"#,
    r#"{ "timestamp":"2026-10-16T12:08:00Z", "event":"ScientificResearch", "Name":"iron", "Category":"Raw", "Count":1 }"#,
    r#"{ "timestamp":"2026-10-16T12:09:00Z", "event":"EngineerContribution", "Engineer":"Tod McQuinn", "Type":"Materials", "Material":"iron", "Category":"Raw", "Quantity":1, "TotalQuantity":1 }"#,
];

#[test]
fn every_event_the_ledger_reacts_to_is_in_the_watch_list() {
    let start = replay_lines(std::iter::once(SAMPLES[0]));
    for line in SAMPLES {
        assert!(LEDGER_EVENTS.iter().any(|e| line.contains(e)), "not in LEDGER_EVENTS: {line}");
        // On top of a snapshot, the line must move the ledger (the snapshot samples replace it).
        let l = replay_lines([SAMPLES[0], line]);
        if !line.contains("\"Materials\"") {
            assert_ne!(l, start, "the ledger ignores {line}");
        }
    }
}

#[test]
fn the_replay_reads_only_the_lines_the_watch_list_names() {
    let d = tempfile::tempdir().unwrap();
    let mut text = String::from(r#"{ "timestamp":"2026-10-16T12:00:00Z", "event":"Fileheader", "part":1 }"#);
    text.push('\n');
    text.push_str(r#"{ "timestamp":"2026-10-16T12:00:05Z", "event":"Music", "MusicTrack":"NoTrack" }"#);
    text.push('\n');
    for s in SAMPLES {
        text.push_str(s);
        text.push('\n');
    }
    std::fs::write(d.path().join("Journal.2026-10-16T120000.01.log"), text).unwrap();
    let lines = events_from_last_snapshot(d.path());
    assert!(!lines.is_empty());
    for l in &lines {
        assert!(LEDGER_EVENTS.iter().any(|e| l.raw_json.contains(e)), "read a line outside the list: {}", l.raw_json);
    }
}
