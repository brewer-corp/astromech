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
        remote: Arc::new(astromech_data::Client::new(astromech_data::Cache::in_memory().unwrap())),
        galaxy: None,
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
fn kill_switch_during_the_live_run_wait_presses_nothing() {
    let r = rig();
    *r.exec.front.lock().unwrap() = Some("astromech".into());
    // Kill switch on and off again while waiting, then the user switches to Elite.
    let (exec, policy) = (r.exec.clone(), r.runner.policy.clone());
    let h = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(80));
        policy.kill();
        policy.rearm();
        *exec.front.lock().unwrap() = None;
    });
    let m = tester::live(&r.runner, "landing_gear.down", Duration::from_secs(3)).unwrap();
    h.join().unwrap();
    assert_eq!(m, "The kill switch was used, so nothing was sent to the game.");
    assert!(r.exec.sent.lock().unwrap().is_empty());
}

#[test]
fn push_to_talk_in_test_mode_presses_nothing_until_the_page_closes() {
    let r = rig();
    let t = Tester::new();
    let mut stt = FakeStt::new(&["Landing gear dawn.", "Gear down."]);
    let now = Instant::now();
    t.arm(now);
    let heard = stt.transcribe(&[0.0; 160], 16000).unwrap();
    match tester::on_heard(&t, &r.assistant, &heard, t.armed(now)) {
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
    match tester::on_heard(&t, &r.assistant, &heard, t.armed(later)) {
        Heard::Handled(Handled::Ran { command: "landing_gear.down", .. }) => {}
        h => panic!("{h:?}"),
    }
    assert_eq!(r.exec.sent.lock().unwrap().len(), 2, "one press, down and up");
}

/// Two wrong yes answers teach a key-pressing phrase for good; forgetting it makes astromech ask
/// again, and the saved list no longer has it.
#[test]
fn a_learned_phrase_can_be_forgotten() {
    use astromech_proto::Origin::Voice;
    let r = rig();
    let store = Arc::new(Mutex::new(astromech_store::Store::in_memory().unwrap()));
    let a = Assistant::new(r.runner.clone()).with_store(store.clone());
    let heard = "Definitely hard points.";
    for _ in 0..2 {
        assert!(matches!(a.handle(heard, Voice), Handled::Asked { command: "hardpoints.deploy", .. }));
        assert!(matches!(a.handle("yes", Voice), Handled::Ran { .. }));
    }
    assert_eq!(store.lock().unwrap().aliases().unwrap().len(), 1);
    assert!(
        matches!(a.handle(heard, Voice), Handled::Ran { command: "hardpoints.deploy", .. }),
        "learned: runs without asking"
    );

    assert!(a.forget(heard));
    assert!(store.lock().unwrap().aliases().unwrap().is_empty());
    assert!(matches!(a.handle(heard, Voice), Handled::Asked { .. }), "forgotten: asks again");
    assert!(!a.forget(heard), "nothing left to forget");
}

/// astromech-1c66: a near-miss from the mishearing table ("I'll stop", "Pays") asks every time,
/// however often it is answered yes, and never shows in the learned-phrases list.
#[test]
fn ask_only_near_misses_are_never_learned() {
    use astromech_proto::Origin::Voice;
    let r = rig();
    let data = tempfile::tempdir().unwrap();
    let store = astromech_store::Store::open(&data.path().join("astromech.sqlite")).unwrap();
    let a = Assistant::new(r.runner.clone()).with_store(Arc::new(Mutex::new(store)));
    for (heard, command) in [("I'll stop.", "throttle.zero"), ("Pays.", "galaxy_map.paste")] {
        for _ in 0..3 {
            let h = a.handle(heard, Voice);
            assert!(matches!(h, Handled::Asked { command: c, .. } if c == command), "{heard}: {h:?}");
            let h = a.handle("yes", Voice);
            assert!(matches!(h, Handled::Ran { command: c, .. } if c == command), "{heard}: yes runs it once, {h:?}");
        }
        let h = a.handle(heard, Voice);
        assert!(matches!(h, Handled::Asked { .. }), "{heard}: still asks, {h:?}");
    }
    assert_eq!(astromech_core::learned::list(data.path())["phrases"], serde_json::json!([]));
}

/// A genuine near-miss is still learned: after one yes for a spoken answer, after two for a
/// command that presses a key (astromech-1c66).
#[test]
fn genuine_phrases_are_learned_key_presses_after_two_yes() {
    use astromech_proto::Origin::Voice;
    let r = rig();
    let data = tempfile::tempdir().unwrap();
    let store = astromech_store::Store::open(&data.path().join("astromech.sqlite")).unwrap();
    let a = Assistant::new(r.runner.clone()).with_store(Arc::new(Mutex::new(store)));
    // A status question: one yes.
    assert!(matches!(a.handle("hull help", Voice), Handled::Asked { command: "status.hull", .. }));
    assert!(matches!(a.handle("yes", Voice), Handled::Ran { command: "status.hull", .. }));
    assert!(matches!(a.handle("hull help", Voice), Handled::Ran { command: "status.hull", .. }));
    // A key press: the first yes runs it but still asks next time; the second teaches it.
    let heard = "Definitely hard points.";
    assert!(matches!(a.handle(heard, Voice), Handled::Asked { command: "hardpoints.deploy", .. }));
    assert!(matches!(a.handle("yes", Voice), Handled::Ran { command: "hardpoints.deploy", .. }));
    assert!(matches!(a.handle(heard, Voice), Handled::Asked { command: "hardpoints.deploy", .. }));
    let listed = astromech_core::learned::list(data.path());
    assert_eq!(listed["phrases"].as_array().unwrap().len(), 1, "only the status phrase so far: {listed}");
    assert!(matches!(a.handle("yes", Voice), Handled::Ran { command: "hardpoints.deploy", .. }));
    assert!(matches!(a.handle(heard, Voice), Handled::Ran { command: "hardpoints.deploy", .. }));
    assert_eq!(astromech_core::learned::list(data.path())["phrases"].as_array().unwrap().len(), 2);

    // A restart loads the same rule from the saved counts.
    let mut g = astromech_core::assistant::grammar();
    let st = astromech_store::Store::open(&data.path().join("astromech.sqlite")).unwrap();
    st.learn_alias("lamps on", "lights.on").unwrap();
    astromech_core::assistant::add_aliases(&mut g, &st);
    assert_eq!(g.best(heard).decision, astromech_intent::Decision::Execute);
    assert_ne!(g.best("lamps on").decision, astromech_intent::Decision::Execute, "one yes is not enough");
}

/// A phrase is tested or run according to the mode when push-to-talk was released: leaving the
/// page while the clip is transcribed neither runs a tested phrase for real nor tests a live one.
#[test]
fn a_phrase_follows_the_mode_at_release_not_when_transcription_ends() {
    let r = rig();
    let t = Tester::new();
    let now = Instant::now();

    // Released in test mode; the page is closed before the transcript arrives.
    t.arm(now);
    let testing = t.armed(now);
    t.disarm();
    match tester::on_heard(&t, &r.assistant, "Gear down.", testing) {
        Heard::Tested(d) => assert_eq!(d.intent.unwrap().command, "landing_gear.down"),
        h => panic!("a tested phrase must stay a test: {h:?}"),
    }
    r.nothing_happened();

    // Released live; the page is opened before the transcript arrives.
    let testing = t.armed(now);
    t.arm(now);
    match tester::on_heard(&t, &r.assistant, "Gear down.", testing) {
        Heard::Handled(Handled::Ran { command: "landing_gear.down", .. }) => {}
        h => panic!("a live phrase must stay live: {h:?}"),
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

/// The route commands (docs/PLOTTING.md) are described, not reported as unavailable.
#[test]
fn route_commands_are_described() {
    let dir = tempfile::tempdir().unwrap();
    let mut cfg = Config::default();
    cfg.game.bindings_dir = write_binds(dir.path()).display().to_string();
    cfg.data_dir = dir.path().join("data").display().to_string();
    for (said, command, about) in
        [("next waypoint", "route.next", "clipboard"), ("clear route", "route.clear", "route")]
    {
        let d = tester::dry_run_idle(&cfg, said);
        assert_eq!(d.intent.unwrap().command, command);
        assert_eq!(d.would, Would::Speak, "{said}: {}", d.message);
        assert!(d.message.contains(about), "{said}: {}", d.message);
    }
    // Paste presses keys, and only in the galaxy map.
    let d = tester::dry_run_idle(&cfg, "paste");
    assert_eq!(d.intent.unwrap().command, "galaxy_map.paste");
    assert_eq!((d.would, d.message.as_str()), (Would::Refused, astromech_core::route::PASTE_REFUSED));
    let grammar = astromech_core::assistant::grammar();
    let map = astromech_gamestate::GameState {
        status_seen: true,
        gui_focus: Some(astromech_status::GUI_GALAXY),
        ..Default::default()
    };
    let d = tester::dry_run(&grammar, "paste", &map, &health::BindsState::default(), Source::Typed);
    assert_eq!(d.would, Would::Press, "{}", d.message);
    assert!(d.message.contains("Ctrl+V"), "{}", d.message);
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
    let Heard::Tested(d) = tester::on_heard(&t, &r.assistant, &heard, t.armed(now)) else { panic!("not tested") };
    println!("clip {clip}: heard {heard:?} -> {:?} would {:?}, keys {:?}", d.intent, d.would, d.keys);
    assert!(!d.transcript.is_empty());
    assert_eq!(d.intent.as_ref().map(|i| i.command.as_str()), Some(expect.as_str()));
    r.nothing_happened();
}
