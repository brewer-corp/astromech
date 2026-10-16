//! End-to-end: fake game backend writes Status.json (after 0–1000 ms, the S5 range) when the
//! landing-gear key goes down; the real pipeline watches the folder with file notifications.

use astromech_core::actions::{Outcome, Runner, Verified};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health;
use astromech_core::hub::StateHub;
use astromech_core::speech::{FakeStt, RecordingSpeaker, Stt};
use astromech_input::exec::keys::{lookup, KeyLookup, ScanKey};
use astromech_input::exec::platform::{mock, GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_policy::{Config, Denial, Policy};
use astromech_proto::Origin;
use astromech_status::{FLAG_DOCKED, FLAG_GEAR, FLAG_IN_MAIN_SHIP, FLAG_SUPERCRUISE};
use astromech_tools::{Target, Toggle, ToolCall};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

const GEAR_DOWN: ToolCall = ToolCall::Toggle { toggle: Toggle::LandingGear, target: Target::On };

fn key(n: &str) -> ScanKey {
    match lookup(n) {
        KeyLookup::Ok(k) => k,
        _ => panic!("{n}"),
    }
}

/// Fake game: Key_L toggles the gear bit after a delay from `delays` (cycled).
struct FakeGame {
    status: PathBuf,
    flags: Arc<Mutex<u64>>,
    drop_presses: AtomicU32,
    presses: AtomicU32,
    delays: Vec<u64>,
    next: AtomicU64,
    log: Mutex<Vec<(ScanKey, bool)>>,
}

impl FakeGame {
    fn new(dir: &Path, flags: u64, delays: &[u64]) -> Arc<Self> {
        let status = dir.join("Status.json");
        mock::write_status(&status, flags, false);
        Arc::new(Self {
            status,
            flags: Arc::new(Mutex::new(flags)),
            drop_presses: AtomicU32::new(0),
            presses: AtomicU32::new(0),
            delays: delays.to_vec(),
            next: AtomicU64::new(0),
            log: Mutex::new(vec![]),
        })
    }
    fn set_flags(&self, f: u64) {
        *self.flags.lock().unwrap() = f;
        mock::write_status(&self.status, f, false);
    }
}

impl Platform for FakeGame {
    fn foreground_title(&self) -> Option<String> {
        Some(GAME_WINDOW_TITLE.into())
    }
    fn send_key(&self, k: ScanKey, up: bool) -> Result<(), String> {
        self.log.lock().unwrap().push((k, up));
        if k == key("Key_L") && !up {
            self.presses.fetch_add(1, Ordering::SeqCst);
            if self.drop_presses.load(Ordering::SeqCst) > 0 {
                self.drop_presses.fetch_sub(1, Ordering::SeqCst);
                return Ok(());
            }
            let i = self.next.fetch_add(1, Ordering::SeqCst) as usize;
            let delay = Duration::from_millis(self.delays[i % self.delays.len()]);
            let (flags, path) = (self.flags.clone(), self.status.clone());
            std::thread::spawn(move || {
                std::thread::sleep(delay);
                let mut f = flags.lock().unwrap();
                *f ^= FLAG_GEAR;
                mock::write_status(&path, *f, true);
            });
        }
        Ok(())
    }
    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
    }
}

const BINDS: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="DualVKB" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="231D3201" Key="Joy_4" /><Secondary Device="Keyboard" Key="Key_L" /></LandingGearToggle>
 <ShipSpotLightToggle><Primary Device="231D0200" Key="Joy_7" /><Secondary Device="{NoDevice}" Key="" /></ShipSpotLightToggle>
</Root>"#;

struct Rig {
    _dir: tempfile::TempDir,
    game: Arc<FakeGame>,
    speaker: Arc<RecordingSpeaker>,
    runner: Arc<Runner>,
    hub: Arc<StateHub>,
    logs: Arc<std::sync::Mutex<Vec<String>>>,
    _pipe: astromech_core::pipeline::Pipeline,
}

