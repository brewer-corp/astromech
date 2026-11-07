//! The window itself, built with the `window` feature (see main.rs).
//!
//! The app window (docs/V1-DESIGN.md §7.1): a thin Tauri v2 shell around the Svelte app in
//! `app/`. It holds no game logic. It talks to `astromech-core` over the core's local control
//! API (`%LOCALAPPDATA%\astromech\data\core.json`: port + token), and:
//! - starts the core if it isn't running (the Start menu shortcut opens this window);
//! - restarts the core if it crashed while the window is open (§7.7 crash restart);
//! - is single-instance: a second start (e.g. from the tray) focuses this window and switches to
//!   the page given with `--route`.
//!
//! Closing the window exits this process and frees WebView2; the core keeps running.

use astromech_api::control;
use astromech_api::state::JournalState;
use astromech_store::Paths;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::Duration;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const CORE_EXE: &str = if cfg!(windows) { "astromech-core.exe" } else { "astromech-core" };

/// The core's app dir (`astromech_store::Paths`, the core's own rule), where its `core.json` is.
fn app_dir() -> PathBuf {
    Paths::current().app_dir().to_path_buf()
}

fn endpoint_file() -> PathBuf {
    app_dir().join(control::FILE)
}

/// A status check: bounded, so the watchdog notices a core that stopped answering.
fn status() -> Result<Value, String> {
    control::call(&app_dir(), control::Request::new("status"), Some(control::SHORT), &mut |_| {})
}

/// One of the page's requests to the core, stamped `origin: "app_window"`, so the core refuses
/// its `local` commands. `text` is the voice tester's phrase or command id, `args` a command's
/// arguments (e.g. `set_ptt` with `{device, key}`). No read timeout: a plot polls Spansh for
/// minutes; a dropped connection means the core died.
fn call_with(cmd: &str, text: Option<&str>, args: Option<Value>) -> Result<Value, String> {
    let req = control::Request {
        text: text.map(String::from),
        args,
        origin: Some(astromech_proto::Origin::AppWindow),
        ..control::Request::new(cmd)
    };
    control::call(&app_dir(), req, None, &mut |_| {})
}

fn core_alive() -> bool {
    status().is_ok()
}

/// Start `astromech-core --background` from next to this exe, without a console window.
fn start_core() -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?.with_file_name(CORE_EXE);
    if !exe.is_file() {
        return Err(format!("{} not found", exe.display()));
    }
    let mut c = std::process::Command::new(exe);
    c.arg("--background");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        c.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }
    c.spawn().map(|_| ()).map_err(|e| e.to_string())
}

/// While the window is open: if the core's endpoint file is there but the core doesn't answer
/// twice in a row, it crashed: start it again. (A clean Quit removes the file, so it stays off.)
fn watchdog(app: tauri::AppHandle) {
    let mut misses = 0;
    loop {
        std::thread::sleep(Duration::from_secs(5));
        if !endpoint_file().exists() {
            misses = 0;
            continue;
        }
        if core_alive() {
            misses = 0;
            continue;
        }
        misses += 1;
        if misses >= 2 {
            misses = 0;
            let r = start_core();
            let _ = app.emit("core-restarted", r.clone().err());
        }
    }
}

// ---- The window's journal and local-file commands are the core's (astromech_api::commands):
// each forwards the page's call unchanged.

/// The page's call to a command the core serves; the reply is the core's, as before.
async fn forward(cmd: &'static str, args: Option<Value>) -> Result<Value, String> {
    blocking(move || call_with(cmd, None, args)).await
}

#[tauri::command]
async fn binds_state() -> Result<Value, String> {
    forward("binds_state", None).await
}

#[tauri::command]
async fn binds_refresh_devices() -> Result<Value, String> {
    forward("binds_refresh_devices", None).await
}

#[tauri::command]
async fn binds_reset() -> Result<Value, String> {
    forward("binds_reset", None).await
}

#[tauri::command]
async fn binds_view(path: PathBuf) -> Result<Value, String> {
    forward("binds_view", Some(json!({ "path": path }))).await
}

#[tauri::command]
async fn binds_set_slot(edit: Value) -> Result<Value, String> {
    forward("binds_set_slot", Some(json!({ "edit": edit }))).await
}

#[tauri::command]
async fn binds_copy(src: PathBuf, name: String) -> Result<Value, String> {
    forward("binds_copy", Some(json!({ "src": src, "name": name }))).await
}

#[tauri::command]
async fn binds_set_active(name: String, contexts: Vec<usize>) -> Result<Value, String> {
    forward("binds_set_active", Some(json!({ "name": name, "contexts": contexts }))).await
}

#[tauri::command]
async fn binds_restore(backup: PathBuf) -> Result<Value, String> {
    forward("binds_restore", Some(json!({ "backup": backup }))).await
}

#[tauri::command]
async fn binds_set_override(device: String, control: String, joy: Option<String>) -> Result<Value, String> {
    forward("binds_set_override", Some(json!({ "device": device, "control": control, "joy": joy }))).await
}

