//! Journal replay tests for the colonisation market line: on docking, how many of the projects'
//! outstanding commodities this market has in stock, once per dock.

mod common;

use common::Run;

const DOCKED: &str = r#"{"timestamp":"2026-10-16T20:00:00Z","event":"Docked","StationName":"Jameson Memorial","StationType":"Coriolis","MarketID":128000000,"StarSystem":"Shinrarta Dezhra","SystemAddress":1}"#;
const UNDOCKED: &str =
    r#"{"timestamp":"2026-10-16T20:30:00Z","event":"Undocked","StationName":"Jameson Memorial","MarketID":128000000}"#;
/// Steel and titanium still to deliver; aluminium delivered in full; copper not asked for.
const DEPOT: &str = r#"{"timestamp":"2026-10-16T19:00:00Z","event":"ColonisationConstructionDepot","MarketID":3900000001,"ConstructionProgress":0.4,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$steel_name;","Name_Localised":"Steel","RequiredAmount":100,"ProvidedAmount":10,"Payment":1},{"Name":"$titanium_name;","Name_Localised":"Titanium","RequiredAmount":50,"ProvidedAmount":0,"Payment":1},{"Name":"$aluminium_name;","Name_Localised":"Aluminium","RequiredAmount":20,"ProvidedAmount":20,"Payment":1}]}"#;
const EVENT_TS: &str = "2026-10-16T20:00:05Z";

fn item(name: &str, stock: i64) -> String {
    format!(
        r#"{{"id":1,"Name":"${name}_name;","BuyPrice":10,"SellPrice":9,"MeanPrice":10,"StockBracket":1,"DemandBracket":0,"Stock":{stock},"Demand":0,"Consumer":false,"Producer":true,"Rare":false}}"#
    )
}

fn market(timestamp: &str, market_id: i64, items: &[(&str, i64)]) -> String {
    let items: Vec<String> = items.iter().map(|(n, s)| item(n, *s)).collect();
    format!(
        r#"{{"timestamp":"{timestamp}","event":"Market","MarketID":{market_id},"StationName":"Jameson Memorial","StationType":"Coriolis","StarSystem":"Shinrarta Dezhra","Items":[{}]}}"#,
        items.join(",")
    )
}

fn docked_with_project() -> Run {
    let mut r = Run::new();
    assert!(r.spoken(DEPOT).is_empty());
    assert!(r.spoken(DOCKED).is_empty());
    r
}

#[test]
fn docking_with_a_matching_market_says_how_many_project_commodities_it_sells() {
    let mut r = docked_with_project();
    // Steel and titanium are needed and stocked; aluminium is complete, copper not asked for,
    // and gold is needed by no one; titanium out of stock would not count.
    let m = market(EVENT_TS, 128000000, &[("steel", 500), ("titanium", 3), ("aluminium", 90), ("copper", 9)]);
    assert_eq!(r.market(&m, EVENT_TS), ["Sells 2 project commodities."]);
}

#[test]
fn one_commodity_is_singular_and_none_in_stock_says_nothing() {
    let mut r = docked_with_project();
    let m = market(EVENT_TS, 128000000, &[("steel", 500), ("titanium", 0)]);
    assert_eq!(r.market(&m, EVENT_TS), ["Sells 1 project commodity."]);
    r.spoken(UNDOCKED);
    r.spoken(DOCKED);
    let none = market(EVENT_TS, 128000000, &[("titanium", 0), ("copper", 9)]);
    assert!(r.market(&none, EVENT_TS).is_empty());
}

#[test]
fn it_speaks_once_per_dock() {
    let mut r = docked_with_project();
    let m = market(EVENT_TS, 128000000, &[("steel", 500)]);
    assert_eq!(r.market(&m, EVENT_TS).len(), 1);
    // The commander opens the market again.
    assert!(r.market(&m, EVENT_TS).is_empty());
    // A new dock gets its own line.
    r.spoken(UNDOCKED);
    r.spoken(DOCKED);
    assert_eq!(r.market(&m, EVENT_TS).len(), 1);
}

#[test]
fn a_market_from_another_station_or_an_earlier_visit_is_not_used() {
    let mut r = docked_with_project();
    // Another station's file (not the market docked at).
    let other = market(EVENT_TS, 999, &[("steel", 500)]);
    assert!(r.market(&other, EVENT_TS).is_empty());
    // A file older than the Market event that announced it: the earlier visit's.
    let old = market("2026-10-15T20:00:05Z", 128000000, &[("steel", 500)]);
    assert!(r.market(&old, EVENT_TS).is_empty());
    // The right one still speaks: the failed attempts did not use up the dock.
    let m = market(EVENT_TS, 128000000, &[("steel", 500)]);
    assert_eq!(r.market(&m, EVENT_TS).len(), 1);
}

#[test]
fn no_active_project_or_a_finished_one_says_nothing() {
    let m = market(EVENT_TS, 128000000, &[("steel", 500)]);
    let mut r = Run::new();
    r.spoken(DOCKED);
    assert!(r.market(&m, EVENT_TS).is_empty());

    let mut r = docked_with_project();
    r.spoken(&DEPOT.replace(r#""ConstructionComplete":false"#, r#""ConstructionComplete":true"#));
    assert!(r.market(&m, EVENT_TS).is_empty());
}

#[test]
fn catch_up_is_silent_and_does_not_use_up_the_dock() {
    let mut r = docked_with_project();
    let m = market(EVENT_TS, 128000000, &[("steel", 500)]);
    r.engine.set_live(false);
    assert!(r.market(&m, EVENT_TS).is_empty());
    r.engine.set_live(true);
    assert_eq!(r.market(&m, EVENT_TS).len(), 1);
}