fn rig(flags: u64, delays: &[u64]) -> Rig {
    let dir = tempfile::tempdir().unwrap();
    let journal = dir.path().join("journal");
    let bindings = dir.path().join("Bindings");
    std::fs::create_dir_all(&journal).unwrap();
    std::fs::create_dir_all(&bindings).unwrap();
    std::fs::write(bindings.join("DualVKB.4.2.binds"), BINDS).unwrap();
    std::fs::write(bindings.join("StartPreset.4.start"), "DualVKB\nDualVKB\nDualVKB\nDualVKB\n").unwrap();
    let game = FakeGame::new(&journal, flags, delays);
    let hub = Arc::new(StateHub::new());
    let pipe =
        astromech_core::pipeline::start(&journal, hub.clone(), Some(&dir.path().join("db.sqlite")), |_| {}, None)
            .unwrap();
    let binds = Arc::new(RwLock::new(health::run_check(&bindings, Some(&dir.path().join("nogame")), false)));
    let speaker = Arc::new(RecordingSpeaker::default());
    let logs = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let l2 = logs.clone();
    let runner = Arc::new(Runner {
        policy: Policy::new(game.clone(), Config::default()),
        hub: hub.clone(),
        binds,
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(2000),
        log: Arc::new(move |s: &str| l2.lock().unwrap().push(s.into())),
    });
    assert!(hub.wait_flag(FLAG_IN_MAIN_SHIP, true, Duration::from_secs(2), &|| false), "initial Status.json read");
    Rig { _dir: dir, game, speaker, runner, hub, logs, _pipe: pipe }
}

fn sent(o: Outcome) -> Verified {
    match o {
        Outcome::Sent(h) => h.join().unwrap(),
        o => panic!("expected Sent, got {o:?}"),
    }
}

#[test]
fn gear_down_success_across_the_measured_delay_range() {
    for d in [0, 350, 1000] {
        let r = rig(FLAG_IN_MAIN_SHIP, &[d]);
        let v = sent(r.runner.run(GEAR_DOWN, Origin::Voice));
        assert!(matches!(v, Verified::Confirmed { .. }), "delay {d} ms");
        assert_eq!(r.speaker.take(), vec!["Landing gear deployed."], "optimistic confirmation only");
        assert!(r.hub.snapshot().status_flags & FLAG_GEAR != 0);
        let log = r.game.log.lock().unwrap().clone();
        assert_eq!(log, vec![(key("Key_L"), false), (key("Key_L"), true)]);
    }
}

#[test]
fn missed_press_is_logged_not_retried_or_spoken() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[0]);
    let logs = r.logs.clone();
    r.game.drop_presses.store(5, Ordering::SeqCst);
    assert_eq!(sent(r.runner.run(GEAR_DOWN, Origin::Voice)), Verified::NotConfirmed);
    assert_eq!(r.speaker.take(), vec!["Landing gear deployed."], "no failure spoken");
    assert_eq!(r.game.presses.load(Ordering::SeqCst), 1, "never pressed again");
    assert!(logs.lock().unwrap().iter().any(|l| l.contains("not confirmed within 2.0 s")), "{logs:?}");
}

#[test]
fn confirmation_is_logged_with_latency() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[600]);
    let logs = r.logs.clone();
    let Verified::Confirmed { ms } = sent(r.runner.run(GEAR_DOWN, Origin::Voice)) else { panic!() };
    assert!((550..1500).contains(&ms), "{ms}");
    assert!(logs.lock().unwrap()[0].contains("confirmed in"));
}

