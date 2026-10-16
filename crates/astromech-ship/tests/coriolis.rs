//! Reference builds against Coriolis.
//!
//! Each fixture in `tests/fixtures/` is a synthetic journal `Loadout` (made by hand, no personal
//! journal) without `Modifiers`, so Coriolis applies each blueprint at the fixture's `Quality`
//! the way astromech-ship does. The expected values are what Coriolis shows for that build, as
//! strings in its display format, produced by running Coriolis's own calculation code on the
//! fixture with node:
//!
//! - Coriolis EDCD/coriolis@68c042ca6e3db62372cbbb2077cf972345511712 (master, 24 Apr 2026) and
//!   EDCD/coriolis-data@0db9234b5b9ce8c939ea84133d7ce336eea88e27, the same data as
//!   `data/catalog`.
//! - `scripts/coriolis-ref/` bundles `JournalUtils.shipFromLoadoutJSON`, `Ship` and
//!   `Calculations` with esbuild, imports the fixture, rebuilds the ship from its build code as
//!   the import and outfitting pages do, and prints the summary-table values (`RUST=1` prints
//!   the blocks below).
//!
//! `tests/fixtures/plans/` holds the Krait MkII combat fixture with a build plan done, one file
//! per plan (see `check_plan`): the same Coriolis run on those gives the planned values, and the
//! Ships page's component tests (`app/src/modules/engineering/Build.test.ts`) show them.
//!
//! A value matches when it is within half a unit of Coriolis's last displayed digit: speeds,
//! masses, shields and armour to the unit, jump ranges and power to 0.01, resistances to 0.1 %,
//! DPS to 0.1, distributor values to 0.1 and time to drain to the second.

use astromech_engineering::{planned_modules, FleetBuilder, Target};
use astromech_journal::{parse_line, JournalEvent, ModuleSnap};
use astromech_ship::{compare, stats, ShipStats};

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

/// A build plan for the Krait MkII combat fixture, against `plans/krait_mkii_combat.<variant>`:
/// the fixture with each target's `Engineering` replaced by the planned blueprint, grade,
/// `Quality` 1.0 and experimental, which is what Coriolis was run on.
fn check_plan(variant: &str, targets: &[(&str, &str, &str)], expected: &[(&str, Option<&str>)]) {
    let targets: Vec<Target> = targets
        .iter()
        .map(|(slot, blueprint, experimental)| Target {
            slot: slot.to_string(),
            blueprint: blueprint.to_string(),
            grade: 5,
            experimental: Some(experimental.to_string()),
        })
        .collect();
    let (ship, base) = loadout("krait_mkii_combat");
    let (_, want) = loadout(&format!("plans/krait_mkii_combat.{variant}"));
    assert_eq!(planned_modules(&base, &targets), want, "{variant}: the plan doesn't give the fixture's modules");
    // Through the fleet, as the app asks for it.
    let mut fleet = FleetBuilder::default();
    let l = parse_line(&line("krait_mkii_combat")).unwrap();
    fleet.apply(&l.timestamp, &l.event);
    let c = compare(&fleet.build("test").ships[0], &targets).unwrap();
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
            ("sustained_dps", Some("136.2")),
            ("eps", Some("11.8")),
            ("time_to_drain", Some("0:21")),
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

#[test]
fn plan_thermal_boosters_4() {
    check_plan(
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
