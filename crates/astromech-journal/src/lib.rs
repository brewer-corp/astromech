//! # astromech-journal
//!
//! Reads the game's journal (`Journal.*.log`, one JSON object per line) and companion files.
//! Seed of the ingest pipeline (ARCHITECTURE.md §5.1, S6 verdict: own parser, lenient on
//! unknown events, `ed-journals` corpus only for tests).
//!
//! Events with a dedicated form are in [`JournalEvent`] (the first 15) and
//! [`ExtraEvent`] (the toolbox set, in `detail`); everything else stays as
//! [`JournalEvent::Other`]. Every parsed line keeps its original text
//! ([`JournalLine::raw_json`]) so the EDDN sender can forward the game's own keys.
//! Companion files (`Market.json`, `Cargo.json`, ...) are in [`companion`].

pub mod companion;
mod detail;
pub mod ore;

use detail::system_powerplay;
pub use detail::*;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error("not JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("missing field `{0}`")]
    Missing(&'static str),
}

/// One journal line. Known events are typed; the rest keep their raw JSON.
#[derive(Debug, Clone, PartialEq)]
pub enum JournalEvent {
    FileHeader {
        game_version: String,
        build: String,
        odyssey: bool,
    },
    LoadGame {
        commander: String,
        ship: Option<String>,
        ship_name: Option<String>,
        ship_ident: Option<String>,
        credits: Option<i64>,
    },
    Loadout {
        ship: Option<String>,
        ship_name: Option<String>,
        ship_ident: Option<String>,
        hull_health: Option<f64>,
        cargo_capacity: Option<i64>,
        max_jump_range: Option<f64>,
        fuel_capacity_main: Option<f64>,
        rebuy: Option<i64>,
        hull_value: Option<i64>,
        modules_value: Option<i64>,
        ship_id: Option<i64>,
        modules: Vec<ModuleSnap>,
    },
    /// Ships at this station and elsewhere (`StoredShips`, written when the shipyard opens).
    /// The ship being flown is not listed.
    StoredShips {
        ships: Vec<StoredShip>,
    },
    /// Modules in storage (`StoredModules`, written when outfitting opens).
    StoredModules {
        items: Vec<StoredModule>,
    },
    /// A stored ship was ordered to the system the player is in (`ShipyardTransfer`).
    ShipTransfer {
        ship_id: i64,
        ship_type: String,
    },
    /// A ship left the fleet (`ShipyardSell`, `SellShipOnRebuy`).
    ShipSold {
        ship_id: i64,
    },
    Location {
        star_system: String,
        body: Option<String>,
        docked: bool,
        allegiance: String,
        population: i64,
        factions: Vec<FactionSnap>,
        system_address: Option<i64>,
        star_pos: Option<[f64; 3]>,
        /// Set when starting docked (`Docked: true`).
        station_name: Option<String>,
        market_id: Option<i64>,
        /// What the station offers; `Some` when `docked`.
        station: Option<StationInfo>,
        /// The system's Powerplay fields, if it has any.
        powerplay: Option<Box<SystemPowerplay>>,
    },
    ApproachBody {
        star_system: String,
        body: String,
    },
    LeaveBody {
        star_system: String,
        body: String,
    },
    FsdJump {
        star_system: String,
        jump_dist: f64,
        allegiance: String,
        population: i64,
        factions: Vec<FactionSnap>,
        /// True for `CarrierJump` (stay docked; do not clear station).
        from_carrier: bool,
        system_address: Option<i64>,
        star_pos: Option<[f64; 3]>,
        /// The system's Powerplay fields, if it has any.
        powerplay: Option<Box<SystemPowerplay>>,
        /// Tonnes of fuel the jump used and left in the main tank (`FuelUsed`, `FuelLevel`;
        /// `FSDJump` only).
        fuel_used: Option<f64>,
        fuel_level: Option<f64>,
        /// A supercharged or injected jump (`BoostUsed`: neutron, white dwarf, FSD injection):
        /// the distance is boosted but the fuel is an ordinary jump's.
        boost_used: bool,
    },
    /// FSS signal in the current system (`FSSSignalDiscovered`).
    FssSignal {
        uss_type: String,
        high_grade: bool,
        /// `SignalName` (stations, carriers, conflict zones, ...).
        signal_name: Option<String>,
        is_station: bool,
        system_address: Option<i64>,
        /// The minor faction that spawned a USS and its state (`SpawningFaction`,
        /// `SpawningState`, localised when the game gives it: "Boom").
        spawning_faction: Option<String>,
        spawning_state: Option<String>,
    },
    Docked {
        station_name: String,
        star_system: String,
        market_id: Option<i64>,
        system_address: Option<i64>,
        station: StationInfo,
    },
    Undocked {
        station_name: String,
    },
    /// Full ship-materials inventory at login (`Materials`).
    Materials {
        raw: Vec<MaterialStack>,
        manufactured: Vec<MaterialStack>,
        encoded: Vec<MaterialStack>,
    },
    /// Signed counts from collect / discard / trade / synthesis / rewards.
    MaterialDelta {
        items: Vec<MaterialStack>,
    },
    /// One engineering roll (`EngineerCraft`); `ingredients` are already negative counts.
    EngineerCraft {
        engineer: String,
        engineer_id: Option<i64>,
        blueprint: String,
        level: u8,
        /// Progress within `level` after this roll, 0.0–1.0.
        quality: f64,
        slot: Option<String>,
        /// Experimental on the module after this craft (`ExperimentalEffect`, or the one being
        /// applied, `ApplyExperimentalEffect`).
        experimental: Option<String>,
        experimental_localised: Option<String>,
        /// The module's engineered stats after this roll (`Modifiers`).
        modifiers: Vec<EngModifier>,
        /// A preview (no change to the module).
        preview: bool,
        ingredients: Vec<MaterialStack>,
    },
    /// Engineer access and rank (`EngineerProgress`): all engineers at login, one on a change.
    EngineerProgress {
        engineers: Vec<EngineerSnap>,
    },
    /// One of the events typed for the toolbox work (see [`ExtraEvent`]).
    Extra(ExtraEvent),
    Other {
        event: String,
        raw: serde_json::Value,
    },
}

/// One material quantity. `count` is absolute in [`JournalEvent::Materials`] and signed in
/// [`JournalEvent::MaterialDelta`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialStack {
    pub name: String,
    pub name_localised: Option<String>,
    pub count: i64,
    pub category: Option<String>,
}

