//! Nearby Powerplay targets against recorded Spansh answers (`fixtures/spansh-powerplay-*.json`,
//! recorded 16 Oct 2026 around Lembava for Li Yong-Rui, trimmed to the system fields).

use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Galaxy, Here, Sphere, Store, Target, Targets};
use serde_json::Value;
use std::sync::{Arc, Mutex};

const LYR: &str = "Li Yong-Rui";

/// Answers Spansh's systems search with the fixture for the kind of search asked, and keeps the
/// request bodies.
#[derive(Default)]
struct Recorded {
    sent: Mutex<Vec<Value>>,
    /// Answer every search with this fixture instead (to test the re-check).
    always: Option<&'static str>,
    /// Shuffle the answer (farthest first) to show the order is ours, not Spansh's.
    reversed: bool,
}

fn fixture(name: &str) -> &'static str {
    match name {
        "acquisition" => include_str!("fixtures/spansh-powerplay-acquisition.json"),
        "reinforcement" => include_str!("fixtures/spansh-powerplay-reinforcement.json"),
        "undermining" => include_str!("fixtures/spansh-powerplay-undermining.json"),
        _ => unreachable!(),
    }
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        assert_eq!(url, "https://spansh.co.uk/api/systems/search", "only Spansh is asked");
        let Body::Json(b) = &req.body else { panic!("{req:?}") };
        self.sent.lock().unwrap().push(b.clone());
        let f = &b["filters"];
        let name = if f.get("controlling_power").is_some() && f.get("power_state").is_some() {
            "undermining"
        } else if f.get("controlling_power").is_some() {
            "reinforcement"
        } else {
            "acquisition"
        };
        let mut body: Value = serde_json::from_str(self.always.unwrap_or_else(|| fixture(name))).unwrap();
        if self.reversed {
            body["results"].as_array_mut().unwrap().reverse();
        }
        Ok(Response { status: 200, body: body.to_string() })
    }
}

const LEMBAVA: [f64; 3] = [-43.25, -64.34375, -77.6875];

fn galaxy(rec: Arc<Recorded>) -> Galaxy {
    let client = Client::with_transport(Cache::in_memory().unwrap(), rec).limiter(Limiter::none());
    let s = Store::in_memory().unwrap();
    // Covered space, so no EDSM fill-in is asked for.
    s.add_coverage(Sphere { center: LEMBAVA, radius: 100.0 }, "test", "t").unwrap();
    Galaxy::with_remote(s, client)
}

fn here() -> Here {
    Here { system: "Lembava".into(), id64: Some(3824408316259), pos: LEMBAVA }
}

fn targets(g: &Galaxy, t: Target) -> Targets {
    g.powerplay_targets(t, LYR, &here(), 20.0, 40.0, 10, &Filters::default()).unwrap()
}

fn assert_nearest_first(t: &Targets) {
    let ly: Vec<f64> = t.hits.iter().map(|h| h.hop.ly).collect();
    assert!(ly.windows(2).all(|w| w[0] <= w[1]), "distance order: {ly:?}");
}

#[test]
fn reinforcement_systems_are_the_pledged_powers_nearest_first() {
    let rec = Arc::new(Recorded::default());
    let t = targets(&galaxy(rec.clone()), Target::Reinforce);
    let sent = rec.sent.lock().unwrap()[0].clone();
    assert_eq!(sent["filters"]["controlling_power"]["value"], serde_json::json!([LYR]));
    assert_eq!(sent["filters"]["distance"]["max"], "40");
    assert_eq!(sent["reference_coords"]["x"], -43.25);

    assert_eq!(t.hits.len(), 5);
    assert!(t.hits.iter().all(|h| h.controlling_power.as_deref() == Some(LYR)));
    assert_nearest_first(&t);
    let first = &t.hits[0];
    assert_eq!((first.name.as_str(), first.state.as_deref()), ("Lembava", Some("Stronghold")));
    assert_eq!((first.control_progress, first.reinforcement, first.undermining), (Some(0.250634), Some(0), Some(166)));
    assert_eq!(t.hits[2].name, "Piscium Sector BV-Y b3");
    assert!(t.hits[2].hop.ly > 5.0 && t.hits[2].hop.ly < 6.0, "{}", t.hits[2].hop.ly);
    assert!(t.warning.is_none());
}

