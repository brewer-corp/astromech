//! Mining lookups: where to sell a load of ore ([`sale`]) and the nearest ring with a hotspot
//! of an ore ([`hotspots`]). The ores are `astromech_journal::ore::VALUE_ORDER`, the order the
//! prospector callouts rank them in.
//!
//! The sale is the hold's sale plan ([`crate::cargo`]) for one commodity: stations by what they
//! would pay for the load with each station's demand capping what it takes (mining loads run to
//! hundreds of tonnes, so the highest price is often a market that wants a fraction of it), and
//! the best market with demand for all of it. Hotspots are Spansh's bodies search on ring
//! signals, one row for each ring that holds the ore (the search matches a body with the ore in
//! any ring), of the ring type asked for (checked here: Spansh can't filter it) and with the
//! reserve level asked for (Spansh's `reserve_level`). The Mining hotspots finder in Places
//! lists them most hotspots first within a radius ([`most_hotspots`]: Spansh sorts by distance
//! only, so it asks for the highest hotspot counts first); the voice query says the nearest.

use crate::cargo::{plan, StationSale};
use crate::catalog;
use crate::trade::{Offer, Origin};
use astromech_data::spansh::{Body, Reference, Search, Signal};
use astromech_data::{Client, Error};
use astromech_galaxy::{dist, estimate, Filters, Galaxy, Here, Hop};
use astromech_journal::CargoItem;
use serde::{Deserialize, Serialize};

/// The sale of `tonnes` of one ore.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sale {
    pub commodity: String,
    pub name: String,
    pub tonnes: i64,
    /// The best market with demand for all of it.
    pub whole: Option<Offer>,
    /// Stations by credits for the load, each capped by its demand, best first.
    pub stations: Vec<StationSale>,
    /// When Ardent was asked (unix seconds; may be a cached answer).
    pub fetched_at: i64,
}

/// Where to sell `tonnes` of the ore `symbol` near `origin`.
pub fn sale(
    client: &Client,
    origin: &Origin,
    symbol: &str,
    tonnes: i64,
    filters: &Filters,
    range: f64,
    now_s: i64,
) -> Result<Sale, Error> {
    let symbol = symbol.trim().to_lowercase();
    let item =
        CargoItem { name: catalog::name_of(&symbol), symbol: symbol.clone(), count: tonnes, stolen: 0, mission: false };
    let mut p = plan(client, origin, &[item], filters, range, now_s)?;
    let one = p.per_commodity.pop();
    Ok(Sale {
        name: catalog::name_of(&symbol),
        commodity: symbol,
        tonnes,
        whole: one.and_then(|c| c.best),
        stations: p.stations,
        fetched_at: p.fetched_at,
    })
}

/// The lookup id the app remembers the Mining hotspots finder's filters under.
pub const LOOKUP: &str = "mining.hotspots";

/// The finder's search radius when the commander has picked none, in light years.
pub const DEFAULT_RADIUS_LY: u32 = 100;

/// Bodies asked for in one search. Spansh can't filter by ring type, so rings of the type
/// asked for are picked from these here; one page of 50 is still one request.
pub const BODIES: usize = 50;

/// The most Spansh requests one [`most_hotspots`] search makes (spike astromech-yn6y's budget:
/// about four a query, at one a second).
pub const REQUESTS: usize = 4;

/// The bodies a raised band is expected to hold: under a page, so a low guess still fits in one.
const AIM: f64 = 30.0;

/// The least reserve level a ring must have. Spansh files it on the ringed body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "lowercase")]
pub enum Reserve {
    Pristine,
    /// Major or Pristine.
    Major,
}

impl Reserve {
    /// The levels Spansh's `reserve_level` filter is sent.
    pub fn levels(self) -> &'static [&'static str] {
        match self {
            Reserve::Pristine => &["Pristine"],
            Reserve::Major => &["Pristine", "Major"],
        }
    }
}

