//! Replay a docking (journal + `Outfitting.json`) the way the core does, then ask what is here
//! and, from a book reopened later, where a module was seen.

use astromech_gamestate::GameState;
use astromech_journal::companion::{load, triggers, CompanionFile, OutfittingSnap};
use astromech_station::{state_from_journal_dir, DockedHere, OutfittingBook, DB_FILE};
use std::path::{Path, PathBuf};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dock")
}

/// A game folder holding the docking fixture: its journal and the `Outfitting.json` written then.
fn game_dir() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    std::fs::copy(
        fixtures().join("Journal.2026-10-10T200000.01.log"),
        d.path().join("Journal.2026-10-10T200000.01.log"),
    )
    .unwrap();
    std::fs::copy(fixtures().join("Outfitting.json"), d.path().join("Outfitting.json")).unwrap();
    d
}

/// The `Outfitting.json` the journal's events announce, if fresh: what the core's pipeline
/// reads and hands to the book.
fn fresh_outfitting(dir: &Path) -> Vec<OutfittingSnap> {
    let lines = astromech_journal::parse_file(&astromech_journal::journal_logs(dir)[0]);
    let (files, reports) = load(dir, &triggers(&lines));
    assert_eq!(reports, vec![]);
    files.into_iter().filter_map(|f| if let CompanionFile::Outfitting(o) = f { Some(o) } else { None }).collect()
}

fn other_port() -> OutfittingSnap {
    // The file is announced by an event at its own time.
    let text = std::fs::read_to_string(fixtures().join("Outfitting-other.json")).unwrap();
    let CompanionFile::Outfitting(o) = astromech_journal::companion::parse_fresh(
        astromech_journal::companion::CompanionKind::Outfitting,
        Some(text.as_bytes()),
        "2026-10-12T09:30:00Z",
    )
    .unwrap() else {
        panic!("not outfitting")
    };
    o
}

#[test]
fn replaying_a_docking_shows_the_facts() {
    let dir = game_dir();
    let mut book = OutfittingBook::in_memory().unwrap();
    let snaps = fresh_outfitting(dir.path());
    assert_eq!(snaps.len(), 1);
    assert!(book.record(&snaps[0]).unwrap());

    let state = state_from_journal_dir(dir.path());
    let fleet = astromech_engineering::from_journal_dir(dir.path());
    let ship = fleet.ships.iter().find(|s| s.current);
    let here = DockedHere::new(&state, ship, Some(&book));

    assert!(here.docked);
    assert_eq!(
        (here.station.as_deref(), here.system.as_deref(), here.market_id),
        (Some("Test Orbital"), Some("Test System"), Some(128000001))
    );
    assert_eq!(here.info.station_type.as_deref(), Some("Coriolis"));
    assert!(["outfitting", "shipyard", "refuel", "engineer"].iter().all(|s| here.info.services.iter().any(|x| x == s)));
    let eco: Vec<_> = here.info.economies.iter().map(|e| (e.name_localised.clone().unwrap(), e.proportion)).collect();
    assert_eq!(eco, [("High Tech".to_string(), 0.7), ("Refinery".to_string(), 0.3)]);
    let f = here.info.faction.as_ref().unwrap();
    assert_eq!((f.name.as_str(), f.state.as_deref()), ("Test Holdings", Some("Boom")));
    assert_eq!(here.info.government.as_deref(), Some("Corporate"));
    let pads = here.info.landing_pads.unwrap();
    assert_eq!((pads.small, pads.medium, pads.large, here.max_pad), (4, 6, 3, Some("Large")));
    assert_eq!(here.ship.as_deref(), Some("Synthetic"));

    // The build has a 5C FSD, a 5C shield generator and a cargo rack. The station sells a 5A and
    // a 5B FSD (the 5A is the upgrade), a 4A FSD (the wrong size) and no shield generator.
    let out = here.outfitting.expect("outfitting seen");
    assert_eq!((out.items, out.seen.as_str()), (5, "2026-10-10T20:10:06Z"));
    assert_eq!(out.upgrades.len(), 1, "{:?}", out.upgrades);
    let u = &out.upgrades[0];
    assert_eq!(
        (u.slot_name.as_str(), u.fitted.as_str(), u.name.as_str(), u.price),
        ("Frame shift drive", "5C Frame Shift Drive", "5A Frame Shift Drive", 9_000_000)
    );
}

