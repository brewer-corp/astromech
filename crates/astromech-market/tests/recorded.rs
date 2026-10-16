//! Trade lookups on recorded Ardent answers: nearby imports and exports around Sol asked with
//! maxDistance=40 on 16 Oct 2026, trimmed to a dozen markets each (tests/fixtures; fleet
//! carrier names and ids replaced). No test here touches the network.

use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Galaxy, Here, Pad, Store};
use astromech_market::{prices, search, Origin, Side};
use std::sync::{Arc, Mutex};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("fixtures/", $name))
    };
}

/// Answers Ardent's nearby searches from the fixtures by commodity and side; logs requests.
#[derive(Default)]
struct Recorded {
    log: Mutex<Vec<(String, Request)>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push((url.to_string(), req.clone()));
        let body = if url.contains("/commodity/name/painite/nearby/imports") {
            fixture!("ardent-sell-painite-sol.json")
        } else if url.contains("/commodity/name/platinum/nearby/imports") {
            fixture!("ardent-sell-platinum-sol.json")
        } else if url.contains("/commodity/name/lowtemperaturediamond/nearby/imports") {
            fixture!("ardent-sell-lowtemperaturediamond-sol.json")
        } else if url.contains("/commodity/name/tritium/nearby/exports") {
            fixture!("ardent-buy-tritium-sol.json")
        } else if url.contains("/api-v1/sphere-systems") {
            "[]"
        } else {
            panic!("no recorded answer for {url}")
        };
        Ok(Response { status: 200, body: body.to_string() })
    }
}

impl Recorded {
    fn requests(&self) -> usize {
        self.log.lock().unwrap().len()
    }

    fn last_query(&self) -> Vec<(String, String)> {
        self.log.lock().unwrap().last().map(|(_, r)| r.query.clone()).unwrap_or_default()
    }
}

fn client() -> (Client, Arc<Recorded>) {
    let rec = Arc::new(Recorded::default());
    (Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()), rec)
}

/// The evening the fixtures were captured.
fn now() -> i64 {
    chrono::DateTime::parse_from_rfc3339("2026-10-16T21:00:00Z").unwrap().timestamp()
}

fn sol() -> Origin {
    Origin { system: "Sol".into(), pos: Some([0.0; 3]) }
}

fn stations(p: &astromech_market::Prices) -> Vec<&str> {
    p.offers.iter().map(|o| o.station.as_str()).collect()
}

// --- Done when: the best price within the distance and filters ---

#[test]
fn sell_returns_the_best_price_within_the_distance_and_filters() {
    let (c, rec) = client();
    let f = Filters { max_distance_ly: Some(30), max_age_days: Some(7), ..Filters::default() };
    let r = prices(&c, &sol(), Side::Sell, &["painite"], &f, 20.0, now()).unwrap();
    let p = &r[0];
    assert_eq!((p.commodity.as_str(), p.side, p.truncated), ("painite", Side::Sell, false));

    // Ardent was asked with the filters it understands.
    let q = rec.last_query();
    assert!(q.contains(&("maxDistance".into(), "30".into())), "{q:?}");
    assert!(q.contains(&("maxDaysAgo".into(), "7".into())), "{q:?}");

    let best = &p.offers[0];
    assert_eq!((best.station.as_str(), best.system.as_str()), ("Good Landing", "LHS 1827"));
    assert_eq!((best.price, best.volume, best.pad), (211_587, 13_884, Some(Pad::L)));
    assert!(
        (best.hop.ly - 18.77).abs() < 0.01,
        "measured from Sol's position, not Ardent's rounded 19: {}",
        best.hop.ly
    );
    // Higher prices exist, but too far (LFT 65 at 32 ly) or reported weeks ago (LHS 273, Apan).
    assert!(p.offers.iter().all(|o| o.hop.ly <= 30.0), "{:?}", stations(p));
    assert!(!stations(p).iter().any(|s| ["Townshend Lab", "Radium Station", "Cousteau Vista"].contains(s)));
    assert!(p.offers.windows(2).all(|w| w[0].price >= w[1].price), "highest price first");

    // The same answer without an age limit: the stale offers come back on top.
    let all = Filters { max_distance_ly: Some(40), ..Filters::default() };
    let r = prices(&c, &sol(), Side::Sell, &["painite"], &all, 20.0, now()).unwrap();
    assert_eq!((r[0].offers[0].price, &r[0].offers[0].updated_at[..10]), (289_998, "2026-09-07"));

    // Pad, fleet carriers, Odyssey settlements and arrival distance.
    let strict = Filters {
        max_distance_ly: Some(40),
        max_age_days: Some(7),
        pad: Some(Pad::L),
        exclude_carriers: true,
        exclude_odyssey: true,
        max_arrival_ls: Some(5000),
        ..Filters::default()
    };
    let r = prices(&c, &sol(), Side::Sell, &["painite"], &strict, 20.0, now()).unwrap();
    // Good Landing is 22,200 ls out; Cousteau Vista and the outposts are medium pads.
    assert_eq!(r[0].offers[0].station, "Radium Station", "{:?}", stations(&r[0]));
    assert!(r[0].offers.iter().all(|o| o.pad == Some(Pad::L) && !o.fleet_carrier && !o.odyssey));
    let q = rec.last_query();
    assert!(
        q.contains(&("minLandingPadSize".into(), "3".into())) && q.contains(&("fleetCarriers".into(), "false".into()))
    );
}