/// What a hotspot search asks for: rings with a hotspot of `ore`, of one ring type, with at
/// least a reserve level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "MiningWant")]
pub struct Want {
    /// The ore's symbol, from the ore list: "painite".
    pub ore: String,
    /// "Icy", "Rocky", "Metallic" or "Metal Rich"; `None`: any.
    #[serde(default)]
    pub ring_type: Option<String>,
    #[serde(default)]
    pub reserve: Option<Reserve>,
}

impl Want {
    /// Any ring with a hotspot of `ore`.
    pub fn ore(ore: &str) -> Self {
        Want { ore: ore.trim().to_lowercase(), ring_type: None, reserve: None }
    }

    /// The ore's name as the rings list it: "Void Opal", "Low Temperature Diamonds".
    pub fn name(&self) -> String {
        catalog::name_of(&self.ore)
    }
}

/// (a way to say it, the ore's symbol) for every ore the hotspot lookups offer
/// (`astromech_journal::ore::VALUE_ORDER`): its catalogue name and the catalogue's other names
/// for it ("LTDs", "void opals"). The voice grammar's `{mineral}` slot.
pub fn spoken_ores() -> Vec<(&'static str, &'static str)> {
    astromech_journal::ore::VALUE_ORDER
        .iter()
        .flat_map(|o| {
            let said: Vec<&'static str> = match catalog::by_symbol(o.symbol) {
                Some(c) => std::iter::once(c.name.as_str()).chain(c.aliases.iter().map(String::as_str)).collect(),
                None => vec![o.name],
            };
            said.into_iter().map(move |s| (s, o.symbol))
        })
        .collect()
}

/// One ring with a hotspot of the ore asked for.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct Hotspot {
    pub system: String,
    #[serde(skip)]
    pub system_pos: [f64; 3],
    pub body: String,
    pub ring: String,
    /// "Icy", "Rocky", "Metallic", "Metal Rich".
    pub ring_type: String,
    /// Hotspots of the ore in the ring.
    pub count: u32,
    /// The ring's hotspots of other ores, most first.
    pub also: Vec<Signal>,
    /// The body's reserve level: "Pristine", "Major", "Common", "Low" or "Depleted".
    pub reserve: Option<String>,
    #[serde(flatten)]
    pub hop: Hop,
    /// Light seconds from the arrival star to the body.
    pub arrival_ls: Option<f64>,
    /// When the ring's hotspots were last reported (RFC 3339).
    pub updated_at: Option<String>,
}

/// How a hotspot answer is ordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    /// Nearest first: "the nearest Painite hotspot".
    Nearest,
    /// Most hotspots of the ore first, then nearest: the finder's list, ranked by its main value.
    Most,
}

/// One hotspot search's answer.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct Hotspots {
    /// Bodies within the radius with the ore in a ring and the reserve asked for (Spansh's
    /// count, which stops at 10000); their rings are what `rings` was picked from.
    pub bodies: u64,
    /// `Some(n)`: the request budget ran out before every ring in the radius could be ranked,
    /// so `rings` are every ring within the radius with at least `n` hotspots of the ore.
    pub least: Option<u32>,
    /// `Some`: the budget ran out before any count was known in full, so `rings` are the most
    /// hotspots within this many ly of `here` only. `None` with `least` `None`: `rings` are the
    /// most hotspots within `radius_ly`.
    pub reach_ly: Option<f64>,
    pub want: Want,
    /// The ore's name: "Painite".
    pub name: String,
    pub here: Here,
    /// Jump range the jumps were counted at, in ly.
    pub range: f64,
    /// The radius searched, in ly; `None`: the nearest rings, however far.
    pub radius_ly: Option<u32>,
    pub rings: Vec<Hotspot>,
    /// When Spansh was asked (unix seconds; may be a cached answer).
    pub fetched_at: i64,
}

