//! The voice command tester: a typed phrase or a push-to-talk transcript shows its intent and
//! keys with nothing pressed; only a live run presses, and it goes through the policy.

use astromech_core::actions::Runner;
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::config::Config;
use astromech_core::health;
use astromech_core::hub::StateHub;
use astromech_core::speech::{FakeStt, RecordingSpeaker, Stt};
use astromech_core::tester::{self, Heard, Source, Tester, Would};
use astromech_input::exec::keys::{lookup, KeyLookup, ScanKey};
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_policy::{Config as PolicyConfig, Policy};
use astromech_status::FLAG_IN_MAIN_SHIP;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

/// A fake executor: records every key event it is asked to send. `front` is the active window
/// (Elite unless a test says otherwise).
#[derive(Default)]
struct Recorder {
    sent: Mutex<Vec<(ScanKey, bool)>>,
    front: Mutex<Option<String>>,
}

impl Platform for Recorder {
    fn foreground_title(&self) -> Option<String> {
        Some(self.front.lock().unwrap().clone().unwrap_or_else(|| GAME_WINDOW_TITLE.into()))
    }
    fn send_key(&self, k: ScanKey, up: bool) -> Result<(), String> {
        self.sent.lock().unwrap().push((k, up));
        Ok(())
    }
    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
    }
}

const BINDS: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="Tester" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="231D3201" Key="Joy_4" /><Secondary Device="Keyboard" Key="Key_L" /></LandingGearToggle>
 <ShipSpotLightToggle><Primary Device="Keyboard" Key="Key_K"><Modifier Device="Keyboard" Key="Key_LeftShift" /></Primary><Secondary Device="{NoDevice}" Key="" /></ShipSpotLightToggle>
</Root>"#;

fn write_binds(dir: &Path) -> std::path::PathBuf {
    let b = dir.join("Bindings");
    std::fs::create_dir_all(&b).unwrap();
    std::fs::write(b.join("Tester.4.2.binds"), BINDS).unwrap();
    std::fs::write(b.join("StartPreset.4.start"), "Tester\nTester\nTester\nTester\n").unwrap();
    b
}

fn key(n: &str) -> ScanKey {
    match lookup(n) {
        KeyLookup::Ok(k) => k,
        _ => panic!("{n}"),
    }
}

struct Rig {
    _dir: tempfile::TempDir,
    exec: Arc<Recorder>,
    speaker: Arc<RecordingSpeaker>,
    runner: Arc<Runner>,
    assistant: Assistant,
}

/// In the ship, gear up, lights off.
fn rig() -> Rig {
    let dir = tempfile::tempdir().unwrap();
    let bindings = write_binds(dir.path());
    let hub = Arc::new(StateHub::new());
    hub.update(|g| {
        g.status_flags = FLAG_IN_MAIN_SHIP;
        g.status_seen = true;
    });
    let exec = Arc::new(Recorder::default());
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(exec.clone(), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(health::run_check(&bindings, Some(&dir.path().join("nogame")), false))),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
    });
    let assistant = Assistant::new(runner.clone());
    Rig { _dir: dir, exec, speaker, runner, assistant }
}

impl Rig {
    fn nothing_happened(&self) {
        assert!(self.exec.sent.lock().unwrap().is_empty(), "keys were sent: {:?}", self.exec.sent.lock().unwrap());
        assert!(self.speaker.take().is_empty(), "the tester must not speak");
    }
}

#[test]
fn typed_phrase_shows_intent_and_keys_with_nothing_pressed() {
    let r = rig();
    let d = r.assistant.dry_run("Gear down, please", Source::Typed);
    let i = d.intent.as_ref().unwrap();
    assert_eq!((i.command.as_str(), i.decision), ("landing_gear.down", "execute"));
    assert!(i.score >= 0.85, "{}", i.score);
    assert_eq!(d.would, Would::Press);
    assert_eq!(d.message, "Landing gear deployed.");
    assert_eq!(d.action.as_deref(), Some("LandingGearToggle"));
    let k = d.keys.as_ref().unwrap();
    assert_eq!((k.key.as_str(), k.modifiers.len()), ("Key_L", 0));
    assert_eq!(d.live.as_deref(), Some("landing_gear.down"));

    let d = r.assistant.dry_run("lights on", Source::Typed);
    let k = d.keys.as_ref().unwrap();
    assert_eq!((k.key.as_str(), k.modifiers.clone()), ("Key_K", vec!["Key_LeftShift".to_string()]));
    assert_eq!(d.would, Would::Press);

    // Status questions answer in words and press nothing either.
    let d = r.assistant.dry_run("how much fuel", Source::Typed);
    assert_eq!((d.would, d.keys.is_none()), (Would::Speak, true));

    r.nothing_happened();
    // The dry runs took no policy slot: a live run straight after is not a "duplicate".
    assert_eq!(
        tester::live(&r.runner, "landing_gear.down", Duration::ZERO).unwrap(),
        "Pressed, but the game didn't show the change."
    );
    assert_eq!(*r.exec.sent.lock().unwrap(), vec![(key("Key_L"), false), (key("Key_L"), true)]);
}