#[tauri::command]
async fn materials_inventory() -> Result<Value, String> {
    forward("materials_inventory", None).await
}

#[tauri::command]
async fn engineering_fleet() -> Result<Value, String> {
    forward("engineering_fleet", None).await
}

#[tauri::command]
async fn engineering_plan(plan: Value) -> Result<Value, String> {
    forward("engineering_plan", Some(json!({ "plan": plan }))).await
}

#[tauri::command]
async fn engineering_stats(plan: Value) -> Result<Value, String> {
    forward("engineering_stats", Some(json!({ "plan": plan }))).await
}

#[tauri::command]
async fn engineering_headlines() -> Result<Value, String> {
    forward("engineering_headlines", None).await
}

#[tauri::command]
async fn engineering_engineers() -> Result<Value, String> {
    forward("engineering_engineers", None).await
}

#[tauri::command]
async fn engineering_unlocks() -> Result<Value, String> {
    forward("engineering_unlocks", None).await
}

#[tauri::command]
async fn odyssey_view() -> Result<Value, String> {
    forward("odyssey_view", None).await
}

#[tauri::command]
async fn odyssey_shopping(picks: Value) -> Result<Value, String> {
    forward("odyssey_shopping", Some(json!({ "picks": picks }))).await
}

#[tauri::command]
async fn powerplay_overview() -> Result<Value, String> {
    forward("powerplay_overview", None).await
}

#[tauri::command]
async fn colonisation_projects() -> Result<Value, String> {
    forward("colonisation_projects", None).await
}

#[tauri::command]
async fn exploration_system() -> Result<Value, String> {
    forward("exploration_system", None).await
}

#[tauri::command]
async fn exploration_unsold() -> Result<Value, String> {
    forward("exploration_unsold", None).await
}

#[tauri::command]
async fn logbook(query: String, cmdr: Option<String>) -> Result<Value, String> {
    forward("logbook", Some(json!({ "query": query, "cmdr": cmdr }))).await
}

#[tauri::command]
async fn session_summary() -> Result<Value, String> {
    forward("session_summary", None).await
}

#[tauri::command]
async fn docked_here() -> Result<Value, String> {
    forward("docked_here", None).await
}

#[tauri::command]
async fn trade_route_defaults() -> Result<Value, String> {
    forward("trade_route_defaults", None).await
}

// ---- The star map, lookups, plotting and the neutron star file, still the window's own until
// they move too (astromech-ndzx).

/// The parts of the core's `astromech.toml` the window reads, from the file the core uses
/// (`Paths::config_in_use`: its `--config`, or the default); defaults if missing.
fn config() -> astromech_store::window::WindowConfig {
    astromech_store::window::WindowConfig::load(&Paths::current())
}

/// The core's data folder (`Config::data_dir()` in the core): every file the window shares with
/// the core (`galaxy.db`, `outfitting.sqlite`, `astromech.sqlite`, the
/// remote-data cache) is opened here.
fn data_dir() -> PathBuf {
    astromech_store::window::data_dir(&Paths::current())
}

/// The game's journal folder, by the core's rule (`Paths::journal_dir`); empty if there is none,
/// which callers report as a folder not found.
fn journal_dir() -> PathBuf {
    Paths::current().journal_dir(&config().game.journal_dir).unwrap_or_default()
}

/// The Powerplay journal, followed: each read takes only the lines added since the last one
/// (the page polls every 2 s while the game appends).
struct PowerplayCache(std::sync::Mutex<Option<astromech_powerplay::Follower>>);

fn powerplay_journal(cache: &PowerplayCache) -> Result<astromech_powerplay::Journal, String> {
    let dir = journal_dir();
    if !dir.is_dir() {
        return Err(format!("journal folder not found: {}", dir.display()));
    }
    let mut f = cache.0.lock().map_err(|_| "busy".to_string())?;
    if f.as_ref().is_some_and(|x| x.dir() != dir.as_path()) {
        *f = None; // the journal folder setting changed
    }
    Ok(f.get_or_insert_with(|| astromech_powerplay::Follower::new(&dir)).refresh().clone())
}

/// The pledged power's nearest acquisition, reinforcement or undermining systems within
/// `within_ly` of `origin` (a typed system name) or, without one, of the commander.
#[tauri::command]
async fn powerplay_targets(
    app: tauri::AppHandle,
    target: astromech_galaxy::Target,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
    within_ly: f64,
) -> Result<Value, String> {
    blocking(move || {
        let j = powerplay_journal(&app.state::<PowerplayCache>())?;
        let power = j.standing.power.clone().ok_or("Not pledged to a power: pledge to one to see its targets.")?;
        let st = engineering_state(&app.state::<EngineeringCache>());
        let g = galaxy(&app)?;
        let here = match origin.as_deref().map(str::trim).filter(|o| !o.is_empty()) {
            Some(name) => g.locate(name).map_err(|e| e.to_string())?,
            None => st.as_ref().map_err(|e| e.clone())?.here.clone().ok_or("no position in the journal yet")?,
        };
        let current = st.as_ref().ok().and_then(|s| s.fleet.ships.iter().find(|s| s.current));
        let within = within_ly.clamp(5.0, 200.0);
        let t = g
            .powerplay_targets(target, &power, &here, range_of(current), within, 10, &filters.unwrap_or_default())
            .map_err(|e| e.to_string())?;
        serde_json::to_value(t).map_err(|e| e.to_string())
    })
    .await
}

