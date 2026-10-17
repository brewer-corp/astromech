//! Journal replay tests for fuel along the route: the next scoopable star, and the warning when
//! the fuel won't reach one.

mod common;

use common::Run;

/// A route along the x axis, 20 ly per hop: (name, star class).
fn route(hops: &[(&str, &str)]) -> String {
    let h: Vec<String> = hops
        .iter()
        .enumerate()
        .map(|(i, (name, class))| {
            format!(
                r#"{{"StarSystem":"{name}","SystemAddress":{i},"StarPos":[{}.0,0.0,0.0],"StarClass":"{class}"}}"#,
                i * 20
            )
        })
        .collect();
    format!(r#"{{"timestamp":"2026-10-16T19:59:00Z","event":"NavRoute","Route":[{}]}}"#, h.join(","))
}

/// An FSDJump of 20 ly into `system`, using `used` t and leaving `level` t.
fn jump(system: &str, used: f64, level: f64) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"FSDJump","StarSystem":"{system}","SystemAddress":1,"StarPos":[0,0,0],"JumpDist":20.0,"FuelUsed":{used},"FuelLevel":{level},"Population":0,"Factions":[]}}"#
    )
}

const LOADOUT: &str = r#"{"timestamp":"2026-10-16T19:00:00Z","event":"Loadout","Ship":"asp","FuelCapacity":{"Main":32.0,"Reserve":0.63},"Modules":[{"Slot":"FrameShiftDrive","Item":"int_hyperdrive_size5_class5","On":true,"Priority":0}]}"#;

fn run(hops: &[(&str, &str)]) -> Run {
    let mut r = Run::new();
    assert!(r.spoken(LOADOUT).is_empty());
    r.route(&route(hops));
    r
}

#[test]
fn the_next_scoopable_star_is_said_once_per_stretch_without_one() {
    let mut r = run(&[
        ("Start", "K"),
        ("Neut", "N"),
        ("Dwarf", "DA"),
        ("Brown", "L"),
        ("Giant", "K_OrangeGiant"),
        ("End", "M"),
    ]);
    assert_eq!(r.spoken(&jump("Neut", 2.0, 20.0)), ["Next scoopable star in 3 jumps."]);
    // The same stretch: already said.
    assert!(r.spoken(&jump("Dwarf", 2.0, 18.0)).is_empty());
    assert!(r.spoken(&jump("Brown", 2.0, 16.0)).is_empty());
    // At a scoopable star: the HUD's scoop prompt says it; nothing.
    assert!(r.spoken(&jump("Giant", 2.0, 14.0)).is_empty());
    // Arrived: the route is done.
    assert!(r.spoken(&jump("End", 2.0, 12.0)).is_empty());
}

#[test]
fn a_neutron_route_does_not_hear_it_every_other_jump() {
    // Neutron, scoopable, neutron, ...: every stretch is one jump; nothing to say.
    let mut r = run(&[("Start", "K"), ("N1", "N"), ("K1", "K"), ("N2", "N"), ("K2", "G"), ("N3", "N"), ("End", "M")]);
    for (sys, level) in [("N1", 30.0), ("K1", 28.0), ("N2", 26.0), ("K2", 24.0), ("N3", 22.0)] {
        assert!(r.spoken(&jump(sys, 2.0, level)).is_empty(), "{sys}");
    }
}

#[test]
fn a_boosted_jump_does_not_cheapen_the_fuel_estimate() {
    // A 20 ly jump for 2 t, then a neutron-boosted 80 ly jump that also used 2 t. Four 20 ly
    // jumps (8 t) to the G star with 5 t left must still warn; from the boosted jump they
    // would look like 0.07 t each.
    let mut r = run(&[("Start", "K"), ("Prev", "N"), ("A", "N"), ("B", "N"), ("C", "DA"), ("D", "Y"), ("Fuel", "G")]);
    r.spoken(&jump("Prev", 2.0, 25.0));
    let boosted = jump("A", 2.0, 5.0).replace("\"JumpDist\":20.0", "\"JumpDist\":80.0,\"BoostUsed\":4");
    assert_eq!(r.spoken(&boosted), ["Fuel too low to reach a scoopable star."]);
    assert_eq!(r.state.last_jump, Some((20.0, 2.0)));
}

