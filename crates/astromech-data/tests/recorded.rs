//! Behaviour on recorded responses (captured from the live services on 16 and 17 Oct 2026, trimmed;
//! see tests/fixtures). No test here touches the network.

use astromech_data::ardent::{MarketFilter, StationService, Trade};
use astromech_data::spansh::{
    ExactRoute, JobStatus, NeutronRoute, Polling, Reference, RouteStatus, Search, TradeRoute,
};
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

/// Answers by URL substring (last matching rule wins; a sequence answers in turn before the
/// rules) and logs every request.
#[derive(Default)]
struct Recorded {
    rules: Mutex<Vec<(String, Result<Response, TransportError>)>>,
    sequences: Mutex<Vec<(String, std::collections::VecDeque<Response>)>>,
    log: Mutex<Vec<(String, Request)>>,
}

impl Recorded {
    /// Answer requests matching `url_part` with `answers`, one each, in order.
    fn in_turn(&self, url_part: &str, answers: &[(u16, &str)]) -> &Self {
        let q = answers.iter().map(|(status, body)| Response { status: *status, body: body.to_string() }).collect();
        self.sequences.lock().unwrap().push((url_part.into(), q));
        self
    }

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
        for (part, answers) in self.sequences.lock().unwrap().iter_mut() {
            if url.contains(part.as_str()) {
                if let Some(r) = answers.pop_front() {
                    return Ok(r);
                }
            }
        }
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

/// `c` with its log lines kept in the returned list.
fn logged(c: Client) -> (Client, Arc<Mutex<Vec<String>>>) {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let l = lines.clone();
    (c.log(Arc::new(move |s: &str| l.lock().unwrap().push(s.to_string()))), lines)
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
            assert_eq!(e.service(), service);
            let msg = e.to_string();
            assert!(msg.starts_with(service.name()), "names the service: {msg}");
            assert!(msg.contains("does not understand"), "plain words: {msg}");
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

#[test]
fn a_schema_error_is_one_spoken_sentence_without_the_serde_detail() {
    let (c, rec, _) = client();
    let (c, log) = logged(c);
    rec.on("/api-v1/system", 200, r#"{"name": "Sol", "id64": 10477373803}"#);
    let e = c.edsm_system("Sol").unwrap_err();
    let log = log.lock().unwrap();
    assert_eq!(log.len(), 1);
    assert!(log[0].starts_with("EDSM answer to /api-v1/system failed the schema check: "), "{log:?}");
    assert!(log[0].contains("missing field `coords`"), "the detail is logged: {log:?}");
    assert_eq!(e.to_string(), "EDSM sent an answer astromech does not understand. Its format may have changed.");
    match e {
        Error::Schema { detail, .. } => assert!(detail.contains("coords"), "the detail stays in the field: {detail}"),
        e => panic!("{e:?}"),
    }
}

// --- Done when: a cache that can't be used is a miss; expired rows are pruned ---

#[test]
fn a_broken_cache_still_returns_the_network_answer() {
    let dir = temp_dir("broken");
    let path = dir.join("remote.db");
    let (c, rec, _) = client_with(Cache::open(&path).unwrap());
    let (c, log) = logged(c);
    rec.on("/api-v1/system", 200, fixture!("edsm-system.json"));
    // Something else wrecked the file's table: every read and write fails.
    rusqlite::Connection::open(&path).unwrap().execute_batch("DROP TABLE remote_cache").unwrap();
    for _ in 0..2 {
        let sol = c.edsm_system("Sol").unwrap();
        assert!(!sol.cached);
        assert_eq!(sol.value.coords.pos(), [0.0, 0.0, 0.0]);
    }
    assert_eq!(rec.requests(), 2, "nothing could be cached, so each lookup asked");
    let log = log.lock().unwrap();
    assert_eq!(log.len(), 4, "each failed read and write is logged: {log:?}");
    assert!(log[0].starts_with("EDSM answer not read from the remote data cache: no such table"), "{log:?}");
    assert!(log[1].starts_with("EDSM answer not saved to the remote data cache: no such table"), "{log:?}");
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_cache_locked_by_the_other_process_waits_for_it_then_caches() {
    let dir = temp_dir("locked");
    let path = dir.join("remote.db");
    let (c, rec, _) = client_with(Cache::open(&path).unwrap());
    let (c, log) = logged(c);
    rec.on("/api-v1/system", 200, fixture!("edsm-system.json"));
    // The app window is writing: it holds the write lock for a moment.
    let other = rusqlite::Connection::open(&path).unwrap();
    other.execute_batch("BEGIN IMMEDIATE; DELETE FROM remote_cache;").unwrap();
    let (locked_tx, locked_rx) = std::sync::mpsc::channel();
    let writer = std::thread::spawn(move || {
        locked_tx.send(()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(300));
        other.execute_batch("COMMIT").unwrap();
    });
    locked_rx.recv().unwrap();
    let sol = c.edsm_system("Sol").unwrap();
    assert_eq!(sol.value.coords.pos(), [0.0, 0.0, 0.0], "the network answer, lock or not");
    writer.join().unwrap();
    assert!(log.lock().unwrap().is_empty(), "the write waited for the lock: {:?}", log.lock().unwrap());
    assert!(c.edsm_system("Sol").unwrap().cached, "and was saved");
    assert_eq!(rec.requests(), 1);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn expired_rows_are_pruned_so_the_cache_holds_only_live_answers() {
    let dir = temp_dir("prune");
    let path = dir.join("remote.db");
    let (c, rec, clock) = client_with(Cache::open(&path).unwrap());
    rec.on("/api-v1/system", 200, fixture!("edsm-system.json"));
    rec.on("/commodities", 200, fixture!("ardent-commodities.json"));
    c.edsm_system("Sol").unwrap();
    for system in ["Sol", "Achenar", "Lave"] {
        c.ardent_markets(system).unwrap();
    }
    assert_eq!(Cache::open(&path).unwrap().len().unwrap(), 4);
    // The markets have expired (15 min); the system data lasts a week.
    clock.set(T0 + ttl::MARKET);
    c.ardent_markets("Diso").unwrap();
    assert_eq!(Cache::open(&path).unwrap().len().unwrap(), 2, "the system and the new market are left");
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
        "Cannot reach Spansh. Check your internet connection. If it is working, Spansh may be down."
    );

    let (c, rec, _) = client();
    rec.fail("ardent", TransportError::Offline("dns error: failed to lookup address".into()));
    let e = c.ardent_nearest("Sol", StationService::InterstellarFactors, None).unwrap_err();
    assert_eq!(
        e.to_string(),
        "Cannot reach Ardent Insight. Check your internet connection. If it is working, Ardent Insight may be down."
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

    // Powerplay fields (recorded 16 Oct 2026 near Lembava, unoccupied systems Li Yong-Rui can acquire).
    rec.on("/api/systems/search", 200, fixture!("spansh-powerplay-acquisition.json"));
    let q = Search::near(Reference::System("Lembava".into())).power_state(&["Unoccupied"]).power(&["Li Yong-Rui"]);
    let systems = c.spansh_systems(&q).unwrap().value.results;
    let (_, req) = rec.last();
    let sent: serde_json::Value = match &req.body {
        astromech_data::Body::Json(v) => v.clone(),
        other => panic!("{other:?}"),
    };
    assert_eq!(sent["filters"]["power_state"], serde_json::json!({ "value": ["Unoccupied"] }));
    assert_eq!(systems[0].power_state.as_deref(), Some("Unoccupied"));
    assert_eq!(systems[0].controlling_power, None);
    assert_eq!(systems[0].power_conflict_progress[0].power, "Li Yong-Rui");
    assert_eq!(systems[0].power_conflict_progress[0].progress, 0.316925);
    assert!(systems[2].power.iter().any(|p| p == "Pranav Antal"));

    let bodies = c.spansh_bodies(&near_sol()).unwrap().value.results;
    assert_eq!(bodies[0].name, "Earth");
    assert_eq!(bodies[0].subtype.as_deref(), Some("Earth-like world"));
    assert_eq!(bodies[0].signals[0].name, "Human");

    // A system's bodies with biology (recorded 17 Oct 2026: Luhman 16, trimmed).
    rec.on("/api/bodies/search", 200, fixture!("spansh-bodies-biology.json"));
    let q =
        Search::near(Reference::Coords([6.3125, 0.59375, 1.71875])).in_systems(&[22960358574928]).signal("Biological");
    let bodies = c.spansh_bodies(&q).unwrap().value.results;
    let (_, req) = rec.last();
    let sent: serde_json::Value = match &req.body {
        astromech_data::Body::Json(v) => v.clone(),
        other => panic!("{other:?}"),
    };
    assert_eq!(sent["filters"]["signals"], serde_json::json!([{ "name": ["Biological"] }]));
    assert_eq!(sent["filters"]["system_id64"], serde_json::json!({ "value": [22960358574928i64] }));
    let b3 = &bodies[0];
    assert_eq!((b3.name.as_str(), b3.body_id), ("Luhman 16 B 3", Some(17)));
    assert_eq!(b3.signals.iter().find(|s| s.name == "Biological").map(|s| s.count), Some(4));
    assert_eq!(
        b3.genuses.iter().map(|g| g.name.as_str()).collect::<Vec<_>>(),
        ["Fungoids", "Osseus", "Bacterial", "Fonticulus"]
    );
    let osseus = b3.landmarks.iter().find(|l| l.landmark_type == "Osseus").unwrap();
    assert_eq!((osseus.subtype.as_str(), osseus.value), ("Osseus Pumice", Some(3_156_300)));
}

#[test]
fn spansh_system_names_autocomplete() {
    let (c, rec, _) = client();
    rec.on("/api/systems/field_values/system_names", 200, fixture!("spansh-system-names.json"));

    let names = c.spansh_system_names(" Shin ").unwrap();
    assert!(!names.cached);
    let (url, req) = rec.last();
    assert_eq!(url, "https://spansh.co.uk/api/systems/field_values/system_names");
    assert_eq!(req.method, Method::Get);
    assert_eq!(req.query, vec![("q".to_string(), "Shin".to_string())], "the prefix is trimmed");
    let n: Vec<&str> = names.value.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(n, ["Shingon", "Shinigami", "Shinarui", "Shinrarta Dezhra"]);
    let jameson = &names.value[3];
    assert_eq!(jameson.id64, 3932277478106);
    assert_eq!(jameson.pos(), [55.71875, 17.59375, 27.15625]);

    assert!(c.spansh_system_names("Shin").unwrap().cached, "the same prefix is served from the cache");
    assert_eq!(rec.requests(), 1);

    assert!(c.spansh_system_names("  ").unwrap().value.is_empty());
    assert_eq!(rec.requests(), 1, "a blank prefix asks nothing");

    rec.on("/api/systems/field_values/system_names", 200, r#"{"min_max":[],"values":[]}"#);
    assert!(c.spansh_system_names("zzqxv").unwrap().value.is_empty(), "no match is an empty list");
    rec.on("/api/systems/field_values/system_names", 200, r#"{"values":["Sol"]}"#);
    assert_schema_error(c.spansh_system_names("Sol"), Service::Spansh);
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

// --- Trade routes: a recorded job and its polls (16 Oct 2026, Sol / Abraham Lincoln, one hop) ---

fn trade_request() -> TradeRoute {
    TradeRoute {
        system: "Sol".into(),
        station: "Abraham Lincoln".into(),
        max_hops: 1,
        max_hop_distance: 30.0,
        starting_capital: 10_000_000,
        max_cargo: 200,
        max_system_distance: 10_000_000,
        max_price_age: Some(30 * 86_400),
        requires_large_pad: true,
        allow_planetary: true,
        allow_player_owned: true,
    }
}

const NO_WAIT: Polling = Polling { every: std::time::Duration::ZERO, timeout: std::time::Duration::from_secs(5) };

#[test]
fn spansh_trade_route_polls_until_the_job_is_done() {
    let (c, rec, _) = client();
    rec.on("/api/trade/route", 202, fixture!("spansh-trade-submit.json"));
    rec.in_turn(
        "/api/results/",
        &[
            (202, fixture!("spansh-trade-queued.json")),
            (202, fixture!("spansh-trade-queued.json")),
            (200, fixture!("spansh-trade-result.json")),
        ],
    );
    let hops = c.spansh_trade_route_wait(&trade_request(), NO_WAIT).unwrap();

    // The job was submitted as the site's form, then polled three times.
    let log = rec.log.lock().unwrap().clone();
    assert_eq!(log.len(), 4, "one submit and three polls");
    assert_eq!(log[0].0, "https://spansh.co.uk/api/trade/route");
    let astromech_data::Body::Form(form) = &log[0].1.body else { panic!("a form post") };
    for (k, v) in [
        ("system", "Sol"),
        ("station", "Abraham Lincoln"),
        ("max_hops", "1"),
        ("max_hop_distance", "30"),
        ("starting_capital", "10000000"),
        ("max_cargo", "200"),
        ("max_price_age", "2592000"),
        ("requires_large_pad", "1"),
        ("allow_player_owned", "1"),
    ] {
        assert!(form.contains(&(k.into(), v.into())), "form has {k}={v}: {form:?}");
    }
    assert!(log[1..].iter().all(|(u, _)| u == "https://spansh.co.uk/api/results/00000000-0000-0000-0000-000000000002"));

    assert_eq!(hops.len(), 1);
    let h = &hops[0];
    assert_eq!((h.source.station.as_str(), h.source.system.as_str()), ("Abraham Lincoln", "Sol"));
    assert_eq!((h.destination.station.as_str(), h.destination.system.as_str()), ("Starbase Pratchett", "LP 71-165"));
    assert!((h.distance - 23.77).abs() < 0.01);
    assert_eq!(h.destination.distance_to_arrival, Some(6.0));
    let cargo = &h.commodities[0];
    assert_eq!((cargo.name.as_str(), cargo.amount, cargo.profit, cargo.total_profit), ("Biowaste", 200, 644, 128_800));
    assert_eq!((cargo.source_commodity.buy_price, cargo.destination_commodity.sell_price), (110, 754));
}

#[test]
fn spansh_trade_route_errors_are_plain_words() {
    // An unknown start station: HTTP 400 with Spansh's reason.
    let (c, rec, _) = client();
    rec.on("/api/trade/route", 400, r#"{"error":"Could not find station"}"#);
    let e = c.spansh_trade_route_wait(&trade_request(), NO_WAIT).unwrap_err();
    assert!(matches!(e, Error::Rejected { service: Service::Spansh, .. }), "{e:?}");
    assert_eq!(e.to_string(), "Spansh turned the request down: Could not find station.");
    assert_eq!(rec.requests(), 1, "no polling after a refused submit");

    // A job that fails while polling (the site reads `{"status": "error", "error": ...}`).
    let (c, rec, _) = client();
    rec.on("/api/trade/route", 202, fixture!("spansh-trade-submit.json"));
    rec.in_turn(
        "/api/results/",
        &[(202, fixture!("spansh-trade-queued.json")), (200, r#"{"status":"error","error":"Station has no market"}"#)],
    );
    let e = c.spansh_trade_route_wait(&trade_request(), NO_WAIT).unwrap_err();
    assert_eq!(e.to_string(), "Spansh turned the request down: Station has no market.");

    // An expired job: HTTP 404.
    let (c, rec, _) = client();
    rec.on("/api/results/", 404, r#"{"error":"Unable to find route"}"#);
    assert_eq!(c.spansh_trade_result("gone").unwrap_err().to_string(), "Spansh found nothing: Unable to find route.");

    // A job that never finishes gives up at the timeout.
    let (c, rec, _) = client();
    rec.on("/api/trade/route", 202, fixture!("spansh-trade-submit.json"));
    rec.on("/api/results/", 202, fixture!("spansh-trade-queued.json"));
    let slow = Polling { every: std::time::Duration::from_millis(5), timeout: std::time::Duration::from_millis(40) };
    let e = c.spansh_trade_route_wait(&trade_request(), slow).unwrap_err();
    assert!(matches!(e, Error::Timeout { service: Service::Spansh }), "{e:?}");
    assert!(rec.requests() > 2, "polled more than once before giving up");

    // No route: done, with nothing in it.
    let (c, rec, _) = client();
    rec.on("/api/results/", 200, fixture!("spansh-trade-empty.json"));
    assert_eq!(c.spansh_trade_result("job").unwrap(), JobStatus::Done(vec![]));

    // A leg without its stations fails the schema check.
    let no_source = fixture!("spansh-trade-result.json").replace("\"source\"", "\"from\"");
    rec.on("/api/results/", 200, &no_source);
    assert_schema_error(c.spansh_trade_result("job"), Service::Spansh);
}

// --- Exact routes: a recorded job (17 Oct 2026, Sol to Maia, an Asp with a 32 t tank) ---

fn exact_request() -> ExactRoute {
    ExactRoute {
        from: "Sol".into(),
        to: "Maia".into(),
        optimal_mass: 1470.0,
        max_fuel_per_jump: 5.0,
        fuel_multiplier: 0.012,
        fuel_power: 2.45,
        base_mass: 400.63,
        tank_size: 32.0,
        internal_tank_size: 0.63,
        range_boost: 0.0,
        cargo: 0.0,
        refuel_every_scoopable: true,
    }
}

#[test]
fn spansh_exact_route_posts_the_drive_and_polls_until_done() {
    let (c, rec, _) = client();
    rec.on("/api/generic/route", 202, fixture!("spansh-exact-submit.json"));
    rec.in_turn(
        "/api/results/",
        &[(202, fixture!("spansh-exact-pending.json")), (200, fixture!("spansh-exact-result.json"))],
    );
    let plan = c.spansh_exact_route_wait(&exact_request(), NO_WAIT).unwrap();

    let log = rec.log.lock().unwrap().clone();
    assert_eq!(log.len(), 3, "one submit and two polls");
    assert_eq!(log[0].0, "https://spansh.co.uk/api/generic/route");
    let astromech_data::Body::Form(form) = &log[0].1.body else { panic!("a form post") };
    // The fields the site posts (its exact-plotter controller), with the values of the
    // recorded request: the job answer echoes them back as its `parameters`.
    let echoed: serde_json::Value = serde_json::from_str(fixture!("spansh-exact-result.json")).unwrap();
    for (k, v) in [
        ("source", "Sol"),
        ("destination", "Maia"),
        ("optimal_mass", "1470"),
        ("max_fuel_per_jump", "5"),
        ("fuel_multiplier", "0.012"),
        ("fuel_power", "2.45"),
        ("base_mass", "400.63"),
        ("tank_size", "32"),
        ("internal_tank_size", "0.63"),
        ("range_boost", "0"),
        ("cargo", "0"),
        ("refuel_every_scoopable", "1"),
        ("use_supercharge", "1"),
        ("use_injections", "0"),
        ("algorithm", "optimistic"),
    ] {
        assert!(form.contains(&(k.into(), v.into())), "form has {k}={v}: {form:?}");
        let param = match k {
            "source" => "source_system",
            "destination" => "destination_system",
            k => k,
        };
        let e = &echoed["parameters"][param];
        let same =
            e.as_str() == Some(v) || e.as_f64().is_some_and(|n| n.to_string() == v) || e.as_bool() == Some(v == "1");
        assert!(same, "Spansh took {k}={v} as {e}");
    }
    assert!(log[1..].iter().all(|(u, _)| u == "https://spansh.co.uk/api/results/00000000-0000-0000-0000-000000000003"));

    assert_eq!(plan.jumps.len(), 10);
    assert_eq!(plan.jumps[0].name, "Sol");
    assert_eq!(plan.jumps.last().unwrap().name, "Maia");
    let refuels: Vec<&str> = plan.jumps.iter().filter(|j| j.must_refuel).map(|j| j.name.as_str()).collect();
    assert_eq!(refuels, ["Tascheter Sector MN-T b3-6", "Hyades Sector DB-W b2-6", "98 k Tauri"]);
    assert!(plan.jumps[8].has_neutron, "98 k Tauri is a neutron star");
    assert!((plan.jumps[1].fuel_used - 4.517).abs() < 0.001);
}

#[test]
fn spansh_exact_route_failures_are_plain_words() {
    // Spansh queues any system name; the job then fails with HTTP 500 and its reason.
    let (c, rec, _) = client();
    rec.on("/api/generic/route", 202, fixture!("spansh-exact-submit.json"));
    rec.on("/api/results/", 500, fixture!("spansh-exact-failed.json"));
    let e = c.spansh_exact_route_wait(&exact_request(), NO_WAIT).unwrap_err();
    assert!(matches!(e, Error::Rejected { service: Service::Spansh, .. }), "{e:?}");
    assert_eq!(e.to_string(), "Spansh turned the request down: Unable to find route.");

    // A 500 without Spansh's error object is still the service having trouble.
    rec.on("/api/results/", 500, "<html>Internal Server Error</html>");
    assert!(matches!(c.spansh_exact_result("job").unwrap_err(), Error::Unavailable { status: 500, .. }));

    // A finished route without its jumps fails the schema check.
    let no_jumps = fixture!("spansh-exact-result.json").replace("\"jumps\"", "\"systems\"");
    rec.on("/api/results/", 200, &no_jumps);
    assert_schema_error(c.spansh_exact_result("job"), Service::Spansh);
}

#[test]
fn spansh_neutron_route_polls_until_done() {
    let (c, rec, _) = client();
    rec.on("/api/route", 202, fixture!("spansh-route-queued.json"));
    rec.in_turn(
        "/api/results/",
        &[(202, fixture!("spansh-route-pending.json")), (200, fixture!("spansh-route-result.json"))],
    );
    let r = NeutronRoute { from: "Sol".into(), to: "Colonia".into(), range: 50.0, efficiency: 60 };
    let plan = c.spansh_route_wait(&r, NO_WAIT).unwrap();
    assert_eq!(rec.requests(), 3, "one submit and two polls");
    assert_eq!(plan.system_jumps.len(), 3);

    // The neutron plotter checks its systems when the job is submitted.
    let (c, rec, _) = client();
    rec.on("/api/route", 400, r#"{"error":"Could not find finishing system"}"#);
    let e = c.spansh_route_wait(&r, NO_WAIT).unwrap_err();
    assert_eq!(e.to_string(), "Spansh turned the request down: Could not find finishing system.");
    assert_eq!(rec.requests(), 1);
}
