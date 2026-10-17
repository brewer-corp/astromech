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
            if asked.contains("\"Platinum\"") {
                fixture!("spansh-rings-platinum-sol.json")
            } else if asked.contains("\"Low Temperature Diamonds\"") {
                fixture!("spansh-rings-lowtemperaturediamond-sol.json")
            } else if asked.contains("\"Void Opal\"") {
                fixture!("spansh-rings-opal-sol.json")
            } else {
                panic!("no recorded ring search for {asked}")
            }
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

#[test]
fn hotspots_are_the_nearest_rings_holding_the_chosen_ore() {
    let (c, rec) = client();
    let h = mining::hotspots(&c, &sol(), "platinum", 20.0, 3).unwrap();
    // Asked the way Spansh understands it: the ore's name in the ring signals filter.
    assert!(rec.last_body().contains("ring_signals"), "{}", rec.last_body());
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
    let h = mining::hotspots(&c, &sol(), "lowtemperaturediamond", 20.0, 2).unwrap();
    assert_eq!(hotspot_names(&h), [("Barnard's Star 5 B Ring", 1), ("Barnard's Star 6 B Ring", 1)]);
    assert!(rec.last_body().contains("Low Temperature Diamonds"));
    // Void Opal: the catalogue's name, which is how the rings list it.
    let h = mining::hotspots(&c, &sol(), "opal", 20.0, 1).unwrap();
    assert_eq!(hotspot_names(&h), [("Barnard's Star 6 B Ring", 2)]);
    // Counts one ring's hotspots, not the body's.
    let h = mining::hotspots(&c, &sol(), "platinum", 20.0, 4).unwrap();
    assert_eq!(h[3].ring, "TZ Arietis 3 A Ring");
    assert_eq!(h[3].count, 2);
}
