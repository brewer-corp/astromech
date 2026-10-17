//! astromech-ztrb: what the Engineering page returns (`engineering_fleet`,
//! `engineering_engineers`, and the material ledger behind build plans) must follow events
//! appended to a journal the game still holds open, without a rebuild for events nothing reads.
//! This drives `JournalCache` with the same readers and watched lines as the app window.

use astromech_engineering::{engineers, from_journal_dir, EngineerView, Fleet, FLEET_EVENTS};
use astromech_journal::JournalCache;
use astromech_materials::{replay_journal_dir, Ledger, LEDGER_EVENTS};
use std::io::Write;
use std::sync::Arc;

struct State {
    fleet: Fleet,
    engineers: Vec<EngineerView>,
    ledger: Ledger,
}

fn build(dir: &std::path::Path) -> State {
    State { fleet: from_journal_dir(dir), engineers: engineers::from_journal_dir(dir), ledger: replay_journal_dir(dir) }
}

fn progress(s: &State, name: &str) -> String {
    s.engineers.iter().find(|e| e.name == name).unwrap().progress.clone()
}

#[test]
fn appends_to_an_open_journal_reach_the_pages_on_the_next_call() {
    let d = tempfile::tempdir().unwrap();
    let mut game = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(d.path().join("Journal.2026-10-16T120000.01.log"))
        .unwrap();
    let mut put = |line: &str| {
        writeln!(game, "{line}").unwrap();
        game.flush().unwrap();
    };
    put(r#"{ "timestamp":"2026-10-16T12:00:00Z", "event":"Fileheader", "part":1 }"#);
    put(
        r#"{ "timestamp":"2026-10-16T12:00:01Z", "event":"Materials", "Raw":[{"Name":"iron","Count":50}], "Manufactured":[], "Encoded":[] }"#,
    );

    let events: Vec<&str> = [FLEET_EVENTS, engineers::EVENTS, LEDGER_EVENTS].concat();
    let cache = JournalCache::<State>::new(&events);
    let get = || cache.get(d.path(), || build(d.path()));
    let first = get();
    assert!(first.fleet.ships.is_empty());
    assert_eq!(progress(&first, "Felicity Farseer"), "Unknown");
    assert_eq!(first.ledger.counts.get("iron"), Some(&50));

    // Events no Engineering reader looks at keep the cached state.
    for noise in [
        r#"{ "timestamp":"2026-10-16T12:00:30Z", "event":"Music", "MusicTrack":"NoTrack" }"#,
        r#"{ "timestamp":"2026-10-16T12:00:31Z", "event":"ReceiveText", "From":"Cmdr X", "Message":"o7", "Channel":"local" }"#,
        r#"{ "timestamp":"2026-10-16T12:00:32Z", "event":"FuelScoop", "Scooped":1.5, "Total":16.0 }"#,
    ] {
        put(noise);
    }
    assert!(Arc::ptr_eq(&first, &get()), "an event no page reads rebuilt the state");

    put(
        r#"{ "timestamp":"2026-10-16T12:01:00Z", "event":"Loadout", "Ship":"python", "ShipID":7, "ShipName":"", "ShipIdent":"", "Modules":[] }"#,
    );
    let after_loadout = get();
    assert_eq!(after_loadout.fleet.ships.len(), 1, "Loadout did not reach engineering_fleet");

    put(
        r#"{ "timestamp":"2026-10-16T12:02:00Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked", "RankProgress":0, "Rank":1 }"#,
    );
    let after_progress = get();
    assert_eq!(
        progress(&after_progress, "Felicity Farseer"),
        "Unlocked",
        "EngineerProgress did not reach engineering_engineers"
    );

    put(
        r#"{ "timestamp":"2026-10-16T12:03:00Z", "event":"EngineerCraft", "Engineer":"Felicity Farseer", "EngineerID":300100, "BlueprintName":"FSD_LongRange", "Level":1, "Ingredients":[{"Name":"iron","Count":2}] }"#,
    );
    let after_craft = get();
    assert_eq!(after_craft.ledger.counts.get("iron"), Some(&48), "EngineerCraft did not reach the material ledger");
    assert!(!Arc::ptr_eq(&after_progress, &after_craft));
}
