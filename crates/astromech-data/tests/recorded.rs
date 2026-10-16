//! Behaviour on recorded responses (captured from the live services on 16 Oct 2026, trimmed;
//! see tests/fixtures). No test here touches the network.

use astromech_data::ardent::{MarketFilter, StationService, Trade};
use astromech_data::spansh::{NeutronRoute, Reference, RouteStatus, Search};
use astromech_data::{
    ttl, Cache, Client, Clock, Error, HttpTransport, Limiter, Method, Request, Response, Service, Transport,
    TransportError,
};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("fixtures/", $name))
    };
}

/// Answers by URL substring (last matching rule wins) and logs every request.
#[derive(Default)]
struct Recorded {
    rules: Mutex<Vec<(String, Result<Response, TransportError>)>>,
    log: Mutex<Vec<(String, Request)>>,
}

impl Recorded {
    fn on(&self, url_part: &str, status: u16, body: &str) -> &Self {
        self.rules.lock().unwrap().push((url_part.into(), Ok(Response { status, body: body.into() })));
        self
    }

    fn fail(&self, url_part: &str, e: TransportError) -> &Self {
        self.rules.lock().unwrap().push((url_part.into(), Err(e)));
        self
    }

    fn requests(&self) -> usize {
        self.log.lock().unwrap().len()
    }

    fn last(&self) -> (String, Request) {
        self.log.lock().unwrap().last().cloned().expect("a request was made")
    }
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push((url.into(), req.clone()));
        let rules = self.rules.lock().unwrap();
        let rule = rules.iter().rev().find(|(part, _)| url.contains(part.as_str()));
        rule.map(|(_, r)| r.clone()).unwrap_or_else(|| panic!("no recorded answer for {url}"))
    }
}

struct ManualClock(AtomicI64);

impl ManualClock {
    fn set(&self, t: i64) {
        self.0.store(t, Ordering::SeqCst);
    }
}

impl Clock for ManualClock {
    fn now_s(&self) -> i64 {
        self.0.load(Ordering::SeqCst)
    }
}

const T0: i64 = 1_800_000_000;

fn client_with(cache: Cache) -> (Client, Arc<Recorded>, Arc<ManualClock>) {
    let rec = Arc::new(Recorded::default());
    let clock = Arc::new(ManualClock(AtomicI64::new(T0)));
    let client = Client::with_transport(cache, rec.clone()).clock(clock.clone()).limiter(Limiter::none());
    (client, rec, clock)
}

