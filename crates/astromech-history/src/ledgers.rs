//! Ledgers derived from journal events. [`derive`] runs once per new event, in import order,
//! and reads only what earlier events wrote, so an import resumed after a crash produces the
//! same rows as one clean run.
//!
//! - `visits`: arrivals in a system (`FSDJump`, `CarrierJump`, and `Location` when it isn't the
//!   system already recorded last, so logging in again doesn't count) and dockings (`Docked`,
//!   and `Location` when starting docked somewhere other than the last docking).
//! - `earnings`: credits received, one row per event: `trade` (`MarketSell`, with `cost` from
//!   the average price paid), `exploration`, `organic`, `bounty`, `combat_bond`, `voucher`
//!   (other `RedeemVoucher` types), `mission`, `search_and_rescue`, `community_goal`,
//!   `micro_resources`. Bounties count when redeemed, not when the `Bounty` event happens.
//! - `ships`: every ship id seen (`Loadout`, `StoredShips`, `ShipyardNew`), with its latest type,
//!   name and ident, when it was bought (`acquired`), last flown and sold. Assumes one commander.
//! - `scans`: `body` (`Scan`), `mapped` (`SAAScanComplete`) and `organic` (`ScanOrganic` with
//!   `ScanType: Analyse`, a completed sample), one row per event.

use astromech_journal::{ExtraEvent as X, JournalEvent as J, JournalLine};
use astromech_store::rusqlite::{params, Connection, OptionalExtension, Result};
use serde_json::Value;

/// A row of `visits`. `kind` is `system` or `station`.
#[derive(Debug, Clone, PartialEq)]
pub struct Visit {
    pub ts: String,
    pub kind: String,
    pub star_system: String,
    pub system_address: Option<i64>,
    pub station_name: Option<String>,
    pub market_id: Option<i64>,
    pub jump_dist: Option<f64>,
}

/// A row of `earnings`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Earning {
    pub ts: String,
    pub kind: String,
    pub amount: i64,
    /// What the goods cost, for `trade`, when the game reported the average price paid.
    pub cost: Option<i64>,
    /// Commodity, mission name or factions.
    pub detail: Option<String>,
}

/// A row of `ships`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ship {
    pub ship_id: i64,
    /// Lowercase journal symbol (`krait_mkii`).
    pub ship_type: String,
    pub name: Option<String>,
    pub ident: Option<String>,
    pub first_seen: String,
    pub last_flown: Option<String>,
    pub acquired: Option<String>,
    pub sold: Option<String>,
}

/// A row of `scans`. `kind` is `body`, `mapped` or `organic`; `class` is the star type, planet
/// class or species.
#[derive(Debug, Clone, PartialEq)]
pub struct Scan {
    pub ts: String,
    pub kind: String,
    pub system_address: Option<i64>,
    pub star_system: Option<String>,
    pub body_name: Option<String>,
    pub body_id: Option<i64>,
    pub class: Option<String>,
    pub landable: Option<bool>,
    pub terraformable: Option<bool>,
    pub was_discovered: Option<bool>,
    pub was_mapped: Option<bool>,
}