#[test]
fn duplicate_is_suppressed_while_in_flight() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[800]);
    let first = r.runner.run(GEAR_DOWN, Origin::Voice);
    let second = r.runner.run(GEAR_DOWN, Origin::Voice);
    assert!(matches!(second, Outcome::Denied(Denial::Duplicate)), "{second:?}");
    assert!(matches!(sent(first), Verified::Confirmed { .. }));
    // After it landed: the gear is down, so a repeat says so and presses nothing.
    let third = r.runner.run(GEAR_DOWN, Origin::Voice);
    assert!(matches!(third, Outcome::Already(_)), "{third:?}");
    assert_eq!(r.game.presses.load(Ordering::SeqCst), 1);
    assert_eq!(r.speaker.take(), vec!["Landing gear deployed.", "Already on it.", "Landing gear is already deployed."]);
}

#[test]
fn kill_switch_cancels_and_blocks_until_rearmed() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[0]);
    r.game.drop_presses.store(1, Ordering::SeqCst);
    let o = r.runner.run(GEAR_DOWN, Origin::Voice);
    std::thread::sleep(Duration::from_millis(300));
    r.runner.policy.kill();
    assert_eq!(sent(o), Verified::Cancelled);
    assert_eq!(r.game.presses.load(Ordering::SeqCst), 1, "one press only");
    let log = r.game.log.lock().unwrap().clone();
    assert_eq!(log.iter().filter(|e| !e.1).count(), log.iter().filter(|e| e.1).count(), "no key left down");
    assert!(matches!(r.runner.run(GEAR_DOWN, Origin::Voice), Outcome::Denied(Denial::KillSwitch)));
    r.runner.policy.rearm();
    std::thread::sleep(Duration::from_millis(3100)); // past the 3 s duplicate window
    assert!(matches!(sent(r.runner.run(GEAR_DOWN, Origin::Voice)), Verified::Confirmed { .. }));
}

#[test]
fn state_rules_press_nothing() {
    for (f, word) in [(FLAG_DOCKED, "docked"), (FLAG_SUPERCRUISE, "supercruise")] {
        let r = rig(FLAG_IN_MAIN_SHIP | f, &[0]);
        assert!(matches!(r.runner.run(GEAR_DOWN, Origin::Voice), Outcome::Refused(m) if m.contains(word)));
        assert!(r.game.log.lock().unwrap().is_empty());
    }
    let r = rig(FLAG_IN_MAIN_SHIP | FLAG_GEAR, &[0]);
    assert!(matches!(r.runner.run(GEAR_DOWN, Origin::Voice), Outcome::Already(_)));
    // Game state changes are picked up live.
    r.game.set_flags(FLAG_IN_MAIN_SHIP);
    assert!(r.hub.wait_flag(FLAG_GEAR, false, Duration::from_secs(2), &|| false));
    assert!(matches!(r.runner.run(GEAR_DOWN, Origin::Voice), Outcome::Sent(_)));
}

#[test]
fn no_keyboard_binding_refuses_clearly() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[0]);
    let lights = ToolCall::Toggle { toggle: Toggle::Lights, target: Target::On };
    assert!(
        matches!(r.runner.run(lights, Origin::Voice), Outcome::NoBinding(m) if m.contains("I can't switch lights"))
    );
    assert!(r
        .runner
        .binds
        .read()
        .unwrap()
        .issues
        .iter()
        .any(|i| i.text.starts_with("ShipSpotLightToggle has no keyboard binding")));
    assert!(r.game.log.lock().unwrap().is_empty());
}

#[test]
fn voice_path_with_fake_stt() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[100]);
    let a = Assistant::new(r.runner.clone());
    let mut stt = FakeStt::new(&["Definitely hard points.", "no", "Landing gear dawn.", "banana bread"]);
    let mut next = || stt.transcribe(&[0.0; 160], 16000).unwrap();
    assert!(matches!(a.handle(&next(), Origin::Voice), Handled::Asked { command: "hardpoints.deploy", .. }));
    assert!(matches!(a.handle(&next(), Origin::Voice), Handled::Cancelled));
    match a.handle(&next(), Origin::Voice) {
        Handled::Ran { command: "landing_gear.down", outcome } => {
            assert!(matches!(sent(outcome), Verified::Confirmed { .. }))
        }
        h => panic!("{h:?}"),
    }
    assert!(matches!(a.handle(&next(), Origin::Voice), Handled::NotUnderstood));
    assert_eq!(
        r.speaker.take(),
        vec!["Did you mean deploy hardpoints?", "Cancelled.", "Landing gear deployed.", "Didn't catch that."]
    );
}

