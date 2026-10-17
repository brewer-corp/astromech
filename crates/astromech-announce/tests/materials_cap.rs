//! Journal replay tests for the material cap alert and the trader suggestion.

mod common;

use common::Run;

fn materials(arsenic: i64) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"Materials","Raw":[{{"Name":"arsenic","Count":{arsenic}}},{{"Name":"iron","Count":10}}],"Manufactured":[],"Encoded":[]}}"#
    )
}

fn collected(name: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:01:00Z","event":"MaterialCollected","Category":"Raw","Name":"{name}","Count":1}}"#
    )
}

fn docked(services: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:02:00Z","event":"Docked","StationName":"Test Orbital","StationType":"Orbis","StarSystem":"Sys","SystemAddress":1,"MarketID":2,"StationServices":[{services}]}}"#
    )
}

#[test]
fn filling_a_material_says_so_once() {
    let mut r = Run::new();
    // Arsenic is grade 2: 250 is full.
    r.engine.set_live(false);
    assert!(r.spoken(&materials(248)).is_empty());
    r.engine.set_live(true);
    assert!(r.spoken(&collected("arsenic")).is_empty()); // 249
    assert_eq!(r.spoken(&collected("arsenic")), ["Arsenic at 250/250."]);
    // Still full: the game would not take more, and a repeat event says nothing.
    assert!(r.spoken(&collected("arsenic")).is_empty());
    assert!(r.spoken(&collected("iron")).is_empty());
}

#[test]
fn a_material_full_since_the_catch_up_says_nothing_on_a_live_collect() {
    let mut r = Run::new();
    r.engine.set_live(false);
    assert!(r.spoken(&materials(249)).is_empty());
    assert!(r.spoken(&collected("arsenic")).is_empty()); // 250, in the catch-up
    r.engine.set_live(true);
    assert!(r.spoken(&collected("arsenic")).is_empty(), "already full before this collect");
}

#[test]
fn trading_below_the_cap_and_collecting_back_to_it_says_so_again() {
    let mut r = Run::new();
    r.engine.set_live(false);
    assert!(r.spoken(&materials(250)).is_empty());
    r.engine.set_live(true);
    let trade = r#"{"timestamp":"2026-10-16T20:01:30Z","event":"MaterialTrade","MarketID":2,"TraderType":"raw","Paid":{"Material":"arsenic","Category":"Raw","Quantity":6},"Received":{"Material":"iron","Category":"Raw","Quantity":1}}"#;
    assert!(r.spoken(trade).is_empty()); // 244
    let spoken: Vec<String> = (0..6).flat_map(|_| r.spoken(&collected("arsenic"))).collect();
    assert_eq!(spoken, ["Arsenic at 250/250."]);
}

#[test]
fn catch_up_is_silent_even_when_full() {
    let mut r = Run::new();
    r.engine.set_live(false);
    assert!(r.spoken(&[materials(249), collected("arsenic")].join("\n")).is_empty());
}

#[test]
fn docking_at_a_trader_names_what_to_trade_down() {
    let mut r = Run::new();
    r.engine.set_live(false);
    r.spoken(&materials(250));
    r.engine.set_live(true);
    assert_eq!(r.spoken(&docked(r#""dock","materialtrader""#)), ["Trader here. Trade down Arsenic."]);
    // No trader, or nothing full: nothing.
    assert!(r.spoken(&docked(r#""dock","refuel""#)).is_empty());
    let mut r = Run::new();
    r.engine.set_live(false);
    r.spoken(&materials(100));
    r.engine.set_live(true);
    assert!(r.spoken(&docked(r#""dock","materialtrader""#)).is_empty());
}

#[test]
fn the_trader_line_speaks_after_an_approach() {
    let mut r = Run::new();
    r.engine.set_live(false);
    r.spoken(&materials(250));
    r.engine.set_live(true);
    r.spoken(r#"{"timestamp":"2026-10-16T20:01:30Z","event":"DockingGranted","LandingPad":3,"MarketID":2,"StationName":"Test Orbital","StationType":"Orbis"}"#);
    assert_eq!(r.spoken(&docked(r#""materialtrader""#)).len(), 1);
}
