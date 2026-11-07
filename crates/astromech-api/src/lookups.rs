//! The window's star-map, lookup and route-plotting commands, the neutron star file, and
//! `open_external` and `shell_info`, backed by [`State`]. Lookups go through the state's one
//! remote-data client and star map, which the session's voice lookups share. Each is a `window`
//! command with typed arguments (the window's names, in snake_case) and a typed reply.
//!
//! Where a search starts is `origin` (a typed system name) or, without one, the commander's
//! system; jumps are counted at the current ship's range, and the pad follows the current ship
//! unless the commander set one (resolved on every search, so a ship change applies at once).

use crate::state::{Busy, JournalState, State};
use crate::{Call, Exposure::Window, Registry};
use astromech_galaxy::{Filters, Here};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// The common arguments of a search near somewhere.
#[derive(Deserialize)]
struct Near<W> {
    #[serde(flatten)]
    what: W,
    origin: Option<String>,
    filters: Option<Filters>,
}

#[derive(Deserialize)]
struct QueryArg {
    query: String,
}

#[derive(Deserialize)]
struct WantArg<W> {
    want: W,
}

#[derive(Deserialize)]
struct KindArg {
    kind: String,
}

#[derive(Deserialize)]
struct InterestArg {
    interest: astromech_galaxy::Interest,
}

#[derive(Deserialize)]
struct TradeArgs {
    side: astromech_market::Side,
    commodities: Vec<String>,
}

#[derive(Deserialize)]
struct AnywhereArgs {
    side: astromech_market::Side,
    commodity: String,
}

#[derive(Deserialize)]
struct AskArg<A> {
    ask: A,
}

#[derive(Deserialize)]
struct NeedsArg {
    needs: Vec<astromech_market::list::Need>,
}

#[derive(Deserialize)]
struct Nothing {}

#[derive(Deserialize)]
struct TargetsArgs {
    target: astromech_galaxy::Target,
    origin: Option<String>,
    filters: Option<Filters>,
    within_ly: f64,
}

#[derive(Deserialize)]
struct PlanArg {
    plan: astromech_engineering::Plan,
}

#[derive(Deserialize)]
struct LookupArg {
    lookup: String,
}

#[derive(Deserialize)]
struct SetFiltersArgs {
    lookup: String,
    filters: Filters,
}

#[derive(Deserialize)]
struct PrefixArg {
    prefix: String,
}

#[derive(Deserialize)]
struct TargetArg {
    target: String,
}

/// Where a module was seen, from every `Outfitting.json` the core kept.
#[derive(Serialize)]
pub struct Outfitting {
    pub stations: i64,
    pub snapshots: i64,
    pub found: Vec<astromech_station::Seen>,
}

/// Where to sell the cargo in the hold.
#[derive(Serialize)]
pub struct CargoSale {
    pub here: Here,
    pub range: f64,
    pub plan: astromech_market::cargo::CargoPlan,
    pub hold: Vec<astromech_journal::CargoItem>,
}

/// A trade route from where the search started.
#[derive(Serialize)]
pub struct TradeRoute {
    pub here: Here,
    pub route: astromech_market::route::Route,
}

/// The current ship as the route plotters see it, or why there is none, and the system the
/// commander is in.
#[derive(Serialize)]
pub struct PlotShip {
    pub here: Option<String>,
    pub ship: Option<astromech_galaxy::plot::Ship>,
    pub ship_error: Option<String>,
}

/// A plotted route and the follower's view once it took it.
#[derive(Serialize)]
pub struct RoutePlotted {
    pub plotted: astromech_galaxy::plot::Plotted,
    pub view: Value,
}

/// What the Route page shows of the neutron star file: its header if it is there, and why it
/// can't be read if not (also when it failed to load for a plot).
#[derive(Serialize)]
pub struct NeutronFile {
    pub file: Option<astromech_galaxy::neutron::Header>,
    pub error: Option<String>,
}

/// A download's progress, about once a megabyte and at the end.
#[derive(Serialize)]
pub struct NeutronProgress {
    pub done: u64,
    pub total: Option<u64>,
}

#[derive(Serialize)]
pub struct NeutronCheck {
    pub newer: bool,
}

/// Where to buy a project's shopping list.
#[derive(Serialize)]
pub struct ShoppingPlan {
    pub here: Here,
    pub range: f64,
    pub plan: astromech_market::list::ListPlan,
}

#[derive(Serialize)]
pub struct ShellInfo {
    pub version: &'static str,
}

