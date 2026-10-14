//! # astromech-engineering
//!
//! Ship engineering planner (ARCHITECTURE.md §18.7 M3): every owned ship and its fitted
//! modules from journal history, a build plan per ship (blueprint, grade and experimental per
//! slot), and the materials it costs against the inventory from `astromech-materials`, with
//! trader swaps from surplus and farming notes for the rest.

pub mod data;
pub mod fleet;
pub mod plan;

pub use fleet::{from_journal_dir, Applied, Fitted, Fleet, FleetBuilder, Ship};
pub use plan::{evaluate, MatLine, Plan, PlanResult, Rolls, Target, TargetCost, Trade};

/// Fleet read from the crate's fake journal fixtures (app window sample).
pub fn sample_fleet() -> Fleet {
    let mut f = from_journal_dir(&fixture_dir());
    f.source = "Sample journal".into();
    f
}

/// A sample plan for the current fixture ship, evaluated against the fixture inventory.
pub fn sample_plan() -> PlanResult {
    let fleet = from_journal_dir(&fixture_dir());
    let ledger = astromech_materials::replay_journal_dir(&fixture_dir());
    let ship = fleet.ships.iter().find(|s| s.id == 7).expect("fixture ship");
    evaluate(ship, &sample_plan_input(), &ledger)
}

pub fn sample_plan_input() -> Plan {
    Plan {
        ship_id: 7,
        targets: vec![
            Target {
                slot: "FrameShiftDrive".into(),
                blueprint: "FSD_LongRange".into(),
                grade: 5,
                experimental: Some("special_fsd_heavy".into()),
            },
            Target {
                slot: "MainEngines".into(),
                blueprint: "Engine_Dirty".into(),
                grade: 5,
                experimental: Some("special_engine_overloaded".into()),
            },
        ],
        rolls: Rolls::default(),
    }
}

