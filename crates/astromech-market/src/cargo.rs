//! Where to sell the cargo in the hold (`Cargo.json`): the best station for the whole load and
//! the best station for each commodity.
//!
//! The whole load: every market near the origin that buys at least one of the commodities,
//! scored by what it would pay for everything it can take, `min(carried, demand) × price` per
//! commodity (demand caps matter for mined goods, which come in hundreds of tonnes). Each
//! commodity on its own: the single-commodity sell lookup ([`crate::prices`]) asking for demand
//! of at least the tonnes carried, so the answer takes the whole lot; when no market has that
//! much demand (mined loads run to hundreds of tonnes), the market that pays the most for the
//! part it takes.
//!
//! Mission cargo isn't the commander's to sell, stolen goods only sell at a black market, and
//! limpets are kept for the next trip, so those are left out and listed as such.

use crate::trade::{prices, rank_offers, Offer, Origin, Side};
use astromech_data::{Client, Error};
use astromech_galaxy::{Filters, Galaxy, Here, Hop, Pad};
use astromech_journal::CargoItem;
use serde::Serialize;
use std::collections::BTreeMap;

/// The id the search kit remembers the cargo lookup's filters under (voice uses them too).
pub const LOOKUP: &str = "trade.cargo";

/// One commodity to sell, merged across its inventory lines.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Load {
    pub symbol: String,
    pub name: String,
    pub tonnes: i64,
}

/// Cargo left out of the plan, and why.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Kept {
    pub name: String,
    pub tonnes: i64,
    pub reason: &'static str,
}

/// What can be sold from `inventory`, most tonnes first, and what is left out.
pub fn sellable(inventory: &[CargoItem]) -> (Vec<Load>, Vec<Kept>) {
    let mut load: Vec<Load> = vec![];
    let mut kept = vec![];
    for it in inventory {
        let free = it.count - it.stolen.clamp(0, it.count);
        if it.mission {
            kept.push(Kept { name: it.name.clone(), tonnes: it.count, reason: "mission cargo" });
            continue;
        }
        if it.symbol == "drones" {
            kept.push(Kept { name: it.name.clone(), tonnes: it.count, reason: "limpets" });
            continue;
        }
        if it.count > free {
            kept.push(Kept { name: it.name.clone(), tonnes: it.count - free, reason: "stolen" });
        }
        if free <= 0 {
            continue;
        }
        match load.iter_mut().find(|l| l.symbol == it.symbol) {
            Some(l) => l.tonnes += free,
            None => {
                // Cargo.json leaves out Name_Localised for some goods: use the catalogue's name.
                let name = if it.name == it.symbol { crate::catalog::name_of(&it.symbol) } else { it.name.clone() };
                load.push(Load { symbol: it.symbol.clone(), name, tonnes: free })
            }
        }
    }
    load.sort_by(|a, b| b.tonnes.cmp(&a.tonnes).then(a.symbol.cmp(&b.symbol)));
    (load, kept)
}

/// One commodity sold at a station.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SaleLine {
    pub commodity: String,
    pub name: String,
    pub carried: i64,
    /// What the station takes: the tonnes carried, capped by its demand.
    pub sold: i64,
    pub price: i64,
    pub proceeds: i64,
}

/// A station's offer for the whole load.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StationSale {
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
    /// The oldest of its prices used here (RFC 3339).
    pub updated_at: String,
    /// Credits for everything it takes.
    pub total: i64,
    /// Tonnes it takes, of all those carried.
    pub sold: i64,
    /// The commodities it buys, in load order.
    pub lines: Vec<SaleLine>,
}

/// Every station in `prices` (one per [`Load`] line, same order), best total first; nearer
/// first on a tie.
pub fn whole_load(load: &[Load], prices: &[Vec<Offer>]) -> Vec<StationSale> {
    let mut by_market: BTreeMap<i64, StationSale> = BTreeMap::new();
    for (l, offers) in load.iter().zip(prices) {
        for o in offers {
            let sold = l.tonnes.min(o.volume.max(0));
            let line = SaleLine {
                commodity: l.symbol.clone(),
                name: l.name.clone(),
                carried: l.tonnes,
                sold,
                price: o.price,
                proceeds: sold * o.price,
            };
            let s = by_market.entry(o.market_id).or_insert_with(|| StationSale {
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
                total: 0,
                sold: 0,
                lines: vec![],
            });
            if o.updated_at < s.updated_at {
                s.updated_at = o.updated_at.clone();
            }
            // Ardent lists a market once per commodity; keep the first if it ever repeats.
            if s.lines.iter().any(|x| x.commodity == line.commodity) {
                continue;
            }
            s.total += line.proceeds;
            s.sold += line.sold;
            s.lines.push(line);
        }
    }
    let mut out: Vec<StationSale> = by_market.into_values().filter(|s| s.total > 0).collect();
    out.sort_by(|a, b| b.total.cmp(&a.total).then(a.hop.ly.total_cmp(&b.hop.ly)));
    out
}

