//! Colonisation candidates against recorded Spansh answers (`fixtures/spansh-colony-*.json`,
//! recorded 17 Oct 2026 around Greae Phoea EM-K d9-675, a colony 17,100 ly from Sol, with a
//! 30 ly radius; trimmed to the fields used). The area has every case: systems Spansh's own
//! nearest-populated distance puts in range, systems only a newer colony puts in range (Spansh
//! still says 20 ly or more), systems out of range, and a claimed one.

use astromech_data::spansh::{Page, System};
use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::colonies::CLAIM_RANGE_LY;
use astromech_galaxy::{dist, Candidates, Filters, Galaxy, Here, Interest, Sphere, Store};
use serde_json::Value;
use std::sync::{Arc, Mutex};

const CANDIDATES: &str = include_str!("fixtures/spansh-colony-candidates.json");
const POPULATED: &str = include_str!("fixtures/spansh-colony-populated.json");
const BODIES: &str = include_str!("fixtures/spansh-colony-bodies.json");

const EMK: [f64; 3] = [4758.5, -367.21875, 16423.46875];

/// Answers Spansh's systems search (unpopulated or populated, by the population filter) and
/// bodies search (only the bodies of the systems asked for, as Spansh does), and keeps the
/// request bodies.
#[derive(Default)]
struct Recorded {
    sent: Mutex<Vec<(String, Value)>>,
    /// Answer the unpopulated search with only the systems Spansh's distance settles.
    settled_only: bool,
    /// Answer the unpopulated search with only the claimed system.
    claimed_only: bool,
    /// Say there are more populated systems than the page holds.
    populated_count: Option<u64>,
    /// Answer the unpopulated search with the populated answer (as if the filter were ignored).
    ignore_population: bool,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let Body::Json(b) = &req.body else { panic!("{req:?}") };
        self.sent.lock().unwrap().push((url.to_string(), b.clone()));
        let mut answer: Value = match url {
            "https://spansh.co.uk/api/systems/search" => {
                let unpopulated = b["filters"]["population"]["value"][1] == "0";
                if unpopulated && !self.ignore_population {
                    let mut a: Value = serde_json::from_str(CANDIDATES).unwrap();
                    if self.claimed_only {
                        a["results"].as_array_mut().unwrap().retain(|s| s["is_being_colonised"] == true);
                    }
                    if self.settled_only {
                        a["results"]
                            .as_array_mut()
                            .unwrap()
                            .retain(|s| s["nearest_populated_distance"].as_f64().is_some_and(|ly| ly <= 15.0));
                    }
                    a
                } else {
                    let mut a: Value = serde_json::from_str(POPULATED).unwrap();
                    if let Some(n) = self.populated_count.filter(|_| !unpopulated) {
                        a["count"] = n.into();
                    }
                    a
                }
            }
            "https://spansh.co.uk/api/bodies/search" => {
                let ids = b["filters"]["system_id64"]["value"].as_array().unwrap().clone();
                let mut a: Value = serde_json::from_str(BODIES).unwrap();
                a["results"].as_array_mut().unwrap().retain(|x| ids.contains(&x["system_id64"]));
                a["count"] = a["results"].as_array().unwrap().len().into();
                a
            }
            other => panic!("unexpected request to {other}"),
        };
        answer["size"] = b["size"].clone();
        Ok(Response { status: 200, body: answer.to_string() })
    }
}

fn galaxy(rec: Arc<Recorded>) -> Galaxy {
    let client = Client::with_transport(Cache::in_memory().unwrap(), rec).limiter(Limiter::none());
    let s = Store::in_memory().unwrap();
    // Covered space, so no EDSM fill-in is asked for.
    s.add_coverage(Sphere { center: EMK, radius: 100.0 }, "test", "t").unwrap();
    Galaxy::with_remote(s, client)
}

fn here() -> Here {
    Here { system: "Greae Phoea EM-K d9-675".into(), id64: Some(23204307324883), pos: EMK }
}

fn radius(ly: u32) -> Filters {
    Filters { max_distance_ly: Some(ly), ..Filters::default() }
}

fn search(rec: Arc<Recorded>, interest: Interest) -> Candidates {
    galaxy(rec).colony_candidates(interest, &here(), 20.0, 100, &radius(30)).unwrap()
}