fn fixture_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_materials::catalog;

    #[test]
    fn catalogue_materials_resolve() {
        let c = data::catalog();
        let ids = c
            .blueprints
            .values()
            .flat_map(|b| b.grades.iter().flat_map(|g| g.materials.keys()))
            .chain(c.experimentals.values().flat_map(|e| e.materials.keys()));
        for id in ids {
            assert!(catalog::lookup(id).is_some(), "material {id} not in astromech-materials catalogue");
        }
        assert_eq!(data::blueprint("fsd_longrange").unwrap().1.name, "Increased range");
        assert_eq!(data::module("Int_Hyperdrive_Size5_Class5").unwrap().display(), "5A Frame Shift Drive");
        assert_eq!(data::ship_name("krait_mkii"), "Krait MkII");
        for g in c.groups.values() {
            for b in &g.blueprints {
                assert!(data::blueprint(b).is_some(), "{b}");
            }
        }
    }

    #[test]
    fn fleet_from_history() {
        let f = from_journal_dir(&fixture_dir());
        let ids: Vec<i64> = f.ships.iter().map(|s| s.id).collect();
        // current first, then flown ships, then stored-only; the Cobra was sold
        assert_eq!(ids, vec![3, 7, 12]);
        let ana = &f.ships[0];
        assert!(ana.current);
        assert_eq!(ana.system.as_deref(), None);
        let krait = &f.ships[1];
        assert_eq!(krait.type_name, "Krait MkII");
        assert_eq!(krait.name.as_deref(), Some("Long Way Round"));
        assert_eq!(krait.modules[0].slot_name, "Armour");
        assert!(krait.modules.iter().all(|m| !m.item.starts_with("paintjob")));
        let fsd = krait.modules.iter().find(|m| m.slot == "FrameShiftDrive").unwrap();
        assert_eq!(fsd.group.as_deref(), Some("fsd"));
        let e = fsd.engineering.as_ref().unwrap();
        assert_eq!((e.blueprint_name.as_str(), e.grade), ("Increased range", 3));
        let thr = krait.modules.iter().find(|m| m.slot == "MainEngines").unwrap();
        assert_eq!(thr.engineering.as_ref().unwrap().experimental_name.as_deref(), Some("Drag Drives"));
        let sb = krait.modules.iter().find(|m| m.slot == "TinyHardpoint1").unwrap();
        assert_eq!(sb.slot_name, "Utility 1");
        let asp = &f.ships[2];
        assert_eq!(asp.loadout_ts, None);
        assert_eq!(asp.system.as_deref(), Some("Sol"));
        assert!(f.groups.contains_key("fsd") && f.groups.contains_key("t"));
    }

    #[test]
    fn grades_remaining_and_shopping_list() {
        let r = sample_plan();
        let fsd = &r.targets[0];
        // G3 at 25% → ceil(0.75 × 3) = 3 rolls, then G4 ×4, G5 ×5, plus Mass Manager
        assert_eq!(fsd.steps, vec![(3, 3), (4, 4), (5, 5)]);
        assert!(fsd.experimental_needed);
        let n = |id: &str| fsd.materials.iter().find(|(m, _)| m == id).map(|x| x.1).unwrap_or(0);
        assert_eq!(n("wakesolutions"), 3);
        assert_eq!(n("hyperspacetrajectories"), 4 + 1); // G4 rolls + Mass Manager
        assert_eq!(n("dataminedwake"), 5);
        let thr = &r.targets[1];
        // Dirty drives G3 at 50%: ceil(1.5) = 2 rolls; Drag Drives already on → not needed
        assert_eq!(thr.steps[0], (3, 2));
        assert!(!thr.experimental_needed);
        let dw = r.materials.iter().find(|m| m.id == "dataminedwake").unwrap();
        assert_eq!((dw.have, dw.short), (0, 5));
        // 300 sulphur (full) trades up for the cadmium the thrusters need, 6:1 per grade
        let cd = r.trades.iter().find(|t| t.get_id == "cadmium").unwrap();
        assert_eq!((cd.give_id.as_str(), cd.give, cd.get), ("sulphur", 180, 5));
        // trades never spend more than is spare
        let csu: i64 = r.trades.iter().filter(|t| t.give_id == "chemicalstorageunits").map(|t| t.give).sum();
        assert!(csu <= 200);
        // what trades cover is not in the farm list; G5 wake data has no donor
        assert!(r.farm.iter().all(|f| f.id != "cadmium"));
        assert_eq!(r.farm.iter().find(|f| f.id == "dataminedwake").unwrap().short, 5);
    }

    #[test]
    fn single_grade_blueprints_and_vehicles() {
        // Expanded Capacity exists only at G5: no phantom G1–G4 steps
        let mut b = FleetBuilder::default();
        let l = astromech_journal::parse_line(
            r#"{"timestamp":"t","event":"Loadout","Ship":"krait_mkii","ShipID":7,"Modules":[{"Slot":"Slot01_Size6","Item":"Int_CargoRack_Size6_Class1"}]}"#,
        )
        .unwrap();
        b.apply(&l.timestamp, &l.event);
        // a taxi's Loadout is not a ship of the fleet
        let taxi = astromech_journal::parse_line(
            r#"{"timestamp":"t","event":"Loadout","Ship":"adder_taxi","ShipID":99,"Modules":[]}"#,
        )
        .unwrap();
        b.apply(&taxi.timestamp, &taxi.event);
        let fleet = b.build("t");
        assert_eq!(fleet.ships.len(), 1);
        assert!(fleet.ships[0].current);
        let t = Target {
            slot: "Slot01_Size6".into(),
            blueprint: "CargoRack_IncreasedCapacity".into(),
            grade: 5,
            experimental: None,
        };
        let c = plan::cost_target(&fleet.ships[0], &t, &Rolls::default());
        assert_eq!(c.steps.iter().map(|s| s.0).collect::<Vec<_>>(), vec![5]);
        assert!(
            fleet::is_vehicle("testbuggy") && fleet::is_vehicle("federation_fighter") && !fleet::is_vehicle("anaconda")
        );
    }

    #[test]
    fn trader_rates() {
        let s = |id| catalog::lookup(id).unwrap();
        assert_eq!(plan::rate(s("iron"), s("nickel")), Some((6, 1))); // same grade, other category
        assert_eq!(plan::rate(s("nickel"), s("germanium")), Some((6, 1))); // G1 → G2, same category
        assert_eq!(plan::rate(s("chemicalstorageunits"), s("chemicalmanipulators")), Some((216, 1)));
        assert_eq!(plan::rate(s("chemicalmanipulators"), s("chemicalstorageunits")), Some((1, 27)));
        assert_eq!(plan::rate(s("dataminedwake"), s("fsdtelemetry")), Some((1, 27)));
        // down one grade across categories: 6:3 → 2:1
        assert_eq!(plan::rate(s("fsdtelemetry"), s("shieldcyclerecordings")), Some((2, 1)));
        assert_eq!(plan::rate(s("iron"), s("emissiondata")), None);
    }

    #[test]
    fn other_blueprint_restarts_and_bad_targets() {
        let f = from_journal_dir(&fixture_dir());
        let krait = f.ships.iter().find(|s| s.id == 7).unwrap();
        let rolls = Rolls::default();
        let t =
            |slot: &str, bp: &str, g| Target { slot: slot.into(), blueprint: bp.into(), grade: g, experimental: None };
        let c = plan::cost_target(krait, &t("FrameShiftDrive", "FSD_FastBoot", 2), &rolls);
        assert_eq!(c.steps, vec![(1, 1), (2, 2)]);
        assert!(c.note.unwrap().contains("grade 1"));
        let c = plan::cost_target(krait, &t("PowerPlant", "PowerPlant_Boosted", 5), &rolls);
        assert!(c.steps.is_empty() && c.materials.is_empty());
        assert_eq!(c.note.as_deref(), Some("Already done."));
        let c = plan::cost_target(krait, &t("FrameShiftDrive", "Engine_Dirty", 5), &rolls);
        assert!(c.note.unwrap().contains("cannot take"));
        assert!(c.materials.is_empty());
    }
}
