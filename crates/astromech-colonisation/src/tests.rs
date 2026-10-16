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
