//! The galaxy-wide best price on recorded Ardent answers: painite imports and tritium exports
//! anywhere (`/v2/commodity/name/{c}/imports|exports?maxDaysAgo=30`, tritium also with
//! `fleetCarriers=false`) asked on 17 Oct 2026, trimmed from 100 markets to a handful each
//! (tests/fixtures; fleet carrier names and ids replaced). No test here touches the network.

use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Here, Pad};
use astromech_market::{anywhere, Prices, Side};
use std::sync::{Arc, Mutex};

/// Answers Ardent's galaxy-wide searches from the fixtures; logs the requests.
#[derive(Default)]
struct Recorded {
    log: Mutex<Vec<(String, Request)>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push((url.to_string(), req.clone()));
        let no_carriers = req.query.contains(&("fleetCarriers".into(), "false".into()));
        let body = if url.ends_with("/v2/commodity/name/painite/imports") {
            include_str!("fixtures/ardent-galaxy-sell-painite.json")
        } else if url.ends_with("/v2/commodity/name/tritium/exports") && no_carriers {
            include_str!("fixtures/ardent-galaxy-buy-tritium-nocarriers.json")
        } else if url.ends_with("/v2/commodity/name/tritium/exports") {
            include_str!("fixtures/ardent-galaxy-buy-tritium.json")
        } else {
            panic!("no recorded answer for {url}")
        };
        Ok(Response { status: 200, body: body.to_string() })
    }
}

impl Recorded {
    fn last(&self) -> (String, Vec<(String, String)>) {
        let log = self.log.lock().unwrap();
        let (url, req) = log.last().expect("a request");
        (url.clone(), req.query.clone())
    }
}

fn client() -> (Client, Arc<Recorded>) {
    let rec = Arc::new(Recorded::default());
    (Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()), rec)
}

/// Just after the fixtures were captured.
fn now() -> i64 {
    chrono::DateTime::parse_from_rfc3339("2026-10-17T00:30:00Z").unwrap().timestamp()
}

fn sol() -> Here {
    Here { system: "Sol".into(), id64: Some(10_477_373_803), pos: [0.0; 3] }
}

fn stations(p: &Prices) -> Vec<&str> {
    p.offers.iter().map(|o| o.station.as_str()).collect()
}

fn has(q: &[(String, String)], k: &str) -> bool {
    q.iter().any(|(key, _)| key == k)
}

// --- Done when: a recorded-response test lists them in price order with data age ---

#[test]
fn sell_anywhere_lists_the_best_prices_in_order_with_data_age() {
    let (c, rec) = client();
    let s = anywhere(&c, &sol(), 20.0, Side::Sell, "Painite", &Filters::default(), now()).unwrap();
    let (url, q) = rec.last();
    assert_eq!(url, "https://api.ardent-insight.com/v2/commodity/name/painite/imports");
    assert!(q.contains(&("maxDaysAgo".into(), "30".into())), "the 30-day default is asked of Ardent: {q:?}");
    assert!(!has(&q, "maxDistance"), "no search radius: {q:?}");

    assert_eq!((s.side, s.here.system.as_str(), s.results.len()), (Side::Sell, "Sol", 1));
    let p = &s.results[0];
    assert_eq!((p.commodity.as_str(), p.side, p.truncated), ("painite", Side::Sell, false));
    assert_eq!(
        stations(p),
        [
            "FCX-800",
            "Parekh Chemical Holdings",
            "FCX-802",
            "SavorgnandeBrazza Legacy",
            "FCX-804",
            "Asgaard",
            "Prior Depot",
            "Borg Grob's Grog Dock",
            // Same price as Berg Holdings: nearer first.
            "Good Landing",
            "Berg Holdings",
            "Radium Station",
            "Levi-Strauss Installation",
        ]
    );
    assert!(p.offers.windows(2).all(|w| w[0].price >= w[1].price), "highest price first");

    // Each market keeps its price, demand and when it was reported.
    let best = &p.offers[0];
    assert_eq!((best.price, best.volume, best.fleet_carrier), (390_780, 1_632, true));
    assert_eq!(best.updated_at, "2026-09-18T09:39:20.000Z");
    assert_eq!(best.system, "Chamaeleon Sector HR-W d1-42");
    // Far beyond any nearby radius, measured from Sol; jumps are straight-line estimates.
    assert!((best.hop.ly - 582.0).abs() < 0.1, "{}", best.hop.ly);
    assert_eq!((best.hop.jumps, best.hop.exact), (30, false));
    let good = p.offers.iter().find(|o| o.station == "Good Landing").unwrap();
    assert_eq!((good.price, good.volume, good.pad), (211_587, 13_874, Some(Pad::L)));
    assert_eq!(good.updated_at, "2026-10-17T00:03:31.000Z");
}

