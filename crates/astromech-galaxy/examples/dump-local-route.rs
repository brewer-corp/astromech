//! Print the browser preview's local neutron route (`local` in app/src/preview/fixtures/plot.json):
//! the local plotter's route from Sol to HR 3230 (its position from data/catalog/raw-farms.json)
//! over the neutron star sample the tests use (tests/fixtures/neutron-sample.bin), for the
//! preview's ship (plot.json's `ship`: 39.87 ly, the Asp Explorer's drive in tests/plot_route.rs).
//! The whole neutron star file isn't in the repository, so this short route is the one recorded.
//! `cargo run -q -p astromech-galaxy --example dump-local-route`
use astromech_galaxy::neutron::NeutronStars;
use astromech_galaxy::plot::Ship;
use astromech_galaxy::Here;

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = NeutronStars::open(&root.join("tests/fixtures/neutron-sample.bin")).unwrap();
    let farms: serde_json::Value = serde_json::from_str(include_str!("../../../data/catalog/raw-farms.json")).unwrap();
    let to = farms["farms"].as_array().unwrap().iter().find(|f| f["system"] == "HR 3230").unwrap();
    let pos: [f64; 3] = serde_json::from_value(to["pos"].clone()).unwrap();
    let to = Here { system: "HR 3230".into(), id64: to["id64"].as_i64(), pos };
    let sol = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };
    let ship = Ship {
        name: "Asp Explorer".into(),
        fsd: astromech_ship::Fsd {
            optimal_mass: 1470.0,
            max_fuel_per_jump: 8.0,
            fuel_multiplier: 0.012,
            fuel_power: 2.45,
            base_mass: 300.0,
            tank_size: 32.0,
            reserve_size: 0.63,
            range_boost: 0.0,
            supercharge_multiplier: 4,
        },
        cargo: 0.0,
        range: 39.87,
        unresolved: vec![],
    };
    let p = astromech_galaxy::neutron_plot::plot(&file, &sol, &to, &ship).unwrap();
    println!("{}", serde_json::to_string_pretty(&p).unwrap());
}