#[test]
fn no_facts_when_not_docked() {
    let mut state = GameState::default();
    for l in astromech_journal::parse_file(&fixtures().join("Journal.2026-10-10T200000.01.log")) {
        state.apply_journal(&l.event);
    }
    assert!(DockedHere::new(&state, None, None).docked);
    let undock = astromech_journal::parse_line(
        r#"{"timestamp":"2026-10-10T21:00:00Z","event":"Undocked","StationName":"Test Orbital"}"#,
    )
    .unwrap();
    state.apply_journal(&undock.event);
    // Only the system is kept, so the page can still link it.
    let here = DockedHere::new(&state, None, None);
    assert_eq!(here.system.as_deref(), Some("Test System"));
    assert_eq!(here, DockedHere { system: here.system.clone(), ..DockedHere::default() });
}

#[test]
fn a_later_offline_query_returns_the_station() {
    let dir = game_dir();
    let data = tempfile::tempdir().unwrap();
    let path = data.path().join(DB_FILE);
    {
        let mut book = OutfittingBook::open(&path).unwrap();
        book.record(&fresh_outfitting(dir.path())[0]).unwrap();
    }
    // The game has overwritten Outfitting.json with another station's, or no journal is left.
    drop(dir);

    let book = OutfittingBook::open_read_only(&path).unwrap().expect("book on disk");
    let hits = book.find("5a frame shift", 10).unwrap();
    assert_eq!(hits.len(), 1);
    let h = &hits[0];
    assert_eq!(
        (h.station_name.as_str(), h.star_system.as_str(), h.market_id),
        ("Test Orbital", "Test System", 128000001)
    );
    assert_eq!(
        (h.seen.as_str(), h.item.as_str(), h.name.as_str(), h.price),
        ("2026-10-10T20:10:06Z", "int_hyperdrive_size5_class5", "5A Frame Shift Drive", 9_000_000)
    );

    // By symbol, by the short name players use, and by size and rating alone.
    assert_eq!(book.find("hyperdrive_size5", 10).unwrap().len(), 2);
    assert_eq!(book.find("fsd 4a", 10).unwrap().len(), 1);
    assert_eq!(book.find("PULSE laser", 10).unwrap().len(), 1);
    // Nothing sells this; wildcard characters are text, not patterns.
    assert!(book.find("frame shift 1a", 10).unwrap().is_empty());
    assert!(book.find("%", 10).unwrap().is_empty());
    assert!(book.find("   ", 10).unwrap().is_empty());
}

#[test]
fn each_visit_is_kept_and_the_newest_list_answers() {
    let dir = game_dir();
    let mut book = OutfittingBook::in_memory().unwrap();
    let first = fresh_outfitting(dir.path()).remove(0);
    assert!(book.record(&first).unwrap());
    assert!(!book.record(&first).unwrap(), "the same file again is not a new snapshot");

    // A later visit to the same station: the FSDs are gone, the lasers stay.
    let mut later = first.clone();
    later.timestamp = "2026-10-11T08:00:00Z".into();
    later.items.retain(|i| i.name.contains("pulselaser"));
    later.raw_json = later.raw_json.replace("2026-10-10T20:10:06Z", "2026-10-11T08:00:00Z");
    assert!(book.record(&later).unwrap());
    assert!(book.find("frame shift", 10).unwrap().is_empty());
    assert_eq!(book.find("pulse", 10).unwrap()[0].seen, "2026-10-11T08:00:00Z");

    // A file older than the list is kept but doesn't replace it.
    let mut older = first.clone();
    older.timestamp = "2026-10-09T08:00:00Z".into();
    assert!(book.record(&older).unwrap());
    assert!(book.find("frame shift", 10).unwrap().is_empty());

    assert_eq!(book.counts().unwrap(), (1, 3));
    let raw = book.raw_snapshots(128000001).unwrap();
    assert_eq!(
        raw.iter().map(|(ts, _)| ts.as_str()).collect::<Vec<_>>(),
        ["2026-10-09T08:00:00Z", "2026-10-10T20:10:06Z", "2026-10-11T08:00:00Z"]
    );
    assert_eq!(raw[1].1, std::fs::read_to_string(fixtures().join("Outfitting.json")).unwrap());
}

#[test]
fn most_recently_seen_station_first() {
    let dir = game_dir();
    let mut book = OutfittingBook::in_memory().unwrap();
    book.record(&other_port()).unwrap();
    book.record(&fresh_outfitting(dir.path())[0]).unwrap();
    let names: Vec<_> = book.find("5a frame shift", 10).unwrap().into_iter().map(|h| h.station_name).collect();
    assert_eq!(names, ["Other Port", "Test Orbital"]);
}
