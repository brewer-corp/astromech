//! This market against nearby prices, on a synthetic `Market.json` and `Cargo.json` docked at
//! "Test Gateway" in LHS 2435 and Ardent's nearby imports around LHS 2435 asked with
//! maxDistance=20 and maxDaysAgo=30 on 16 Oct 2026, trimmed to ten markets each (fleet carrier
//! names and ids replaced). No test here touches the network.

use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::Filters;
use astromech_market::here::{compare_here, parse_market, plan, symbol, Comparison, Line};
use astromech_market::Origin;
use std::sync::{Arc, Mutex};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("fixtures/", $name))
    };
}

#[derive(Default)]
struct Recorded {
    log: Mutex<Vec<String>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push(url.to_string());
        assert!(url.contains("/system/name/LHS%202435/"), "asked near the docked system: {url}");
        let body = match url.rsplit("/commodity/name/").next().unwrap() {
            "gold/nearby/imports" => fixture!("ardent-sell-gold-lhs2435.json"),
            "silver/nearby/imports" => fixture!("ardent-sell-silver-lhs2435.json"),
            "palladium/nearby/imports" => fixture!("ardent-sell-palladium-lhs2435.json"),
            "tritium/nearby/imports" => fixture!("ardent-sell-tritium-lhs2435.json"),
            "hydrogenfuel/nearby/imports" => fixture!("ardent-sell-hydrogenfuel-lhs2435.json"),
            other => panic!("no recorded answer for {other}"),
        };
        Ok(Response { status: 200, body: body.to_string() })
    }
}

fn client() -> (Client, Arc<Recorded>) {
    let rec = Arc::new(Recorded::default());
    (Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()), rec)
}

/// The evening the answers were recorded.
fn now() -> i64 {
    chrono::DateTime::parse_from_rfc3339("2026-10-16T21:00:00Z").unwrap().timestamp()
}

fn lhs2435() -> Origin {
    Origin { system: "LHS 2435".into(), pos: Some([58.3125, 21.71875, 19.53125]) }
}

fn hold() -> Vec<astromech_journal::CargoItem> {
    astromech_journal::parse_cargo(fixture!("cargo-here.json").as_bytes()).unwrap().inventory
}

fn market() -> astromech_journal::companion::MarketSnap {
    parse_market(fixture!("market-here.json").as_bytes()).unwrap()
}

/// (commodity, best station, best price, gain).
fn summary(lines: &[Line]) -> Vec<(&str, Option<&str>, Option<i64>, Option<i64>)> {
    lines
        .iter()
        .map(|l| {
            (
                l.commodity.as_str(),
                l.best.as_ref().map(|b| b.station.as_str()),
                l.best.as_ref().map(|b| b.price),
                l.gain,
            )
        })
        .collect()
}

fn run(filters: &Filters, inventory: &[astromech_journal::CargoItem]) -> (Comparison, usize) {
    let (c, rec) = client();
    let r = compare_here(&c, &lhs2435(), &market(), inventory, filters, 20.0, now()).unwrap();
    let asked = rec.log.lock().unwrap().len();
    (r, asked)
}

// --- Done when: a fixture Market.json plus recorded nearby prices gives the expected differences ---

