//! Every owned ship with its fitted modules, from journal history.
//!
//! The game writes `Loadout` only for the ship being flown, but a ship can only be refitted or
//! engineered while flown, so the newest `Loadout` per `ShipID` stays true until that ship is
//! flown again. `StoredShips` (each shipyard visit) lists the rest of the fleet and drops ships
//! sold while no journal was kept; `ShipyardSell` / `SellShipOnRebuy` drop them as they happen.

use crate::data;
use astromech_journal::{JournalEvent, ModuleSnap, StoredShip};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Fleet {
    pub source: String,
    pub ships: Vec<Ship>,
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
    /// When this loadout was last seen. `None`: never flown while a journal was kept.
    pub loadout_ts: Option<String>,
    pub modules: Vec<Fitted>,
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
    modules: Vec<ModuleSnap>,
}

/// Folds fleet events in journal order.
#[derive(Debug, Default)]
pub struct FleetBuilder {
    loadouts: HashMap<i64, Seen>,
    stored: HashMap<i64, StoredShip>,
    current: Option<i64>,
}

impl FleetBuilder {
    pub fn apply(&mut self, ts: &str, ev: &JournalEvent) {
        match ev {
            JournalEvent::Loadout { ship_id: Some(id), ship, ship_name, ship_ident, modules, .. } => {
                self.current = Some(*id);
                self.stored.remove(id);
                self.loadouts.insert(
                    *id,
                    Seen {
                        ts: ts.to_string(),
                        ship_type: ship.clone().unwrap_or_default().to_ascii_lowercase(),
                        name: ship_name.clone(),
                        ident: ship_ident.clone(),
                        modules: modules.clone(),
                    },
                );
            }
            JournalEvent::StoredShips { ships } => {
                self.stored = ships.iter().map(|s| (s.ship_id, s.clone())).collect();
                let current = self.current;
                self.loadouts.retain(|id, _| Some(*id) == current || self.stored.contains_key(id));
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
        Fleet { source: source.into(), ships, groups }
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
            loadout_ts: seen.map(|s| s.ts.clone()),
            modules,
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

const FLEET_EVENTS: &[&str] = &["\"Loadout\"", "\"StoredShips\"", "\"ShipyardSell\"", "\"SellShipOnRebuy\""];

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
            }
        }
    }
    let source = if b.loadouts.is_empty() {
        format!("no Loadout event in {}", dir.display())
    } else {
        format!("{} journal files", logs.len())
    };
    b.build(source)
}