fn names(c: &Candidates) -> Vec<&str> {
    c.hits.iter().map(|h| h.name.as_str()).collect()
}

/// The recorded systems, worked out independently of the lookup: unpopulated, no claim flag,
/// and a populated system within 15 ly (by Spansh's distance or by coordinates).
fn expected() -> Vec<String> {
    let cands: Page<System> = serde_json::from_str(CANDIDATES).unwrap();
    let populated: Page<System> = serde_json::from_str(POPULATED).unwrap();
    cands
        .results
        .into_iter()
        .filter(|s| s.population == 0 && s.is_colonised != Some(true) && s.is_being_colonised != Some(true))
        .filter(|s| {
            s.nearest_populated_distance.is_some_and(|ly| ly <= CLAIM_RANGE_LY)
                || populated.results.iter().any(|p| p.population > 0 && dist(s.pos(), p.pos()) <= CLAIM_RANGE_LY)
        })
        .map(|s| s.name)
        .collect()
}

#[test]
fn a_recorded_answer_gives_only_unclaimed_systems_within_15_ly_of_a_populated_one() {
    let rec = Arc::new(Recorded::default());
    let c = search(rec.clone(), Interest::Any);
    let cands: Page<System> = serde_json::from_str(CANDIDATES).unwrap();
    let populated: Page<System> = serde_json::from_str(POPULATED).unwrap();
    let recorded = |name: &str| cands.results.iter().find(|s| s.name == name).unwrap();

    // Exactly the qualifying systems (17 of the 35 unpopulated), nearest first.
    let mut want = expected();
    assert_eq!(want.len(), 17);
    let mut got: Vec<String> = names(&c).into_iter().map(String::from).collect();
    let ly: Vec<f64> = c.hits.iter().map(|h| h.hop.ly).collect();
    assert!(ly.windows(2).all(|w| w[0] <= w[1]), "nearest first: {ly:?}");
    got.sort();
    want.sort();
    assert_eq!(got, want);
    assert_eq!((c.unchecked, c.warning.as_deref()), (0, None));

    for h in &c.hits {
        let s = recorded(&h.name);
        assert_eq!(s.population, 0, "{}", h.name);
        assert!(!s.is_claimed(), "{}", h.name);
        assert!(h.populated.ly <= CLAIM_RANGE_LY, "{}: {}", h.name, h.populated.ly);
        if let Some(name) = &h.populated.name {
            let p = populated.results.iter().find(|p| &p.name == name).unwrap();
            assert!(p.population > 0);
            assert!((dist(s.pos(), p.pos()) - h.populated.ly).abs() < 1e-9);
        }
    }

    // Spansh says its nearest populated system is 27.4 ly away; the colony that went up since
    // is 9.5 ly away.
    let rescued = c.hits.iter().find(|h| h.name == "Greae Phoea VS-M b39-29").unwrap();
    assert_eq!(recorded(&rescued.name).nearest_populated_distance.map(|ly| ly.round()), Some(27.0));
    assert_eq!(rescued.populated.name.as_deref(), Some("Greae Phoea EM-K d9-675"));
    assert_eq!(rescued.detail, "M (Red dwarf) star · 4 landable · 9.5 ly from Greae Phoea EM-K d9-675");
    // In range by Spansh's own distance.
    assert!(names(&c).contains(&"Greae Phoea ZY-K b40-42"));
    // Claimed (a construction site is active), though in range.
    assert!(!names(&c).contains(&"Greae Phoea VS-M b39-15"));
    // 15.2 ly from the nearest populated system.
    assert!(!names(&c).contains(&"Greae Phoea SX-M b39-41"));

    // Three searches: unpopulated, populated (radius + 15 ly), and the bodies of the hits.
    let sent = rec.sent.lock().unwrap();
    assert_eq!(sent.len(), 3);
    let f = &sent[0].1["filters"];
    assert_eq!(f["population"], serde_json::json!({ "value": ["0", "0"], "comparison": "<=>" }));
    assert_eq!(f["distance"]["max"], "30");
    assert_eq!(sent[0].1["reference_coords"]["x"], EMK[0]);
    let f = &sent[1].1["filters"];
    assert_eq!(f["population"]["value"][0], "1");
    assert_eq!(f["distance"]["max"], "45");
    assert!(f.get("is_colonised").is_none(), "colonies don't all carry the flag");
    assert!(sent[2].0.ends_with("/api/bodies/search"));
    let asked = sent[2].1["filters"]["system_id64"]["value"].as_array().unwrap();
    assert_eq!(asked.len(), 17);
}

