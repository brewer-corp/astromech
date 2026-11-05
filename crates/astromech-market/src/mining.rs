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
//! lists them most hotspots first within a radius; the voice query says the nearest.

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

/// The least reserve level a ring must have. Spansh files it on the ringed body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Hotspots {
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

/// The bodies search for rings holding `want`'s ore around `origin`, within `radius_ly` when
/// given, nearest first.
pub fn hotspot_search(origin: &Origin, want: &Want, radius_ly: Option<u32>, size: usize) -> Search {
    let reference = match origin.pos {
        Some(p) => Reference::Coords(p),
        None => Reference::System(origin.system.clone()),
    };
    let mut q = Search::near(reference).size(size).ring_signal(&want.name());
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
            let ly = match origin.pos {
                Some(p) => dist(p, b.pos()),
                None => b.distance,
            };
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

/// Put `rings` in `order` (they come nearest first; the sort is stable).
pub fn order(rings: &mut [Hotspot], order: Order) {
    if order == Order::Most {
        rings.sort_by_key(|r| std::cmp::Reverse(r.count));
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
) -> Result<(Vec<Hotspot>, i64), Error> {
    let got = client.spansh_bodies(&hotspot_search(origin, want, radius_ly, BODIES))?;
    Ok((rings_with(&got.value.results, origin, want, range), got.fetched_at))
}

/// [`hotspots`] for the app and the voice query: from `here`, in `order`, at most `limit`
/// rings, with jumps counted on the star map.
pub fn hotspots_search(
    galaxy: &Galaxy,
    here: &Here,
    range: f64,
    want: &Want,
    radius_ly: Option<u32>,
    order_by: Order,
    limit: usize,
) -> Result<Hotspots, Error> {
    let (mut rings, fetched_at) = hotspots(galaxy.remote(), &Origin::from(here), want, radius_ly, range)?;
    order(&mut rings, order_by);
    rings.truncate(limit.max(1));
    let _ = galaxy.ensure_covered(here.pos);
    if let Ok(map) = galaxy.map() {
        for r in &mut rings {
            r.hop = map.hop(here.pos, r.system_pos, range);
        }
    }
    Ok(Hotspots { name: want.name(), want: want.clone(), here: here.clone(), range, radius_ly, rings, fetched_at })
}
