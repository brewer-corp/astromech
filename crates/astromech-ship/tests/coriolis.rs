//! Reference builds against Coriolis.
//!
//! Each fixture in `tests/fixtures/` is a synthetic journal `Loadout` (made by hand, no personal
//! journal). Most have no `Modifiers`, so Coriolis applies each blueprint at the fixture's
//! `Quality`. `fer_de_lance_modifiers` (`fer_de_lance_combat` with every stat rolled on its own,
//! as the game does) and `anaconda_legacy` (`anaconda_explorer` with a drive and shield
//! generator engineered before 3.0: `Quality` 0, the real values only in `Modifiers`) carry
//! `Modifiers` in the game's units, which Coriolis then reads instead. The expected values are
//! what Coriolis shows for that build, as strings in its display format, produced by running
//! Coriolis's own calculation code on the fixture with node:
//!
//! - Coriolis EDCD/coriolis@68c042ca6e3db62372cbbb2077cf972345511712 (master, 24 Apr 2026) and
//!   EDCD/coriolis-data@0db9234b5b9ce8c939ea84133d7ce336eea88e27, the same data as
//!   `data/catalog`.
//! - `scripts/coriolis-ref/` bundles `JournalUtils.shipFromLoadoutJSON`, `Ship` and
//!   `Calculations` with esbuild, imports the fixture, rebuilds the ship from its build code as
//!   the import and outfitting pages do, and prints the summary-table values (`RUST=1` prints
//!   the blocks below).
//!
//! `tests/fixtures/plans/` holds fixtures with a build plan done, one file per plan (see
//! `check_plan`): the same Coriolis run on those gives the planned values, and the Ships page's
//! component tests (`app/src/modules/engineering/Build.test.ts`) show the Krait MkII ones.
//!
//! A value matches when it is within half a unit of Coriolis's last displayed digit: speeds,
//! masses, shields and armour to the unit, jump ranges and power to 0.01, resistances to 0.1 %,
//! DPS to 0.1, distributor values to 0.1 and time to drain to the second.

use astromech_engineering::{planned_modules, FleetBuilder, Target};
use astromech_journal::{parse_line, JournalEvent, ModuleSnap};
use astromech_ship::{compare, headlines, stats, Compared, ShipStats};

/// Decimals Coriolis shows for each stat (`int`, `f1`, `f2`, `round`, `round1`, `pct1`).
fn decimals(key: &str) -> i32 {
    match key {
        "jump_max" | "jump_unladen" | "jump_laden" | "jump_total_unladen" | "jump_total_laden" => 2,
        "power_generation" | "power_retracted" | "power_deployed" => 2,
        "fuel_capacity" | "cargo_capacity" => 2,
        "dps" | "sustained_dps" | "eps" => 1,
        k if k.starts_with("shield_") || k.starts_with("hull_") && k != "hull_mass" => 1,
        k if k.ends_with("_capacity") || k.ends_with("_recharge") => 1,
        _ => 0,
    }
}

fn value(s: &ShipStats, key: &str) -> Option<f64> {
    let pct = |x: f64| x * 100.0;
    let d = s.distributor.as_ref();
    let j = s.jump.as_ref();
    let sh = s.shield.as_ref();
    Some(match key {
        "hull_mass" => s.hull_mass,
        "unladen_mass" => s.unladen_mass,
        "laden_mass" => s.laden_mass,
        "fuel_capacity" => s.fuel_capacity,
        "cargo_capacity" => s.cargo_capacity,
        // Coriolis shows 0 when the drive is unpowered.
        "jump_max" => j.map_or(0.0, |j| j.max),
        "jump_unladen" => j.map_or(0.0, |j| j.unladen),
        "jump_laden" => j.map_or(0.0, |j| j.laden),
        "jump_total_unladen" => j.map_or(0.0, |j| j.total_unladen),
        "jump_total_laden" => j.map_or(0.0, |j| j.total_laden),
        "power_generation" => s.power.generation,
        "power_retracted" => s.power.retracted,
        "power_deployed" => s.power.deployed,
        "shield" => sh?.strength,
        "shield_explosive" => pct(sh?.explosive),
        "shield_kinetic" => pct(sh?.kinetic),
        "shield_thermal" => pct(sh?.thermal),
        "armour" => s.armour.integrity,
        "hull_explosive" => pct(s.armour.explosive),
        "hull_kinetic" => pct(s.armour.kinetic),
        "hull_thermal" => pct(s.armour.thermal),
        "hull_caustic" => pct(s.armour.caustic),
        "module_armour" => s.armour.module_armour,
        "dps" => s.weapons.dps,
        "sustained_dps" => s.weapons.sustained_dps,
        "eps" => s.weapons.eps,
        "time_to_drain" => s.time_to_drain(4.0)?,
        "wep_capacity" => d?.weapons.capacity,
        "wep_recharge" => d?.weapons.recharge,
        "sys_capacity" => d?.systems.capacity,
        "sys_recharge" => d?.systems.recharge,
        "eng_capacity" => d?.engines.capacity,
        "eng_recharge" => d?.engines.recharge,
        "speed" => s.speed.unwrap_or(0.0),
        "boost" => s.boost.unwrap_or(0.0),
        _ => panic!("unknown stat {key}"),
    })
}