#[test]
fn the_hold_and_the_goods_for_sale_against_the_best_nearby_prices() {
    let (r, asked) = run(&Filters::default(), &hold());
    assert_eq!((r.station.as_str(), r.system.as_str(), r.market_id), ("Test Gateway", "LHS 2435", Some(3229999001)));
    assert_eq!(asked, 5, "one request per commodity: gold, palladium, silver, tritium, hydrogen fuel");

    // Carried: gold sells here for 52,600 and at Arc City (15 ly) for 56,521: +3,921/t.
    // Palladium isn't bought here at all, so there is a best price but no difference.
    assert_eq!(
        summary(&r.carried),
        vec![
            ("gold", Some("Arc City"), Some(56_521), Some(3_921)),
            ("palladium", Some("Arc City"), Some(59_563), None),
        ]
    );
    let gold = &r.carried[0];
    assert_eq!((gold.carried, gold.sell_here, gold.buy_here), (20, Some(52_600), None));
    assert_eq!(gold.best.as_ref().unwrap().system, "Crucis Sector AL-X b1-5");
    assert!((gold.best.as_ref().unwrap().hop.ly - 15.41).abs() < 0.01, "measured from the docked system");
    assert_eq!(r.carried[1].sell_here, None);

    // For sale here, the biggest gain first: tritium bought at 50,500 sells to a carrier 14 ly
    // away for 103,006; silver at 46,100 sells at Arc City for 49,421; hydrogen fuel at 107 to
    // an Odyssey settlement for 187.
    assert_eq!(
        summary(&r.for_sale),
        vec![
            ("tritium", Some("FCX-701"), Some(103_006), Some(52_506)),
            ("silver", Some("Arc City"), Some(49_421), Some(3_321)),
            ("hydrogenfuel", Some("Dovbush Industrial Silo"), Some(187), Some(80)),
        ]
    );
    assert_eq!(
        (r.for_sale[1].name.as_str(), r.for_sale[1].buy_here, r.for_sale[1].stock),
        ("Silver", Some(46_100), 800)
    );
    // Beer is only bought here and limpets aren't traded: neither is looked up.
    assert!(r.unchecked.is_empty());
    let kept: Vec<(&str, i64, &str)> = r.kept.iter().map(|k| (k.name.as_str(), k.tonnes, k.reason)).collect();
    assert_eq!(kept, vec![("Gold", 4, "stolen"), ("Limpet", 8, "limpets")]);
}

#[test]
fn the_lookups_filters_apply_to_the_nearby_markets() {
    let f = Filters { exclude_carriers: true, exclude_odyssey: true, ..Filters::default() };
    let (r, _) = run(&f, &hold());
    assert_eq!(
        summary(&r.for_sale),
        vec![
            ("tritium", Some("Arc City"), Some(61_889), Some(11_389)),
            ("silver", Some("Arc City"), Some(49_421), Some(3_321)),
            // Every recorded hydrogen fuel buyer is an Odyssey settlement.
            ("hydrogenfuel", None, None, None),
        ]
    );
}

#[test]
fn carried_goods_need_a_market_with_demand_for_all_of_them() {
    let mut big = hold();
    big[0].count = 15_000;
    let (r, _) = run(&Filters::default(), &big);
    // Arc City (demand 10,210), Molina Platform (14,591) and Fanning Ring (1,922) pay more but
    // can't take 15,000 t; Wiberg Hub (41,322) can.
    assert_eq!(summary(&r.carried)[0], ("gold", Some("Wiberg Hub"), Some(56_126), Some(3_526)));
    // Goods for sale are compared with the best price whatever the demand.
    assert_eq!(r.for_sale[1].best.as_ref().unwrap().station, "Arc City");
}

#[test]
fn lookups_are_capped_carried_first_then_the_goods_furthest_below_average() {
    let m = market();
    let (load, _) = astromech_market::cargo::sellable(&hold());
    // Silver is 1,900 below its average, tritium 500, hydrogen fuel 3.
    let (checked, unchecked) = plan(&m, &load, 3);
    assert_eq!(checked, vec!["silver"]);
    assert_eq!(unchecked, vec!["tritium", "hydrogenfuel"]);
    let (checked, unchecked) = plan(&m, &load, 2);
    assert!(checked.is_empty() && unchecked.len() == 3, "the hold always comes first");

    let r = astromech_market::here::compare(&m, &hold(), &[], &Filters::default(), 3, now());
    assert_eq!(r.unchecked, vec!["Tritium", "Hydrogen Fuel"]);
    assert_eq!(r.for_sale.len(), 1);
}

#[test]
fn a_second_look_is_answered_from_the_cache() {
    let (c, rec) = client();
    compare_here(&c, &lhs2435(), &market(), &hold(), &Filters::default(), 20.0, now()).unwrap();
    compare_here(&c, &lhs2435(), &market(), &hold(), &Filters::default(), 20.0, now()).unwrap();
    assert_eq!(rec.log.lock().unwrap().len(), 5);
}

#[test]
fn market_symbols_are_ardents() {
    assert_eq!(symbol("$gold_name;"), "gold");
    assert_eq!(symbol("$LowTemperatureDiamond_Name;"), "lowtemperaturediamond");
    assert_eq!(symbol("tritium"), "tritium");
}
