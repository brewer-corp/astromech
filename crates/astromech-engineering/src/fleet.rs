//! Every owned ship with its fitted modules, from journal history.
//!
//! The game writes `Loadout` only for the ship being flown, but a ship can only be refitted or
//! engineered while flown, so the newest `Loadout` per `ShipID` stays true until that ship is
//! flown again. `StoredShips` (each shipyard visit) lists the rest of the fleet and drops ships
//! sold while no journal was kept; `ShipyardSell` / `SellShipOnRebuy` drop them as they happen.
//! `ModuleStore`, `ModuleRetrieve`, `ModuleSellRemote` and `FetchRemoteModule` keep the stored
//! modules current between outfitting visits, and a `TransferTime` settles ships and modules
//! ordered to a station.
//! Each `EngineerCraft` (in person or remote) updates the flown ship's module straight away, so
//! the plan and trip follow engineering without waiting for the next `Loadout`.

use crate::{data, links};
use astromech_journal::{JournalEvent, ModEngineering, ModuleSnap, StoredModule, StoredShip};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Fleet {
    pub source: String,
    pub ships: Vec<Ship>,
    /// Modules in storage, from the last `StoredModules`.
    pub stored_modules: Vec<Stored>,
    /// Engineering options for every module group on these ships.
    pub groups: BTreeMap<String, GroupOptions>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Ship {
    pub id: i64,
    pub ship_type: String,
    pub type_name: String,
    pub name: Option<String>,
    pub ident: Option<String>,
    pub system: Option<String>,
    pub current: bool,
    /// Being moved to a shipyard; `system` is where it is going, if known.
    pub in_transit: bool,
    /// Market id of the station a stored ship is at, for the Spansh station link.
    pub market_id: Option<i64>,
    /// When this loadout was last seen. `None`: never flown while a journal was kept.
    pub loadout_ts: Option<String>,
    /// Unladen maximum from the last `Loadout`.
    pub max_jump_range: Option<f64>,
    /// What the ship is worth: the shipyard's value for a stored ship, else hull plus modules.
    pub value: Option<i64>,
    /// Insurance excess if the ship is lost, from its last `Loadout`.
    pub rebuy: Option<i64>,
    pub modules: Vec<Fitted>,
    /// The `Loadout` modules as the journal wrote them, crafts since included: what ship stats
    /// are worked out from.
    #[serde(skip)]
    pub loadout: Vec<ModuleSnap>,
    /// Builder links for this loadout. `None` when the raw `Loadout` was not kept, or an
    /// `EngineerCraft` since then means it no longer matches the ship.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ShipLinks>,
}

/// A module in storage.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Stored {
    pub item: String,
    pub name: String,
    /// `None` while in transit.
    pub system: Option<String>,
    pub value: Option<i64>,
    /// Blueprint name and grade when engineered.
    pub blueprint_name: Option<String>,
    pub grade: Option<u8>,
}