/// The journal state the lookups below read (the core's command layer keeps its own for the
/// window's journal commands); rebuilt only when a journal event it reads is appended.
struct EngineeringCache(astromech_journal::JournalCache<JournalState>);

impl Default for EngineeringCache {
    fn default() -> Self {
        Self(JournalState::cache())
    }
}

fn engineering_state(cache: &EngineeringCache) -> Result<std::sync::Arc<JournalState>, String> {
    JournalState::get(&cache.0, &journal_dir())
}

/// The local star map (`galaxy.db` in the core's data folder), seeded from the bundled preload.
/// A failed open is retried on the next call instead of being remembered.
struct GalaxyState(std::sync::Mutex<Option<std::sync::Arc<astromech_galaxy::Galaxy>>>);

/// The app window's one remote-data client, made at startup; every lookup goes through it (via
/// the star map). Its cache is the core's file (in the data folder), so an answer the core fetched
/// for a voice question is served here too. Its problems go to the window's log.
struct Remote(std::sync::Arc<astromech_data::Client>);

fn galaxy(app: &tauri::AppHandle) -> Result<std::sync::Arc<astromech_galaxy::Galaxy>, String> {
    let st = app.state::<GalaxyState>();
    let mut g = st.0.lock().map_err(|_| "busy".to_string())?;
    if let Some(x) = g.as_ref() {
        return Ok(x.clone());
    }
    let preload = app.path().resolve("starmap.db", tauri::path::BaseDirectory::Resource).ok();
    let remote = app.state::<Remote>().0.clone();
    let opened = astromech_galaxy::Galaxy::open(&data_dir().join("galaxy.db"), preload.as_deref(), remote)
        .map(std::sync::Arc::new)
        .map_err(|e| e.to_string())?;
    *g = Some(opened.clone());
    Ok(opened)
}

/// Jump range for counting: the ship's unladen maximum, else a typical 20 ly.
fn range_of(ship: Option<&astromech_engineering::Ship>) -> f64 {
    ship.and_then(|s| s.max_jump_range).filter(|r| *r > 1.0).unwrap_or(20.0)
}

/// Run blocking work (journal reads, network, SQLite) off the main thread.
async fn blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T, String> + Send + 'static) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f).await.map_err(|e| e.to_string())?
}

/// Where a module was seen, from every `Outfitting.json` the core kept (`outfitting.sqlite`, read
/// only): stations, nearest first when their distance from here is known, else newest first.
#[tauri::command]
async fn outfitting_search(app: tauri::AppHandle, query: String) -> Result<Value, String> {
    blocking(move || {
        let book = astromech_station::OutfittingBook::open_read_only(&data_dir().join(astromech_station::DB_FILE))
            .map_err(|e| e.to_string())?;
        let Some(book) = book else {
            return Ok(serde_json::json!({ "stations": 0, "snapshots": 0, "found": [] }));
        };
        let mut found = book.find(&query, 100).map_err(|e| e.to_string())?;
        // Distance from here, where the local star map knows both ends (no network).
        let here = engineering_state(&app.state::<EngineeringCache>()).ok().and_then(|s| s.here.clone());
        if let (Some(h), Ok(g)) = (here, galaxy(&app)) {
            for f in &mut found {
                f.distance_ly = g.position(&f.star_system).ok().flatten().map(|p| astromech_galaxy::dist(h.pos, p));
            }
            found.sort_by(|a, b| match (a.distance_ly, b.distance_ly) {
                (Some(x), Some(y)) => x.total_cmp(&y),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });
        }
        let (stations, snapshots) = book.counts().map_err(|e| e.to_string())?;
        Ok(serde_json::json!({ "stations": stations, "snapshots": snapshots, "found": found }))
    })
    .await
}

/// Which engineers to visit for a plan, in order, from the commander's current system.
#[tauri::command]
async fn engineering_trip(app: tauri::AppHandle, plan: astromech_engineering::Plan) -> Result<Value, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        let ship = st.fleet.ships.iter().find(|s| s.id == plan.ship_id).ok_or("ship not in the fleet")?;
        let result = astromech_engineering::evaluate(ship, &plan, &st.ledger);
        let range = range_of(Some(ship));
        let g = galaxy(&app).ok();
        if let (Some(g), Some(h)) = (&g, &st.here) {
            let _ = g.ensure_covered(h.pos);
        }
        let map = g.as_ref().and_then(|g| g.map().ok());
        let hop = |a: [f64; 3], b: [f64; 3]| {
            let h = match &map {
                Some(m) => m.hop(a, b, range),
                None => {
                    let ly = astromech_galaxy::dist(a, b);
                    astromech_galaxy::Hop { ly, jumps: astromech_galaxy::estimate(ly, range), exact: false }
                }
            };
            astromech_engineering::Hop { ly: h.ly, jumps: h.jumps, exact: h.exact }
        };
        let start = st.here.as_ref().map(|h| (h.system.as_str(), h.pos));
        let trip = astromech_engineering::plan_trip(ship, &result.targets, &st.engineers, start, range, &hop);
        serde_json::to_value(trip).map_err(|e| e.to_string())
    })
    .await
}

