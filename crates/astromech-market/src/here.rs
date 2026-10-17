//! This market against nearby prices: when docked, each commodity in the hold and each one for
//! sale here (`Market.json`) next to the best price a station near here pays for it. "Sell gold
//! for +3,921/t 15 ly away"; "silver bought here sells for +3,321/t at Arc City".
//!
//! Both questions are the sell lookup ([`crate::prices`]): one Ardent request per commodity,
//! cached 15 minutes. A market lists dozens of goods, so only [`MAX_LOOKUPS`] are asked about per
//! dock: what is carried first (the most tonnes first; past the cap a line says it wasn't looked
//! up), then the goods for sale here priced furthest below their galactic average (the market's
//! own `MeanPrice`), the usual first guess at a good buy. Ardent
//! leaves the searched system's own markets out of a nearby answer (observed 16 Oct 2026), so
//! "nearby" means other systems.

use crate::cargo::{sellable, single_filters, Kept, Load};
use crate::trade::{prices, rank_offers, Offer, Origin, Side};
use astromech_data::{Client, Error};
use astromech_galaxy::{Filters, Galaxy, Here};
use astromech_journal::companion::{parse_fresh, CompanionFile, CompanionKind, MarketItem, MarketSnap};
use astromech_journal::CargoItem;
use serde::Serialize;

/// The most commodities looked up per dock (each is one Ardent request, about a second each).
pub const MAX_LOOKUPS: usize = 12;

/// `$gold_name;` -> `gold`, the symbol Ardent and `Cargo.json` use.
pub fn symbol(name: &str) -> String {
    let n = name.trim().to_lowercase();
    let n = n.strip_prefix('$').unwrap_or(&n);
    n.strip_suffix("_name;").unwrap_or(n).to_string()
}

/// Limpets are bought for the trip, not traded.
fn traded(symbol: &str) -> bool {
    symbol != "drones"
}

/// For sale here: the station asks a price and has stock.
fn for_sale(i: &MarketItem) -> bool {
    i.buy_price > 0 && i.stock > 0 && traded(&symbol(&i.name))
}

/// The commodities looked up for one market, at most `max` in all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookups {
    /// Carried commodities looked up: the first `max` of the load (most tonnes first).
    pub carried: Vec<String>,
    /// Goods for sale here looked up, furthest below their average price first, filling what
    /// the load leaves of `max`.
    pub for_sale: Vec<String>,
    /// Goods for sale here not looked up.
    pub unchecked: Vec<String>,
}

impl Lookups {
    /// Every symbol to ask about, carried first, each once.
    pub fn symbols(&self) -> Vec<&str> {
        let mut out: Vec<&str> = self.carried.iter().map(String::as_str).collect();
        for s in &self.for_sale {
            if !out.contains(&s.as_str()) {
                out.push(s);
            }
        }
        out
    }
}

/// What to look up for this market and the `load` (everything carried that can be sold, most
/// tonnes first): the load first, then the goods for sale here, never more than `max`.
pub fn plan(market: &MarketSnap, load: &[Load], max: usize) -> Lookups {
    let carried: Vec<String> = load.iter().take(max).map(|l| l.symbol.clone()).collect();
    let mut goods: Vec<&MarketItem> = market.items.iter().filter(|i| for_sale(i)).collect();
    goods.sort_by_key(|i| (i.buy_price - i.mean_price, symbol(&i.name)));
    let room = max - carried.len();
    let mut symbols: Vec<String> = goods.iter().map(|i| symbol(&i.name)).collect();
    let unchecked = symbols.split_off(room.min(symbols.len()));
    Lookups { carried, for_sale: symbols, unchecked }
}

/// One commodity here against the best nearby price.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Line {
    pub commodity: String,
    pub name: String,
    /// Tonnes carried that can be sold (0 for a line of goods for sale here).
    pub carried: i64,
    /// Whether nearby prices were asked for: carried commodities past [`MAX_LOOKUPS`] aren't.
    pub checked: bool,
    /// What this station pays per tonne; `None`: it doesn't buy it.
    pub sell_here: Option<i64>,
    /// What this station asks per tonne; `None`: not for sale here.
    pub buy_here: Option<i64>,
    /// Tonnes this station has for sale.
    pub stock: i64,
    /// The best price a nearby station pays (for the carried, one with demand for all of it).
    pub best: Option<Offer>,
    /// Credits per tonne more than here: the best price less what this station pays (carried)
    /// or asks (for sale). `None` without a best price, or for cargo this station doesn't buy.
    pub gain: Option<i64>,
}

/// This market against nearby prices.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Comparison {
    pub market_id: Option<i64>,
    pub station: String,
    pub system: String,
    /// When the game wrote `Market.json`.
    pub seen: String,
    /// The hold, in load order (most tonnes first).
    pub carried: Vec<Line>,
    /// Goods for sale here, the biggest gain first.
    pub for_sale: Vec<Line>,
    /// Names of goods for sale here not looked up ([`MAX_LOOKUPS`]).
    pub unchecked: Vec<String>,
    /// Cargo left out (mission cargo, stolen goods, limpets).
    pub kept: Vec<Kept>,
    /// When Ardent was asked (the oldest answer used; unix seconds). 0 with nothing looked up.
    pub fetched_at: i64,
}

