//! astromech-4irh: ships and modules ordered to a station arrive when their `TransferTime` passes
//! on the Clock, whenever the page reads the fleet: the journal says nothing at that moment, so
//! nothing is written to prompt a rebuild.

use astromech_engineering::{from_journal_dir_at, Settled, FLEET_EVENTS};
use astromech_journal::JournalCache;
use astromech_proto::Clock;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

struct Fake(AtomicI64);
impl Clock for Fake {
    fn now_s(&self) -> i64 {
        self.0.load(Ordering::SeqCst)
    }
}

/// 2026-10-10T11:01:00Z, when the ship and the module are ordered (300 s and 600 s to arrive).
const ORDERED: i64 = 1_791_630_060;

const JOURNAL: &[&str] = &[
    r#"{"timestamp":"2026-10-10T09:00:00Z","event":"Loadout","Ship":"asp","ShipID":4,"ShipName":"A","ShipIdent":"X","Modules":[]}"#,
    r#"{"timestamp":"2026-10-10T10:00:00Z","event":"Docked","StationName":"Abraham Lincoln","StarSystem":"Sol","MarketID":1}"#,
    r#"{"timestamp":"2026-10-10T10:01:00Z","event":"StoredModules","MarketID":1,"StarSystem":"Sol","Items":[{"Name":"$hpt_pulselaser_fixed_small_name;","StorageSlot":1,"StarSystem":"Sol","BuyPrice":10,"Hot":true},{"Name":"$int_shieldgenerator_size5_class5_name;","StorageSlot":2,"StarSystem":"LHS 3447","BuyPrice":20}]}"#,
    r#"{"timestamp":"2026-10-10T11:00:00Z","event":"Docked","StationName":"Hub","StarSystem":"LHS 3447","MarketID":2}"#,
    r#"{"timestamp":"2026-10-10T11:00:30Z","event":"StoredShips","StarSystem":"LHS 3447","MarketID":2,"ShipsHere":[],"ShipsRemote":[{"ShipID":12,"ShipType":"anaconda","StarSystem":"Sol","ShipMarketID":1}]}"#,
    r#"{"timestamp":"2026-10-10T11:01:00Z","event":"ShipyardTransfer","ShipType":"anaconda","ShipID":12,"System":"Sol","Distance":10.0,"TransferPrice":1200,"TransferTime":600}"#,
    r#"{"timestamp":"2026-10-10T11:01:00Z","event":"FetchRemoteModule","StorageSlot":2,"StoredItem":"$hpt_pulselaser_fixed_small_name;","ServerId":9,"TransferCost":500,"TransferTime":300,"Ship":"asp","ShipID":4}"#,
];

#[test]
fn a_ship_and_a_module_arrive_as_the_clock_passes_their_transfer_time() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("Journal.2026-10-10T090000.01.log"), JOURNAL.join("\n") + "\n").unwrap();
    let clock = Fake(AtomicI64::new(ORDERED + 60));
    // The page's cache: rebuilt when the journals change or the settled fleet goes out of date.
    let cache = JournalCache::<Settled>::new(FLEET_EVENTS);
    let builds = AtomicUsize::new(0);
    let read = || {
        cache.get_fresh(
            d.path(),
            |s| Settled::current(s.next_arrival, &clock),
            || {
                builds.fetch_add(1, Ordering::SeqCst);
                from_journal_dir_at(d.path(), &clock)
            },
        )
    };
    let laser = |s: &Settled| {
        s.fleet.stored_modules.iter().find(|m| m.item == "hpt_pulselaser_fixed_small").unwrap().system.clone()
    };
    let anaconda = |s: &Settled| s.fleet.ships.iter().find(|x| x.id == 12).unwrap().in_transit;

    let s = read();
    assert!(anaconda(&s), "ordered a minute ago");
    assert_eq!(laser(&s), None, "the module is on its way");
    assert_eq!(s.next_arrival, Some(ORDERED + 300));
    read();
    assert_eq!(builds.load(Ordering::SeqCst), 1, "nothing changed: no rebuild");

    // Nothing is written; the clock moves past the module's TransferTime.
    clock.0.store(ORDERED + 301, Ordering::SeqCst);
    let s = read();
    assert_eq!(laser(&s).as_deref(), Some("LHS 3447"), "the fetched module has arrived");
    assert!(anaconda(&s), "the ship is still on its way");
    assert_eq!(s.next_arrival, Some(ORDERED + 600));

    clock.0.store(ORDERED + 601, Ordering::SeqCst);
    let s = read();
    assert!(!anaconda(&s), "the ship has arrived");
    assert_eq!(s.next_arrival, None);
    read();
    assert_eq!(builds.load(Ordering::SeqCst), 3, "one rebuild per arrival, none once nothing is pending");
}