/// Rings holding `lo` to `hi` hotspots of the ore (`hi` `None`: any more).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Band {
    pub lo: u32,
    pub hi: Option<u32>,
}

impl Band {
    /// Every ring holding the ore.
    pub const ANY: Band = Band { lo: 1, hi: None };

    fn holds(self, count: u32) -> bool {
        count >= self.lo && self.hi.is_none_or(|hi| count <= hi)
    }
}

/// The bodies search for rings holding `band` hotspots of `want`'s ore around `origin`, within
/// `radius_ly` when given, nearest first.
pub fn hotspot_search(origin: &Origin, want: &Want, radius_ly: Option<u32>, band: Band, size: usize) -> Search {
    let reference = match origin.pos {
        Some(p) => Reference::Coords(p),
        None => Reference::System(origin.system.clone()),
    };
    let q = Search::near(reference).size(size);
    let mut q = if band == Band::ANY {
        q.ring_signal(&want.name())
    } else {
        q.ring_signal_count(&want.name(), band.lo, band.hi)
    };
    if let Some(r) = radius_ly {
        q = q.within(f64::from(r));
    }
    if let Some(r) = want.reserve {
        q = q.reserve_levels(r.levels());
    }
    q
}

/// The rings in `bodies` holding `want`'s ore, of its ring type, nearest first. Spansh matches
/// a body with the ore in any ring, and can't filter ring type, so each ring is checked here.
pub fn rings_with(bodies: &[Body], origin: &Origin, want: &Want, range: f64) -> Vec<Hotspot> {
    let ore = want.name();
    let mut out: Vec<Hotspot> = bodies
        .iter()
        .flat_map(|b| {
            let ly = ly_to(origin, b);
            let ore = &ore;
            b.rings
                .iter()
                .filter(move |r| r.hotspots(ore) > 0)
                .filter(|r| want.ring_type.as_deref().is_none_or(|t| r.ring_type.eq_ignore_ascii_case(t)))
                .map(move |r| {
                    let mut also: Vec<Signal> = r
                        .signals
                        .iter()
                        .filter(|s| !s.name.eq_ignore_ascii_case(ore) && s.count > 0)
                        .cloned()
                        .collect();
                    also.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));
                    Hotspot {
                        system: b.system_name.clone(),
                        system_pos: b.pos(),
                        body: b.name.clone(),
                        ring: r.name.clone(),
                        ring_type: r.ring_type.clone(),
                        count: r.hotspots(ore),
                        also,
                        reserve: b.reserve_level.clone(),
                        hop: Hop { ly, jumps: estimate(ly, range), exact: false },
                        arrival_ls: b.distance_to_arrival,
                        updated_at: r.signals_updated_at.clone(),
                    }
                })
        })
        .collect();
    out.sort_by(|a, b| a.hop.ly.total_cmp(&b.hop.ly));
    out
}

fn ly_to(origin: &Origin, b: &Body) -> f64 {
    match origin.pos {
        Some(p) => dist(p, b.pos()),
        None => b.distance,
    }
}

/// Rings holding `want`'s ore from `origin`, within `radius_ly` when given: every ring of the
/// nearest [`BODIES`] bodies that matches, nearest first. One Spansh request.
pub fn hotspots(
    client: &Client,
    origin: &Origin,
    want: &Want,
    radius_ly: Option<u32>,
    range: f64,
) -> Result<Ranked, Error> {
    let got = client.spansh_bodies(&hotspot_search(origin, want, radius_ly, Band::ANY, BODIES))?;
    Ok(Ranked {
        rings: rings_with(&got.value.results, origin, want, range),
        bodies: got.value.count,
        cut: Cut::Whole,
        requests: 1,
        fetched_at: got.fetched_at,
    })
}

