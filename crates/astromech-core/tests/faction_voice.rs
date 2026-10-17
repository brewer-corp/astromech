//! Faction-state lookups by voice, end to end: "nearest famine system" → grammar → the app's
//! Nearby search (`Galaxy::nearby`) with the lookup's remembered filters, on Spansh's recorded
//! famine search from Sol (astromech-galaxy's fixture) → spoken answer. Nothing is pressed and
//! no test touches the network.

use astromech_core::actions::{Outcome, Runner};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health::BindsState;
use astromech_core::hub::StateHub;
use astromech_core::speech::RecordingSpeaker;
use astromech_core::tester::{Source, Would};
use astromech_data::{Cache, Client, Clock, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Galaxy};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_policy::{Config as PolicyConfig, Policy};
use astromech_proto::Origin;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

/// Spansh's systems search answers with the recorded famine search from Sol (14 Oct 2026);
/// EDSM knows no stars. Keeps the Spansh request bodies.
#[derive(Default)]
struct Recorded {
    bodies: Mutex<Vec<serde_json::Value>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let body = if url.ends_with("/api/systems/search") {
            if let astromech_data::Body::Json(b) = &req.body {
                self.bodies.lock().unwrap().push(b.clone());
            }
            include_str!("../../astromech-galaxy/tests/fixtures/spansh-famine.json")
        } else if url.contains("edsm.net") {
            "[]"
        } else {
            panic!("no recorded answer for {url}")
        };
        Ok(Response { status: 200, body: body.into() })
    }
}

/// Two days after the search was recorded.
struct Later;

impl Clock for Later {
    fn now_s(&self) -> i64 {
        chrono::DateTime::parse_from_rfc3339("2026-10-16T21:00:00Z").unwrap().timestamp()
    }
}

struct NoKeys;

impl Platform for NoKeys {
    fn foreground_title(&self) -> Option<String> {
        Some(GAME_WINDOW_TITLE.into())
    }
    fn send_key(&self, k: ScanKey, _up: bool) -> Result<(), String> {
        panic!("a faction lookup pressed {k:?}")
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

/// In Sol (position known when `at_sol`), on recorded answers with no rate limit.
fn rig(at_sol: bool) -> Rig {
    rig_with_journals(at_sol, None)
}

/// [`rig`] with the voice reading the commander's journals in `journals`.
fn rig_with_journals(at_sol: bool, journals: Option<&std::path::Path>) -> Rig {
    let rec = Arc::new(Recorded::default());
    let client = Arc::new(Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()));
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
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(Arc::new(NoKeys), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        galaxy: Some(Arc::new(Galaxy::with_remote(astromech_galaxy::Store::in_memory().unwrap(), client.clone()))),
        remote: client,
    });
    let store = Arc::new(Mutex::new(astromech_store::Store::in_memory().unwrap()));
    let mut assistant = Assistant::new(runner).with_store(store.clone()).with_clock(Arc::new(Later));
    if let Some(dir) = journals {
        assistant = assistant.with_journals(dir.to_path_buf());
    }
    Rig { assistant, speaker, rec, store }
}

fn looked_up(h: Handled) -> (&'static str, String) {
    match h {
        Handled::Ran { command, outcome: Outcome::LookingUp(h) } => (command, h.join().unwrap()),
        h => panic!("expected a lookup, got {h:?}"),
    }
}

const FAMINE: &str = "Nearest famine system: WISE 0855-0714. 7.2 light years, about 1 jump. Reported 2 days ago.";

