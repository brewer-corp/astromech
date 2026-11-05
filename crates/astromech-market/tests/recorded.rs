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
        } else if url.contains("/api/bodies/search") {
            let asked = format!("{:?}", req.body);
            if asked.contains("\"Painite\"") && asked.contains("String(\"100\")") && !asked.contains("reserve_level") {
                let all: serde_json::Value =
                    serde_json::from_str(fixture!("spansh-hotspots-painite-sol-100ly.json")).unwrap();
                let band = painite_band(&asked);
                assert!(all.get(&band).is_some(), "no recorded Painite page for band {band}: {asked}");
                return Ok(Response { status: 200, body: all[&band].to_string() });
            } else if asked.contains("\"Painite\"") {
                assert!(!asked.contains("\"count\""), "no recorded Painite band without the radius: {asked}");
                let all: serde_json::Value =
                    serde_json::from_str(fixture!("spansh-hotspots-painite-sol.json")).unwrap();
                let page = if asked.contains("reserve_level") { "pristine" } else { "any" };
                return Ok(Response { status: 200, body: all[page].to_string() });
            } else if asked.contains("\"Platinum\"") {
                fixture!("spansh-rings-platinum-sol.json")
            } else if asked.contains("\"Low Temperature Diamonds\"") {
                fixture!("spansh-rings-lowtemperaturediamond-sol.json")
            } else if asked.contains("\"Void Opal\"") {
                fixture!("spansh-rings-opal-sol.json")
            } else {
                panic!("no recorded ring search for {asked}")
            }
        } else if url.contains("/api/systems/field_values/system_names") {
            r#"{"min_max":[{"id64":2869977753001,"name":"Wolf 1481","x":5.1875,"y":13.375,"z":13.5625}],"values":["Wolf 1481"]}"#
        } else if url.contains("/api-v1/sphere-systems") {
            "[]"
        } else {
            panic!("no recorded answer for {url}")
        };
        Ok(Response { status: 200, body: body.to_string() })
    }
}

/// The band of hotspot counts a Painite search asked for, as the 100 ly fixture keys its
/// pages: "1+" (every ring with Painite), "5+" (5 or more), "5" (exactly 5).
fn painite_band(asked: &str) -> String {
    let Some(at) = asked.find("\"count\": Array [") else { return "1+".into() };
    let nums: Vec<u32> = asked[at..]
        .split(']')
        .next()
        .unwrap()
        .split("Number(")
        .skip(1)
        .map(|n| n.split(')').next().unwrap().parse().unwrap())
        .collect();
    match nums[..] {
        [lo, 999] => format!("{lo}+"),
        [lo, hi] if lo == hi => lo.to_string(),
        _ => panic!("unexpected band {nums:?}"),
    }
}

impl Recorded {
    fn requests(&self) -> usize {
        self.log.lock().unwrap().len()
    }

