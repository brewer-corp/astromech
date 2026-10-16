//! Where to buy a colonisation shopping list, on recorded Ardent answers: nearby exports of
//! ceramic composites, CMM composites, insulating membrane and steel around Sol, asked with
//! maxDistance=20 and maxDaysAgo=30 on 16 Oct 2026 and trimmed to a dozen markets (fleet carrier
//! names and ids replaced). No test here touches the network.

use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Pad};
use astromech_market::list::{plan, Need, Stockist};
use astromech_market::Origin;
use std::sync::{Arc, Mutex};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("fixtures/", $name))
    };
}

/// Answers Ardent's nearby exports from the fixtures by commodity; logs requests.
#[derive(Default)]
struct Recorded {
    log: Mutex<Vec<(String, Request)>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push((url.to_string(), req.clone()));
        let body = if url.ends_with("/commodity/name/ceramiccomposites/nearby/exports") {
            fixture!("ardent-buy-ceramiccomposites-sol.json")
        } else if url.ends_with("/commodity/name/cmmcomposite/nearby/exports") {
            fixture!("ardent-buy-cmmcomposite-sol.json")
        } else if url.ends_with("/commodity/name/insulatingmembrane/nearby/exports") {
            fixture!("ardent-buy-insulatingmembrane-sol.json")
        } else if url.ends_with("/commodity/name/steel/nearby/exports") {
            fixture!("ardent-buy-steel-sol.json")
        } else if url.ends_with("/commodity/name/landenrichmentsystems/nearby/exports") {
            "[]"
        } else {
            panic!("no recorded answer for {url}")
        };
        Ok(Response { status: 200, body: body.to_string() })
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

fn need(symbol: &str, name: &str, tonnes: i64) -> Need {
    Need { symbol: symbol.into(), name: name.into(), tonnes }
}

/// What a construction site still wants, as the shopping list gives it.
fn needs() -> Vec<Need> {
    vec![
        need("steel", "Steel", 6_000),
        need("ceramiccomposites", "Ceramic Composites", 800),
        need("cmmcomposite", "CMM Composite", 3_000),
        need("insulatingmembrane", "Insulating Membrane", 400),
    ]
}

fn within_20() -> Filters {
    Filters { max_distance_ly: Some(20), ..Filters::default() }
}

fn find<'a>(s: &'a [Stockist], station: &str) -> (usize, &'a Stockist) {
    s.iter().enumerate().find(|(_, x)| x.station == station).unwrap_or_else(|| panic!("{station} not listed"))
}

fn names(s: &[Stockist]) -> Vec<&str> {
    s.iter().map(|x| x.station.as_str()).collect()
}

// --- Done when: a station stocking three listed commodities ranks above a closer one stocking one ---

