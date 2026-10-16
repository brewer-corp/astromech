//! Where to sell or buy commodities near a system. One Ardent nearby search per commodity
//! (imports to sell, exports to buy), then every market is checked against the lookup's
//! [`Filters`] here as well, because a cached or recorded answer may have been asked with other
//! filters, and ranked: selling by the price paid, highest first; buying by the price asked,
//! lowest first; nearer first on a tie.

use astromech_data::ardent::{Listing, MarketFilter, Trade, NEARBY_LIMIT};
use astromech_data::{Client, Error};
use astromech_galaxy::{dist, estimate, Facts, Filters, Galaxy, Here, Hop, Pad};
use serde::{Deserialize, Serialize};

/// Search radius when the commander hasn't picked one, in light years.
pub const DEFAULT_DISTANCE_LY: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    /// Where the commander can sell: stations that import the commodity.
    Sell,
    /// Where the commander can buy: stations that export it.
    Buy,
}

impl Side {
    fn trade(self) -> Trade {
        match self {
            Side::Sell => Trade::Imports,
            Side::Buy => Trade::Exports,
        }
    }

    /// The id the search kit remembers this lookup's filters under.
    pub fn lookup(self) -> &'static str {
        match self {
            Side::Sell => "trade.sell",
            Side::Buy => "trade.buy",
        }
    }
}

/// Where a search measures from. Without a position, distances are Ardent's (whole light years).
#[derive(Debug, Clone, PartialEq)]
pub struct Origin {
    pub system: String,
    pub pos: Option<[f64; 3]>,
}

impl From<&Here> for Origin {
    fn from(h: &Here) -> Self {
        Origin { system: h.system.clone(), pos: Some(h.pos) }
    }
}

/// One market for one commodity.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Offer {
    /// Symbol, e.g. "painite".
    pub commodity: String,
    pub market_id: i64,
    pub station: String,
    pub station_type: Option<String>,
    pub system: String,
    #[serde(skip)]
    pub system_pos: [f64; 3],
    #[serde(flatten)]
    pub hop: Hop,
    /// Credits per tonne: what the station pays (selling) or asks (buying).
    pub price: i64,
    /// Tonnes: the station's demand (selling) or stock (buying).
    pub volume: i64,
    pub pad: Option<Pad>,
    /// Light seconds from the arrival star.
    pub arrival_ls: Option<f64>,
    pub fleet_carrier: bool,
    pub odyssey: bool,
    /// When the market was last reported (RFC 3339).
    pub updated_at: String,
}

/// One commodity's markets, best first.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Prices {
    pub commodity: String,
    pub side: Side,
    /// When Ardent was asked (unix seconds; may be a cached answer).
    pub fetched_at: i64,
    /// Ardent's answer was full ([`NEARBY_LIMIT`]), so worse prices may be missing.
    pub truncated: bool,
    pub offers: Vec<Offer>,
}

fn pad_of(n: Option<u8>) -> Option<Pad> {
    match n? {
        1 => Some(Pad::S),
        2 => Some(Pad::M),
        3 => Some(Pad::L),
        _ => None,
    }
}

fn pad_num(p: Pad) -> u8 {
    match p {
        Pad::S => 1,
        Pad::M => 2,
        Pad::L => 3,
    }
}

/// Ardent's side of `filters` (it filters before its 1000-row cap, so asking it matters).
pub fn market_filter(filters: &Filters) -> MarketFilter {
    MarketFilter {
        max_distance: Some(filters.max_distance_ly.unwrap_or(DEFAULT_DISTANCE_LY) as f64),
        max_days_ago: filters.max_age_days,
        min_landing_pad: filters.pad.map(pad_num),
        fleet_carriers: filters.exclude_carriers.then_some(false),
        min_volume: filters.min_volume.map(u64::from),
    }
}

