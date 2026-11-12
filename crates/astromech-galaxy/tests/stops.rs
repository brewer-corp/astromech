//! Inhabited stops' details from Spansh's systems search, on its answer recorded on 12 Nov 2026
//! for the ten stops of the recorded Sol to Maia exact route (`fixtures/spansh-route-stops-maia.json`;
//! each system's bodies, stations, factions and synthesis recipes dropped). No test here touches
//! the network.
#![allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)

use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::stops::{details, Details};
use std::sync::{Arc, Mutex};

struct Recorded {
    answer: Result<&'static str, ()>,
    sent: Mutex<Vec<serde_json::Value>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        assert_eq!(url, "https://spansh.co.uk/api/systems/search");
        let Body::Json(b) = &req.body else { panic!("a JSON post, not {:?}", req.body) };
        self.sent.lock().unwrap().push(b.clone());
        match self.answer {
            Ok(body) => Ok(Response { status: 200, body: body.to_string() }),
            Err(()) => Err(TransportError::Offline("no network".into())),
        }
    }
}

fn client(answer: Result<&'static str, ()>) -> (Client, Arc<Recorded>) {
    let rec = Arc::new(Recorded { answer, sent: Mutex::default() });
    (Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()), rec)
}

fn maia_stops() -> Vec<String> {
    let plan: serde_json::Value =
        serde_json::from_str(include_str!("../../astromech-data/tests/fixtures/spansh-exact-result.json")).unwrap();
    plan["result"]["jumps"].as_array().unwrap().iter().map(|j| j["name"].as_str().unwrap().to_string()).collect()
}

/// One request names every stop; the inhabited ones come back with their details, the two
/// with no population don't.
#[test]
fn inhabited_stops_get_their_details() {
    let (c, rec) = client(Ok(include_str!("fixtures/spansh-route-stops-maia.json")));
    let names = maia_stops();
    let got = details(&c, &names).unwrap();
    let sent = rec.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0]["filters"]["name"]["value"].as_array().unwrap().len(), 10);
    let found: Vec<&str> = got.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(found.len(), 8);
    assert!(!found.contains(&"Arietis Sector UE-O a7-1") && !found.contains(&"Hyades Sector XJ-Z b3"), "{found:?}");
    let (_, sol) = got.iter().find(|(n, _)| n == "Sol").unwrap();
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

#[test]
fn a_failed_lookup_is_an_error() {
    let (c, _) = client(Err(()));
    assert!(details(&c, &maia_stops()).is_err());
}
