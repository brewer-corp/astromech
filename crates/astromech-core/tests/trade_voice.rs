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
use std::time::{Duration, Instant};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("../../astromech-market/tests/fixtures/", $name))
    };
}

/// Ardent's nearby answers from the recordings, or no network at all; logs request URLs and
/// when they were sent.
#[derive(Default)]
struct Recorded {
    offline: bool,
    log: Mutex<Vec<String>>,
    sent: Mutex<Vec<Instant>>,
}

impl Transport for Recorded {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push(url.to_string());
        self.sent.lock().unwrap().push(Instant::now());
        if self.offline {
            return Err(TransportError::Offline("dns error".into()));
        }
        let body = if url.ends_with("/system/name/Sol/commodity/name/painite/nearby/imports") {
            fixture!("ardent-sell-painite-sol.json")
        } else if url.ends_with("/system/name/Sol/commodity/name/platinum/nearby/imports") {
            fixture!("ardent-sell-platinum-sol.json")
        } else if url.ends_with("/system/name/Sol/commodity/name/lowtemperaturediamond/nearby/imports") {
            fixture!("ardent-sell-lowtemperaturediamond-sol.json")
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

/// In Sol in a Krait Mk II (medium pad), on recorded answers with no rate limit.
fn rig(offline: bool) -> Rig {
    let rec = Arc::new(Recorded { offline, ..Recorded::default() });
    let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
    rig_with(rec, Arc::new(client), None)
}

/// [`rig`] with the lookups going through `remote`, and `galaxy` as the star map.
fn rig_with(rec: Arc<Recorded>, remote: Arc<Client>, galaxy: Option<Arc<astromech_galaxy::Galaxy>>) -> Rig {
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
        remote,
        galaxy,
    });
    let store = Arc::new(Mutex::new(astromech_store::Store::in_memory().unwrap()));
    let assistant = Assistant::new(runner).with_store(store.clone()).with_clock(Arc::new(Evening));
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
    // No filters saved: 100 ly, data up to 30 days old, the Krait's medium pad. The carriers
    // asking 2,565 were last reported over two months ago, so they are left out.
    let s = spoken(r.assistant.handle("Where can I buy tritium?", Origin::Voice));
    assert_eq!(
        s,
        "Cheapest Tritium: Bogacki Extraction Territory in Core Sys Sector YU-P a5-3, 30 light years. 44,738 credits, 31 in stock. Reported 25 days ago."
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
    let d = dry_run(&g, "Where should I sell my cargo?", &game, &BindsState::default(), Source::Typed);
    assert_eq!(
        (d.would, d.message.as_str()),
        (Would::Speak, "Looks up where to sell the cargo in your hold near you on Ardent Insight.")
    );
}

#[test]
fn where_should_i_sell_my_cargo_names_the_best_station_for_the_whole_load() {
    let r = rig(false);
    // 200 t painite, 100 t platinum, 300 t LTDs; stolen gold, limpets and mission gold stay.
    let hold = astromech_journal::parse_cargo(fixture!("cargo-mined.json").as_bytes()).unwrap();
    r.hub.update(|g| g.apply_cargo(&hold));

    let s = spoken(r.assistant.handle("Where should I sell my cargo?", Origin::Voice));
    assert_eq!(
        s,
        "Best for your cargo: Good Landing in LHS 1827, 19 light years. 183,674,700 credits for all 600 tonnes."
    );
    assert_eq!(r.rec.log.lock().unwrap().len(), 3, "one lookup per commodity sold");

    // The cargo lookup's own filters (the Trade page's My cargo): within 15 ly, data up to 90
    // days old, only a buyer for part of the platinum is left.
    let f = Filters { max_distance_ly: Some(15), max_age_days: Some(90), ..Filters::default() };
    astromech_galaxy::filters::save(&r.store.lock().unwrap(), "trade.cargo", &f).unwrap();
    let s = spoken(r.assistant.handle("sell my cargo", Origin::Voice));
    assert_eq!(
        s,
        "Best for your cargo: Carvalho Engineering Foundry in Luyten's Star, 12 light years. 18,976,350 credits for 73 of 600 tonnes. Reported 40 days ago. No market takes all your Platinum: Carvalho Engineering Foundry in Luyten's Star takes 73 of 100 tonnes at 259,950 credits each."
    );

    r.hub.update(|g| g.cargo_inventory.retain(|i| i.mission || i.symbol == "drones"));
    let s = spoken(r.assistant.handle("Where to sell my cargo", Origin::Voice));
    assert_eq!(s, "Nothing in your hold to sell: mission cargo, stolen goods and limpets are left out.");
}

/// What a Trade page search for `commodity` near Sol asks for, through `remote` (the star-map
/// fill-in after it is another service).
fn page_search(remote: &Client, r: &Rig, commodity: &str) -> Vec<astromech_market::Prices> {
    let here = astromech_galaxy::Here { system: "Sol".into(), id64: None, pos: [0.0; 3] };
    // The page's filters: the saved ones, the pad following the current ship, like the voice's.
    let pad = astromech_galaxy::Pad::for_ship("krait_mkii");
    let filters = astromech_galaxy::filters::load(&r.store.lock().unwrap(), "trade.sell", pad).unwrap();
    let origin = astromech_market::Origin::from(&here);
    astromech_market::prices(
        remote,
        &origin,
        astromech_market::Side::Sell,
        &[commodity],
        &filters,
        20.0,
        Evening.now_s(),
    )
    .unwrap()
}

/// Done when: the core's one client, made the way the session makes it, serves the voice
/// lookups and the star map: one 1 request/s limit and one cache, in the file the app window
/// opens too, so a voice lookup then the Trade page make one Ardent request.
#[test]
fn the_sessions_voice_lookups_and_star_map_share_one_client() {
    let dir = tempfile::tempdir().unwrap();
    let rec = Arc::new(Recorded::default());
    let (remote, galaxy) = astromech_core::session::remote_data(dir.path(), rec.clone());
    let galaxy = galaxy.expect("the star map opens");
    assert!(std::ptr::eq(galaxy.remote(), &*remote), "the star map uses the process's client");
    let r = rig_with(rec.clone(), remote, Some(galaxy.clone()));

    let voice = spoken(r.assistant.handle("Where can I sell painite?", Origin::Voice));
    assert_eq!(rec.log.lock().unwrap().len(), 1, "the voice lookup asked Ardent once");

    // The same search through the star map's client is served from the voice lookup's answer.
    let page = page_search(galaxy.remote(), &r, "painite");
    assert_eq!(rec.log.lock().unwrap().len(), 1, "served from the shared cache");
    assert_eq!(page[0].offers[0].station, "Good Landing");
    assert!(voice.contains("Good Landing"), "voice and the page name the same best market: {voice}");

    // Another search through the star map waits for the voice lookup's slot: one limit.
    page_search(galaxy.remote(), &r, "platinum");
    let sent = rec.sent.lock().unwrap().clone();
    assert_eq!(sent.len(), 2);
    let gap = sent[1] - sent[0];
    assert!(gap >= Duration::from_millis(950), "Ardent asked twice within {gap:?}");

    // The app window's client on the same data dir is served from the file.
    let window = Client::shared_with_transport(dir.path(), rec.clone(), Arc::new(|_: &str| {}));
    assert_eq!(page_search(&window, &r, "painite")[0].offers[0].station, "Good Landing");
    assert_eq!(rec.log.lock().unwrap().len(), 2, "the app window asked nothing");
    assert_eq!(Cache::open(&dir.path().join(astromech_data::CACHE_FILE)).unwrap().len().unwrap(), 2);
}