#[test]
fn nearest_famine_system_is_spoken_with_the_remembered_filters() {
    let r = rig(true);
    // The commander set the Famine lookup's data age in the app.
    let f = Filters { max_age_days: Some(3650), ..Filters::default() };
    astromech_galaxy::filters::save(&r.store.lock().unwrap(), "faction.famine", &f).unwrap();

    let (command, said) = looked_up(r.assistant.handle("Where is there a famine near me?", Origin::Voice));
    assert_eq!((command, said.as_str()), ("faction.famine", FAMINE));
    assert_eq!(r.speaker.take(), vec!["Looking that up.".to_string(), FAMINE.to_string()]);

    let bodies = r.rec.bodies.lock().unwrap();
    assert_eq!(bodies.len(), 1, "one Spansh search");
    let b = &bodies[0];
    assert_eq!(b["filters"]["minor_faction_presences"][0]["state"]["value"][0], "Famine");
    assert_eq!(b["reference_coords"]["x"], 0.0, "searched from the journal's position");
    assert!(b["filters"]["updated_at"].is_object(), "the remembered data age goes to Spansh: {b}");
}

#[test]
fn a_war_near_me_ranks_by_population_and_finds_ross_154() {
    let r = rig(true);
    let (command, said) = looked_up(r.assistant.handle("nearest war system", Origin::Voice));
    assert_eq!(command, "faction.military");
    // Ross 154's Independents are in Civil War; the recorded page has no other war.
    assert_eq!(said, "Best war system: Ross 154. 9.7 light years, about 1 jump. Reported 2 days ago.");
    let bodies = r.rec.bodies.lock().unwrap();
    assert_eq!(bodies.len(), 2, "the nearest matches, then the nearest over a billion people");
    assert!(bodies[0]["filters"].get("updated_at").is_none(), "no data age set: {}", bodies[0]);
}

#[test]
fn without_a_position_it_says_so_and_asks_nothing() {
    let r = rig(false);
    match r.assistant.handle("nearest boom system", Origin::Voice) {
        Handled::Ran { command: "faction.proto", outcome: Outcome::Spoken(s) } => {
            assert_eq!(s, "Your position is not known yet. It comes from the next jump or login.");
            assert_eq!(r.speaker.take(), vec![s]);
        }
        h => panic!("{h:?}"),
    }
    assert!(r.rec.bodies.lock().unwrap().is_empty());
}

#[test]
fn the_tester_shows_the_lookup_without_running_it() {
    let r = rig(true);
    let d = r.assistant.dry_run("nearest outbreak system", Source::Typed);
    assert_eq!(d.would, Would::Speak);
    assert_eq!(d.intent.map(|i| i.command), Some("faction.pharma".to_string()));
    assert_eq!(d.message, "Would look up the best outbreak system on Spansh and say it.");
    assert!(r.rec.bodies.lock().unwrap().is_empty(), "a dry run asks Spansh nothing");
}

/// The app records the commander's own visits before a faction search (astromech-9mvu), so a
/// system visited since Spansh's report is said with the states seen on the visit.
#[test]
fn a_visit_newer_than_spanshs_report_is_said_as_seen_on_your_visit() {
    let journals = tempfile::tempdir().unwrap();
    // WISE 0855-0714, jumped to on 16 Oct, after Spansh's report of 14 Oct.
    let jump = r#"{"timestamp":"2026-10-16T20:00:00Z","event":"FSDJump","StarSystem":"WISE 0855-0714","SystemAddress":58144730663760,"StarPos":[6.5,-2.2,2.0],"JumpDist":7.2,"Factions":[{"Name":"WISE 0855-0714 Gold Gang","Allegiance":"Independent","FactionState":"Famine","Influence":0.4,"ActiveStates":[{"State":"Famine"}]}]}"#;
    std::fs::write(journals.path().join("Journal.2026-10-16T190000.01.log"), format!("{jump}\n")).unwrap();
    let r = rig_with_journals(true, Some(journals.path()));

    let (_, said) = looked_up(r.assistant.handle("nearest famine system", Origin::Voice));
    assert_eq!(said, "Nearest famine system: WISE 0855-0714. 7.2 light years, about 1 jump. Seen on your visit today.");

    // Without journals the answer is Spansh's, as before.
    let r = rig(true);
    let (_, said) = looked_up(r.assistant.handle("nearest famine system", Origin::Voice));
    assert_eq!(said, FAMINE);
}
