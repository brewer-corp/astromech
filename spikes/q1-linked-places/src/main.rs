//! Spike Q1 (astromech-yn6y): linked-place queries ("place A with conditions, within N ly of
//! place B with conditions, ranked by one goal") run end to end on Spansh and Ardent through
//! the real `astromech_data::Client`. Every answer is recorded under `cache/` and replayed on
//! later runs; `--offline` refuses any request not already recorded.
//!
//! ```text
//! q1 recipe1 origin=Sol radius=60 link=20 allegiance=Empire trader=Manufactured [fanout=on]
//! q1 recipe2 origin=Sol radius=60 link=30 mineral=Platinum min_price=0 [fanout=on]
//! q1 probe bodies Sol '{"ring_signals":[{"name":["Platinum"]}]}' [size]
//! q1 edsm-factions "Some System"
//! ```

mod disk;
mod join;

use anyhow::{anyhow, bail, Result};
use astromech_data::ardent::{Listing, MarketFilter, Trade};
use astromech_data::spansh::{Body, Page, Reference, Search, Station, System, CARRIER_TYPES, STATION_TYPES};
use astromech_data::{Cache, Client, Limiter, Request, Service};
use disk::DiskTransport;
use join::{dist, Point};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

struct Ctx {
    client: Client,
    disk: Arc<DiskTransport>,
}

/// Forwards to the shared transport, so the Ctx keeps a handle for the tallies.
struct Shared(Arc<DiskTransport>);
impl astromech_data::Transport for Shared {
    fn send(
        &self,
        url: &str,
        req: &Request,
    ) -> std::result::Result<astromech_data::Response, astromech_data::TransportError> {
        self.0.send(url, req)
    }
}

fn args_map(args: &[String]) -> BTreeMap<String, String> {
    args.iter().filter_map(|a| a.split_once('=').map(|(k, v)| (k.to_string(), v.to_string()))).collect()
}

fn get<'a>(m: &'a BTreeMap<String, String>, k: &str, default: &'a str) -> &'a str {
    m.get(k).map(|s| s.as_str()).unwrap_or(default)
}

fn now_s() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// Unix seconds from Spansh's / Ardent's timestamps ("2026-10-24 07:18:57+00", "2026-10-24T07:18:57Z").
fn parse_ts(s: &str) -> Option<i64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let n = |r: std::ops::Range<usize>| s.get(r)?.parse::<i64>().ok();
    let (y, mo, d, h, mi, se) = (n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?);
    // days from civil (Howard Hinnant)
    let (y2, m2) = if mo <= 2 { (y - 1, mo + 9) } else { (y, mo - 3) };
    let era = y2.div_euclid(400);
    let yoe = y2 - era * 400;
    let doy = (153 * m2 + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    Some(days * 86400 + h * 3600 + mi * 60 + se)
}

fn age_h(s: Option<&str>) -> Option<f64> {
    s.and_then(parse_ts).map(|t| (now_s() - t) as f64 / 3600.0)
}

