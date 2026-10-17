//! Raw-material site lookups from fixed positions, on Spansh bodies searches recorded on
//! 16 Oct 2026 from Sol (trimmed to six bodies and three landmarks each).

use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{sites, Filters, Galaxy, Here, Sphere, Store};
use std::sync::{Arc, Mutex};

/// Answers each site kind's bodies search with its recording and EDSM with no systems; logs
/// the request bodies.
#[derive(Default)]
struct Recorded {
    log: Mutex<Vec<String>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let body = match &req.body {
            Body::Json(v) => v.to_string(),
            _ => String::new(),
        };
        self.log.lock().unwrap().push(format!("{url} {body}"));
        let answer = if url.ends_with("/api-v1/sphere-systems") {
            "[]"
        } else if url.ends_with("/api/bodies/search") {
            if body.contains("Crystalline Shards") {
                include_str!("fixtures/spansh-sites-crystalline_shards.json")
            } else if body.contains("Brain Tree") {
                include_str!("fixtures/spansh-sites-brain_trees.json")
            } else if body.contains("Crashed Ship") {
                include_str!("fixtures/spansh-sites-crashed_ships.json")
            } else if body.contains("Geological") {
                include_str!("fixtures/spansh-sites-geological.json")
            } else {
                panic!("unexpected bodies search {body}")
            }
        } else {
            panic!("unexpected request {url}")
        };
        Ok(Response { status: 200, body: answer.to_string() })
    }
}

fn galaxy(store: Store) -> (Galaxy, Arc<Recorded>) {
    let rec = Arc::new(Recorded::default());
    let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
    (Galaxy::with_remote(store, client), rec)
}

fn sol() -> Here {
    Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] }
}