/// Jump range for counting: the ship's unladen maximum, else a typical 20 ly.
fn range_of(ship: Option<&astromech_engineering::Ship>) -> f64 {
    ship.and_then(|s| s.max_jump_range).filter(|r| *r > 1.0).unwrap_or(20.0)
}

fn now() -> i64 {
    use astromech_data::Clock;
    astromech_data::SystemClock.now_s()
}

fn text(e: impl std::fmt::Display) -> String {
    e.to_string()
}

/// Where a search starts, the jump range to count with and the resolved filters.
fn search_origin(
    s: &State,
    g: &astromech_galaxy::Galaxy,
    origin: Option<&str>,
    filters: Option<Filters>,
) -> Result<(Here, f64, Filters), String> {
    let st = s.engineering();
    let here = match origin.map(str::trim).filter(|o| !o.is_empty()) {
        Some(name) => g.locate(name).map_err(text)?,
        None => st.as_ref().map_err(|e| e.clone())?.here.clone().ok_or("no position in the journal yet")?,
    };
    let current = st.as_ref().ok().and_then(|s| s.fleet.ships.iter().find(|s| s.current));
    let fleet = st.as_ref().ok().map(|s| &s.fleet);
    Ok((here, range_of(current), astromech_galaxy::filters::resolve(filters, fleet)))
}

/// The star map and where a search near somewhere starts.
fn near<W>(s: &State, n: &Near<W>) -> Result<(Arc<astromech_galaxy::Galaxy>, Here, f64, Filters), String> {
    let g = s.galaxy()?;
    let (here, range, filters) = search_origin(s, &g, n.origin.as_deref(), n.filters.clone())?;
    Ok((g, here, range, filters))
}

fn plot_ship(s: &State, st: &JournalState) -> Result<astromech_galaxy::plot::Ship, String> {
    let cargo = astromech_station::state_from_journal_dir(&s.journal_dir()).cargo_tonnes.unwrap_or(0.0);
    astromech_galaxy::plot::Ship::current(&st.fleet, cargo)
}

fn neutron_status(s: &State) -> NeutronFile {
    match s.neutron.installed() {
        Ok(file) => NeutronFile { file, error: None },
        Err(e) => NeutronFile { file: None, error: Some(format!("{e:#}")) },
    }
}

/// Why a download couldn't reach the server, as " (cause)" for the log; the page gets only the
/// sentence.
fn download_detail(e: &anyhow::Error) -> String {
    e.downcast_ref::<astromech_data::download::DownloadError>()
        .and_then(|d| d.detail())
        .map(|d| format!(" ({d})"))
        .unwrap_or_default()
}

/// Plot `ask` for the current ship from the commander's system and hand the route to the
/// follower (`route_set`, through the registry). A neutron route is plotted on this PC from the
/// kept neutron star file when it is there and readable, else by Spansh; exact routes come from
/// Spansh. A second plot while one runs is turned down.
fn route_plot(s: &State, ask: &astromech_galaxy::plot::Ask, call: &Call) -> Result<RoutePlotted, String> {
    let Some(_plotting) = Busy::take(&s.plot_busy) else {
        return Err("A route is already being plotted. Wait for it to finish.".into());
    };
    let st = s.engineering()?;
    let ship = plot_ship(s, &st)?;
    let locate = |name: &str| -> anyhow::Result<Here> { s.galaxy().map_err(anyhow::Error::msg)?.locate(name) };
    let plotted = astromech_galaxy::plot::plot_route(
        s.remote(),
        &s.neutron,
        locate,
        ask,
        st.here.as_ref(),
        &ship,
        s.plot_polling,
    )?;
    if let Some(why) = &plotted.fallback {
        astromech_log::log(&format!("neutron star file: can't be read, Spansh plotted the route: {why}"));
    }
    let args = serde_json::to_value(&plotted).map_err(text)?;
    let mut r =
        call.run("route_set", Some(args)).map_err(|e| format!("Plotted, but the route could not be followed: {e}."))?;
    if r["ok"] != true {
        return Err(format!(
            "Plotted, but the route could not be followed: {}.",
            r["error"].as_str().unwrap_or("no answer")
        ));
    }
    Ok(RoutePlotted { plotted, view: r["view"].take() })
}