#[test]
fn did_you_mean_then_yes_executes() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[100]);
    let a = Assistant::new(r.runner.clone());
    assert!(matches!(a.handle("gear town please", Origin::Voice), Handled::Asked { .. } | Handled::Ran { .. }));
    let r2 = rig(FLAG_IN_MAIN_SHIP, &[100]);
    let a2 = Assistant::new(r2.runner.clone());
    assert!(matches!(a2.handle("lights of", Origin::Voice), Handled::Asked { command: "lights.off", .. }));
    // Lights are already off: 'yes' runs the tool, which says so.
    assert!(matches!(a2.handle("yes", Origin::Voice), Handled::Ran { outcome: Outcome::Already(_), .. }));
    // A lone 'yes' with nothing pending is not a command.
    assert!(matches!(a2.handle("yes", Origin::Voice), Handled::NotUnderstood));
}

#[test]
fn real_game_round_two() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[100]);
    let a = Assistant::new(r.runner.clone());
    // Noise-only clips are ignored silently.
    for t in ["Mm-hmm.", "Yeah.", "Okay.", "Mm."] {
        assert!(matches!(a.handle(t, Origin::Voice), Handled::Ignored), "{t}");
    }
    assert!(r.speaker.take().is_empty());
    // Known but unavailable.
    for t in ["Super Cruise", "Supercruise.", "Engage Super Crews", "Engage Superus."] {
        assert!(matches!(a.handle(t, Origin::Voice), Handled::NotAvailable("supercruise")), "{t}");
    }
    assert!(r.speaker.take().iter().all(|s| s.starts_with("Supercruise isn't available yet")));
    // "You're down." asks; "Yeah." confirms it.
    assert!(matches!(a.handle("You're down.", Origin::Voice), Handled::Asked { command: "landing_gear.down", .. }));
    match a.handle("Yeah.", Origin::Voice) {
        Handled::Ran { command: "landing_gear.down", outcome } => {
            assert!(matches!(sent(outcome), Verified::Confirmed { .. }))
        }
        h => panic!("{h:?}"),
    }
    assert_eq!(r.speaker.take(), vec!["Did you mean gear down?", "Landing gear deployed."]);
    assert_eq!(r.game.presses.load(Ordering::SeqCst), 1);
}

#[test]
fn night_vision_without_keyboard_key_is_refused_clearly() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[100]);
    let a = Assistant::new(r.runner.clone());
    let h = a.handle("night vision on", Origin::Voice);
    assert!(matches!(h, Handled::Ran { outcome: Outcome::NoBinding(_), .. }), "{h:?}");
    let said = r.speaker.take();
    assert_eq!(said.len(), 1);
    assert!(said[0].starts_with("I can't switch night vision:"), "{said:?}");
    assert!(said[0].contains("give Night Vision a keyboard key") && !said[0].contains("NightVisionToggle"), "{said:?}");
    assert_eq!(r.game.presses.load(Ordering::SeqCst), 0);
}

