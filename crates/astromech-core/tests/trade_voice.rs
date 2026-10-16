//! Trade questions by voice, end to end: phrase → grammar (commodity slot) → Ardent lookup on
//! recorded answers (astromech-market's fixtures: Sol, 16 Oct 2026) → spoken answer. Nothing is
//! pressed and no test touches the network.

use astromech_core::actions::{Outcome, Runner};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health::BindsState;
use astromech_core::hub::StateHub;
use astromech_core::speech::RecordingSpeaker;
use astromech_core::tester::{dry_run, Source, Would};
use astromech_data::{Cache, Client, Clock, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::Filters;
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_policy::{Config as PolicyConfig, Policy};
use astromech_proto::Origin;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("../../astromech-market/tests/fixtures/", $name))
    };
}

/// Ardent's nearby answers from the recordings, or no network at all; logs request URLs.
#[derive(Default)]
struct Recorded {
    offline: bool,
    log: Mutex<Vec<String>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push(url.to_string());
        if self.offline {
            return Err(TransportError::Offline("dns error".into()));
        }
        let body = if url.ends_with("/system/name/Sol/commodity/name/painite/nearby/imports") {
            fixture!("ardent-sell-painite-sol.json")
        } else if url.ends_with("/system/name/Sol/commodity/name/tritium/nearby/exports") {
            fixture!("ardent-buy-tritium-sol.json")
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
        panic!("a trade question pressed {k:?}")
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
}

/// In Sol in a Krait Mk II (medium pad).
fn rig(offline: bool) -> Rig {
    let hub = Arc::new(StateHub::new());
    hub.update(|g| {
        g.system = Some("Sol".into());
        g.ship = Some("krait_mkii".into());
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
    });
    let rec = Arc::new(Recorded { offline, ..Recorded::default() });
    let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
    let store = Arc::new(Mutex::new(astromech_store::Store::in_memory().unwrap()));
    let assistant = Assistant::new(runner).with_store(store.clone()).with_remote(client, Arc::new(Evening));
    Rig { assistant, speaker, rec, store, hub }
}

fn spoken(h: Handled) -> String {
    match h {
        Handled::Ran { outcome: Outcome::Spoken(s), .. } => s,
        h => panic!("expected a spoken answer, got {h:?}"),
    }
}

#[test]
fn where_can_i_sell_uses_the_remembered_filters() {
    let r = rig(false);
    // The commander set the Trade page's sell filters: within 40 ly, data up to a week old.
    let f = Filters { max_distance_ly: Some(40), max_age_days: Some(7), ..Filters::default() };
    astromech_galaxy::filters::save(&r.store.lock().unwrap(), "trade.sell", &f).unwrap();

    let s = spoken(r.assistant.handle("Where can I sell painite?", Origin::Voice));
    assert_eq!(s, "Best price for Painite: Good Landing in LHS 1827, 19 light years. 211,587 credits, demand 13,884.");
    assert_eq!(r.speaker.take(), vec![s]);
    let log = r.rec.log.lock().unwrap();
    assert_eq!(log.len(), 1);
}

#[test]
fn where_can_i_buy_answers_the_cheapest_with_its_age() {
    let r = rig(false);
    // No filters saved: 100 ly, any age, the Krait's medium pad.
    let s = spoken(r.assistant.handle("Where can I buy tritium?", Origin::Voice));
    assert_eq!(
        s, "Cheapest Tritium: FCX-656 in Okinura, 32 light years. 2,565 credits, 431 in stock. Reported 69 days ago.",
        "two carriers ask 2,565: the nearer one"
    );
    // Fleet carriers off for buying, and enough stock for a load.
    let f = Filters { exclude_carriers: true, min_volume: Some(100), ..Filters::default() };
    astromech_galaxy::filters::save(&r.store.lock().unwrap(), "trade.buy", &f).unwrap();
    let s = spoken(r.assistant.handle("Where to buy tritium", Origin::Voice));
    assert_eq!(s, "Cheapest Tritium: Ray Freeport in YZ Canis Minoris, 19 light years. 46,968 credits, 1,872 in stock. Reported 5 days ago.");
}

#[test]
fn a_misheard_commodity_asks_first_then_answers() {
    let r = rig(false);
    match r.assistant.handle("Where can I sell pain night?", Origin::Voice) {
        Handled::Asked { command: "trade.sell", phrase } => assert_eq!(phrase, "where can i sell Painite"),
        h => panic!("{h:?}"),
    }
    assert!(r.rec.log.lock().unwrap().is_empty(), "nothing looked up before the yes");
    let s = spoken(r.assistant.handle("yes", Origin::Voice));
    assert!(s.starts_with("Best price for Painite: "), "{s}");
    assert_eq!(r.store.lock().unwrap().aliases().unwrap(), vec![], "a slot phrase is not learned as an alias");
}

#[test]
fn no_location_or_no_network_are_said_plainly() {
    let r = rig(true);
    let s = spoken(r.assistant.handle("Where can I sell painite?", Origin::Voice));
    assert_eq!(
        s,
        "Can't reach Ardent Insight. Check your internet connection; if it's working, Ardent Insight may be down."
    );
    r.hub.update(|g| g.system = None);
    let s = spoken(r.assistant.handle("Where can I sell painite?", Origin::Voice));
    assert_eq!(s, "I don't know where you are yet.");
}

#[test]
fn the_voice_tester_says_what_it_would_look_up_without_asking_ardent() {
    let g = astromech_core::assistant::grammar();
    let game = astromech_gamestate::GameState { system: Some("Sol".into()), ..Default::default() };
    let d = dry_run(&g, "Where can I sell low temperature diamonds?", &game, &BindsState::default(), Source::Typed);
    assert_eq!(d.would, Would::Speak);
    assert_eq!(d.message, "Looks up where to sell Low Temperature Diamonds near you on Ardent Insight.");
    assert_eq!(d.intent.unwrap().command, "trade.sell");
    assert!(d.live.is_none() && d.keys.is_none(), "nothing to press or run live");
}
