//! Exobiology in the core (feature astromech-znhx): biology other players reported, from Spansh,
//! for the arrival briefing's "Known biology on 3 bodies." ([`SpanshBiology`]), and the
//! Biology page's view of the system (the `exobiology` control command, [`view`]).
//!
//! The reaction engine asks on the pipeline thread and must never block, so a system's lookup
//! runs on its own thread: the first ask starts it and answers None, later asks answer from
//! what it found (`biology` says a late answer on the clock tick). One Spansh bodies search per
//! system (`system_id64` plus `signals: Biological`), cached for the session by the client.

use crate::logging::log;
use astromech_announce::biology::KnownBiology;
use astromech_data::spansh::{self, Reference, Search};
use astromech_data::Client;
use astromech_exobiology::known::{self, KnownBody};
use astromech_gamestate::GameState;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// At most this many systems' answers are kept.
const KEEP: usize = 64;

/// The known bodies with biology from a Spansh bodies search.
pub fn known_bodies(bodies: &[spansh::Body]) -> Vec<KnownBody> {
    bodies
        .iter()
        .map(|b| {
            let signals = b.signals.iter().filter(|s| s.name.eq_ignore_ascii_case("Biological")).map(|s| s.count).sum();
            KnownBody::new(
                &b.name,
                b.body_id,
                signals,
                b.genuses.iter().map(|g| g.name.as_str()),
                b.landmarks.iter().filter(|l| l.value.is_some()).map(|l| l.subtype.as_str()),
            )
        })
        .filter(|k| k.signals > 0 || !k.genera.is_empty())
        .collect()
}

/// Ask Spansh for the bodies with biology in `system_address`.
pub fn look_up(
    client: &Client,
    system_address: i64,
    star_pos: Option<[f64; 3]>,
) -> astromech_data::Result<Vec<KnownBody>> {
    let q = Search::near(Reference::Coords(star_pos.unwrap_or_default()))
        .in_systems(&[system_address])
        .signal("Biological")
        .size(50);
    let page = client.spansh_bodies(&q)?.value;
    let here: Vec<spansh::Body> = page.results.into_iter().filter(|b| b.system_id64 == system_address).collect();
    Ok(known_bodies(&here))
}

enum Lookup {
    Pending,
    Done(Vec<KnownBody>),
}

/// Spansh lookups for [`KnownBiology`], one background thread per system.
pub struct SpanshBiology {
    client: Arc<Client>,
    systems: Arc<Mutex<HashMap<i64, Lookup>>>,
}

impl SpanshBiology {
    pub fn new(client: Client) -> Self {
        Self { client: Arc::new(client), systems: Arc::default() }
    }
}

impl KnownBiology for SpanshBiology {
    fn known(&self, system_address: i64, star_pos: Option<[f64; 3]>) -> Option<Vec<KnownBody>> {
        let mut systems = self.systems.lock().unwrap();
        match systems.get(&system_address) {
            Some(Lookup::Done(v)) => return Some(v.clone()),
            Some(Lookup::Pending) => return None,
            None => {}
        }
        if systems.len() >= KEEP {
            systems.retain(|_, l| matches!(l, Lookup::Pending));
        }
        systems.insert(system_address, Lookup::Pending);
        let (client, all) = (self.client.clone(), self.systems.clone());
        let spawned = std::thread::Builder::new().name("known-biology".into()).spawn(move || {
            let found = look_up(&client, system_address, star_pos).unwrap_or_else(|e| {
                // Nothing to say rather than a wrong line; the page shows the system's own results.
                log(&format!("known biology: {e}"));
                vec![]
            });
            all.lock().unwrap().insert(system_address, Lookup::Done(found));
        });
        if spawned.is_err() {
            systems.remove(&system_address);
        }
        None
    }
}

