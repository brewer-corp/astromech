//! Print the app window's sample fleet, plan, engineers and trip:
//! `cargo run -q -p astromech-engineering --example dump-fixture`
use astromech_engineering::{engineers, plan_trip, Hop};

fn main() {
    let fleet = astromech_engineering::sample_fleet();
    let plan = astromech_engineering::sample_plan();
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let engs = engineers::from_journal_dir(&dir);
    let ship = fleet.ships.iter().find(|s| s.id == 7).unwrap();
    let pins = engineers::effective_pins(&engs, &Default::default());
    let range = ship.max_jump_range.unwrap_or(20.0);
    // straight-line estimate: the sample has no star map
    let hop = |a: [f64; 3], b: [f64; 3]| {
        let ly = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
        Hop { ly, jumps: (ly / range).ceil() as u32, exact: false }
    };
    let start = ("Shinrarta Dezhra", [55.71875, 17.59375, 27.15625]);
    let trip = plan_trip(ship, &plan.targets, &engs, &pins, Some(start), range, &hop);
    let v = serde_json::json!({
        "fleet": fleet,
        "plan_input": astromech_engineering::sample_plan_input(),
        "plan": plan,
        "engineers": { "engineers": engs, "here": { "system": start.0, "id64": 3932277478106i64, "pos": start.1 } },
        "trip": trip,
    });
    print!("{}", serde_json::to_string_pretty(&v).unwrap());
}