fn quantiles(mut v: Vec<f64>) -> String {
    if v.is_empty() {
        return "n/a".into();
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    format!("min {:.1} / median {:.1} / p90 {:.1} / max {:.1} (n={})", q(0.0), q(0.5), q(0.9), q(1.0), v.len())
}

impl Ctx {
    fn search_raw(&self, kind: &str, q: &Search) -> Result<Value> {
        let req = Request::post_json(format!("/api/{kind}/search"), q.body());
        Ok(self.client.fetch::<Value>(Service::Spansh, &req, None)?.value)
    }

    /// Every page of a Spansh search, up to `max` results. Returns (count Spansh reported, hits, pages).
    fn search_all<T: DeserializeOwned>(&self, kind: &str, q: Search, max: usize) -> Result<(u64, Vec<T>, Vec<Value>)> {
        let mut hits = vec![];
        let mut raws = vec![];
        let mut page = 0;
        loop {
            let raw = self.search_raw(kind, &q.clone().page(page))?;
            let p: Page<Value> = serde_json::from_value(raw.clone())?;
            let n = p.results.len();
            for r in p.results {
                raws.push(r.clone());
                hits.push(serde_json::from_value::<T>(r)?);
            }
            page += 1;
            if n < q.size || hits.len() as u64 >= p.count || hits.len() >= max {
                return Ok((p.count, hits, raws));
            }
        }
    }

    fn origin(&self, name: &str) -> Result<Point> {
        let s = self.client.spansh_system_names(name)?.value;
        let hit =
            s.into_iter().find(|s| s.name.eq_ignore_ascii_case(name)).ok_or_else(|| anyhow!("no system {name}"))?;
        Ok(hit.pos())
    }

    fn report_tally(&self, label: &str, started: Instant) {
        println!("\n## Requests ({label})");
        let mut live = 0;
        for (svc, t) in self.disk.tally() {
            live += t.live;
            println!(
                "- {svc}: {} live, {} replayed, {:.1} MB, live wall time {:.1} s",
                t.live,
                t.cached,
                t.bytes as f64 / 1e6,
                t.live_ms as f64 / 1000.0
            );
        }
        println!(
            "- this run: {live} live requests, {:.1} s elapsed; spike total so far {} live",
            started.elapsed().as_secs_f64(),
            self.disk.ledger_count()
        );
    }
}

fn norm(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase()
}

/// Recipe 1's condition on A, re-checked on every hit (Spansh ignores what it doesn't know).
fn hge_match(s: &System, allegiances: &[String], states: &[&str], min_pop: i64) -> Option<String> {
    if s.population < min_pop {
        return None;
    }
    for f in &s.minor_faction_presences {
        if !allegiances.is_empty() && !allegiances.iter().any(|a| norm(a) == norm(&f.allegiance)) {
            continue;
        }
        let mut st: Vec<String> = f.active_states.iter().map(|x| norm(x)).collect();
        if let Some(x) = &f.state {
            st.push(norm(x));
        }
        if let Some(hit) = states.iter().find(|w| st.contains(&norm(w))) {
            return Some(format!("{} ({}, {}, {:.0}%)", f.name, f.allegiance, hit, f.influence * 100.0));
        }
    }
    None
}

fn non_carrier_types() -> Vec<&'static str> {
    STATION_TYPES.iter().copied().filter(|t| !CARRIER_TYPES.contains(t)).collect()
}

/// Station answers carry whole markets and module lists (~20 kB each); bigger pages time out.
const STATION_PAGE: usize = 50;

const HGE_STATES: &[&str] = &["Boom", "Expansion", "War", "Civil War"];

