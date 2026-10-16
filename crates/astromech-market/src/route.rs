//! Trade routes from a station: the most profitable single hop (buy here, sell there) within a
//! hop distance, or a loop (that hop and the best cargo back to the start), for the ship's
//! hold, the commander's credits and the lookup's filters.
//!
//! The outbound hop is Spansh's trade router (`/api/trade/route`, one hop, a job polled until
//! done, see `astromech_data::spansh`). Spansh has no loop option, so a loop's return leg is
//! worked out here from both stations' markets on Ardent Insight: the commodity with the most
//! profit for the whole leg, within the hold, the credits after the outbound sale, the stock
//! there and the demand at the start.
//!
//! Spansh gives no profit per hour, so [`Route::per_hour`] is an estimate from a simple model
//! (see [`leg_seconds`]); the app says so.
//!
//! Spansh can only require a large pad, so a medium-pad ship may be routed to a small-pad-only
//! station; Spansh's planetary switch is always on (planetary ports are ordinary stations).

use crate::trade::{with_defaults, PLACEHOLDER_VOLUME};
use astromech_data::ardent::Listing;
pub use astromech_data::spansh::Polling;
use astromech_data::spansh::{TradeHop, TradeRoute, TradeStop};
use astromech_data::{Client, Error, Service};
use astromech_galaxy::{estimate, Filters, Pad};
use serde::{Deserialize, Serialize};

/// Arrival distance Spansh may use when the commander hasn't limited it (its own default).
const ANY_ARRIVAL_LS: u32 = 10_000_000;

/// Station types that can't start a route: construction sites, carriers and settlements.
const NOT_A_START: &[&str] =
    &["SpaceConstructionDepot", "PlanetaryConstructionDepot", "FleetCarrier", "OnFootSettlement"];

/// The time model behind [`Route::per_hour`] (an estimate; Spansh gives none). A jump with its
/// charge, cooldown and fuel scooping takes about 50 s; docking, trading and leaving about
/// 2 min; supercruise to the station about 40 s plus 0.03 s per light second from the arrival
/// star (about 1 min at 500 ls, 3 min at 5,000 ls).
pub const JUMP_S: f64 = 50.0;
pub const DOCK_S: f64 = 120.0;
pub const SUPERCRUISE_S: f64 = 40.0;
pub const SUPERCRUISE_S_PER_LS: f64 = 0.03;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Buy here, sell at the best station within reach.
    Single,
    /// The single hop, then the best cargo back to the start.
    Loop,
}

/// What the commander asks for. The app fills the numbers from the current ship and the
/// journal ([`Defaults`]); all can be changed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Ask {
    pub kind: Kind,
    /// The station to start from; `None`: the station in the origin system nearest its star
    /// that fits the pad ([`pick_station`]).
    pub station: Option<String>,
    /// Tonnes of cargo space.
    pub capacity: u32,
    pub credits: i64,
    /// Light years between the systems of the two stations.
    pub max_hop_ly: f64,
}

/// The current ship's and commander's numbers for an [`Ask`].
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Defaults {
    /// Cargo capacity from the latest `Loadout`.
    pub capacity: Option<i64>,
    /// `Status.json`'s balance, else the latest `LoadGame`.
    pub credits: Option<i64>,
    /// The ship's maximum jump range, the default hop distance.
    pub range: Option<f64>,
    /// The station docked at, the default start.
    pub station: Option<String>,
    /// The system docked in (a typed origin elsewhere doesn't start there).
    pub system: Option<String>,
}

/// A station on a route.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Stop {
    pub station: String,
    pub system: String,
    pub market_id: i64,
    /// Light seconds from the arrival star.
    pub arrival_ls: Option<f64>,
}

/// One commodity carried on a leg.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Cargo {
    pub name: String,
    pub tonnes: i64,
    /// Credits per tonne, paid at the start of the leg.
    pub buy: i64,
    /// Credits per tonne, paid at its end.
    pub sell: i64,
    pub profit_per_t: i64,
    pub profit: i64,
}