fn client() -> (Client, Arc<Recorded>, Arc<ManualClock>) {
    client_with(Cache::in_memory().unwrap())
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("astromech-data-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn near_sol() -> Search {
    Search::near(Reference::System("Sol".into())).supply("Gold", 1000).within(40.0)
}

// --- Done when: a cached answer is served without a request until its TTL ends ---

#[test]
fn cached_answer_is_served_without_a_request_until_ttl_ends() {
    let (c, rec, clock) = client();
    rec.on("/api/stations/search", 200, fixture!("spansh-stations-search.json"));

    let first = c.spansh_stations(&near_sol()).unwrap();
    assert!(!first.cached);
    assert_eq!(first.fetched_at, T0);
    assert_eq!(rec.requests(), 1);

    clock.set(T0 + ttl::SEARCH - 1);
    let again = c.spansh_stations(&near_sol()).unwrap();
    assert!(again.cached, "served from the cache inside the TTL");
    assert_eq!(again.fetched_at, T0, "reports when it was really fetched");
    assert_eq!(again.value, first.value);
    assert_eq!(rec.requests(), 1, "no request inside the TTL");

    // a different search is a different cache entry
    c.spansh_stations(&near_sol().within(10.0)).unwrap();
    assert_eq!(rec.requests(), 2);

    clock.set(T0 + ttl::SEARCH);
    let expired = c.spansh_stations(&near_sol()).unwrap();
    assert!(!expired.cached, "refetched once the TTL ends");
    assert_eq!(expired.fetched_at, T0 + ttl::SEARCH);
    assert_eq!(rec.requests(), 3);
}

#[test]
fn cache_survives_a_restart_and_ttls_differ_by_kind() {
    let dir = temp_dir("restart");
    let path = dir.join("remote.db");
    {
        let (c, rec, _) = client_with(Cache::open(&path).unwrap());
        rec.on("/api-v1/system", 200, fixture!("edsm-system.json"));
        rec.on("/commodities", 200, fixture!("ardent-commodities.json"));
        c.edsm_system("Sol").unwrap();
        c.ardent_markets("Sol").unwrap();
        assert_eq!(rec.requests(), 2);
    }
    let (c, rec, clock) = client_with(Cache::open(&path).unwrap());
    rec.on("/commodities", 200, fixture!("ardent-commodities.json"));
    clock.set(T0 + ttl::MARKET + 1);
    let sol = c.edsm_system("Sol").unwrap();
    assert!(sol.cached, "system data lasts a week, and across restarts");
    assert_eq!(sol.value.coords.pos(), [0.0, 0.0, 0.0]);
    assert!(!c.ardent_markets("Sol").unwrap().cached, "market data expires sooner");
    assert_eq!(rec.requests(), 1);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn uncached_calls_always_ask() {
    let (c, rec, _) = client();
    rec.on("/api/results/", 200, fixture!("spansh-route-pending.json"));
    assert_eq!(c.spansh_route_result("00000000-0000-0000-0000-000000000001").unwrap(), RouteStatus::Pending);
    assert_eq!(c.spansh_route_result("00000000-0000-0000-0000-000000000001").unwrap(), RouteStatus::Pending);
    assert_eq!(rec.requests(), 2, "route polling is never served from the cache");
}

// --- Done when: a response that fails the schema check is an error naming the service ---

fn assert_schema_error(r: Result<impl std::fmt::Debug, Error>, service: Service) {
    match r {
        Err(e @ Error::Schema { .. }) => {
            assert_eq!(e.service(), Some(service));
            let msg = e.to_string();
            assert!(msg.starts_with(service.name()), "names the service: {msg}");
            assert!(msg.contains("doesn't understand"), "plain words: {msg}");
        }
        other => panic!("expected a {service} schema error, got {other:?}"),
    }
}

#[test]
fn schema_mismatch_is_an_error_naming_the_service() {
    let dir = temp_dir("schema");
    let path = dir.join("remote.db");
    let (c, rec, _) = client_with(Cache::open(&path).unwrap());

    // A renamed price field must not turn into a price of 0.
    let renamed = fixture!("ardent-exports.json").replace("\"sellPrice\"", "\"sell_price\"");
    rec.on("/nearby/exports", 200, &renamed);
    assert_schema_error(c.ardent_nearby("Sol", "gold", Trade::Exports, &MarketFilter::default()), Service::Ardent);

    // Spansh: results that aren't a list; a station without its system.
    rec.on("/api/stations/search", 200, r#"{"count": 1, "results": {"name": "x"}}"#);
    assert_schema_error(c.spansh_stations(&near_sol()), Service::Spansh);
    let no_system = fixture!("spansh-stations-search.json").replace("\"system_name\"", "\"system\"");
    rec.on("/api/stations/search", 200, &no_system);
    assert_schema_error(c.spansh_stations(&near_sol()), Service::Spansh);

    // A finished route without its jumps.
    let no_jumps = fixture!("spansh-route-result.json").replace("\"system_jumps\"", "\"waypoints\"");
    rec.on("/api/results/", 200, &no_jumps);
    assert_schema_error(c.spansh_route_result("job"), Service::Spansh);

    // EDSM: a system without coordinates; an HTML page (Cloudflare); truncated JSON.
    rec.on("/api-v1/system", 200, r#"{"name": "Sol", "id64": 10477373803}"#);
    assert_schema_error(c.edsm_system("Sol"), Service::Edsm);
    rec.on("/api-system-v1/bodies", 200, "<!DOCTYPE html><html><body>Just a moment...</body></html>");
    assert_schema_error(c.edsm_bodies("Sol"), Service::Edsm);
    rec.on("/api-system-v1/traffic", 200, &fixture!("edsm-traffic.json")[..100]);
    assert_schema_error(c.edsm_traffic("Sol"), Service::Edsm);

    // Nothing that failed the check was cached, and a good answer is fetched next time.
    assert!(Cache::open(&path).unwrap().is_empty().unwrap());
    let before = rec.requests();
    rec.on("/nearby/exports", 200, fixture!("ardent-exports.json"));
    let good = c.ardent_nearby("Sol", "gold", Trade::Exports, &MarketFilter::default()).unwrap();
    assert!(!good.cached);
    assert_eq!(rec.requests(), before + 1);
    assert_eq!(Cache::open(&path).unwrap().len().unwrap(), 1);
    let _ = std::fs::remove_dir_all(dir);
}

// --- Done when: no network gives a plain-words error ---

#[test]
fn no_network_gives_a_plain_words_error() {
    // Real HTTP to a closed local port: connection refused, as with no network.
    let c = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(HttpTransport::new()))
        .limiter(Limiter::none())
        .base_url(Service::Spansh, "http://127.0.0.1:1");
    let e = c.spansh_stations(&near_sol()).unwrap_err();
    assert!(matches!(e, Error::Offline { service: Service::Spansh, .. }), "{e:?}");
    assert_eq!(
        e.to_string(),
        "Can't reach Spansh. Check your internet connection; if it's working, Spansh may be down."
    );

    let (c, rec, _) = client();
    rec.fail("ardent", TransportError::Offline("dns error: failed to lookup address".into()));
    let e = c.ardent_nearest("Sol", StationService::InterstellarFactors, None).unwrap_err();
    assert_eq!(
        e.to_string(),
        "Can't reach Ardent Insight. Check your internet connection; if it's working, Ardent Insight may be down."
    );
    rec.fail("edsm", TransportError::Timeout);
    assert_eq!(c.edsm_system("Sol").unwrap_err().to_string(), "EDSM took too long to answer. Try again in a minute.");
}

#[test]
fn offline_still_serves_a_fresh_cached_answer() {
    let (c, rec, clock) = client();
    rec.on("/api-v1/system", 200, fixture!("edsm-system.json"));
    c.edsm_system("Sol").unwrap();
    rec.fail("edsm", TransportError::Offline("network unreachable".into()));
    clock.set(T0 + 3600);
    assert!(c.edsm_system("Sol").unwrap().cached);
    assert!(matches!(c.edsm_traffic("Sol"), Err(Error::Offline { service: Service::Edsm, .. })));
}

// --- Other service answers become plain-words errors ---

#[test]
fn http_errors_are_plain_words() {
    let (c, rec, _) = client();
    rec.on("/api/stations/search", 400, r#"{"error":"Invalid request"}"#);
    let e = c.spansh_stations(&Search::near(Reference::System("NoSuchSystemXYZ".into()))).unwrap_err();
    assert_eq!(e.to_string(), "Spansh turned the request down: Invalid request.");

    rec.on("/commodities", 404, fixture!("ardent-not-found.json"));
    assert_eq!(
        c.ardent_markets("NoSuchSystemXYZ").unwrap_err().to_string(),
        "Ardent Insight found nothing: System not found."
    );

    rec.on("/api-v1/system", 200, "[]");
    assert_eq!(
        c.edsm_system("NoSuchSystemXYZ").unwrap_err().to_string(),
        "EDSM found nothing: no system named NoSuchSystemXYZ."
    );
    rec.on("/api-system-v1/bodies", 200, "{}");
    assert!(matches!(c.edsm_bodies("NoSuchSystemXYZ"), Err(Error::NotFound { service: Service::Edsm, .. })));

    rec.on("/api/systems/search", 200, r#"{"error":"bad filter"}"#);
    assert!(matches!(c.spansh_systems(&near_sol()), Err(Error::Rejected { service: Service::Spansh, .. })));
    rec.on("/api/bodies/search", 429, "");
    assert_eq!(
        c.spansh_bodies(&near_sol()).unwrap_err().to_string(),
        "Spansh asked astromech to slow down. Try again in a minute."
    );
    rec.on("/nearest/", 503, "<html>Service Unavailable</html>");
    assert_eq!(
        c.ardent_nearest("Sol", StationService::Repair, None).unwrap_err().to_string(),
        "Ardent Insight is having trouble (HTTP 503). Try again later."
    );
}

// --- Each endpoint: request shape and the recorded answer ---

#[test]
fn spansh_searches_parse_recorded_answers() {
    let (c, rec, _) = client();
    rec.on("/api/stations/search", 200, fixture!("spansh-stations-search.json"));
    rec.on("/api/systems/search", 200, fixture!("spansh-systems-search.json"));
    rec.on("/api/bodies/search", 200, fixture!("spansh-bodies-search.json"));

    let page = c.spansh_stations(&near_sol()).unwrap().value;
    let (url, req) = rec.last();
    assert_eq!(url, "https://spansh.co.uk/api/stations/search");
    assert_eq!(req.method, Method::Post);
    assert_eq!(page.count, 224);
    assert_eq!(page.reference.as_ref().unwrap().name, "Sol");
    let galileo = &page.results[0];
    assert_eq!((galileo.name.as_str(), galileo.system_name.as_str()), ("Galileo", "Sol"));
    assert_eq!(galileo.commodity("gold").unwrap().supply, 29189);
    assert!(galileo.has_service("Shipyard") && !galileo.has_service("Interstellar Factors Contact"));
    assert!(galileo.sells_ship("Adder"));
    assert!(galileo.sells_module("Reinforced Alloy", Some(1), Some("I")));
    assert!(!galileo.sells_module("Reinforced Alloy", Some(2), None));

    let systems = c.spansh_systems(&near_sol()).unwrap().value.results;
    assert_eq!(systems[0].name, "WISE 0855-0714");
    assert!(systems[0].minor_faction_presences.iter().any(|f| f.state.as_deref() == Some("Famine")));

    let bodies = c.spansh_bodies(&near_sol()).unwrap().value.results;
    assert_eq!(bodies[0].name, "Earth");
    assert_eq!(bodies[0].subtype.as_deref(), Some("Earth-like world"));
    assert_eq!(bodies[0].signals[0].name, "Human");
}

#[test]
fn spansh_route_job_round_trip() {
    let (c, rec, _) = client();
    rec.on("/api/route", 202, fixture!("spansh-route-queued.json"));
    let job = c
        .spansh_route(&NeutronRoute { from: "Sol".into(), to: "Colonia".into(), range: 50.0, efficiency: 60 })
        .unwrap();
    assert_eq!(job.job, "00000000-0000-0000-0000-000000000001");
    let (url, req) = rec.last();
    assert_eq!(url, "https://spansh.co.uk/api/route");
    assert!(matches!(&req.body, astromech_data::Body::Form(f) if f.contains(&("to".into(), "Colonia".into()))));

    rec.on("/api/results/", 200, fixture!("spansh-route-result.json"));
    let RouteStatus::Done(plan) = c.spansh_route_result(&job.job).unwrap() else { panic!("route should be done") };
    assert_eq!(plan.destination_system, "Colonia");
    assert_eq!(plan.system_jumps[0].system, "Sol");
    assert!(plan.system_jumps[1].neutron_star);
    assert_eq!(rec.last().0, "https://spansh.co.uk/api/results/00000000-0000-0000-0000-000000000001");
}

#[test]
fn ardent_endpoints_parse_recorded_answers() {
    let (c, rec, _) = client();
    rec.on("/nearby/exports", 200, fixture!("ardent-exports.json"));
    rec.on("/nearby/imports", 200, fixture!("ardent-imports.json"));
    rec.on("/nearest/", 200, fixture!("ardent-nearest.json"));
    rec.on("/commodities", 200, fixture!("ardent-commodities.json"));

    let filter = MarketFilter {
        max_distance: Some(900.0),
        max_days_ago: Some(7),
        min_landing_pad: Some(3),
        fleet_carriers: Some(false),
        min_volume: Some(100),
    };
    let sells = c.ardent_nearby("Barnard's Star", "gold", Trade::Exports, &filter).unwrap().value;
    let (url, req) = rec.last();
    assert_eq!(
        url,
        "https://api.ardent-insight.com/v2/system/name/Barnard%27s%20Star/commodity/name/gold/nearby/exports"
    );
    assert!(req.query.contains(&("maxDistance".into(), "500".into())), "clamped to 500 ly: {:?}", req.query);
    for k in ["maxDaysAgo", "minLandingPadSize", "fleetCarriers", "minVolume"] {
        assert!(req.query.iter().any(|(q, _)| q == k), "{k} sent");
    }
    assert_eq!(sells[0].commodity_name, "gold");
    assert!(sells[0].stock > 0 && sells[0].buy_price > 0);
    assert_eq!(sells[0].distance, Some(16.0));

    let buys = c.ardent_nearby("Sol", "gold", Trade::Imports, &MarketFilter::default()).unwrap().value;
    assert!(rec.last().1.query.is_empty());
    assert!(buys[0].demand > 0 && buys[0].sell_price > 0);

    let ifactors = c.ardent_nearest("Sol", StationService::InterstellarFactors, Some(3)).unwrap().value;
    assert!(rec.last().0.ends_with("/v2/system/name/Sol/nearest/interstellar-factors"));
    assert_eq!(ifactors[0].system_name, "Barnard's Star");
    assert_eq!(ifactors[0].max_landing_pad_size, Some(3));

    let markets = c.ardent_markets("Sol").unwrap().value;
    assert!(markets.iter().all(|l| l.system_name == "Sol" && l.distance.is_none()));
}

#[test]
fn edsm_endpoints_parse_recorded_answers() {
    let (c, rec, _) = client();
    rec.on("/api-v1/system", 200, fixture!("edsm-system.json"));
    rec.on("/api-system-v1/bodies", 200, fixture!("edsm-bodies.json"));
    rec.on("/api-system-v1/traffic", 200, fixture!("edsm-traffic.json"));
    rec.on("/api-v1/sphere-systems", 200, fixture!("edsm-sphere.json"));

    assert_eq!(c.edsm_system("Sol").unwrap().value.id64, 10477373803);
    assert!(rec.last().1.query.contains(&("systemName".into(), "Sol".into())));

    let bodies = c.edsm_bodies("Alpha Centauri").unwrap().value;
    assert_eq!(bodies.name, "Alpha Centauri");
    assert_eq!(bodies.bodies[0].body_type, "Star");
    assert_eq!(bodies.bodies[0].is_scoopable, Some(true));

    let traffic = c.edsm_traffic("Sol").unwrap().value;
    assert!(traffic.traffic.total >= traffic.traffic.week && traffic.traffic.week >= traffic.traffic.day);

    let sphere = c.edsm_sphere([0.0; 3], 500.0).unwrap().value;
    assert!(rec.last().1.query.contains(&("radius".into(), "100".into())), "capped at 100 ly");
    assert!(sphere.iter().any(|s| s.name == "Alpha Centauri" && s.coords.is_some()));
    rec.on("/api-v1/sphere-systems", 200, "{}");
    assert!(c.edsm_sphere([1e5, 0.0, 0.0], 50.0).unwrap().value.is_empty(), "EDSM's empty answer is no systems");
}