fn recipe1(ctx: &Ctx, m: &BTreeMap<String, String>) -> Result<()> {
    let started = Instant::now();
    let origin_name = get(m, "origin", "Sol");
    let radius: f64 = get(m, "radius", "60").parse()?;
    let link: f64 = get(m, "link", "20").parse()?;
    let allegiances: Vec<String> =
        get(m, "allegiance", "Empire").split(',').filter(|s| !s.is_empty()).map(String::from).collect();
    let trader = get(m, "trader", "Manufactured");
    let min_pop: i64 = get(m, "min_pop", "1000000000").parse()?;
    let fanout = get(m, "fanout", "off") == "on";
    let fanout_max: usize = get(m, "fanout_max", "60").parse()?;
    println!(
        "# Recipe 1: HGE-likely system ({} in {:?}, >= {} people) within {link} ly of a {trader} trader, within {radius} ly of {origin_name}, nearest first",
        if allegiances.is_empty() { "any allegiance".to_string() } else { allegiances.join("/") },
        HGE_STATES,
        min_pop
    );
    let o = ctx.origin(origin_name)?;

    // Side A: HGE-likely systems near the origin (the Spansh filters, re-checked).
    // spansh_state=off leaves the state to the local re-check, to see whether Spansh's filter
    // misses factions whose wanted state is only among their active states.
    let mut fac =
        if get(m, "spansh_state", "on") == "on" { json!({ "state": { "value": HGE_STATES } }) } else { json!({}) };
    if !allegiances.is_empty() {
        fac["allegiance"] = json!({ "value": allegiances });
    }
    let a_q = Search::near(Reference::Coords(o))
        .within(radius)
        .population(min_pop, 1_000_000_000_000)
        .filter("minor_faction_presences", json!([fac]))
        .size(500);
    let (a_count, a_hits, a_raw) = ctx.search_all::<System>("systems", a_q, 5000)?;
    // A third, static set: permit-locked systems near the origin (station records don't say).
    let permit_q = Search::near(Reference::Coords(o))
        .within(radius + link)
        .filter("needs_permit", json!({ "value": true }))
        .size(500);
    let (permit_count, permit_hits, _) = ctx.search_all::<System>("systems", permit_q, 5000)?;
    let permit: std::collections::HashSet<i64> = permit_hits.iter().map(|s| s.id64).collect();
    let a_all: Vec<(&System, String)> =
        a_hits.iter().filter_map(|s| hge_match(s, &allegiances, HGE_STATES, min_pop).map(|why| (s, why))).collect();
    let a: Vec<(&System, String)> = a_all.iter().filter(|(s, _)| !permit.contains(&s.id64)).cloned().collect();
    println!(
        "\n## Side A (systems)\n- Spansh count {a_count}, fetched {}, pass re-check {}, not permit-locked {} (permit systems within {} ly: {permit_count})",
        a_hits.len(),
        a_all.len(),
        a.len(),
        radius + link
    );
    let ages: Vec<f64> = a.iter().filter_map(|(s, _)| age_h(s.updated_at.as_deref())).collect();
    println!("- system data age, hours: {}", quantiles(ages));
    if let Some(r) = a_raw.first() {
        let f0 = &r["minor_faction_presences"][0];
        println!("- faction record keys: {:?}", f0.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()));
    }
    // How many matched only through active_states (state != wanted)
    let via_active = a
        .iter()
        .filter(|(s, _)| {
            !s.minor_faction_presences.iter().any(|f| {
                f.state.as_deref().is_some_and(|x| HGE_STATES.iter().any(|w| norm(w) == norm(x)))
                    && (allegiances.is_empty() || allegiances.iter().any(|al| norm(al) == norm(&f.allegiance)))
            })
        })
        .count();
    println!("- matched only via active_states (not the faction's main `state`): {via_active}");

    // Side B: traders within radius + link of the origin.
    let b_q = Search::near(Reference::Coords(o))
        .within(radius + link)
        .material_trader(trader)
        .types(&non_carrier_types())
        .size(STATION_PAGE);
    let (b_count, b_hits, _) = ctx.search_all::<Station>("stations", b_q, 5000)?;
    let b: Vec<&Station> = b_hits
        .iter()
        .filter(|s| {
            s.material_trader.as_deref().is_some_and(|t| t.eq_ignore_ascii_case(trader))
                && !s.station_type.as_deref().is_some_and(|t| CARRIER_TYPES.contains(&t))
                && !permit.contains(&s.system_id64)
        })
        .collect();
    let b_systems: HashMap<i64, &Station> = b.iter().map(|s| (s.system_id64, *s)).collect();
    println!(
        "\n## Side B (stations)\n- Spansh count {b_count}, fetched {}, pass re-check and not permit-locked {}, in {} systems",
        b_hits.len(),
        b.len(),
        b_systems.len()
    );

    // Join locally: every A with its nearest B within `link`; rank by distance from origin.
    let pairs = join::nearest_within(
        a.iter().map(|(s, _)| s.pos()).collect::<Vec<_>>().as_slice(),
        b.iter().map(|s| s.pos()).collect::<Vec<_>>().as_slice(),
        link,
    );
    let mut trips: Vec<(f64, f64, &System, &str, &Station)> =
        pairs.iter().map(|&(ai, bi, d)| (dist(o, a[ai].0.pos()), d, a[ai].0, a[ai].1.as_str(), b[bi])).collect();
    trips.sort_by(|x, y| (x.0 + x.1).partial_cmp(&(y.0 + y.1)).unwrap());
    println!(
        "\n## Trips (A then B), by origin->A->B distance: {} of {} A systems have a trader within {link} ly",
        trips.len(),
        a.len()
    );
    for (oa, ab, s, why, st) in trips.iter().take(8) {
        println!(
            "- {} ({:.1} ly, {:.2} bn, data {:.0} h old; {why}) -> {} / {} ({:.1} ly, {:.0} ls)",
            s.name,
            oa,
            s.population as f64 / 1e9,
            age_h(s.updated_at.as_deref()).unwrap_or(-1.0),
            st.system_name,
            st.name,
            ab,
            st.distance_to_arrival.unwrap_or(-1.0)
        );
    }
    let set_tally = ctx.disk.tally();
    ctx.report_tally("set join: one search per side", started);

    if fanout {
        // Rarer side first (traders), then the other side first (HGE systems): one search per
        // candidate. Compare request counts and answers with the set join.
        println!("\n## Fan-out strategies");
        ctx.disk.reset_tally();
        let t0 = Instant::now();
        let mut fan_a: Vec<i64> = vec![];
        let trader_systems: Vec<&Station> = b_systems.values().copied().collect();
        if trader_systems.len() <= fanout_max {
            for st in &trader_systems {
                let mut fac = json!({ "state": { "value": HGE_STATES } });
                if !allegiances.is_empty() {
                    fac["allegiance"] = json!({ "value": allegiances });
                }
                let q = Search::near(Reference::Coords(st.pos()))
                    .within(link)
                    .population(min_pop, 1_000_000_000_000)
                    .filter("minor_faction_presences", json!([fac]))
                    .size(100);
                let (_, hits, _) = ctx.search_all::<System>("systems", q, 1000)?;
                for h in hits {
                    if hge_match(&h, &allegiances, HGE_STATES, min_pop).is_some()
                        && dist(o, h.pos()) <= radius
                        && !permit.contains(&h.id64)
                    {
                        fan_a.push(h.id64);
                    }
                }
            }
            fan_a.sort();
            fan_a.dedup();
            let live: u32 = ctx.disk.tally().iter().map(|(_, t)| t.live + t.cached).sum();
            println!(
                "- traders first: {} requests ({} trader systems), {:.1} s, {} A systems found (set join: {})",
                live,
                trader_systems.len(),
                t0.elapsed().as_secs_f64(),
                fan_a.len(),
                trips.len()
            );
        } else {
            println!("- traders first: skipped, {} trader systems > fanout_max", trader_systems.len());
        }
        ctx.disk.reset_tally();
        let t1 = Instant::now();
        if a.len() <= fanout_max {
            let mut with_trader = 0;
            for (s, _) in &a {
                let q = Search::near(Reference::Coords(s.pos()))
                    .within(link)
                    .material_trader(trader)
                    .types(&non_carrier_types())
                    .size(10);
                let (_, hits, _) = ctx.search_all::<Station>("stations", q, 10)?;
                if hits.iter().any(|h| h.material_trader.as_deref().is_some_and(|t| t.eq_ignore_ascii_case(trader))) {
                    with_trader += 1;
                }
            }
            let live: u32 = ctx.disk.tally().iter().map(|(_, t)| t.live + t.cached).sum();
            println!(
                "- HGE systems first: {} requests, {:.1} s, {with_trader} A systems with a trader (set join: {})",
                live,
                t1.elapsed().as_secs_f64(),
                trips.len()
            );
        } else {
            println!("- HGE systems first: skipped, {} A systems > fanout_max", a.len());
        }
        let _ = set_tally;
    }
    Ok(())
}