/// One fitted module on `Loadout`.
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleSnap {
    pub slot: String,
    /// Outfitting symbol, lowercase (`int_hyperdrive_size5_class5`).
    pub item: String,
    /// Powered on (`On`; missing counts as on).
    pub on: bool,
    /// Power priority group, 0–4 (`Priority`; the game shows it as 1–5).
    pub priority: u8,
    pub engineering: Option<ModEngineering>,
}

/// `Engineering` block of a fitted module.
#[derive(Debug, Clone, PartialEq)]
pub struct ModEngineering {
    pub blueprint: String,
    pub level: u8,
    /// Progress within `level`, 0.0–1.0.
    pub quality: f64,
    pub experimental: Option<String>,
    pub experimental_localised: Option<String>,
    pub engineer: Option<String>,
    /// The module's engineered stats as the game reports them (`Modifiers`): the real roll of
    /// each stat, which `level` and `quality` only approximate (and say nothing about for
    /// modules engineered before 3.0, which show `Quality` 0). Empty when not written.
    pub modifiers: Vec<EngModifier>,
}

/// One entry of an engineering `Modifiers` list: a stat's value on the module and on the
/// unengineered module, in the journal's units (resistances and shield or hull multipliers in
/// percent). Entries with only a text value (`ValueStr`) are not kept.
#[derive(Debug, Clone, PartialEq)]
pub struct EngModifier {
    /// `FSDOptimalMass`, `ShieldGenStrength`, `KineticResistance`, ...
    pub label: String,
    pub value: f64,
    pub original_value: f64,
}

/// One entry of `StoredShips` (`ShipsHere` and `ShipsRemote`).
#[derive(Debug, Clone, PartialEq)]
pub struct StoredShip {
    pub ship_id: i64,
    pub ship_type: String,
    pub name: Option<String>,
    pub system: Option<String>,
    /// On its way to `system` (or to wherever it was last written, if `system` is `None`).
    pub in_transit: bool,
    pub value: Option<i64>,
    /// Market id of the station it is at (`ShipMarketID`, or the shipyard's `MarketID` for ships
    /// here); `None` in transit or when the journal did not say.
    pub market_id: Option<i64>,
}

/// One entry of `StoredModules`.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredModule {
    /// Outfitting symbol, lowercase.
    pub item: String,
    pub localised: Option<String>,
    /// `None` while in transit.
    pub system: Option<String>,
    pub value: Option<i64>,
    /// Blueprint symbol and grade, if the stored module is engineered.
    pub blueprint: Option<String>,
    pub level: Option<u8>,
}

/// One engineer on `EngineerProgress`. `progress` is `Known`, `Invited`, `Acquainted` or
/// `Unlocked`; `rank` is set once unlocked.
#[derive(Debug, Clone, PartialEq)]
pub struct EngineerSnap {
    pub engineer: String,
    pub engineer_id: Option<i64>,
    pub progress: String,
    pub rank: Option<u8>,
    pub rank_progress: Option<u8>,
}

/// One minor faction on `Location` / `FSDJump` / `CarrierJump`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct FactionSnap {
    pub name: String,
    pub allegiance: String,
    pub state: String,
    pub influence: f64,
    pub active_states: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JournalLine {
    pub timestamp: String,
    /// The journal event name (`FSDJump`, `Market`, ...), also for events typed as one variant.
    pub name: String,
    pub event: JournalEvent,
    /// The line exactly as read (no re-serialising), for forwarding the game's own keys.
    pub raw_json: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CargoSnap {
    pub count: i64,
    /// (display name, count), most first: for speaking the hold's contents.
    pub items: Vec<(String, i64)>,
    /// Every `Inventory` line as the game wrote it, in file order.
    pub inventory: Vec<CargoItem>,
}

/// One `Cargo.json` inventory line.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct CargoItem {
    /// Lowercase symbol, as markets use it: "lowtemperaturediamond".
    pub symbol: String,
    /// `Name_Localised` if the game gave one, else the symbol.
    pub name: String,
    pub count: i64,
    /// Of `count`, how many are stolen (only a black market buys those).
    pub stolen: i64,
    /// Mission cargo: not the commander's to sell.
    pub mission: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NavRouteSnap {
    pub systems: Vec<String>,
    /// The same systems with their star class and position.
    pub hops: Vec<RouteHop>,
}

/// One system of `NavRoute.json`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct RouteHop {
    pub system: String,
    /// `StarClass` as the game writes it: "K", "DA", "N", "M_RedGiant", ...
    pub star_class: Option<String>,
    pub star_pos: Option<[f64; 3]>,
}

struct Head {
    timestamp: Option<String>,
    event: Option<String>,
}

fn s(v: &serde_json::Value, k: &str) -> String {
    v.get(k).and_then(|x| x.as_str()).unwrap_or_default().to_string()
}

fn opt_s(v: &serde_json::Value, k: &str) -> Option<String> {
    v.get(k).and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(str::to_string)
}

fn opt_f(v: &serde_json::Value, k: &str) -> Option<f64> {
    v.get(k).and_then(|x| x.as_f64()).or_else(|| v.get(k).and_then(|x| x.as_i64()).map(|n| n as f64))
}

fn opt_i(v: &serde_json::Value, k: &str) -> Option<i64> {
    v.get(k).and_then(|x| x.as_i64()).or_else(|| v.get(k).and_then(|x| x.as_f64()).map(|n| n as i64))
}

