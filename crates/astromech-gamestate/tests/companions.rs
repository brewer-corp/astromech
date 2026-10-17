//! Replay a journal against companion files: each event leaves its file's snapshot in the game
//! state; a file that is missing, half-written or older than its event is reported, not used.

use astromech_gamestate::GameState;
use astromech_journal::companion::{load, read_companion, triggers, CompanionIssue, CompanionKind, CompanionReport};
use astromech_journal::{parse_line, JournalLine};
use std::path::{Path, PathBuf};

const JOURNAL: &str = include_str!("fixtures/companion-journal.jsonl");

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/companions")
}

fn lines(journal: &str) -> Vec<JournalLine> {
    journal.lines().map(|l| parse_line(l).unwrap()).collect()
}

/// What the app does, minus the file watcher: fold the events, then read the companion files
/// their events announced.
fn replay(journal: &str, dir: &Path) -> (GameState, Vec<CompanionReport>) {
    let lines = lines(journal);
    let mut g = GameState::default();
    for l in &lines {
        g.apply_journal(&l.event);
    }
    let (files, reports) = load(dir, &triggers(&lines));
    for f in files {
        g.apply_companion(f);
    }
    (g, reports)
}

fn file_text(name: &str) -> String {
    std::fs::read_to_string(fixture_dir().join(name)).unwrap()
}

#[test]
fn each_event_leaves_its_snapshot_in_game_state() {
    let (g, reports) = replay(JOURNAL, &fixture_dir());
    assert_eq!(reports, vec![]);
    let c = &g.companions;

    let m = c.market.as_ref().expect("market");
    assert_eq!(
        (m.market_id, m.station_name.as_str(), m.star_system.as_str()),
        (Some(128000001), "Test Orbital", "Test System")
    );
    assert_eq!(m.items.len(), 2);
    assert_eq!(
        (m.items[0].name.as_str(), m.items[0].buy_price, m.items[0].sell_price, m.items[0].stock),
        ("$gold_name;", 9000, 8800, 150)
    );
    assert!(m.items[0].producer && !m.items[0].consumer);
    assert_eq!((m.items[1].demand, m.items[1].demand_bracket), (400, 3));
    assert_eq!(m.raw_json, file_text("Market.json"), "raw file text kept for EDDN");

    let o = c.outfitting.as_ref().expect("outfitting");
    assert_eq!(o.items[0].name, "hpt_pulselaser_fixed_small");
    assert_eq!((o.items[0].buy_price, o.horizons), (2200, Some(true)));
    assert_eq!(o.raw_json, file_text("Outfitting.json"));

    let y = c.shipyard.as_ref().expect("shipyard");
    assert_eq!(
        (y.price_list[0].ship_type.as_str(), y.price_list[0].ship_price, y.allow_cobra_mk_iv),
        ("sidewinder", 32000, Some(false))
    );
    assert_eq!(y.raw_json, file_text("Shipyard.json"));

    let mi = c.modules_info.as_ref().expect("modules info");
    assert_eq!(mi.modules.len(), 2);
    assert_eq!(
        (mi.modules[0].slot.as_str(), mi.modules[0].power, mi.modules[1].power),
        ("FrameShiftDrive", Some(0.72), None)
    );

    let sl = c.ship_locker.as_ref().expect("ship locker");
    assert_eq!(sl.items.len(), 2);
    assert_eq!((sl.items[0].category.as_str(), sl.items[0].count), ("Items", 4));

    let bp = c.backpack.as_ref().expect("backpack");
    assert_eq!(
        bp.items.iter().map(|i| (i.category.as_str(), i.name.as_str())).collect::<Vec<_>>(),
        vec![("Items", "energycell"), ("Consumables", "bypass")]
    );

    let fc = c.fc_materials.as_ref().expect("fc materials");
    assert_eq!(
        (fc.carrier_name.as_deref(), fc.carrier_id.as_deref(), fc.items[0].stock),
        (Some("TEST CARRIER"), Some("3700000001"), 5)
    );

    // The dock is known too, so a consumer can match the market to it.
    assert_eq!(g.station_market_id, Some(128000001));
    assert_eq!(m.market_id, g.station_market_id);
}

#[test]
fn a_file_older_than_its_event_is_reported_and_not_used() {
    // Same files, events a minute later: the files are last visit's data.
    let later = JOURNAL.replace("2026-10-10T20:00:", "2026-10-10T20:01:");
    let (g, reports) = replay(&later, &fixture_dir());
    assert_eq!(g.companions, Default::default());
    assert_eq!(reports.len(), 7);
    assert!(reports.iter().all(|r| matches!(r.issue, CompanionIssue::Stale { .. })));
    let m = reports.iter().find(|r| r.kind == CompanionKind::Market).unwrap();
    assert_eq!(
        m.issue,
        CompanionIssue::Stale { file: "2026-10-10T20:00:05Z".into(), event: "2026-10-10T20:01:05Z".into() }
    );
    assert!(m.to_string().contains("Market.json") && m.to_string().contains("older"), "{m}");
}