/// One leg: buy at `from`, sell at `to`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Leg {
    pub from: Stop,
    pub to: Stop,
    pub ly: f64,
    /// Straight-line estimate at the ship's range.
    pub jumps: u32,
    /// Empty: nothing worth carrying (a loop's way back).
    pub cargo: Vec<Cargo>,
    pub profit: i64,
    /// Profit per tonne carried; 0 when flying empty.
    pub profit_per_t: i64,
    /// Who worked the leg out: "Spansh" or "Ardent Insight".
    pub source: &'static str,
    /// Estimated time for the leg (jumps, supercruise, docking), in seconds.
    pub seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Route {
    pub kind: Kind,
    /// No legs: nothing profitable within the limits.
    pub legs: Vec<Leg>,
    pub profit: i64,
    /// One run of the route, estimated.
    pub minutes: f64,
    /// Estimated profit per hour of repeating the route.
    pub per_hour: i64,
    /// The jump range the estimate used.
    pub range: f64,
    /// When the market data was asked for (unix seconds).
    pub fetched_at: i64,
}

impl Stop {
    fn from_spansh(s: &TradeStop) -> Self {
        Stop {
            station: s.station.clone(),
            system: s.system.clone(),
            market_id: s.market_id,
            arrival_ls: s.distance_to_arrival,
        }
    }
}

/// Seconds for a leg of `ly` at `range`: the jumps, supercruise to the station and docking.
pub fn leg_seconds(ly: f64, range: f64, arrival_ls: Option<f64>) -> f64 {
    f64::from(estimate(ly, range)) * JUMP_S + SUPERCRUISE_S + arrival_ls.unwrap_or(0.0) * SUPERCRUISE_S_PER_LS + DOCK_S
}

fn leg(from: Stop, to: Stop, ly: f64, cargo: Vec<Cargo>, range: f64, source: &'static str) -> Leg {
    let profit = cargo.iter().map(|c| c.profit).sum();
    let tonnes: i64 = cargo.iter().map(|c| c.tonnes).sum();
    let seconds = leg_seconds(ly, range, to.arrival_ls);
    Leg {
        jumps: estimate(ly, range),
        profit_per_t: if tonnes > 0 { profit / tonnes } else { 0 },
        from,
        to,
        ly,
        cargo,
        profit,
        source,
        seconds,
    }
}

fn spansh_leg(h: &TradeHop, range: f64) -> Leg {
    let cargo = h
        .commodities
        .iter()
        .map(|c| Cargo {
            name: c.name.clone(),
            tonnes: c.amount,
            buy: c.source_commodity.buy_price,
            sell: c.destination_commodity.sell_price,
            profit_per_t: c.profit,
            profit: c.total_profit,
        })
        .collect();
    leg(Stop::from_spansh(&h.source), Stop::from_spansh(&h.destination), h.distance, cargo, range, "Spansh")
}

fn pad_num(n: Option<u8>) -> Option<Pad> {
    match n? {
        1 => Some(Pad::S),
        2 => Some(Pad::M),
        3 => Some(Pad::L),
        _ => None,
    }
}

fn fresh(updated_at: &str, max_age_days: Option<u32>, now_s: i64) -> bool {
    let Some(days) = max_age_days else { return true };
    chrono::DateTime::parse_from_rfc3339(updated_at).is_ok_and(|t| now_s - t.timestamp() <= i64::from(days) * 86_400)
}

/// The station in `system` to start from when none is given: one with a market that fits the
/// pad and arrival filters, nearest the star (no carriers, settlements or construction sites).
pub fn pick_station(client: &Client, system: &str, filters: &Filters) -> Result<String, Error> {
    let markets = client.ardent_markets(system)?.value;
    markets
        .iter()
        .filter(|l| !l.station_type.as_deref().is_some_and(|t| NOT_A_START.contains(&t)))
        .filter(|l| match (filters.pad, pad_num(l.max_landing_pad_size)) {
            (Some(need), Some(has)) => has >= need,
            (Some(_), None) => false,
            (None, _) => true,
        })
        .filter(|l| filters.max_arrival_ls.is_none_or(|max| l.distance_to_arrival.is_none_or(|a| a <= f64::from(max))))
        .min_by(|a, b| a.distance_to_arrival.unwrap_or(f64::MAX).total_cmp(&b.distance_to_arrival.unwrap_or(f64::MAX)))
        .map(|l| l.station_name.clone())
        .ok_or_else(|| Error::NotFound {
            service: Service::Ardent,
            message: format!("no station with a market in {system} fits the filters; type the station to start from"),
        })
}