fn ardent_commodity(mineral: &str) -> String {
    match norm(mineral).as_str() {
        "lowtemperaturediamonds" | "ltd" | "ltds" => "lowtemperaturediamond".into(),
        "voidopal" | "voidopals" => "opal".into(),
        other => other.into(),
    }
}

/// Ardent's placeholder rows (see ARCHITECTURE: demand or stock 999,999) and empty demand.
fn real_buyer(l: &Listing) -> bool {
    l.demand > 0 && l.demand != 999_999 && !l.station_type.as_deref().unwrap_or("").contains("Construction")
}

/// Ardent's nearby imports let some rows below `minLandingPadSize` through (seen 24 Oct 2026:
/// an outpost with pad 2 for minLandingPadSize=3), so the pad is re-checked.
fn fits_pad(l: &Listing, pad: u8) -> bool {
    l.max_landing_pad_size.is_none_or(|p| p >= pad)
}

/// Illustrative mining time: overlapping hotspots and richer reserves fill the hold faster.
fn mining_minutes(base: f64, hotspots: u32, reserve: &str) -> f64 {
    let reserve = match reserve {
        "Pristine" => 0.8,
        "Major" => 0.9,
        "Low" => 1.15,
        "Depleted" => 1.3,
        _ => 1.0,
    };
    base * reserve / (1.0 + 0.5 * (hotspots.max(1) - 1) as f64)
}

