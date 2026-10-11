//! # astromech-materials
//!
//! Engineering materials from the Elite journal, shown like the three in-game material traders
//! (ARCHITECTURE.md §18.5 / §18.7 M3). A `Materials` snapshot at login is the baseline; later
//! collect / discard / trade / craft / synthesis / mission / broker events are applied as deltas.

pub mod catalog;
pub mod ledger;
pub mod view;

pub use catalog::{capacity, lookup, Kind, Spec};
pub use ledger::{replay_bytes, replay_journal_dir, replay_lines, replay_path, Ledger};
pub use view::{from_journal_dir, trader_view, Cell, GroupView, Inventory, KindView};

/// Inventory produced from the crate's fake journal fixtures (app window sample).
pub fn sample_inventory() -> Inventory {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut v = from_journal_dir(&dir);
    v.source = "Sample journal".into();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn fixture_dir() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    #[test]
    fn catalogue_covers_traders() {
        assert_eq!(catalog::groups(Kind::Raw).len(), 7);
        assert_eq!(catalog::groups(Kind::Manufactured).len(), 10);
        assert_eq!(catalog::groups(Kind::Encoded).len(), 6);
        assert_eq!(lookup("iron").unwrap().grade, 1);
        assert_eq!(lookup("FedProprietaryComposites").unwrap().name, "Proprietary Composites");
        assert_eq!(lookup("uncutFocusCrystals").unwrap().name, "Flawed Focus Crystals");
        assert_eq!(capacity(1), 300);
        assert_eq!(capacity(5), 100);
    }

    #[test]
    fn json_array_fixture() {
        let p = fixture_dir().join("materials-events.json");
        let l = replay_path(&p);
        assert_eq!(l.count("iron"), 22);
        assert_eq!(l.count("carbon"), 89);
        assert_eq!(l.count("vanadium"), 9);
        assert_eq!(l.count("phosphorus"), 4);
        assert_eq!(l.count("manganese"), 4);
        assert_eq!(l.count("germanium"), 4);
        assert_eq!(l.count("chemicalstorageunits"), 40);
        assert_eq!(l.count("emissiondata"), 32);
        assert_eq!(l.count("disruptedwakeechoes"), 4);
        assert_eq!(l.snapshot_ts.as_deref(), Some("2026-10-02T12:00:00Z"));
    }

    #[test]
    fn journal_dir_uses_latest_snapshot() {
        let l = replay_journal_dir(&fixture_dir());
        // Older file had iron=12; the 2 Oct snapshot + deltas win.
        assert_eq!(l.count("iron"), 22);
        assert_eq!(l.count("carbon"), 89);
        assert!(l.count("sulphur") == 0); // only in the ignored older snapshot
        let v = trader_view(&l, "test");
        let raw = v.kinds.iter().find(|k| k.id == "raw").unwrap();
        assert_eq!(raw.groups.len(), 7);
        let cat4 = raw.groups.iter().find(|g| g.name == "Category 4").unwrap();
        let iron = cat4.cells.iter().find(|c| c.id == "iron").unwrap();
        assert_eq!(iron.count, 22);
        assert_eq!(iron.capacity, 300);
        let enc = v.kinds.iter().find(|k| k.id == "encoded").unwrap();
        let wakes = enc.groups.iter().find(|g| g.name == "Wake Scans").unwrap();
        assert_eq!(wakes.cells[0].count, 4);
    }

    #[test]
    fn unknown_material_lands_in_other() {
        let l = replay_lines([
            r#"{"timestamp":"t","event":"Materials","Raw":[{"Name":"notarealmat","Count":7}],"Manufactured":[],"Encoded":[]}"#,
        ]);
        let v = trader_view(&l, "x");
        assert_eq!(v.other[0].id, "notarealmat");
        assert_eq!(v.other[0].count, 7);
    }

    #[test]
    fn writes_and_replays_temp_journal() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("Journal.2026-10-11T000000.01.log");
        let mut f = std::fs::File::create(&p).unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-10-11T00:00:00Z","event":"Materials","Raw":[{{"Name":"nickel","Count":8}}],"Manufactured":[],"Encoded":[]}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"timestamp":"2026-10-11T00:01:00Z","event":"MaterialCollected","Category":"Raw","Name":"nickel","Count":2}}"#
        )
        .unwrap();
        assert_eq!(replay_journal_dir(d.path()).count("nickel"), 10);
    }
}