/// The Biology page's view: the system's bodies with biology from the journal, ranked by value,
/// what others reported (Spansh) when asked, and the sampling in progress.
pub fn view(game: &GameState, known: Option<&dyn KnownBiology>) -> Value {
    let ex = &game.exobiology;
    let sys = &game.system_bodies;
    let reported = sys.system_address().and_then(|a| known?.known(a, game.star_pos)).map(known::ranked);
    let sampling = ex.sampling.current.as_ref().map(|p| {
        json!({
            "genus": p.genus,
            "species": p.species,
            "count": p.count,
            "complete": p.complete,
            "colony_range_m": p.colony_range_m,
            "body_id": p.body_id,
            "clearance": ex.sampling.clearance(),
        })
    });
    json!({
        "ok": true,
        "system": sys.system(),
        "bodies": astromech_exploration::biology::ranked(sys),
        "known": reported.map(|k| k.iter().map(|b| json!({
            "name": b.name,
            "body_id": b.body_id,
            "signals": b.signals,
            "genera": b.genera,
            "species": b.species.iter().map(|(n, v)| json!({"name": n, "value": v})).collect::<Vec<_>>(),
            "value": b.value(),
        })).collect::<Vec<_>>()),
        "sampling": sampling,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_data::{Cache, Request, Response, Transport, TransportError};
    use std::time::{Duration, Instant};

    const RECORDED: &str = include_str!("../../astromech-data/tests/fixtures/spansh-bodies-biology.json");
    const LUHMAN: i64 = 22960358574928;

    struct Recorded;
    impl Transport for Recorded {
        fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
            assert!(url.ends_with("/api/bodies/search"), "{url}");
            Ok(Response { status: 200, body: RECORDED.into() })
        }
    }

    fn client() -> Client {
        Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Recorded))
    }

    #[test]
    fn a_recorded_answer_becomes_the_known_bodies() {
        let k = look_up(&client(), LUHMAN, Some([6.3125, 0.59375, 1.71875])).unwrap();
        assert_eq!(k.len(), 2);
        let b3 = k.iter().find(|b| b.name == "Luhman 16 B 3").unwrap();
        assert_eq!((b3.body_id, b3.signals), (Some(17), 4));
        assert_eq!(b3.genera, ["Bacterium", "Fonticulua", "Fungoida", "Osseus"]);
        assert_eq!(b3.best(), Some(("Fungoida Bullarum".to_string(), 3_703_200, true)));
        assert_eq!(b3.value(), (8_859_500, 8_859_500), "Spansh's landmark_value for the body");
        // Other systems' bodies in an answer are dropped.
        assert!(look_up(&client(), 1, None).unwrap().is_empty());
    }

    #[test]
    fn the_first_ask_starts_a_lookup_and_later_asks_answer() {
        let k = SpanshBiology::new(client());
        assert_eq!(k.known(LUHMAN, None), None);
        let deadline = Instant::now() + Duration::from_secs(10);
        let found = loop {
            if let Some(v) = k.known(LUHMAN, None) {
                break v;
            }
            assert!(Instant::now() < deadline, "the lookup never finished");
            std::thread::sleep(Duration::from_millis(5));
        };
        assert_eq!(found.len(), 2);
        assert_eq!(k.known(LUHMAN, None).map(|v| v.len()), Some(2), "answered from what it found");
    }

    #[test]
    fn the_view_lists_bodies_known_and_sampling() {
        let mut g = GameState::default();
        for l in [
            r#"{"timestamp":"2026-10-17T10:00:00Z","event":"FSDJump","StarSystem":"Luhman 16","SystemAddress":22960358574928,"StarPos":[6.3125,0.59375,1.71875],"JumpDist":8.2}"#,
            r#"{"timestamp":"2026-10-17T10:02:00Z","event":"FSSBodySignals","BodyName":"Luhman 16 B 4","BodyID":19,"SystemAddress":22960358574928,"Signals":[{"Type":"$SAA_SignalType_Biological;","Count":2}]}"#,
            r#"{"timestamp":"2026-10-17T10:20:00Z","event":"ScanOrganic","ScanType":"Log","Genus":"$Codex_Ent_Bacterial_Genus_Name;","Species":"$Codex_Ent_Bacterial_04_Name;","Species_Localised":"Bacterium Vesicula","SystemAddress":22960358574928,"Body":19}"#,
        ] {
            g.apply_line(&astromech_journal::parse_line(l).unwrap());
        }
        struct Ready;
        impl KnownBiology for Ready {
            fn known(&self, a: i64, _: Option<[f64; 3]>) -> Option<Vec<KnownBody>> {
                (a == LUHMAN).then(|| {
                    known_bodies(
                        &serde_json::from_value::<spansh::Page<spansh::Body>>(serde_json::from_str(RECORDED).unwrap())
                            .unwrap()
                            .results,
                    )
                })
            }
        }
        let v = view(&g, Some(&Ready));
        assert_eq!(v["system"], "Luhman 16");
        assert_eq!(v["bodies"][0]["name"], "Luhman 16 B 4");
        assert_eq!(v["bodies"][0]["signals"], 2);
        assert_eq!(v["known"][0]["name"], "Luhman 16 B 3", "most valuable first");
        assert_eq!(v["sampling"]["species"], "Bacterium Vesicula");
        assert_eq!(v["sampling"]["count"], 1);
        let none = view(&g, None);
        assert!(none["known"].is_null());
    }
}