/// The fixture as one journal line.
fn line(fixture: &str) -> String {
    let path = format!("{}/tests/fixtures/{fixture}.json", env!("CARGO_MANIFEST_DIR"));
    let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    json.to_string()
}

fn loadout(fixture: &str) -> (String, Vec<ModuleSnap>) {
    let JournalEvent::Loadout { ship, modules, .. } = parse_line(&line(fixture)).unwrap().event else {
        panic!("{fixture}: not a Loadout")
    };
    (ship.unwrap(), modules)
}

fn check(fixture: &str, expected: &[(&str, Option<&str>)]) {
    let (ship, modules) = loadout(fixture);
    check_stats(fixture, &stats(&ship, &modules).unwrap(), expected);
}

fn check_stats(fixture: &str, s: &ShipStats, expected: &[(&str, Option<&str>)]) {
    assert!(s.unresolved.is_empty(), "{fixture}: unresolved {:?}", s.unresolved);
    let mut wrong = vec![];
    for (key, want) in expected {
        let got = value(s, key);
        let ok = match (want, got) {
            (None, None) => true,
            // Time to drain shows whole seconds, m:ss.
            (Some(w), Some(g)) if *key == "time_to_drain" => {
                let (m, sec) = w.split_once(':').unwrap();
                g.floor() == m.parse::<f64>().unwrap() * 60.0 + sec.parse::<f64>().unwrap()
            }
            (Some(w), Some(g)) => (g - w.parse::<f64>().unwrap()).abs() <= 0.5 * 10f64.powi(-decimals(key)) + 1e-9,
            _ => false,
        };
        if !ok {
            wrong.push(format!("{key}: Coriolis {want:?}, astromech {got:?}"));
        }
    }
    assert!(wrong.is_empty(), "{fixture}:\n  {}", wrong.join("\n  "));
}

/// A build plan for the `fixture`, against `plans/<fixture>.<variant>`: the fixture with each
/// target's `Engineering` replaced by the planned blueprint, grade, `Quality` 1.0 and
/// experimental (no `Modifiers`), which is what Coriolis was run on.
fn check_plan(fixture: &str, variant: &str, targets: &[(&str, &str, &str)], expected: &[(&str, Option<&str>)]) {
    let targets: Vec<Target> = targets
        .iter()
        .map(|(slot, blueprint, experimental)| Target {
            slot: slot.to_string(),
            blueprint: blueprint.to_string(),
            grade: 5,
            experimental: Some(experimental.to_string()),
        })
        .collect();
    let (ship, base) = loadout(fixture);
    let (_, want) = loadout(&format!("plans/{fixture}.{variant}"));
    assert_eq!(planned_modules(&base, &targets), want, "{variant}: the plan doesn't give the fixture's modules");
    // Through the fleet, as the app asks for it.
    let mut fleet = FleetBuilder::default();
    let l = parse_line(&line(fixture)).unwrap();
    fleet.apply(&l.timestamp, &l.event);
    let Compared::Stats(c) = compare(&fleet.build("test").ships[0], &targets).unwrap() else { panic!("hull unknown") };
    assert_eq!(c.current.stats, stats(&ship, &base).unwrap());
    assert_eq!(c.planned.time_to_drain, c.planned.stats.time_to_drain(4.0));
    check_stats(variant, &c.planned.stats, expected);
}

