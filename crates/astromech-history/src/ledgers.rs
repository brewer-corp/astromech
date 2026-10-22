//! Ledgers derived from journal events. [`derive`] runs once per new event and reads only
//! what earlier events (by game time) wrote, so an import resumed after a crash produces the
//! same rows as one clean run. Events arriving older than what a commander's ledgers already
//! hold (an old journal imported late) make the import rebuild that commander's ledgers in
//! game-time order; ship rows also only take values from newer events than the ones they have.
//! Every row belongs to one commander (`cmdr`, see `commander`), and lookups of earlier rows
//! stay within that commander.
//!
//! - `commanders`: every commander key seen, with the latest name and whether it is Legacy.
//! - `visits`: arrivals in a system (`FSDJump` with its jump distance, `CarrierJump` without
//!   one, and `Location` when it isn't the system already recorded last, so logging in again
//!   doesn't count) and dockings (`Docked`, and `Location` when starting docked somewhere other
//!   than the last docking). `via` names the event.
//! - `earnings`: credits paid to the commander for something they did, one row per event:
//!   - `trade`: `MarketSell` `TotalSale`, with `cost` = `AvgPricePaid` × count when the game
//!     reports it (profit is `amount - cost`; mined or looted goods have no cost);
//!   - `exploration` (`SellExplorationData`, `MultiSellExplorationData` `TotalEarnings`),
//!     `organic` (`SellOrganicData` value + bonus);
//!   - `bounty`, `combat_bond`, `voucher` (other types, e.g. codex): `RedeemVoucher` `Amount`.
//!     Bounties count when redeemed; the `Bounty` award itself isn't paid yet;
//!   - `mission` (`MissionCompleted` `Reward`), `search_and_rescue`, `community_goal`
//!     (`CommunityGoalReward`), `micro_resources` (`SellMicroResources`), `powerplay`
//!     (`PowerplaySalary`).
//!
//!   Not earnings: selling ships or modules (`ShipyardSell`, `SellShipOnRebuy`, `ModuleSell`),
//!   drones (`SellDrones`) and carrier transfers (`CarrierBankTransfer`, trade orders) turn
//!   assets into credits; costs (purchases, fines, crew wages, rebuys) aren't recorded.
//! - `ships`: one row per ship owned (`Loadout`, `StoredShips`, `ShipyardNew`, `ShipRedeemed`),
//!   with its type, latest name and ident, when it was bought (`acquired`), last flown and sold
//!   (`ShipyardSell`, `SellShipOnRebuy`, a trade-in). The game gives a ship bought with a
//!   trade-in the id of the ship traded in, so one id can have several rows, at most one unsold.
//!   A ship id seen on another type closes the unsold row as sold by then (the journal with the
//!   sale is gone).
//! - `scans`: `body` (`Scan`), `mapped` (`SAAScanComplete`) and `organic` (`ScanOrganic` with
//!   `ScanType: Analyse`, a completed sample), one row per event.

use astromech_journal::{ExtraEvent as X, JournalEvent as J, JournalLine};
use astromech_store::rusqlite::{params, Connection, OptionalExtension, Result};
use serde::Serialize;
use serde_json::Value;

/// A row of `commanders`. `cmdr` is the key every ledger uses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Commander {
    pub cmdr: String,
    pub name: String,
    pub legacy: bool,
    pub first_seen: String,
    pub last_seen: String,
}

/// A row of `visits`. `kind` is `system` or `station`. Count jumps with `via == "FSDJump"`:
/// carrier jumps and logins are arrivals too.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Visit {
    pub ts: String,
    pub kind: String,
    pub star_system: String,
    pub system_address: Option<i64>,
    pub station_name: Option<String>,
    pub market_id: Option<i64>,
    /// Hyperspace jump distance (`FSDJump` only).
    pub jump_dist: Option<f64>,
    /// The event: `FSDJump`, `CarrierJump`, `Location` or `Docked`.
    pub via: String,
}

/// A row of `earnings`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Earning {
    pub ts: String,
    pub kind: String,
    pub amount: i64,
    /// What the goods cost, for `trade`, when the game reported the average price paid.
    pub cost: Option<i64>,
    /// Commodity, mission name or factions.
    pub detail: Option<String>,
}