/// Write the ledger rows for one newly stored event.
pub(crate) fn derive(c: &Connection, event_id: i64, line: &JournalLine) -> Result<()> {
    let ts = line.timestamp.as_str();
    match &line.event {
        J::FsdJump { star_system, jump_dist, system_address, .. } => {
            add_visit(c, event_id, ts, "system", star_system, *system_address, None, None, Some(*jump_dist))?;
        }
        J::Location { star_system, system_address, docked, station_name, market_id, .. } => {
            let last = c
                .prepare_cached(
                    "SELECT star_system, system_address FROM visits WHERE kind = 'system' ORDER BY id DESC LIMIT 1",
                )?
                .query_row([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<i64>>(1)?)))
                .optional()?;
            let same = last.is_some_and(|(name, addr)| match (addr, system_address) {
                (Some(a), Some(b)) => a == *b,
                _ => name == *star_system,
            });
            if !same {
                add_visit(c, event_id, ts, "system", star_system, *system_address, None, None, None)?;
            }
            if let (true, Some(station)) = (*docked, station_name) {
                let last = c
                    .prepare_cached("SELECT kind, station_name, market_id FROM visits ORDER BY id DESC LIMIT 1")?
                    .query_row([], |r| {
                        Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?, r.get::<_, Option<i64>>(2)?))
                    })
                    .optional()?;
                let same = last.is_some_and(|(kind, name, market)| {
                    kind == "station"
                        && match (market, market_id) {
                            (Some(a), Some(b)) => a == *b,
                            _ => name.as_deref() == Some(station.as_str()),
                        }
                });
                if !same {
                    add_visit(
                        c,
                        event_id,
                        ts,
                        "station",
                        star_system,
                        *system_address,
                        Some(station),
                        *market_id,
                        None,
                    )?;
                }
            }
        }
        J::Docked { station_name, star_system, market_id } => {
            let addr = system_address_of(c, star_system)?;
            add_visit(c, event_id, ts, "station", star_system, addr, Some(station_name), *market_id, None)?;
        }
        J::Loadout { ship_id: Some(id), ship: Some(ship), ship_name, ship_ident, .. } => {
            c.prepare_cached(
                "INSERT INTO ships(ship_id, ship_type, name, ident, first_seen, last_flown)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?5)
                 ON CONFLICT(ship_id) DO UPDATE SET ship_type = excluded.ship_type,
                    name = coalesce(excluded.name, name), ident = coalesce(excluded.ident, ident),
                    last_flown = excluded.last_flown",
            )?
            .execute(params![id, ship.to_lowercase(), ship_name, ship_ident, ts])?;
        }
        J::StoredShips { ships } => {
            for s in ships {
                seen_ship(c, s.ship_id, &s.ship_type, s.name.as_deref(), ts)?;
            }
        }
        J::ShipSold { ship_id } => {
            c.prepare_cached("UPDATE ships SET sold = ?2 WHERE ship_id = ?1")?.execute(params![ship_id, ts])?;
        }
        J::Extra(x) => extra(c, event_id, ts, x)?,
        J::Other { event, raw } => other(c, event_id, ts, event, raw)?,
        _ => {}
    }
    Ok(())
}

fn extra(c: &Connection, event_id: i64, ts: &str, x: &X) -> Result<()> {
    match x {
        X::MarketTrade(t) if t.sell => {
            let cost = t.avg_price_paid.map(|p| p * t.count);
            earn(c, event_id, ts, "trade", t.total, cost, Some(&t.commodity))?;
        }
        X::ExplorationSale { total_earnings, .. } => earn(c, event_id, ts, "exploration", *total_earnings, None, None)?,
        X::SellOrganicData { bio_data, .. } => {
            let total = bio_data.iter().map(|b| b.value + b.bonus).sum();
            earn(c, event_id, ts, "organic", total, None, None)?;
        }
        X::RedeemVoucher { kind, amount, factions } => {
            let k = match kind.as_str() {
                "bounty" => "bounty",
                "CombatBond" => "combat_bond",
                _ => "voucher",
            };
            let detail = if k == "voucher" { Some(kind.clone()) } else { Some(factions.join(", ")) };
            earn(c, event_id, ts, k, *amount, None, detail.filter(|d| !d.is_empty()).as_deref())?;
        }
        X::MissionCompleted { name, reward: Some(r), .. } if *r > 0 => {
            earn(c, event_id, ts, "mission", *r, None, Some(name))?;
        }
        X::Scan(b) => {
            let class = b.star_type.clone().or_else(|| b.planet_class.clone());
            let star_system = match &b.star_system {
                Some(s) => Some(s.clone()),
                None => system_name_of(c, b.system_address)?,
            };
            let is_star = b.star_type.is_some();
            c.prepare_cached(
                "INSERT INTO scans(event_id, ts, kind, system_address, star_system, body_name, body_id, class,
                    landable, terraformable, was_discovered, was_mapped)
                 VALUES (?1, ?2, 'body', ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            )?
            .execute(params![
                event_id,
                ts,
                b.system_address,
                star_system,
                b.body_name,
                b.body_id,
                class,
                (!is_star).then_some(b.landable),
                (!is_star).then(|| b.terraform_state.as_deref().is_some_and(|t| !t.is_empty())),
                b.was_discovered,
                b.was_mapped,
            ])?;
        }
        X::SaaScanComplete { body_name, body_id, system_address, .. } => {
            let star_system = system_name_of(c, *system_address)?;
            c.prepare_cached(
                "INSERT INTO scans(event_id, ts, kind, system_address, star_system, body_name, body_id)
                 VALUES (?1, ?2, 'mapped', ?3, ?4, ?5, ?6)",
            )?
            .execute(params![event_id, ts, system_address, star_system, body_name, body_id])?;
        }
        X::ScanOrganic { scan_type, species, species_localised, body, system_address, .. }
            if scan_type == "Analyse" =>
        {
            let star_system = system_name_of(c, *system_address)?;
            let body_name: Option<String> = c
                .prepare_cached(
                    "SELECT body_name FROM scans WHERE system_address = ?1 AND body_id = ?2 AND body_name IS NOT NULL
                     ORDER BY event_id DESC LIMIT 1",
                )?
                .query_row(params![system_address, body], |r| r.get(0))
                .optional()?;
            let class = species_localised.clone().unwrap_or_else(|| species.clone());
            c.prepare_cached(
                "INSERT INTO scans(event_id, ts, kind, system_address, star_system, body_name, body_id, class)
                 VALUES (?1, ?2, 'organic', ?3, ?4, ?5, ?6, ?7)",
            )?
            .execute(params![event_id, ts, system_address, star_system, body_name, body, class])?;
        }
        _ => {}
    }
    Ok(())
}