/// The cargo with the most profit from `from` to `to`, within `capacity`, `credits`, the stock
/// at `from` and the demand at `to`, using market data no older than `max_age_days`. Empty when
/// nothing pays.
pub fn best_cargo(
    from: &[Listing],
    to: &[Listing],
    capacity: i64,
    credits: i64,
    max_age_days: Option<u32>,
    now_s: i64,
) -> Vec<Cargo> {
    let usable = |l: &&Listing| fresh(&l.updated_at, max_age_days, now_s);
    from.iter()
        .filter(usable)
        .filter(|b| b.buy_price > 0 && b.stock > 0 && b.stock != PLACEHOLDER_VOLUME)
        .filter_map(|b| {
            let s = to.iter().filter(usable).find(|s| {
                s.commodity_name == b.commodity_name
                    && s.sell_price > b.buy_price
                    && s.demand > 0
                    && s.demand != PLACEHOLDER_VOLUME
            })?;
            let tonnes = capacity.min(credits / b.buy_price).min(b.stock).min(s.demand);
            let per_t = s.sell_price - b.buy_price;
            (tonnes > 0).then(|| Cargo {
                name: crate::catalog::name_of(&b.commodity_name),
                tonnes,
                buy: b.buy_price,
                sell: s.sell_price,
                profit_per_t: per_t,
                profit: per_t * tonnes,
            })
        })
        .max_by_key(|c| c.profit)
        .into_iter()
        .collect()
}

/// Spansh's request for `ask` from `station` in `system`.
pub fn request(system: &str, station: &str, ask: &Ask, filters: &Filters) -> TradeRoute {
    let f = with_defaults(filters);
    TradeRoute {
        system: system.to_string(),
        station: station.to_string(),
        max_hops: 1,
        max_hop_distance: ask.max_hop_ly,
        starting_capital: ask.credits,
        max_cargo: ask.capacity,
        max_system_distance: f.max_arrival_ls.unwrap_or(ANY_ARRIVAL_LS),
        max_price_age: f.max_age_days.map(|d| u64::from(d) * 86_400),
        requires_large_pad: f.pad == Some(Pad::L),
        allow_planetary: true,
        allow_player_owned: !f.exclude_carriers,
    }
}

/// The route for `ask` from `system`. Blocks while Spansh works (up to `polling.timeout`).
pub fn plan(
    client: &Client,
    system: &str,
    range: f64,
    ask: &Ask,
    filters: &Filters,
    polling: Polling,
    now_s: i64,
) -> Result<Route, Error> {
    let station = match ask.station.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(s) => s.to_string(),
        None => pick_station(client, system, filters)?,
    };
    let hops = client.spansh_trade_route_wait(&request(system, &station, ask, filters), polling)?;
    let mut legs: Vec<Leg> = hops.iter().take(1).map(|h| spansh_leg(h, range)).collect();
    if ask.kind == Kind::Loop {
        if let Some(out) = legs.first() {
            let back = return_leg(client, out, range, ask, filters, now_s)?;
            legs.push(back);
        }
    }
    let profit = legs.iter().map(|l| l.profit).sum();
    let seconds: f64 = legs.iter().map(|l| l.seconds).sum();
    Ok(Route {
        kind: ask.kind,
        per_hour: if seconds > 0.0 { (profit as f64 * 3600.0 / seconds).round() as i64 } else { 0 },
        minutes: seconds / 60.0,
        legs,
        profit,
        range,
        fetched_at: now_s,
    })
}

/// The way back from `out`'s destination to its start, with the best cargo for it.
fn return_leg(client: &Client, out: &Leg, range: f64, ask: &Ask, filters: &Filters, now_s: i64) -> Result<Leg, Error> {
    let there = client.ardent_markets(&out.to.system)?.value;
    let home = client.ardent_markets(&out.from.system)?.value;
    let at = |ls: &[Listing], id: i64| ls.iter().filter(|l| l.market_id == id).cloned().collect::<Vec<_>>();
    let cargo = best_cargo(
        &at(&there, out.to.market_id),
        &at(&home, out.from.market_id),
        i64::from(ask.capacity),
        ask.credits + out.profit,
        with_defaults(filters).max_age_days,
        now_s,
    );
    Ok(leg(out.to.clone(), out.from.clone(), out.ly, cargo, range, "Ardent Insight"))
}