/// A row of `ships`: one ship owned. A ship bought with a trade-in reuses the old ship's id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize)]
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

/// The event being derived, and where its rows go.
struct Ev<'a> {
    c: &'a Connection,
    id: i64,
    cmdr: &'a str,
    ts: &'a str,
    /// The journal event name.
    via: &'a str,
}

/// Write the ledger rows for one newly stored event of commander `cmdr`.
pub(crate) fn derive(c: &Connection, event_id: i64, cmdr: &str, line: &JournalLine) -> Result<()> {
    let e = Ev { c, id: event_id, cmdr, ts: line.timestamp.as_str(), via: line.name.as_str() };
    match &line.event {
        J::LoadGame { commander, .. } => e.commander(commander)?,
        J::FsdJump { star_system, jump_dist, system_address, from_carrier, .. } => {
            let dist = (!from_carrier).then_some(*jump_dist);
            e.visit("system", star_system, *system_address, None, None, dist)?;
        }
        J::Location { star_system, system_address, docked, station_name, market_id, .. } => {
            let last = c
                .prepare_cached(
                    "SELECT star_system, system_address FROM visits WHERE cmdr = ?1 AND kind = 'system' AND ts <= ?2
                     ORDER BY ts DESC, id DESC LIMIT 1",
                )?
                .query_row([cmdr, e.ts], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<i64>>(1)?)))
                .optional()?;
            let same = last.is_some_and(|(name, addr)| match (addr, system_address) {
                (Some(a), Some(b)) => a == *b,
                _ => name == *star_system,
            });
            if !same {
                e.visit("system", star_system, *system_address, None, None, None)?;
            }
            if let (true, Some(station)) = (*docked, station_name) {
                let last = c
                    .prepare_cached(
                        "SELECT kind, station_name, market_id FROM visits WHERE cmdr = ?1 AND ts <= ?2
                         ORDER BY ts DESC, id DESC LIMIT 1",
                    )?
                    .query_row([cmdr, e.ts], |r| {
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
                    e.visit("station", star_system, *system_address, Some(station), *market_id, None)?;
                }
            }
        }
        J::Docked { station_name, star_system, market_id, system_address, .. } => {
            e.visit("station", star_system, *system_address, Some(station_name), *market_id, None)?;
        }
        J::Loadout { ship_id: Some(id), ship: Some(ship), ship_name, ship_ident, .. } => {
            e.owned_ship(*id, ship, ship_name.as_deref(), ship_ident.as_deref(), true)?;
        }
        J::StoredShips { ships } => {
            for s in ships {
                e.owned_ship(s.ship_id, &s.ship_type, s.name.as_deref(), None, false)?;
            }
        }
        J::ShipSold { ship_id } => {
            c.prepare_cached("UPDATE ships SET sold = ?3 WHERE cmdr = ?1 AND ship_id = ?2 AND sold IS NULL")?
                .execute(params![cmdr, ship_id, e.ts])?;
        }
        J::Extra(x) => e.extra(x)?,
        J::Other { event, raw } => e.other(event, raw)?,
        _ => {}
    }
    Ok(())
}