fn other(c: &Connection, event_id: i64, ts: &str, event: &str, raw: &Value) -> Result<()> {
    let int = |k: &str| raw.get(k).and_then(Value::as_i64);
    let text = |k: &str| raw.get(k).and_then(Value::as_str).filter(|s| !s.is_empty());
    match event {
        "SearchAndRescue" => {
            if let Some(r) = int("Reward") {
                earn(c, event_id, ts, "search_and_rescue", r, None, text("Name_Localised").or(text("Name")))?;
            }
        }
        "CommunityGoalReward" => {
            if let Some(r) = int("Reward") {
                earn(c, event_id, ts, "community_goal", r, None, text("Name"))?;
            }
        }
        "SellMicroResources" => {
            if let Some(p) = int("Price") {
                earn(c, event_id, ts, "micro_resources", p, None, None)?;
            }
        }
        "ShipyardNew" => {
            if let (Some(id), Some(t)) = (int("NewShipID"), text("ShipType")) {
                seen_ship(c, id, t, None, ts)?;
                c.prepare_cached("UPDATE ships SET acquired = ?2 WHERE ship_id = ?1")?.execute(params![id, ts])?;
            }
        }
        "SetUserShipName" => {
            if let Some(id) = int("ShipID") {
                c.prepare_cached(
                    "UPDATE ships SET name = coalesce(?2, name), ident = coalesce(?3, ident) WHERE ship_id = ?1",
                )?
                .execute(params![id, text("UserShipName"), text("UserShipId")])?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn add_visit(
    c: &Connection,
    event_id: i64,
    ts: &str,
    kind: &str,
    star_system: &str,
    system_address: Option<i64>,
    station_name: Option<&String>,
    market_id: Option<i64>,
    jump_dist: Option<f64>,
) -> Result<()> {
    c.prepare_cached(
        "INSERT INTO visits(event_id, ts, kind, star_system, system_address, station_name, market_id, jump_dist)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )?
    .execute(params![event_id, ts, kind, star_system, system_address, station_name, market_id, jump_dist])?;
    Ok(())
}

fn earn(
    c: &Connection,
    event_id: i64,
    ts: &str,
    kind: &str,
    amount: i64,
    cost: Option<i64>,
    detail: Option<&str>,
) -> Result<()> {
    c.prepare_cached("INSERT INTO earnings(event_id, ts, kind, amount, cost, detail) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?
        .execute(params![event_id, ts, kind, amount, cost, detail])?;
    Ok(())
}

fn seen_ship(c: &Connection, ship_id: i64, ship_type: &str, name: Option<&str>, ts: &str) -> Result<()> {
    c.prepare_cached(
        "INSERT INTO ships(ship_id, ship_type, name, first_seen) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(ship_id) DO UPDATE SET ship_type = excluded.ship_type, name = coalesce(excluded.name, name)",
    )?
    .execute(params![ship_id, ship_type.to_lowercase(), name, ts])?;
    Ok(())
}

/// Address of the last arrival in the system called `name` (`Docked` carries only the name).
fn system_address_of(c: &Connection, name: &str) -> Result<Option<i64>> {
    Ok(c.prepare_cached(
        "SELECT system_address FROM visits WHERE kind = 'system' AND star_system = ?1 ORDER BY id DESC LIMIT 1",
    )?
    .query_row([name], |r| r.get(0))
    .optional()?
    .flatten())
}

/// Name of the system with this address, from the arrivals recorded so far.
fn system_name_of(c: &Connection, address: Option<i64>) -> Result<Option<String>> {
    let Some(a) = address else { return Ok(None) };
    c.prepare_cached(
        "SELECT star_system FROM visits WHERE kind = 'system' AND system_address = ?1 ORDER BY id DESC LIMIT 1",
    )?
    .query_row([a], |r| r.get(0))
    .optional()
}

impl crate::History {
    pub fn visits(&self) -> crate::Result<Vec<Visit>> {
        let mut st = self.conn.prepare(
            "SELECT ts, kind, star_system, system_address, station_name, market_id, jump_dist FROM visits ORDER BY id",
        )?;
        let rows = st.query_map([], |r| {
            Ok(Visit {
                ts: r.get(0)?,
                kind: r.get(1)?,
                star_system: r.get(2)?,
                system_address: r.get(3)?,
                station_name: r.get(4)?,
                market_id: r.get(5)?,
                jump_dist: r.get(6)?,
            })
        })?;
        Ok(rows.collect::<Result<_>>()?)
    }

    pub fn earnings(&self) -> crate::Result<Vec<Earning>> {
        let mut st = self.conn.prepare("SELECT ts, kind, amount, cost, detail FROM earnings ORDER BY event_id")?;
        let rows = st.query_map([], |r| {
            Ok(Earning { ts: r.get(0)?, kind: r.get(1)?, amount: r.get(2)?, cost: r.get(3)?, detail: r.get(4)? })
        })?;
        Ok(rows.collect::<Result<_>>()?)
    }

    pub fn ships(&self) -> crate::Result<Vec<Ship>> {
        let mut st = self.conn.prepare(
            "SELECT ship_id, ship_type, name, ident, first_seen, last_flown, acquired, sold FROM ships ORDER BY ship_id",
        )?;
        let rows = st.query_map([], |r| {
            Ok(Ship {
                ship_id: r.get(0)?,
                ship_type: r.get(1)?,
                name: r.get(2)?,
                ident: r.get(3)?,
                first_seen: r.get(4)?,
                last_flown: r.get(5)?,
                acquired: r.get(6)?,
                sold: r.get(7)?,
            })
        })?;
        Ok(rows.collect::<Result<_>>()?)
    }

    pub fn scans(&self) -> crate::Result<Vec<Scan>> {
        let mut st = self.conn.prepare(
            "SELECT ts, kind, system_address, star_system, body_name, body_id, class, landable, terraformable,
                    was_discovered, was_mapped FROM scans ORDER BY event_id",
        )?;
        let rows = st.query_map([], |r| {
            Ok(Scan {
                ts: r.get(0)?,
                kind: r.get(1)?,
                system_address: r.get(2)?,
                star_system: r.get(3)?,
                body_name: r.get(4)?,
                body_id: r.get(5)?,
                class: r.get(6)?,
                landable: r.get(7)?,
                terraformable: r.get(8)?,
                was_discovered: r.get(9)?,
                was_mapped: r.get(10)?,
            })
        })?;
        Ok(rows.collect::<Result<_>>()?)
    }
}
