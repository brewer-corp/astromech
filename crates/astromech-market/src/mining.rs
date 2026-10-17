//! Mining lookups: where to sell a load of ore ([`sale`]) and the nearest ring with a hotspot
//! of an ore ([`hotspots`]). The ores are `astromech_journal::ore::VALUE_ORDER`, the order the
//! prospector callouts rank them in.
//!
//! The sale is the hold's sale plan ([`crate::cargo`]) for one commodity: stations by what they
//! would pay for the load with each station's demand capping what it takes (mining loads run to
//! hundreds of tonnes, so the highest price is often a market that wants a fraction of it), and
//! the best market with demand for all of it. Hotspots are Spansh's bodies search on ring
//! signals, nearest first, one row for each ring that holds the ore (the search matches a body
//! with the ore in any ring).

use crate::cargo::{plan, StationSale};
use crate::catalog;
use crate::trade::{Offer, Origin};
use astromech_data::spansh::{Body, Reference, Search};
use astromech_data::{Client, Error};
use astromech_galaxy::{dist, estimate, Filters, Galaxy, Here, Hop};
use astromech_journal::CargoItem;
use serde::Serialize;

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
    #[serde(flatten)]
    pub hop: Hop,
    /// Light seconds from the arrival star to the body.
    pub arrival_ls: Option<f64>,
    /// When the ring's hotspots were last reported (RFC 3339).
    pub updated_at: Option<String>,
}

/// The bodies search for rings with `symbol` (a symbol from the ore list) around `origin`.
pub fn hotspot_search(origin: &Origin, symbol: &str, size: usize) -> Search {
    let reference = match origin.pos {
        Some(p) => Reference::Coords(p),
        None => Reference::System(origin.system.clone()),
    };
    Search::near(reference).size(size).ring_signal(&catalog::name_of(symbol))
}

/// The rings in `bodies` holding the ore `symbol`, nearest first. Spansh matches a body with
/// the ore in any ring, so each ring is checked here.
pub fn rings_with(bodies: &[Body], origin: &Origin, symbol: &str, range: f64) -> Vec<Hotspot> {
    let ore = catalog::name_of(symbol);
    let mut out: Vec<Hotspot> = bodies
        .iter()
        .flat_map(|b| {
            let ly = match origin.pos {
                Some(p) => dist(p, b.pos()),
                None => b.distance,
            };
            let ore = &ore;
            b.rings.iter().filter(move |r| r.hotspots(ore) > 0).map(move |r| Hotspot {
                system: b.system_name.clone(),
                system_pos: b.pos(),
                body: b.name.clone(),
                ring: r.name.clone(),
                ring_type: r.ring_type.clone(),
                count: r.hotspots(ore),
                hop: Hop { ly, jumps: estimate(ly, range), exact: false },
                arrival_ls: b.distance_to_arrival,
                updated_at: r.signals_updated_at.clone(),
            })
        })
        .collect();
    out.sort_by(|a, b| a.hop.ly.total_cmp(&b.hop.ly));
    out
}

/// The nearest `limit` rings with a hotspot of the ore `symbol` from `origin`.
pub fn hotspots(
    client: &Client,
    origin: &Origin,
    symbol: &str,
    range: f64,
    limit: usize,
) -> Result<Vec<Hotspot>, Error> {
    let limit = limit.max(1);
    let got = client.spansh_bodies(&hotspot_search(origin, symbol, limit))?;
    let mut rings = rings_with(&got.value.results, origin, symbol, range);
    rings.truncate(limit);
    Ok(rings)
}

/// [`hotspots`] for the app: jumps counted on the star map.
pub fn hotspots_search(
    galaxy: &Galaxy,
    here: &Here,
    range: f64,
    symbol: &str,
    limit: usize,
) -> Result<Vec<Hotspot>, Error> {
    let mut rings = hotspots(galaxy.remote(), &Origin::from(here), symbol, range, limit)?;
    let _ = galaxy.ensure_covered(here.pos);
    if let Ok(map) = galaxy.map() {
        for r in &mut rings {
            r.hop = map.hop(here.pos, r.system_pos, range);
        }
    }
    Ok(rings)
}