/// Where to open a ship's build in the community outfitting tools.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShipLinks {
    pub coriolis: String,
    pub edsy: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Fitted {
    pub slot: String,
    pub slot_name: String,
    pub item: String,
    pub name: String,
    /// Module group id in [`Fleet::groups`]; `None` if the module cannot be engineered.
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engineering: Option<Applied>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Applied {
    pub blueprint: String,
    pub blueprint_name: String,
    pub grade: u8,
    pub quality: f64,
    pub experimental: Option<String>,
    pub experimental_name: Option<String>,
    pub engineer: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GroupOptions {
    pub name: String,
    pub blueprints: Vec<Choice>,
    pub experimentals: Vec<Choice>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Choice {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_grade: Option<u8>,
}

#[derive(Debug, Clone)]
struct Seen {
    ts: String,
    ship_type: String,
    name: Option<String>,
    ident: Option<String>,
    max_jump_range: Option<f64>,
    rebuy: Option<i64>,
    value: Option<i64>,
    modules: Vec<ModuleSnap>,
    /// The `Loadout` line as written, for the builder links.
    raw: Option<Value>,
    /// An `EngineerCraft` changed a module since `raw` was written.
    crafted: bool,
}

/// A transfer in progress: when it lands (epoch seconds), and where.
#[derive(Debug, Clone)]
struct Arrival {
    at: i64,
    system: Option<String>,
    market_id: Option<i64>,
}

fn epoch(ts: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(ts).ok().map(|t| t.timestamp())
}

/// Folds fleet events in journal order.
#[derive(Debug, Default)]
pub struct FleetBuilder {
    loadouts: HashMap<i64, Seen>,
    stored: HashMap<i64, StoredShip>,
    /// With the epoch second a module ordered to us arrives, and the system it arrives in.
    stored_modules: Vec<(StoredModule, Option<Arrival>)>,
    /// When ships ordered to us arrive.
    ship_arrivals: HashMap<i64, Arrival>,
    current: Option<i64>,
    /// System of the last dock, for a ship left behind by a swap.
    here: Option<String>,
    /// Market id of the last dock, for the same.
    here_market: Option<i64>,
}

impl FleetBuilder {
    pub fn apply(&mut self, ts: &str, ev: &JournalEvent) {
        if let Some(now) = epoch(ts) {
            self.settle(now);
        }
        match ev {
            JournalEvent::Loadout {
                ship_id: Some(id),
                ship,
                ship_name,
                ship_ident,
                modules,
                max_jump_range,
                rebuy,
                hull_value,
                modules_value,
                ..
            } if !is_vehicle(ship.as_deref().unwrap_or("")) => {
                // Flying out of another ship leaves it where we are docked: StoredShips was
                // written before the swap and does not list it.
                if let Some(prev) = self.current.filter(|p| p != id) {
                    if let Some(old) = self.loadouts.get(&prev) {
                        self.stored.entry(prev).or_insert_with(|| StoredShip {
                            ship_id: prev,
                            ship_type: old.ship_type.clone(),
                            name: old.name.clone(),
                            system: self.here.clone(),
                            in_transit: false,
                            value: old.value,
                            market_id: self.here_market,
                        });
                    }
                }
                self.current = Some(*id);
                self.stored.remove(id);
                self.loadouts.insert(
                    *id,
                    Seen {
                        ts: ts.to_string(),
                        ship_type: ship.clone().unwrap_or_default().to_ascii_lowercase(),
                        name: ship_name.clone(),
                        ident: ship_ident.clone(),
                        max_jump_range: *max_jump_range,
                        rebuy: *rebuy,
                        value: hull_value.map(|h| h + modules_value.unwrap_or(0)),
                        modules: modules.clone(),
                        raw: None,
                        crafted: false,
                    },
                );
            }
            JournalEvent::StoredShips { ships } => {
                self.stored = ships.iter().map(|s| (s.ship_id, s.clone())).collect();
                self.ship_arrivals.clear();
                let current = self.current;
                self.loadouts.retain(|id, _| Some(*id) == current || self.stored.contains_key(id));
            }
            JournalEvent::Docked { star_system, market_id, .. } => {
                self.here = Some(star_system.clone());
                self.here_market = *market_id;
            }
            JournalEvent::Location { star_system, docked, market_id, .. } => {
                self.here = docked.then(|| star_system.clone());
                self.here_market = market_id.filter(|_| *docked);
            }
            JournalEvent::ShipTransfer { ship_id, ship_type, transfer_time, .. } => {
                // It arrives where we are: in transit until `TransferTime` has passed.
                let here = self.here.clone();
                let s = self.stored.entry(*ship_id).or_insert_with(|| StoredShip {
                    ship_id: *ship_id,
                    ship_type: ship_type.clone(),
                    name: None,
                    system: None,
                    in_transit: false,
                    value: None,
                    market_id: None,
                });
                s.system = here.clone();
                s.market_id = None;
                s.in_transit = true;
                self.ship_arrivals.remove(ship_id);
                if let Some(at) = epoch(ts).zip(*transfer_time).map(|(t, secs)| t + secs) {
                    self.ship_arrivals.insert(*ship_id, Arrival { at, system: here, market_id: self.here_market });
                }
            }
            JournalEvent::StoredModules { items } => {
                self.stored_modules = items.iter().map(|m| (m.clone(), None)).collect();
            }
            JournalEvent::ModuleStore { module } => {
                self.stored_modules.push((StoredModule { system: self.here.clone(), ..module.clone() }, None));
            }
            JournalEvent::ModuleRetrieve { module, swapped_out } => {
                self.take_module(|m| {
                    m.item == module.item && m.blueprint == module.blueprint && m.level == module.level
                });
                if let Some(old) = swapped_out {
                    self.stored_modules.push((StoredModule { system: self.here.clone(), ..old.clone() }, None));
                }
            }
            JournalEvent::ModuleSellRemote { item } => self.take_module(|m| m.item == *item),
            JournalEvent::ModuleFetch { item, transfer_time, .. } => {
                // The event does not say which one; the nearest to being ready is not known
                // either, so take the first stored copy that is not already on its way here.
                let at = epoch(ts).zip(*transfer_time).map(|(t, secs)| t + secs);
                let here = self.here.clone();
                if let Some((m, arrival)) = self
                    .stored_modules
                    .iter_mut()
                    .find(|(m, a)| m.item == *item && a.is_none() && m.system.is_some() && m.system != here)
                {
                    m.system = None;
                    *arrival = at.map(|at| Arrival { at, system: here, market_id: None });
                }
            }
            JournalEvent::EngineerCraft {
                engineer,
                blueprint,
                level,
                quality,
                slot: Some(slot),
                experimental,
                experimental_localised,
                modifiers,
                preview: false,
                ..
            } => {
                let Some(seen) = self.current.and_then(|id| self.loadouts.get_mut(&id)) else { return };
                let Some(m) = seen.modules.iter_mut().find(|m| m.slot.eq_ignore_ascii_case(slot)) else { return };
                let same = m.engineering.as_ref().is_some_and(|e| e.blueprint.eq_ignore_ascii_case(blueprint));
                // An experimental stays unless the craft names one or the blueprint changed.
                let kept = if same { m.engineering.as_ref().and_then(|e| e.experimental.clone()) } else { None };
                let kept_name =
                    if same { m.engineering.as_ref().and_then(|e| e.experimental_localised.clone()) } else { None };
                m.engineering = Some(ModEngineering {
                    blueprint: blueprint.clone(),
                    level: *level,
                    quality: *quality,
                    experimental: experimental.clone().or(kept),
                    experimental_localised: experimental_localised.clone().or(kept_name),
                    engineer: Some(engineer.clone()),
                    // The craft reports the module's stats after it; none means the stats
                    // follow the blueprint and roll.
                    modifiers: modifiers.clone(),
                });
                seen.ts = ts.to_string();
                seen.crafted = true;
            }
            JournalEvent::ShipSold { ship_id } => {
                self.loadouts.remove(ship_id);
                self.stored.remove(ship_id);
                if self.current == Some(*ship_id) {
                    self.current = None;
                }
            }
            _ => {}
        }
    }

    /// Remove the first stored module `pred` accepts, preferring one stored where we are docked.
    fn take_module(&mut self, pred: impl Fn(&StoredModule) -> bool) {
        let at = |(m, _): &&(StoredModule, Option<Arrival>)| pred(m) && m.system.is_some() && m.system == self.here;
        let i = self
            .stored_modules
            .iter()
            .position(|e| at(&e))
            .or_else(|| self.stored_modules.iter().position(|(m, _)| pred(m)));
        if let Some(i) = i {
            self.stored_modules.remove(i);
        }
    }

    /// Land every transfer due by `now` (epoch seconds): ships and modules ordered to a station
    /// stay in transit for their `TransferTime`, and the journal says nothing when they arrive.
    pub fn settle(&mut self, now: i64) {
        let due: Vec<i64> = self.ship_arrivals.iter().filter(|(_, a)| a.at <= now).map(|(id, _)| *id).collect();
        for id in due {
            let Some(a) = self.ship_arrivals.remove(&id) else { continue };
            if let Some(s) = self.stored.get_mut(&id) {
                s.in_transit = false;
                s.system = a.system;
                s.market_id = a.market_id;
            }
        }
        for (m, arrival) in &mut self.stored_modules {
            if arrival.as_ref().is_some_and(|a| a.at <= now) {
                m.system = arrival.take().and_then(|a| a.system);
            }
        }
    }

    /// Keep the raw `Loadout` line that was just applied (`ts` and `ship_id` must match it).
    pub fn keep_raw(&mut self, ts: &str, raw: Value) {
        let Some(id) = raw.get("ShipID").and_then(Value::as_i64) else { return };
        if let Some(seen) = self.loadouts.get_mut(&id).filter(|s| s.ts == ts) {
            seen.raw = Some(raw);
        }
    }

    pub fn build(&self, source: impl Into<String>) -> Fleet {
        let mut ids: Vec<i64> = self.loadouts.keys().chain(self.stored.keys()).copied().collect();
        ids.sort();
        ids.dedup();
        let mut ships: Vec<Ship> = ids.into_iter().map(|id| self.ship(id)).collect();
        ships.sort_by(|a, b| {
            b.current.cmp(&a.current).then_with(|| b.loadout_ts.is_some().cmp(&a.loadout_ts.is_some())).then_with(
                || {
                    let an = a.name.as_deref().unwrap_or(&a.type_name).to_lowercase();
                    let bn = b.name.as_deref().unwrap_or(&b.type_name).to_lowercase();
                    an.cmp(&bn)
                },
            )
        });
        let mut groups = BTreeMap::new();
        for g in ships.iter().flat_map(|s| s.modules.iter()).filter_map(|m| m.group.as_deref()) {
            if !groups.contains_key(g) {
                if let Some(o) = group_options(g) {
                    groups.insert(g.to_string(), o);
                }
            }
        }
        let stored_modules = self.stored_modules.iter().map(|(m, _)| stored_module(m)).collect();
        Fleet { source: source.into(), ships, stored_modules, groups }
    }

    fn ship(&self, id: i64) -> Ship {
        let seen = self.loadouts.get(&id);
        let stored = self.stored.get(&id);
        let ship_type =
            seen.map(|s| s.ship_type.clone()).or_else(|| stored.map(|s| s.ship_type.clone())).unwrap_or_default();
        let mut modules: Vec<Fitted> = seen.map(|s| s.modules.iter().filter_map(fitted).collect()).unwrap_or_default();
        modules.sort_by_key(|m| slot_order(&m.slot));
        Ship {
            id,
            type_name: data::ship_name(&ship_type),
            ship_type,
            name: seen.and_then(|s| s.name.clone()).or_else(|| stored.and_then(|s| s.name.clone())),
            ident: seen.and_then(|s| s.ident.clone()),
            system: stored.and_then(|s| s.system.clone()),
            current: self.current == Some(id),
            in_transit: stored.is_some_and(|s| s.in_transit),
            market_id: stored.and_then(|s| s.market_id),
            loadout_ts: seen.map(|s| s.ts.clone()),
            max_jump_range: seen.and_then(|s| s.max_jump_range),
            value: stored.and_then(|s| s.value).or_else(|| seen.and_then(|s| s.value)),
            rebuy: seen.and_then(|s| s.rebuy),
            modules,
            loadout: seen.map(|s| s.modules.clone()).unwrap_or_default(),
            links: seen
                .filter(|s| !s.crafted)
                .and_then(|s| s.raw.as_ref())
                .map(|raw| ShipLinks { coriolis: links::coriolis_url(raw), edsy: links::edsy_url(raw) }),
        }
    }
}

pub fn group_options(id: &str) -> Option<GroupOptions> {
    let g = data::group(id)?;
    Some(GroupOptions {
        name: g.name.clone(),
        blueprints: g
            .blueprints
            .iter()
            .filter_map(|b| data::blueprint(b))
            .map(|(id, b)| Choice { id: id.to_string(), name: b.name.clone(), max_grade: Some(b.max_grade()) })
            .collect(),
        experimentals: g
            .experimentals
            .iter()
            .filter_map(|e| data::experimental(e))
            .map(|(id, e)| Choice { id: id.to_string(), name: e.name.clone(), max_grade: None })
            .collect(),
    })
}

fn stored_module(m: &StoredModule) -> Stored {
    let name =
        data::module(&m.item).map(|i| i.display()).or_else(|| m.localised.clone()).unwrap_or_else(|| m.item.clone());
    let bp = m.blueprint.as_deref();
    Stored {
        item: m.item.clone(),
        name,
        system: m.system.clone(),
        value: m.value,
        blueprint_name: bp
            .map(|b| data::blueprint(b).map(|(_, d)| d.name.clone()).unwrap_or_else(|| b.replace('_', " "))),
        grade: bp.and(m.level),
    }
}

fn fitted(m: &ModuleSnap) -> Option<Fitted> {
    let info = data::module(&m.item);
    // Paint, decals, cockpit and the like are not in the outfitting catalogue: skip them.
    if info.is_none() && m.engineering.is_none() {
        return None;
    }
    let group = info.map(|i| i.group.clone()).filter(|g| data::group(g).is_some());
    let engineering = m.engineering.as_ref().map(|e| {
        let bp = data::blueprint(&e.blueprint);
        let ex = e.experimental.as_deref().map(|x| (x, data::experimental(x)));
        Applied {
            blueprint: bp.map(|(id, _)| id.to_string()).unwrap_or_else(|| e.blueprint.clone()),
            blueprint_name: bp.map(|(_, b)| b.name.clone()).unwrap_or_else(|| e.blueprint.replace('_', " ")),
            grade: e.level,
            quality: e.quality,
            experimental: ex.map(|(x, d)| d.map(|(id, _)| id.to_string()).unwrap_or_else(|| x.to_string())),
            experimental_name: ex.map(|(x, d)| {
                d.map(|(_, d)| d.name.clone())
                    .or_else(|| e.experimental_localised.clone())
                    .unwrap_or_else(|| x.to_string())
            }),
            engineer: e.engineer.clone(),
        }
    });
    Some(Fitted {
        slot: m.slot.clone(),
        slot_name: slot_name(&m.slot),
        item: m.item.clone(),
        name: info.map(|i| i.display()).unwrap_or_else(|| m.item.clone()),
        group,
        engineering,
    })
}

/// Taxis, SRVs and fighters write `Loadout` too; they are not part of the fleet.
pub fn is_vehicle(ship_type: &str) -> bool {
    let s = ship_type.to_ascii_lowercase();
    s.ends_with("_taxi") || s.contains("buggy") || s.contains("_srv") || s.ends_with("_fighter")
}

const CORE: &[(&str, &str)] = &[
    ("Armour", "Armour"),
    ("PowerPlant", "Power plant"),
    ("MainEngines", "Thrusters"),
    ("FrameShiftDrive", "Frame shift drive"),
    ("LifeSupport", "Life support"),
    ("PowerDistributor", "Power distributor"),
    ("Radar", "Sensors"),
    ("FuelTank", "Fuel tank"),
];

fn trailing_number(s: &str) -> (&str, Option<u32>) {
    let i = s.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    (&s[..i], s[i..].parse().ok())
}

/// "HugeHardpoint1" → "Huge hardpoint 1", "TinyHardpoint2" → "Utility 2",
/// "Slot03_Size5" → "Optional 3 (size 5)", "Military01" → "Military 1".
pub fn slot_name(slot: &str) -> String {
    if let Some((_, n)) = CORE.iter().find(|(k, _)| k.eq_ignore_ascii_case(slot)) {
        return n.to_string();
    }
    let (stem, n) = trailing_number(slot);
    let num = n.map(|n| format!(" {n}")).unwrap_or_default();
    if let Some(size) = stem.strip_suffix("Hardpoint") {
        return if size == "Tiny" { format!("Utility{num}") } else { format!("{size} hardpoint{num}") };
    }
    if let Some(rest) = slot.strip_prefix("Slot") {
        if let Some((n, size)) = rest.split_once("_Size") {
            return format!("Optional {} (size {size})", n.trim_start_matches('0'));
        }
    }
    if stem == "Military" {
        return format!("Military{num}");
    }
    slot.to_string()
}

fn slot_order(slot: &str) -> (u8, u8, String) {
    if let Some(i) = CORE.iter().position(|(k, _)| k.eq_ignore_ascii_case(slot)) {
        return (0, i as u8, String::new());
    }
    let (stem, _) = trailing_number(slot);
    let rank = match stem {
        "HugeHardpoint" => 0,
        "LargeHardpoint" => 1,
        "MediumHardpoint" => 2,
        "SmallHardpoint" => 3,
        _ => 0,
    };
    let class = if stem == "TinyHardpoint" {
        2
    } else if stem.ends_with("Hardpoint") {
        1
    } else {
        3
    };
    (class, rank, slot.to_string())
}

const FLEET_EVENTS: &[&str] = &[
    "\"Loadout\"",
    "\"StoredShips\"",
    "\"StoredModules\"",
    "\"ShipyardTransfer\"",
    "\"ModuleStore\"",
    "\"ModuleRetrieve\"",
    "\"ModuleSellRemote\"",
    "\"FetchRemoteModule\"",
    "\"Docked\"",
    "\"Location\"",
    "\"ShipyardSell\"",
    "\"ShipyardBuy\"",
    "\"SellShipOnRebuy\"",
    "\"EngineerCraft\"",
];

/// Replay every journal in `dir`, oldest first. Lines are pre-filtered by event name, so years
/// of journals cost one read each.
pub fn from_journal_dir(dir: &Path) -> Fleet {
    let mut b = FleetBuilder::default();
    let logs = astromech_journal::journal_logs(dir);
    for p in &logs {
        let Ok(text) = std::fs::read_to_string(p) else { continue };
        for line in text.lines().filter(|l| FLEET_EVENTS.iter().any(|e| l.contains(e))) {
            if let Ok(l) = astromech_journal::parse_line(line.trim()) {
                b.apply(&l.timestamp, &l.event);
                if matches!(l.event, astromech_journal::JournalEvent::Loadout { .. }) {
                    if let Ok(raw) = serde_json::from_str::<Value>(line.trim()) {
                        b.keep_raw(&l.timestamp, raw);
                    }
                }
            }
        }
    }
    let source = if b.loadouts.is_empty() {
        format!("no Loadout event in {}", dir.display())
    } else {
        format!("{} journal files", logs.len())
    };
    // Anything ordered to a station while the journals were being kept may have landed since.
    b.settle(chrono::Utc::now().timestamp());
    b.build(source)
}
