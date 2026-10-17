//! Colonisation questions by voice, end to end: phrase → grammar → the projects replayed from a
//! synthetic journal folder (two projects being built, one finished) → spoken progress and what
//! is left; and "where can I buy CMM composite for the depot" → the Colonisation page's Ardent
//! lookup on the recorded answer around Sol (astromech-market's fixture, 16 Oct 2026). Nothing
//! is pressed and no test touches the network.

use astromech_core::actions::{Outcome, Runner};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health::BindsState;
use astromech_core::hub::StateHub;
use astromech_core::speech::RecordingSpeaker;
use astromech_core::tester::{Source, Would};
use astromech_data::{Cache, Client, Clock, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Galaxy, Sphere};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_intent::Decision;
use astromech_policy::{Config as PolicyConfig, Policy};
use astromech_proto::Origin;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

/// A Type-9 (400 t) building Test Hub (docked there first, half built) and Far Port (delivered
/// to last); Old Yard is finished.
const JOURNAL: &str = r#"{"timestamp":"2026-10-14T09:00:00Z","event":"Fileheader","part":1,"gameversion":"4.2.0.0","build":"r1"}
{"timestamp":"2026-10-14T09:00:01Z","event":"Loadout","Ship":"type9","ShipID":7,"CargoCapacity":400,"Modules":[]}
{"timestamp":"2026-10-14T10:00:00Z","event":"Docked","StationName":"Planetary Construction Site: Test Hub","StationType":"SurfaceStation","StarSystem":"Test System","SystemAddress":1001,"MarketID":3900000001}
{"timestamp":"2026-10-14T10:30:00Z","event":"ColonisationConstructionDepot","MarketID":3900000001,"ConstructionProgress":0.5,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$steel_name;","Name_Localised":"Steel","RequiredAmount":800,"ProvidedAmount":500,"Payment":4000},{"Name":"$aluminium_name;","Name_Localised":"Aluminium","RequiredAmount":200,"ProvidedAmount":50,"Payment":3000}]}
{"timestamp":"2026-10-14T10:40:00Z","event":"ColonisationContribution","MarketID":3900000001,"Contributions":[{"Name":"$steel_name;","Amount":100}]}
{"timestamp":"2026-10-14T12:00:00Z","event":"Docked","StationName":"Orbital Construction Site: Old Yard","StationType":"SpaceConstructionDepot","StarSystem":"Old System","SystemAddress":1002,"MarketID":3900000002}
{"timestamp":"2026-10-14T12:00:05Z","event":"ColonisationConstructionDepot","MarketID":3900000002,"ConstructionProgress":1.0,"ConstructionComplete":true,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$steel_name;","RequiredAmount":100,"ProvidedAmount":100,"Payment":4000}]}
{"timestamp":"2026-10-15T10:00:00Z","event":"Docked","StationName":"Orbital Construction Site: Far Port","StationType":"SpaceConstructionDepot","StarSystem":"Far System","SystemAddress":1003,"MarketID":3900000003}
{"timestamp":"2026-10-15T10:00:05Z","event":"ColonisationConstructionDepot","MarketID":3900000003,"ConstructionProgress":0.1,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$cmmcomposite_name;","Name_Localised":"CMM Composite","RequiredAmount":5000,"ProvidedAmount":100,"Payment":6000}]}
{"timestamp":"2026-10-15T10:10:00Z","event":"ColonisationContribution","MarketID":3900000003,"Contributions":[{"Name":"$cmmcomposite_name;","Amount":100}]}
{"timestamp":"2026-10-15T11:00:00Z","event":"Undocked","StationName":"Orbital Construction Site: Far Port","MarketID":3900000003}
"#;

/// 120 t of steel aboard.
const CARGO: &str = r#"{"timestamp":"2026-10-15T11:00:00Z","event":"Cargo","Vessel":"Ship","Count":120,"Inventory":[{"Name":"steel","Count":120,"Stolen":0}]}"#;

/// Ardent's CMM composite sellers near Sol, recorded; logs each request's URL and query.
#[derive(Default)]
struct Recorded {
    log: Mutex<Vec<String>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push(format!("{url} {:?}", req.query));
        let body = if url.ends_with("/system/name/Sol/commodity/name/cmmcomposite/nearby/exports") {
            include_str!("../../astromech-market/tests/fixtures/ardent-buy-cmmcomposite-sol.json")
        } else {
            panic!("no recorded answer for {url}")
        };
        Ok(Response { status: 200, body: body.into() })
    }
}

/// The evening the answers were recorded.
struct Evening;

impl Clock for Evening {
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
        panic!("a colonisation question pressed {k:?}")
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
    hub: Arc<StateHub>,
    _journals: tempfile::TempDir,
}

/// In Sol, undocked, with the journal folder above.
fn rig() -> Rig {
    let journals = tempfile::tempdir().unwrap();
    std::fs::write(journals.path().join("Journal.2026-10-14T090000.01.log"), JOURNAL).unwrap();
    std::fs::write(journals.path().join("Cargo.json"), CARGO).unwrap();
    let rec = Arc::new(Recorded::default());
    let client = Arc::new(Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none()));
    let stars = astromech_galaxy::Store::in_memory().unwrap();
    // Covered space, so no EDSM fill-in is asked for.
    stars.add_coverage(Sphere { center: [0.0; 3], radius: 100.0 }, "test", "t").unwrap();
    let hub = Arc::new(StateHub::new());
    hub.update(|g| {
        g.status_seen = true;
        g.status_flags = astromech_status::FLAG_IN_MAIN_SHIP;
        g.system = Some("Sol".into());
        g.star_pos = Some([0.0; 3]);
        g.ship = Some("type9".into());
        g.max_jump_range = Some(20.0);
    });
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(Arc::new(NoKeys), PolicyConfig::default()),
        hub: hub.clone(),
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        galaxy: Some(Arc::new(Galaxy::with_remote(stars, client.clone()))),
        remote: client,
    });
    let store = Arc::new(Mutex::new(astromech_store::Store::in_memory().unwrap()));
    let assistant = Assistant::new(runner)
        .with_store(store.clone())
        .with_clock(Arc::new(Evening))
        .with_journals(journals.path().to_path_buf());
    Rig { assistant, speaker, rec, store, hub, _journals: journals }
}