struct Ring2 {
    body: String,
    ring: String,
    system: String,
    pos: Point,
    count: u32,
    ring_type: String,
    reserve: String,
    signals_age_h: Option<f64>,
}

fn recipe2(ctx: &Ctx, m: &BTreeMap<String, String>) -> Result<()> {
    let started = Instant::now();
    let origin_name = get(m, "origin", "Sol");
    let radius: f64 = get(m, "radius", "60").parse()?;
    let link: f64 = get(m, "link", "30").parse()?;
    let mineral = get(m, "mineral", "Platinum");
    let min_price: i64 = get(m, "min_price", "0").parse()?;
    let days: u32 = get(m, "days", "7").parse()?;
    let pad: u8 = get(m, "pad", "3").parse()?;
    let jump: f64 = get(m, "jump", "20").parse()?;
    let cargo: f64 = get(m, "cargo", "256").parse()?;
    let mine_min: f64 = get(m, "mine_min", "60").parse()?;
    let fanout = get(m, "fanout", "off") == "on";
    let fanout_max: usize = get(m, "fanout_max", "60").parse()?;
    let compare_spansh = get(m, "compare_spansh", "off") == "on";
    println!(
        "# Recipe 2: a ring with a {mineral} hotspot within {radius} ly of {origin_name}, sold within {link} ly at >= {min_price} cr/t (pad >= {pad}, prices <= {days} days old), ranked by credits per hour ({cargo} t, {mine_min} min mining, {jump} ly jumps at 50 s, there and back)"
    );
    let o = ctx.origin(origin_name)?;

    // Side A: bodies with a hotspot ring for the mineral.
    let a_q = Search::near(Reference::Coords(o)).within(radius).ring_signal(mineral).size(500);
    let (a_count, bodies, _) = ctx.search_all::<Body>("bodies", a_q, 5000)?;
    let mut rings: Vec<Ring2> = vec![];
    let mut all_rings_on_hits = 0;
    let mut unscanned_rings_on_hits = 0;
    for b in &bodies {
        for r in &b.rings {
            all_rings_on_hits += 1;
            if r.signals.is_empty() {
                unscanned_rings_on_hits += 1;
            }
            let c = r.hotspots(mineral);
            if c > 0 {
                rings.push(Ring2 {
                    body: b.name.clone(),
                    ring: r.name.clone(),
                    system: b.system_name.clone(),
                    pos: b.pos(),
                    count: c,
                    ring_type: r.ring_type.clone(),
                    reserve: b.reserve_level.clone().unwrap_or_default(),
                    signals_age_h: age_h(r.signals_updated_at.as_deref()),
                });
            }
        }
    }
    let mut ring_systems: Vec<&str> = rings.iter().map(|r| r.system.as_str()).collect();
    ring_systems.sort();
    ring_systems.dedup();
    println!(
        "\n## Side A (rings)\n- Spansh body count {a_count}, fetched {}, {} rings with a {mineral} hotspot in {} systems; {} with 2+ hotspots",
        bodies.len(),
        rings.len(),
        ring_systems.len(),
        rings.iter().filter(|r| r.count >= 2).count()
    );
    println!(
        "- other rings on those bodies with no signals recorded: {unscanned_rings_on_hits} of {all_rings_on_hits}"
    );
    println!("- hotspot scan age, hours: {}", quantiles(rings.iter().filter_map(|r| r.signals_age_h).collect()));
    let mut types: BTreeMap<String, u32> = BTreeMap::new();
    for r in &rings {
        *types.entry(format!("{} {}", r.ring_type, r.reserve)).or_default() += 1;
    }
    println!("- ring type / reserve: {types:?}");

    // Side B: buyers near the origin from Ardent (best price first, at most 1000 rows).
    let commodity = ardent_commodity(mineral);
    let filter = MarketFilter {
        max_distance: Some(radius + link),
        max_days_ago: Some(days),
        min_landing_pad: Some(pad),
        fleet_carriers: Some(false),
        min_volume: None,
    };
    let listings = ctx.client.ardent_nearby(origin_name, &commodity, Trade::Imports, &filter)?.value;
    let below_pad = listings.iter().filter(|l| !fits_pad(l, pad)).count();
    let buyers: Vec<&Listing> =
        listings.iter().filter(|l| real_buyer(l) && fits_pad(l, pad) && l.sell_price >= min_price).collect();
    println!("\n- Ardent rows below the pad asked for (dropped): {below_pad}");
    let mut prices: Vec<f64> = buyers.iter().map(|l| l.sell_price as f64).collect();
    prices.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!(
        "\n## Side B (buyers, Ardent)\n- {} rows (cap 1000), {} real buyers at >= {min_price}; price {}",
        listings.len(),
        buyers.len(),
        quantiles(prices.clone())
    );
    println!("- price age, hours: {}", quantiles(buyers.iter().filter_map(|l| age_h(Some(&l.updated_at))).collect()));

    // Join locally: each ring against every buyer within `link`; score credits per hour.
    let jumps = |d: f64| if d < 0.01 { 0.0 } else { (d / jump).ceil() };
    let mut best: Vec<(f64, &Ring2, &Listing, f64, f64)> = vec![];
    let mut rings_with_buyer = 0;
    for r in &rings {
        let mut mine: Vec<(f64, &Ring2, &Listing, f64, f64)> = buyers
            .iter()
            .filter_map(|l| {
                let d = dist(r.pos, [l.system_x, l.system_y, l.system_z]);
                (d <= link).then(|| {
                    let j = jumps(d);
                    let sc = l.distance_to_arrival.unwrap_or(1000.0).min(100_000.0);
                    // supercruise: ~1 min per 1000 ls, capped; jumps 50 s; there and back.
                    let secs = mining_minutes(mine_min, r.count, &r.reserve) * 60.0
                        + 2.0 * (j * 50.0 + 30.0 + sc / 1000.0 * 60.0);
                    (l.sell_price as f64 * cargo / (secs / 3600.0), r, *l, d, j)
                })
            })
            .collect();
        if !mine.is_empty() {
            rings_with_buyer += 1;
        }
        mine.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        best.extend(mine.into_iter().take(1));
    }
    best.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut joined_systems: Vec<&str> = best.iter().map(|x| x.1.system.as_str()).collect();
    joined_systems.sort();
    joined_systems.dedup();
    let own_only = best
        .iter()
        .filter(|x| {
            let s = x.1;
            buyers.iter().all(|l| {
                let d = dist(s.pos, [l.system_x, l.system_y, l.system_z]);
                d > link || l.system_name == s.system
            })
        })
        .map(|x| x.1.system.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    println!(
        "\n- ring systems with a buyer: {} of {}; {} only through a buyer in the ring's own system ({:?})",
        joined_systems.len(),
        ring_systems.len(),
        own_only.len(),
        own_only
    );
    println!(
        "\n## Trips (ring then buyer), best buyer per ring by credits per hour: {rings_with_buyer} of {} rings have a buyer within {link} ly",
        rings.len()
    );
    for (score, r, l, d, j) in best.iter().take(8) {
        println!(
            "- {:.1} M cr/h: {} ({} x{}, {} {}, scanned {:.0} h ago; {:.1} ly from origin) -> {} / {} at {} cr/t, demand {}, {:.1} ly = {} jumps, {:.0} ls, price {:.0} h old",
            score / 1e6,
            r.ring,
            mineral,
            r.count,
            r.ring_type,
            r.reserve,
            r.signals_age_h.unwrap_or(-1.0),
            dist(o, r.pos),
            l.system_name,
            l.station_name,
            l.sell_price,
            l.demand,
            d,
            j,
            l.distance_to_arrival.unwrap_or(-1.0),
            age_h(Some(&l.updated_at)).unwrap_or(-1.0)
        );
        let _ = &r.body;
    }
    // The other ranking: the best price reachable at all versus extra jumps (Pareto front).
    if let Some((_, r, ..)) = best.first() {
        let mut front: Vec<(f64, i64, String)> = vec![];
        let mut opts: Vec<(f64, i64, String)> = buyers
            .iter()
            .map(|l| {
                let d = dist(r.pos, [l.system_x, l.system_y, l.system_z]);
                (jumps(d), l.sell_price, format!("{} / {}", l.system_name, l.station_name))
            })
            .collect();
        opts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(b.1.cmp(&a.1)));
        let mut top = i64::MIN;
        for x in opts {
            if x.1 > top {
                top = x.1;
                front.push(x);
            }
        }
        println!("- price against extra jumps from {} (each step: more jumps, better price):", r.ring);
        for (j, p, n) in front {
            println!("  - {j} jumps: {p} cr/t at {n}");
        }
    }
    ctx.report_tally("set join: one search per side", started);

    if compare_spansh {
        ctx.disk.reset_tally();
        let q = Search::near(Reference::Coords(o))
            .within(radius + link)
            .demand(mineral, 1)
            .types(&non_carrier_types())
            .size(STATION_PAGE);
        let (count, st, _) = ctx.search_all::<Station>("stations", q, 200)?;
        let mut p: Vec<f64> = st.iter().filter_map(|s| s.commodity(mineral)).map(|c| c.sell_price as f64).collect();
        p.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let ages = st.iter().filter_map(|s| age_h(s.market_updated_at.as_deref())).collect();
        println!(
            "\n## Side B from Spansh instead: count {count}, first page {}, price {}; market age h {}",
            st.len(),
            quantiles(p),
            quantiles(ages)
        );
        ctx.report_tally("Spansh buyers page", started);
    }

    if fanout {
        println!("\n## Fan-out strategies");
        ctx.disk.reset_tally();
        let t0 = Instant::now();
        if ring_systems.len() <= fanout_max {
            let mut found = 0;
            for s in &ring_systems {
                let f = MarketFilter { max_distance: Some(link), ..filter.clone() };
                match ctx.client.ardent_nearby(s, &commodity, Trade::Imports, &f) {
                    Ok(l) => {
                        // Ardent leaves the searched system's own markets out of nearby answers.
                        if l.value.iter().any(|l| real_buyer(l) && fits_pad(l, pad) && l.sell_price >= min_price) {
                            found += 1;
                        }
                    }
                    Err(e) => println!("  - {s}: {e}"),
                }
            }
            let n: u32 = ctx.disk.tally().iter().map(|(_, t)| t.live + t.cached).sum();
            println!(
                "- hotspots first: {n} requests, {:.1} s, {found} of {} ring systems have a buyer",
                t0.elapsed().as_secs_f64(),
                ring_systems.len()
            );
        } else {
            println!("- hotspots first: skipped, {} ring systems > fanout_max", ring_systems.len());
        }
        let mut buyer_systems: Vec<&str> = buyers.iter().map(|l| l.system_name.as_str()).collect();
        buyer_systems.sort();
        buyer_systems.dedup();
        println!(
            "- buyers first would need {} bodies searches (one per buyer system){}",
            buyer_systems.len(),
            if buyer_systems.len() > fanout_max { "; not run" } else { "" }
        );
        if buyer_systems.len() <= fanout_max {
            ctx.disk.reset_tally();
            let t1 = Instant::now();
            let mut seen = std::collections::HashSet::new();
            for l in buyers.iter().filter(|l| seen.insert(l.system_address)) {
                let q = Search::near(Reference::Coords([l.system_x, l.system_y, l.system_z]))
                    .within(link)
                    .ring_signal(mineral)
                    .size(100);
                ctx.search_all::<Body>("bodies", q, 100)?;
            }
            let n: u32 = ctx.disk.tally().iter().map(|(_, t)| t.live + t.cached).sum();
            println!("- buyers first: {n} requests, {:.1} s", t1.elapsed().as_secs_f64());
        }
    }
    Ok(())
}

