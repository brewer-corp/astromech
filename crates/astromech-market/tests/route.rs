//! Trade routes on a recorded Spansh trade job (16 Oct 2026: one hop from Abraham Lincoln in
//! Sol, submitted, polled while queued, then done; astromech-data's fixtures) and the two
//! stations' Ardent markets from the same evening, trimmed to a few commodities
//! (tests/fixtures). No test here touches the network.

use astromech_data::spansh::Polling;
use astromech_data::{Body, Cache, Client, Error, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Pad};
use astromech_market::route::{self, Ask, Kind};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;

macro_rules! spansh {
    ($name:literal) => {
        include_str!(concat!("../../astromech-data/tests/fixtures/", $name))
    };
}

/// Spansh's submit and polls in turn; Ardent's markets by system. Logs every request.
struct Recorded {
    submit: (u16, &'static str),
    polls: Mutex<VecDeque<&'static str>>,
    /// LP 71-165's markets (the destination).
    destination: String,
    log: Mutex<Vec<(String, Request)>>,
}

impl Recorded {
    fn new(submit: (u16, &'static str), polls: &[&'static str]) -> Self {
        Recorded {
            submit,
            polls: Mutex::new(polls.iter().copied().collect()),
            destination: include_str!("fixtures/ardent-markets-lp-71-165.json").to_string(),
            log: Mutex::default(),
        }
    }

    /// The destination's markets with its largest pad changed to `pad` (1 small .. 3 large).
    fn destination_pad(mut self, pad: u8) -> Self {
        self.destination =
            self.destination.replace("\"maxLandingPadSize\": 3", &format!("\"maxLandingPadSize\": {pad}"));
        self
    }

    fn urls(&self) -> Vec<String> {
        self.log.lock().unwrap().iter().map(|(u, _)| u.clone()).collect()
    }

    fn submitted(&self, key: &str) -> Option<String> {
        let log = self.log.lock().unwrap();
        let (_, req) = log.iter().find(|(u, _)| u.ends_with("/api/trade/route"))?;
        let Body::Form(f) = &req.body else { return None };
        f.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
    }
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push((url.to_string(), req.clone()));
        let (status, body) = if url.ends_with("/api/trade/route") {
            self.submit
        } else if url.contains("/api/results/") {
            let body = self.polls.lock().unwrap().pop_front().expect("no more recorded polls");
            (if body.contains("\"queued\"") { 202 } else { 200 }, body)
        } else if url.ends_with("/v2/system/name/Sol/commodities") {
            (200, include_str!("fixtures/ardent-markets-sol.json"))
        } else if url.ends_with("/v2/system/name/LP%2071-165/commodities") {
            return Ok(Response { status: 200, body: self.destination.clone() });
        } else if url.ends_with("/commodities") {
            (200, "[]")
        } else {
            panic!("no recorded answer for {url}")
        };
        Ok(Response { status, body: body.to_string() })
    }
}

const SUBMIT: (u16, &str) = (202, spansh!("spansh-trade-submit.json"));
const QUEUED: &str = spansh!("spansh-trade-queued.json");
const DONE: &str = spansh!("spansh-trade-result.json");
const NO_WAIT: Polling = Polling { every: Duration::ZERO, timeout: Duration::from_secs(5) };
const RANGE: f64 = 20.0;

fn client(rec: Recorded) -> (Client, std::sync::Arc<Recorded>) {
    let rec = std::sync::Arc::new(rec);
    (Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()), rec)
}

/// Just after the recording.
fn now() -> i64 {
    chrono::DateTime::parse_from_rfc3339("2026-10-17T00:00:00Z").unwrap().timestamp()
}

fn ask(kind: Kind, capacity: u32) -> Ask {
    Ask { kind, station: Some("Abraham Lincoln".into()), capacity, credits: 10_000_000, max_hop_ly: 30.0 }
}

fn large() -> Filters {
    Filters { pad: Some(Pad::L), ..Filters::default() }
}

// --- Done when: a recorded Spansh job/poll exchange produces a route ---

