//! Journal replay tests for the arrival briefing.

mod common;

use astromech_announce::arrival::ShoppingList;
use astromech_announce::*;
use common::Run;

/// A jump into `system` with the given factions (allegiance, state, influence) and extra keys.
fn jump(system: &str, population: i64, factions: &[(&str, &str, f64)], extra: &str) -> String {
    let f: Vec<String> = factions
        .iter()
        .map(|(al, st, inf)| {
            let active = if *st == "None" { String::new() } else { format!(r#"{{"State":"{st}"}}"#) };
            format!(
                r#"{{"Name":"{al} {st} Party","Allegiance":"{al}","FactionState":"{st}","Influence":{inf},"ActiveStates":[{active}]}}"#
            )
        })
        .collect();
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"FSDJump","StarSystem":"{system}","SystemAddress":1,"StarPos":[1,2,3],"SystemAllegiance":"Independent","SystemEconomy_Localised":"Industrial","SystemSecurity":"$SYSTEM_SECURITY_low;","SystemSecurity_Localised":"Low Security","Population":{population},"JumpDist":12.5,"FuelUsed":2.1,"FuelLevel":20.0,"Factions":[{}]{extra}}}"#,
        f.join(",")
    )
}

/// The arrival star's automatic scan.
fn autoscan(was_discovered: bool) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:01Z","event":"Scan","ScanType":"AutoScan","BodyName":"Sys A","BodyID":1,"StarSystem":"Sys","SystemAddress":1,"DistanceFromArrivalLS":0.0,"StarType":"K","Subclass":3,"Luminosity":"Va","WasDiscovered":{was_discovered},"WasMapped":false}}"#
    )
}

const START_JUMP: &str = r#"{"timestamp":"2026-10-16T19:59:50Z","event":"StartJump","JumpType":"Hyperspace","StarSystem":"Sys","SystemAddress":1,"StarClass":"K"}"#;

fn shopping(ids: &[&str]) -> Run {
    let r = Run::new();
    *r.shared.shopping.write().unwrap() = ShoppingList::wanting(ids);
    r
}

#[test]
fn a_system_whose_state_the_shopping_list_wants_gets_its_state_line() {
    let mut r = shopping(&["fedcorecomposites", "protoheatradiators"]);
    let boom = jump("Sys", 3_000_000, &[("Independent", "Boom", 0.4), ("Federation", "None", 0.6)], "");
    assert_eq!(r.spoken(&boom), ["Boom here. Proto Heat Radiators."]);
    // Famine counts for wake scans.
    let mut r = shopping(&["dataminedwake"]);
    assert_eq!(
        r.spoken(&jump("Sys", 0, &[("Empire", "Famine", 0.3)], "")),
        ["Famine here. Datamined Wake Exceptions."]
    );
    // War and civil war name themselves; the most influential faction's state goes first.
    let mut r = shopping(&["militarysupercapacitors", "improvisedcomponents"]);
    let two = jump("Sys", 5_000_000, &[("Alliance", "CivilUnrest", 0.2), ("Federation", "CivilWar", 0.5)], "");
    assert_eq!(r.spoken(&two), ["Civil war here. Military Supercapacitors."]);
    let one = jump("Sys", 5_000_000, &[("Alliance", "CivilUnrest", 0.2)], "");
    assert_eq!(r.spoken(&one), ["Civil unrest here. Improvised Components."]);
}

#[test]
fn a_state_the_shopping_list_does_not_want_gets_nothing() {
    // Boom, but nothing on the list comes from it.
    let mut r = shopping(&["militarygradealloys"]);
    assert!(r.spoken(&jump("Sys", 3_000_000, &[("Independent", "Boom", 0.9)], "")).is_empty());
    // An empty list: nothing, whatever the states.
    let mut r = Run::new();
    assert!(r.spoken(&jump("Sys", 3_000_000, &[("Independent", "Boom", 0.5), ("Empire", "War", 0.5)], "")).is_empty());
    // A Federation faction in Boom spawns Federation composites, not the Proto pool.
    let mut r = shopping(&["protoheatradiators"]);
    assert!(r.spoken(&jump("Sys", 3_000_000, &[("Federation", "Boom", 0.9)], "")).is_empty());
}

#[test]
fn a_visited_fully_known_system_with_nothing_notable_gets_nothing() {
    let mut r = shopping(&["protoheatradiators", "dataminedwake"]);
    let quiet = [
        START_JUMP.to_string(),
        jump("Sys", 85_000_000, &[("Federation", "None", 0.7), ("Empire", "Election", 0.3)], ""),
        autoscan(true),
    ];
    assert!(r.spoken(&quiet.join("\n")).is_empty());
}

#[test]
fn an_undiscovered_system_is_said_after_the_jump() {
    let mut r = Run::new();
    let lines = [START_JUMP.to_string(), jump("Sys", 0, &[], ""), autoscan(false)];
    assert_eq!(r.spoken(&lines.join("\n")), ["Undiscovered system."]);
    // Only the arrival star's automatic scan: a body scanned later in the FSS is not news.
    let later = autoscan(false).replace("\"AutoScan\"", "\"Detailed\"");
    assert!(r.spoken(&later).is_empty());
    let companion = autoscan(false).replace("\"DistanceFromArrivalLS\":0.0", "\"DistanceFromArrivalLS\":1520.4");
    assert!(r.spoken(&companion).is_empty());
}