#[test]
fn nearest_crystalline_shards_from_sol() {
    let (g, rec) = galaxy(Store::in_memory().unwrap());
    let s = g.sites("crystalline_shards", &sol(), 30.0, 3, &Filters::default()).unwrap();
    let h = &s.hits[0];
    assert_eq!(h.body, "Oochorrs SB-J d10-5 B 6 a");
    assert_eq!(h.system, "Oochorrs SB-J d10-5");
    assert!((h.hop.ly - 1426.15).abs() < 0.01, "{}", h.hop.ly);
    // outside the star map: the straight-line estimate at 30 ly per jump
    assert_eq!((h.hop.jumps, h.hop.exact), (48, false));
    assert_eq!(h.arrival_ls.map(f64::round), Some(26249.0));
    assert!(h.detail.starts_with("Crystalline Shards · first at "), "{}", h.detail);
    assert!(h.detail.contains("Technetium 0.7%"), "{}", h.detail);
    assert_eq!(s.source, "Spansh");
    assert_eq!(s.hits.len(), 3);
    assert_eq!(
        sites::spoken(&s),
        "Nearest crystalline shards: Oochorrs SB-J d10-5 B 6 a, 1,426 light years, about 48 jumps, \
         26,249 light seconds from the arrival star."
    );
    let log = rec.log.lock().unwrap();
    let search = log.iter().find(|l| l.contains("/api/bodies/search")).unwrap();
    assert!(search.contains(r#""landmark_subtype":{"value":["Crystalline Shards"]}"#), "{search}");
    assert!(search.contains(r#""reference_coords":{"x":0.0,"y":0.0,"z":0.0}"#), "{search}");
}

#[test]
fn nearest_of_each_kind_from_sol() {
    let (g, _) = galaxy(Store::in_memory().unwrap());
    let nearest = |kind: &str| {
        let s = g.sites(kind, &sol(), 30.0, 5, &Filters::default()).unwrap();
        let h = s.hits[0].clone();
        (h.body, (h.hop.ly * 100.0).round() / 100.0, h.hop.jumps)
    };
    assert_eq!(nearest("brain_trees"), ("Ch'i Lingo B 2".to_string(), 91.89, 4));
    // in the same system the body nearest the arrival star comes first
    assert_eq!(nearest("crashed_ships"), ("Mercury".to_string(), 0.0, 0));
    assert_eq!(nearest("geological"), ("Europa".to_string(), 0.0, 0));
    // the bundled farms need no request
    assert_eq!(nearest("raw_farms"), ("Synuefe BY-F d12-39 A 2".to_string(), 388.01, 13));
}

#[test]
fn known_farms_from_hip_36601() {
    let (g, rec) = galaxy(Store::in_memory().unwrap());
    let here = Here { system: "HIP 36601".into(), id64: Some(84456968626), pos: [337.8125, 562.96875, -1457.84375] };
    let s = g.sites("raw_farms", &here, 30.0, 4, &Filters::default()).unwrap();
    let names: Vec<&str> = s.hits.iter().map(|h| h.body.as_str()).collect();
    assert_eq!(names, ["HIP 36601 C 1 a", "HIP 36601 C 1 d", "HIP 36601 C 3 b", "HIP 36601 C 5 a"]);
    assert_eq!(s.hits[0].detail, "Polonium · crystalline shards");
    assert_eq!(s.source, "astromech");
    assert_eq!(
        sites::spoken(&s),
        "Nearest raw material farm: HIP 36601 C 1 a, polonium from crystalline shards, in this system, \
         153,095 light seconds from the arrival star."
    );
    assert!(!rec.log.lock().unwrap().iter().any(|l| l.contains("spansh")), "farms are bundled");
    // the other HIP 36601 farms sound different
    let d = Here { pos: [337.8125, 562.96875, -1457.84375], ..here.clone() };
    let mut f = g.sites("raw_farms", &d, 30.0, 4, &Filters::default()).unwrap();
    f.hits.remove(0);
    assert!(sites::spoken(&f).contains("HIP 36601 C 1 d, ruthenium from crystalline shards"), "{}", sites::spoken(&f));
}

#[test]
fn jump_count_is_exact_on_the_star_map() {
    // Stars every 9 ly from Sol to Ch'i Lingo (91.89 ly), all inside covered space: at 10 ly
    // per jump that is 10 jumps to the last star and one more, not the estimate's 10.
    let target = [-73.21875, -42.6875, 35.5];
    let d = astromech_galaxy::dist([0.0; 3], target);
    let mut store = Store::in_memory().unwrap();
    let line: Vec<(i64, String, [f64; 3])> = (1..=10)
        .map(|k| {
            let f = 9.0 * k as f64 / d;
            (100 + k, format!("L{k}"), [target[0] * f, target[1] * f, target[2] * f])
        })
        .chain([(5359582071664, "Ch'i Lingo".to_string(), target)])
        .collect();
    store.insert_systems(line.iter().map(|(a, b, c)| (*a, b.as_str(), *c))).unwrap();
    store.add_coverage(Sphere { center: [0.0; 3], radius: 500.0 }, "test", "t").unwrap();
    let (g, _) = galaxy(store);
    let s = g.sites("brain_trees", &sol(), 10.0, 1, &Filters::default()).unwrap();
    assert_eq!(s.hits[0].body, "Ch'i Lingo B 2");
    assert_eq!((s.hits[0].hop.jumps, s.hits[0].hop.exact), (11, true));
    assert!(sites::spoken(&s).contains("91.9 light years, 11 jumps"), "{}", sites::spoken(&s));
}

#[test]
fn filters_apply_to_sites() {
    let (g, _) = galaxy(Store::in_memory().unwrap());
    // Persephone is 484,213 ls out; Europa at 2,612 ls stays
    let near = Filters { max_arrival_ls: Some(3000), ..Filters::default() };
    let s = g.sites("geological", &sol(), 30.0, 10, &near).unwrap();
    assert!(s.hits.iter().all(|h| h.arrival_ls.unwrap() <= 3000.0), "{:?}", s.hits);
    assert_eq!(s.hits[0].body, "Europa");
    assert!(s.hits[0].detail.starts_with("1 geological signal") || s.hits[0].detail.contains("geological signals"));
    // every brain tree in the recording is older than a day
    let fresh = Filters { max_age_days: Some(0), ..Filters::default() };
    assert!(g.sites("brain_trees", &sol(), 30.0, 5, &fresh).unwrap().hits.is_empty());
}

#[test]
fn bodies_without_the_site_are_dropped_with_a_warning() {
    struct Ignored;
    impl Transport for Ignored {
        fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
            let body =
                if url.contains("edsm") { "[]" } else { include_str!("fixtures/spansh-sites-crashed_ships.json") };
            Ok(Response { status: 200, body: body.into() })
        }
    }
    let client = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Ignored)).limiter(Limiter::none());
    let g = Galaxy::with_remote(Store::in_memory().unwrap(), client);
    // Spansh answered crashed ships to a shards search, as if it had ignored the filter
    let s = g.sites("crystalline_shards", &sol(), 30.0, 5, &Filters::default()).unwrap();
    assert!(s.hits.is_empty());
    assert!(s.warning.unwrap().contains("without crystalline shards"));
    assert!(g.sites("nope", &sol(), 30.0, 5, &Filters::default()).is_err());
}

/// The data age goes to Spansh as an `updated_at` range starting that many days ago, so the
/// nearest matches are already fresh; without an age filter nothing is sent.
#[test]
fn the_data_age_is_sent_with_the_bodies_search() {
    let (g, rec) = galaxy(Store::in_memory().unwrap());
    let week = Filters { max_age_days: Some(7), ..Filters::default() };
    let now = chrono::Utc::now().timestamp();
    g.sites("geological", &sol(), 30.0, 3, &week).unwrap();
    g.sites("brain_trees", &sol(), 30.0, 3, &Filters::default()).unwrap();
    let sent: Vec<serde_json::Value> = rec
        .log
        .lock()
        .unwrap()
        .iter()
        .filter_map(|l| l.strip_prefix("https://spansh.co.uk/api/bodies/search "))
        .map(|b| serde_json::from_str(b).unwrap())
        .collect();
    assert_eq!(sent.len(), 2);
    let age = &sent[0]["filters"]["updated_at"];
    assert_eq!(age["comparison"], "<=>");
    let since = chrono::DateTime::parse_from_rfc3339(age["value"][0].as_str().unwrap()).unwrap().timestamp();
    assert!((now - 7 * 86_400 - since).abs() <= 2 * 3600, "{age}");
    assert!(sent[1]["filters"].get("updated_at").is_none(), "{}", sent[1]);
}