    fn last_body(&self) -> String {
        self.log.lock().unwrap().last().map(|(_, r)| format!("{:?}", r.body)).unwrap_or_default()
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

    // Ardent's placeholder rows (demand 999,999 at one price, all from 7 Sep, incl. a
    // construction site) are never offered, at the default age limit or any other.
    let default_age = Filters { max_distance_ly: Some(40), ..Filters::default() };
    let year = Filters { max_age_days: Some(365), ..default_age.clone() };
    for f in [&default_age, &year] {
        let r = prices(&c, &sol(), Side::Sell, &["painite"], f, 20.0, now()).unwrap();
        assert_eq!(r[0].offers[0].station, "Good Landing", "{:?}", stations(&r[0]));
        assert!(r[0]
            .offers
            .iter()
            .all(|o| o.volume != 999_999 && o.station_type.as_deref() != Some("SpaceConstructionDepot")));
    }
    let r = prices(&c, &sol(), Side::Sell, &["painite"], &year, 20.0, now()).unwrap();
    assert!(stations(&r[0]).contains(&"FCX-640"), "a real but old offer comes back with a wider age");
    let r = prices(&c, &sol(), Side::Sell, &["painite"], &default_age, 20.0, now()).unwrap();
    assert!(!stations(&r[0]).contains(&"FCX-640"), "39 days old: past the 30-day default");
    let asked_default =
        rec.log.lock().unwrap().iter().any(|(_, q)| q.query.contains(&("maxDaysAgo".into(), "30".into())));
    assert!(asked_default, "the default is asked of Ardent too");

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
    // The carriers asking 2,565 were reported two and three months ago: past the 30-day
    // default, back with a wider age limit.
    let near = Filters { max_distance_ly: Some(40), ..Filters::default() };
    let r = prices(&c, &sol(), Side::Buy, &["tritium"], &near, 20.0, now()).unwrap();
    assert!(!r[0].offers.iter().any(|o| o.fleet_carrier), "{:?}", stations(&r[0]));
    let old = Filters { max_age_days: Some(120), ..near.clone() };
    let r = prices(&c, &sol(), Side::Buy, &["tritium"], &old, 20.0, now()).unwrap();
    let p = &r[0];
    assert!(p.offers.windows(2).all(|w| w[0].price <= w[1].price), "lowest price first");
    assert!(p.offers[0].fleet_carrier, "carriers sell tritium cheapest");
    assert_eq!(p.offers[0].price, 2565);

    let stations_only = Filters { exclude_carriers: true, ..old.clone() };
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

// --- astromech-sd56: sell my cargo (a fixture hold of three mined commodities) ---

use astromech_market::cargo::{plan, sellable, single_filters};

fn hold() -> Vec<astromech_journal::CargoItem> {
    astromech_journal::parse_cargo(fixture!("cargo-mined.json").as_bytes()).unwrap().inventory
}

/// Within 40 ly, prices up to a week old.
fn week() -> Filters {
    Filters { max_distance_ly: Some(40), max_age_days: Some(7), ..Filters::default() }
}

#[test]
fn the_hold_leaves_out_mission_cargo_stolen_goods_and_limpets() {
    let (load, kept) = sellable(&hold());
    let load: Vec<(&str, &str, i64)> = load.iter().map(|l| (l.symbol.as_str(), l.name.as_str(), l.tonnes)).collect();
    assert_eq!(
        load,
        [
            ("lowtemperaturediamond", "Low Temperature Diamonds", 300),
            ("painite", "Painite", 200),
            ("platinum", "Platinum", 100)
        ],
        "most first; the catalogue names platinum, which Cargo.json left unnamed"
    );
    let kept: Vec<(&str, i64, &str)> = kept.iter().map(|k| (k.name.as_str(), k.tonnes, k.reason)).collect();
    assert_eq!(kept, [("Gold", 10, "stolen"), ("Limpet", 30, "limpets"), ("Gold", 12, "mission cargo")]);
}

// --- Done when: the whole-load answer maximises total proceeds within demand caps ---

#[test]
fn whole_load_answer_maximises_proceeds_within_demand_caps() {
    let (c, rec) = client();
    let p = plan(&c, &sol(), &hold(), &week(), 20.0, now()).unwrap();
    assert_eq!(rec.requests(), 3, "one request per commodity; none for the kept cargo");

    // Worked by hand from the fixtures: 300 × 375,061 + 200 × 211,587 + 100 × 288,390.
    let best = &p.stations[0];
    assert_eq!((best.station.as_str(), best.system.as_str()), ("Good Landing", "LHS 1827"));
    assert_eq!(best.total, 112_518_300 + 42_317_400 + 28_839_000);
    assert_eq!((best.sold, best.lines.len()), (600, 3));

    // Independently: every market's take for the whole hold, capped by its demand, from the
    // single-commodity lookups. None beats the answer, and the list is in order.
    let mut totals: std::collections::BTreeMap<i64, i64> = Default::default();
    for (sym, tonnes) in [("lowtemperaturediamond", 300), ("painite", 200), ("platinum", 100)] {
        let any = Filters { min_volume: None, ..week() };
        for o in &prices(&c, &sol(), Side::Sell, &[sym], &any, 20.0, now()).unwrap()[0].offers {
            *totals.entry(o.market_id).or_default() += tonnes.min(o.volume) * o.price;
        }
    }
    assert_eq!(best.total, *totals.values().max().unwrap());
    assert_eq!(p.stations.len(), totals.values().filter(|t| **t > 0).count());
    assert!(p.stations.windows(2).all(|w| w[0].total >= w[1].total));
    for s in &p.stations {
        assert_eq!(s.total, totals[&s.market_id], "{}", s.station);
    }
}

#[test]
fn demand_caps_keep_a_small_buyer_from_winning_a_big_load() {
    let (c, _) = client();
    // 1,000 t of LTDs: Sabine Point pays the most (384,562) but wants only 483 t.
    let mut inv = hold();
    inv[2].count = 1000;
    let p = plan(&c, &sol(), &inv, &week(), 20.0, now()).unwrap();
    let sabine = p.stations.iter().find(|s| s.station == "Sabine Point").unwrap();
    let ltd = sabine.lines.iter().find(|l| l.commodity == "lowtemperaturediamond").unwrap();
    assert_eq!((ltd.carried, ltd.sold, ltd.proceeds), (1000, 483, 483 * 384_562));
    assert_eq!(p.stations[0].station, "Good Landing", "takes all 1,000 t at 375,061");
    // On its own, the LTDs go to the best market that takes all of them.
    let ltds = p.per_commodity.iter().find(|x| x.commodity == "lowtemperaturediamond").unwrap();
    assert_eq!(ltds.best.as_ref().map(|o| o.station.as_str()), Some("Good Landing"));
}

// --- Done when: per-commodity answers match the single-commodity sell lookup ---

#[test]
fn per_commodity_answers_match_the_single_commodity_sell_lookup() {
    let (c, _) = client();
    let p = plan(&c, &sol(), &hold(), &week(), 20.0, now()).unwrap();
    assert_eq!(p.per_commodity.len(), 3);
    for x in &p.per_commodity {
        // What the Commodities page shows for this commodity with demand of at least the load.
        let single = single_filters(&week(), x.carried);
        assert_eq!(single.min_volume, Some(x.carried as u32));
        let lookup = prices(&c, &sol(), Side::Sell, &[x.commodity.as_str()], &single, 20.0, now()).unwrap();
        assert_eq!(x.best.as_ref(), lookup[0].offers.first(), "{}", x.name);
    }
    let names: Vec<(&str, Option<&str>)> =
        p.per_commodity.iter().map(|x| (x.name.as_str(), x.best.as_ref().map(|o| o.station.as_str()))).collect();
    assert_eq!(
        names,
        [
            ("Low Temperature Diamonds", Some("Sabine Point")),
            ("Painite", Some("Good Landing")),
            ("Platinum", Some("Cousteau Vista"))
        ]
    );
}

/// Ardent's answer is full (1,000 rows) of markets wanting 10 t, so the per-commodity answer
/// asks again with the demand it needs instead of trusting the cut-off list.
#[test]
fn a_full_answer_is_asked_again_with_the_demand_needed() {
    struct Full(Mutex<Vec<Vec<(String, String)>>>);
    impl Transport for Full {
        fn send(&self, _url: &str, req: &Request) -> Result<Response, TransportError> {
            self.0.lock().unwrap().push(req.query.clone());
            let row = |id: i64, demand: i64, price: i64| {
                serde_json::json!({ "commodityName": "silver", "marketId": id, "stationName": format!("S{id}"),
                    "stationType": "Coriolis", "distanceToArrival": 100.0, "maxLandingPadSize": 3,
                    "systemAddress": 1, "systemName": "Near", "systemX": 1.0, "systemY": 0.0, "systemZ": 0.0,
                    "buyPrice": 0, "sellPrice": price, "stock": 0, "demand": demand,
                    "updatedAt": "2026-10-16T20:00:00.000Z", "distance": 1 })
            };
            let rows: Vec<serde_json::Value> = if req.query.iter().any(|(k, _)| k == "minVolume") {
                vec![row(5000, 900, 40_000)]
            } else {
                (0..1000).map(|i| row(i, 10, 50_000 - i)).collect()
            };
            Ok(Response { status: 200, body: serde_json::Value::from(rows).to_string() })
        }
    }
    let t = Arc::new(Full(Mutex::new(vec![])));
    let c = Client::with_transport(Cache::in_memory().unwrap(), t.clone()).limiter(Limiter::none());
    let inv = vec![astromech_journal::CargoItem {
        symbol: "silver".into(),
        name: "Silver".into(),
        count: 500,
        ..Default::default()
    }];
    let p = plan(&c, &sol(), &inv, &week(), 20.0, now()).unwrap();
    let asked = t.0.lock().unwrap();
    assert_eq!(asked.len(), 2);
    assert!(asked[1].contains(&("minVolume".into(), "500".into())), "{:?}", asked[1]);
    assert_eq!(p.per_commodity[0].best.as_ref().map(|o| o.market_id), Some(5000));
    // The re-asked market counts for the whole load too: 500 t × 40,000 beats 10 t × 50,000.
    assert_eq!((p.stations[0].market_id, p.stations[0].sold, p.stations[0].total), (5000, 500, 20_000_000));
    assert_eq!(p.stations.len(), 1001, "every listed market, plus the re-asked one once");
}

/// Done when: a load no market has the demand for gets the best partial buyer, not nothing.
#[test]
fn a_load_too_big_for_any_market_gets_the_best_partial_buyer() {
    let (c, _) = client();
    let inv = vec![astromech_journal::CargoItem {
        symbol: "lowtemperaturediamond".into(),
        name: "Low Temperature Diamonds".into(),
        count: 50_000,
        ..Default::default()
    }];
    let p = plan(&c, &sol(), &inv, &week(), 20.0, now()).unwrap();
    let ltd = &p.per_commodity[0];
    assert!(ltd.best.is_none(), "no market wants 50,000 t");

    // Independently, from the single-commodity lookup: the most credits for what a market takes.
    let any = Filters { min_volume: None, ..week() };
    let all = &prices(&c, &sol(), Side::Sell, &["lowtemperaturediamond"], &any, 20.0, now()).unwrap()[0].offers;
    let top = all.iter().map(|o| o.volume.min(50_000) * o.price).max().unwrap();
    let got = ltd.partial.as_ref().expect("a partial buyer");
    assert_eq!(got.volume.min(50_000) * got.price, top);
    assert!(got.volume > 0 && got.volume < 50_000);
    // It is also the headline station for the whole load, taking the same tonnes.
    assert_eq!((p.stations[0].market_id, p.stations[0].sold), (got.market_id, got.volume));

    // A load some market takes whole has no partial answer.
    let small = vec![astromech_journal::CargoItem { count: 5, ..inv[0].clone() }];
    let p = plan(&c, &sol(), &small, &week(), 20.0, now()).unwrap();
    assert!(p.per_commodity[0].best.is_some() && p.per_commodity[0].partial.is_none());
}

// --- astromech-mcxc: mining sale advisor and hotspot finder ---

use astromech_market::mining;

#[test]
fn ore_sale_is_the_best_proceeds_within_demand_not_the_best_price() {
    let (c, _) = client();
    // 600 t of LTDs: Sabine Point pays the most (384,562) but wants only 483 t.
    let s = mining::sale(&c, &sol(), "lowtemperaturediamond", 600, &week(), 20.0, now()).unwrap();
    assert_eq!((s.name.as_str(), s.tonnes), ("Low Temperature Diamonds", 600));
    let best = &s.stations[0];
    assert_eq!((best.station.as_str(), best.sold, best.total), ("Good Landing", 600, 600 * 375_061));
    let sabine = s.stations.iter().find(|x| x.station == "Sabine Point").unwrap();
    assert_eq!((sabine.sold, sabine.total), (483, 483 * 384_562));
    assert!(sabine.total < best.total);
    // The only market with demand for all of it is the one that wins on proceeds as well.
    assert_eq!(s.whole.as_ref().map(|o| o.station.as_str()), Some("Good Landing"));
    assert!(s.stations.windows(2).all(|w| w[0].total >= w[1].total));

    // A small load goes to the best price.
    let s = mining::sale(&c, &sol(), "lowtemperaturediamond", 100, &week(), 20.0, now()).unwrap();
    assert_eq!((s.stations[0].station.as_str(), s.stations[0].total), ("Sabine Point", 100 * 384_562));
    assert_eq!(s.whole.as_ref().map(|o| o.station.as_str()), Some("Sabine Point"));
}

#[test]
fn ore_sale_with_demand_for_none_of_it_is_still_a_partial_sale() {
    let (c, _) = client();
    // More than any market wants: nothing takes all of it, the best partial sale leads.
    let s = mining::sale(&c, &sol(), "platinum", 100_000, &week(), 20.0, now()).unwrap();
    assert!(s.whole.is_none());
    assert_eq!(s.stations[0].station, "Radium Station");
    assert_eq!((s.stations[0].sold, s.stations[0].total), (58_584, 58_584 * 294_712));
}

fn hotspot_names(h: &[mining::Hotspot]) -> Vec<(&str, u32)> {
    h.iter().map(|x| (x.ring.as_str(), x.count)).collect()
}

fn hotspots(c: &Client, ore: &str, limit: usize) -> Vec<mining::Hotspot> {
    let mut h = mining::hotspots(c, &sol(), &mining::Want::ore(ore), None, 20.0).unwrap().rings;
    h.truncate(limit);
    h
}

#[test]
fn hotspots_are_the_nearest_rings_holding_the_chosen_ore() {
    let (c, rec) = client();
    let h = hotspots(&c, "platinum", 3);
    // Asked the way Spansh understands it: the ore's name in the ring signals filter, one page
    // of bodies, no radius or reserve unless asked.
    let body = rec.last_body();
    assert!(body.contains("ring_signals") && body.contains("\"size\": Number(50)"), "{body}");
    assert!(!body.contains("reserve_level") && !body.contains("\"max\""), "{body}");
    assert_eq!(
        hotspot_names(&h),
        [("Barnard's Star 5 A Ring", 1), ("WISE 0855-0714 6 A Ring", 1), ("Luyten's Star 1 A Ring", 1)],
        "the A ring: Barnard's Star 5's B ring has no platinum"
    );
    assert_eq!((h[0].system.as_str(), h[0].ring_type.as_str()), ("Barnard's Star", "Metal Rich"));
    assert!((h[0].hop.ly - 5.95).abs() < 0.01, "{}", h[0].hop.ly);
    assert_eq!(h[0].hop.jumps, 1);
    assert!(h[0].arrival_ls.is_some() && h[0].updated_at.is_some());
    assert!(h.windows(2).all(|w| w[0].hop.ly <= w[1].hop.ly));
}

#[test]
fn hotspots_pick_the_ring_not_the_body() {
    let (c, rec) = client();
    // LTDs: Barnard's Star 5 holds them in its B ring; 6's A ring is empty and its B ring has them.
    let h = hotspots(&c, "lowtemperaturediamond", 2);
    assert_eq!(hotspot_names(&h), [("Barnard's Star 5 B Ring", 1), ("Barnard's Star 6 B Ring", 1)]);
    assert!(rec.last_body().contains("Low Temperature Diamonds"));
    // Void Opal: the catalogue's name, which is how the rings list it.
    let h = hotspots(&c, "opal", 1);
    assert_eq!(hotspot_names(&h), [("Barnard's Star 6 B Ring", 2)]);
    // Counts one ring's hotspots, not the body's.
    let h = hotspots(&c, "platinum", 4);
    assert_eq!(h[3].ring, "TZ Arietis 3 A Ring");
    assert_eq!(h[3].count, 2);
}

// --- astromech-f7i0: the Mining hotspots finder in Places, and its voice query ---

/// Painite rings around Sol (Spansh, 5 Nov 2026, 30 bodies a page): any reserve, and Pristine.
fn painite(c: &Client, want: mining::Want, radius: Option<u32>, order: mining::Order) -> Vec<mining::Hotspot> {
    match order {
        mining::Order::Nearest => mining::hotspots(c, &sol(), &want, radius, 20.0).unwrap().rings,
        mining::Order::Most => mining::most_hotspots(c, &sol(), &want, radius, 20.0, 10).unwrap().rings,
    }
}

/// The finder's search within 100 ly of Sol on the pages Spansh gave on 5 Nov 2026
/// (spansh-hotspots-painite-sol-100ly.json): every ring with Painite (1101 bodies, the nearest
/// 50 out to 32.5 ly), 5 or more (55), 6 or more (30) and exactly 5 (25).
fn painite_100ly(ring_type: Option<&str>) -> (mining::Ranked, Vec<String>) {
    let (c, rec) = client();
    let want = mining::Want { ring_type: ring_type.map(Into::into), ..mining::Want::ore("painite") };
    let r = mining::most_hotspots(&c, &sol(), &want, Some(100), 20.0, 10).unwrap();
    let asked = rec.log.lock().unwrap().iter().map(|(_, q)| painite_band(&format!("{:?}", q.body))).collect();
    (r, asked)
}

#[test]
fn the_finder_ranks_every_ring_within_the_radius_most_hotspots_first() {
    let (r, asked) = painite_100ly(None);
    // The nearest 50 bodies reach 32.5 ly and hold at most 4; asked for the highest counts
    // first, the 10-hotspot ring 69.4 ly out leads. Three requests: every ring (its count
    // suggests 5 or more), then 6 or more, which came back whole.
    assert_eq!(asked, ["1+", "5+", "6+"]);
    assert_eq!((r.requests, r.bodies, r.cut), (3, 1101, mining::Cut::Whole));
    assert_eq!(
        hotspot_names(&r.rings),
        [
            ("LHS 3602 3 A Ring", 10),
            ("LB 3303 B 2 B Ring", 9),
            ("63 chi Leonis BC 2 B Ring", 8),
            ("Naunin A 4 A Ring", 8),
            ("Ross 130 AB 3 A Ring", 7),
            ("Xihe 1 A Ring", 7),
            ("q1 Eridani 1 A Ring", 7),
            ("LTT 7421 A 6 A Ring", 7),
            ("Belarsuk 2 A Ring", 7),
            ("Greila 4 A Ring", 7),
        ],
        "most first, then nearest: Ualame 5's 7 at 94.0 ly is the eleventh"
    );
    assert!((r.rings[0].hop.ly - 69.4).abs() < 0.1, "{}", r.rings[0].hop.ly);
    let also: Vec<(&str, u32)> = r.rings[0].also.iter().map(|s| (s.name.as_str(), s.count)).collect();
    assert_eq!(
        also,
        [("Platinum", 5), ("Monazite", 3), ("Serendibite", 3), ("Rhodplumsite", 2)],
        "most first, Painite left out"
    );
    assert_eq!(r.rings[0].reserve.as_deref(), Some("Common"));
}

#[test]
fn a_ring_type_rare_in_the_radius_lists_the_counts_known_in_full() {
    // Spansh can't filter ring type: 6 or more holds 4 metallic rings, exactly 5 two more, and
    // the fourth request is the last, so 4s (known only out to 32.5 ly) are left out.
    let (r, asked) = painite_100ly(Some("Metallic"));
    assert_eq!(asked, ["1+", "5+", "6+", "5"]);
    assert_eq!((r.requests, r.cut), (mining::REQUESTS, mining::Cut::Least(5)));
    assert_eq!(
        hotspot_names(&r.rings),
        [
            ("Naunin A 4 A Ring", 8),
            ("Greila 4 A Ring", 7),
            ("Parjanya BC 3 A Ring", 6),
            ("Domoirene A 9 A Ring", 6),
            ("Meticuli 2 A Ring", 5),
            ("HR 3499 5 A Ring", 5),
        ]
    );
    assert!(r.rings.iter().all(|h| h.ring_type == "Metallic"));
}

#[test]
fn with_no_count_known_in_full_the_list_says_how_far_it_reaches() {
    // No icy ring holds Painite: after four requests only the nearest 50 bodies (32.5 ly) are
    // known for 1 to 4 hotspots, so the answer covers 32.5 ly, not 100.
    let (r, _) = painite_100ly(Some("Icy"));
    assert_eq!(r.requests, mining::REQUESTS);
    assert!(r.rings.is_empty());
    match r.cut {
        mining::Cut::Reach(ly) => assert!((ly - 32.5).abs() < 0.01, "{ly}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_nearest_rings_are_one_request_and_keep_their_order() {
    let (c, rec) = client();
    let near = painite(&c, mining::Want::ore("painite"), Some(100), mining::Order::Nearest);
    assert!(rec.last_body().contains("\"max\": String(\"100\")"), "the radius goes to Spansh: {}", rec.last_body());
    assert_eq!(rec.requests(), 1);
    assert_eq!((near[0].ring.as_str(), near[0].count), ("Barnard's Star 5 A Ring", 2));
    assert!(near.windows(2).all(|w| w[0].hop.ly <= w[1].hop.ly));
}

#[test]
fn ring_type_is_checked_here_and_reserve_is_asked_of_spansh() {
    let (c, rec) = client();
    let metallic = mining::Want { ring_type: Some("Metallic".into()), ..mining::Want::ore("painite") };
    let h = painite(&c, metallic, None, mining::Order::Nearest);
    assert!(!h.is_empty() && h.iter().all(|r| r.ring_type == "Metallic"), "{:?}", hotspot_names(&h));
    assert_eq!(h[0].ring, "TZ Arietis 3 A Ring");
    assert!(!rec.last_body().contains("Metallic"), "Spansh can't filter ring type: {}", rec.last_body());

    let pristine = mining::Want { reserve: Some(mining::Reserve::Pristine), ..mining::Want::ore("painite") };
    let h = painite(&c, pristine, None, mining::Order::Nearest);
    assert!(rec.last_body().contains("reserve_level"), "{}", rec.last_body());
    assert!(h.iter().all(|r| r.reserve.as_deref() == Some("Pristine")));
    assert_eq!(h[0].ring, "LFT 65 1 A Ring");
    let major = mining::Want { reserve: Some(mining::Reserve::Major), ..mining::Want::ore("painite") };
    painite(&c, major, None, mining::Order::Nearest);
    assert!(rec.last_body().contains("String(\"Major\")"), "Major or better: {}", rec.last_body());
    assert!(rec.last_body().contains("String(\"Pristine\")"), "{}", rec.last_body());
}

/// The shell's command searches from the system typed or picked (a route's next stop) once it
/// is located, else from the commander's position: either way the search and its distances
/// are measured from that origin's coordinates, and the answer says where it searched from.
#[test]
fn a_typed_origin_is_located_and_searched_from() {
    let (c, rec) = client();
    let g = Galaxy::with_remote(Store::in_memory().unwrap(), c);
    let here = g.locate("Wolf 1481").unwrap();
    assert_eq!(here.system, "Wolf 1481");
    let want = mining::Want::ore("painite");
    let found = mining::hotspots_search(&g, &here, 15.0, &want, Some(100), mining::Order::Most, 10).unwrap();
    let body = rec
        .log
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|(u, _)| u.contains("/api/bodies/search"))
        .map(|(_, r)| format!("{:?}", r.body))
        .unwrap();
    assert!(body.contains("reference_coords"), "{body}");
    assert!(body.contains("5.1875") && body.contains("13.5625"), "{body}");
    assert_eq!(found.here.system, "Wolf 1481");
    assert_eq!((found.name.as_str(), found.radius_ly, found.rings.len()), ("Painite", Some(100), 10));
    // Measured from Wolf 1481, not from Sol (where the recorded pages were measured).
    let top = &found.rings[0];
    assert!((top.hop.ly - astromech_galaxy::dist(here.pos, top.system_pos)).abs() < 0.01, "{:?}", top.hop);
    assert!((top.hop.ly - 69.4).abs() > 1.0, "{:?}", top.hop);
    let json = serde_json::to_value(&found).unwrap();
    assert_eq!(json["bodies"], 1101);
    assert!(json.get("least").is_some() && json.get("reach_ly").is_some(), "{json}");
    assert_eq!(json["rings"][0]["jumps"], serde_json::json!(found.rings[0].hop.jumps));
    assert_eq!(json["want"]["ore"], "painite");
    assert!(json["rings"][0]["also"].is_array() && json["rings"][0].get("reserve").is_some());
}

/// Ardent cuts its answer at 1000 rows, placeholders included (they carry a high price, so they
/// come first and use up the slots). A full answer is therefore truncated however many of its
/// rows are placeholders: the markets cut are the real ones with the worst prices. An answer
/// with room left is complete, placeholders or not.
#[test]
fn placeholders_use_up_ardents_rows_so_a_full_answer_is_truncated() {
    struct Rows(usize);
    impl Transport for Rows {
        fn send(&self, _url: &str, _req: &Request) -> Result<Response, TransportError> {
            let rows: Vec<_> = (0..self.0)
                .map(|i| {
                    let placeholder = i < 600;
                    serde_json::json!({
                        "commodityName": "painite", "marketId": 1000 + i, "stationName": format!("S{i}"),
                        "stationType": "Outpost", "distanceToArrival": 100.0, "maxLandingPadSize": 2, "systemAddress": 5,
                        "systemName": "Near", "systemX": 10.0, "systemY": 0.0, "systemZ": 0.0,
                        "buyPrice": 0, "demand": if placeholder { 999_999 } else { 50 },
                        "sellPrice": if placeholder { 289_998 } else { 200_000 - i as i64 },
                        "stock": 0, "updatedAt": "2026-10-16T12:00:00.000Z", "distance": 10
                    })
                })
                .collect();
            Ok(Response { status: 200, body: serde_json::Value::Array(rows).to_string() })
        }
    }
    let ask = |rows: usize| {
        let c = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Rows(rows))).limiter(Limiter::none());
        prices(&c, &sol(), Side::Sell, &["painite"], &Filters::default(), 20.0, now()).unwrap().remove(0)
    };
    let full = ask(1000);
    assert!(full.truncated);
    assert_eq!(full.offers.len(), 400, "the placeholders are not offered");
    assert!(full.offers.iter().all(|o| o.volume != 999_999));
    let room = ask(999);
    assert!(!room.truncated);
    assert_eq!(room.offers.len(), 399);
}
