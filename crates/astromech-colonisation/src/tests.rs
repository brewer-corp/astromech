use super::*;

/// Synthetic journal: two projects (market ids are made up), a claim, a release, a beacon.
const JOURNAL: &str = include_str!("../tests/fixtures/colonisation.jsonl");

const MARKET_A: i64 = 3900000001;
const MARKET_B: i64 = 3900000002;

fn replayed() -> Tracker {
    let mut t = Tracker::new();
    replay_lines(&mut t, JOURNAL.lines());
    t
}

#[test]
fn replay_gives_progress_per_project() {
    let t = replayed();
    assert_eq!(t.projects().count(), 2);

    let a = t.project(MARKET_A).unwrap();
    assert_eq!(a.progress, 0.5);
    assert!(!a.complete && !a.failed);
    assert_eq!(a.station_name.as_deref(), Some("Planetary Construction Site: Test Hub"));
    assert_eq!(a.system_name.as_deref(), Some("Test System"));
    assert_eq!(a.first_seen, "2026-10-01T10:00:05Z");
    assert_eq!(a.depot_updated, "2026-10-01T10:30:00Z");
    // Latest depot snapshot wins: steel 800/500, aluminium 200/50.
    assert_eq!(a.remaining_total(), 300 + 150);
    assert_eq!(a.resources[0].remaining(), 300);
    assert_eq!(a.resources[0].payment, 4000);
    // Two deliveries, each counted once.
    assert_eq!(a.contributed("$steel_name;"), 300);
    assert_eq!(a.contributed("$aluminium_name;"), 50);
    assert_eq!(a.contributed_total(), 350);

    let b = t.project(MARKET_B).unwrap();
    assert_eq!(b.progress, 1.0);
    assert!(b.complete);
    assert_eq!(b.remaining_total(), 0);
    assert_eq!(b.contributed_total(), 0);
}

#[test]
fn claims_and_beacons() {
    let t = replayed();
    let claims: Vec<_> = t.claims().collect();
    assert_eq!(claims.len(), 2);
    let kept = claims.iter().find(|c| c.system_address == Some(1001)).unwrap();
    assert!(kept.active() && kept.system_name.as_deref() == Some("Test System"));
    let gone = claims.iter().find(|c| c.system_address == Some(1002)).unwrap();
    assert_eq!(gone.released_at.as_deref(), Some("2026-10-02T09:00:00Z"));
    assert!(!gone.active());
    let beacons: Vec<_> = t.beacons().collect();
    assert_eq!(beacons.len(), 1);
    assert_eq!((beacons[0].system_name.as_deref(), beacons[0].system_address), (Some("Test System"), Some(1001)));
}

#[test]
fn replaying_again_changes_nothing() {
    let mut t = replayed();
    let before: Vec<_> = t.projects().cloned().collect();
    let claims: Vec<_> = t.claims().cloned().collect();
    replay_lines(&mut t, JOURNAL.lines());
    assert_eq!(t.projects().cloned().collect::<Vec<_>>(), before);
    assert_eq!(t.claims().cloned().collect::<Vec<_>>(), claims);
    assert_eq!(t.beacons().count(), 1);
}

#[test]
fn stale_depot_does_not_overwrite_newer() {
    let mut t = replayed();
    let old = r#"{"timestamp":"2026-10-01T09:00:00Z","event":"ColonisationConstructionDepot","MarketID":3900000001,"ConstructionProgress":0.01,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[]}"#;
    replay_lines(&mut t, [old]);
    assert_eq!(t.project(MARKET_A).unwrap().progress, 0.5);
}

#[test]
fn survives_restart() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("astromech.db");
    let expected: Vec<_> = {
        let store = Store::open(&db).unwrap();
        let mut t = replayed();
        t.save(&store).unwrap();
        t.projects().cloned().collect()
    };
    // New process: reopen the file, restore, then the same journal is read again.
    let store = Store::open(&db).unwrap();
    let mut t = Tracker::load(&store).unwrap();
    assert_eq!(t.projects().cloned().collect::<Vec<_>>(), expected);
    assert_eq!(t.project(MARKET_A).unwrap().progress, 0.5);
    assert_eq!(t.project(MARKET_B).unwrap().progress, 1.0);
    assert_eq!(t.claims().count(), 2);
    assert_eq!(t.beacons().count(), 1);
    replay_lines(&mut t, JOURNAL.lines());
    assert_eq!(t.project(MARKET_A).unwrap().contributed_total(), 350);

    // New events after the restart extend the restored state and persist again.
    replay_lines(
        &mut t,
        [
            r#"{"timestamp":"2026-10-03T10:00:00Z","event":"ColonisationContribution","MarketID":3900000001,"Contributions":[{"Name":"$steel_name;","Amount":100}]}"#,
        ],
    );
    t.save(&store).unwrap();
    let t2 = Tracker::load(&store).unwrap();
    assert_eq!(t2.project(MARKET_A).unwrap().contributed("$steel_name;"), 400);
}

