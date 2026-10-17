//! Bodies known to EDSM in the arrival briefing, replayed through the reaction engine with the
//! core's real lookup ([`EdsmBodies`]) over a recorded EDSM answer (astromech-data's
//! `edsm-bodies.json`: Alpha Centauri, 2 of 9 bodies). No test here touches the network.

use astromech_announce::{builtin, Engine, Settings, Shared};
use astromech_core::known_bodies::EdsmBodies;
use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_gamestate::GameState;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const RECORDED: &str = include_str!("../../astromech-data/tests/fixtures/edsm-bodies.json");

/// EDSM as recorded, offline, or slow (answers after a delay); counts requests.
enum Edsm {
    Recorded,
    Offline,
    Slow(Duration),
}

struct Fake(Edsm, AtomicUsize);

impl Transport for Fake {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        assert!(url.ends_with("/api-system-v1/bodies"), "{url}");
        self.1.fetch_add(1, Ordering::SeqCst);
        match &self.0 {
            Edsm::Recorded => Ok(Response { status: 200, body: RECORDED.into() }),
            Edsm::Offline => Err(TransportError::Offline("no route to host".into())),
            Edsm::Slow(d) => {
                std::thread::sleep(*d);
                Ok(Response { status: 200, body: RECORDED.into() })
            }
        }
    }
}

struct Rig {
    engine: Engine,
    state: GameState,
    edsm: Arc<Fake>,
    log: Arc<Mutex<Vec<String>>>,
}

fn rig(edsm: Edsm) -> Rig {
    let fake = Arc::new(Fake(edsm, AtomicUsize::new(0)));
    let client = Client::with_transport(Cache::in_memory().unwrap(), fake.clone()).limiter(Limiter::none());
    let log = Arc::new(Mutex::new(vec![]));
    let l = log.clone();
    let shared = Shared::default();
    shared.known_bodies.set(Arc::new(EdsmBodies::new(
        Arc::new(client),
        Arc::new(move |s: &str| l.lock().unwrap().push(s.to_string())),
    )));
    Rig { engine: Engine::new(builtin(shared), &Settings::default()), state: GameState::default(), edsm: fake, log }
}

const T0: i64 = 1_791_000_000; // 2026-10-03T03:20:00Z

fn ts(secs: i64) -> String {
    chrono::DateTime::from_timestamp(T0 + secs, 0).unwrap().to_rfc3339()
}

impl Rig {
    /// The bodies line(s) said for a jump at `secs` after T0, and how long the call took.
    fn jump(&mut self, secs: i64, system: &str) -> (Vec<String>, Duration) {
        let l = astromech_journal::parse_line(&format!(
            r#"{{"timestamp":"{}","event":"FSDJump","StarSystem":"{system}","SystemAddress":1178708478315,"StarPos":[3.03,-0.09,3.16],"JumpDist":4.4}}"#,
            ts(secs)
        ))
        .unwrap();
        self.state.apply_line(&l);
        let t = Instant::now();
        let r = self.engine.journal(&l, &self.state, Instant::now());
        (said(r), t.elapsed())
    }

    fn tick(&mut self, secs: i64) -> Vec<String> {
        said(self.engine.tick(T0 + secs, &self.state, Instant::now()))
    }

    /// Ticks once a (simulated) second from `from` to `to` after T0, waiting a little real time
    /// between them for the background lookup; everything said.
    fn ticks(&mut self, from: i64, to: i64) -> Vec<String> {
        let mut out = vec![];
        for s in from..=to {
            std::thread::sleep(Duration::from_millis(20));
            out.extend(self.tick(s));
        }
        out
    }
}

fn said(r: Vec<astromech_announce::Reaction>) -> Vec<String> {
    r.into_iter().filter(|r| r.rule == "known_bodies" && r.channels.speech).map(|r| r.text).collect()
}

#[test]
fn a_live_jump_gets_the_bodies_known_line() {
    let mut r = rig(Edsm::Recorded);
    r.engine.set_live(true);
    let (now, _) = r.jump(0, "Alpha Centauri");
    // The first ask starts the lookup; the answer comes on a tick.
    assert!(now.is_empty(), "the pipeline thread does not wait for EDSM");
    assert_eq!(r.ticks(1, 59), ["2 of 9 bodies known."], "said within the minute");
    assert_eq!(r.edsm.1.load(Ordering::SeqCst), 1);
    // Back again later: answered at once from what was found, said once per arrival.
    r.jump(100, "Sol");
    let (again, _) = r.jump(200, "Alpha Centauri");
    assert_eq!(again, ["2 of 9 bodies known."]);
    assert!(r.ticks(201, 210).is_empty());
}

#[test]
fn the_catch_up_says_nothing_and_looks_nothing_up() {
    let mut r = rig(Edsm::Recorded);
    let (now, _) = r.jump(0, "Alpha Centauri");
    assert!(now.is_empty());
    assert!(r.ticks(1, 10).is_empty());
    assert_eq!(r.edsm.1.load(Ordering::SeqCst), 0, "no lookup for a replayed jump");
    // Going live afterwards doesn't say the replayed arrival either.
    r.engine.set_live(true);
    assert!(r.ticks(11, 20).is_empty());
}

#[test]
fn offline_edsm_says_nothing_and_logs_why() {
    let mut r = rig(Edsm::Offline);
    r.engine.set_live(true);
    let (now, took) = r.jump(0, "Alpha Centauri");
    assert!(now.is_empty());
    assert!(took < Duration::from_millis(200), "no stall: {took:?}");
    assert!(r.ticks(1, 70).is_empty(), "nothing said");
    let log = r.log.lock().unwrap().clone();
    assert!(log.iter().any(|l| l.starts_with("bodies known to EDSM, Alpha Centauri: Cannot reach EDSM.")), "{log:?}");
}

#[test]
fn slow_edsm_doesnt_stall_and_is_dropped_after_a_minute() {
    let mut r = rig(Edsm::Slow(Duration::from_millis(600)));
    r.engine.set_live(true);
    let (now, took) = r.jump(0, "Alpha Centauri");
    assert!(now.is_empty());
    assert!(took < Duration::from_millis(200), "the jump didn't wait for EDSM: {took:?}");
    // The answer is still out when the minute after the arrival is over: nothing said, then or
    // when it lands.
    let t = Instant::now();
    assert!(r.tick(61).is_empty());
    assert!(t.elapsed() < Duration::from_millis(200), "the tick didn't wait either");
    std::thread::sleep(Duration::from_millis(800));
    assert!(r.ticks(62, 70).is_empty());
}