#[test]
fn krait_mkii_combat() {
    check(
        "krait_mkii_combat",
        &[
            ("hull_mass", Some("320")),
            ("unladen_mass", Some("820")),
            ("laden_mass", Some("852")),
            ("fuel_capacity", Some("32")),
            ("cargo_capacity", Some("32")),
            ("jump_max", Some("24.87")),
            ("jump_unladen", Some("24.05")),
            ("jump_laden", Some("23.15")),
            ("jump_total_unladen", Some("163.74")),
            ("jump_total_laden", Some("157.49")),
            ("power_generation", Some("44.10")),
            ("power_retracted", Some("23.27")),
            ("power_deployed", Some("28.18")),
            ("shield", Some("1428")),
            ("shield_explosive", Some("52.9")),
            ("shield_kinetic", Some("43.4")),
            ("shield_thermal", Some("36.2")),
            ("armour", Some("2143")),
            ("hull_explosive", Some("-18.4")),
            ("hull_kinetic", Some("-1.5")),
            ("hull_thermal", Some("36.1")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("107.2")),
            ("sustained_dps", Some("92.5")),
            ("eps", Some("11.7")),
            ("time_to_drain", Some("1:10")),
            ("wep_capacity", Some("92.2")),
            ("wep_recharge", Some("9")),
            ("sys_capacity", Some("33.5")),
            ("sys_recharge", Some("4")),
            ("eng_capacity", Some("33.5")),
            ("eng_recharge", Some("3.5")),
            ("speed", Some("376")),
            ("boost", Some("516")),
        ],
    );
}

#[test]
fn fer_de_lance_combat() {
    check(
        "fer_de_lance_combat",
        &[
            ("hull_mass", Some("250")),
            ("unladen_mass", Some("487")),
            ("laden_mass", Some("487")),
            ("fuel_capacity", Some("8")),
            ("cargo_capacity", Some("0")),
            ("jump_max", Some("15.25")),
            ("jump_unladen", Some("15.10")),
            ("jump_laden", Some("15.10")),
            ("jump_total_unladen", Some("43.10")),
            ("jump_total_laden", Some("43.10")),
            ("power_generation", Some("27.47")),
            ("power_retracted", Some("18.19")),
            ("power_deployed", Some("24.89")),
            ("shield", Some("629")),
            ("shield_explosive", Some("61.8")),
            ("shield_kinetic", Some("51.2")),
            ("shield_thermal", Some("42.0")),
            ("armour", Some("1120")),
            ("hull_explosive", Some("12.9")),
            ("hull_kinetic", Some("42.1")),
            ("hull_thermal", Some("5.2")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("124.5")),
            // Coriolis shows 136.2: it leaves the rail guns' charge out of the reload cycle.
            ("sustained_dps", Some("108.8")),
            ("eps", Some("11.8")),
            // Coriolis shows 0:21: the rail guns' draw uses the same charge-and-reload cycle as
            // their damage (astromech-bnnh, astromech-xca4); the other weapons are unchanged.
            ("time_to_drain", Some("0:15")),
            ("wep_capacity", Some("45.6")),
            ("wep_recharge", Some("7.8")),
            ("sys_capacity", Some("31.9")),
            ("sys_recharge", Some("4.8")),
            ("eng_capacity", Some("31.9")),
            ("eng_recharge", Some("4.8")),
            ("speed", Some("356")),
            ("boost", Some("479")),
        ],
    );
}

#[test]
fn anaconda_explorer() {
    check(
        "anaconda_explorer",
        &[
            ("hull_mass", Some("400")),
            ("unladen_mass", Some("608")),
            ("laden_mass", Some("612")),
            ("fuel_capacity", Some("64")),
            ("cargo_capacity", Some("4")),
            ("jump_max", Some("74.44")),
            ("jump_unladen", Some("68.56")),
            ("jump_laden", Some("68.19")),
            ("jump_total_unladen", Some("571.37")),
            ("jump_total_laden", Some("568.04")),
            ("power_generation", Some("22.93")),
            ("power_retracted", Some("15.17")),
            ("power_deployed", Some("15.17")),
            ("shield", Some("361")),
            ("shield_explosive", Some("50.0")),
            ("shield_kinetic", Some("40.0")),
            ("shield_thermal", Some("-20.0")),
            ("armour", Some("898")),
            ("hull_explosive", Some("-19.0")),
            ("hull_kinetic", Some("-2.0")),
            ("hull_thermal", Some("15.0")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("0")),
            ("dps", Some("0.0")),
            ("sustained_dps", Some("0.0")),
            ("eps", Some("0.0")),
            ("time_to_drain", None),
            ("wep_capacity", Some("35.7")),
            ("wep_recharge", Some("3.8")),
            ("sys_capacity", Some("24.4")),
            ("sys_recharge", Some("2.3")),
            ("eng_capacity", Some("32.2")),
            ("eng_recharge", Some("2.9")),
            ("speed", Some("234")),
            ("boost", Some("312")),
        ],
    );
}

