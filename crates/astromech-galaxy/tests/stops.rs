//! Inhabited stops' details from Spansh's systems search, on its answer recorded on 12 Nov 2026
//! for the ten stops of the recorded Sol to Maia exact route (`fixtures/spansh-route-stops-maia.json`;
//! each system's bodies, stations, factions and synthesis recipes dropped). No test here touches
//! the network.
#![allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)

use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::stops::{details, Details};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Answers each request in turn with a recorded body, or a dropped connection (`None`).
struct Recorded {
    answers: Mutex<VecDeque<Option<&'static str>>>,
    sent: Mutex<Vec<serde_json::Value>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        assert_eq!(url, "https://spansh.co.uk/api/systems/search");
        let Body::Json(b) = &req.body else { panic!("a JSON post, not {:?}", req.body) };
        self.sent.lock().unwrap().push(b.clone());
        match self.answers.lock().unwrap().pop_front().expect("no more answers recorded") {
            Some(body) => Ok(Response { status: 200, body: body.to_string() }),
            None => Err(TransportError::Offline("no network".into())),
        }
    }
}

fn client(answers: &[Option<&'static str>]) -> (Client, Arc<Recorded>) {
    let rec = Arc::new(Recorded { answers: Mutex::new(answers.iter().copied().collect()), sent: Mutex::default() });
    (Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()), rec)
}

const MAIA: &str = include_str!("fixtures/spansh-route-stops-maia.json");
/// Sol, PSR J1752-2806 and Colonia, recorded the same day.
const COLONIA: &str = include_str!("fixtures/spansh-route-stops.json");

fn maia_stops() -> Vec<String> {
    let plan: serde_json::Value =
        serde_json::from_str(include_str!("../../astromech-data/tests/fixtures/spansh-exact-result.json")).unwrap();
    plan["result"]["jumps"].as_array().unwrap().iter().map(|j| j["name"].as_str().unwrap().to_string()).collect()
}

/// One request names every stop; the inhabited ones come back with their details, the two
/// with no population don't.
#[test]
fn inhabited_stops_get_their_details() {
    let (c, rec) = client(&[Some(MAIA)]);
    let names = maia_stops();
    let got = details(&c, &names);
    assert!(got.failed.is_empty());
    let sent = rec.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0]["filters"]["name"]["value"].as_array().unwrap().len(), 10);
    let found: Vec<&str> = got.details.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(found.len(), 8);
    assert!(!found.contains(&"Arietis Sector UE-O a7-1") && !found.contains(&"Hyades Sector XJ-Z b3"), "{found:?}");
    let (_, sol) = got.details.iter().find(|(n, _)| n == "Sol").unwrap();
    assert_eq!(
        sol,
        &Details {
            population: 18320926115,
            allegiance: Some("Federation".into()),
            government: Some("Democracy".into()),
            security: Some("High".into()),
            faction: Some("Mother Gaia".into()),
            faction_state: Some("Expansion".into()),
        }
    );
}

/// A long route takes several requests. One that fails leaves only its own names without
/// details: what an earlier request found is kept, and the later requests still go.
#[test]
fn a_failed_request_costs_only_its_own_stops() {
    let mut names = maia_stops();
    names.extend((names.len()..100).map(|i| format!("Unlisted {i}")));
    names.extend(["PSR J1752-2806".to_string(), "Colonia".to_string()]);
    let (c, rec) = client(&[Some(MAIA), None, Some(COLONIA)]);
    let got = details(&c, &names);
    assert_eq!(rec.sent.lock().unwrap().len(), 3, "every request went");
    assert_eq!(got.failed.len(), 1);
    assert_eq!(got.failed[0].0, names[50..100]);
    let found: Vec<&str> = got.details.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(found.len(), 10, "{found:?}");
    assert!(found.contains(&"Sol") && found.contains(&"Maia") && found.contains(&"Colonia"), "{found:?}");
}