#[test]
fn stations_stocking_more_of_the_list_rank_above_closer_ones() {
    let (c, rec) = client();
    let p = plan(&c, &sol(), &needs(), &within_20(), 20.0, now()).unwrap();
    let s = &p.stations;

    // One buy lookup per commodity, with the filters Ardent understands.
    let log = rec.log.lock().unwrap();
    assert_eq!(log.len(), 4);
    assert!(log.iter().all(|(u, r)| u.ends_with("/nearby/exports")
        && r.query.contains(&("maxDistance".into(), "20".into()))
        && r.query.contains(&("maxDaysAgo".into(), "30".into()))));
    drop(log);

    // Efremov Plant (Sirius, 8.6 ly) sells three of the four; the fleet carrier FCX-801 at
    // Barnard's Star is nearer (6.0 ly) but sells only steel.
    let (i3, three) = find(s, "Efremov Plant");
    let (i1, one) = find(s, "FCX-801");
    assert_eq!((three.stocked, one.stocked), (3, 1));
    assert!(one.hop.ly < three.hop.ly, "{} vs {}", one.hop.ly, three.hop.ly);
    assert!(i3 < i1, "three of the list ranks above one: {:?}", names(s));
    assert_eq!(i3, 0, "nearest of the three-commodity stations: {:?}", names(s));

    // Supply and pad for each commodity it sells, in list order, and what it lacks.
    assert_eq!(three.system, "Sirius");
    assert_eq!(three.pad, Some(Pad::L));
    let lines: Vec<(&str, i64, i64)> = three.lines.iter().map(|l| (l.name.as_str(), l.stock, l.needed)).collect();
    assert_eq!(lines, [("Steel", 25_553, 6_000), ("Ceramic Composites", 2_120, 800), ("CMM Composite", 10_061, 3_000)]);
    assert_eq!(three.missing, ["Insulating Membrane"]);

    // Most of the list first, then nearest: the Barnard's Star stations selling two come after
    // every station selling three, however near they are.
    assert!(s
        .windows(2)
        .all(|w| w[0].stocked > w[1].stocked || (w[0].stocked == w[1].stocked && w[0].hop.ly <= w[1].hop.ly)));
    let (i2, two) = find(s, "Boston Base");
    assert_eq!(two.stocked, 2);
    assert!(two.hop.ly < three.hop.ly && i3 < i2 && i2 < i1);
    assert_eq!(s.iter().filter(|x| x.stocked == 3).count(), 5, "{:?}", names(s));

    assert!(p.unavailable.is_empty() && p.truncated.is_empty());
}

#[test]
fn the_lookup_filters_apply_to_every_commodity() {
    let (c, _) = client();
    // Large pad, no carriers, a week's data: Barnard's Star's medium-pad outpost, the carriers,
    // Sanger Settlement (3 Oct) and Hah Industrial Moulding (6 Oct) drop out.
    let f = Filters { pad: Some(Pad::L), exclude_carriers: true, max_age_days: Some(7), ..within_20() };
    let p = plan(&c, &sol(), &needs(), &f, 20.0, now()).unwrap();
    let got = names(&p.stations);
    for gone in [
        "Levi-Strauss Installation",
        "Heisenberg Colony",
        "FCX-801",
        "FCX-802",
        "Sanger Settlement",
        "Hah Industrial Moulding",
    ] {
        assert!(!got.contains(&gone), "{gone} in {got:?}");
    }
    assert_eq!(got[0], "Efremov Plant");

    // A minimum stock: a market below it doesn't count as selling that commodity. Hah Industrial
    // Moulding has 1,534 t of ceramic composites but only 167 t of CMM and 140 t of membrane.
    let (c, _) = client();
    let f = Filters { min_volume: Some(1_000), ..within_20() };
    let p = plan(&c, &sol(), &needs(), &f, 20.0, now()).unwrap();
    let (_, hah) = find(&p.stations, "Hah Industrial Moulding");
    assert_eq!(hah.stocked, 1);
    assert_eq!(hah.missing, ["Steel", "CMM Composite", "Insulating Membrane"]);
    assert!(p.stations.iter().all(|s| s.lines.iter().all(|l| l.stock >= 1_000)));
}

#[test]
fn commodities_nobody_sells_are_named_and_finished_lines_are_not_asked() {
    let (c, rec) = client();
    let mut list = needs();
    list.push(need("landenrichmentsystems", "Land Enrichment Systems", 50));
    // Already aboard: nothing to buy, so not asked.
    list.push(need("polymers", "Polymers", 0));
    // The same commodity twice is asked once.
    list.push(need("Steel", "Steel", 100));
    let p = plan(&c, &sol(), &list, &within_20(), 20.0, now()).unwrap();
    assert_eq!(p.unavailable, ["Land Enrichment Systems"]);
    assert_eq!(rec.log.lock().unwrap().len(), 5);
    assert!(p.stations.iter().all(|s| s.missing.contains(&"Land Enrichment Systems".to_string())));

    let (c, rec) = client();
    let p = plan(&c, &sol(), &[], &within_20(), 20.0, now()).unwrap();
    assert!(p.stations.is_empty() && p.fetched_at == 0 && rec.log.lock().unwrap().is_empty());
}