#[test]
fn krait_phantom_explorer() {
    check(
        "krait_phantom_explorer",
        &[
            ("hull_mass", Some("270")),
            ("unladen_mass", Some("400")),
            ("laden_mass", Some("408")),
            ("fuel_capacity", Some("40")),
            ("cargo_capacity", Some("8")),
            ("jump_max", Some("58.94")),
            ("jump_unladen", Some("54.76")),
            ("jump_laden", Some("53.89")),
            ("jump_total_unladen", Some("436.22")),
            ("jump_total_laden", Some("428.98")),
            ("power_generation", Some("22.85")),
            ("power_retracted", Some("14.35")),
            ("power_deployed", Some("14.95")),
            ("shield", Some("219")),
            ("shield_explosive", Some("52.5")),
            ("shield_kinetic", Some("46.4")),
            ("shield_thermal", Some("-17.4")),
            ("armour", Some("314")),
            ("hull_explosive", Some("-29.5")),
            ("hull_kinetic", Some("-11.0")),
            ("hull_thermal", Some("7.5")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("0")),
            ("dps", Some("9.6")),
            ("sustained_dps", Some("9.6")),
            ("eps", Some("1.9")),
            ("time_to_drain", None),
            ("wep_capacity", Some("28.2")),
            ("wep_recharge", Some("3.1")),
            ("sys_capacity", Some("20")),
            ("sys_recharge", Some("1.7")),
            ("eng_capacity", Some("30.1")),
            ("eng_recharge", Some("2.4")),
            ("speed", Some("301")),
            ("boost", Some("421")),
        ],
    );
}

#[test]
fn python_miner() {
    check(
        "python_miner",
        &[
            ("hull_mass", Some("350")),
            ("unladen_mass", Some("730")),
            ("laden_mass", Some("854")),
            ("fuel_capacity", Some("32")),
            ("cargo_capacity", Some("124")),
            ("jump_max", Some("26.24")),
            ("jump_unladen", Some("25.27")),
            ("jump_laden", Some("21.60")),
            ("jump_total_unladen", Some("172.40")),
            ("jump_total_laden", Some("146.97")),
            ("power_generation", Some("25.00")),
            ("power_retracted", Some("18.37")),
            ("power_deployed", Some("25.30")),
            ("shield", Some("467")),
            ("shield_explosive", Some("48.8")),
            ("shield_kinetic", Some("38.5")),
            ("shield_thermal", Some("-4.1")),
            ("armour", Some("655")),
            ("hull_explosive", Some("-45.6")),
            ("hull_kinetic", Some("-12.8")),
            ("hull_thermal", Some("-4.0")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("0")),
            ("dps", Some("34.5")),
            ("sustained_dps", Some("16.5")),
            ("eps", Some("19.1")),
            ("time_to_drain", Some("0:15")),
            ("wep_capacity", Some("79.1")),
            ("wep_recharge", Some("4.7")),
            ("sys_capacity", Some("52.3")),
            ("sys_recharge", Some("3")),
            ("eng_capacity", Some("52.3")),
            ("eng_recharge", Some("3")),
            ("speed", Some("369")),
            ("boost", Some("481")),
        ],
    );
}

// The distributor with its experimental: Super Conduits (as fitted, rolled to 100 %), then
// Cluster Capacitors.
#[test]
fn plan_pd_super_conduits() {
    check_plan(
        "krait_mkii_combat",
        "pd_super_conduits",
        &[("PowerDistributor", "PowerDistributor_PriorityWeapons", "special_powerdistributor_fast")],
        &[
            ("hull_mass", Some("320")),
            ("unladen_mass", Some("820")),
            ("laden_mass", Some("852")),
            ("fuel_capacity", Some("32")),
            ("cargo_capacity", Some("32")),
            ("jump_max", Some("24.87")),
            ("jump_unladen", Some("24.05")),
            ("jump_laden", Some("23.15")),
            ("jump_total_unladen", Some("163.74")),
            ("jump_total_laden", Some("157.49")),
            ("power_generation", Some("44.10")),
            ("power_retracted", Some("23.27")),
            ("power_deployed", Some("28.18")),
            ("shield", Some("1428")),
            ("shield_explosive", Some("52.9")),
            ("shield_kinetic", Some("43.4")),
            ("shield_thermal", Some("36.2")),
            ("armour", Some("2143")),
            ("hull_explosive", Some("-18.4")),
            ("hull_kinetic", Some("-1.5")),
            ("hull_thermal", Some("36.1")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("107.2")),
            ("sustained_dps", Some("92.5")),
            ("eps", Some("11.7")),
            ("time_to_drain", Some("1:18")),
            ("wep_capacity", Some("93.7")),
            ("wep_recharge", Some("9.1")),
            ("sys_capacity", Some("33.5")),
            ("sys_recharge", Some("4")),
            ("eng_capacity", Some("33.5")),
            ("eng_recharge", Some("3.5")),
            ("speed", Some("376")),
            ("boost", Some("516")),
        ],
    );
}

