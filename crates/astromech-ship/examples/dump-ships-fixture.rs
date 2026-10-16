//! Print the Ships page's stats fixture: the synthetic Krait MkII combat build from
//! `tests/fixtures` as a fleet ship, and its current and planned stats for the build plans the
//! Coriolis tests check (`tests/coriolis.rs`, `check_plan`), plus an empty plan; and the ship
//! card stats for it and for the sample journal's fleet.
//! `cargo run -q -p astromech-ship --example dump-ships-fixture > app/src/lib/fixtures/ships.json`
use astromech_engineering::{FleetBuilder, Plan, Rolls, Target};

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/krait_mkii_combat.json");
    let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let line = astromech_journal::parse_line(&json.to_string()).unwrap();
    let mut b = FleetBuilder::default();
    b.apply(&line.timestamp, &line.event);
    let fleet = b.build("Sample journal");
    let ship = &fleet.ships[0];
    let t = |slot: &str, blueprint: &str, experimental: &str| Target {
        slot: slot.into(),
        blueprint: blueprint.into(),
        grade: 5,
        experimental: Some(experimental.into()),
    };
    let thermal = |slot: &str| t(slot, "ShieldBooster_Thermic", "special_shieldbooster_thermic");
    let plans = [
        vec![],
        vec![t("PowerDistributor", "PowerDistributor_PriorityWeapons", "special_powerdistributor_fast")],
        vec![t("PowerDistributor", "PowerDistributor_PriorityWeapons", "special_powerdistributor_capacity")],
        vec![t("FrameShiftDrive", "FSD_FastBoot", "special_fsd_heavy")],
        vec![thermal("TinyHardpoint1")],
        vec![thermal("TinyHardpoint1"), thermal("TinyHardpoint2")],
    ];
    let cases: Vec<_> = plans
        .into_iter()
        .map(|targets| {
            let stats = astromech_ship::compare(ship, &targets).unwrap();
            let plan = Plan { ship_id: ship.id, targets, rolls: Rolls::default() };
            serde_json::json!({ "plan": plan, "stats": stats })
        })
        .collect();
    // Card stats for this fleet and the sample journal's (the app window's sample fleet).
    let mut headlines = astromech_ship::headlines(&astromech_engineering::sample_fleet());
    headlines.extend(astromech_ship::headlines(&fleet));
    let v = serde_json::json!({ "fleet": fleet, "cases": cases, "headlines": headlines });
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}