#[test]
fn another_ship_starts_without_a_fuel_estimate() {
    let mut r = run(&[("Start", "K"), ("A", "N"), ("Fuel", "G")]);
    r.spoken(&jump("A", 2.0, 25.0));
    assert!(r.state.last_jump.is_some());
    // The same ship's loadout again (outfitting) keeps it; a swap to another ship drops it.
    let ship = |id: &str| LOADOUT.replace("\"Ship\":\"asp\"", &format!("\"Ship\":\"asp\",\"ShipID\":{id}"));
    r.spoken(&ship("1"));
    r.spoken(&ship("1"));
    assert!(r.state.last_jump.is_some());
    r.spoken(&ship("2"));
    assert_eq!(r.state.last_jump, None);
}

#[test]
fn no_scoopable_star_left_is_said_once_per_arrival() {
    let mut r = run(&[("Start", "G"), ("A", "N"), ("B", "DB"), ("End", "H")]);
    assert_eq!(r.spoken(&jump("A", 1.0, 30.0)), ["No scoopable stars left on the route."]);
}

#[test]
fn not_enough_fuel_to_reach_a_scoopable_star_warns_once() {
    // 2 t per 20 ly jump; four jumps to the G star need 8 t, and only 5 t are left.
    let mut r = run(&[("Start", "K"), ("A", "N"), ("B", "N"), ("C", "DA"), ("D", "Y"), ("Fuel", "G")]);
    let said = r.spoken(&jump("A", 2.0, 5.0));
    assert_eq!(said, ["Next scoopable star in 4 jumps.", "Fuel too low to reach a scoopable star."]);
    // The next jumps are still short: the warning isn't repeated within its cooldown.
    assert!(r.spoken(&jump("B", 2.0, 3.0)).is_empty());
    assert!(r.spoken(&jump("C", 2.0, 1.0)).is_empty());
}

#[test]
fn enough_fuel_gives_no_warning() {
    let mut r = run(&[("Start", "K"), ("A", "N"), ("B", "N"), ("C", "DA"), ("D", "Y"), ("Fuel", "G")]);
    assert_eq!(r.spoken(&jump("A", 2.0, 8.5)), ["Next scoopable star in 4 jumps."]);
}

#[test]
fn at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here() {
    let mut r = run(&[("Start", "K"), ("Scoop", "F"), ("A", "N"), ("B", "N"), ("Fuel", "M")]);
    assert_eq!(r.spoken(&jump("Scoop", 2.0, 4.0)), ["Scoop here. Next scoopable star out of reach."]);
    // Longer hops cost more than in proportion to the distance (the FSD's power law).
    let far = route(&[("Start", "K"), ("Scoop", "F"), ("A", "N"), ("Fuel", "M")])
        .replace("[40.0,", "[50.0,")
        .replace("[60.0,", "[80.0,");
    let mut r = run(&[]);
    r.route(&far);
    // 30 ly then 30 ly at 2 t per 20 ly: 2 × 2 × 1.5^2.45 ≈ 10.8 t, more than the 10 t left.
    assert_eq!(r.spoken(&jump("Scoop", 2.0, 10.0)), ["Scoop here. Next scoopable star out of reach."]);
}

#[test]
fn off_the_route_or_before_any_jump_nothing_is_said() {
    let mut r = run(&[("Start", "K"), ("A", "N"), ("Fuel", "G")]);
    assert!(r.spoken(&jump("Elsewhere", 2.0, 1.0)).is_empty());
    // No route at all.
    let mut r = Run::new();
    assert!(r.spoken(&jump("A", 2.0, 1.0)).is_empty());
}

#[test]
fn fuel_callouts_are_silent_during_the_catch_up() {
    let mut r = run(&[("Start", "K"), ("A", "N"), ("B", "N"), ("Fuel", "G")]);
    r.engine.set_live(false);
    assert!(r.spoken(&jump("A", 2.0, 1.0)).is_empty());
}