#[test]
fn plan_pd_cluster_capacitors() {
    check_plan(
        "krait_mkii_combat",
        "pd_cluster_capacitors",
        &[("PowerDistributor", "PowerDistributor_PriorityWeapons", "special_powerdistributor_capacity")],
        &[
            ("hull_mass", Some("320")),
            ("unladen_mass", Some("820")),
            ("laden_mass", Some("852")),
            ("fuel_capacity", Some("32")),
            ("cargo_capacity", Some("32")),
            ("jump_max", Some("24.87")),
            ("jump_unladen", Some("24.05")),
            ("jump_laden", Some("23.15")),
            ("jump_total_unladen", Some("163.74")),
            ("jump_total_laden", Some("157.49")),
            ("power_generation", Some("44.10")),
            ("power_retracted", Some("23.27")),
            ("power_deployed", Some("28.18")),
            ("shield", Some("1428")),
            ("shield_explosive", Some("52.9")),
            ("shield_kinetic", Some("43.4")),
            ("shield_thermal", Some("36.2")),
            ("armour", Some("2143")),
            ("hull_explosive", Some("-18.4")),
            ("hull_kinetic", Some("-1.5")),
            ("hull_thermal", Some("36.1")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("107.2")),
            ("sustained_dps", Some("92.5")),
            ("eps", Some("11.7")),
            ("time_to_drain", Some("1:01")),
            ("wep_capacity", Some("105.4")),
            ("wep_recharge", Some("8.6")),
            ("sys_capacity", Some("37.6")),
            ("sys_recharge", Some("3.7")),
            ("eng_capacity", Some("37.6")),
            ("eng_recharge", Some("3.3")),
            ("speed", Some("376")),
            ("boost", Some("516")),
        ],
    );
}

// The drive from Increased range to Faster boot sequence, keeping Mass Manager.
#[test]
fn plan_fsd_fast_boot() {
    check_plan(
        "krait_mkii_combat",
        "fsd_fast_boot",
        &[("FrameShiftDrive", "FSD_FastBoot", "special_fsd_heavy")],
        &[
            ("hull_mass", Some("320")),
            ("unladen_mass", Some("814")),
            ("laden_mass", Some("846")),
            ("fuel_capacity", Some("32")),
            ("cargo_capacity", Some("32")),
            ("jump_max", Some("18.71")),
            ("jump_unladen", Some("18.09")),
            ("jump_laden", Some("17.41")),
            ("jump_total_unladen", Some("123.19")),
            ("jump_total_laden", Some("118.45")),
            ("power_generation", Some("44.10")),
            ("power_retracted", Some("23.18")),
            ("power_deployed", Some("28.09")),
            ("shield", Some("1428")),
            ("shield_explosive", Some("52.9")),
            ("shield_kinetic", Some("43.4")),
            ("shield_thermal", Some("36.2")),
            ("armour", Some("2143")),
            ("hull_explosive", Some("-18.4")),
            ("hull_kinetic", Some("-1.5")),
            ("hull_thermal", Some("36.1")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("107.2")),
            ("sustained_dps", Some("92.5")),
            ("eps", Some("11.7")),
            ("time_to_drain", Some("1:10")),
            ("wep_capacity", Some("92.2")),
            ("wep_recharge", Some("9")),
            ("sys_capacity", Some("33.5")),
            ("sys_recharge", Some("4")),
            ("eng_capacity", Some("33.5")),
            ("eng_recharge", Some("3.5")),
            ("speed", Some("376")),
            ("boost", Some("517")),
        ],
    );
}

