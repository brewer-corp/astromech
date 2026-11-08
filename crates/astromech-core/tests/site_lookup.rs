//! "Nearest crystalline shards" by voice: the transcript resolves to the site lookup, which is
//! answered from the commander's position (here Sol, on Spansh's recorded answer) and spoken.
//! Nothing is pressed.
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

/// Spansh's crystalline shards search from Sol (recorded 16 Oct 2026), and its autocomplete
/// for any name: Sol; EDSM knows no stars.
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
        } else if url.ends_with("/api/systems/field_values/system_names") {
            r#"{"min_max":[{"id64":10477373803,"name":"Sol","x":0.0,"y":0.0,"z":0.0}],"values":["Sol"]}"#
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
    let (a, speaker, exec, _) = rig_opening(at_sol, transport, 0);
    (a, speaker, exec)
}

/// The threads the star map's opens ran on.
type OpenedOn = std::sync::mpsc::Receiver<std::thread::ThreadId>;

/// [`rig_with`] whose star map is opened on the first lookup, and fails to open `failures`
/// times first; each try records the thread it ran on.
fn rig_opening(
    at_sol: bool,
    transport: Recorded,
    failures: usize,
) -> (Assistant, Arc<RecordingSpeaker>, Arc<NoKeys>, OpenedOn) {
    let (opening, opened_on) = std::sync::mpsc::channel();
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
    let client =
        Arc::new(Client::with_transport(Cache::in_memory().unwrap(), Arc::new(transport)).limiter(Limiter::none()));
    let exec = Arc::new(NoKeys::default());
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(exec.clone(), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        stars: Arc::new(astromech_api::GalaxyHolder::new({
            let (client, left, on) = (client.clone(), Mutex::new(failures), opening);
            move || {
                let _ = on.send(std::thread::current().id());
                let mut left = left.lock().unwrap();
                if *left > 0 {
                    *left -= 1;
                    anyhow::bail!("galaxy.db is locked");
                }
                Ok(Galaxy::with_remote(Store::in_memory()?, client.clone()))
            }
        })),
        remote: client,
    });
    (Assistant::new(runner), speaker, exec, opened_on)
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
    assert_eq!(s, astromech_tools::lookup::NO_POSITION);
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

/// "... near <system>" searches from that system (astromech-j75v), even before the journal gives
/// a position, and says which; a name Spansh does not know is said plainly.
#[test]
fn a_named_system_is_searched_from_and_named() {
    let (a, speaker, _) = rig(false);
    let said = |t: &str| match a.handle(t, astromech_proto::Origin::Voice) {
        Handled::Ran { outcome: Outcome::LookingUp(h), .. } => h.join().unwrap(),
        other => panic!("{other:?}"),
    };
    // (jumps at 20 light years: no ship range from the journal yet)
    let s = said("Nearest crystalline shards near Sol.");
    assert!(
        s.starts_with("From Sol. Nearest crystalline shards: Oochorrs SB-J d10-5 B 6 a, 1,426 light years, "),
        "{s}"
    );
    assert_eq!(
        said("nearest crystalline shards from Nowhere Prime"),
        "Nowhere Prime. Spansh does not know that system."
    );
    speaker.take();
    // without a named system, the position is still needed
    match a.handle("nearest crystalline shards", astromech_proto::Origin::Voice) {
        Handled::Ran { outcome: Outcome::Spoken(s), .. } => assert_eq!(s, astromech_tools::lookup::NO_POSITION),
        other => panic!("{other:?}"),
    }
    let d = a.dry_run("nearest crystalline shards near Sol", Source::Typed);
    assert_eq!(d.message, "Would look up the nearest crystalline shards and say where it is. Searches from Sol.");
    // Run live searches from the system the tester names, too (astromech-d50l).
    assert_eq!((d.live.as_deref(), d.live_from.as_deref()), (Some("sites.crystalline_shards"), Some("Sol")));
    let m = astromech_core::tester::live(&a.runner, "sites.crystalline_shards", d.live_from.as_deref(), Duration::ZERO)
        .unwrap();
    assert!(m.starts_with("From Sol. Nearest crystalline shards: "), "{m}");
    assert_eq!(a.dry_run("nearest crystalline shards", Source::Typed).live_from, None);
}