/// Rings found by a hotspot search, before the star map counts their jumps.
#[derive(Debug, Clone, PartialEq)]
pub struct Ranked {
    pub rings: Vec<Hotspot>,
    /// Spansh's count of bodies within the radius with the ore in a ring (stops at 10000).
    pub bodies: u64,
    pub cut: Cut,
    /// Spansh requests made (a cached answer counts too).
    pub requests: usize,
    /// When Spansh was asked: the oldest answer used (unix seconds).
    pub fetched_at: i64,
}

/// How much of the radius a [`Ranked`] list covers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cut {
    /// The radius (for the nearest rings: the nearest [`BODIES`] bodies in it).
    Whole,
    /// Only rings with at least this many hotspots (see [`Hotspots::least`]).
    Least(u32),
    /// Only this many ly (see [`Hotspots::reach_ly`]).
    Reach(f64),
}

/// One band's page: the nearest [`BODIES`] bodies with a ring in the band, and how many there are.
struct Seen {
    band: Band,
    count: u64,
    bodies: Vec<Body>,
}

impl Seen {
    fn whole(&self) -> bool {
        self.bodies.len() as u64 >= self.count
    }

    /// Every ring in the band is known this far out, in ly: all of the radius when the page held
    /// every match, else as far as its farthest body.
    fn reach(&self, origin: &Origin) -> f64 {
        if self.whole() {
            f64::INFINITY
        } else {
            self.bodies.last().map_or(0.0, |b| ly_to(origin, b))
        }
    }
}

/// The least hotspot count from which every ring is known: every higher count is in a band
/// that came back whole.
fn floor(seen: &[Seen]) -> Option<u32> {
    let whole = |c: u32| seen.iter().any(|s| s.whole() && s.band.holds(c));
    let mut c = seen.iter().filter(|s| s.whole() && s.band.hi.is_none()).map(|s| s.band.lo).min()?;
    while c > 1 && whole(c - 1) {
        c -= 1;
    }
    Some(c)
}

/// The next band to ask for, or `None` when the one needed was asked already. With no count
/// known in full yet, a higher open band, raised to where the last one's bodies suggest about
/// [`AIM`] bodies are left; else the count just below the ones known.
fn next_band(seen: &[Seen], ore: &str) -> Option<Band> {
    let band = match floor(seen) {
        Some(1) => return None,
        Some(f) => Band { lo: f - 1, hi: Some(f - 1) },
        None => {
            let top = seen.iter().filter(|s| s.band.hi.is_none()).max_by_key(|s| s.band.lo)?;
            let most: Vec<u32> =
                top.bodies.iter().map(|b| b.rings.iter().map(|r| r.hotspots(ore)).max().unwrap_or(0)).collect();
            if most.is_empty() {
                return None;
            }
            let left = |c: u32| top.count as f64 * most.iter().filter(|&&m| m >= c).count() as f64 / most.len() as f64;
            let lo = (top.band.lo + 1..).find(|&c| left(c) <= AIM)?;
            Band { lo, hi: None }
        }
    };
    (!seen.iter().any(|s| s.band == band)).then_some(band)
}

/// The rings `seen` holds, most hotspots first, then nearest, each from a band that holds its
/// count (a body can also match through another of its rings). `done` when they are the
/// `limit` that outrank every other ring, or every ring. Otherwise, the rings of the counts
/// known in full ([`Cut::Least`]), or with none known, the rings within the distance every
/// count is known to ([`Cut::Reach`]).
fn rank(seen: &[Seen], origin: &Origin, want: &Want, range: f64, limit: usize) -> (Vec<Hotspot>, Cut, bool) {
    let mut rings: Vec<Hotspot> = Vec::new();
    for s in seen {
        for r in rings_with(&s.bodies, origin, want, range) {
            if s.band.holds(r.count) && !rings.iter().any(|x| x.ring == r.ring) {
                rings.push(r);
            }
        }
    }
    rings.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.hop.ly.total_cmp(&b.hop.ly)));
    let floor = floor(seen);
    // A ring is placed when every count above its own is known in full: the rings of its own
    // count nearer than it are known, as it came from a band that holds them.
    let placed = rings.iter().take_while(|r| floor.is_some_and(|f| f <= r.count + 1)).count();
    if placed >= limit || floor == Some(1) {
        rings.truncate(placed.min(limit));
        return (rings, Cut::Whole, true);
    }
    if let Some(f) = floor.filter(|&f| rings.first().is_some_and(|r| r.count >= f)) {
        rings.retain(|r| r.count >= f);
        rings.truncate(limit);
        return (rings, Cut::Least(f), false);
    }
    let reach = |c: u32| seen.iter().filter(|s| s.band.holds(c)).map(|s| s.reach(origin)).fold(0.0, f64::max);
    let top = seen.iter().map(|s| s.band.lo).max().unwrap_or(1);
    let known = (1..=top + 1).map(reach).fold(f64::INFINITY, f64::min);
    rings.retain(|r| r.hop.ly <= known);
    rings.truncate(limit);
    (rings, Cut::Reach(known), false)
}