#[test]
fn single_hop_from_a_recorded_job_and_its_polls() {
    let (c, rec) = client(Recorded::new(SUBMIT, &[QUEUED, QUEUED, DONE]));
    let r = route::plan(&c, "Sol", RANGE, &ask(Kind::Single, 200), &large(), NO_WAIT, now()).unwrap();

    // Asked from the given station with the ship's numbers and the filters.
    assert_eq!(rec.submitted("station").as_deref(), Some("Abraham Lincoln"));
    assert_eq!(rec.submitted("system").as_deref(), Some("Sol"));
    assert_eq!(rec.submitted("max_hops").as_deref(), Some("1"));
    assert_eq!(rec.submitted("max_cargo").as_deref(), Some("200"));
    assert_eq!(rec.submitted("starting_capital").as_deref(), Some("10000000"));
    assert_eq!(rec.submitted("max_hop_distance").as_deref(), Some("30"));
    assert_eq!(rec.submitted("requires_large_pad").as_deref(), Some("1"));
    assert_eq!(rec.submitted("max_price_age").as_deref(), Some("2592000"), "the trade lookups' 30 days");
    assert_eq!(rec.urls().iter().filter(|u| u.contains("/api/results/")).count(), 3, "polled until done");

    assert_eq!(r.kind, Kind::Single);
    assert_eq!(r.legs.len(), 1);
    let l = &r.legs[0];
    assert_eq!(
        (l.from.station.as_str(), l.to.station.as_str(), l.to.system.as_str()),
        ("Abraham Lincoln", "Starbase Pratchett", "LP 71-165")
    );
    assert_eq!(
        (l.cargo[0].name.as_str(), l.cargo[0].tonnes, l.cargo[0].buy, l.cargo[0].sell),
        ("Biowaste", 200, 110, 754)
    );
    assert_eq!((l.profit_per_t, l.profit, r.profit), (644, 128_800, 128_800));
    assert_eq!((l.jumps, l.source), (2, "Spansh"));

    // Per hour is the documented estimate over the round trip, the way back flown empty: each
    // way 2 jumps and one dock, supercruise to 6 ls there and 505 ls back.
    let there = 2.0 * route::JUMP_S + route::SUPERCRUISE_S + 6.0 * route::SUPERCRUISE_S_PER_LS + route::DOCK_S;
    let back = 2.0 * route::JUMP_S + route::SUPERCRUISE_S + 505.0 * route::SUPERCRUISE_S_PER_LS + route::DOCK_S;
    assert!((l.seconds - there).abs() < 1e-9);
    assert!((r.minutes - (there + back) / 60.0).abs() < 1e-9);
    assert_eq!(r.per_hour, (128_800.0 * 3600.0 / (there + back)).round() as i64);

    // The destination's largest pad comes from its Ardent market; it fits.
    assert_eq!(l.to.pad, Some(Pad::L));
    assert_eq!(r.warning, None);
}

#[test]
fn single_hop_per_hour_compares_with_a_loop_flying_back_empty() {
    // Data too old for any cargo back: the loop flies back empty, the same run as a single hop.
    let week = Filters { max_age_days: Some(7), ..large() };
    let (c, _) = client(Recorded::new(SUBMIT, &[DONE]));
    let single = route::plan(&c, "Sol", RANGE, &ask(Kind::Single, 200), &week, NO_WAIT, now()).unwrap();
    let (c, _) = client(Recorded::new(SUBMIT, &[DONE]));
    let looped = route::plan(&c, "Sol", RANGE, &ask(Kind::Loop, 200), &week, NO_WAIT, now()).unwrap();
    assert!(looped.legs[1].cargo.is_empty());
    assert_eq!(single.profit, looped.profit);
    assert!((single.minutes - looped.minutes).abs() < 1e-9);
    assert_eq!(single.per_hour, looped.per_hour);
}

#[test]
fn a_destination_too_small_for_the_ship_is_flagged() {
    let medium = Filters { pad: Some(Pad::M), ..Filters::default() };
    let (c, rec) = client(Recorded::new(SUBMIT, &[DONE]).destination_pad(1));
    let r = route::plan(&c, "Sol", RANGE, &ask(Kind::Single, 200), &medium, NO_WAIT, now()).unwrap();
    assert_eq!(rec.submitted("requires_large_pad").as_deref(), Some("0"), "Spansh can't require a medium pad");
    assert_eq!(r.legs[0].to.pad, Some(Pad::S));
    assert_eq!(
        r.warning.as_deref(),
        Some("Starbase Pratchett (LP 71-165) has no M pad: its largest is S. Spansh can only require a large pad.")
    );

    // A medium pad there is fine for a medium ship.
    let (c, _) = client(Recorded::new(SUBMIT, &[DONE]).destination_pad(2));
    let r = route::plan(&c, "Sol", RANGE, &ask(Kind::Loop, 200), &medium, NO_WAIT, now()).unwrap();
    assert_eq!((r.legs[0].to.pad, r.legs[1].to.pad), (Some(Pad::M), Some(Pad::L)));
    assert_eq!(r.warning, None);
}