// Three, then four, thermal-resistant shield boosters in place of two (Heavy duty swapped for
// Thermal resistance with Thermo Block).
#[test]
fn plan_thermal_boosters_3() {
    check_plan(
        "krait_mkii_combat",
        "thermal_boosters_3",
        &[("TinyHardpoint1", "ShieldBooster_Thermic", "special_shieldbooster_thermic")],
        &[
            ("hull_mass", Some("320")),
            ("unladen_mass", Some("809")),
            ("laden_mass", Some("841")),
            ("fuel_capacity", Some("32")),
            ("cargo_capacity", Some("32")),
            ("jump_max", Some("25.21")),
            ("jump_unladen", Some("24.36")),
            ("jump_laden", Some("23.44")),
            ("jump_total_unladen", Some("165.90")),
            ("jump_total_laden", Some("159.49")),
            ("power_generation", Some("44.10")),
            ("power_retracted", Some("22.97")),
            ("power_deployed", Some("27.88")),
            ("shield", Some("1197")),
            ("shield_explosive", Some("51.0")),
            ("shield_kinetic", Some("41.2")),
            ("shield_thermal", Some("43.9")),
            ("armour", Some("2143")),
            ("hull_explosive", Some("-18.4")),
            ("hull_kinetic", Some("-1.5")),
            ("hull_thermal", Some("36.1")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("107.2")),
            ("sustained_dps", Some("92.5")),
            ("eps", Some("11.7")),
            ("time_to_drain", Some("1:10")),
            ("wep_capacity", Some("92.2")),
            ("wep_recharge", Some("9")),
            ("sys_capacity", Some("33.5")),
            ("sys_recharge", Some("4")),
            ("eng_capacity", Some("33.5")),
            ("eng_recharge", Some("3.5")),
            ("speed", Some("377")),
            ("boost", Some("518")),
        ],
    );
}

// Sustained DPS counts the last burst's charge (decision astromech-bnnh), so it departs from
// Coriolis for the rail guns and never exceeds burst. Worked by hand from each weapon's
// engineered stats; the cycle is the shots after the first, the burst overhead, the charge and
// the reload, and the other weapons keep Coriolis's number:
//   multi-cannon: 5.882 damage x 2 rounds a shot, 3.19875 shots/s, clip 77, reload 5 s, no charge:
//     77 / (76 / 3.19875 + 5) = 2.6775 shots/s, 31.497 DPS.
//   rail gun 1: 70.882 damage, 0.83 s interval + 1.2 s charge, clip 1, reload 1.5 s: one shot
//     per 1.2 + 1.5 = 2.7 s (Coriolis: 1.5 s), 26.253 DPS.
//   rail gun 2: 33.2 damage, 1 / 1.291339 s interval + 1.2 s charge, clip 2, reload 1 s: two
//     shots per (1.2 + 1 / 1.291339 + 1.2) + 1 s, 15.907 DPS (the shot rate counts the charge).
//   pulse laser 16.406 DPS and beam laser 18.759 DPS: no clip, unchanged.
// Total 108.82 DPS, against 124.53 burst (Coriolis: 136.2).
#[test]
fn fer_de_lance_sustained_counts_charge() {
    let (ship, modules) = loadout("fer_de_lance_combat");
    let s = stats(&ship, &modules).unwrap();
    let multi = 5.882 * 2.0 * 77.0 / (76.0 / 3.1987491611280325 + 5.0);
    let rail1 = 70.882 / (1.2 + 1.5);
    let shot2 = 1.0 / 1.291338989398107 + 1.2;
    let rail2 = 2.0 * 33.2 / (shot2 + 1.2 + 1.0);
    let expected: f64 = multi + rail1 + rail2 + 16.406199799012665 + 18.759384;
    assert!((expected - 108.82).abs() < 0.01, "hand value {expected}");
    assert!((s.weapons.sustained_dps - expected).abs() < 0.01, "{}", s.weapons.sustained_dps);
    assert!(s.weapons.sustained_dps <= s.weapons.dps);
}

#[test]
fn plan_thermal_boosters_4() {
    check_plan(
        "krait_mkii_combat",
        "thermal_boosters_4",
        &[
            ("TinyHardpoint1", "ShieldBooster_Thermic", "special_shieldbooster_thermic"),
            ("TinyHardpoint2", "ShieldBooster_Thermic", "special_shieldbooster_thermic"),
        ],
        &[
            ("hull_mass", Some("320")),
            ("unladen_mass", Some("799")),
            ("laden_mass", Some("831")),
            ("fuel_capacity", Some("32")),
            ("cargo_capacity", Some("32")),
            ("jump_max", Some("25.55")),
            ("jump_unladen", Some("24.68")),
            ("jump_laden", Some("23.73")),
            ("jump_total_unladen", Some("168.12")),
            ("jump_total_laden", Some("161.54")),
            ("power_generation", Some("44.10")),
            ("power_retracted", Some("22.67")),
            ("power_deployed", Some("27.58")),
            ("shield", Some("967")),
            ("shield_explosive", Some("49.0")),
            ("shield_kinetic", Some("38.8")),
            ("shield_thermal", Some("49.5")),
            ("armour", Some("2143")),
            ("hull_explosive", Some("-18.4")),
            ("hull_kinetic", Some("-1.5")),
            ("hull_thermal", Some("36.1")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("107.2")),
            ("sustained_dps", Some("92.5")),
            ("eps", Some("11.7")),
            ("time_to_drain", Some("1:10")),
            ("wep_capacity", Some("92.2")),
            ("wep_recharge", Some("9")),
            ("sys_capacity", Some("33.5")),
            ("sys_recharge", Some("4")),
            ("eng_capacity", Some("33.5")),
            ("eng_recharge", Some("3.5")),
            ("speed", Some("378")),
            ("boost", Some("519")),
        ],
    );
}