#[test]
fn the_pledged_power_hears_undermining_and_acquisition_targets() {
    let mut r = Run::new();
    r.spoken(r#"{"timestamp":"2026-10-16T19:00:00Z","event":"Powerplay","Power":"Aisling Duval","Rank":10,"Merits":5000,"TimePledged":1000}"#);
    let enemy = jump(
        "Sys",
        1000,
        &[],
        r#","ControllingPower":"Zachary Hudson","Powers":["Zachary Hudson"],"PowerplayState":"Fortified","PowerplayStateControlProgress":0.4"#,
    );
    assert_eq!(r.spoken(&enemy), ["Undermining target."]);
    let unoccupied = jump(
        "Sys",
        1000,
        &[],
        r#","Powers":["Aisling Duval","Zachary Hudson"],"PowerplayState":"Unoccupied","PowerplayConflictProgress":[{"Power":"Aisling Duval","ConflictProgress":0.3}]"#,
    );
    assert_eq!(r.spoken(&unoccupied), ["Acquisition target."]);
    // Her own system: the normal case, quiet.
    let own = jump(
        "Sys",
        1000,
        &[],
        r#","ControllingPower":"Aisling Duval","Powers":["Aisling Duval"],"PowerplayState":"Stronghold""#,
    );
    assert!(r.spoken(&own).is_empty());
    // With a state line, the Powerplay line follows it.
    *r.shared.shopping.write().unwrap() = ShoppingList::wanting(&["protoheatradiators"]);
    let both = jump(
        "Sys",
        3_000_000,
        &[("Independent", "Boom", 0.5)],
        r#","ControllingPower":"Zachary Hudson","Powers":["Zachary Hudson"],"PowerplayState":"Exploited""#,
    );
    assert_eq!(r.spoken(&both), ["Boom here. Proto Heat Radiators. Undermining target."]);
    // Not pledged: no Powerplay line.
    let mut r = Run::new();
    assert!(r.spoken(&enemy).is_empty());
}

#[test]
fn star_class_security_and_allegiance_are_never_spoken() {
    let mut r = shopping(&["protoheatradiators", "militarygradealloys", "dataminedwake"]);
    r.spoken(r#"{"timestamp":"2026-10-16T19:00:00Z","event":"Powerplay","Power":"Aisling Duval","Rank":10}"#);
    let lines = [
        START_JUMP.to_string(),
        jump(
            "Sys",
            3_000_000,
            &[("Independent", "Boom", 0.5), ("Empire", "War", 0.3), ("Alliance", "Famine", 0.2)],
            r#","ControllingPower":"Zachary Hudson","Powers":["Zachary Hudson"],"PowerplayState":"Exploited""#,
        ),
        autoscan(false),
    ];
    let said = r.spoken(&lines.join("\n"));
    assert_eq!(said.len(), 2, "{said:?}");
    for l in &said {
        let low = l.to_lowercase();
        for banned in ["security", "class", "independent", "empire", "alliance", "federation", "industrial", "sys"] {
            assert!(!low.split(|c: char| !c.is_alphanumeric()).any(|w| w == banned), "{banned:?} in {l:?}");
        }
        assert!(!l.contains(" K ") && !l.starts_with("K "), "{l:?}");
    }
}

#[test]
fn the_briefing_is_silent_during_the_catch_up() {
    let shared = Shared::default();
    *shared.shopping.write().unwrap() = ShoppingList::wanting(&["protoheatradiators"]);
    let mut r = Run::with(shared, &Settings::default());
    r.engine.set_live(false);
    let lines = [jump("Sys", 3_000_000, &[("Independent", "Boom", 0.5)], ""), autoscan(false)];
    assert!(r.spoken(&lines.join("\n")).is_empty());
    r.engine.set_live(true);
    assert_eq!(r.spoken(&lines.join("\n")), ["Boom here. Proto Heat Radiators.", "Undiscovered system."]);
}

#[test]
fn a_material_collected_up_to_its_count_drops_off_the_list() {
    use astromech_announce::arrival::Want;
    let mut r = Run::new();
    *r.shared.shopping.write().unwrap() = ShoppingList(vec![Want { id: "protoheatradiators".into(), until: 5 }]);
    let held = |n: i64| {
        format!(
            r#"{{"timestamp":"2026-10-16T19:00:00Z","event":"Materials","Raw":[],"Manufactured":[{{"Name":"protoheatradiators","Count":{n}}}],"Encoded":[]}}"#
        )
    };
    let boom = jump("Sys", 3_000_000, &[("Independent", "Boom", 0.5)], "");
    r.spoken(&held(4));
    assert_eq!(r.spoken(&boom), ["Boom here. Proto Heat Radiators."]);
    // Collected in game, with the app closed: the list needs no update.
    r.spoken(&held(5));
    assert!(r.spoken(&boom).is_empty());
}