/// Parse one line. Unknown events never fail; malformed JSON does.
pub fn parse_line(line: &str) -> Result<JournalLine, JournalError> {
    let raw: serde_json::Value = serde_json::from_str(line)?;
    let head = Head { timestamp: opt_s(&raw, "timestamp"), event: opt_s(&raw, "event") };
    let event = head.event.ok_or(JournalError::Missing("event"))?;
    let timestamp = head.timestamp.ok_or(JournalError::Missing("timestamp"))?;
    let name = event.clone();
    let ev = match event.as_str() {
        "Fileheader" => JournalEvent::FileHeader {
            game_version: s(&raw, "gameversion"),
            build: s(&raw, "build"),
            odyssey: raw.get("Odyssey").and_then(|x| x.as_bool()).unwrap_or(false),
        },
        "LoadGame" => JournalEvent::LoadGame {
            commander: s(&raw, "Commander"),
            ship: opt_s(&raw, "Ship"),
            ship_name: opt_s(&raw, "ShipName"),
            ship_ident: opt_s(&raw, "ShipIdent"),
            credits: opt_i(&raw, "Credits"),
        },
        "Loadout" => JournalEvent::Loadout {
            ship: opt_s(&raw, "Ship"),
            ship_name: opt_s(&raw, "ShipName"),
            ship_ident: opt_s(&raw, "ShipIdent"),
            hull_health: opt_f(&raw, "HullHealth"),
            cargo_capacity: opt_i(&raw, "CargoCapacity"),
            max_jump_range: opt_f(&raw, "MaxJumpRange"),
            fuel_capacity_main: raw.get("FuelCapacity").and_then(|f| opt_f(f, "Main")),
            rebuy: opt_i(&raw, "Rebuy"),
            hull_value: opt_i(&raw, "HullValue"),
            modules_value: opt_i(&raw, "ModulesValue"),
            ship_id: opt_i(&raw, "ShipID"),
            modules: modules(&raw),
        },
        "StoredShips" => JournalEvent::StoredShips { ships: stored_ships(&raw) },
        "StoredModules" => JournalEvent::StoredModules { items: stored_modules(&raw) },
        "ShipyardTransfer" => match opt_i(&raw, "ShipID") {
            Some(ship_id) => {
                JournalEvent::ShipTransfer { ship_id, ship_type: s(&raw, "ShipType").to_ascii_lowercase() }
            }
            None => JournalEvent::Other { event, raw },
        },
        "ShipyardSell" => match opt_i(&raw, "SellShipID") {
            Some(ship_id) => JournalEvent::ShipSold { ship_id },
            None => JournalEvent::Other { event, raw },
        },
        // Buying a ship can trade in the current one.
        "ShipyardBuy" => match opt_i(&raw, "SellShipID") {
            Some(ship_id) => JournalEvent::ShipSold { ship_id },
            None => JournalEvent::Other { event, raw },
        },
        "SellShipOnRebuy" => match opt_i(&raw, "SellShipId").or_else(|| opt_i(&raw, "SellShipID")) {
            Some(ship_id) => JournalEvent::ShipSold { ship_id },
            None => JournalEvent::Other { event, raw },
        },
        "Location" => JournalEvent::Location {
            star_system: s(&raw, "StarSystem"),
            body: opt_s(&raw, "Body"),
            docked: raw.get("Docked").and_then(|x| x.as_bool()).unwrap_or(false),
            allegiance: system_allegiance(&raw),
            population: population(&raw),
            factions: factions(&raw),
            system_address: opt_i(&raw, "SystemAddress"),
            star_pos: star_pos(&raw),
            station_name: opt_s(&raw, "StationName"),
            market_id: opt_i(&raw, "MarketID"),
            station: raw.get("Docked").and_then(|x| x.as_bool()).unwrap_or(false).then(|| station_info(&raw)),
            powerplay: system_powerplay(&raw),
        },
        "ApproachBody" => JournalEvent::ApproachBody { star_system: s(&raw, "StarSystem"), body: s(&raw, "Body") },
        "LeaveBody" => JournalEvent::LeaveBody { star_system: s(&raw, "StarSystem"), body: s(&raw, "Body") },
        "FSDJump" | "CarrierJump" => JournalEvent::FsdJump {
            star_system: s(&raw, "StarSystem"),
            jump_dist: raw.get("JumpDist").and_then(|x| x.as_f64()).unwrap_or(0.0),
            allegiance: system_allegiance(&raw),
            population: population(&raw),
            factions: factions(&raw),
            from_carrier: event == "CarrierJump",
            system_address: opt_i(&raw, "SystemAddress"),
            star_pos: star_pos(&raw),
            powerplay: system_powerplay(&raw),
            fuel_used: opt_f(&raw, "FuelUsed"),
            fuel_level: opt_f(&raw, "FuelLevel"),
            boost_used: opt_f(&raw, "BoostUsed").is_some_and(|b| b > 0.0),
        },
        "FSSSignalDiscovered" => {
            let uss_type = opt_s(&raw, "USSType")
                .or_else(|| opt_s(&raw, "SignalType"))
                .or_else(|| opt_s(&raw, "SignalName"))
                .unwrap_or_default();
            let localised = opt_s(&raw, "USSType_Localised").or_else(|| opt_s(&raw, "SignalType_Localised"));
            JournalEvent::FssSignal {
                high_grade: is_high_grade(&uss_type, localised.as_deref()),
                uss_type,
                signal_name: opt_s(&raw, "SignalName"),
                is_station: raw.get("IsStation").and_then(|x| x.as_bool()).unwrap_or(false),
                system_address: opt_i(&raw, "SystemAddress"),
                spawning_faction: opt_s(&raw, "SpawningFaction_Localised").or_else(|| opt_s(&raw, "SpawningFaction")),
                spawning_state: opt_s(&raw, "SpawningState_Localised").or_else(|| opt_s(&raw, "SpawningState")),
            }
        }
        "Docked" => JournalEvent::Docked {
            station_name: s(&raw, "StationName"),
            star_system: s(&raw, "StarSystem"),
            market_id: opt_i(&raw, "MarketID"),
            system_address: opt_i(&raw, "SystemAddress"),
            station: station_info(&raw),
        },
        "Undocked" => JournalEvent::Undocked { station_name: s(&raw, "StationName") },
        "Materials" => JournalEvent::Materials {
            raw: stacks(&raw, "Raw", Some("Raw")),
            manufactured: stacks(&raw, "Manufactured", Some("Manufactured")),
            encoded: stacks(&raw, "Encoded", Some("Encoded")),
        },
        "MaterialCollected" => JournalEvent::MaterialDelta { items: vec![stack_signed(&raw, 1)] },
        "MaterialDiscarded" => JournalEvent::MaterialDelta { items: vec![stack_signed(&raw, -1)] },
        "MaterialTrade" => JournalEvent::MaterialDelta {
            items: vec![
                raw.get("Paid").map(|p| stack_signed(p, -1)).unwrap_or_else(empty_stack),
                raw.get("Received").map(|p| stack_signed(p, 1)).unwrap_or_else(empty_stack),
            ]
            .into_iter()
            .filter(|m| !m.name.is_empty())
            .collect(),
        },
        "EngineerCraft" => JournalEvent::EngineerCraft {
            engineer: s(&raw, "Engineer"),
            engineer_id: opt_i(&raw, "EngineerID"),
            blueprint: s(&raw, "BlueprintName"),
            level: opt_i(&raw, "Level").unwrap_or(0).clamp(0, 5) as u8,
            quality: opt_f(&raw, "Quality").unwrap_or(0.0).clamp(0.0, 1.0),
            slot: opt_s(&raw, "Slot"),
            experimental: opt_s(&raw, "ExperimentalEffect").or_else(|| opt_s(&raw, "ApplyExperimentalEffect")),
            experimental_localised: opt_s(&raw, "ExperimentalEffect_Localised")
                .or_else(|| opt_s(&raw, "ApplyExperimentalEffect_Localised")),
            modifiers: eng_modifiers(&raw),
            preview: raw.get("IsPreview").and_then(|x| x.as_bool()).unwrap_or(false),
            ingredients: stacks_signed(&raw, "Ingredients", -1),
        },
        "EngineerProgress" => JournalEvent::EngineerProgress { engineers: engineer_progress(&raw) },
        "Synthesis" => JournalEvent::MaterialDelta { items: stacks_signed(&raw, "Materials", -1) },
        "ScientificResearch" => JournalEvent::MaterialDelta { items: vec![stack_signed(&raw, -1)] },
        "TechnologyBroker" => JournalEvent::MaterialDelta { items: stacks_signed(&raw, "Materials", -1) },
        "EngineerContribution" => {
            let kind = s(&raw, "Type").to_ascii_lowercase();
            if kind == "materials" || kind == "material" {
                JournalEvent::MaterialDelta { items: vec![stack_signed(&raw, -1)] }
            } else {
                JournalEvent::Other { event, raw }
            }
        }
        _ => match parse_extra(&event, &raw) {
            Some(x) => JournalEvent::Extra(x),
            None => JournalEvent::Other { event, raw },
        },
    };
    Ok(JournalLine { timestamp, name, event: ev, raw_json: line.to_string() })
}