/// `listing` as an offer, or `None` if the station doesn't trade it that way (price 0).
fn offer(side: Side, l: &Listing, origin: &Origin, range: f64) -> Option<Offer> {
    let (price, volume) = match side {
        Side::Sell => (l.sell_price, l.demand),
        Side::Buy => (l.buy_price, l.stock),
    };
    if price <= 0 {
        return None;
    }
    let system_pos = [l.system_x, l.system_y, l.system_z];
    let ly = match origin.pos {
        Some(p) => dist(p, system_pos),
        None => l.distance.unwrap_or(0.0),
    };
    let kind = l.station_type.as_deref();
    Some(Offer {
        commodity: l.commodity_name.clone(),
        market_id: l.market_id,
        station: l.station_name.clone(),
        station_type: l.station_type.clone(),
        system: l.system_name.clone(),
        system_pos,
        hop: Hop { ly, jumps: estimate(ly, range), exact: false },
        price,
        volume,
        pad: pad_of(l.max_landing_pad_size),
        arrival_ls: l.distance_to_arrival,
        fleet_carrier: kind == Some("FleetCarrier"),
        odyssey: kind == Some("OnFootSettlement"),
        updated_at: l.updated_at.clone(),
    })
}

/// The markets in `listings` that pass `filters`, best first. Jumps are straight-line
/// estimates at `range` until [`search`] counts them on the star map.
pub fn rank(
    side: Side,
    listings: &[Listing],
    origin: &Origin,
    range: f64,
    filters: &Filters,
    now_s: i64,
) -> Vec<Offer> {
    let filters =
        Filters { max_distance_ly: Some(filters.max_distance_ly.unwrap_or(DEFAULT_DISTANCE_LY)), ..filters.clone() };
    let mut out: Vec<Offer> = listings
        .iter()
        .filter_map(|l| offer(side, l, origin, range))
        .filter(|o| {
            let facts = Facts {
                pad: o.pad,
                arrival_ls: o.arrival_ls,
                fleet_carrier: o.fleet_carrier,
                odyssey: o.odyssey,
                updated_at: Some(&o.updated_at),
                ly: Some(o.hop.ly),
                volume: Some(o.volume),
            };
            filters.keeps(&facts, now_s)
        })
        .collect();
    out.sort_by(|a, b| {
        let by_price = match side {
            Side::Sell => b.price.cmp(&a.price),
            Side::Buy => a.price.cmp(&b.price),
        };
        by_price.then(a.hop.ly.total_cmp(&b.hop.ly))
    });
    out
}

/// Where to sell or buy each of `commodities` (symbols) near `origin`, best first, one
/// [`Prices`] per commodity in the order asked. The colonisation shopping list asks for many
/// commodities at once; each is one Ardent request (cached 15 minutes).
pub fn prices(
    client: &Client,
    origin: &Origin,
    side: Side,
    commodities: &[&str],
    filters: &Filters,
    range: f64,
    now_s: i64,
) -> Result<Vec<Prices>, Error> {
    let mf = market_filter(filters);
    commodities
        .iter()
        .map(|c| {
            let symbol = c.trim().to_lowercase();
            let got = client.ardent_nearby(&origin.system, &symbol, side.trade(), &mf)?;
            Ok(Prices {
                commodity: symbol,
                side,
                fetched_at: got.fetched_at,
                truncated: got.value.len() >= NEARBY_LIMIT,
                offers: rank(side, &got.value, origin, range, filters, now_s),
            })
        })
        .collect()
}

/// A search for the app: [`prices`] from a located system, keeping the best `limit` markets
/// per commodity, with jumps counted on the star map at `range`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Search {
    pub side: Side,
    pub here: Here,
    pub range: f64,
    pub results: Vec<Prices>,
}

pub fn search(
    galaxy: &Galaxy,
    here: &Here,
    range: f64,
    side: Side,
    commodities: &[&str],
    filters: &Filters,
    limit: usize,
    now_s: i64,
) -> Result<Search, Error> {
    let mut results = prices(galaxy.remote(), &Origin::from(here), side, commodities, filters, range, now_s)?;
    // Fill-in is best effort: without it the jump counts stay estimates.
    let _ = galaxy.ensure_covered(here.pos);
    for p in &mut results {
        p.offers.truncate(limit.max(1));
        count_jumps(galaxy, here, range, &mut p.offers);
    }
    Ok(Search { side, here: here.clone(), range, results })
}

/// Jumps from `here` to each offer's system on the star map (estimates if the map won't open).
pub fn count_jumps(galaxy: &Galaxy, here: &Here, range: f64, offers: &mut [Offer]) {
    if let Ok(map) = galaxy.map() {
        for o in offers {
            o.hop = map.hop(here.pos, o.system_pos, range);
        }
    }
}