// Every stat of every engineered module from `Modifiers`, each rolled apart from `Quality`.
#[test]
fn fer_de_lance_modifiers() {
    check(
        "fer_de_lance_modifiers",
        &[
            ("hull_mass", Some("250")),
            ("unladen_mass", Some("487")),
            ("laden_mass", Some("487")),
            ("fuel_capacity", Some("8")),
            ("cargo_capacity", Some("0")),
            ("jump_max", Some("15.22")),
            ("jump_unladen", Some("15.06")),
            ("jump_laden", Some("15.06")),
            ("jump_total_unladen", Some("43.00")),
            ("jump_total_laden", Some("43.00")),
            ("power_generation", Some("27.55")),
            ("power_retracted", Some("18.19")),
            ("power_deployed", Some("24.89")),
            ("shield", Some("629")),
            ("shield_explosive", Some("61.8")),
            ("shield_kinetic", Some("50.4")),
            ("shield_thermal", Some("37.5")),
            ("armour", Some("1117")),
            ("hull_explosive", Some("12.9")),
            ("hull_kinetic", Some("43.7")),
            ("hull_thermal", Some("8.1")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("235")),
            ("dps", Some("123.7")),
            // Coriolis shows 135.0, for the same reason.
            ("sustained_dps", Some("108.3")),
            ("eps", Some("11.8")),
            // Coriolis shows 0:19, for the same reason.
            ("time_to_drain", Some("0:14")),
            ("wep_capacity", Some("45.6")),
            ("wep_recharge", Some("7.6")),
            ("sys_capacity", Some("31.9")),
            ("sys_recharge", Some("4.7")),
            ("eng_capacity", Some("31.9")),
            ("eng_recharge", Some("4.8")),
            ("speed", Some("354")),
            ("boost", Some("477")),
        ],
    );
}

// A pre-3.0 Increased range drive (+53.1 % optimal mass) and Reinforced shield generator, both
// `Quality` 0: the stats come from their `Modifiers`, not grade 5 (or 3) at the bottom of its roll.
#[test]
fn anaconda_legacy() {
    check(
        "anaconda_legacy",
        &[
            ("hull_mass", Some("400")),
            ("unladen_mass", Some("609")),
            ("laden_mass", Some("613")),
            ("fuel_capacity", Some("64")),
            ("cargo_capacity", Some("4")),
            ("jump_max", Some("71.16")),
            ("jump_unladen", Some("65.59")),
            ("jump_laden", Some("65.23")),
            ("jump_total_unladen", Some("546.39")),
            ("jump_total_laden", Some("543.23")),
            ("power_generation", Some("22.93")),
            ("power_retracted", Some("15.24")),
            ("power_deployed", Some("15.24")),
            ("shield", Some("356")),
            ("shield_explosive", Some("50.0")),
            ("shield_kinetic", Some("40.0")),
            ("shield_thermal", Some("-20.0")),
            ("armour", Some("898")),
            ("hull_explosive", Some("-21.3")),
            ("hull_kinetic", Some("-2.8")),
            ("hull_thermal", Some("14.6")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("0")),
            ("dps", Some("0.0")),
            ("sustained_dps", Some("0.0")),
            ("eps", Some("0.0")),
            ("time_to_drain", None),
            ("wep_capacity", Some("35.7")),
            ("wep_recharge", Some("3.8")),
            ("sys_capacity", Some("24.4")),
            ("sys_recharge", Some("2.3")),
            ("eng_capacity", Some("33.6")),
            ("eng_recharge", Some("2.9")),
            ("speed", Some("230")),
            ("boost", Some("306")),
        ],
    );
    // Read from the blueprint and roll alone, the drive would be at +45 %.
    let (ship, mut modules) = loadout("anaconda_legacy");
    for m in &mut modules {
        if let Some(e) = &mut m.engineering {
            e.modifiers.clear();
        }
    }
    let rolled = stats(&ship, &modules).unwrap().jump.unwrap().max;
    assert!(rolled < 69.0, "blueprint and roll give {rolled} ly");
}