/// Where a search starts: `origin` (a typed system name) or, without one, the commander's
/// system; the jump range to count with (the current ship's); and `filters` with the pad
/// following the current ship unless the commander set one (resolved on every search, so a
/// ship change applies at once).
fn search_origin(
    app: &tauri::AppHandle,
    g: &astromech_galaxy::Galaxy,
    origin: Option<&str>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<(astromech_galaxy::Here, f64, astromech_galaxy::Filters), String> {
    let st = engineering_state(&app.state::<EngineeringCache>());
    let here = match origin.map(str::trim).filter(|o| !o.is_empty()) {
        Some(name) => g.locate(name).map_err(|e| e.to_string())?,
        None => st.as_ref().map_err(|e| e.clone())?.here.clone().ok_or("no position in the journal yet")?,
    };
    let current = st.as_ref().ok().and_then(|s| s.fleet.ships.iter().find(|s| s.current));
    let fleet = st.as_ref().ok().map(|s| &s.fleet);
    Ok((here, range_of(current), astromech_galaxy::filters::resolve(filters, fleet)))
}

/// Systems matching a lookup (`famine`, or an HGE pool id) near `origin` (a typed system name)
/// or, without one, near the commander, with jumps at the current ship's range. The pad follows
/// the current ship at search time unless the commander set one.
#[tauri::command]
async fn galaxy_nearby(
    app: tauri::AppHandle,
    query: String,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        if let Ok(s) = engineering_state(&app.state::<EngineeringCache>()) {
            // Best effort: without them the lookup shows Spansh's (older) states.
            let _ = g.record_visits(&s.visits);
        }
        let n = g.nearby(&query, &here, range, 8, &filters).map_err(|e| e.to_string())?;
        serde_json::to_value(n).map_err(|e| e.to_string())
    })
    .await
}

/// The nearest stations offering `want` (a service, module or ship) near `origin` or the
/// commander. The pad follows the current ship at search time unless the commander set one.
#[tauri::command]
async fn stations_nearby(
    app: tauri::AppHandle,
    want: astromech_galaxy::stations::Want,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let n = g.stations(&want, &here, range, 10, &filters).map_err(|e| e.to_string())?;
        serde_json::to_value(n).map_err(|e| e.to_string())
    })
    .await
}

/// Raw-material sites of `kind` (crystalline shards, brain trees, crashed ships, geological
/// sites, the known farms) near `origin` or the commander, with jumps at the current ship's range.
#[tauri::command]
async fn galaxy_sites(
    app: tauri::AppHandle,
    kind: String,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let s = g.sites(&kind, &here, range, 8, &filters).map_err(|e| e.to_string())?;
        serde_json::to_value(s).map_err(|e| e.to_string())
    })
    .await
}

/// Colonisation candidates near `origin` or the commander: unclaimed systems within 15 ly of a
/// populated one, with a body of `interest`, and jumps at the current ship's range.
#[tauri::command]
async fn colony_candidates(
    app: tauri::AppHandle,
    interest: astromech_galaxy::Interest,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let c = g.colony_candidates(interest, &here, range, 10, &filters).map_err(|e| e.to_string())?;
        serde_json::to_value(c).map_err(|e| e.to_string())
    })
    .await
}

/// Where to sell or buy `commodities` (journal symbols) near `origin` or the commander, best
/// price first (Ardent Insight), with jumps at the current ship's range.
#[tauri::command]
async fn trade_search(
    app: tauri::AppHandle,
    side: astromech_market::Side,
    commodities: Vec<String>,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let list: Vec<&str> = commodities.iter().map(String::as_str).collect();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
        let s = astromech_market::search(&g, &here, range, side, &list, &filters, 8, now).map_err(|e| e.to_string())?;
        serde_json::to_value(s).map_err(|e| e.to_string())
    })
    .await
}

/// Where to sell or buy `commodity` (a journal symbol) anywhere in the galaxy, best price first
/// (Ardent Insight's best 100), with distances from `origin` or the commander and jumps
/// estimated at the current ship's range.
#[tauri::command]
async fn trade_anywhere(
    app: tauri::AppHandle,
    side: astromech_market::Side,
    commodity: String,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
        let s = astromech_market::anywhere(g.remote(), &here, range, side, &commodity, &filters, now)
            .map_err(|e| e.to_string())?;
        serde_json::to_value(s).map_err(|e| e.to_string())
    })
    .await
}

/// Where to sell the cargo in the hold (`Cargo.json`) near `origin` or the commander: the best
/// stations for the whole load and the best for each commodity.
#[tauri::command]
async fn trade_cargo(
    app: tauri::AppHandle,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let hold = astromech_market::cargo::from_journal_dir(&journal_dir());
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
        let p = astromech_market::cargo::plan_search(&g, &here, range, &hold, &filters, 8, now)
            .map_err(|e| e.to_string())?;
        Ok(serde_json::json!({ "here": here, "range": range, "plan": p, "hold": hold }))
    })
    .await
}