/// What a voice command said, when it said one line and started no lookup.
fn spoken_alone(a: &Assistant, speaker: &RecordingSpeaker, text: &str) -> String {
    let Handled::Ran { outcome: Outcome::Spoken(s), .. } = a.handle(text, astromech_proto::Origin::Voice) else {
        panic!("{text:?}: not spoken")
    };
    assert_eq!(speaker.take(), vec![s.clone()], "{text:?}: the line alone");
    s
}

/// While the session's first open of the star map still runs, a voice lookup says "Looking
/// that up." and waits for it on the lookup's own thread, so the speech thread is free.
#[test]
fn a_lookup_while_the_star_map_opens_waits_for_it_off_the_speech_thread() {
    let (a, speaker, _, opened_on) = rig_opening(true, Recorded::default(), 0);
    let Handled::Ran { outcome: Outcome::LookingUp(h), .. } =
        a.handle("nearest crystalline shards", astromech_proto::Origin::Voice)
    else {
        panic!("not a lookup")
    };
    assert_eq!(h.join().unwrap(), ANSWER);
    assert_eq!(speaker.take(), vec!["Looking that up.".to_string(), ANSWER.to_string()]);
    assert_ne!(opened_on.recv().unwrap(), std::thread::current().id(), "not opened on the speech thread");
}

/// A star map whose last open failed: the lookup says lookups are off, and nothing else, as
/// before the star map moved off the speech thread. It is opened again off the speech thread,
/// and once open the next lookup answers.
#[test]
fn a_star_map_that_would_not_open_is_tried_again_on_the_next_lookup() {
    let (a, speaker, _, opened_on) = rig_opening(true, Recorded::default(), 1);
    assert!(a.runner.stars.get().is_err(), "the session's open failed");
    let s = spoken_alone(&a, &speaker, "nearest crystalline shards");
    assert_eq!(s, astromech_tools::lookup::NO_STAR_MAP);
    let here = std::thread::current().id();
    assert_eq!(opened_on.recv(), Ok(here), "the session's open (the test's, here)");
    // The lookup tried the open again, off the speech thread: wait for that try to start.
    assert_ne!(opened_on.recv().unwrap(), here, "tried again off the speech thread");
    // Once that open is done the star map is open: this waits for it and opens nothing.
    a.runner.stars.get().unwrap();
    assert_eq!(opened_on.try_recv(), Err(std::sync::mpsc::TryRecvError::Empty), "opened once more, not twice");
    let Handled::Ran { outcome: Outcome::LookingUp(h), .. } =
        a.handle("nearest crystalline shards", astromech_proto::Origin::Voice)
    else {
        panic!("not a lookup")
    };
    assert_eq!(h.join().unwrap(), ANSWER);
}

/// With the star map off, each lookup's checks come in their old order: the sites lookup asks
/// for a position first; the faction and hotspot lookups say the star map is off before asking
/// for a position; the Powerplay targets say "not pledged", then the star map.
#[test]
fn with_the_star_map_off_the_lookups_check_in_their_old_order() {
    use astromech_tools::lookup::{NO_POSITION, NO_STAR_MAP};
    let (a, speaker, _, _) = rig_opening(false, Recorded::default(), usize::MAX);
    assert!(a.runner.stars.get().is_err());
    assert_eq!(spoken_alone(&a, &speaker, "nearest crystalline shards"), NO_POSITION);
    assert_eq!(spoken_alone(&a, &speaker, "nearest famine system"), NO_STAR_MAP);
    assert_eq!(spoken_alone(&a, &speaker, "nearest painite hotspot"), NO_STAR_MAP);
    let undermine = "where can I undermine near me";
    assert_eq!(spoken_alone(&a, &speaker, undermine), astromech_tools::targets::NOT_PLEDGED);
    a.runner.hub.update(|g| g.powerplay.power = Some("Zachary Hudson".into()));
    assert_eq!(spoken_alone(&a, &speaker, undermine), NO_STAR_MAP);
}