fn name_of(item: Option<&MarketItem>, symbol: &str) -> String {
    item.and_then(|i| i.name_localised.clone()).unwrap_or_else(|| crate::catalog::name_of(symbol))
}

/// Compare `market` and the hold (`inventory`) with the nearby markets in `nearby` (the sell
/// lookup's ranked answers, one per commodity, any order) that pass `filters`. Pure.
pub fn compare(
    market: &MarketSnap,
    inventory: &[CargoItem],
    nearby: &[crate::Prices],
    filters: &Filters,
    max: usize,
    now_s: i64,
) -> Comparison {
    let (load, kept) = sellable(inventory);
    let lookups = plan(market, &load, max);
    let item = |s: &str| market.items.iter().find(|i| symbol(&i.name) == s);
    let offers = |s: &str| nearby.iter().find(|p| p.commodity == s).map(|p| p.offers.clone()).unwrap_or_default();

    let carried = load
        .iter()
        .map(|l| {
            let here = item(&l.symbol);
            let sell_here = here.filter(|i| i.sell_price > 0).map(|i| i.sell_price);
            let checked = lookups.carried.contains(&l.symbol);
            let best = rank_offers(Side::Sell, offers(&l.symbol), &single_filters(filters, l.tonnes), now_s)
                .into_iter()
                .next();
            Line {
                commodity: l.symbol.clone(),
                name: l.name.clone(),
                carried: l.tonnes,
                checked,
                sell_here,
                buy_here: here.filter(|i| for_sale(i)).map(|i| i.buy_price),
                stock: here.map_or(0, |i| i.stock),
                gain: best.as_ref().zip(sell_here).map(|(b, s)| b.price - s),
                best,
            }
        })
        .collect();

    let mut for_sale: Vec<Line> = lookups
        .for_sale
        .iter()
        .filter_map(|s| {
            let here = item(s)?;
            let best = rank_offers(Side::Sell, offers(s), filters, now_s).into_iter().next();
            Some(Line {
                commodity: s.clone(),
                name: name_of(Some(here), s),
                carried: 0,
                checked: true,
                sell_here: (here.sell_price > 0).then_some(here.sell_price),
                buy_here: Some(here.buy_price),
                stock: here.stock,
                gain: best.as_ref().map(|b| b.price - here.buy_price),
                best,
            })
        })
        .collect();
    for_sale.sort_by(|a, b| b.gain.unwrap_or(i64::MIN).cmp(&a.gain.unwrap_or(i64::MIN)));

    Comparison {
        market_id: market.market_id,
        station: market.station_name.clone(),
        system: market.star_system.clone(),
        seen: market.timestamp.clone(),
        carried,
        for_sale,
        unchecked: lookups.unchecked.iter().map(|s| name_of(item(s), s)).collect(),
        kept,
        fetched_at: nearby.iter().map(|p| p.fetched_at).min().unwrap_or(0),
    }
}

/// Look up the best nearby prices for this market and the hold near `origin` (the docked system)
/// and [`compare`]. One Ardent request per commodity looked up, at most [`MAX_LOOKUPS`].
pub fn compare_here(
    client: &Client,
    origin: &Origin,
    market: &MarketSnap,
    inventory: &[CargoItem],
    filters: &Filters,
    range: f64,
    now_s: i64,
) -> Result<Comparison, Error> {
    let (load, _) = sellable(inventory);
    let lookups = plan(market, &load, MAX_LOOKUPS);
    let symbols = lookups.symbols();
    // The carried need demand for all of it, checked in `compare`; ask without that limit so one
    // answer serves both lists.
    let ask = Filters { min_volume: None, ..filters.clone() };
    let nearby = prices(client, origin, Side::Sell, &symbols, &ask, range, now_s)?;
    Ok(compare(market, inventory, &nearby, filters, MAX_LOOKUPS, now_s))
}

/// [`compare_here`] for the app, from a located system, with jumps counted on the star map.
pub fn compare_search(
    galaxy: &Galaxy,
    here: &Here,
    range: f64,
    market: &MarketSnap,
    inventory: &[CargoItem],
    filters: &Filters,
    now_s: i64,
) -> Result<Comparison, Error> {
    let mut c = compare_here(galaxy.remote(), &Origin::from(here), market, inventory, filters, range, now_s)?;
    let _ = galaxy.ensure_covered(here.pos);
    if let Ok(map) = galaxy.map() {
        for b in c.carried.iter_mut().chain(c.for_sale.iter_mut()).filter_map(|l| l.best.as_mut()) {
            b.hop = map.hop(here.pos, b.system_pos, range);
        }
    }
    Ok(c)
}

/// The market in `bytes` (`Market.json`'s text), or `None` if it is unreadable.
pub fn parse_market(bytes: &[u8]) -> Option<MarketSnap> {
    // No event to be fresher than: which station it is for is the caller's check.
    match parse_fresh(CompanionKind::Market, Some(bytes), "") {
        Ok(CompanionFile::Market(m)) => Some(m),
        _ => None,
    }
}

/// The market `Market.json` in `journal_dir` has (the last one the game opened), if readable.
pub fn from_journal_dir(journal_dir: &std::path::Path) -> Option<MarketSnap> {
    parse_market(&std::fs::read(journal_dir.join("Market.json")).ok()?)
}
