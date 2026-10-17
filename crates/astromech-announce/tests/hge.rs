//! Journal replay tests for the High Grade Emissions callouts: the signal with its likely pool,
//! and the arrival line for a good HGE system on the shopping list.

mod common;

use astromech_announce::arrival::ShoppingList;
use common::Run;

/// A jump into a system with these factions (name, allegiance, state, influence).
fn jump(population: i64, factions: &[(&str, &str, &str, f64)]) -> String {
    let f: Vec<String> = factions
        .iter()
        .map(|(name, al, st, inf)| {
            let active = if *st == "None" { String::new() } else { format!(r#"{{"State":"{st}"}}"#) };
            format!(r#"{{"Name":"{name}","Allegiance":"{al}","FactionState":"{st}","Influence":{inf},"ActiveStates":[{active}]}}"#)
        })
        .collect();
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"FSDJump","StarSystem":"Sys","SystemAddress":1,"StarPos":[0,0,0],"SystemAllegiance":"Federation","Population":{population},"JumpDist":10.0,"Factions":[{}]}}"#,
        f.join(",")
    )
}

/// An HGE as the game writes it, spawned by `faction` (None: the game didn't say).
fn hge(faction: Option<(&str, &str)>) -> String {
    let spawn = faction
        .map(|(f, st)| format!(r#","SpawningState":"$FactionState_{st}_desc;","SpawningState_Localised":"{st}","SpawningFaction":"{f}""#))
        .unwrap_or_default();
    format!(
        r#"{{"timestamp":"2026-10-16T20:01:00Z","event":"FSSSignalDiscovered","SystemAddress":1,"SignalName":"$USS_HighGradeEmission;","SignalName_Localised":"Unidentified signal source","USSType":"$USS_Type_VeryValuableSalvage;","USSType_Localised":"High grade emissions"{spawn},"ThreatLevel":0,"TimeRemaining":1800.0}}"#
    )
}

const FAMINE_SYSTEM: &[(&str, &str, &str, f64)] = &[
    ("Hungry Feds", "Federation", "Famine", 0.5),
    ("Booming Locals", "Independent", "Boom", 0.3),
    ("Imperial Traders", "Empire", "None", 0.2),
];

#[test]
fn an_hge_in_a_famine_system_names_the_spawning_factions_pool() {
    let mut r = Run::new();
    assert!(r.spoken(&jump(80_000_000, FAMINE_SYSTEM)).is_empty());
    // Famine has no pool of its own: a Federation faction spawns Federation composites.
    assert_eq!(r.spoken(&hge(Some(("Hungry Feds", "Famine")))), ["High Grade Emissions. Federation composites."]);
}

#[test]
fn each_spawning_faction_gives_its_own_pool() {
    let settings = astromech_announce::Settings(
        [("hge".to_string(), astromech_announce::RuleSettings { cooldown_s: Some(0.0), ..Default::default() })].into(),
    );
    let mut r = Run::with(Default::default(), &settings);
    r.spoken(&jump(80_000_000, FAMINE_SYSTEM));
    assert_eq!(r.spoken(&hge(Some(("Booming Locals", "Boom")))), ["High Grade Emissions. Proto materials."]);
    assert_eq!(r.spoken(&hge(Some(("Imperial Traders", "None")))), ["High Grade Emissions. Imperial Shielding."]);
    // Who spawned it unknown: the most influential faction's pool.
    assert_eq!(r.spoken(&hge(None)), ["High Grade Emissions. Federation composites."]);
    // A material on the shopping list is named instead of the pool.
    *r.shared.shopping.write().unwrap() = ShoppingList::wanting(&["protolightalloys"]);
    assert_eq!(r.spoken(&hge(Some(("Booming Locals", "Boom")))), ["High Grade Emissions. Proto Light Alloys."]);
    // A faction with no pool (Independent, no state that gives one).
    let mut r = Run::with(Default::default(), &settings);
    r.spoken(&jump(80_000_000, &[("Quiet Locals", "Independent", "None", 1.0)]));
    assert_eq!(r.spoken(&hge(Some(("Quiet Locals", "None")))), ["High Grade Emissions."]);
}

#[test]
fn other_signals_stay_quiet() {
    let mut r = Run::new();
    r.spoken(&jump(80_000_000, FAMINE_SYSTEM));
    for sig in [
        r#"{"timestamp":"2026-10-16T20:01:00Z","event":"FSSSignalDiscovered","SystemAddress":1,"SignalName":"$USS_Salvage;","USSType":"$USS_Type_Salvage;","USSType_Localised":"Degraded emissions","SpawningFaction":"Hungry Feds"}"#,
        r#"{"timestamp":"2026-10-16T20:01:00Z","event":"FSSSignalDiscovered","SystemAddress":1,"SignalName":"$USS_EncodedEmission;","USSType":"$USS_Type_ValuableSalvage;","USSType_Localised":"Encoded emissions"}"#,
        r#"{"timestamp":"2026-10-16T20:01:00Z","event":"FSSSignalDiscovered","SystemAddress":1,"SignalName":"Jameson Memorial","IsStation":true}"#,
    ] {
        assert!(r.spoken(sig).is_empty(), "{sig}");
    }
}

#[test]
fn a_burst_of_hges_from_one_honk_is_said_once() {
    let mut r = Run::new();
    r.spoken(&jump(80_000_000, FAMINE_SYSTEM));
    let one = hge(Some(("Hungry Feds", "Famine")));
    assert_eq!(r.spoken(&[one.clone(), one.clone(), one].join("\n")).len(), 1);
}

#[test]
fn a_billion_people_system_for_a_wanted_pool_material_is_a_good_hge_system_on_arrival() {
    let mut r = Run::new();
    *r.shared.shopping.write().unwrap() = ShoppingList::wanting(&["imperialshielding"]);
    let empire = [("Imperial Traders", "Empire", "None", 0.6), ("Locals", "Independent", "None", 0.4)];
    assert_eq!(r.spoken(&jump(2_500_000_000, &empire)), ["Good for Imperial Shielding."]);
    // Under a billion people the allegiance pools rarely spawn: nothing.
    assert!(r.spoken(&jump(800_000_000, &empire)).is_empty());
    // Not on the list: nothing.
    *r.shared.shopping.write().unwrap() = ShoppingList::wanting(&["fedcorecomposites"]);
    assert!(r.spoken(&jump(2_500_000_000, &empire)).is_empty());
}

#[test]
fn hge_callouts_are_silent_during_the_catch_up() {
    let mut r = Run::new();
    r.engine.set_live(false);
    assert!(r.spoken(&[jump(80_000_000, FAMINE_SYSTEM), hge(None)].join("\n")).is_empty());
}