#[test]
fn bodies_of_interest_narrow_the_list() {
    let rings = search(Arc::new(Recorded::default()), Interest::Rings);
    assert_eq!(names(&rings), ["Greae Phoea KJ-T c19-51"]);
    assert_eq!(rings.hits[0].bodies.ringed, 3);
    assert!(rings.hits[0].detail.starts_with("G (White-Yellow) star · "), "{}", rings.hits[0].detail);

    let landable = search(Arc::new(Recorded::default()), Interest::Landable);
    assert!(!landable.hits.is_empty());
    assert!(landable.hits.iter().all(|h| h.bodies.landable > 0));
    assert!(landable.hits.len() < search(Arc::new(Recorded::default()), Interest::Any).hits.len());

    let elw = search(Arc::new(Recorded::default()), Interest::EarthLike);
    assert!(elw.hits.is_empty(), "no Earth-like worlds were recorded here");
    assert!(elw.warning.is_none());
}

#[test]
fn spansh_distances_alone_skip_the_populated_search() {
    let rec = Arc::new(Recorded { settled_only: true, ..Recorded::default() });
    let c = search(rec.clone(), Interest::Any);
    assert_eq!(c.hits.len(), 6);
    assert!(c.hits.iter().all(|h| h.populated.name.is_none() && h.populated.ly <= CLAIM_RANGE_LY));
    let zy = c.hits.iter().find(|h| h.name == "Greae Phoea ZY-K b40-42").unwrap();
    assert_eq!(zy.detail, "bodies not on Spansh · 6.2 ly from a populated system");
    let sent = rec.sent.lock().unwrap();
    assert_eq!(sent.len(), 2, "unpopulated and bodies only");
    assert!(sent[1].0.ends_with("/api/bodies/search"));
}

#[test]
fn a_truncated_populated_list_counts_what_it_couldnt_check() {
    let rec = Arc::new(Recorded { populated_count: Some(600), ..Recorded::default() });
    let c = search(rec, Interest::Any);
    assert_eq!(c.hits.len(), 17, "what was proven stays");
    assert_eq!(c.unchecked, 17, "the rest might have a populated system beyond the page");
    assert!(c.warning.unwrap().contains("17 unclaimed systems couldn't be checked"));
}

#[test]
fn an_area_where_everything_is_claimed_gives_nothing_without_a_warning() {
    let c = search(Arc::new(Recorded { claimed_only: true, ..Recorded::default() }), Interest::Any);
    assert!(c.hits.is_empty());
    assert!(c.warning.is_none(), "{:?}", c.warning);
}

#[test]
fn an_ignored_population_filter_gives_nothing_and_a_warning() {
    let c = search(Arc::new(Recorded { ignore_population: true, ..Recorded::default() }), Interest::Any);
    assert!(c.hits.is_empty());
    assert!(c.warning.unwrap().contains("filter may have changed"));
}

#[test]
fn the_age_filter_and_the_cache_apply() {
    let rec = Arc::new(Recorded::default());
    let g = galaxy(rec.clone());
    let recent = Filters { max_age_days: Some(10), ..radius(30) };
    let now = chrono::Utc::now().timestamp();
    let c = g.colony_candidates(Interest::Any, &here(), 20.0, 100, &recent).unwrap();
    assert!(c.hits.iter().all(|h| {
        let t = chrono::DateTime::parse_from_rfc3339(h.updated_at.as_deref().unwrap()).unwrap().timestamp();
        now - t <= 10 * 86_400
    }));
    let n = rec.sent.lock().unwrap().len();
    let _ = g.colony_candidates(Interest::Any, &here(), 20.0, 100, &recent).unwrap();
    assert_eq!(rec.sent.lock().unwrap().len(), n, "the second search is served from the cache");
}
