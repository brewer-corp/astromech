//! # astromech-engineering
//!
//! Ship engineering planner (ARCHITECTURE.md §18.7 M3): every owned ship and its fitted
//! modules from journal history, a build plan per ship (blueprint, grade and experimental per
//! slot), and the materials it costs against the inventory from `astromech-materials`, with
//! trader swaps from surplus and farming notes for the rest.

pub mod data;
pub mod engineers;
pub mod fleet;
pub mod links;
pub mod plan;
pub mod trip;

pub use engineers::EngineerView;
pub use fleet::{from_journal_dir, Applied, Fitted, Fleet, FleetBuilder, Ship, ShipLinks, Stored};
pub use plan::{evaluate, planned_modules, FarmStep, MatLine, Plan, PlanResult, Rolls, Target, TargetCost, Trade};
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
        b.apply(&astromech_journal::JournalEvent::EngineerProgress { engineers: snaps });
        b.build()
    }

    fn straight(range: f64) -> impl Fn([f64; 3], [f64; 3]) -> Hop {
        move |a, b| {
            let ly = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
            Hop { ly, jumps: (ly / range).ceil() as u32, exact: false }
        }
    }

    #[test]
    fn ship_links_follow_the_loadout_until_a_craft() {
        let loadout: serde_json::Value = serde_json::from_str(include_str!("../tests/links/loadout-asp.json")).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("Journal.2026-10-03T090000.01.log");
        let line = serde_json::to_string(&loadout).unwrap();
        std::fs::write(&log, format!("{line}\n")).unwrap();
        let ship = from_journal_dir(dir.path()).ships.remove(0);
        let links = ship.links.expect("a flown ship has links");
        assert_eq!(links.coriolis, links::coriolis_url(&loadout));
        assert_eq!(links.edsy, links::edsy_url(&loadout));

        let craft = r#"{"timestamp":"2026-10-03T09:30:00Z","event":"EngineerCraft","Slot":"FrameShiftDrive","Module":"int_hyperdrive_size5_class5","Engineer":"Felicity Farseer","BlueprintName":"FSD_LongRange","Level":5,"Quality":1.0}"#;
        std::fs::write(&log, format!("{line}\n{craft}\n")).unwrap();
        assert_eq!(from_journal_dir(dir.path()).ships[0].links, None, "the saved Loadout is stale after a craft");

        // The game writes a new Loadout after the craft: the links are back, for the new build.
        let mut newer = loadout.clone();
        newer["timestamp"] = "2026-10-03T09:45:00Z".into();
        let newer_line = serde_json::to_string(&newer).unwrap();
        std::fs::write(&log, format!("{line}\n{craft}\n{newer_line}\n")).unwrap();
        let links = from_journal_dir(dir.path()).ships.remove(0).links.expect("a later Loadout restores the links");
        assert_eq!(links.coriolis, links::coriolis_url(&newer));
    }

    #[test]
    fn trip_in_person_stops_and_remote_note() {
        let fleet = from_journal_dir(&fixture_dir());
        let ship = fleet.ships.iter().find(|s| s.id == 7).unwrap();
        let r = sample_plan();
        let engs = engineers_with(&[
            ("Felicity Farseer", "Unlocked", Some(5)),
            ("Professor Palin", "Unlocked", Some(3)),
            ("Elvira Martuuk", "Invited", None),
        ]);
        assert!(engineers::info("Felicity Farseer").is_some());
        let hop = straight(20.0);
        let trip = plan_trip(ship, &r.targets, &engs, Some(("Sol", [0.0; 3])), 20.0, &hop);
        // FSD G4–G5 needs Felicity (Palin stops at G3); Palin does Dirty drives and, as he
        // works on FSDs too, can take Mass Manager: two stops
        let names: Vec<&str> = trip.stops.iter().map(|s| s.engineer.as_str()).collect();
        assert_eq!(names.len(), 2, "{names:?}");
        let fel = trip.stops.iter().find(|s| s.engineer == "Felicity Farseer").unwrap();
        let fsd = fel.jobs.iter().find(|j| j.what == "Increased range G3 → G5").unwrap();
        // Felicity is rank 5: the grades could be remote if pinned there
        assert!(fsd.note.as_deref().unwrap().contains("pinned at Felicity Farseer"));
        let palin = trip.stops.iter().find(|s| s.engineer == "Professor Palin").unwrap();
        let dirty = palin.jobs.iter().find(|j| j.kind == "grades").unwrap();
        assert!(dirty.note.as_deref().unwrap().contains("rank 3"));
        assert_eq!(trip.total_jumps, trip.stops.iter().map(|s| s.hop.jumps).sum::<u32>());
        assert!(trip.blocked.is_empty());

        // nobody unlocked → everything blocked, with who to unlock
        let none = engineers_with(&[]);
        let trip = plan_trip(ship, &r.targets, &none, None, 20.0, &hop);
        assert!(trip.stops.is_empty());
        assert_eq!(trip.blocked.len(), 3);
        assert!(trip.blocked[0].unlock.iter().any(|(n, _)| n == "Felicity Farseer"));
    }

    #[test]
    fn experimental_never_before_its_blueprint() {
        // New blueprint (Faster boot G5: Felicity) plus Mass Manager on the FSD, and Dirty
        // drives G5 (only Palin). Starting at Palin, the cheapest route visits Palin first; he
        // could apply Mass Manager, but the new blueprint at Felicity afterwards would wipe it.
        let fleet = from_journal_dir(&fixture_dir());
        let ship = fleet.ships.iter().find(|s| s.id == 7).unwrap();
        let plan = Plan {
            ship_id: 7,
            targets: vec![
                Target {
                    slot: "FrameShiftDrive".into(),
                    blueprint: "FSD_FastBoot".into(),
                    grade: 5,
                    experimental: Some("special_fsd_heavy".into()),
                },
                Target { slot: "MainEngines".into(), blueprint: "Engine_Dirty".into(), grade: 5, experimental: None },
            ],
            rolls: Rolls::default(),
        };
        let ledger = astromech_materials::Ledger::default();
        let r = evaluate(ship, &plan, &ledger);
        let engs =
            engineers_with(&[("Felicity Farseer", "Unlocked", Some(5)), ("Professor Palin", "Unlocked", Some(5))]);
        let arque = engineers::info("Professor Palin").unwrap().coords;
        let trip = plan_trip(ship, &r.targets, &engs, Some(("Arque", arque)), 20.0, &straight(20.0));
        let stop_of =
            |what: &str| trip.stops.iter().position(|s| s.jobs.iter().any(|j| j.what.contains(what))).unwrap();
        assert_eq!(trip.stops[0].engineer, "Professor Palin");
        assert!(stop_of("Mass Manager") >= stop_of("Faster boot"), "{:?}", trip.stops);
        assert_eq!(trip.stops[stop_of("Mass Manager")].engineer, "Felicity Farseer");
        // no start: first leg is not claimed exact
        let trip = plan_trip(ship, &r.targets, &engs, None, 20.0, &straight(20.0));
        assert!(!trip.stops[0].hop.exact);
    }

    #[test]
    fn fixture_fleet_lists_every_ship_and_stored_module() {
        let f = sample_fleet();
        let ships: Vec<_> =
            f.ships.iter().map(|s| (s.id, s.system.as_deref(), s.value, s.rebuy, s.max_jump_range)).collect();
        assert_eq!(
            ships,
            vec![
                (3, None, Some(165_000_000), Some(12_000_000), Some(18.0)),
                (7, Some("Shinrarta Dezhra"), Some(134_000_000), Some(6_700_000), Some(32.5)),
                (12, Some("Sol"), Some(6_000_000), None, None),
            ]
        );
        let stored: Vec<_> = f
            .stored_modules
            .iter()
            .map(|m| (m.name.as_str(), m.system.as_deref(), m.blueprint_name.as_deref(), m.grade))
            .collect();
        assert_eq!(
            stored,
            vec![
                ("5A Frame Shift Drive", Some("Shinrarta Dezhra"), Some("Increased range"), Some(3)),
                ("5A Shield Generator", Some("Sol"), None, None),
                ("2F Pulse Laser", None, None, None),
            ]
        );
    }

    #[test]
    fn transfer_marks_a_ship_in_transit_to_where_we_are() {
        let mut b = FleetBuilder::default();
        for p in astromech_journal::journal_logs(&fixture_dir()) {
            for l in astromech_journal::parse_file(&p) {
                b.apply(&l.timestamp, &l.event);
            }
        }
        let l = astromech_journal::parse_line(
            r#"{"timestamp":"t","event":"ShipyardTransfer","ShipType":"asp","ShipID":12,"System":"Sol"}"#,
        )
        .unwrap();
        b.apply(&l.timestamp, &l.event);
        let asp = b.build("test").ships.into_iter().find(|s| s.id == 12).unwrap();
        assert!(asp.in_transit);
        assert_eq!(asp.system.as_deref(), Some("Shinrarta Dezhra"));
    }

    #[test]
    fn crafts_update_the_fleet_and_shrink_the_trip() {
        use astromech_journal::JournalEvent;
        let craft = |bp: &str, level: u8, quality: f64, slot: &str, exp: Option<&str>| JournalEvent::EngineerCraft {
            engineer: "Felicity Farseer".into(),
            engineer_id: Some(300100),
            blueprint: bp.into(),
            level,
            quality,
            slot: Some(slot.into()),
            experimental: exp.map(str::to_string),
            experimental_localised: None,
            preview: false,
            ingredients: vec![],
        };
        let mut b = FleetBuilder::default();
        for p in astromech_journal::journal_logs(&fixture_dir()) {
            for l in astromech_journal::parse_file(&p) {
                b.apply(&l.timestamp, &l.event);
            }
        }
        // the fixture's current ship is the Anaconda (3); switch to the Krait by flying it
        let krait_loadout = astromech_journal::parse_file(&fixture_dir().join("Journal.2026-09-20T180000.01.log"))
            .into_iter()
            .find(|l| matches!(l.event, JournalEvent::Loadout { ship_id: Some(7), .. }))
            .unwrap();
        b.apply("2026-10-04T10:00:00Z", &krait_loadout.event);
        // FSD G5 at full progress plus Mass Manager, rolled remotely or not: same events
        b.apply("2026-10-04T10:05:00Z", &craft("FSD_LongRange", 5, 1.0, "FrameShiftDrive", Some("special_fsd_heavy")));
        // a preview changes nothing
        let mut preview = craft("Engine_Dirty", 5, 1.0, "MainEngines", None);
        if let JournalEvent::EngineerCraft { preview: p, .. } = &mut preview {
            *p = true;
        }
        b.apply("2026-10-04T10:06:00Z", &preview);
        let fleet = b.build("test");
        let krait = fleet.ships.iter().find(|s| s.id == 7).unwrap();
        let fsd = krait.modules.iter().find(|m| m.slot == "FrameShiftDrive").unwrap().engineering.clone().unwrap();
        assert_eq!((fsd.grade, fsd.quality, fsd.experimental.as_deref()), (5, 1.0, Some("special_fsd_heavy")));
        let thr = krait.modules.iter().find(|m| m.slot == "MainEngines").unwrap().engineering.clone().unwrap();
        assert_eq!(thr.grade, 3, "preview ignored");
        assert_eq!(thr.experimental.as_deref(), Some("special_engine_overloaded"));
        assert_eq!(krait.loadout_ts.as_deref(), Some("2026-10-04T10:05:00Z"));
        // the FSD target is now done, so the trip only has the thrusters
        let ledger = astromech_materials::replay_journal_dir(&fixture_dir());
        let r = evaluate(krait, &sample_plan_input(), &ledger);
        assert_eq!(r.targets[0].note.as_deref(), Some("Already done."));
        let engs =
            engineers_with(&[("Felicity Farseer", "Unlocked", Some(5)), ("Professor Palin", "Unlocked", Some(3))]);
        let trip = plan_trip(krait, &r.targets, &engs, Some(("Sol", [0.0; 3])), 20.0, &straight(20.0));
        let jobs: Vec<&str> = trip.stops.iter().flat_map(|s| s.jobs.iter().map(|j| j.what.as_str())).collect();
        assert_eq!(jobs, vec!["Dirty G3 → G5"]);
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
    #[test]
    fn planned_modules_follow_the_plan_like_the_game() {
        let f = from_journal_dir(&fixture_dir());
        let krait = f.ships.iter().find(|s| s.id == 7).unwrap();
        let t = |slot: &str, bp: &str, g, x: Option<&str>| Target {
            slot: slot.into(),
            blueprint: bp.into(),
            grade: g,
            experimental: x.map(Into::into),
        };
        let eng = |mods: &[astromech_journal::ModuleSnap], slot: &str| {
            let e = mods.iter().find(|m| m.slot == slot).unwrap().engineering.clone();
            e.map(|e| (e.blueprint, e.level, e.quality, e.experimental))
        };
        // Up to the target grade, fully rolled, with the target's experimental.
        let p = planned_modules(&krait.loadout, &[t("FrameShiftDrive", "fsd_longrange", 5, Some("special_fsd_heavy"))]);
        assert_eq!(
            eng(&p, "FrameShiftDrive"),
            Some(("FSD_LongRange".into(), 5, 1.0, Some("special_fsd_heavy".into())))
        );
        // Already above the target on the same blueprint: grade, roll and experimental stay.
        let p = planned_modules(&krait.loadout, &[t("MainEngines", "Engine_Dirty", 2, None)]);
        assert_eq!(
            eng(&p, "MainEngines"),
            Some(("Engine_Dirty".into(), 3, 0.5, Some("special_engine_overloaded".into())))
        );
        // Same blueprint, higher grade: the experimental stays; another blueprint drops it.
        let p = planned_modules(&krait.loadout, &[t("MainEngines", "Engine_Dirty", 5, None)]);
        assert_eq!(
            eng(&p, "MainEngines"),
            Some(("Engine_Dirty".into(), 5, 1.0, Some("special_engine_overloaded".into())))
        );
        let p = planned_modules(&krait.loadout, &[t("MainEngines", "Engine_Reinforced", 5, None)]);
        assert_eq!(eng(&p, "MainEngines"), Some(("Engine_Reinforced".into(), 5, 1.0, None)));
        // A grade past the blueprint's top clamps to it; an experimental the module can't take is
        // ignored; targets cost_target rejects change nothing.
        let p = planned_modules(
            &krait.loadout,
            &[
                t("TinyHardpoint1", "ShieldBooster_Thermic", 9, Some("special_fsd_heavy")),
                t("FrameShiftDrive", "Engine_Dirty", 5, None),
                t("PowerPlant", "Not_A_Blueprint", 5, None),
                t("Slot09_Size1", "FSD_LongRange", 5, None),
            ],
        );
        assert_eq!(eng(&p, "TinyHardpoint1"), Some(("ShieldBooster_Thermic".into(), 5, 1.0, None)));
        assert_eq!(eng(&p, "FrameShiftDrive"), eng(&krait.loadout, "FrameShiftDrive"));
        assert_eq!(eng(&p, "PowerPlant"), eng(&krait.loadout, "PowerPlant"));
        assert_eq!(p.len(), krait.loadout.len());
    }
}