/// Rings with a hotspot of `want`'s ore (of its ring type, with its reserve) near `origin` (a
/// typed system, or the route's next stop) or the commander, within the finder's radius (100 ly
/// unless the commander picked one), most hotspots first, with jumps at the current ship's
/// range. One Spansh bodies search.
#[tauri::command]
async fn mining_hotspots(
    app: tauri::AppHandle,
    want: astromech_market::mining::Want,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    use astromech_market::mining::{hotspots_search, Order, DEFAULT_RADIUS_LY};
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let radius = filters.max_distance_ly.unwrap_or(DEFAULT_RADIUS_LY);
        let h = hotspots_search(&g, &here, range, &want, Some(radius), Order::Most, 10).map_err(|e| e.to_string())?;
        serde_json::to_value(h).map_err(|e| e.to_string())
    })
    .await
}

/// The station docked at (`Market.json`) and the hold against the best nearby prices: what each
/// carried commodity fetches here and nearby, and what the goods for sale here sell for nearby
/// (Ardent Insight, the sell lookup's remembered filters). `null` when not docked.
#[tauri::command]
async fn market_compare(app: tauri::AppHandle) -> Result<Value, String> {
    blocking(move || {
        let state = astromech_station::state_from_journal_dir(&journal_dir());
        if !state.docked {
            return Ok(Value::Null);
        }
        let market = astromech_market::here::from_journal_dir(&journal_dir())
            .filter(|m| m.market_id.is_some() && m.market_id == state.station_market_id)
            .ok_or("no market seen here yet: open the commodity market once")?;
        let g = galaxy(&app)?;
        let saved = astromech_galaxy::filters::load(&settings()?, astromech_market::Side::Sell.lookup(), None)
            .map_err(|e| e.to_string())?;
        let (here, range, filters) = search_origin(&app, &g, None, Some(saved))?;
        let hold = astromech_market::cargo::from_journal_dir(&journal_dir());
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
        let c = astromech_market::here::compare_search(&g, &here, range, &market, &hold, &filters, now)
            .map_err(|e| e.to_string())?;
        serde_json::to_value(c).map_err(|e| e.to_string())
    })
    .await
}

/// A trade route search is running (one Spansh job at a time, even if the page is left and
/// opened again while the first still polls).
static ROUTE_BUSY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// A route is being plotted for the follower (one at a time, the same way).
static PLOT_BUSY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Holds one of the flags above while its job runs and clears it when the job ends, however it
/// ends. `None` when it is already held.
struct Busy(&'static std::sync::atomic::AtomicBool);

impl Busy {
    fn take(flag: &'static std::sync::atomic::AtomicBool) -> Option<Busy> {
        (!flag.swap(true, std::sync::atomic::Ordering::SeqCst)).then_some(Busy(flag))
    }
}

impl Drop for Busy {
    fn drop(&mut self) {
        self.0.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

/// A single-hop or loop trade route from `origin` (a typed system name) or the commander's
/// system (Spansh's trade router, polled off the UI thread until done or its timeout). A second
/// search while one runs is turned down.
#[tauri::command]
async fn trade_route(
    app: tauri::AppHandle,
    ask: astromech_market::route::Ask,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let Some(_searching) = Busy::take(&ROUTE_BUSY) else {
            return Err("A route search is already running. Wait for it to finish.".into());
        };
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
        let polling = astromech_market::route::Polling::default();
        let r = astromech_market::route::plan(g.remote(), &here.system, range, &ask, &filters, polling, now)
            .map_err(|e| e.to_string())?;
        Ok(serde_json::json!({ "here": here, "route": r }))
    })
    .await
}

/// The current ship as the route plotters see it (its range with a full tank and the cargo
/// aboard now, from the latest `Loadout`), or why there is none, and the system the commander
/// is in.
#[tauri::command]
async fn route_plot_ship(app: tauri::AppHandle) -> Result<Value, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        let ship = plot_ship(&st);
        Ok(serde_json::json!({
            "here": st.here.as_ref().map(|h| h.system.clone()),
            "ship": ship.as_ref().ok(),
            "ship_error": ship.err(),
        }))
    })
    .await
}

fn plot_ship(st: &JournalState) -> Result<astromech_galaxy::plot::Ship, String> {
    let cargo = astromech_station::state_from_journal_dir(&journal_dir()).cargo_tonnes.unwrap_or(0.0);
    astromech_galaxy::plot::Ship::current(&st.fleet, cargo)
}

