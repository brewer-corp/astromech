//! "Nearest painite hotspot" by voice (astromech-f7i0): the transcript resolves to the mining
//! hotspot lookup with the ore in its `{mineral}` slot, answered from the commander's position
//! (here Sol, on Spansh's recorded Painite rings) and spoken: the nearest ring, its distance,
//! jumps, hotspots and reserves. Nothing is pressed.
#![allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)

use astromech_core::actions::{Outcome, Runner};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health::BindsState;
use astromech_core::hub::StateHub;
use astromech_core::speech::RecordingSpeaker;
use astromech_core::tester::{Source, Would};
use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Galaxy, Store};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_policy::{Config as PolicyConfig, Policy};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

#[derive(Default)]
struct NoKeys {
    sent: Mutex<Vec<ScanKey>>,
}

impl Platform for NoKeys {
    fn foreground_title(&self) -> Option<String> {
        Some(GAME_WINDOW_TITLE.into())
    }
    fn send_key(&self, k: ScanKey, _up: bool) -> Result<(), String> {
        self.sent.lock().unwrap().push(k);
        Ok(())
    }
    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
    }
}

/// Spansh's Painite rings from Sol (recorded 5 Nov 2026), and its autocomplete for any name:
/// Wolf 1481; EDSM knows no stars. Keeps each bodies search asked.
#[derive(Default)]
struct Recorded {
    asked: Mutex<Vec<String>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let body = if url.ends_with("/api/bodies/search") {
            self.asked.lock().unwrap().push(format!("{:?}", req.body));
            let all: serde_json::Value = serde_json::from_str(include_str!(
                "../../astromech-market/tests/fixtures/spansh-hotspots-painite-sol.json"
            ))
            .unwrap();
            all["any"].to_string()
        } else if url.ends_with("/api/systems/field_values/system_names") {
            r#"{"min_max":[{"id64":2869977753001,"name":"Wolf 1481","x":5.1875,"y":13.375,"z":13.5625}],"values":["Wolf 1481"]}"#.into()
        } else {
            "[]".into()
        };
        Ok(Response { status: 200, body })
    }
}

fn rig(at_sol: bool) -> (Assistant, Arc<RecordingSpeaker>, Arc<NoKeys>, Arc<Recorded>) {
    let rec = Arc::new(Recorded::default());
    let (a, s, e) = rig_with(at_sol, rec.clone());
    (a, s, e, rec)
}

fn rig_with(at_sol: bool, transport: Arc<Recorded>) -> (Assistant, Arc<RecordingSpeaker>, Arc<NoKeys>) {
    let hub = Arc::new(StateHub::new());
    hub.update(|g| {
        g.status_seen = true;
        g.status_flags = astromech_status::FLAG_IN_MAIN_SHIP;
        if at_sol {
            g.system = Some("Sol".into());
            g.star_pos = Some([0.0; 3]);
            g.max_jump_range = Some(30.0);
        }
    });
    let client = Arc::new(Client::with_transport(Cache::in_memory().unwrap(), transport).limiter(Limiter::none()));
    let exec = Arc::new(NoKeys::default());
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(exec.clone(), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        stars: Arc::new(astromech_api::GalaxyHolder::opened(Galaxy::with_remote(
            Store::in_memory().unwrap(),
            client.clone(),
        ))),
        remote: client,
    });
    (Assistant::new(runner), speaker, exec)
}

const ANSWER: &str =
    "Nearest Painite hotspot. Barnard's Star 5 A Ring. 6.0 light years, about 1 jump. 2 hotspots, depleted reserves.";

#[test]
fn nearest_painite_hotspot_is_spoken_from_the_commanders_position() {
    let (a, speaker, exec, rec) = rig(true);
    match a.handle("Nearest painite hotspot.", astromech_proto::Origin::Voice) {
        Handled::Ran { command, outcome: Outcome::LookingUp(h) } => {
            assert_eq!(command, "mining.hotspots");
            assert_eq!(h.join().unwrap(), ANSWER);
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(speaker.take(), vec!["Looking that up.".to_string(), ANSWER.to_string()]);
    assert!(exec.sent.lock().unwrap().is_empty(), "a lookup presses nothing");
    let asked = rec.asked.lock().unwrap().clone();
    assert_eq!(asked.len(), 1, "one Spansh request");
    assert!(asked[0].contains("String(\"Painite\")") && asked[0].contains("reference_coords"), "{}", asked[0]);
    // the finder's radius, ring type and reserve don't apply to "nearest"
    assert!(!asked[0].contains("reserve_level") && !asked[0].contains("\"max\""), "{}", asked[0]);

    // the tester shows what it would do without asking Spansh
    let d = a.dry_run("nearest painite hotspot", Source::Typed);
    assert_eq!(d.would, Would::Speak);
    assert_eq!(d.message, "Would look up the nearest Painite hotspot on Spansh and say it.");
}

#[test]
fn a_named_system_is_searched_from_and_named() {
    let (a, speaker, _, rec) = rig(false);
    let said = match a.handle("nearest painite hotspot near Wolf 1481", astromech_proto::Origin::Voice) {
        Handled::Ran { outcome: Outcome::LookingUp(h), .. } => h.join().unwrap(),
        other => panic!("{other:?}"),
    };
    // Wolf 1481's own ring, at 0 ly: the system is named, not "this system"
    assert_eq!(said, "From Wolf 1481. Nearest Painite hotspot. Wolf 1481 4 A Ring. 2 hotspots, low reserves.");
    assert!(rec.asked.lock().unwrap()[0].contains("5.1875"), "searched from Wolf 1481's coordinates");
    speaker.take();
    let d = a.dry_run("nearest painite hotspot near Wolf 1481", Source::Typed);
    assert_eq!(d.message, "Would look up the nearest Painite hotspot on Spansh and say it. Searches from Wolf 1481.");
}

#[test]
fn without_a_position_or_a_mineral_it_says_so() {
    let (a, speaker, _, rec) = rig(false);
    let Handled::Ran { outcome: Outcome::Spoken(s), .. } =
        a.handle("nearest painite hotspot", astromech_proto::Origin::Voice)
    else {
        panic!("not spoken")
    };
    assert_eq!(s, astromech_tools::lookup::NO_POSITION);
    assert_eq!(speaker.take(), vec![s]);
    assert!(rec.asked.lock().unwrap().is_empty());
}