/// One commodity's best station on its own.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CommoditySale {
    pub commodity: String,
    pub name: String,
    pub carried: i64,
    /// The single-commodity sell lookup's best market with demand for all of it.
    pub best: Option<Offer>,
    /// With no market that takes it all: the market that pays the most for what it does take
    /// (`min(carried, demand)` tonnes; its `volume` is its demand). Never set alongside `best`.
    pub partial: Option<Offer>,
}

/// Where to sell the hold.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CargoPlan {
    pub load: Vec<Load>,
    pub kept: Vec<Kept>,
    /// Stations for the whole load, best total first.
    pub stations: Vec<StationSale>,
    /// The best station for each commodity, in load order.
    pub per_commodity: Vec<CommoditySale>,
    /// When Ardent was asked (the oldest answer used; unix seconds). 0 with nothing to sell.
    pub fetched_at: i64,
}

/// The filters for one commodity sold on its own: demand for at least `tonnes`.
pub fn single_filters(filters: &Filters, tonnes: i64) -> Filters {
    let need = u32::try_from(tonnes.max(0)).unwrap_or(u32::MAX).max(filters.min_volume.unwrap_or(0));
    Filters { min_volume: Some(need), ..filters.clone() }
}

/// The market among `offers` that pays the most in all for up to `tonnes` (capped by its
/// demand); nearer first on a tie. Only markets that pass `filters` and want some.
fn best_partial(tonnes: i64, offers: &[Offer], filters: &Filters, now_s: i64) -> Option<Offer> {
    let mut kept = rank_offers(Side::Sell, offers.to_vec(), filters, now_s);
    kept.retain(|o| o.volume > 0);
    kept.into_iter().max_by(|a, b| {
        (tonnes.min(a.volume) * a.price).cmp(&(tonnes.min(b.volume) * b.price)).then(b.hop.ly.total_cmp(&a.hop.ly))
    })
}

/// Plan the sale of `inventory` near `origin`. One Ardent request per commodity (cached), plus
/// one more for a commodity whose answer was full, so its markets with enough demand aren't cut.
pub fn plan(
    client: &Client,
    origin: &Origin,
    inventory: &[CargoItem],
    filters: &Filters,
    range: f64,
    now_s: i64,
) -> Result<CargoPlan, Error> {
    let (load, kept) = sellable(inventory);
    // The whole load takes partial demand, so the demand filter doesn't apply to it.
    let whole = Filters { min_volume: None, ..filters.clone() };
    let symbols: Vec<&str> = load.iter().map(|l| l.symbol.as_str()).collect();
    let all = prices(client, origin, Side::Sell, &symbols, &whole, range, now_s)?;
    let mut per_commodity = vec![];
    let mut offers: Vec<Vec<Offer>> = vec![];
    for (l, p) in load.iter().zip(&all) {
        let single = single_filters(filters, l.tonnes);
        let mut whole_offers = p.offers.clone();
        let ranked = if p.truncated {
            // The full answer may have cut the markets with demand for the whole lot: they
            // count for the whole load as well as on their own.
            let again =
                prices(client, origin, Side::Sell, &[l.symbol.as_str()], &single, range, now_s)?.remove(0).offers;
            for o in &again {
                if !whole_offers.iter().any(|w| w.market_id == o.market_id) {
                    whole_offers.push(o.clone());
                }
            }
            again
        } else {
            rank_offers(Side::Sell, p.offers.clone(), &single, now_s)
        };
        let best = ranked.into_iter().next();
        let partial = if best.is_some() { None } else { best_partial(l.tonnes, &whole_offers, filters, now_s) };
        per_commodity.push(CommoditySale {
            commodity: l.symbol.clone(),
            name: l.name.clone(),
            carried: l.tonnes,
            best,
            partial,
        });
        offers.push(whole_offers);
    }
    Ok(CargoPlan {
        stations: whole_load(&load, &offers),
        fetched_at: all.iter().map(|p| p.fetched_at).min().unwrap_or(0),
        load,
        kept,
        per_commodity,
    })
}

/// [`plan`] for the app: the best `limit` whole-load stations, with jumps on the star map.
pub fn plan_search(
    galaxy: &Galaxy,
    here: &Here,
    range: f64,
    inventory: &[CargoItem],
    filters: &Filters,
    limit: usize,
    now_s: i64,
) -> Result<CargoPlan, Error> {
    let mut p = plan(galaxy.remote(), &Origin::from(here), inventory, filters, range, now_s)?;
    let _ = galaxy.ensure_covered(here.pos);
    p.stations.truncate(limit.max(1));
    if let Ok(map) = galaxy.map() {
        for s in &mut p.stations {
            s.hop = map.hop(here.pos, s.system_pos, range);
        }
        for c in &mut p.per_commodity {
            for o in c.best.iter_mut().chain(c.partial.iter_mut()) {
                o.hop = map.hop(here.pos, o.system_pos, range);
            }
        }
    }
    Ok(p)
}

/// The hold as `Cargo.json` in `journal_dir` has it (empty if the file is missing or unreadable).
pub fn from_journal_dir(journal_dir: &std::path::Path) -> Vec<CargoItem> {
    std::fs::read(journal_dir.join("Cargo.json"))
        .ok()
        .and_then(|b| astromech_journal::parse_cargo(&b))
        .map(|c| c.inventory)
        .unwrap_or_default()
}
