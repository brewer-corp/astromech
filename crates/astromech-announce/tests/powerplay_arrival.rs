//! Journal replay tests for the Powerplay arrival callout (`powerplay_arrival`, off by default).

mod common;

use astromech_announce::arrival::ShoppingList;
use astromech_announce::*;
use common::Run;

const PLEDGE: &str = r#"{"timestamp":"2026-10-16T19:00:00Z","event":"Powerplay","Power":"Aisling Duval","Rank":10,"Merits":5000,"TimePledged":1000}"#;

fn jump(event: &str, extra: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"{event}","StarSystem":"Sys","SystemAddress":1,"StarPos":[1,2,3],"SystemAllegiance":"Independent","Population":3000000,"JumpDist":12.5,"FuelUsed":2.1,"FuelLevel":20.0,"Factions":[{{"Name":"Boom Party","Allegiance":"Independent","FactionState":"Boom","Influence":0.5,"ActiveStates":[{{"State":"Boom"}}]}}]{extra}}}"#
    )
}

const ENEMY: &str = r#","ControllingPower":"Zachary Hudson","Powers":["Zachary Hudson"],"PowerplayState":"Fortified","PowerplayStateControlProgress":0.4,"PowerplayStateReinforcement":10,"PowerplayStateUndermining":20"#;

fn on() -> Settings {
    toml::from_str("[powerplay_arrival]\nenabled = true\n").unwrap()
}

fn run(settings: &Settings) -> Run {
    let mut r = Run::with(Default::default(), settings);
    r.spoken(PLEDGE);
    r
}

#[test]
fn speaks_the_systems_powerplay_state_when_on() {
    for event in ["FSDJump", "CarrierJump"] {
        let mut r = run(&on());
        let said = r.spoken(&jump(event, ENEMY));
        assert_eq!(
            said,
            ["Zachary Hudson fortified, 40 percent.", "Reinforcement 10, undermining 20.", "Undermining target."],
            "{event}"
        );
    }
}

#[test]
fn is_off_by_default() {
    let mut r = run(&Settings::default());
    assert!(r.spoken(&jump("FSDJump", ENEMY)).iter().all(|l| l == "Undermining target."));
    // Explicitly off, too.
    let off: Settings = toml::from_str("[powerplay_arrival]\nenabled = false\n").unwrap();
    assert!(run(&off).spoken(&jump("FSDJump", ENEMY)).iter().all(|l| l == "Undermining target."));
    // Not even the state is spoken.
    assert_eq!(run(&Settings::default()).spoken(&jump("FSDJump", ENEMY)), ["Undermining target."]);
}

#[test]
fn a_system_without_powerplay_activity_is_silent() {
    assert!(run(&on()).spoken(&jump("FSDJump", "")).is_empty());
}

#[test]
fn a_login_location_is_not_an_arrival() {
    assert!(run(&on()).spoken(&jump("Location", ENEMY)).is_empty());
}

#[test]
fn the_arrival_rule_does_not_repeat_the_target_when_on() {
    let boom = ShoppingList::wanting(&["protoheatradiators"]);
    let mut r = run(&on());
    *r.shared.shopping.write().unwrap() = boom.clone();
    let said = r.spoken(&jump("FSDJump", ENEMY));
    assert_eq!(said[0], "Boom here. Proto Heat Radiators.");
    assert_eq!(said.iter().filter(|l| l.contains("target")).count(), 1, "{said:?}");
    // Off: the arrival rule keeps saying it.
    let mut r = run(&Settings::default());
    *r.shared.shopping.write().unwrap() = boom;
    assert_eq!(r.spoken(&jump("FSDJump", ENEMY)), ["Boom here. Proto Heat Radiators. Undermining target."]);
}

#[test]
fn is_silent_during_the_catch_up() {
    let mut r = run(&on());
    r.engine.set_live(false);
    assert!(r.spoken(&jump("FSDJump", ENEMY)).is_empty());
    r.engine.set_live(true);
    assert!(!r.spoken(&jump("FSDJump", ENEMY)).is_empty());
}

#[test]
fn a_long_list_is_said_a_clause_at_a_time() {
    use astromech_announce::powerplay_arrival::sentences;
    assert_eq!(
        sentences("Unoccupied. Acquisition: A B 30 percent, C D 30 percent, E F 30 percent. Acquisition target."),
        ["Unoccupied.", "Acquisition: A B 30 percent.", "C D 30 percent.", "E F 30 percent.", "Acquisition target."]
    );
}