#[test]
fn save_writes_only_changes() {
    let store = Store::in_memory().unwrap();
    let mut t = replayed();
    t.save(&store).unwrap();
    store.put_record(PROJECT, &MARKET_B.to_string(), "sentinel").unwrap();
    t.save(&store).unwrap();
    assert!(store.records(PROJECT).unwrap().iter().any(|(_, b)| b == "sentinel"));
}

#[test]
fn corrupt_record_is_an_error_not_silent_loss() {
    let store = Store::in_memory().unwrap();
    store.put_record(PROJECT, "1", "not json").unwrap();
    assert!(matches!(Tracker::load(&store), Err(Error::Corrupt { .. })));
}

#[test]
fn claim_without_address_keys_by_name_and_empty_events_are_tolerated() {
    let mut t = Tracker::new();
    replay_lines(
        &mut t,
        [
            r#"{"timestamp":"2026-10-01T00:00:00Z","event":"ColonisationSystemClaim","SystemName":"Named Only"}"#,
            r#"{"timestamp":"2026-10-01T00:00:01Z","event":"ColonisationBeaconDeployed"}"#,
            r#"{"timestamp":"2026-10-01T00:00:02Z","event":"ColonisationContribution","Contributions":[]}"#,
        ],
    );
    assert_eq!(t.claims().next().unwrap().system_name.as_deref(), Some("Named Only"));
    assert_eq!(t.beacons().count(), 1);
    assert_eq!(t.projects().count(), 0);
}

mod shopping_tests {
    use super::*;
    use crate::shopping::{commodity_id, shopping_list, Hold};

    fn hold(lines: &[&str]) -> Hold {
        let mut h = Hold::default();
        for l in lines {
            h.apply(&astromech_journal::parse_line(l).unwrap());
        }
        h
    }

    const LOADOUT: &str = r#"{"timestamp":"2026-10-02T10:00:00Z","event":"Loadout","Ship":"type9","ShipID":3,"CargoCapacity":100,"Modules":[]}"#;
    const CARGO: &str = r#"{"timestamp":"2026-10-02T10:01:00Z","event":"Cargo","Vessel":"Ship","Count":130,"Inventory":[{"Name":"steel","Count":120},{"Name":"gold","Count":10}]}"#;

    #[test]
    fn remaining_minus_cargo_in_loads() {
        // Project A at the end of the fixture: steel 300 and aluminium 150 still to deliver.
        let a = replayed().project(MARKET_A).unwrap().clone();
        let l = shopping_list(&a, &hold(&[LOADOUT, CARGO]));
        let rows: Vec<_> = l.lines.iter().map(|x| (x.name.as_str(), x.remaining, x.in_cargo, x.to_buy)).collect();
        assert_eq!(rows, vec![("$steel_name;", 300, 120, 180), ("$aluminium_name;", 150, 0, 150)]);
        assert_eq!(l.to_buy_total, 330);
        assert_eq!((l.capacity, l.loads), (Some(100), Some(4)));
    }

    #[test]
    fn exact_multiple_extra_cargo_unknown_capacity_and_srv() {
        let a = replayed().project(MARKET_A).unwrap().clone();
        // 400 steel aboard is more than the 300 needed: only 300 count, so just aluminium is left.
        let big = r#"{"timestamp":"t","event":"Cargo","Vessel":"Ship","Count":400,"Inventory":[{"Name":"steel","Count":400}]}"#;
        let l = shopping_list(&a, &hold(&[LOADOUT, big]));
        assert_eq!((l.to_buy_total, l.loads), (150, Some(2)));
        assert_eq!(l.lines.len(), 2);
        assert_eq!(l.lines[1].to_buy, 0);
        // No Loadout seen: list is right, loads unknown. An SRV hold is ignored.
        let srv =
            r#"{"timestamp":"t","event":"Cargo","Vessel":"SRV","Count":5,"Inventory":[{"Name":"steel","Count":5}]}"#;
        let l = shopping_list(&a, &hold(&[CARGO, srv]));
        assert_eq!((l.to_buy_total, l.loads), (330, None));
        // A finished project needs nothing.
        let b = replayed().project(MARKET_B).unwrap().clone();
        let l = shopping_list(&b, &hold(&[LOADOUT]));
        assert_eq!((l.lines.len(), l.loads), (0, Some(0)));
    }

    #[test]
    fn names_match_between_depot_and_cargo() {
        assert_eq!(commodity_id("$CeramicComposites_name;"), "ceramiccomposites");
        assert_eq!(commodity_id("ceramiccomposites"), "ceramiccomposites");
    }
}