/// Plot `ask` for the current ship from the commander's system off the
/// UI thread, and hand the route to the core's follower. A neutron route is plotted on this PC
/// from the kept neutron star file when it is there and readable, else by Spansh
/// (`astromech_galaxy::plot::plot_route`); exact routes come from Spansh. The route is set even
/// if the page is left meanwhile. A second plot while one runs is turned down.
#[tauri::command]
async fn route_plot(app: tauri::AppHandle, ask: astromech_galaxy::plot::Ask) -> Result<Value, String> {
    blocking(move || {
        let Some(_plotting) = Busy::take(&PLOT_BUSY) else {
            return Err("A route is already being plotted. Wait for it to finish.".into());
        };
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        let ship = plot_ship(&st)?;
        let remote = app.state::<Remote>().0.clone();
        let polling = astromech_data::spansh::Polling::default();
        let locate = |name: &str| -> anyhow::Result<astromech_galaxy::Here> {
            galaxy(&app).map_err(anyhow::Error::msg)?.locate(name)
        };
        let neutron = app.state::<NeutronKept>();
        let plotted =
            astromech_galaxy::plot::plot_route(&remote, &neutron.0, locate, &ask, st.here.as_ref(), &ship, polling)?;
        if let Some(why) = &plotted.fallback {
            astromech_log::log(&format!("neutron star file: can't be read, Spansh plotted the route: {why}"));
        }
        let args = serde_json::to_value(&plotted).map_err(|e| e.to_string())?;
        let r = call_with("route_set", None, Some(args))
            .map_err(|e| format!("Plotted, but the route could not be followed: {e}."))?;
        if r["ok"] != true {
            return Err(format!(
                "Plotted, but the route could not be followed: {}.",
                r["error"].as_str().unwrap_or("no answer")
            ));
        }
        Ok(serde_json::json!({ "plotted": plotted, "view": r["view"] }))
    })
    .await
}

// ---- The neutron star file (astromech_galaxy::neutron) in the data folder, downloaded only
// when the commander presses Download on the Route page.

/// The neutron star file in the data folder, loaded on the first local neutron plot and kept
/// (about 80 MB), and loaded again when a download replaces it.
struct NeutronKept(astromech_galaxy::neutron::Kept);

/// What the Route page shows: the file's header if it is there, and why it can't be read if not
/// (also when it failed to load for a plot).
fn neutron_status(app: &tauri::AppHandle) -> Value {
    match app.state::<NeutronKept>().0.installed() {
        Ok(file) => serde_json::json!({ "file": file, "error": null }),
        Err(e) => serde_json::json!({ "file": null, "error": format!("{e:#}") }),
    }
}

#[tauri::command]
async fn neutron_file(app: tauri::AppHandle) -> Result<Value, String> {
    blocking(move || Ok(neutron_status(&app))).await
}

#[derive(Clone, serde::Serialize)]
struct NeutronProgress {
    done: u64,
    total: Option<u64>,
}

/// Downloads the latest file from the `neutron` pre-release (about 70 MB), reporting progress
/// about once a megabyte. The old file, if any, stays until the new one is complete and checked.
/// A second download while one runs is turned down (by `neutron::download`). The new file is
/// loaded straight away, so the next plot uses it and a file that won't load says why.
#[tauri::command]
async fn neutron_download(
    app: tauri::AppHandle,
    on_progress: tauri::ipc::Channel<NeutronProgress>,
) -> Result<Value, String> {
    blocking(move || {
        let mut sent = 0u64;
        let progress = |done: u64, total: Option<u64>| {
            if done - sent >= 1 << 20 || total == Some(done) {
                sent = done;
                let _ = on_progress.send(NeutronProgress { done, total });
            }
        };
        let url = astromech_galaxy::neutron::RELEASE_URL;
        if let Err(e) = astromech_galaxy::neutron::download(&data_dir(), url, progress) {
            astromech_log::log(&format!("neutron star file: download failed: {e:#}{}", download_detail(&e)));
            return Err(format!("{e:#}"));
        }
        if let Err(e) = app.state::<NeutronKept>().0.get() {
            astromech_log::log(&format!("neutron star file: the download won't load: {e:#}"));
        }
        Ok(neutron_status(&app))
    })
    .await
}

/// Why a download couldn't reach the server, as " (cause)" for the log; the page gets only the
/// sentence.
fn download_detail(e: &anyhow::Error) -> String {
    e.downcast_ref::<astromech_data::download::DownloadError>()
        .and_then(|d| d.detail())
        .map(|d| format!(" ({d})"))
        .unwrap_or_default()
}

/// Whether the published file differs from the one downloaded (a small checksum file, no download).
#[tauri::command]
async fn neutron_check() -> Result<Value, String> {
    blocking(|| {
        let newer = astromech_galaxy::neutron::newer_available(&data_dir(), astromech_galaxy::neutron::RELEASE_URL)
            .map_err(|e| {
                astromech_log::log(&format!("neutron star file: check failed: {e:#}{}", download_detail(&e)));
                format!("{e:#}")
            })?;
        Ok(serde_json::json!({ "newer": newer }))
    })
    .await
}