#[test]
fn journal_rollover_and_saved_positions() {
    let dir = tempfile::tempdir().unwrap();
    let j = dir.path();
    let a = j.join("Journal.2026-10-10T180000.01.log");
    std::fs::write(&a, "{\"timestamp\":\"2026-10-10T18:00:00Z\",\"event\":\"Fileheader\",\"part\":1,\"gameversion\":\"4.2\",\"build\":\"r1\"}\r\n{\"timestamp\":\"2026-10-10T18:00:01Z\",\"event\":\"Location\",\"StarSystem\":\"Sol\",\"Docked\":true}\r\n").unwrap();
    let db = j.join("db.sqlite");
    let hub = Arc::new(StateHub::new());
    let p = astromech_core::pipeline::start(j, hub.clone(), Some(&db), |_| {}, None).unwrap();
    assert_eq!(hub.snapshot().system.as_deref(), Some("Sol"));
    std::thread::sleep(Duration::from_millis(50));
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().append(true).open(&a).unwrap();
    write!(f, "{{\"timestamp\":\"2026-10-10T18:00:05Z\",\"event\":\"FSDJump\",\"StarSystem\":\"Alpha Centauri\"}}\r\n")
        .unwrap();
    drop(f);
    wait(|| hub.snapshot().system.as_deref() == Some("Alpha Centauri"));
    // The game starts a new file.
    std::thread::sleep(Duration::from_millis(1100));
    let b = j.join("Journal.2026-10-10T190000.01.log");
    std::fs::write(&b, "{\"timestamp\":\"2026-10-10T19:00:00Z\",\"event\":\"Fileheader\",\"part\":1}\r\n{\"timestamp\":\"2026-10-10T19:00:01Z\",\"event\":\"FSDJump\",\"StarSystem\":\"Barnard's Star\"}\r\n").unwrap();
    wait(|| hub.snapshot().system.as_deref() == Some("Barnard's Star"));
    assert_eq!(hub.snapshot().journal_file.as_deref(), Some("Journal.2026-10-10T190000.01.log"));
    wait(|| p.stats.lock().unwrap().rollovers == 1);
    drop(p);
    let s = astromech_store::Store::open(&db).unwrap();
    let la = std::fs::metadata(&a).unwrap().len();
    let lb = std::fs::metadata(&b).unwrap().len();
    assert_eq!(s.journal_offset(&a.display().to_string()).unwrap(), Some((la, la)));
    assert_eq!(s.journal_offset(&b.display().to_string()).unwrap(), Some((lb, lb)));
}

#[test]
fn status_questions_are_spoken_not_pressed() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[0]);
    let a = Assistant::new(r.runner.clone());
    match a.handle("how much fuel", Origin::Voice) {
        Handled::Ran { command: "status.fuel", outcome: Outcome::Spoken(s) } => {
            assert!(s.contains("32"), "{s}");
        }
        h => panic!("{h:?}"),
    }
    assert!(r.game.log.lock().unwrap().is_empty());
}

#[test]
fn silent_running_asks_then_runs() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[0]);
    let a = Assistant::new(r.runner.clone());
    assert!(matches!(
        a.handle("silent running on", Origin::Voice),
        Handled::Asked { command: "silent_running.on", .. }
    ));
    assert!(r.speaker.take().iter().any(|s| s.contains("heat")));
    let h = a.handle("yes", Origin::Voice);
    assert!(matches!(h, Handled::Ran { command: "silent_running.on", outcome: Outcome::NoBinding(_) }), "{h:?}");
}

#[test]
fn did_you_mean_yes_learns_alias() {
    let r = rig(FLAG_IN_MAIN_SHIP, &[100]);
    let a = Assistant::new(r.runner.clone());
    assert!(matches!(a.handle("You're down.", Origin::Voice), Handled::Asked { command: "landing_gear.down", .. }));
    match a.handle("yes", Origin::Voice) {
        Handled::Ran { command: "landing_gear.down", outcome } => {
            assert!(matches!(sent(outcome), Verified::Confirmed { .. }))
        }
        h => panic!("{h:?}"),
    }
    match a.handle("You're down.", Origin::Voice) {
        Handled::Ran { command: "landing_gear.down", outcome: Outcome::Already(_) } => {}
        h => panic!("alias should execute, {h:?}"),
    }
}

fn wait(f: impl Fn() -> bool) {
    for _ in 0..100 {
        if f() {
            return;
        }
        std::thread::sleep(Duration::from_millis(30));
    }
    panic!("timed out waiting");
}
