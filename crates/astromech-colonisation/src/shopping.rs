//! Shopping list for a construction project: what is still needed, minus what the ship already
//! carries, in loads at the current ship's capacity.
//!
//! Carrier cargo is not known: the journal gives only the carrier's free space (`CarrierStats`),
//! not what is stored, so it is not subtracted.

use crate::Project;
use astromech_journal::{ExtraEvent, JournalEvent, JournalLine};
use serde::Serialize;
use std::collections::BTreeMap;

/// The ship's hold, from `Loadout` (`CargoCapacity`) and `Cargo` (ship inventory).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hold {
    /// Tonnes the current ship can carry; `None` until a `Loadout` with the field is seen.
    pub capacity: Option<i64>,
    /// Commodity id (see [`commodity_id`]) to tonnes aboard.
    pub cargo: BTreeMap<String, i64>,
}

impl Hold {
    pub fn apply(&mut self, line: &JournalLine) {
        match &line.event {
            JournalEvent::Loadout { cargo_capacity: Some(c), .. } => self.capacity = Some(*c),
            // A `Cargo` event for the SRV lists its own hold, not the ship's.
            JournalEvent::Extra(ExtraEvent::Cargo { vessel, count, inventory })
                if vessel.as_deref().is_none_or(|v| v.eq_ignore_ascii_case("ship")) =>
            {
                // Count > 0 with no inventory means the list was not given: keep what we had.
                if inventory.is_empty() && count.is_none_or(|c| c > 0) {
                    return;
                }
                self.cargo.clear();
                for i in inventory {
                    // Mission cargo is not deliverable to a depot. Stolen goods are counted:
                    // depots are assumed to accept them (unverified).
                    if i.mission_id.is_none() {
                        self.add(&i.name, i.count);
                    }
                }
            }
            JournalEvent::Extra(ExtraEvent::MarketTrade(m)) => {
                self.add(&m.commodity, if m.sell { -m.count } else { m.count });
            }
            JournalEvent::Extra(ExtraEvent::ColonisationContribution { contributions, .. }) => {
                for (name, amount) in contributions {
                    self.add(name, -amount);
                }
            }
            // Not typed in the journal crate. `Transfers[]` entries carry `Type`, `Count` and
            // `Direction`: `toship` adds to the hold, `tocarrier` and `tosrv` take it off the ship.
            JournalEvent::Other { event, raw } if event == "CargoTransfer" => {
                for t in raw.get("Transfers").and_then(|v| v.as_array()).into_iter().flatten() {
                    let name = t.get("Type").and_then(|v| v.as_str()).unwrap_or_default();
                    let count = t.get("Count").and_then(|v| v.as_i64()).unwrap_or(0);
                    match t.get("Direction").and_then(|v| v.as_str()) {
                        Some("toship") => self.add(name, count),
                        Some("tocarrier" | "tosrv") => self.add(name, -count),
                        _ => {}
                    }
                }
            }
            // The cargo is lost with the ship.
            JournalEvent::Other { event, .. } if event == "Died" => self.cargo.clear(),
            // Not typed in the journal crate: `CollectCargo` is one unit per event, `EjectCargo`
            // carries `Count`. Either may be mission cargo (`MissionID`), which is not tracked.
            JournalEvent::Other { event, raw } if raw.get("MissionID").is_none() => {
                let name = raw.get("Type").and_then(|v| v.as_str()).unwrap_or_default();
                match event.as_str() {
                    "CollectCargo" => self.add(name, 1),
                    "EjectCargo" => self.add(name, -raw.get("Count").and_then(|v| v.as_i64()).unwrap_or(1)),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Take the ship's hold from `Cargo.json` (the game rewrites it on every change). An SRV's file, one that doesn't
    /// parse, or one with a count but no list leaves the hold as it was. Mission cargo is skipped, as in `Cargo` events.
    pub fn apply_cargo_json(&mut self, bytes: &[u8]) {
        let Ok(v) = serde_json::from_slice::<serde_json::Value>(bytes) else { return };
        if v.get("Vessel").and_then(|x| x.as_str()).is_some_and(|x| !x.eq_ignore_ascii_case("ship")) {
            return;
        }
        let inv = v.get("Inventory").and_then(|x| x.as_array()).map(Vec::as_slice).unwrap_or_default();
        // As in `apply`: no list with a count > 0 means it wasn't given; an empty hold is written
        // as `"Count":0` with no Inventory.
        if inv.is_empty() && v.get("Count").and_then(|x| x.as_i64()).is_none_or(|c| c > 0) {
            return;
        }
        self.cargo.clear();
        for it in inv {
            if it.get("MissionID").is_some_and(|m| !m.is_null()) {
                continue;
            }
            let name = it.get("Name").and_then(|x| x.as_str()).unwrap_or_default();
            self.add(name, it.get("Count").and_then(|x| x.as_i64()).unwrap_or(0));
        }
    }

    fn add(&mut self, name: &str, delta: i64) {
        if name.is_empty() {
            return;
        }
        let n = self.cargo.entry(commodity_id(name)).or_insert(0);
        *n = (*n + delta).max(0);
    }
}

/// `$steel_name;` (depot) and `steel` (cargo) both become `steel`.
pub fn commodity_id(name: &str) -> String {
    let n = name.trim().to_lowercase();
    let n = n.strip_prefix('$').unwrap_or(&n);
    n.strip_suffix("_name;").unwrap_or(n).to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Line {
    /// Market symbol ([`commodity_id`]), as Ardent and `Cargo.json` name it: `steel`.
    pub symbol: String,
    pub name: String,
    pub name_localised: Option<String>,
    pub remaining: i64,
    /// Aboard now, counted only up to `remaining`.
    pub in_cargo: i64,
    pub to_buy: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShoppingList {
    /// Commodities still needed, largest `to_buy` first.
    pub lines: Vec<Line>,
    pub to_buy_total: i64,
    pub capacity: Option<i64>,
    /// Full ship loads to buy and haul; `None` without a known capacity.
    pub loads: Option<i64>,
}

pub fn shopping_list(project: &Project, hold: &Hold) -> ShoppingList {
    let mut lines: Vec<Line> = project
        .resources
        .iter()
        .filter(|r| r.remaining() > 0)
        .map(|r| {
            let remaining = r.remaining();
            let in_cargo = hold.cargo.get(&commodity_id(&r.name)).copied().unwrap_or(0).clamp(0, remaining);
            Line {
                symbol: commodity_id(&r.name),
                name: r.name.clone(),
                name_localised: r.name_localised.clone(),
                remaining,
                in_cargo,
                to_buy: remaining - in_cargo,
            }
        })
        .collect();
    lines.sort_by(|a, b| b.to_buy.cmp(&a.to_buy).then_with(|| a.name.cmp(&b.name)));
    let to_buy_total = lines.iter().map(|l| l.to_buy).sum();
    let loads = hold.capacity.filter(|c| *c > 0).map(|c| (to_buy_total + c - 1) / c);
    ShoppingList { lines, to_buy_total, capacity: hold.capacity, loads }
}
