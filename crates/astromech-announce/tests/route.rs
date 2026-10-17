//! Replay tests for the route briefing when a route is plotted (`NavRoute.json`).

mod common;

use common::Run;

/// A route along the x axis, `step` ly per hop: star classes, the first being where it starts.
fn route(step: f64, classes: &[&str]) -> String {
    let h: Vec<String> = classes
        .iter()
        .enumerate()
        .map(|(i, c)| {
            format!(
                r#"{{"StarSystem":"Sys {i}","SystemAddress":{i},"StarPos":[{:.1},0.0,0.0],"StarClass":"{c}"}}"#,
                i as f64 * step
            )
        })
        .collect();
    format!(r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"NavRoute","Route":[{}]}}"#, h.join(","))
}

/// In "Sys 0" with a 32 t tank holding `fuel` t, after a 20 ly jump that used 2 t.
fn at_start(fuel: f64) -> Run {
    let mut r = Run::new();
    r.spoken(r#"{"timestamp":"2026-10-16T19:00:00Z","event":"Loadout","Ship":"asp","FuelCapacity":{"Main":32.0,"Reserve":0.63},"Modules":[{"Slot":"FrameShiftDrive","Item":"int_hyperdrive_size5_class5","On":true,"Priority":0}]}"#);
    r.spoken(&format!(
        r#"{{"timestamp":"2026-10-16T19:59:00Z","event":"FSDJump","StarSystem":"Sys 0","StarPos":[0,0,0],"JumpDist":20.0,"FuelUsed":2.0,"FuelLevel":{fuel},"Population":0,"Factions":[]}}"#
    ));
    r
}

#[test]
fn a_white_dwarf_and_a_fuel_gap_give_one_hazard_line_each() {
    let mut r = at_start(30.0);
    // 19 jumps of 20 ly; 18 jumps from the G star to the M star with nothing to scoop (36 t,
    // the tank holds 32 t), a white dwarf at jump 4 and neutron stars.
    let mut classes = vec!["K", "G"];
    classes.extend(["N", "N", "DA", "L", "T", "Y", "N", "N", "TTS", "H", "N", "N", "N", "N", "N", "N", "N"]);
    classes.push("M");
    let said = r.plot(&route(20.0, &classes));
    assert_eq!(
        said,
        [
            "Fuel gap: 18 jumps between scoopable stars.",
            "Route: 19 jumps, 380 ly.",
            "White dwarf at jump 4.",
            "11 neutron stars, first at jump 2.",
        ]
    );
}

#[test]
fn the_fuel_warning_is_not_said_again_on_the_next_arrival() {
    // Plotted with too little fuel, then the first jump of the route seconds later.
    let mut r = at_start(3.0);
    let said = r.plot(&route(20.0, &["N", "N", "L", "Y", "G"]));
    assert_eq!(said[0], "Fuel too low to reach a scoopable star.");
    let arrive = r#"{"timestamp":"2026-10-16T20:00:10Z","event":"FSDJump","StarSystem":"Sys 1","StarPos":[20,0,0],"JumpDist":20.0,"FuelUsed":2.0,"FuelLevel":1.0,"Population":0,"Factions":[]}"#;
    assert!(!r.spoken(arrive).iter().any(|l| l.starts_with("Fuel too low")));
}

#[test]
fn a_plain_route_gets_only_its_summary() {
    let mut r = at_start(30.0);
    assert_eq!(r.plot(&route(12.5, &["K", "G", "M", "F", "K"])), ["Route: 4 jumps, 50 ly."]);
}

#[test]
fn too_little_fuel_for_the_first_scoopable_star_is_a_fuel_gap_too() {
    // At a neutron star with 3 t left, four jumps (8 t) to the first scoopable star.
    let mut r = at_start(3.0);
    let said = r.plot(&route(20.0, &["N", "N", "L", "Y", "G"]));
    assert_eq!(said[0], "Fuel too low to reach a scoopable star.");
    // At a scoopable star the tank can be filled first: 8 t is well within 32 t.
    let said = r.plot(&route(20.0, &["K", "N", "L", "Y", "G"]));
    assert_eq!(said[0], "Route: 4 jumps, 80 ly.");
}

#[test]
fn the_same_route_written_again_is_not_said_again_and_a_cleared_route_is_quiet() {
    let mut r = at_start(30.0);
    let plain = route(12.5, &["K", "G", "M"]);
    assert_eq!(r.plot(&plain), ["Route: 2 jumps, 25 ly."]);
    assert!(r.plot(&plain).is_empty());
    assert!(r.plot(r#"{"timestamp":"2026-10-16T20:01:00Z","event":"NavRouteClear","Route":[]}"#).is_empty());
    // Plotting it again after a clear is a new route.
    assert_eq!(r.plot(&plain), ["Route: 2 jumps, 25 ly."]);
}

#[test]
fn without_a_jump_to_go_by_there_is_no_fuel_line() {
    let mut r = Run::new();
    let mut classes = vec!["K"];
    classes.extend(["N"; 20]);
    classes.push("G");
    assert_eq!(r.plot(&route(20.0, &classes)), ["Route: 21 jumps, 420 ly.", "20 neutron stars, first at jump 1."]);
}

#[test]
fn the_route_briefing_is_silent_during_the_catch_up() {
    let mut r = at_start(30.0);
    r.engine.set_live(false);
    let plain = route(12.5, &["K", "DA", "M"]);
    assert!(r.plot(&plain).is_empty());
    // The file read at startup is not news once live.
    r.engine.set_live(true);
    assert!(r.plot(&plain).is_empty());
}