#[test]
fn sell_anywhere_applies_the_data_age_and_station_filters() {
    let (c, rec) = client();
    // A week: the carrier from 18 Sep and Berg Holdings (also 18 Sep) drop out.
    let week = Filters { max_age_days: Some(7), ..Filters::default() };
    let s = anywhere(&c, &sol(), 20.0, Side::Sell, "painite", &week, now()).unwrap();
    assert!(rec.last().1.contains(&("maxDaysAgo".into(), "7".into())));
    let p = &s.results[0];
    assert_eq!(p.offers[0].station, "Parekh Chemical Holdings");
    assert!(!stations(p).iter().any(|s| ["FCX-800", "Berg Holdings"].contains(s)), "{:?}", stations(p));

    // Ardent filters carriers but ignores the pad, so the pad, Odyssey and arrival filters are
    // checked here: medium-pad outposts, the settlement, Asgaard (430,000 ls) and Good Landing
    // (22,200 ls) go.
    let strict = Filters {
        pad: Some(Pad::L),
        exclude_carriers: true,
        exclude_odyssey: true,
        max_arrival_ls: Some(5000),
        // A radius would be meaningless galaxy-wide; it is not applied.
        max_distance_ly: Some(10),
        ..Filters::default()
    };
    let s = anywhere(&c, &sol(), 20.0, Side::Sell, "painite", &strict, now()).unwrap();
    let q = rec.last().1;
    assert!(q.contains(&("fleetCarriers".into(), "false".into())), "{q:?}");
    assert!(!has(&q, "minLandingPadSize") && !has(&q, "maxDistance"), "{q:?}");
    assert_eq!(stations(&s.results[0]), ["Prior Depot", "Borg Grob's Grog Dock", "Radium Station"]);
}

#[test]
fn buy_anywhere_lists_the_cheapest_first_and_asks_ardent_to_drop_carriers() {
    let (c, rec) = client();
    // Carriers sell tritium at 2,565 everywhere: nearest first among them.
    let s = anywhere(&c, &sol(), 20.0, Side::Buy, "tritium", &Filters::default(), now()).unwrap();
    let p = &s.results[0];
    assert!(p.offers.iter().all(|o| o.fleet_carrier && o.price == 2565), "{:?}", stations(p));
    assert!(p.offers.windows(2).all(|w| w[0].hop.ly <= w[1].hop.ly), "nearer first on a tie");
    assert_eq!(p.offers[0].system, "82 Eridani");
    assert_eq!(p.offers.len(), 6);

    // Ardent's 100 cheapest are all carriers, so excluding them must be asked of Ardent, not
    // filtered from that answer (which would leave nothing).
    let stations_only = Filters { exclude_carriers: true, ..Filters::default() };
    let s = anywhere(&c, &sol(), 20.0, Side::Buy, "tritium", &stations_only, now()).unwrap();
    assert!(rec.last().1.contains(&("fleetCarriers".into(), "false".into())));
    let p = &s.results[0];
    assert_eq!(
        (p.offers[0].station.as_str(), p.offers[0].price, p.offers[0].volume),
        ("Piceous Elicitation Ltd", 40_596, 19)
    );
    assert_eq!(p.offers[0].updated_at, "2026-10-02T13:43:36.000Z");
    assert!(p.offers.windows(2).all(|w| w[0].price <= w[1].price), "lowest price first");
    assert!(!p.offers.iter().any(|o| o.fleet_carrier));

    // Without Odyssey settlements only the megaship is left.
    let no_odyssey = Filters { exclude_odyssey: true, ..stations_only };
    let s = anywhere(&c, &sol(), 20.0, Side::Buy, "tritium", &no_odyssey, now()).unwrap();
    assert_eq!(stations(&s.results[0]), ["CB-50 The Green Star"]);
}