/// Where to buy `needs` (a project's shopping list) near `origin` (the construction site's
/// system, a typed one) or the commander: stations stocking most of the list first (Ardent).
#[tauri::command]
async fn colonisation_buy(
    app: tauri::AppHandle,
    needs: Vec<astromech_market::list::Need>,
    origin: Option<String>,
    filters: Option<astromech_galaxy::Filters>,
) -> Result<Value, String> {
    blocking(move || {
        let g = galaxy(&app)?;
        let (here, range, filters) = search_origin(&app, &g, origin.as_deref(), filters)?;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
        let p = astromech_market::list::plan_search(&g, &here, range, &needs, &filters, 12, now)
            .map_err(|e| e.to_string())?;
        Ok(serde_json::json!({ "here": here, "range": range, "plan": p }))
    })
    .await
}

/// The settings store, `astromech.sqlite` in the core's data folder, so filters land in the file
/// the core uses.
fn settings() -> Result<astromech_store::Store, String> {
    let dir = data_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    astromech_store::Store::open(&dir.join("astromech.sqlite")).map_err(|e| e.to_string())
}

/// A search's remembered filters, with the pad taken from the current ship unless set.
#[tauri::command]
async fn search_filters(app: tauri::AppHandle, lookup: String) -> Result<astromech_galaxy::Filters, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>()).ok();
        astromech_galaxy::filters::load_for_fleet(&settings()?, &lookup, st.as_ref().map(|s| &s.fleet))
            .map_err(|e| e.to_string())
    })
    .await
}

#[tauri::command]
async fn set_search_filters(lookup: String, filters: astromech_galaxy::Filters) -> Result<(), String> {
    blocking(move || astromech_galaxy::filters::save(&settings()?, &lookup, &filters).map_err(|e| e.to_string())).await
}

/// System names resembling `prefix` (Spansh's fuzzy autocomplete), for the search origin box.
#[tauri::command]
async fn galaxy_systems(app: tauri::AppHandle, prefix: String) -> Result<Vec<String>, String> {
    blocking(move || galaxy(&app)?.system_names(&prefix).map_err(|e| e.to_string())).await
}

/// Presses from every stick, pad and key, for "press a control to find it", the key tester
/// and press-to-bind. Read-only (Raw Input with INPUTSINK + XInput polling, like the core).
#[cfg(windows)]
fn start_input(app: tauri::AppHandle) {
    use astromech_input::listen::{tracker::Processor, win};
    std::thread::spawn(move || {
        let start = std::time::Instant::now();
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = win::rawinput::start(tx.clone(), start);
        let _ = win::xinput::start(tx, start);
        let mut p = Processor::new(astromech_input::ELITE_TITLE, None, true);
        for d in win::rawinput::list_devices() {
            p.add_device(d);
        }
        loop {
            let evs = match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(win::Msg::Device(d)) => {
                    p.add_device(d);
                    continue;
                }
                Ok(win::Msg::Sample(s)) => p.process(&s),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => p.tick(start.elapsed().as_secs_f64() * 1000.0),
                Err(_) => break,
            };
            p.events.clear();
            for e in evs {
                let device = match e.kind.as_str() {
                    "key" => "Keyboard".to_string(),
                    "mouse" => "Mouse".to_string(),
                    "xinput" => "GamePad".to_string(),
                    _ => e.elite_id.clone().unwrap_or_default(),
                };
                let Some(key) = e.elite_name.clone() else { continue };
                if e.kind != "axis" && e.down != Some(true) && e.down != Some(false) {
                    continue;
                }
                let _ = app.emit(
                    "input",
                    serde_json::json!({
                        "device": device,
                        "key": key,
                        "kind": e.kind,
                        "down": e.down,
                        "device_name": e.device,
                        "t_ms": e.t_ms,
                    }),
                );
            }
        }
    });
}

#[tauri::command]
async fn core_status() -> Option<Value> {
    blocking(|| Ok(status().ok())).await.ok().flatten()
}

/// Blocking: a voice tester live run waits for Elite to come to the front; calibration and the
/// mic check record for a few seconds.
#[tauri::command]
async fn core_call(cmd: String, text: Option<String>, args: Option<Value>) -> Value {
    blocking(move || {
        Ok(call_with(&cmd, text.as_deref(), args).unwrap_or_else(|e| serde_json::json!({ "ok": false, "error": e })))
    })
    .await
    .unwrap_or_else(|e| serde_json::json!({ "ok": false, "error": e }))
}

#[tauri::command]
fn shell_info() -> Value {
    serde_json::json!({ "version": env!("CARGO_PKG_VERSION") })
}

/// Open the project's GitHub pages or a Coriolis / EDSY / Spansh / Inara link in the browser, or a
/// folder under the core's app, data or log folder.
#[tauri::command]
fn open_external(target: String) -> Result<(), String> {
    let ok_url = astromech_engineering::links::is_openable_url(&target);
    let ok_dir = {
        let paths = Paths::current();
        let roots = [paths.app_dir().to_path_buf(), paths.log_dir(), data_dir()];
        let p = PathBuf::from(&target);
        p.is_dir()
            && p.canonicalize()
                .is_ok_and(|p| roots.iter().filter_map(|r| r.canonicalize().ok()).any(|r| p.starts_with(r)))
    };
    if !ok_url && !ok_dir {
        return Err("not allowed".into());
    }
    #[cfg(windows)]
    let r = std::process::Command::new("explorer.exe").arg(&target).spawn();
    #[cfg(not(windows))]
    let r = std::process::Command::new("xdg-open").arg(&target).spawn();
    r.map(|_| ()).map_err(|e| e.to_string())
}