fn probe(ctx: &Ctx, rest: &[String]) -> Result<()> {
    let kind = rest.first().ok_or_else(|| anyhow!("probe <systems|stations|bodies> <system> <filters> [size]"))?;
    let origin = rest.get(1).ok_or_else(|| anyhow!("origin"))?;
    let filters: Value = serde_json::from_str(rest.get(2).map(|s| s.as_str()).unwrap_or("{}"))?;
    let size: usize = rest.get(3).map(|s| s.parse()).transpose()?.unwrap_or(5);
    let o = ctx.origin(origin)?;
    let mut q = Search::near(Reference::Coords(o)).size(size);
    if let Value::Object(f) = filters {
        for (k, v) in f {
            q = q.filter(&k, v);
        }
    }
    let raw = ctx.search_raw(kind, &q)?;
    println!("count {}", raw["count"]);
    for r in raw["results"].as_array().into_iter().flatten() {
        let mut short = r.clone();
        if let Some(o) = short.as_object_mut() {
            for k in ["market", "modules", "ships", "bodies", "stations", "materials", "landmarks"] {
                if let Some(v) = o.get(k) {
                    let n = v.as_array().map(|a| a.len()).unwrap_or(0);
                    o.insert(k.into(), json!(format!("[{n} items]")));
                }
            }
        }
        println!("{}", serde_json::to_string(&short)?);
    }
    Ok(())
}

