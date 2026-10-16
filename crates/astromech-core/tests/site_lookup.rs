//! "Nearest crystalline shards" by voice: the transcript resolves to the site lookup, which is
//! answered from the commander's position (here Sol, on Spansh's recorded answer) and spoken.
//! Nothing is pressed.

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

/// Spansh's crystalline shards search from Sol (recorded 16 Oct 2026); EDSM knows no stars.
/// With a gate, the Spansh answer waits until the test opens it (a slow Spansh).
#[derive(Default)]
struct Recorded {
    gate: Option<Mutex<std::sync::mpsc::Receiver<()>>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        if let (Some(g), true) = (&self.gate, url.ends_with("/api/bodies/search")) {
            let _ = g.lock().unwrap().recv_timeout(Duration::from_secs(20));
        }
        let body = if url.ends_with("/api/bodies/search") {
            include_str!("../../astromech-galaxy/tests/fixtures/spansh-sites-crystalline_shards.json")
        } else {
            "[]"
        };
        Ok(Response { status: 200, body: body.into() })
    }
}

fn rig(at_sol: bool) -> (Assistant, Arc<RecordingSpeaker>, Arc<NoKeys>) {
    rig_with(at_sol, Recorded::default())
}

fn rig_with(at_sol: bool, transport: Recorded) -> (Assistant, Arc<RecordingSpeaker>, Arc<NoKeys>) {
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
    let client = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(transport)).limiter(Limiter::none());
    let exec = Arc::new(NoKeys::default());
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(exec.clone(), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        galaxy: Some(Arc::new(Galaxy::with_remote(Store::in_memory().unwrap(), client))),
    });
    (Assistant::new(runner), speaker, exec)
}

const ANSWER: &str = "Nearest crystalline shards: Oochorrs SB-J d10-5 B 6 a, 1,426 light years, about 48 jumps, \
                      26,249 light seconds from the arrival star.";

#[test]
fn nearest_crystalline_shards_is_spoken_from_the_commanders_position() {
    let (a, speaker, exec) = rig(true);
    match a.handle("Nearest crystalline shards.", astromech_proto::Origin::Voice) {
        Handled::Ran { command, outcome: Outcome::LookingUp(h) } => {
            assert_eq!(command, "sites.crystalline_shards");
            assert_eq!(h.join().unwrap(), ANSWER);
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(speaker.take(), vec!["Looking that up.".to_string(), ANSWER.to_string()]);
    assert!(exec.sent.lock().unwrap().is_empty(), "a lookup presses nothing");

    // the tester shows what it would do without asking Spansh
    let d = a.dry_run("nearest crystalline shards", Source::Typed);
    assert_eq!(d.would, Would::Speak);
    assert_eq!(d.message, "Would look up the nearest crystalline shards and say where it is.");
}

#[test]
fn a_slow_lookup_does_not_hold_up_the_next_command() {
    let (open, gate) = std::sync::mpsc::channel();
    let (a, speaker, _) = rig_with(true, Recorded { gate: Some(Mutex::new(gate)) });
    let t0 = std::time::Instant::now();
    let Handled::Ran { outcome: Outcome::LookingUp(lookup), .. } =
        a.handle("nearest crystalline shards", astromech_proto::Origin::Voice)
    else {
        panic!("not a lookup")
    };
    // Spansh hasn't answered; the next command is handled straight away
    let next = a.handle("gear down", astromech_proto::Origin::Voice);
    assert!(matches!(next, Handled::Ran { command: "landing_gear.down", .. }), "{next:?}");
    assert!(t0.elapsed() < Duration::from_secs(2), "the gear command waited {:?}", t0.elapsed());
    assert!(!lookup.is_finished(), "the lookup is still waiting on Spansh");
    let said = speaker.take();
    assert_eq!(said[0], "Looking that up.");
    assert_eq!(said.len(), 2, "the gear answer, not the lookup's: {said:?}");
    // Spansh answers: the lookup speaks
    open.send(()).unwrap();
    assert_eq!(lookup.join().unwrap(), ANSWER);
    assert_eq!(speaker.take(), vec![ANSWER.to_string()]);
}

#[test]
fn without_a_position_it_says_so() {
    let (a, speaker, _) = rig(false);
    let Handled::Ran { outcome: Outcome::Spoken(s), .. } =
        a.handle("nearest brain trees", astromech_proto::Origin::Voice)
    else {
        panic!("not spoken")
    };
    assert!(s.starts_with("Your position isn't known yet"), "{s}");
    assert_eq!(speaker.take(), vec![s]);
}

#[test]
fn voice_site_kinds_are_the_galaxy_lookups() {
    let ids: Vec<&str> = astromech_galaxy::SITE_KINDS.iter().map(|k| k.id).collect();
    assert_eq!(ids, astromech_tools::SITE_KINDS);
    for k in astromech_tools::SITE_KINDS {
        assert!(astromech_intent::Grammar::intern(&format!("sites.{k}")).is_some(), "no phrases for {k}");
    }
}