fn answered(h: Handled) -> (&'static str, String) {
    match h {
        Handled::Ran { command, outcome: Outcome::LookingUp(h) } => (command, h.join().unwrap()),
        h => panic!("expected an answer, got {h:?}"),
    }
}

#[test]
fn colonisation_phrases_resolve_to_their_commands() {
    let g = astromech_core::assistant::grammar();
    for (phrase, command, slot) in [
        ("How is the build going?", "colonisation.progress", None),
        ("how's the build going", "colonisation.progress", None),
        ("construction progress", "colonisation.progress", None),
        ("What is left to deliver?", "colonisation.remaining", None),
        ("what's left to deliver", "colonisation.remaining", None),
        ("what does the build need", "colonisation.remaining", None),
        ("Where can I buy steel for the depot?", "colonisation.buy", Some("Steel")),
        ("where can I buy CMM composite for the build", "colonisation.buy", Some("CMM Composite")),
        ("where to buy aluminium for the depot", "colonisation.buy", Some("Aluminium")),
        // the plain trade question keeps its phrase
        ("where can I buy steel", "trade.buy", Some("Steel")),
        ("how much did I make this session", "session.summary", None),
    ] {
        let m = g.best(phrase);
        assert_eq!(
            (m.command, m.slot.as_deref(), m.decision),
            (command, slot, Decision::Execute),
            "{phrase:?} -> {m:?}"
        );
    }
}

#[test]
fn progress_and_what_is_left_are_about_the_project_last_delivered_to() {
    let r = rig();
    let (command, said) = answered(r.assistant.handle("How is the build going?", Origin::Voice));
    assert_eq!(command, "colonisation.progress");
    assert_eq!(said, "Far Port is 10 percent built. 4,900 tonnes left to deliver.");
    let (_, said) = answered(r.assistant.handle("What is left to deliver?", Origin::Voice));
    assert_eq!(
        said,
        "4,900 tonnes left to deliver to Far Port. 13 more loads at 400 tonnes. Most needed: CMM Composite, 4,900 tonnes."
    );
    assert_eq!(r.speaker.take().len(), 2, "each answer once, no lookup line");
}

#[test]
fn docked_at_a_site_the_answers_are_about_that_one() {
    let r = rig();
    r.hub.update(|g| {
        g.docked = true;
        g.station_market_id = Some(3900000001);
    });
    let (_, said) = answered(r.assistant.handle("how is the build going", Origin::Voice));
    assert_eq!(said, "Test Hub is 50 percent built. 450 tonnes left to deliver.");
    // 300 steel (120 aboard, from Cargo.json) and 150 aluminium: 330 to buy at 400 a load.
    let (_, said) = answered(r.assistant.handle("what is left to deliver", Origin::Voice));
    assert_eq!(
        said,
        "450 tonnes left to deliver to Test Hub. 120 tonnes of it aboard. 1 more load at 400 tonnes. \
         Most needed: Steel, 180 tonnes."
    );
}

#[test]
fn where_to_buy_runs_the_pages_lookup_with_its_remembered_filters() {
    let r = rig();
    // The commander set the Colonisation page's filters: within 40 ly.
    let f = Filters { max_distance_ly: Some(40), ..Filters::default() };
    astromech_galaxy::filters::save(&r.store.lock().unwrap(), "colonisation.buy", &f).unwrap();

    let (command, said) = answered(r.assistant.handle("Where can I buy CMM composite for the depot?", Origin::Voice));
    assert_eq!(command, "colonisation.buy");
    // The nearest seller with all 4,900 t: closer carriers and outposts stock less.
    assert_eq!(
        said,
        "4,900 tonnes of CMM Composite still to buy. Nearest: Efremov Plant in Sirius, 9 light years. \
         10,061 in stock, 5,801 credits. Reported 4 days ago."
    );
    assert_eq!(r.speaker.take(), vec!["Looking that up.".to_string(), said]);
    let log = r.rec.log.lock().unwrap();
    assert_eq!(log.len(), 1, "{log:?}");
    assert!(log[0].contains(r#"("maxDistance", "40")"#), "the remembered radius goes to Ardent: {}", log[0]);
}

#[test]
fn where_to_buy_something_the_project_does_not_need_says_so() {
    let r = rig();
    let (_, said) = answered(r.assistant.handle("where can I buy steel for the depot", Origin::Voice));
    assert_eq!(said, "Far Port needs no more Steel.");
    assert!(r.rec.log.lock().unwrap().is_empty(), "Ardent is not asked");
}

#[test]
fn the_tester_shows_the_question_without_answering_it() {
    let r = rig();
    let d = r.assistant.dry_run("where can I buy steel for the depot", Source::Typed);
    assert_eq!(d.would, Would::Speak);
    assert_eq!(d.message, "Looks up where to buy Steel for the construction project near you on Ardent Insight.");
    let d = r.assistant.dry_run("how is the build going", Source::Typed);
    assert_eq!(d.message, "Reads the construction project from your journals and says how far along it is.");
}
