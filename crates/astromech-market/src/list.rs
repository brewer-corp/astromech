//! Where to buy a shopping list (the colonisation tracker's remaining commodities): every market
//! near the origin that sells at least one of them, ranked by how many of the list it stocks,
//! nearer first among equals, with each commodity's stock, price and the station's pad size.
//!
//! One buy lookup ([`crate::prices`]) per commodity, with the lookup's filters (radius, pad, data
//! age, carriers, Odyssey, arrival, and a minimum stock: a market below it doesn't count as
//! stocking that commodity).

use crate::trade::{prices, Offer, Origin, Side};
use astromech_data::{Client, Error};
use astromech_galaxy::{Filters, Galaxy, Here, Hop, Pad};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The id the search kit remembers this lookup's filters under.
pub const LOOKUP: &str = "colonisation.buy";

/// One commodity still to buy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Need {
    /// Journal and Ardent symbol: "steel".
    pub symbol: String,
    pub name: String,
    pub tonnes: i64,
}

/// One listed commodity a station sells.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StockLine {
    pub commodity: String,
    pub name: String,
    pub needed: i64,
    /// Tonnes the station has.
    pub stock: i64,
    /// Credits per tonne.
    pub price: i64,
}

/// A station selling some of the list.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Stockist {
    pub market_id: i64,
    pub station: String,
    pub system: String,
    #[serde(skip)]
    pub system_pos: [f64; 3],
    #[serde(flatten)]
    pub hop: Hop,
    pub pad: Option<Pad>,
    pub arrival_ls: Option<f64>,
    pub fleet_carrier: bool,
    pub odyssey: bool,
    /// The oldest of its listings used here (RFC 3339).
    pub updated_at: String,
    /// How many of the listed commodities it sells.
    pub stocked: usize,
    /// What it sells, in list order.
    pub lines: Vec<StockLine>,
    /// Names of the listed commodities it doesn't sell, in list order.
    pub missing: Vec<String>,
}

/// Where to buy the list.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListPlan {
    /// Stations, most of the list first, then nearest.
    pub stations: Vec<Stockist>,
    /// Names of listed commodities no market in range sells.
    pub unavailable: Vec<String>,
    /// Names of listed commodities whose answer was full, so nearer markets may be missing.
    pub truncated: Vec<String>,
    /// When Ardent was asked (the oldest answer used; unix seconds). 0 with nothing to buy.
    pub fetched_at: i64,
}

/// Every station in `offers` (one list per need, same order), most needs stocked first, then
/// nearer, then by market id.
pub fn stockists(needs: &[Need], offers: &[Vec<Offer>]) -> Vec<Stockist> {
    let mut by_market: BTreeMap<i64, Stockist> = BTreeMap::new();
    for (n, list) in needs.iter().zip(offers) {
        for o in list {
            let s = by_market.entry(o.market_id).or_insert_with(|| Stockist {
                market_id: o.market_id,
                station: o.station.clone(),
                system: o.system.clone(),
                system_pos: o.system_pos,
                hop: o.hop,
                pad: o.pad,
                arrival_ls: o.arrival_ls,
                fleet_carrier: o.fleet_carrier,
                odyssey: o.odyssey,
                updated_at: o.updated_at.clone(),
                stocked: 0,
                lines: vec![],
                missing: vec![],
            });
            // Ardent lists a market once per commodity; keep the first if it ever repeats.
            if s.lines.iter().any(|l| l.commodity == n.symbol) {
                continue;
            }
            if o.updated_at < s.updated_at {
                s.updated_at = o.updated_at.clone();
            }
            s.lines.push(StockLine {
                commodity: n.symbol.clone(),
                name: n.name.clone(),
                needed: n.tonnes,
                stock: o.volume,
                price: o.price,
            });
        }
    }
    let mut out: Vec<Stockist> = by_market.into_values().collect();
    for s in &mut out {
        s.stocked = s.lines.len();
        s.missing =
            needs.iter().filter(|n| !s.lines.iter().any(|l| l.commodity == n.symbol)).map(|n| n.name.clone()).collect();
    }
    out.sort_by(|a, b| {
        b.stocked.cmp(&a.stocked).then(a.hop.ly.total_cmp(&b.hop.ly)).then(a.market_id.cmp(&b.market_id))
    });
    out
}

/// The needs worth asking about: tonnes left, each symbol once (first wins), lowercase symbols.
fn wanted(needs: &[Need]) -> Vec<Need> {
    let mut out: Vec<Need> = vec![];
    for n in needs {
        let symbol = n.symbol.trim().to_lowercase();
        if n.tonnes > 0 && !symbol.is_empty() && !out.iter().any(|w| w.symbol == symbol) {
            out.push(Need { symbol, ..n.clone() });
        }
    }
    out
}

/// Where to buy `needs` near `origin`. One Ardent request per commodity (cached 15 minutes).
pub fn plan(
    client: &Client,
    origin: &Origin,
    needs: &[Need],
    filters: &Filters,
    range: f64,
    now_s: i64,
) -> Result<ListPlan, Error> {
    let needs = wanted(needs);
    let symbols: Vec<&str> = needs.iter().map(|n| n.symbol.as_str()).collect();
    let all = prices(client, origin, Side::Buy, &symbols, filters, range, now_s)?;
    let unavailable =
        needs.iter().zip(&all).filter(|(_, p)| p.offers.is_empty()).map(|(n, _)| n.name.clone()).collect();
    let truncated = needs.iter().zip(&all).filter(|(_, p)| p.truncated).map(|(n, _)| n.name.clone()).collect();
    let offers: Vec<Vec<Offer>> = all.iter().map(|p| p.offers.clone()).collect();
    Ok(ListPlan {
        stations: stockists(&needs, &offers),
        unavailable,
        truncated,
        fetched_at: all.iter().map(|p| p.fetched_at).min().unwrap_or(0),
    })
}

/// [`plan`] for the app: the best `limit` stations, with jumps counted on the star map.
pub fn plan_search(
    galaxy: &Galaxy,
    here: &Here,
    range: f64,
    needs: &[Need],
    filters: &Filters,
    limit: usize,
    now_s: i64,
) -> Result<ListPlan, Error> {
    let mut p = plan(galaxy.remote(), &Origin::from(here), needs, filters, range, now_s)?;
    // Fill-in is best effort: without it the jump counts stay estimates.
    let _ = galaxy.ensure_covered(here.pos);
    p.stations.truncate(limit.max(1));
    if let Ok(map) = galaxy.map() {
        for s in &mut p.stations {
            s.hop = map.hop(here.pos, s.system_pos, range);
        }
    }
    Ok(p)
}