#[test]
fn live_run_waits_for_elite_in_front_and_never_presses_into_another_window() {
    let r = rig();
    // The user just clicked Run live, so astromech is the active window.
    *r.exec.front.lock().unwrap() = Some("astromech".into());
    let m = tester::live(&r.runner, "landing_gear.down", Duration::from_millis(300)).unwrap();
    assert_eq!(m, "Elite didn't come to the front in time, so nothing was pressed.");
    assert!(r.exec.sent.lock().unwrap().is_empty());
    // They switch to Elite within the wait: the press goes ahead.
    let exec = r.exec.clone();
    let h = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(150));
        *exec.front.lock().unwrap() = None;
    });
    let m = tester::live(&r.runner, "landing_gear.down", Duration::from_secs(3)).unwrap();
    h.join().unwrap();
    assert_eq!(m, "Pressed, but the game didn't show the change.");
    assert_eq!(*r.exec.sent.lock().unwrap(), vec![(key("Key_L"), false), (key("Key_L"), true)]);
}

#[test]
fn push_to_talk_in_test_mode_presses_nothing_until_the_page_closes() {
    let r = rig();
    let t = Tester::new();
    let mut stt = FakeStt::new(&["Landing gear dawn.", "Gear down."]);
    let now = Instant::now();
    t.arm(now);
    let heard = stt.transcribe(&[0.0; 160], 16000).unwrap();
    match tester::on_heard(&t, &r.assistant, &heard, now) {
        Heard::Tested(d) => {
            assert_eq!(d.source, Source::Voice);
            assert_eq!(d.transcript, "Landing gear dawn.");
            assert_eq!(d.intent.unwrap().command, "landing_gear.down");
            assert_eq!(d.keys.unwrap().key, "Key_L");
        }
        h => panic!("{h:?}"),
    }
    let (seq, last) = t.last().unwrap();
    assert_eq!((seq, last.transcript.as_str()), (1, "Landing gear dawn."));
    r.nothing_happened();

    // The page stopped polling: after the lease, push-to-talk acts again.
    let later = now + tester::LEASE + Duration::from_millis(1);
    let heard = stt.transcribe(&[0.0; 160], 16000).unwrap();
    match tester::on_heard(&t, &r.assistant, &heard, later) {
        Heard::Handled(Handled::Ran { command: "landing_gear.down", .. }) => {}
        h => panic!("{h:?}"),
    }
    assert_eq!(r.exec.sent.lock().unwrap().len(), 2, "one press, down and up");
}

#[test]
fn typed_phrase_without_elite_running_still_shows_keys() {
    let dir = tempfile::tempdir().unwrap();
    let mut cfg = Config::default();
    cfg.game.bindings_dir = write_binds(dir.path()).display().to_string();
    cfg.game.game_dir = dir.path().join("nogame").display().to_string();
    cfg.data_dir = dir.path().join("data").display().to_string();
    let d = tester::dry_run_idle(&cfg, "deploy landing gear");
    assert_eq!(d.intent.unwrap().command, "landing_gear.down");
    assert_eq!(d.keys.unwrap().key, "Key_L");
    assert_eq!(d.would, Would::Refused, "no game state: {}", d.message);
}

/// A recorded clip through the same speech-to-text and tester path push-to-talk uses. Needs the
/// voice worker and the Parakeet model, so it only runs when pointed at them:
/// `ASTROMECH_TEST_WORKER=<astromech-voice-worker> ASTROMECH_TEST_MODELS=<models dir>
/// ASTROMECH_TEST_CLIP=<wav> [ASTROMECH_TEST_EXPECT=landing_gear.down] cargo test -p astromech-core
/// --test voice_tester -- --ignored`.
#[test]
#[ignore = "needs the voice worker, the speech model and a clip"]
fn recorded_clip_shows_transcript_and_intent() {
    let env = |k: &str| std::env::var(k).unwrap_or_else(|_| panic!("set {k}"));
    let (worker, models, clip) =
        (env("ASTROMECH_TEST_WORKER"), env("ASTROMECH_TEST_MODELS"), env("ASTROMECH_TEST_CLIP"));
    let expect = std::env::var("ASTROMECH_TEST_EXPECT").unwrap_or_else(|_| "landing_gear.down".into());
    let r = rig();
    let data = tempfile::tempdir().unwrap();
    let mut stt =
        astromech_core::speech::WorkerStt::start(Path::new(&worker), Path::new(&models), 2, data.path()).unwrap();
    let (rate, samples) = astromech_voice::wav::read_mono_f32(Path::new(&clip)).unwrap();
    let n = astromech_voice::gain::normalise(&samples, rate);
    let heard = stt.transcribe(&n.samples, rate).unwrap();
    let t = Tester::new();
    let now = Instant::now();
    t.arm(now);
    let Heard::Tested(d) = tester::on_heard(&t, &r.assistant, &heard, now) else { panic!("not tested") };
    println!("clip {clip}: heard {heard:?} -> {:?} would {:?}, keys {:?}", d.intent, d.would, d.keys);
    assert!(!d.transcript.is_empty());
    assert_eq!(d.intent.as_ref().map(|i| i.command.as_str()), Some(expect.as_str()));
    r.nothing_happened();
}
