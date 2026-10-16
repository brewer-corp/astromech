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
    fn mid_session_events_move_the_hold() {
        let h = hold(&[
            CARGO,
            r#"{"timestamp":"t","event":"ColonisationContribution","MarketID":1,"Contributions":[{"Name":"$steel_name;","Amount":120}]}"#,
            r#"{"timestamp":"t","event":"MarketBuy","MarketID":2,"Type":"steel","Count":50,"BuyPrice":10,"TotalCost":500}"#,
        ]);
        assert_eq!(h.cargo.get("steel"), Some(&50));
        assert_eq!(h.cargo.get("gold"), Some(&10));
        // Delivered cargo is not subtracted twice.
        let a = replayed().project(MARKET_A).unwrap().clone();
        let l = shopping_list(&a, &h);
        assert_eq!((l.lines[0].in_cargo, l.lines[0].to_buy), (50, 250));

        let h = hold(&[
            CARGO,
            r#"{"timestamp":"t","event":"MarketSell","MarketID":2,"Type":"steel","Count":20,"SellPrice":10,"TotalSale":200}"#,
            r#"{"timestamp":"t","event":"CollectCargo","Type":"steel","Stolen":false}"#,
            r#"{"timestamp":"t","event":"EjectCargo","Type":"steel","Count":11,"Abandoned":false}"#,
            r#"{"timestamp":"t","event":"EjectCargo","Type":"steel","Count":50,"Abandoned":false,"MissionID":9}"#,
            r#"{"timestamp":"t","event":"MarketSell","Type":"gold","Count":99}"#,
        ]);
        assert_eq!(h.cargo.get("steel"), Some(&90));
        assert_eq!(h.cargo.get("gold"), Some(&0));
    }

    #[test]
    fn cargo_without_inventory_keeps_the_hold() {
        let count_only = r#"{"timestamp":"t","event":"Cargo","Vessel":"Ship","Count":130}"#;
        assert_eq!(hold(&[CARGO, count_only]).cargo.get("steel"), Some(&120));
        // An explicitly empty hold does clear it.
        let empty = r#"{"timestamp":"t","event":"Cargo","Vessel":"Ship","Count":0,"Inventory":[]}"#;
        assert!(hold(&[CARGO, empty]).cargo.is_empty());
    }

    #[test]
    fn mission_cargo_is_skipped_and_stolen_is_counted() {
        let c = r#"{"timestamp":"t","event":"Cargo","Vessel":"Ship","Count":60,"Inventory":[{"Name":"steel","Count":20,"MissionID":5},{"Name":"steel","Count":30,"Stolen":1},{"Name":"steel","Count":10}]}"#;
        assert_eq!(hold(&[c]).cargo.get("steel"), Some(&40));
    }

    #[test]
    fn commodity_ids_from_real_names() {
        for (depot, cargo) in [
            ("$CeramicComposites_name;", "ceramiccomposites"),
            ("$InsulatingMembrane_name;", "insulatingmembrane"),
            ("$LiquidOxygen_name;", "liquidoxygen"),
            ("$Pesticides_name;", "pesticides"),
            ("$Semiconductors_name;", "semiconductors"),
            ("$Superconductors_name;", "superconductors"),
            ("$CMMComposite_name;", "cmmcomposite"),
        ] {
            assert_eq!(commodity_id(depot), cargo);
            assert_eq!(commodity_id(cargo), cargo);
        }
    }

    #[test]
    fn journal_folder_view_lists_projects_with_their_shopping_lists() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Journal.2026-10-01T090000.01.log"), JOURNAL).unwrap();
        std::fs::write(dir.path().join("Journal.2026-10-02T100000.01.log"), format!("{LOADOUT}\n{CARGO}\n")).unwrap();
        let v = crate::from_journal_dir(dir.path());
        // The project being built first, the finished one after.
        assert_eq!(v.iter().map(|p| p.project.market_id).collect::<Vec<_>>(), [MARKET_A, MARKET_B]);
        let a = &v[0].shopping;
        let lines: Vec<(&str, i64, i64)> = a.lines.iter().map(|l| (l.symbol.as_str(), l.in_cargo, l.to_buy)).collect();
        assert_eq!(lines, [("steel", 120, 180), ("aluminium", 0, 150)]);
        assert_eq!((a.to_buy_total, a.capacity, a.loads), (330, Some(100), Some(4)));
        assert!(v[1].shopping.lines.is_empty());

        // The app reads the project's fields and its shopping list from one object.
        let j = serde_json::to_value(&v[0]).unwrap();
        assert_eq!(j["system_name"], "Test System");
        assert_eq!(j["progress"], 0.5);
        assert_eq!(j["shopping"]["lines"][0]["symbol"], "steel");
        assert_eq!(j["shopping"]["lines"][0]["name_localised"], "Steel");
        assert!(crate::from_journal_dir(&dir.path().join("missing")).is_empty());
    }

    #[test]
    fn cargo_json_overrides_the_replayed_hold() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Journal.2026-10-01T090000.01.log"), JOURNAL).unwrap();
        // The journal says 120 t steel aboard; a carrier transfer (not folded) left 40 t, plus
        // 30 t aluminium. Mission cargo in the file is not deliverable.
        std::fs::write(dir.path().join("Journal.2026-10-02T100000.01.log"), format!("{LOADOUT}\n{CARGO}\n")).unwrap();
        let file = r#"{"timestamp":"2026-10-02T11:00:00Z","event":"Cargo","Vessel":"Ship","Count":80,"Inventory":[{"Name":"steel","Count":40,"Stolen":0},{"Name":"aluminium","Name_Localised":"Aluminium","Count":30,"Stolen":0},{"Name":"aluminium","Count":10,"Stolen":0,"MissionID":7}]}"#;
        std::fs::write(dir.path().join("Cargo.json"), file).unwrap();
        let v = crate::from_journal_dir(dir.path());
        let lines: Vec<(&str, i64, i64)> =
            v[0].shopping.lines.iter().map(|l| (l.symbol.as_str(), l.in_cargo, l.to_buy)).collect();
        assert_eq!(lines, [("steel", 40, 260), ("aluminium", 30, 120)]);

        // Everything moved to a carrier: the game writes an empty hold with no Inventory.
        std::fs::write(
            dir.path().join("Cargo.json"),
            r#"{"timestamp":"2026-10-02T12:00:00Z","event":"Cargo","Vessel":"Ship","Count":0}"#,
        )
        .unwrap();
        let v = crate::from_journal_dir(dir.path());
        assert!(v[0].shopping.lines.iter().all(|l| l.in_cargo == 0), "{:?}", v[0].shopping.lines);
        assert_eq!(v[0].shopping.to_buy_total, 450);

        // An SRV's file, a broken one, or a count with no list leaves the journal's hold.
        for other in [
            r#"{"Vessel":"SRV","Count":2,"Inventory":[{"Name":"steel","Count":2}]}"#,
            "{not json",
            r#"{"Vessel":"Ship","Count":120}"#,
        ] {
            std::fs::write(dir.path().join("Cargo.json"), other).unwrap();
            let v = crate::from_journal_dir(dir.path());
            assert_eq!(v[0].shopping.lines[0].in_cargo, 120, "{other}");
        }
    }
}