fn modules(v: &serde_json::Value) -> Vec<ModuleSnap> {
    let Some(a) = v.get("Modules").and_then(|x| x.as_array()) else { return vec![] };
    a.iter()
        .map(|m| ModuleSnap {
            slot: s(m, "Slot"),
            item: s(m, "Item").to_ascii_lowercase(),
            on: m.get("On").and_then(|x| x.as_bool()).unwrap_or(true),
            priority: opt_i(m, "Priority").unwrap_or(0).clamp(0, 4) as u8,
            engineering: m.get("Engineering").filter(|e| e.is_object()).map(|e| ModEngineering {
                blueprint: s(e, "BlueprintName"),
                level: opt_i(e, "Level").unwrap_or(0).clamp(0, 5) as u8,
                quality: opt_f(e, "Quality").unwrap_or(0.0).clamp(0.0, 1.0),
                experimental: opt_s(e, "ExperimentalEffect"),
                experimental_localised: opt_s(e, "ExperimentalEffect_Localised"),
                engineer: opt_s(e, "Engineer"),
                modifiers: eng_modifiers(e),
            }),
        })
        .filter(|m| !m.item.is_empty())
        .collect()
}

fn eng_modifiers(v: &serde_json::Value) -> Vec<EngModifier> {
    let Some(a) = v.get("Modifiers").and_then(|x| x.as_array()) else { return vec![] };
    a.iter()
        .filter_map(|m| {
            Some(EngModifier {
                label: opt_s(m, "Label")?,
                value: opt_f(m, "Value")?,
                original_value: opt_f(m, "OriginalValue")?,
            })
        })
        .collect()
}

fn stored_ships(v: &serde_json::Value) -> Vec<StoredShip> {
    ["ShipsHere", "ShipsRemote"]
        .iter()
        .filter_map(|k| v.get(*k).and_then(|x| x.as_array()))
        .flatten()
        .filter_map(|x| {
            let in_transit = x.get("InTransit").and_then(|b| b.as_bool()).unwrap_or(false);
            Some(StoredShip {
                ship_id: opt_i(x, "ShipID")?,
                ship_type: s(x, "ShipType").to_ascii_lowercase(),
                name: opt_s(x, "Name"),
                // In transit: no system yet (not the one the list was written in).
                system: if in_transit { None } else { opt_s(x, "StarSystem").or_else(|| opt_s(v, "StarSystem")) },
                in_transit,
                value: opt_i(x, "Value"),
                market_id: if in_transit { None } else { opt_i(x, "ShipMarketID").or_else(|| opt_i(v, "MarketID")) },
            })
        })
        .collect()
}

fn stored_modules(v: &serde_json::Value) -> Vec<StoredModule> {
    v.get("Items")
        .and_then(|x| x.as_array())
        .into_iter()
        .flatten()
        .filter_map(|x| {
            Some(StoredModule {
                // "$int_hyperdrive_size5_class5_name;" → "int_hyperdrive_size5_class5"
                item: opt_s(x, "Name")?
                    .trim_start_matches('$')
                    .trim_end_matches(';')
                    .trim_end_matches("_name")
                    .to_ascii_lowercase(),
                localised: opt_s(x, "Name_Localised"),
                system: if x.get("InTransit").and_then(|b| b.as_bool()).unwrap_or(false) {
                    None
                } else {
                    opt_s(x, "StarSystem")
                },
                value: opt_i(x, "BuyPrice"),
                blueprint: opt_s(x, "EngineerModifications"),
                level: opt_i(x, "Level").map(|l| l.clamp(0, 5) as u8),
            })
        })
        .collect()
}

fn star_pos(v: &serde_json::Value) -> Option<[f64; 3]> {
    let a = v.get("StarPos")?.as_array()?;
    match a.as_slice() {
        [x, y, z] => Some([x.as_f64()?, y.as_f64()?, z.as_f64()?]),
        _ => None,
    }
}

fn engineer_snap(v: &serde_json::Value) -> EngineerSnap {
    EngineerSnap {
        engineer: s(v, "Engineer"),
        engineer_id: opt_i(v, "EngineerID"),
        progress: s(v, "Progress"),
        rank: opt_i(v, "Rank").map(|r| r.clamp(0, 5) as u8),
        rank_progress: opt_i(v, "RankProgress").map(|r| r.clamp(0, 100) as u8),
    }
}