impl Ev<'_> {
    fn extra(&self, x: &X) -> Result<()> {
        let c = self.c;
        match x {
            X::MarketTrade(t) if t.sell => {
                let cost = t.avg_price_paid.map(|p| p * t.count);
                self.earn("trade", t.total, cost, Some(&t.commodity))?;
            }
            X::ExplorationSale { total_earnings, .. } => self.earn("exploration", *total_earnings, None, None)?,
            X::SellOrganicData { bio_data, .. } => {
                let total = bio_data.iter().map(|b| b.value + b.bonus).sum();
                self.earn("organic", total, None, None)?;
            }
            X::RedeemVoucher { kind, amount, factions } => {
                let k = match kind.as_str() {
                    "bounty" => "bounty",
                    "CombatBond" => "combat_bond",
                    _ => "voucher",
                };
                let detail = if k == "voucher" { Some(kind.clone()) } else { Some(factions.join(", ")) };
                self.earn(k, *amount, None, detail.filter(|d| !d.is_empty()).as_deref())?;
            }
            X::Powerplay(p) if p.kind == "PowerplaySalary" => {
                if let Some(a) = p.amount {
                    self.earn("powerplay", a, None, p.power.as_deref())?;
                }
            }
            X::MissionCompleted { name, reward: Some(r), .. } if *r > 0 => {
                self.earn("mission", *r, None, Some(name))?;
            }
            X::Scan(b) => {
                let class = b.star_type.clone().or_else(|| b.planet_class.clone());
                let star_system = match &b.star_system {
                    Some(s) => Some(s.clone()),
                    None => self.system_name_of(b.system_address)?,
                };
                let is_star = b.star_type.is_some();
                c.prepare_cached(
                    "INSERT INTO scans(event_id, cmdr, ts, kind, system_address, star_system, body_name, body_id,
                        class, landable, terraformable, was_discovered, was_mapped)
                     VALUES (?1, ?2, ?3, 'body', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                )?
                .execute(params![
                    self.id,
                    self.cmdr,
                    self.ts,
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
                let star_system = self.system_name_of(*system_address)?;
                c.prepare_cached(
                    "INSERT INTO scans(event_id, cmdr, ts, kind, system_address, star_system, body_name, body_id)
                     VALUES (?1, ?2, ?3, 'mapped', ?4, ?5, ?6, ?7)",
                )?
                .execute(params![
                    self.id,
                    self.cmdr,
                    self.ts,
                    system_address,
                    star_system,
                    body_name,
                    body_id
                ])?;
            }
            X::ScanOrganic { scan_type, species, species_localised, body, system_address, .. }
                if scan_type == "Analyse" =>
            {
                let star_system = self.system_name_of(*system_address)?;
                let body_name: Option<String> = c
                    .prepare_cached(
                        "SELECT body_name FROM scans
                         WHERE system_address = ?1 AND body_id = ?2 AND body_name IS NOT NULL
                         ORDER BY ts DESC, event_id DESC LIMIT 1",
                    )?
                    .query_row(params![system_address, body], |r| r.get(0))
                    .optional()?;
                let class = species_localised.clone().unwrap_or_else(|| species.clone());
                c.prepare_cached(
                    "INSERT INTO scans(event_id, cmdr, ts, kind, system_address, star_system, body_name, body_id, class)
                     VALUES (?1, ?2, ?3, 'organic', ?4, ?5, ?6, ?7, ?8)",
                )?
                .execute(params![self.id, self.cmdr, self.ts, system_address, star_system, body_name, body, class])?;
            }
            _ => {}
        }
        Ok(())
    }

    fn other(&self, event: &str, raw: &Value) -> Result<()> {
        let int = |k: &str| raw.get(k).and_then(Value::as_i64);
        let text = |k: &str| raw.get(k).and_then(Value::as_str).filter(|s| !s.is_empty());
        match event {
            "Commander" => {
                if let Some(name) = text("Name") {
                    self.commander(name)?;
                }
            }
            "SearchAndRescue" => {
                if let Some(r) = int("Reward") {
                    self.earn("search_and_rescue", r, None, text("Name_Localised").or(text("Name")))?;
                }
            }
            "CommunityGoalReward" => {
                if let Some(r) = int("Reward") {
                    self.earn("community_goal", r, None, text("Name"))?;
                }
            }
            "SellMicroResources" => {
                if let Some(p) = int("Price") {
                    self.earn("micro_resources", p, None, None)?;
                }
            }
            "ShipyardNew" | "ShipRedeemed" => {
                if let (Some(id), Some(t)) = (int("NewShipID"), text("ShipType")) {
                    self.owned_ship(id, t, None, None, false)?;
                    self.c
                        .prepare_cached(
                            "UPDATE ships SET acquired = ?3 WHERE cmdr = ?1 AND ship_id = ?2 AND sold IS NULL",
                        )?
                        .execute(params![self.cmdr, id, self.ts])?;
                }
            }
            "SetUserShipName" => {
                if let Some(id) = int("ShipID") {
                    self.c
                        .prepare_cached(
                            "UPDATE ships SET name = coalesce(?3, name), ident = coalesce(?4, ident), updated = ?5
                             WHERE cmdr = ?1 AND ship_id = ?2 AND sold IS NULL AND updated <= ?5",
                        )?
                        .execute(params![self.cmdr, id, text("UserShipName"), text("UserShipId"), self.ts])?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn commander(&self, name: &str) -> Result<()> {
        if name.is_empty() {
            return Ok(());
        }
        let legacy = self.cmdr.ends_with(astromech_journal::commander::LEGACY_SUFFIX);
        self.c
            .prepare_cached(
                "INSERT INTO commanders(cmdr, name, legacy, first_seen, last_seen) VALUES (?1, ?2, ?3, ?4, ?4)
                 ON CONFLICT(cmdr) DO UPDATE SET
                    name = iif(excluded.last_seen >= last_seen, excluded.name, name),
                    first_seen = min(first_seen, excluded.first_seen),
                    last_seen = max(last_seen, excluded.last_seen)",
            )?
            .execute(params![self.cmdr, name, legacy, self.ts])?;
        Ok(())
    }

    fn visit(
        &self,
        kind: &str,
        star_system: &str,
        system_address: Option<i64>,
        station_name: Option<&String>,
        market_id: Option<i64>,
        jump_dist: Option<f64>,
    ) -> Result<()> {
        self.c
            .prepare_cached(
                "INSERT INTO visits(cmdr, event_id, ts, kind, star_system, system_address, station_name, market_id,
                    jump_dist, via)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            )?
            .execute(params![
                self.cmdr,
                self.id,
                self.ts,
                kind,
                star_system,
                system_address,
                station_name,
                market_id,
                jump_dist,
                self.via
            ])?;
        Ok(())
    }

    fn earn(&self, kind: &str, amount: i64, cost: Option<i64>, detail: Option<&str>) -> Result<()> {
        self.c
            .prepare_cached(
                "INSERT INTO earnings(event_id, cmdr, ts, kind, amount, cost, detail)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )?
            .execute(params![self.id, self.cmdr, self.ts, kind, amount, cost, detail])?;
        Ok(())
    }

    /// The commander owns ship `ship_id`, a `ship_type`, at this event (`flown`: it is the ship
    /// they are in). An unsold row for that id of another type is a ship gone by now, whose sale
    /// isn't in the journals; an older event doesn't close it (rows take values only from newer
    /// events, and an import of older events rebuilds the ledgers in game-time order).
    fn owned_ship(
        &self,
        ship_id: i64,
        ship_type: &str,
        name: Option<&str>,
        ident: Option<&str>,
        flown: bool,
    ) -> Result<()> {
        let ship_type = ship_type.to_lowercase();
        self.c
            .prepare_cached(
                "UPDATE ships SET sold = ?4
                 WHERE cmdr = ?1 AND ship_id = ?2 AND sold IS NULL AND ship_type <> ?3 AND updated <= ?4",
            )?
            .execute(params![self.cmdr, ship_id, ship_type, self.ts])?;
        self.c
            .prepare_cached(
                "INSERT INTO ships(cmdr, ship_id, ship_type, name, ident, first_seen, last_flown, updated)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, iif(?7, ?6, NULL), ?6)
                 ON CONFLICT(cmdr, ship_id) WHERE sold IS NULL DO UPDATE SET
                    first_seen = min(first_seen, excluded.first_seen),
                    last_flown = coalesce(max(last_flown, excluded.last_flown), last_flown, excluded.last_flown),
                    name = iif(excluded.updated >= updated, coalesce(excluded.name, name), name),
                    ident = iif(excluded.updated >= updated, coalesce(excluded.ident, ident), ident),
                    updated = max(updated, excluded.updated)",
            )?
            .execute(params![self.cmdr, ship_id, ship_type, name, ident, self.ts, flown])?;
        Ok(())
    }

    /// Name of the system with this address, from the arrivals recorded so far (any
    /// commander: names don't depend on who flew there).
    fn system_name_of(&self, address: Option<i64>) -> Result<Option<String>> {
        let Some(a) = address else { return Ok(None) };
        self.c
            .prepare_cached(
                "SELECT star_system FROM visits WHERE kind = 'system' AND system_address = ?1
                 ORDER BY ts DESC, id DESC LIMIT 1",
            )?
            .query_row([a], |r| r.get(0))
            .optional()
    }
}

/// Reads of one commander's ledgers, oldest first.
impl crate::History {
    pub fn commanders(&self) -> crate::Result<Vec<Commander>> {
        let mut st = self
            .conn
            .prepare("SELECT cmdr, name, legacy, first_seen, last_seen FROM commanders ORDER BY last_seen DESC")?;
        let rows = st.query_map([], |r| {
            Ok(Commander {
                cmdr: r.get(0)?,
                name: r.get(1)?,
                legacy: r.get(2)?,
                first_seen: r.get(3)?,
                last_seen: r.get(4)?,
            })
        })?;
        Ok(rows.collect::<Result<_>>()?)
    }

    /// How often the commander called `name` arrived in the system `address` before `before`
    /// (a journal timestamp, not included), and when last; Legacy-mode journals are ignored.
    /// One indexed lookup (`visits_address`), cheap enough for the reaction engine.
    pub fn visits_before(&self, name: &str, address: i64, before: &str) -> crate::Result<Option<(i64, String)>> {
        let mut row = self.conn.prepare_cached(
            "SELECT count(*), max(ts) FROM visits
             WHERE system_address = ?1 AND kind = 'system' AND ts < ?2
               AND cmdr IN (SELECT cmdr FROM commanders WHERE name = ?3 AND legacy = 0)",
        )?;
        let (n, last): (i64, Option<String>) =
            row.query_row(params![address, before, name], |r| Ok((r.get(0)?, r.get(1)?)))?;
        Ok(last.map(|l| (n, l)))
    }

    pub fn visits(&self, cmdr: &str) -> crate::Result<Vec<Visit>> {
        let mut st = self.conn.prepare(
            "SELECT ts, kind, star_system, system_address, station_name, market_id, jump_dist, via FROM visits
             WHERE cmdr = ?1 ORDER BY ts, id",
        )?;
        let rows = st.query_map([cmdr], |r| {
            Ok(Visit {
                ts: r.get(0)?,
                kind: r.get(1)?,
                star_system: r.get(2)?,
                system_address: r.get(3)?,
                station_name: r.get(4)?,
                market_id: r.get(5)?,
                jump_dist: r.get(6)?,
                via: r.get(7)?,
            })
        })?;
        Ok(rows.collect::<Result<_>>()?)
    }

    pub fn earnings(&self, cmdr: &str) -> crate::Result<Vec<Earning>> {
        let mut st = self
            .conn
            .prepare("SELECT ts, kind, amount, cost, detail FROM earnings WHERE cmdr = ?1 ORDER BY ts, event_id")?;
        let rows = st.query_map([cmdr], |r| {
            Ok(Earning { ts: r.get(0)?, kind: r.get(1)?, amount: r.get(2)?, cost: r.get(3)?, detail: r.get(4)? })
        })?;
        Ok(rows.collect::<Result<_>>()?)
    }

    pub fn ships(&self, cmdr: &str) -> crate::Result<Vec<Ship>> {
        let mut st = self.conn.prepare(
            "SELECT ship_id, ship_type, name, ident, first_seen, last_flown, acquired, sold FROM ships
             WHERE cmdr = ?1 ORDER BY ship_id, id",
        )?;
        let rows = st.query_map([cmdr], |r| {
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

    pub fn scans(&self, cmdr: &str) -> crate::Result<Vec<Scan>> {
        let mut st = self.conn.prepare(
            "SELECT ts, kind, system_address, star_system, body_name, body_id, class, landable, terraformable,
                    was_discovered, was_mapped FROM scans WHERE cmdr = ?1 ORDER BY ts, event_id",
        )?;
        let rows = st.query_map([cmdr], |r| {
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