#[test]
fn a_stale_file_does_not_replace_the_snapshot_already_held() {
    let (mut g, _) = replay(JOURNAL, &fixture_dir());
    let held = g.companions.market.clone().unwrap();
    let later = lines(&JOURNAL.replace("2026-10-10T20:00:", "2026-10-10T20:01:"));
    let (files, reports) = load(&fixture_dir(), &triggers(&later));
    assert!(files.is_empty() && !reports.is_empty());
    for f in files {
        g.apply_companion(f);
    }
    assert_eq!(g.companions.market.unwrap(), held);
}

#[test]
fn a_missing_file_is_reported() {
    let empty = tempfile::tempdir().unwrap();
    let (g, reports) = replay(JOURNAL, empty.path());
    assert_eq!(g.companions, Default::default());
    assert_eq!(reports.len(), 7);
    assert!(reports.iter().all(|r| r.issue == CompanionIssue::Missing));
}

#[test]
fn a_half_written_file_is_reported() {
    let d = tempfile::tempdir().unwrap();
    let text = file_text("Market.json");
    std::fs::write(d.path().join("Market.json"), &text[..text.len() / 2]).unwrap();
    let (g, reports) = replay(JOURNAL, d.path());
    assert!(g.companions.market.is_none());
    let r = reports.iter().find(|r| r.kind == CompanionKind::Market).unwrap();
    assert!(matches!(r.issue, CompanionIssue::Unreadable(_)), "{r:?}");
}

#[test]
fn only_the_latest_event_of_a_kind_is_checked() {
    // Two Market events: the first is older than the file, the second is not. Only the second
    // matters because the file only ever holds the latest.
    let journal = format!(
        "{}\n{}\n",
        r#"{"timestamp":"2026-10-10T19:00:00Z","event":"Market","MarketID":1,"StationName":"A","StarSystem":"S"}"#,
        r#"{"timestamp":"2026-10-10T20:00:05Z","event":"Market","MarketID":128000001,"StationName":"Test Orbital","StarSystem":"Test System"}"#,
    );
    let (g, reports) = replay(&journal, &fixture_dir());
    assert_eq!(reports, vec![]);
    assert!(g.companions.market.is_some());
    assert_eq!(triggers(&lines(&journal)).len(), 1);
}

#[test]
fn a_read_never_waits_for_a_file_the_game_has_not_written_yet() {
    let d = tempfile::tempdir().unwrap();
    let start = std::time::Instant::now();
    assert_eq!(
        read_companion(d.path(), CompanionKind::Market, "2026-10-10T20:00:05Z").unwrap_err(),
        CompanionIssue::Missing
    );
    assert!(start.elapsed() < std::time::Duration::from_millis(30));
    // The late write is picked up by the next read (the watcher's pending path in the app).
    std::fs::write(d.path().join("Market.json"), file_text("Market.json")).unwrap();
    assert!(read_companion(d.path(), CompanionKind::Market, "2026-10-10T20:00:05Z").is_ok());
}

#[test]
fn station_market_id_follows_docking_login_and_undocking() {
    let mut g = GameState::default();
    let ev = |l: &str| parse_line(l).unwrap().event;
    g.apply_journal(&ev(r#"{"timestamp":"t","event":"Location","StarSystem":"S","Docked":true,"StationName":"Test Orbital","MarketID":42}"#));
    assert_eq!((g.station.as_deref(), g.station_market_id, g.docked), (Some("Test Orbital"), Some(42), true));
    g.apply_journal(&ev(r#"{"timestamp":"t","event":"Undocked","StationName":"Test Orbital"}"#));
    assert_eq!((g.station.clone(), g.station_market_id, g.docked), (None, None, false));
    g.apply_journal(&ev(r#"{"timestamp":"t","event":"Docked","StationName":"Other","StarSystem":"S","MarketID":43}"#));
    assert_eq!(g.station_market_id, Some(43));
    g.apply_journal(&ev(r#"{"timestamp":"t","event":"Location","StarSystem":"S","Docked":false}"#));
    assert_eq!((g.station.clone(), g.station_market_id), (None, None));
}

#[test]
fn events_map_to_files() {
    assert_eq!(CompanionKind::for_event("ModuleInfo"), Some(CompanionKind::ModulesInfo));
    assert_eq!(CompanionKind::for_event("FCMaterials"), Some(CompanionKind::FcMaterials));
    assert_eq!(CompanionKind::for_event("BackpackChange"), Some(CompanionKind::Backpack));
    assert_eq!(CompanionKind::for_event("Docked"), None);
    for k in CompanionKind::ALL {
        assert_eq!(CompanionKind::from_file_name(k.file_name()), Some(k));
    }
}