/// Startup form lists every engineer in `Engineers`; a change writes one at the top level.
fn engineer_progress(v: &serde_json::Value) -> Vec<EngineerSnap> {
    match v.get("Engineers").and_then(|x| x.as_array()) {
        Some(a) => a.iter().map(engineer_snap).filter(|e| !e.engineer.is_empty()).collect(),
        None => {
            let e = engineer_snap(v);
            if e.engineer.is_empty() {
                vec![]
            } else {
                vec![e]
            }
        }
    }
}

fn system_allegiance(v: &serde_json::Value) -> String {
    opt_s(v, "SystemAllegiance").or_else(|| opt_s(v, "Allegiance")).unwrap_or_default()
}

fn population(v: &serde_json::Value) -> i64 {
    opt_i(v, "Population").unwrap_or(0)
}

fn factions(v: &serde_json::Value) -> Vec<FactionSnap> {
    v.get("Factions")
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .map(|f| {
                    let active_states = f
                        .get("ActiveStates")
                        .and_then(|x| x.as_array())
                        .map(|st| st.iter().filter_map(|s| opt_s(s, "State")).collect())
                        .unwrap_or_default();
                    FactionSnap {
                        name: s(f, "Name"),
                        allegiance: s(f, "Allegiance"),
                        state: s(f, "FactionState"),
                        influence: opt_f(f, "Influence").unwrap_or(0.0),
                        active_states,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn is_high_grade(uss_type: &str, localised: Option<&str>) -> bool {
    let blob = format!("{} {}", uss_type, localised.unwrap_or("")).to_ascii_lowercase();
    let compact: String = blob.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    compact.contains("highgradeemission")
}

fn empty_stack() -> MaterialStack {
    MaterialStack { name: String::new(), name_localised: None, count: 0, category: None }
}

fn stack_name(v: &serde_json::Value) -> String {
    opt_s(v, "Name").or_else(|| opt_s(v, "Material")).unwrap_or_default()
}

fn stack_count(v: &serde_json::Value) -> i64 {
    opt_i(v, "Count").or_else(|| opt_i(v, "Quantity")).unwrap_or(0)
}

fn stack_one(v: &serde_json::Value, category: Option<&str>) -> MaterialStack {
    MaterialStack {
        name: stack_name(v),
        name_localised: opt_s(v, "Name_Localised").or_else(|| opt_s(v, "Material_Localised")),
        count: stack_count(v),
        category: opt_s(v, "Category").or_else(|| category.map(str::to_string)),
    }
}

fn stack_signed(v: &serde_json::Value, sign: i64) -> MaterialStack {
    let mut m = stack_one(v, None);
    m.count *= sign;
    m
}

fn stacks(v: &serde_json::Value, key: &str, category: Option<&str>) -> Vec<MaterialStack> {
    v.get(key)
        .and_then(|x| x.as_array())
        .map(|a| a.iter().map(|it| stack_one(it, category)).filter(|m| !m.name.is_empty()).collect())
        .unwrap_or_default()
}

fn stacks_signed(v: &serde_json::Value, key: &str, sign: i64) -> Vec<MaterialStack> {
    stacks(v, key, None)
        .into_iter()
        .map(|mut m| {
            m.count *= sign;
            m
        })
        .collect()
}

/// Cargo.json in the journal folder.
pub fn parse_cargo(bytes: &[u8]) -> Option<CargoSnap> {
    let v: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let count = v.get("Count").and_then(|x| x.as_i64()).unwrap_or(0);
    let mut items = vec![];
    let mut inventory = vec![];
    if let Some(inv) = v.get("Inventory").and_then(|x| x.as_array()) {
        for it in inv {
            let symbol = it.get("Name").and_then(|x| x.as_str()).unwrap_or("item").to_lowercase();
            let name = it.get("Name_Localised").and_then(|x| x.as_str()).map(str::to_string).unwrap_or(symbol.clone());
            let n = it.get("Count").and_then(|x| x.as_i64()).unwrap_or(0);
            items.push((name.clone(), n));
            inventory.push(CargoItem {
                symbol,
                name,
                count: n,
                stolen: it.get("Stolen").and_then(|x| x.as_i64()).unwrap_or(0),
                mission: it.get("MissionID").is_some_and(|m| !m.is_null()),
            });
        }
    }
    items.sort_by_key(|a| std::cmp::Reverse(a.1));
    Some(CargoSnap { count, items, inventory })
}

/// NavRoute.json in the journal folder. First entry is the current system.
pub fn parse_nav_route(bytes: &[u8]) -> Option<NavRouteSnap> {
    let v: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let route = v.get("Route").and_then(|x| x.as_array())?;
    let hops: Vec<RouteHop> = route
        .iter()
        .filter_map(|h| {
            Some(RouteHop {
                system: h.get("StarSystem")?.as_str()?.to_string(),
                star_class: opt_s(h, "StarClass"),
                star_pos: star_pos(h),
            })
        })
        .collect();
    Some(NavRouteSnap { systems: hops.iter().map(|h| h.system.clone()).collect(), hops })
}

fn is_journal_name(n: &str) -> bool {
    n.starts_with("Journal.") && n.ends_with(".log")
}

/// Stamp from `Journal.YYYY-MM-DDTHHMMSS.01.log` or the older `Journal.YYMMDDHHMMSS.01.log`.
fn journal_stamp(name: &str) -> Option<(i64, u32)> {
    let rest = name.strip_prefix("Journal.")?.strip_suffix(".log")?;
    let (stamp, part) = rest.rsplit_once('.')?;
    let part: u32 = part.parse().ok()?;
    let dt = chrono::NaiveDateTime::parse_from_str(stamp, "%Y-%m-%dT%H%M%S")
        .ok()
        .or_else(|| chrono::NaiveDateTime::parse_from_str(stamp, "%y%m%d%H%M%S").ok())?;
    Some((dt.and_utc().timestamp(), part))
}

/// Journal logs in `dir`, oldest first by the date in the file name (both naming schemes).
pub fn journal_logs(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<(i64, u32, std::time::SystemTime, PathBuf)> = std::fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| is_journal_name(&e.file_name().to_string_lossy()))
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let mtime = e.metadata().ok()?.modified().ok()?;
            let (ts, part) =
                journal_stamp(&name).unwrap_or((mtime.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64, 0));
            Some((ts, part, mtime, e.path()))
        })
        .collect();
    v.sort();
    v.into_iter().map(|x| x.3).collect()
}

/// The journal file the game is writing: most recently modified `Journal.*.log`.
pub fn newest_journal(dir: &Path) -> Option<PathBuf> {
    let mut v: Vec<(std::time::SystemTime, PathBuf)> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter(|e| is_journal_name(&e.file_name().to_string_lossy()))
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    v.sort();
    v.pop().map(|x| x.1)
}

/// Parse every complete line in a journal file; skip malformed JSON.
pub fn parse_file(path: &Path) -> Vec<JournalLine> {
    std::fs::read_to_string(path)
        .map(|t| t.lines().filter_map(|l| parse_line(l.trim()).ok()).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_and_unknown_events() {
        let l = parse_line(
            r#"{ "timestamp":"2026-10-10T20:00:00Z", "event":"FSDJump", "StarSystem":"Sol", "JumpDist":8.5 }"#,
        )
        .unwrap();
        assert_eq!(
            l.event,
            JournalEvent::FsdJump {
                star_system: "Sol".into(),
                jump_dist: 8.5,
                allegiance: String::new(),
                population: 0,
                factions: vec![],
                from_carrier: false,
                system_address: None,
                star_pos: None,
                powerplay: None,
                fuel_used: None,
                fuel_level: None,
                boost_used: false,
            }
        );
        let l = parse_line(r#"{ "timestamp":"2026-10-10T20:00:01Z", "event":"SomethingNew", "X":1 }"#).unwrap();
        assert!(matches!(l.event, JournalEvent::Other { ref event, .. } if event == "SomethingNew"));
        assert!(parse_line("{ truncated").is_err());
        assert!(matches!(parse_line(r#"{"event":"X"}"#), Err(JournalError::Missing("timestamp"))));
        let l = parse_line(
            r#"{ "timestamp":"t", "event":"LoadGame", "Commander":"Test", "Ship":"krait_mkii", "Credits":123 }"#,
        )
        .unwrap();
        assert!(matches!(l.event, JournalEvent::LoadGame { credits: Some(123), .. }));
    }

    #[test]
    fn materials_snapshot_and_deltas() {
        let l = parse_line(
            r#"{"timestamp":"t","event":"Materials","Raw":[{"Name":"iron","Count":23}],"Encoded":[{"Name":"emissiondata","Count":1}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::Materials { raw, encoded, .. } => {
                assert_eq!(raw[0].name, "iron");
                assert_eq!(raw[0].count, 23);
                assert_eq!(encoded[0].name, "emissiondata");
            }
            _ => panic!("expected Materials"),
        }
        let l =
            parse_line(r#"{"timestamp":"t","event":"MaterialCollected","Category":"Raw","Name":"sulphur","Count":2}"#)
                .unwrap();
        match l.event {
            JournalEvent::MaterialDelta { items } => {
                assert_eq!(items[0].name, "sulphur");
                assert_eq!(items[0].count, 2);
            }
            _ => panic!("expected delta"),
        }
        let l = parse_line(
            r#"{"timestamp":"t","event":"MaterialTrade","Paid":{"Material":"iron","Quantity":6},"Received":{"Material":"vanadium","Quantity":1}}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::MaterialDelta { items } => {
                assert_eq!(items[0].count, -6);
                assert_eq!(items[1].count, 1);
            }
            _ => panic!("expected trade"),
        }
        let l = parse_line(
            r#"{"timestamp":"t","event":"MissionCompleted","MaterialsReward":[{"Name":"disruptedwakeechoes","Name_Localised":"Atypical Disrupted Wake Echoes","Category":"$MICRORESOURCE_CATEGORY_Encoded;","Count":4}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::Extra(ExtraEvent::MissionCompleted { materials_reward, .. }) => {
                assert_eq!(materials_reward[0].count, 4)
            }
            _ => panic!("expected reward"),
        }
        let l = parse_line(r#"{"timestamp":"t","event":"EngineerContribution","Type":"Commodity","Commodity":"soontillrelics","Quantity":2}"#)
            .unwrap();
        assert!(matches!(l.event, JournalEvent::Other { .. }));
    }

    #[test]
    fn loadout_modules_and_fleet_events() {
        let l = parse_line(
            r#"{"timestamp":"t","event":"Loadout","Ship":"krait_mkii","ShipID":7,"ShipName":"Kestrel","Modules":[{"Slot":"FrameShiftDrive","Item":"Int_Hyperdrive_Size5_Class5","On":true,"Priority":2,"Engineering":{"Engineer":"Felicity Farseer","BlueprintName":"FSD_LongRange","Level":4,"Quality":0.5,"ExperimentalEffect":"special_fsd_heavy","ExperimentalEffect_Localised":"Mass Manager","Modifiers":[{"Label":"Mass","Value":24.5,"OriginalValue":20.0,"LessIsGood":1},{"Label":"WeaponMode","ValueStr":"$WeaponMode_Pulse;","LessIsGood":0},{"Label":"FSDOptimalMass","Value":1472,"OriginalValue":1050.0,"LessIsGood":0}]}},{"Slot":"Armour","Item":"Krait_MkII_Armour_Grade1"}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::Loadout { ship_id, modules, .. } => {
                assert_eq!(ship_id, Some(7));
                assert_eq!(modules[0].item, "int_hyperdrive_size5_class5");
                assert!(modules[0].on);
                assert_eq!(modules[0].priority, 2);
                let e = modules[0].engineering.as_ref().unwrap();
                assert_eq!((e.blueprint.as_str(), e.level, e.quality), ("FSD_LongRange", 4, 0.5));
                assert_eq!(e.experimental.as_deref(), Some("special_fsd_heavy"));
                let m = |label: &str, value, original_value| EngModifier { label: label.into(), value, original_value };
                assert_eq!(e.modifiers, [m("Mass", 24.5, 20.0), m("FSDOptimalMass", 1472.0, 1050.0)]);
                assert!(modules[1].engineering.is_none());
            }
            _ => panic!("expected Loadout"),
        }
        let l = parse_line(
            r#"{"timestamp":"t","event":"StoredShips","StarSystem":"Shinrarta Dezhra","ShipsHere":[{"ShipID":3,"ShipType":"Anaconda","Name":"Big"}],"ShipsRemote":[{"ShipID":4,"ShipType":"asp","StarSystem":"Sol","InTransit":false}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::StoredShips { ships } => {
                assert_eq!(ships.len(), 2);
                assert_eq!(ships[0].system.as_deref(), Some("Shinrarta Dezhra"));
                assert_eq!(ships[0].ship_type, "anaconda");
                assert_eq!(ships[1].system.as_deref(), Some("Sol"));
            }
            _ => panic!("expected StoredShips"),
        }
        let l = parse_line(r#"{"timestamp":"t","event":"StoredShips","StarSystem":"Sol","ShipsRemote":[{"ShipID":5,"ShipType":"asp","InTransit":true}]}"#).unwrap();
        assert!(
            matches!(l.event, JournalEvent::StoredShips { ref ships } if ships[0].system.is_none() && ships[0].in_transit)
        );
        let l = parse_line(r#"{"timestamp":"t","event":"ShipyardTransfer","ShipType":"asp","ShipID":4,"System":"Sol","Distance":10.0}"#).unwrap();
        assert_eq!(l.event, JournalEvent::ShipTransfer { ship_id: 4, ship_type: "asp".into() });
        let l = parse_line(
            r#"{"timestamp":"t","event":"StoredModules","MarketID":1,"StarSystem":"Sol","Items":[{"Name":"$int_hyperdrive_size5_class5_name;","Name_Localised":"FSD","StarSystem":"Sol","BuyPrice":100,"EngineerModifications":"FSD_LongRange","Level":3,"Quality":0.5},{"Name":"$hpt_x;","InTransit":true}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::StoredModules { items } => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0].item, "int_hyperdrive_size5_class5");
                assert_eq!((items[0].system.as_deref(), items[0].level), (Some("Sol"), Some(3)));
                assert_eq!(items[1].system, None);
            }
            _ => panic!("expected StoredModules"),
        }
        let l = parse_line(r#"{"timestamp":"t","event":"ShipyardBuy","ShipType":"python","SellOldShip":"asp","SellShipID":4,"SellPrice":100}"#).unwrap();
        assert_eq!(l.event, JournalEvent::ShipSold { ship_id: 4 });
        let l = parse_line(r#"{"timestamp":"t","event":"SellShipOnRebuy","ShipType":"asp","SellShipId":4}"#).unwrap();
        assert_eq!(l.event, JournalEvent::ShipSold { ship_id: 4 });
    }

    #[test]
    fn positions_and_engineer_events() {
        let l = parse_line(
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Deciat","SystemAddress":6681123623626,"StarPos":[122.625,-0.8125,-47.28125],"JumpDist":9.1}"#,
        )
        .unwrap();
        assert!(
            matches!(l.event, JournalEvent::FsdJump { system_address: Some(6681123623626), star_pos: Some([x, _, _]), .. } if x == 122.625)
        );
        let l = parse_line(
            r#"{"timestamp":"t","event":"EngineerProgress","Engineers":[{"Engineer":"Felicity Farseer","EngineerID":300100,"Progress":"Unlocked","RankProgress":0,"Rank":5},{"Engineer":"Liz Ryder","EngineerID":300080,"Progress":"Invited"}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::EngineerProgress { engineers } => {
                assert_eq!(engineers.len(), 2);
                assert_eq!((engineers[0].progress.as_str(), engineers[0].rank), ("Unlocked", Some(5)));
                assert_eq!(engineers[1].rank, None);
            }
            _ => panic!("expected EngineerProgress"),
        }
        let l = parse_line(r#"{"timestamp":"t","event":"EngineerProgress","Engineer":"Liz Ryder","EngineerID":300080,"Progress":"Unlocked","Rank":1,"RankProgress":20}"#).unwrap();
        assert!(matches!(l.event, JournalEvent::EngineerProgress { ref engineers } if engineers[0].rank == Some(1)));
        let l = parse_line(
            r#"{"timestamp":"t","event":"EngineerCraft","Slot":"FrameShiftDrive","Module":"int_hyperdrive_size5_class5","Ingredients":[{"Name":"arsenic","Count":1}],"Engineer":"Felicity Farseer","EngineerID":300100,"BlueprintID":128673694,"BlueprintName":"FSD_LongRange","Level":5,"Quality":0.4,"Modifiers":[{"Label":"FSDOptimalMass","Value":1396.5,"OriginalValue":1050.0,"LessIsGood":0}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::EngineerCraft {
                engineer,
                blueprint,
                level,
                quality,
                ingredients,
                preview,
                modifiers,
                ..
            } => {
                assert_eq!((engineer.as_str(), blueprint.as_str(), level), ("Felicity Farseer", "FSD_LongRange", 5));
                assert_eq!((quality, preview), (0.4, false));
                assert_eq!((modifiers[0].label.as_str(), modifiers[0].value), ("FSDOptimalMass", 1396.5));
                assert_eq!(ingredients[0].count, -1);
            }
            _ => panic!("expected EngineerCraft"),
        }
    }

    #[test]
    fn journal_names_order_across_schemes() {
        let d = tempfile::tempdir().unwrap();
        // Horizons name "22…" sorts after Odyssey "2026…" as a string; date order is the reverse.
        std::fs::write(d.path().join("Journal.220315120000.01.log"), "{}\n").unwrap();
        std::fs::write(d.path().join("Journal.2026-10-02T120000.01.log"), "{}\n").unwrap();
        let names: Vec<String> = journal_logs(d.path())
            .iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .collect();
        assert_eq!(
            names,
            vec!["Journal.220315120000.01.log".to_string(), "Journal.2026-10-02T120000.01.log".to_string()]
        );
    }

    #[test]
    fn cargo_and_route_files() {
        let c = parse_cargo(
            br#"{ "event":"Cargo", "Count":5, "Inventory":[{"Name":"gold","Name_Localised":"Gold","Count":3,"Stolen":1},{"Name":"painite","Count":2,"Stolen":0}] }"#,
        )
        .unwrap();
        assert_eq!((c.count, c.inventory[0].stolen), (5, 1));
        assert_eq!(c.items[0], ("Gold".into(), 3));
        assert_eq!(
            c.inventory[1],
            CargoItem { symbol: "painite".into(), name: "painite".into(), count: 2, stolen: 0, mission: false }
        );
        let c = parse_cargo(br#"{ "Count":9, "Inventory":[{"Name":"Gold","Count":4,"Stolen":1},{"Name":"drones","Name_Localised":"Limpet","Count":5,"MissionID":42}] }"#).unwrap();
        assert_eq!((c.inventory[0].symbol.as_str(), c.inventory[0].stolen), ("gold", 1));
        assert!(c.inventory[1].mission && c.inventory[1].name == "Limpet");
        let r = parse_nav_route(
            br#"{ "event":"NavRoute", "Route":[{"StarSystem":"Sol"},{"StarSystem":"Barnard's Star"}] }"#,
        )
        .unwrap();
        assert_eq!(r.systems, vec!["Sol", "Barnard's Star"]);
        let r = parse_nav_route(
            br#"{ "event":"NavRoute", "Route":[{"StarSystem":"Sol","SystemAddress":10477373803,"StarPos":[0.0,0.0,0.0],"StarClass":"G"},{"StarSystem":"Barnard's Star","StarPos":[-3.03125,1.375,4.9375],"StarClass":"M"}] }"#,
        )
        .unwrap();
        assert_eq!(
            r.hops[1],
            RouteHop {
                system: "Barnard's Star".into(),
                star_class: Some("M".into()),
                star_pos: Some([-3.03125, 1.375, 4.9375])
            }
        );
        let j = parse_line(
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Sol","JumpDist":8.5,"FuelUsed":1.25,"FuelLevel":30.5}"#,
        )
        .unwrap();
        assert!(matches!(
            j.event,
            JournalEvent::FsdJump { fuel_used: Some(1.25), fuel_level: Some(30.5), boost_used: false, .. }
        ));
        let j = parse_line(
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Sol","JumpDist":80.0,"FuelUsed":1.25,"BoostUsed":4}"#,
        )
        .unwrap();
        assert!(matches!(j.event, JournalEvent::FsdJump { boost_used: true, .. }));
    }

    #[test]
    fn jump_carries_factions_and_hge_signal() {
        let l = parse_line(
            r#"{ "timestamp":"t", "event":"FSDJump", "StarSystem":"Shinrarta Dezhra", "SystemAllegiance":"Federation", "Population":85262351, "JumpDist":12.1, "Factions":[{"Name":"The Pilots Federation","Allegiance":"Federation","FactionState":"None","Influence":0.61,"ActiveStates":[]},{"Name":"Empire Cutters","Allegiance":"Empire","FactionState":"Boom","Influence":0.12,"ActiveStates":[{"State":"Boom"}]}] }"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::FsdJump { star_system, allegiance, population, factions, .. } => {
                assert_eq!(star_system, "Shinrarta Dezhra");
                assert_eq!(allegiance, "Federation");
                assert_eq!(population, 85262351);
                assert_eq!(factions[0].allegiance, "Federation");
                assert_eq!(factions[1].active_states, vec!["Boom"]);
            }
            _ => panic!("expected FSDJump"),
        }
        let l = parse_line(
            r#"{ "timestamp":"t", "event":"FSSSignalDiscovered", "USSType":"$USS_Type_HighGradeEmissions;", "USSType_Localised":"High Grade Emissions" }"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::FssSignal { high_grade, .. } => assert!(high_grade),
            _ => panic!("expected FSS"),
        }
        // The game's own shape: a salvage type localised as High Grade Emissions, with who spawned it.
        let l = parse_line(
            r#"{ "timestamp":"t", "event":"FSSSignalDiscovered", "SystemAddress":1, "SignalName":"$USS_HighGradeEmission;", "SignalName_Localised":"Unidentified signal source", "USSType":"$USS_Type_VeryValuableSalvage;", "USSType_Localised":"High grade emissions", "SpawningState":"$FactionState_Boom_desc;", "SpawningState_Localised":"Boom", "SpawningFaction":"Empire League", "ThreatLevel":0, "TimeRemaining":1800.5 }"#,
        )
        .unwrap();
        assert!(matches!(
            l.event,
            JournalEvent::FssSignal { high_grade: true, ref spawning_faction, ref spawning_state, .. }
                if spawning_faction.as_deref() == Some("Empire League") && spawning_state.as_deref() == Some("Boom")
        ));
        let l = parse_line(
            r#"{ "timestamp":"t", "event":"CarrierJump", "StarSystem":"Alioth", "SystemAllegiance":"Alliance", "Population":1000 }"#,
        )
        .unwrap();
        assert!(
            matches!(l.event, JournalEvent::FsdJump { from_carrier: true, ref star_system, .. } if star_system == "Alioth")
        );
    }

    #[test]
    fn docked_keeps_system_address() {
        let l = parse_line(
            r#"{"timestamp":"t","event":"Docked","StationName":"Hutton Orbital","StarSystem":"Alpha Centauri","SystemAddress":1,"MarketID":2}"#,
        )
        .unwrap();
        assert_eq!(
            l.event,
            JournalEvent::Docked {
                station_name: "Hutton Orbital".into(),
                star_system: "Alpha Centauri".into(),
                market_id: Some(2),
                system_address: Some(1),
                station: StationInfo::default(),
            }
        );
    }

    #[test]
    fn docked_reads_what_the_station_offers() {
        let l = parse_line(
            r#"{"timestamp":"t","event":"Docked","StationName":"Test Orbital","StationType":"Orbis","StarSystem":"Test System","SystemAddress":1,"MarketID":2,
            "StationFaction":{"Name":"Test Holdings","FactionState":"Boom"},"StationGovernment":"$government_Corporate;","StationGovernment_Localised":"Corporate","StationAllegiance":"Federation",
            "StationServices":["dock","outfitting","refuel"],
            "StationEconomies":[{"Name":"$economy_HighTech;","Name_Localised":"High Tech","Proportion":0.75},{"Name":"$economy_Refinery;","Proportion":0.25}],
            "DistFromStarLS":512.5,"LandingPads":{"Small":4,"Medium":6,"Large":3}}"#,
        )
        .unwrap();
        let JournalEvent::Docked { station, .. } = l.event else { panic!("not Docked") };
        assert_eq!(station.station_type.as_deref(), Some("Orbis"));
        assert_eq!(station.services, ["dock", "outfitting", "refuel"]);
        assert_eq!(station.economies.len(), 2);
        assert_eq!(station.economies[0].name_localised.as_deref(), Some("High Tech"));
        assert_eq!(station.economies[1].proportion, 0.25);
        assert_eq!(station.faction, Some(StationFaction { name: "Test Holdings".into(), state: Some("Boom".into()) }));
        assert_eq!(station.government.as_deref(), Some("Corporate"));
        assert_eq!(station.landing_pads, Some(LandingPads { small: 4, medium: 6, large: 3 }));
        assert_eq!(station.dist_from_star_ls, Some(512.5));
    }
}
