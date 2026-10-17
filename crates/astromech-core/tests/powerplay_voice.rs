//! Powerplay targets by voice, end to end: "where can I undermine near me" → grammar → the
//! Where to work search (`Galaxy::powerplay_targets`) for the power pledged to in the journal,
//! with the search's remembered filters, on Spansh's recorded answers around Lembava for
//! Li Yong-Rui (astromech-galaxy's fixtures, 16 Oct 2026) → spoken answer. Nothing is pressed
//! and no test touches the network.

use astromech_core::actions::{Outcome, Runner};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health::BindsState;
use astromech_core::hub::StateHub;
use astromech_core::speech::RecordingSpeaker;
use astromech_core::tester::{Source, Would};
use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Galaxy, Sphere};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_policy::{Config as PolicyConfig, Policy};
use astromech_proto::Origin;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

const LEMBAVA: [f64; 3] = [-43.25, -64.34375, -77.6875];

/// Spansh's systems search answers with the recording for the kind of search asked; keeps the
/// request bodies.
#[derive(Default)]
struct Recorded {
    bodies: Mutex<Vec<serde_json::Value>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        assert_eq!(url, "https://spansh.co.uk/api/systems/search", "only Spansh is asked");
        let Body::Json(b) = &req.body else { panic!("{req:?}") };
        self.bodies.lock().unwrap().push(b.clone());
        let f = &b["filters"];
        let body = if f.get("controlling_power").is_some() && f.get("power_state").is_some() {
            include_str!("../../astromech-galaxy/tests/fixtures/spansh-powerplay-undermining.json")
        } else if f.get("controlling_power").is_some() {
            include_str!("../../astromech-galaxy/tests/fixtures/spansh-powerplay-reinforcement.json")
        } else {
            include_str!("../../astromech-galaxy/tests/fixtures/spansh-powerplay-acquisition.json")
        };
        Ok(Response { status: 200, body: body.into() })
    }
}

struct NoKeys;

impl Platform for NoKeys {
    fn foreground_title(&self) -> Option<String> {
        Some(GAME_WINDOW_TITLE.into())
    }
    fn send_key(&self, k: ScanKey, _up: bool) -> Result<(), String> {
        panic!("a Powerplay lookup pressed {k:?}")
    }
    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
    }
}

struct Rig {
    assistant: Assistant,
    speaker: Arc<RecordingSpeaker>,
    rec: Arc<Recorded>,
    store: Arc<Mutex<astromech_store::Store>>,
}

/// In Lembava, pledged to `power` per the journal, on recorded answers with no rate limit.
fn rig(power: Option<&str>) -> Rig {
    let rec = Arc::new(Recorded::default());
    let client = Arc::new(Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()));
    let stars = astromech_galaxy::Store::in_memory().unwrap();
    // Covered space, so no EDSM fill-in is asked for.
    stars.add_coverage(Sphere { center: LEMBAVA, radius: 100.0 }, "test", "t").unwrap();
    let hub = Arc::new(StateHub::new());
    let power = power.map(str::to_string);
    hub.update(|g| {
        g.status_seen = true;
        g.status_flags = astromech_status::FLAG_IN_MAIN_SHIP;
        g.system = Some("Lembava".into());
        g.star_pos = Some(LEMBAVA);
        g.max_jump_range = Some(30.0);
        g.powerplay.power = power;
    });
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(Arc::new(NoKeys), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        galaxy: Some(Arc::new(Galaxy::with_remote(stars, client.clone()))),
        remote: client,
    });
    let store = Arc::new(Mutex::new(astromech_store::Store::in_memory().unwrap()));
    let assistant = Assistant::new(runner).with_store(store.clone());
    Rig { assistant, speaker, rec, store }
}

fn looked_up(h: Handled) -> (&'static str, String) {
    match h {
        Handled::Ran { command, outcome: Outcome::LookingUp(h) } => (command, h.join().unwrap()),
        h => panic!("expected a lookup, got {h:?}"),
    }
}

#[test]
fn where_can_i_undermine_uses_the_pledged_power_and_the_remembered_filters() {
    let r = rig(Some("Li Yong-Rui"));
    // The commander set the Undermine search's data age in the app.
    let f = Filters { max_age_days: Some(3650), ..Filters::default() };
    astromech_galaxy::filters::save(&r.store.lock().unwrap(), "powerplay.undermine", &f).unwrap();

    let (command, said) = looked_up(r.assistant.handle("Where can I undermine near me?", Origin::Voice));
    assert_eq!(command, "powerplay.undermine");
    assert_eq!(said, "Nearest system to undermine: Piscium Sector EB-X b1-3. 22 light years, 1 jump. Pranav Antal exploited, 8 percent.");
    assert_eq!(r.speaker.take(), vec!["Looking that up.".to_string(), said]);

    let bodies = r.rec.bodies.lock().unwrap();
    assert_eq!(bodies.len(), 1, "one Spansh search");
    let b = &bodies[0];
    let others = b["filters"]["controlling_power"]["value"].as_array().unwrap();
    assert!(!others.iter().any(|p| p == "Li Yong-Rui"), "the pledged power is not undermined: {b}");
    assert!(b["filters"]["updated_at"].is_object(), "the remembered data age goes to Spansh: {b}");
}

#[test]
fn reinforce_and_acquire_answer_with_the_state() {
    let r = rig(Some("Li Yong-Rui"));
    let (command, said) = looked_up(r.assistant.handle("nearest system to reinforce", Origin::Voice));
    assert_eq!(
        (command, said.as_str()),
        ("powerplay.reinforce", "Nearest system to reinforce: Lembava. In this system. Stronghold, 25 percent.")
    );
    let (command, said) = looked_up(r.assistant.handle("where can I acquire near me", Origin::Voice));
    assert_eq!((command, said.as_str()), ("powerplay.acquire", "Nearest system to acquire: Piscium Sector YJ-A b2. 10 light years, 1 jump. Unoccupied, acquisition 32 percent."));
    let bodies = r.rec.bodies.lock().unwrap();
    assert_eq!(bodies[1]["filters"]["power"]["value"][0], "Li Yong-Rui", "acquisition for the pledged power");
    assert!(bodies[0].get("filters").unwrap().get("updated_at").is_none(), "no data age set");
}

#[test]
fn an_unpledged_commander_hears_that_plainly() {
    let r = rig(None);
    match r.assistant.handle("where can I undermine near me", Origin::Voice) {
        Handled::Ran { command: "powerplay.undermine", outcome: Outcome::Spoken(s) } => {
            assert_eq!(s, "Not pledged to a power, so there are no targets.");
            assert_eq!(r.speaker.take(), vec![s]);
        }
        h => panic!("{h:?}"),
    }
    assert!(r.rec.bodies.lock().unwrap().is_empty(), "Spansh is not asked");
}

#[test]
fn the_tester_shows_the_search_without_running_it() {
    let r = rig(Some("Li Yong-Rui"));
    let d = r.assistant.dry_run("nearest system to undermine", Source::Typed);
    assert_eq!(d.would, Would::Speak);
    assert_eq!(d.intent.map(|i| i.command), Some("powerplay.undermine".to_string()));
    assert_eq!(d.message, "Would look up the nearest system to undermine for your power on Spansh and say it.");
    assert!(r.rec.bodies.lock().unwrap().is_empty(), "a dry run asks Spansh nothing");
}