fn route_arg(args: &[String]) -> Option<String> {
    let i = args.iter().position(|a| a == "--route")?;
    args.get(i + 1).filter(|r| r.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')).cloned()
}

/// `--webview-debug-port N`: open WebView2's DevTools port (127.0.0.1) for CI's onboarding
/// smoke test (scripts/onboarding_smoke.py). A flag, not an environment variable, so it can't
/// linger in a user's environment.
fn debug_port_arg(args: &[String]) -> Option<u16> {
    let i = args.iter().position(|a| a == "--webview-debug-port")?;
    args.get(i + 1)?.parse().ok()
}

/// When this process started, for the cold-start line (the WebView2 cost harness reads it).
static STARTED: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

pub fn main() {
    STARTED.get_or_init(std::time::Instant::now);
    // Release builds have no console, so the window's problems go to its own log in the core's
    // log folder (`astromech-shell.log`).
    let red = astromech_keybinds::report::Redactor::from_env();
    astromech_log::init(
        &Paths::current().log_dir(),
        astromech_log::SHELL,
        cfg!(debug_assertions),
        Box::new(move |l: &str| red.apply(l)),
    );
    let args: Vec<String> = std::env::args().collect();
    let route = route_arg(&args);
    let debug_port = debug_port_arg(&args);
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
            if let Some(r) = route_arg(&argv) {
                match app.emit("route", &r) {
                    Ok(()) => astromech_log::log(&format!("window: page {r} asked for")),
                    Err(e) => astromech_log::log(&format!("window: couldn't switch to page {r}: {e}")),
                }
            }
        }))
        .manage(EngineeringCache::default())
        .manage(GalaxyState(std::sync::Mutex::new(None)))
        .manage(NeutronKept(astromech_galaxy::neutron::Kept::new(&data_dir())))
        .manage(Remote(astromech_data::Client::shared(&data_dir(), std::sync::Arc::new(astromech_log::log))))
        .manage(PowerplayCache(std::sync::Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            core_status,
            core_call,
            shell_info,
            open_external,
            binds_state,
            binds_refresh_devices,
            binds_reset,
            binds_view,
            binds_set_slot,
            binds_copy,
            binds_set_active,
            binds_restore,
            binds_set_override,
            materials_inventory,
            engineering_fleet,
            engineering_plan,
            engineering_stats,
            engineering_headlines,
            engineering_engineers,
            engineering_unlocks,
            engineering_trip,
            odyssey_view,
            odyssey_shopping,
            galaxy_nearby,
            galaxy_sites,
            colony_candidates,
            stations_nearby,
            galaxy_systems,
            trade_search,
            trade_anywhere,
            trade_cargo,
            mining_hotspots,
            market_compare,
            trade_route_defaults,
            trade_route,
            route_plot_ship,
            route_plot,
            neutron_file,
            neutron_download,
            neutron_check,
            colonisation_projects,
            colonisation_buy,
            search_filters,
            set_search_filters,
            powerplay_overview,
            powerplay_targets,
            logbook,
            docked_here,
            outfitting_search,
            session_summary,
            exploration_system,
            exploration_unsold
        ])
        .setup(move |app| {
            let hash = route.as_deref().map(|r| format!("index.html#/{r}")).unwrap_or_else(|| "index.html".into());
            let mut window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App(hash.into()))
                .title("astromech")
                .inner_size(1200.0, 800.0)
                .min_inner_size(800.0, 500.0)
                .on_page_load(|_, p| {
                    // The first page load is the end of a cold start (scripts/webview_cost.py).
                    static LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                    if p.event() == tauri::webview::PageLoadEvent::Finished
                        && !LOGGED.swap(true, std::sync::atomic::Ordering::SeqCst)
                    {
                        let ms = STARTED.get().map_or(0, |t| t.elapsed().as_millis());
                        astromech_log::log(&format!("window: page loaded {ms} ms after start"));
                    }
                });
            // CI's onboarding smoke test drives the page over the DevTools protocol. WebView2
            // ignores WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS when the app passes its own
            // arguments, so the port is asked for here; wry's defaults are kept.
            if let Some(port) = debug_port {
                window = window.additional_browser_args(&format!(
                    "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection \
                     --autoplay-policy=no-user-gesture-required --remote-debugging-port={port}"
                ));
            }
            window.build()?;
            // Off the setup path, which holds up the first page: a crashed core leaves core.json
            // behind, and connecting to its dead port takes about 2 s on Windows (astromech-k8yk).
            let h = app.handle().clone();
            std::thread::spawn(move || {
                if !core_alive() {
                    if let Err(e) = start_core() {
                        astromech_log::log(&format!("couldn't start the core: {e}"));
                    }
                }
                watchdog(h)
            });
            #[cfg(windows)]
            start_input(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the astromech app window");
}