fn edsm_factions(ctx: &Ctx, rest: &[String]) -> Result<()> {
    for name in rest {
        let req = Request::get("/api-system-v1/factions").query("systemName", name).query("showHistory", 0);
        let v = ctx.client.fetch::<Value>(Service::Edsm, &req, None)?.value;
        println!("## {name} (EDSM)");
        for f in v["factions"].as_array().into_iter().flatten() {
            let upd = f["lastUpdate"].as_i64().unwrap_or(0);
            let active: Vec<String> = f["activeStates"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|s| s["state"].as_str().map(String::from))
                .collect();
            println!(
                "- {} ({}): state {}, active {:?}, influence {:.1}%, updated {:.0} h ago",
                f["name"].as_str().unwrap_or("?"),
                f["allegiance"].as_str().unwrap_or("?"),
                f["state"].as_str().unwrap_or("?"),
                active,
                f["influence"].as_f64().unwrap_or(0.0) * 100.0,
                (now_s() - upd) as f64 / 3600.0
            );
        }
        // Spansh's view of the same system, for comparison.
        let q = Search::near(Reference::System(name.clone())).size(1);
        let raw = ctx.search_raw("systems", &q)?;
        if let Some(s) = raw["results"].get(0) {
            println!("## {name} (Spansh, system updated {})", s["updated_at"]);
            for f in s["minor_faction_presences"].as_array().into_iter().flatten() {
                println!(
                    "- {} ({}): state {}, active {}, influence {:.1}%",
                    f["name"].as_str().unwrap_or("?"),
                    f["allegiance"].as_str().unwrap_or("?"),
                    f["state"],
                    f["active_states"],
                    f["influence"].as_f64().unwrap_or(0.0) * 100.0
                );
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let offline = args.iter().any(|a| a == "--offline");
    args.retain(|a| a != "--offline");
    let cap: usize = std::env::var("Q1_CAP").ok().and_then(|s| s.parse().ok()).unwrap_or(400);
    let dir = std::env::var("Q1_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("cache"));
    let disk = Arc::new(DiskTransport::new(dir, offline, cap));
    let client = Client::with_transport(Cache::in_memory()?, Arc::new(Shared(disk.clone()))).limiter(Limiter::none());
    let ctx = Ctx { client, disk };
    let cmd = args.first().cloned().unwrap_or_default();
    let rest = &args[1.min(args.len())..];
    match cmd.as_str() {
        "recipe1" => recipe1(&ctx, &args_map(rest)),
        "recipe2" => recipe2(&ctx, &args_map(rest)),
        "probe" => probe(&ctx, rest),
        "edsm-factions" => edsm_factions(&ctx, rest),
        _ => bail!("usage: q1 [--offline] recipe1|recipe2|probe|edsm-factions ..."),
    }
}