/// The finder's list: the `limit` rings with the most hotspots of `want`'s ore within
/// `radius_ly` of `origin`, then nearest. Spansh sorts by distance only, so this asks for
/// every ring with the ore first ([`Band::ANY`], which also counts the bodies), then for bands
/// of hotspot counts, highest first, until the rings that could outrank the last one listed are
/// all known: at most [`REQUESTS`] requests. When those run out first, the list is cut to what
/// is known in full, and [`Ranked::cut`] says how.
pub fn most_hotspots(
    client: &Client,
    origin: &Origin,
    want: &Want,
    radius_ly: Option<u32>,
    range: f64,
    limit: usize,
) -> Result<Ranked, Error> {
    let ore = want.name();
    let mut seen: Vec<Seen> = Vec::new();
    let mut fetched_at = i64::MAX;
    let mut band = Some(Band::ANY);
    while let Some(b) = band {
        let got = client.spansh_bodies(&hotspot_search(origin, want, radius_ly, b, BODIES))?;
        fetched_at = fetched_at.min(got.fetched_at);
        seen.push(Seen { band: b, count: got.value.count, bodies: got.value.results });
        let (_, _, done) = rank(&seen, origin, want, range, limit);
        band = if done || seen.len() == REQUESTS { None } else { next_band(&seen, &ore) };
    }
    let (rings, cut, _) = rank(&seen, origin, want, range, limit);
    Ok(Ranked { rings, bodies: seen[0].count, cut, requests: seen.len(), fetched_at })
}

/// [`hotspots`] (nearest first) or [`most_hotspots`] for the app and the voice query: from
/// `here`, at most `limit` rings, with jumps counted on the star map.
pub fn hotspots_search(
    galaxy: &Galaxy,
    here: &Here,
    range: f64,
    want: &Want,
    radius_ly: Option<u32>,
    order_by: Order,
    limit: usize,
) -> Result<Hotspots, Error> {
    let origin = Origin::from(here);
    let limit = limit.max(1);
    let mut found = match order_by {
        Order::Nearest => hotspots(galaxy.remote(), &origin, want, radius_ly, range)?,
        Order::Most => most_hotspots(galaxy.remote(), &origin, want, radius_ly, range, limit)?,
    };
    found.rings.truncate(limit);
    let _ = galaxy.ensure_covered(here.pos);
    if let Ok(map) = galaxy.map() {
        for r in &mut found.rings {
            r.hop = map.hop(here.pos, r.system_pos, range);
        }
    }
    Ok(Hotspots {
        name: want.name(),
        want: want.clone(),
        here: here.clone(),
        range,
        radius_ly,
        bodies: found.bodies,
        least: if let Cut::Least(n) = found.cut { Some(n) } else { None },
        reach_ly: if let Cut::Reach(ly) = found.cut { Some(ly) } else { None },
        rings: found.rings,
        fetched_at: found.fetched_at,
    })
}