// --- Done when: stations whose demand or supply is below what the user asked for are excluded ---

#[test]
fn sell_excludes_markets_with_less_demand_than_asked() {
    let (c, rec) = client();
    let any = Filters { max_distance_ly: Some(40), max_age_days: Some(7), ..Filters::default() };
    let r = prices(&c, &sol(), Side::Sell, &["lowtemperaturediamond"], &any, 20.0, now()).unwrap();
    assert_eq!(
        (r[0].offers[0].station.as_str(), r[0].offers[0].volume),
        ("Sabine Point", 483),
        "best price, small demand"
    );

    let thousand = Filters { min_volume: Some(1000), ..any };
    let r = prices(&c, &sol(), Side::Sell, &["lowtemperaturediamond"], &thousand, 20.0, now()).unwrap();
    assert!(rec.last_query().contains(&("minVolume".into(), "1000".into())));
    let best = &r[0].offers[0];
    assert_eq!((best.station.as_str(), best.price, best.volume), ("Good Landing", 375_061, 6_467));
    assert!(r[0].offers.iter().all(|o| o.volume >= 1000), "{:?}", stations(&r[0]));
}

#[test]
fn buy_returns_the_lowest_price_with_enough_stock() {
    let (c, _) = client();
    let near = Filters { max_distance_ly: Some(40), ..Filters::default() };
    let r = prices(&c, &sol(), Side::Buy, &["tritium"], &near, 20.0, now()).unwrap();
    let p = &r[0];
    assert!(p.offers.windows(2).all(|w| w[0].price <= w[1].price), "lowest price first");
    assert!(p.offers[0].fleet_carrier, "carriers sell tritium cheapest");
    assert_eq!(p.offers[0].price, 2565);

    let stations_only = Filters { exclude_carriers: true, ..near.clone() };
    let r = prices(&c, &sol(), Side::Buy, &["tritium"], &stations_only, 20.0, now()).unwrap();
    assert_eq!((r[0].offers[0].station.as_str(), r[0].offers[0].volume), ("Bogacki Extraction Territory", 31));

    // 31 t and 1 t of stock aren't a load.
    let hundred = Filters { min_volume: Some(100), ..stations_only.clone() };
    let r = prices(&c, &sol(), Side::Buy, &["tritium"], &hundred, 20.0, now()).unwrap();
    let best = &r[0].offers[0];
    assert_eq!(
        (best.station.as_str(), best.price, best.volume, best.pad),
        ("Ray Freeport", 46_968, 1_872, Some(Pad::M))
    );

    let large = Filters { pad: Some(Pad::L), ..hundred };
    let r = prices(&c, &sol(), Side::Buy, &["tritium"], &large, 20.0, now()).unwrap();
    assert_eq!(r[0].offers[0].station, "Olympus Prime");
}

#[test]
fn several_commodities_in_one_lookup_each_ranked_and_cached() {
    let (c, rec) = client();
    let f = Filters { max_distance_ly: Some(40), max_age_days: Some(7), ..Filters::default() };
    let list = ["painite", "Platinum", "lowtemperaturediamond"];
    let r = prices(&c, &sol(), Side::Sell, &list, &f, 20.0, now()).unwrap();
    assert_eq!(
        r.iter().map(|p| p.commodity.as_str()).collect::<Vec<_>>(),
        ["painite", "platinum", "lowtemperaturediamond"]
    );
    assert_eq!(rec.requests(), 3, "one request per commodity");
    assert_eq!(
        r[1].offers[0].station, "Cousteau Vista",
        "platinum: 294,712 at both LFT 65 stations; the nearer one (by ly, then file order) first"
    );
    let again = prices(&c, &sol(), Side::Sell, &list, &f, 20.0, now()).unwrap();
    assert_eq!(rec.requests(), 3, "served from the cache");
    assert_eq!(again, r);
}

#[test]
fn without_a_position_distances_are_ardents() {
    let (c, _) = client();
    let f = Filters { max_distance_ly: Some(30), max_age_days: Some(7), ..Filters::default() };
    let r = prices(&c, &Origin { system: "Sol".into(), pos: None }, Side::Sell, &["painite"], &f, 20.0, now()).unwrap();
    assert_eq!(r[0].offers[0].station, "Good Landing");
    assert_eq!(r[0].offers[0].hop.ly, 19.0);
}

#[test]
fn app_search_keeps_the_best_few_with_jump_counts() {
    let (c, _) = client();
    let g = Galaxy::with_remote(Store::in_memory().unwrap(), c);
    let here = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };
    let f = Filters { max_distance_ly: Some(40), max_age_days: Some(7), ..Filters::default() };
    let s = search(&g, &here, 15.0, Side::Sell, &["painite"], &f, 3, now()).unwrap();
    assert_eq!(s.results[0].offers.len(), 3);
    assert_eq!(stations(&s.results[0]), ["Good Landing", "Cousteau Vista", "Radium Station"]);
    let o = &s.results[0].offers[0];
    assert_eq!(o.hop.jumps, 2, "18.8 ly at 15 ly per jump");
    let json = serde_json::to_value(&s).unwrap();
    assert_eq!(json["results"][0]["offers"][0]["pad"], "L");
    assert_eq!(json["results"][0]["offers"][0]["jumps"], 2);
    assert_eq!(json["side"], "sell");
}