// The legacy drive replaced by Increased range G5 with Mass Manager: the planned drive follows the
// blueprint, every other module keeps its journal values.
#[test]
fn plan_legacy_fsd_long_range() {
    check_plan(
        "anaconda_legacy",
        "fsd_long_range",
        &[("FrameShiftDrive", "FSD_LongRange", "special_fsd_heavy")],
        &[
            ("hull_mass", Some("400")),
            ("unladen_mass", Some("609")),
            ("laden_mass", Some("613")),
            ("fuel_capacity", Some("64")),
            ("cargo_capacity", Some("4")),
            ("jump_max", Some("74.37")),
            ("jump_unladen", Some("68.51")),
            ("jump_laden", Some("68.13")),
            ("jump_total_unladen", Some("570.86")),
            ("jump_total_laden", Some("567.53")),
            ("power_generation", Some("22.93")),
            ("power_retracted", Some("15.24")),
            ("power_deployed", Some("15.24")),
            ("shield", Some("356")),
            ("shield_explosive", Some("50.0")),
            ("shield_kinetic", Some("40.0")),
            ("shield_thermal", Some("-20.0")),
            ("armour", Some("898")),
            ("hull_explosive", Some("-21.3")),
            ("hull_kinetic", Some("-2.8")),
            ("hull_thermal", Some("14.6")),
            ("hull_caustic", Some("0.0")),
            ("module_armour", Some("0")),
            ("dps", Some("0.0")),
            ("sustained_dps", Some("0.0")),
            ("eps", Some("0.0")),
            ("time_to_drain", None),
            ("wep_capacity", Some("35.7")),
            ("wep_recharge", Some("3.8")),
            ("sys_capacity", Some("24.4")),
            ("sys_recharge", Some("2.3")),
            ("eng_capacity", Some("33.6")),
            ("eng_recharge", Some("2.9")),
            ("speed", Some("230")),
            ("boost", Some("306")),
        ],
    );
}

/// The card stats come from the same calculations: the Krait MkII combat fixture's shield, DPS
/// and maximum jump as Coriolis shows them (`krait_mkii_combat` above).
#[test]
fn card_headline_for_the_krait_mkii_combat_build() {
    let mut fleet = FleetBuilder::default();
    let l = parse_line(&line("krait_mkii_combat")).unwrap();
    fleet.apply(&l.timestamp, &l.event);
    let h = headlines(&fleet.build("test"));
    let k = &h[&1];
    assert!(k.unresolved.is_empty());
    assert_eq!(format!("{:.0}", k.shield.unwrap()), "1428");
    assert_eq!(format!("{:.1}", k.dps.unwrap()), "107.2");
    assert_eq!(format!("{:.2}", k.jump.unwrap()), "24.87");
}

/// What a route plotter is given (Spansh's exact plotter, astromech-data `ExactRoute`): the
/// drive with its journal roll, the masses without main-tank fuel, and the booster; its range
/// with a full tank is the laden and unladen range above.
#[test]
fn fsd_for_route_plotters() {
    let (ship, modules) = loadout("anaconda_legacy");
    let s = stats(&ship, &modules).unwrap();
    let f = s.fsd.clone().expect("a powered drive");
    assert_eq!(f.optimal_mass, 2755.8, "the journal's FSDOptimalMass modifier");
    assert_eq!((f.max_fuel_per_jump, f.fuel_multiplier, f.fuel_power), (8.0, 0.012, 2.6), "6A drive");
    assert_eq!(f.tank_size, s.fuel_capacity);
    assert_eq!(f.reserve_size, 1.07, "Anaconda reservoir");
    assert!((f.base_mass - (s.unladen_mass - s.fuel_capacity + 1.07)).abs() < 1e-9);
    assert_eq!(f.range_boost, 10.5, "size 5 Guardian booster");
    let j = s.jump.unwrap();
    assert!((f.range(0.0) - j.unladen).abs() < 1e-9, "{} vs {}", f.range(0.0), j.unladen);
    assert!((f.range(s.cargo_capacity) - j.laden).abs() < 1e-9);
    assert!(f.range(100.0) < f.range(0.0));

    // No drive, no profile.
    let no_fsd: Vec<_> = modules.into_iter().filter(|m| m.slot != "FrameShiftDrive").collect();
    assert_eq!(stats(&ship, &no_fsd).unwrap().fsd, None);
}