#[test]
fn acquisition_systems_carry_the_powers_progress() {
    let rec = Arc::new(Recorded::default());
    let t = targets(&galaxy(rec.clone()), Target::Acquire);
    let sent = rec.sent.lock().unwrap()[0].clone();
    assert_eq!(sent["filters"]["power_state"]["value"], serde_json::json!(["Unoccupied"]));
    assert_eq!(sent["filters"]["power"]["value"], serde_json::json!([LYR]));

    assert!(!t.hits.is_empty());
    assert!(t.hits.iter().all(|h| h.state.as_deref() == Some("Unoccupied") && h.controlling_power.is_none()));
    assert_nearest_first(&t);
    assert_eq!(t.hits[0].name, "Piscium Sector YJ-A b2");
    assert_eq!(t.hits[0].acquisition, Some(0.316925));
    let contested = t.hits.iter().find(|h| h.name == "Piscium Sector GW-W b1-6").unwrap();
    assert_eq!(contested.others, ["Li Yong-Rui", "Pranav Antal"]);
}

#[test]
fn undermining_systems_belong_to_other_powers() {
    let rec = Arc::new(Recorded::default());
    let t = targets(&galaxy(rec.clone()), Target::Undermine);
    let sent = rec.sent.lock().unwrap()[0].clone();
    let asked: Vec<&str> =
        sent["filters"]["controlling_power"]["value"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(asked.len(), 11, "every other power");
    assert!(!asked.contains(&LYR));
    assert_eq!(sent["filters"]["power_state"]["value"], serde_json::json!(["Exploited", "Fortified", "Stronghold"]));

    assert!(!t.hits.is_empty());
    assert!(t.hits.iter().all(|h| h.controlling_power.as_deref().is_some_and(|c| c != LYR)));
    assert_nearest_first(&t);
    assert_eq!(t.hits[0].name, "Piscium Sector EB-X b1-3");
}

#[test]
fn hits_are_in_distance_order_whatever_order_spansh_answers_in() {
    let rec = Arc::new(Recorded { reversed: true, ..Recorded::default() });
    let t = targets(&galaxy(rec), Target::Reinforce);
    assert_nearest_first(&t);
    assert_eq!(t.hits[0].name, "Lembava");
}

#[test]
fn answers_that_ignore_the_filter_are_dropped_with_a_warning() {
    // As if Spansh stopped knowing `controlling_power`: the reinforcement search gets the
    // undermining answer.
    let rec = Arc::new(Recorded { always: Some(fixture("undermining")), ..Recorded::default() });
    let t = targets(&galaxy(rec), Target::Reinforce);
    assert!(t.hits.is_empty());
    assert!(t.warning.unwrap().contains("filters may have changed"));
}

#[test]
fn the_age_filter_applies_and_the_answer_is_cached() {
    let rec = Arc::new(Recorded::default());
    let g = galaxy(rec.clone());
    let fresh = Filters { max_age_days: Some(0), ..Filters::default() };
    let t = g.powerplay_targets(Target::Reinforce, LYR, &here(), 20.0, 40.0, 10, &fresh).unwrap();
    assert!(t.hits.is_empty(), "every recorded report is older than today");
    assert!(t.warning.is_none(), "filtered by the commander, not a Spansh change");
    {
        let sent = rec.sent.lock().unwrap();
        assert!(sent[0]["filters"]["updated_at"]["value"][0].is_string(), "the age goes to Spansh: {:?}", sent[0]);
    }
    let _ = g.powerplay_targets(Target::Reinforce, LYR, &here(), 20.0, 40.0, 10, &fresh).unwrap();
    assert_eq!(rec.sent.lock().unwrap().len(), 1, "the second search is served from the cache");
}