/// Download the latest neutron star file (about 70 MB), sending progress about once a megabyte.
/// The old file, if any, stays until the new one is complete and checked. A second download
/// while one runs is turned down (by `neutron::download`). The new file is loaded straight away,
/// so the next plot uses it and a file that won't load says why.
fn neutron_download(s: &State, call: &Call) -> Result<NeutronFile, String> {
    let mut sent = 0u64;
    let progress = |done: u64, total: Option<u64>| {
        if done - sent >= 1 << 20 || total == Some(done) {
            sent = done;
            call.progress(serde_json::to_value(NeutronProgress { done, total }).unwrap_or_default());
        }
    };
    if let Err(e) = astromech_galaxy::neutron::download(s.data_dir(), &s.neutron_url, progress) {
        astromech_log::log(&format!("neutron star file: download failed: {e:#}{}", download_detail(&e)));
        return Err(format!("{e:#}"));
    }
    if let Err(e) = s.neutron.get() {
        astromech_log::log(&format!("neutron star file: the download won't load: {e:#}"));
    }
    Ok(neutron_status(s))
}

/// Register the commands on `reg`, all sharing `state`.
pub fn register(state: &Arc<State>, reg: &mut Registry) {
    // Systems matching a lookup (`famine`, or an HGE pool id).
    let s = state.clone();
    reg.add_typed("galaxy_nearby", Window, move |x: Near<QueryArg>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        if let Ok(st) = s.engineering() {
            // Best effort: without them the lookup shows Spansh's (older) states.
            let _ = g.record_visits(&st.visits);
        }
        g.nearby(&x.what.query, &here, range, 8, &filters).map_err(text)
    });

    // The nearest stations offering `want` (a service, module or ship).
    let s = state.clone();
    reg.add_typed("stations_nearby", Window, move |x: Near<WantArg<astromech_galaxy::stations::Want>>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        g.stations(&x.what.want, &here, range, 10, &filters).map_err(text)
    });

    // Raw-material sites of `kind` (crystalline shards, brain trees, crashed ships, geological
    // sites, the known farms).
    let s = state.clone();
    reg.add_typed("galaxy_sites", Window, move |x: Near<KindArg>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        g.sites(&x.what.kind, &here, range, 8, &filters).map_err(text)
    });

    // Colonisation candidates: unclaimed systems within 15 ly of a populated one, with a body of
    // `interest`.
    let s = state.clone();
    reg.add_typed("colony_candidates", Window, move |x: Near<InterestArg>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        g.colony_candidates(x.what.interest, &here, range, 10, &filters).map_err(text)
    });

    // Where to sell or buy `commodities` (journal symbols), best price first (Ardent Insight).
    let s = state.clone();
    reg.add_typed("trade_search", Window, move |x: Near<TradeArgs>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        let list: Vec<&str> = x.what.commodities.iter().map(String::as_str).collect();
        astromech_market::search(&g, &here, range, x.what.side, &list, &filters, 8, now()).map_err(text)
    });

    // Where to sell or buy `commodity` anywhere in the galaxy, best price first (Ardent
    // Insight's best 100), with distances from the origin and jumps estimated.
    let s = state.clone();
    reg.add_typed("trade_anywhere", Window, move |x: Near<AnywhereArgs>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        astromech_market::anywhere(g.remote(), &here, range, x.what.side, &x.what.commodity, &filters, now())
            .map_err(text)
    });

    // Where to sell the cargo in the hold (`Cargo.json`): the best stations for the whole load
    // and the best for each commodity.
    let s = state.clone();
    reg.add_typed("trade_cargo", Window, move |x: Near<Nothing>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        let hold = astromech_market::cargo::from_journal_dir(&s.journal_dir());
        let plan = astromech_market::cargo::plan_search(&g, &here, range, &hold, &filters, 8, now()).map_err(text)?;
        Ok(CargoSale { here, range, plan, hold })
    });

    // Rings with a hotspot of `want`'s ore within the finder's radius (100 ly unless the
    // commander picked one), most hotspots first. One Spansh bodies search.
    let s = state.clone();
    reg.add_typed("mining_hotspots", Window, move |x: Near<WantArg<astromech_market::mining::Want>>, _| {
        use astromech_market::mining::{hotspots_search, Order, DEFAULT_RADIUS_LY};
        let (g, here, range, filters) = near(&s, &x)?;
        let radius = filters.max_distance_ly.unwrap_or(DEFAULT_RADIUS_LY);
        hotspots_search(&g, &here, range, &x.what.want, Some(radius), Order::Most, 10).map_err(text)
    });

    // The station docked at (`Market.json`) and the hold against the best nearby prices, with
    // the sell lookup's remembered filters. `null` when not docked.
    let s = state.clone();
    reg.add_typed("market_compare", Window, move |(), _| {
        let dir = s.journal_dir();
        let state = astromech_station::state_from_journal_dir(&dir);
        if !state.docked {
            return Ok(None);
        }
        let market = astromech_market::here::from_journal_dir(&dir)
            .filter(|m| m.market_id.is_some() && m.market_id == state.station_market_id)
            .ok_or("no market seen here yet: open the commodity market once")?;
        let g = s.galaxy()?;
        let saved = astromech_galaxy::filters::load(&s.settings()?, astromech_market::Side::Sell.lookup(), None)
            .map_err(text)?;
        let (here, range, filters) = search_origin(&s, &g, None, Some(saved))?;
        let hold = astromech_market::cargo::from_journal_dir(&dir);
        astromech_market::here::compare_search(&g, &here, range, &market, &hold, &filters, now())
            .map(Some)
            .map_err(text)
    });

    // A single-hop or loop trade route (Spansh's trade router, polled until done or its
    // timeout). A second search while one runs is turned down.
    let s = state.clone();
    reg.add_typed("trade_route", Window, move |x: Near<AskArg<astromech_market::route::Ask>>, _| {
        let Some(_searching) = Busy::take(&s.route_busy) else {
            return Err("A route search is already running. Wait for it to finish.".to_string());
        };
        let (g, here, range, filters) = near(&s, &x)?;
        let polling = astromech_market::route::Polling::default();
        let route =
            astromech_market::route::plan(g.remote(), &here.system, range, &x.what.ask, &filters, polling, now())
                .map_err(text)?;
        Ok(TradeRoute { here, route })
    });

    // Where to buy `needs` (a project's shopping list) near the construction site's system or
    // the commander: stations stocking most of the list first (Ardent).
    let s = state.clone();
    reg.add_typed("colonisation_buy", Window, move |x: Near<NeedsArg>, _| {
        let (g, here, range, filters) = near(&s, &x)?;
        let plan =
            astromech_market::list::plan_search(&g, &here, range, &x.what.needs, &filters, 12, now()).map_err(text)?;
        Ok(ShoppingPlan { here, range, plan })
    });

    // The pledged power's nearest acquisition, reinforcement or undermining systems within
    // `within_ly` of the origin.
    let s = state.clone();
    reg.add_typed("powerplay_targets", Window, move |x: TargetsArgs, _| {
        let j = s.powerplay()?;
        let power = j.standing.power.clone().ok_or("Not pledged to a power: pledge to one to see its targets.")?;
        let st = s.engineering();
        let g = s.galaxy()?;
        let here = match x.origin.as_deref().map(str::trim).filter(|o| !o.is_empty()) {
            Some(name) => g.locate(name).map_err(text)?,
            None => st.as_ref().map_err(|e| e.clone())?.here.clone().ok_or("no position in the journal yet")?,
        };
        let current = st.as_ref().ok().and_then(|s| s.fleet.ships.iter().find(|s| s.current));
        let within = x.within_ly.clamp(5.0, 200.0);
        let filters = x.filters.unwrap_or_default();
        g.powerplay_targets(x.target, &power, &here, range_of(current), within, 10, &filters).map_err(text)
    });

    // Which engineers to visit for a plan, in order, from the commander's current system.
    let s = state.clone();
    reg.add_typed("engineering_trip", Window, move |x: PlanArg, _| {
        let st = s.engineering()?;
        let ship = st.fleet.ships.iter().find(|s| s.id == x.plan.ship_id).ok_or("ship not in the fleet")?;
        let result = astromech_engineering::evaluate(ship, &x.plan, &st.ledger);
        let range = range_of(Some(ship));
        let g = s.galaxy().ok();
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
        Ok(astromech_engineering::plan_trip(ship, &result.targets, &st.engineers, start, range, &hop))
    });

    // Where a module was seen, from every `Outfitting.json` the core kept (`outfitting.sqlite`,
    // read only): stations, nearest first when their distance from here is known (from the
    // local star map, no network), else newest first.
    let s = state.clone();
    reg.add_typed("outfitting_search", Window, move |x: QueryArg, _| {
        let book = astromech_station::OutfittingBook::open_read_only(&s.data_dir().join(astromech_station::DB_FILE))
            .map_err(text)?;
        let Some(book) = book else {
            return Ok(Outfitting { stations: 0, snapshots: 0, found: vec![] });
        };
        let mut found = book.find(&x.query, 100).map_err(text)?;
        let here = s.engineering().ok().and_then(|st| st.here.clone());
        if let (Some(h), Ok(g)) = (here, s.galaxy()) {
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
        let (stations, snapshots) = book.counts().map_err(text)?;
        Ok(Outfitting { stations, snapshots, found })
    });

    // A search's remembered filters, with the pad taken from the current ship unless set.
    let s = state.clone();
    reg.add_typed("search_filters", Window, move |x: LookupArg, _| {
        let st = s.engineering().ok();
        astromech_galaxy::filters::load_for_fleet(&s.settings()?, &x.lookup, st.as_ref().map(|s| &s.fleet))
            .map_err(text)
    });
    let s = state.clone();
    reg.add_typed("set_search_filters", Window, move |x: SetFiltersArgs, _| {
        astromech_galaxy::filters::save(&s.settings()?, &x.lookup, &x.filters).map_err(text)
    });

    // System names resembling `prefix` (Spansh's fuzzy autocomplete), for the search origin box.
    let s = state.clone();
    reg.add_typed("galaxy_systems", Window, move |x: PrefixArg, _| s.galaxy()?.system_names(&x.prefix).map_err(text));

    // The current ship as the route plotters see it (its range with a full tank and the cargo
    // aboard now, from the latest `Loadout`), or why there is none.
    let s = state.clone();
    reg.add_typed("route_plot_ship", Window, move |(), _| {
        let st = s.engineering()?;
        let ship = plot_ship(&s, &st);
        Ok(PlotShip {
            here: st.here.as_ref().map(|h| h.system.clone()),
            ship_error: ship.as_ref().err().cloned(),
            ship: ship.ok(),
        })
    });
    let s = state.clone();
    reg.add_typed("route_plot", Window, move |x: AskArg<astromech_galaxy::plot::Ask>, call| {
        route_plot(&s, &x.ask, call)
    });

    // The neutron star file in the data folder, downloaded only when the commander presses
    // Download on the Route page.
    let s = state.clone();
    reg.add_typed("neutron_file", Window, move |(), _| Ok(neutron_status(&s)));
    let s = state.clone();
    reg.add_typed("neutron_download", Window, move |(), call| neutron_download(&s, call));
    // Whether the published file differs from the one downloaded (a small checksum file).
    let s = state.clone();
    reg.add_typed("neutron_check", Window, move |(), _| {
        let newer = astromech_galaxy::neutron::newer_available(s.data_dir(), &s.neutron_url).map_err(|e| {
            astromech_log::log(&format!("neutron star file: check failed: {e:#}{}", download_detail(&e)));
            format!("{e:#}")
        })?;
        Ok(NeutronCheck { newer })
    });

    // Open the project's GitHub pages or a Coriolis / EDSY / Spansh / Inara link in the browser,
    // or a folder under the core's app, data or log folder.
    let s = state.clone();
    reg.add_typed("open_external", Window, move |x: TargetArg, _| s.open_external(&x.target));
    reg.add_typed("shell_info", Window, |(), _| Ok(ShellInfo { version: env!("CARGO_PKG_VERSION") }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::{call, serve, Request};
    use crate::state::tests::offline;
    use crate::Folders;
    use astromech_proto::Origin;
    use serde_json::json;
    use std::io::{BufRead, BufReader, Write};
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    fn state(data: &Path) -> State {
        State::new(data.to_path_buf(), offline(), None, Folders::default)
    }

    fn registry(state: State) -> Registry {
        let mut reg = Registry::new();
        register(&Arc::new(state), &mut reg);
        reg
    }

    fn open_external(reg: &Registry, target: &Path) -> Result<Value, String> {
        let args = json!({ "target": target });
        let req = Request { args: Some(args), origin: Some(Origin::AppWindow), ..Request::new("open_external") };
        reg.dispatch(&req, &|_| {})
    }

    /// `open_external` opens links the app knows and folders under the core's own; anything
    /// else is "not allowed" and nothing is opened.
    #[test]
    fn open_external_opens_only_known_links_and_the_cores_folders() {
        let d = tempfile::tempdir().unwrap();
        let (data, logs, other) = (d.path().join("data"), d.path().join("logs"), d.path().join("Documents"));
        for dir in [&data, &logs, &other, &data.join("exports")] {
            std::fs::create_dir_all(dir).unwrap();
        }
        let opened = Arc::new(Mutex::new(Vec::<String>::new()));
        let o = opened.clone();
        let reg = registry(state(&data).openable(vec![logs.clone()]).with_opener(move |t| {
            crate::lock(&o).push(t.to_string());
            Ok(())
        }));
        for no in
            [other.clone(), data.join("exports").join("..").join("..").join("Documents"), PathBuf::from("notepad.exe")]
        {
            assert_eq!(open_external(&reg, &no), Err("not allowed".into()), "{}", no.display());
        }
        assert_eq!(open_external(&reg, Path::new("https://evil.example/")), Err("not allowed".into()));
        assert!(crate::lock(&opened).is_empty());
        for yes in [data.join("exports"), logs.clone()] {
            assert_eq!(open_external(&reg, &yes), Ok(Value::Null));
        }
        let link = "https://github.com/brewer-corp/astromech/issues";
        assert!(astromech_engineering::links::is_openable_url(link));
        assert_eq!(open_external(&reg, Path::new(link)), Ok(Value::Null));
        let want = [data.join("exports").display().to_string(), logs.display().to_string(), link.to_string()];
        assert_eq!(*crate::lock(&opened), want);
    }

    /// Serves `files` (path, body) on 127.0.0.1 until the test ends; anything else is a 404.
    fn http(files: Vec<(&'static str, Vec<u8>)>) -> String {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", l.local_addr().unwrap());
        std::thread::spawn(move || {
            for s in l.incoming() {
                let Ok(mut s) = s else { continue };
                let mut r = BufReader::new(s.try_clone().unwrap());
                let mut first = String::new();
                r.read_line(&mut first).unwrap();
                let mut line = String::new();
                while r.read_line(&mut line).unwrap() > 2 {
                    line.clear();
                }
                let path = first.split_whitespace().nth(1).unwrap_or("");
                match files.iter().find(|(p, _)| *p == path) {
                    Some((_, body)) => {
                        write!(s, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len())
                            .unwrap();
                        s.write_all(body).unwrap();
                    }
                    None => {
                        write!(s, "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap()
                    }
                }
            }
        });
        base
    }

    /// `neutron_download` over the control API, as the shell forwards it (no read timeout):
    /// the progress lines reach the caller before the reply, which is the Route page's status of
    /// the new file, and the file is loaded for the next plot.
    #[test]
    fn a_neutron_download_sends_progress_before_its_reply() {
        use sha2::Digest;
        let sample = std::fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../astromech-galaxy/tests/fixtures/neutron-sample.bin"),
        )
        .unwrap();
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        gz.write_all(&sample).unwrap();
        let gz = gz.finish().unwrap();
        let sum: String = sha2::Sha256::digest(&gz).iter().map(|b| format!("{b:02x}")).collect();
        let base = http(vec![
            ("/neutron.bin.gz.sha256", format!("{sum}  neutron.bin.gz\n").into_bytes()),
            ("/neutron.bin.gz", gz.clone()),
        ]);
        let d = tempfile::tempdir().unwrap();
        let data = d.path().join("data");
        let state = Arc::new(state(&data).with_neutron_url(&base));
        let mut reg = Registry::new();
        register(&state, &mut reg);
        let app = d.path().join("app");
        let _srv = serve(&app, Arc::new(reg)).unwrap();
        let ask = |cmd: &str, seen: &mut Vec<Value>| {
            let req = Request { origin: Some(Origin::AppWindow), ..Request::new(cmd) };
            let r = call(&app, req, None, &mut |p| seen.push(json!({ "progress": p })));
            seen.push(json!({ "reply": r.clone().unwrap() }));
            r.unwrap()
        };

        let mut seen = vec![];
        assert_eq!(ask("neutron_file", &mut seen), json!({ "file": null, "error": null }));
        assert_eq!(ask("neutron_check", &mut seen), json!({ "newer": true }));
        let mut seen = vec![];
        let reply = ask("neutron_download", &mut seen);
        let len = gz.len() as u64;
        assert_eq!(seen.first(), Some(&json!({ "progress": { "done": len, "total": len } })), "{seen:?}");
        assert_eq!(seen.last(), Some(&json!({ "reply": reply })));
        assert_eq!(reply["file"]["stars"], 35, "{reply}");
        assert_eq!(reply["error"], Value::Null);
        assert!(state.neutron.get().unwrap().is_some(), "loaded for the next plot");
        assert_eq!(ask("neutron_check", &mut vec![]), json!({ "newer": false }));
    }
}
