//! The session summary end to end (astromech-oo8y), on the fixture history of astromech-bg0x
//! (three sessions of one commander): imported into `history.sqlite`, read through the core's
//! read connection, spoken by the reaction engine at the end of a session and by the voice
//! query.

use astromech_announce::{builtin, Engine, Settings, Shared};
use astromech_core::actions::{Outcome, Runner};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health::BindsState;
use astromech_core::hub::StateHub;
use astromech_core::speech::RecordingSpeaker;
use astromech_core::summary::HistorySessions;
use astromech_gamestate::GameState;
use astromech_history::{History, ImportOptions};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_journal::parse_line;
use astromech_policy::{Config as PolicyConfig, Policy};
use astromech_proto::Origin;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

struct NoKeys;

impl Platform for NoKeys {
    fn foreground_title(&self) -> Option<String> {
        Some(GAME_WINDOW_TITLE.into())
    }
    fn send_key(&self, k: ScanKey, _up: bool) -> Result<(), String> {
        panic!("the session summary pressed {k:?}")
    }
    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
    }
}

/// The fixture journals imported into a file, read the way the core reads it.
fn history() -> (tempfile::TempDir, Shared) {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("history.sqlite");
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../astromech-history/tests/fixtures");
    History::open(&db).unwrap().import(&fixtures, &ImportOptions::default()).unwrap();
    let shared = Shared::default();
    shared.sessions.set(Arc::new(HistorySessions::new(db)));
    (dir, shared)
}

/// Feed `lines` live, then tick once: what was spoken.
fn spoken(shared: &Shared, lines: &[&str]) -> Vec<String> {
    let mut engine = Engine::new(builtin(shared.clone()), &Settings::default());
    engine.set_live(true);
    let mut state = GameState::default();
    let t0 = Instant::now();
    let mut out = vec![];
    for (i, l) in lines.iter().enumerate() {
        let j = parse_line(l).unwrap();
        state.apply_line(&j);
        out.extend(engine.journal(&j, &state, t0 + Duration::from_secs(i as u64)));
    }
    assert!(out.iter().all(|r| r.rule != "session_summary"), "nothing is said before the tick");
    let at = lines.len() as u64;
    let now = chrono::DateTime::parse_from_rfc3339("2026-09-02T11:00:05Z").unwrap().timestamp();
    out.extend(engine.tick(now, &state, t0 + Duration::from_secs(at)));
    out.into_iter().filter(|r| r.rule == "session_summary" && r.channels.speech).map(|r| r.text).collect()
}

const LOAD: &str = r#"{"timestamp":"2026-09-02T10:00:05Z","event":"LoadGame","FID":"F0000001","Commander":"Fixture","Ship":"CobraMkIII"}"#;

#[test]
fn the_end_of_a_session_speaks_its_totals_once() {
    let (_dir, shared) = history();
    // 2 September: 100 thousand from Powerplay in an hour, two FSD jumps (10.33 ly) and two
    // bounties awarded.
    let said =
        ["Earned 100 thousand credits, 100 thousand an hour.", "2 jumps, 10 light years.", "2 bounties awarded."];
    assert_eq!(spoken(&shared, &[LOAD, r#"{"timestamp":"2026-09-02T11:00:00Z","event":"Shutdown"}"#]), said);
    // Back to the main menu, then the game closing: said once.
    let menu = r#"{"timestamp":"2026-09-02T11:00:00Z","event":"Music","MusicTrack":"MainMenu"}"#;
    let quit = r#"{"timestamp":"2026-09-02T11:00:00Z","event":"Shutdown"}"#;
    assert_eq!(spoken(&shared, &[LOAD, menu, quit]), said);
}

#[test]
fn asking_by_voice_answers_the_latest_session() {
    let (_dir, shared) = history();
    let hub = Arc::new(StateHub::new());
    hub.update(|g| g.commander = Some("Fixture".into()));
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(Arc::new(NoKeys), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        remote: Arc::new(astromech_data::Client::new(astromech_data::Cache::in_memory().unwrap())),
        galaxy: None,
    });
    let assistant = Assistant::new(runner).with_sessions(shared.sessions.clone());
    let h = assistant.handle("How much did I make this session?", Origin::Voice);
    let Handled::Ran { command: "session.summary", outcome: Outcome::Spoken(_) } = h else { panic!("{h:?}") };
    let said = speaker.take();
    // 3 September: 7,221,500 in half an hour, no jumps or bounties.
    assert_eq!(said, ["Earned 7.2 million credits, 14.5 million an hour."]);
}

#[test]
fn without_a_history_it_says_so() {
    let hub = Arc::new(StateHub::new());
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(Arc::new(NoKeys), PolicyConfig::default()),
        hub,
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        remote: Arc::new(astromech_data::Client::new(astromech_data::Cache::in_memory().unwrap())),
        galaxy: None,
    });
    Assistant::new(runner).handle("session summary", Origin::Voice);
    assert_eq!(speaker.take(), ["No session history yet."]);
}

#[test]
fn a_history_that_cannot_be_read_says_so_rather_than_no_history() {
    struct Locked;
    impl astromech_announce::session_summary::SessionHistory for Locked {
        fn session(
            &self,
            _: Option<&str>,
            _: Option<&str>,
        ) -> Result<Option<astromech_announce::session_summary::Totals>, astromech_announce::session_summary::Unreadable>
        {
            Err(astromech_announce::session_summary::Unreadable)
        }
    }
    let shared = Shared::default();
    shared.sessions.set(Arc::new(Locked));
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(Arc::new(NoKeys), PolicyConfig::default()),
        hub: Arc::new(StateHub::new()),
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        galaxy: None,
    });
    Assistant::new(runner).with_sessions(shared.sessions.clone()).handle("session summary", Origin::Voice);
    assert_eq!(speaker.take(), ["Session history is unreadable right now."]);
}
