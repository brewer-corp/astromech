//! # astromech-engineering
//!
//! Ship engineering planner (ARCHITECTURE.md §18.7 M3): every owned ship and its fitted
//! modules from journal history, a build plan per ship (blueprint, grade and experimental per
//! slot), and the materials it costs against the inventory from `astromech-materials`, with
//! trader swaps from surplus and farming notes for the rest.

pub mod data;
pub mod engineers;
pub mod fleet;
pub mod plan;
pub mod trip;

pub use engineers::{EngineerView, Pin};
pub use fleet::{from_journal_dir, Applied, Fitted, Fleet, FleetBuilder, Ship};
pub use plan::{evaluate, FarmStep, MatLine, Plan, PlanResult, Rolls, Target, TargetCost, Trade};
pub use trip::{plan_trip, Hop, Trip};

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
        // what trades cover is not in the farm list; G5 wake data is farmed directly (wake scans)
        assert!(r.farm.iter().all(|f| f.id != "cadmium" && f.then.iter().all(|t| t.get_id != "cadmium")));
        let dw = r.farm.iter().find(|f| f.id == "dataminedwake").unwrap();
        assert_eq!((dw.collect, dw.sources[0].kind.as_str()), (5, "wake_scan"));
        // the encoded firmware the thrusters need is either collected at Jameson or traded
        // from Adaptive Encryptors; the leftover raw comes from grade 4 of its category
        let ae = r.farm.iter().find(|f| f.id == "adaptiveencryptors");
        let ind = r.farm.iter().find(|f| f.id == "industrialfirmware").unwrap();
        assert_eq!((ind.collect, ind.sources[0].kind.as_str()), (5, "jameson"));
        assert!(ae.is_none() || ae.unwrap().then.iter().all(|t| t.give_id == "adaptiveencryptors"));
        for f in &r.farm {
            for t in &f.then {
                assert_eq!(t.give_id, f.id);
            }
        }
    }

    #[test]
    fn farm_route_trades_down_from_anchors() {
        let ledger = astromech_materials::Ledger::default();
        let need: std::collections::BTreeMap<String, i64> =
            [("arsenic", 10), ("mercury", 5), ("polonium", 1), ("shieldfrequencydata", 2), ("dataminedwake", 3)]
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect();
        let (trades, left, spare) = plan::suggest_trades(&need, &ledger);
        assert!(trades.is_empty());
        let route = plan::farm_route(&left, &spare, &ledger);
        let po = route.iter().find(|f| f.id == "polonium").unwrap();
        // 1 for itself + arsenic 1:9 ×2 blocks + mercury 1:3 ×2 blocks
        assert_eq!(po.collect, 1 + 2 + 2);
        assert_eq!(
            po.then.iter().map(|t| (t.get_id.as_str(), t.give, t.get)).collect::<Vec<_>>(),
            vec![("arsenic", 2, 18), ("mercury", 2, 6)]
        );
        assert_eq!(po.sources[0].kind, "brain_tree");
        // G5 → G5 across categories: 6:1
        let ae = route.iter().find(|f| f.id == "adaptiveencryptors").unwrap();
        assert_eq!((ae.collect, ae.then[0].get), (12, 2));
        assert!(route.iter().any(|f| f.id == "dataminedwake" && f.collect == 3 && f.then.is_empty()));
    }

    fn engineers_with(set: &[(&str, &str, Option<u8>)]) -> Vec<EngineerView> {
        let mut b = engineers::EngineerBuilder::default();
        let snaps = set
            .iter()
            .map(|(n, p, r)| astromech_journal::EngineerSnap {
                engineer: n.to_string(),
                engineer_id: None,
                progress: p.to_string(),
                rank: *r,
                rank_progress: None,
            })
            .collect();
        b.apply("t", &astromech_journal::JournalEvent::EngineerProgress { engineers: snaps });
        b.build()
    }

    fn straight(range: f64) -> impl Fn([f64; 3], [f64; 3]) -> Hop {
        move |a, b| {
            let ly = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
            Hop { ly, jumps: (ly / range).ceil() as u32, exact: false }
        }
    }

    #[test]
    fn trip_remote_pin_and_in_person_stops() {
        let fleet = from_journal_dir(&fixture_dir());
        let ship = fleet.ships.iter().find(|s| s.id == 7).unwrap();
        let r = sample_plan();
        let engs = engineers_with(&[
            ("Felicity Farseer", "Unlocked", Some(5)),
            ("Professor Palin", "Unlocked", Some(3)),
            ("Elvira Martuuk", "Invited", None),
        ]);
        assert!(engineers::info("Felicity Farseer").is_some());
        let user: std::collections::BTreeMap<String, String> =
            [("Felicity Farseer".to_string(), "FSD_LongRange".to_string())].into_iter().collect();
        let pins = engineers::effective_pins(&engs, &user);
        let hop = straight(20.0);
        let trip = plan_trip(ship, &r.targets, &engs, &pins, Some(("Sol", [0.0; 3])), 20.0, &hop);
        // FSD grades: Felicity is rank 5 with it pinned → remote
        assert_eq!(trip.remote.len(), 1);
        assert_eq!(trip.remote[0].engineer, "Felicity Farseer");
        assert_eq!(trip.remote[0].job.what, "Increased range G3 → G5");
        // in person: Mass Manager and Dirty drives G3 → G5. Palin works on FSDs too, so one
        // stop covers both.
        let names: Vec<&str> = trip.stops.iter().map(|s| s.engineer.as_str()).collect();
        assert_eq!(names, vec!["Professor Palin"]);
        let palin = &trip.stops[0];
        assert_eq!(palin.jobs.len(), 2);
        let dirty = palin.jobs.iter().find(|j| j.kind == "grades").unwrap();
        assert!(dirty.note.as_deref().unwrap().contains("rank 3"));
        assert_eq!(trip.total_jumps, palin.hop.jumps);
        assert!(trip.blocked.is_empty());

        // without the pin, FSD G4–G5 needs Felicity (Palin stops at G3): two stops
        let trip = plan_trip(ship, &r.targets, &engs, &Default::default(), Some(("Sol", [0.0; 3])), 20.0, &hop);
        assert!(trip.remote.is_empty());
        let names: Vec<&str> = trip.stops.iter().map(|s| s.engineer.as_str()).collect();
        assert_eq!(names.len(), 2, "{names:?}");
        assert!(trip
            .stops
            .iter()
            .any(|s| s.engineer == "Felicity Farseer" && s.jobs.iter().any(|j| j.what == "Increased range G3 → G5")));

        // nobody unlocked → everything blocked, with who to unlock
        let none = engineers_with(&[]);
        let trip = plan_trip(ship, &r.targets, &none, &Default::default(), None, 20.0, &hop);
        assert!(trip.stops.is_empty());
        assert_eq!(trip.blocked.len(), 3);
        assert!(trip.blocked[0].unlock.iter().any(|(n, _)| n == "Felicity Farseer"));
    }

    #[test]
    fn pin_inferred_from_remote_craft() {
        let mut b = engineers::EngineerBuilder::default();
        let deciat = engineers::info("Felicity Farseer").unwrap().id64;
        let jump = |id: i64| astromech_journal::JournalEvent::FsdJump {
            star_system: "x".into(),
            jump_dist: 1.0,
            allegiance: String::new(),
            population: 0,
            factions: vec![],
            from_carrier: false,
            system_address: Some(id),
            star_pos: None,
        };
        let craft = |bp: &str| astromech_journal::JournalEvent::EngineerCraft {
            engineer: "Felicity Farseer".into(),
            engineer_id: Some(300100),
            blueprint: bp.into(),
            level: 5,
            slot: None,
            ingredients: vec![],
        };
        b.apply("t1", &jump(deciat));
        b.apply("t2", &craft("FSD_FastBoot")); // in person: not a pin
        b.apply("t3", &jump(10477373803));
        b.apply("t4", &craft("FSD_LongRange")); // from Sol: remote → pinned
        let v = b.build();
        let f = v.iter().find(|e| e.name == "Felicity Farseer").unwrap();
        assert_eq!(f.inferred_pin.as_ref().unwrap().blueprint, "FSD_LongRange");
        assert!(f.offers.iter().any(|o| o.blueprint == "FSD_LongRange" && o.max_grade == 5));
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
    fn trades_and_farm_respect_storage_caps() {
        // 280 carbon (cap 300) and plenty of yttrium: a G4→G1 block gives 27, which would
        // overflow, so the trade is skipped and the 20 are farmed
        let mut ledger = astromech_materials::Ledger::default();
        ledger.counts.insert("carbon".into(), 280);
        ledger.counts.insert("yttrium".into(), 100);
        let need: std::collections::BTreeMap<String, i64> = [("carbon".to_string(), 300)].into_iter().collect();
        let (trades, left, _) = plan::suggest_trades(&need, &ledger);
        assert!(trades.iter().all(|t| t.get_id != "carbon" || 280 + t.get <= 300), "{trades:?}");
        assert_eq!(left.get("carbon"), Some(&20));
        // anchor already full: collecting more for trade-downs is flagged
        let mut ledger = astromech_materials::Ledger::default();
        ledger.counts.insert("polonium".into(), 150);
        let need: std::collections::BTreeMap<String, i64> =
            [("polonium".to_string(), 150), ("arsenic".to_string(), 10)].into_iter().collect();
        let (_, left, spare) = plan::suggest_trades(&need, &ledger);
        let route = plan::farm_route(&left, &spare, &ledger);
        let po = route.iter().find(|f| f.id == "polonium").unwrap();
        assert_eq!((po.collect, po.have, po.over_cap), (2, 150, true));
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