#[test]
fn loop_brings_the_best_cargo_back_to_the_start() {
    let (c, rec) = client(Recorded::new(SUBMIT, &[QUEUED, DONE]));
    let r = route::plan(&c, "Sol", RANGE, &ask(Kind::Loop, 200), &large(), NO_WAIT, now()).unwrap();
    assert_eq!(r.legs.len(), 2);
    let back = &r.legs[1];
    assert_eq!((back.from.station.as_str(), back.to.station.as_str()), ("Starbase Pratchett", "Abraham Lincoln"));
    assert_eq!(back.source, "Ardent Insight");
    // Reactive Armour pays most per tonne (2,674 - 1,581) and a full hold of it is in stock.
    let c0 = &back.cargo[0];
    assert_eq!((c0.name.as_str(), c0.tonnes, c0.profit_per_t, c0.profit), ("Reactive Armour", 200, 1093, 218_600));
    assert_eq!(r.profit, 128_800 + 218_600);
    assert!((back.ly - r.legs[0].ly).abs() < 1e-9);
    assert!(rec.urls().iter().any(|u| u.ends_with("/LP%2071-165/commodities")));

    // A bigger hold: only 644 t of Reactive Armour in stock, so Non-Lethal Weapons make more.
    let (c, _) = client(Recorded::new(SUBMIT, &[DONE]));
    let r = route::plan(&c, "Sol", RANGE, &ask(Kind::Loop, 700), &large(), NO_WAIT, now()).unwrap();
    let c0 = &r.legs[1].cargo[0];
    assert_eq!((c0.name.as_str(), c0.tonnes, c0.profit), ("Non-Lethal Weapons", 700, 739_900));

    // Few credits cap the tonnes bought (after the outbound sale's profit).
    let (c, _) = client(Recorded::new(SUBMIT, &[DONE]));
    let poor = Ask { credits: 100_000, ..ask(Kind::Loop, 200) };
    let r = route::plan(&c, "Sol", RANGE, &poor, &large(), NO_WAIT, now()).unwrap();
    let c0 = &r.legs[1].cargo[0];
    assert_eq!(c0.tonnes, (100_000 + 128_800) / c0.buy.max(1));

    // The station's market data is 19 days old: with a 7-day limit there's nothing to bring back.
    let (c, _) = client(Recorded::new(SUBMIT, &[DONE]));
    let week = Filters { max_age_days: Some(7), ..large() };
    let r = route::plan(&c, "Sol", RANGE, &ask(Kind::Loop, 200), &week, NO_WAIT, now()).unwrap();
    assert!(r.legs[1].cargo.is_empty(), "fly back empty");
    assert_eq!((r.legs[1].profit, r.profit), (0, 128_800));
    assert!(r.per_hour < 128_800 * 3600 / (r.legs[0].seconds as i64), "the empty way back still takes time");
}

#[test]
fn without_a_station_starts_nearest_the_star_that_fits() {
    let (c, rec) = client(Recorded::new(SUBMIT, &[DONE]));
    let a = Ask { station: None, ..ask(Kind::Single, 200) };
    route::plan(&c, "Sol", RANGE, &a, &large(), NO_WAIT, now()).unwrap();
    assert_eq!(rec.submitted("station").as_deref(), Some("Daedalus"), "225 ls, nearer than Abraham Lincoln");

    // Nothing with a market there: say so instead of asking Spansh.
    let (c, rec) = client(Recorded::new(SUBMIT, &[DONE]));
    let e = route::plan(&c, "Nowhere", RANGE, &a, &large(), NO_WAIT, now()).unwrap_err();
    assert!(e.to_string().contains("no station with a market in Nowhere"), "{e}");
    assert!(rec.submitted("station").is_none());
}

#[test]
fn no_route_and_errors() {
    let (c, _) = client(Recorded::new(SUBMIT, &[QUEUED, spansh!("spansh-trade-empty.json")]));
    let r = route::plan(&c, "Sol", RANGE, &ask(Kind::Loop, 200), &large(), NO_WAIT, now()).unwrap();
    assert!(r.legs.is_empty());
    assert_eq!((r.profit, r.per_hour), (0, 0));

    let (c, _) = client(Recorded::new((400, r#"{"error":"Could not find station"}"#), &[]));
    let e = route::plan(&c, "Sol", RANGE, &ask(Kind::Single, 200), &large(), NO_WAIT, now()).unwrap_err();
    assert!(matches!(e, Error::Rejected { .. }));
    assert_eq!(e.to_string(), "Spansh turned the request down: Could not find station.");
}
