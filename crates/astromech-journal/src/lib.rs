//! # astromech-journal
//!
//! Reads the game's journal (`Journal.*.log`, one JSON object per line) and companion files.
//! Seed of the ingest pipeline (ARCHITECTURE.md §5.1, S6 verdict: own parser, lenient on
//! unknown events, `ed-journals` corpus only for tests).
//!
//! Next slice: incremental tailing of the newest journal file, file rotation, and typed
//! events for the ~40 events the MVP needs; everything else stays as [`JournalEvent::Other`].

use serde::Deserialize;
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
        ship_id: Option<i64>,
        modules: Vec<ModuleSnap>,
    },
    /// Ships at this station and elsewhere (`StoredShips`, written when the shipyard opens).
    /// The ship being flown is not listed.
    StoredShips {
        ships: Vec<StoredShip>,
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
    },
    /// FSS signal in the current system (`FSSSignalDiscovered`).
    FssSignal {
        uss_type: String,
        high_grade: bool,
    },
    Docked {
        station_name: String,
        star_system: String,
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
    /// Signed counts from collect / discard / trade / craft / synthesis / rewards.
    MaterialDelta {
        items: Vec<MaterialStack>,
    },
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
}

/// One entry of `StoredShips` (`ShipsHere` and `ShipsRemote`).
#[derive(Debug, Clone, PartialEq)]
pub struct StoredShip {
    pub ship_id: i64,
    pub ship_type: String,
    pub name: Option<String>,
    pub system: Option<String>,
}

/// One minor faction on `Location` / `FSDJump` / `CarrierJump`.
#[derive(Debug, Clone, PartialEq)]
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
    pub event: JournalEvent,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CargoSnap {
    pub count: i64,
    pub items: Vec<(String, i64)>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NavRouteSnap {
    pub systems: Vec<String>,
}

#[derive(Deserialize)]
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
    let head: Head = serde_json::from_value(raw.clone())?;
    let event = head.event.ok_or(JournalError::Missing("event"))?;
    let timestamp = head.timestamp.ok_or(JournalError::Missing("timestamp"))?;
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
            ship_id: opt_i(&raw, "ShipID"),
            modules: modules(&raw),
        },
        "StoredShips" => JournalEvent::StoredShips { ships: stored_ships(&raw) },
        "ShipyardSell" => match opt_i(&raw, "SellShipID") {
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
        },
        "FSSSignalDiscovered" => {
            let uss_type = opt_s(&raw, "USSType")
                .or_else(|| opt_s(&raw, "SignalType"))
                .or_else(|| opt_s(&raw, "SignalName"))
                .unwrap_or_default();
            let localised = opt_s(&raw, "USSType_Localised").or_else(|| opt_s(&raw, "SignalType_Localised"));
            JournalEvent::FssSignal { high_grade: is_high_grade(&uss_type, localised.as_deref()), uss_type }
        }
        "Docked" => JournalEvent::Docked { station_name: s(&raw, "StationName"), star_system: s(&raw, "StarSystem") },
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
        "EngineerCraft" => JournalEvent::MaterialDelta { items: stacks_signed(&raw, "Ingredients", -1) },
        "Synthesis" => JournalEvent::MaterialDelta { items: stacks_signed(&raw, "Materials", -1) },
        "ScientificResearch" => JournalEvent::MaterialDelta { items: vec![stack_signed(&raw, -1)] },
        "TechnologyBroker" => JournalEvent::MaterialDelta { items: stacks_signed(&raw, "Materials", -1) },
        "MissionCompleted" => JournalEvent::MaterialDelta { items: stacks(&raw, "MaterialsReward", None) },
        "EngineerContribution" => {
            let kind = s(&raw, "Type").to_ascii_lowercase();
            if kind == "materials" || kind == "material" {
                JournalEvent::MaterialDelta { items: vec![stack_signed(&raw, -1)] }
            } else {
                JournalEvent::Other { event, raw }
            }
        }
        _ => JournalEvent::Other { event, raw },
    };
    Ok(JournalLine { timestamp, event: ev })
}

fn modules(v: &serde_json::Value) -> Vec<ModuleSnap> {
    let Some(a) = v.get("Modules").and_then(|x| x.as_array()) else { return vec![] };
    a.iter()
        .map(|m| ModuleSnap {
            slot: s(m, "Slot"),
            item: s(m, "Item").to_ascii_lowercase(),
            engineering: m.get("Engineering").filter(|e| e.is_object()).map(|e| ModEngineering {
                blueprint: s(e, "BlueprintName"),
                level: opt_i(e, "Level").unwrap_or(0).clamp(0, 5) as u8,
                quality: opt_f(e, "Quality").unwrap_or(0.0).clamp(0.0, 1.0),
                experimental: opt_s(e, "ExperimentalEffect"),
                experimental_localised: opt_s(e, "ExperimentalEffect_Localised"),
                engineer: opt_s(e, "Engineer"),
            }),
        })
        .filter(|m| !m.item.is_empty())
        .collect()
}

fn stored_ships(v: &serde_json::Value) -> Vec<StoredShip> {
    ["ShipsHere", "ShipsRemote"]
        .iter()
        .filter_map(|k| v.get(*k).and_then(|x| x.as_array()))
        .flatten()
        .filter_map(|x| {
            Some(StoredShip {
                ship_id: opt_i(x, "ShipID")?,
                ship_type: s(x, "ShipType").to_ascii_lowercase(),
                name: opt_s(x, "Name"),
                system: opt_s(x, "StarSystem").or_else(|| opt_s(v, "StarSystem")),
            })
        })
        .collect()
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
    if let Some(inv) = v.get("Inventory").and_then(|x| x.as_array()) {
        for it in inv {
            let name = it
                .get("Name_Localised")
                .and_then(|x| x.as_str())
                .or_else(|| it.get("Name").and_then(|x| x.as_str()))
                .unwrap_or("item");
            let n = it.get("Count").and_then(|x| x.as_i64()).unwrap_or(0);
            items.push((name.to_string(), n));
        }
    }
    items.sort_by_key(|a| std::cmp::Reverse(a.1));
    Some(CargoSnap { count, items })
}

/// NavRoute.json in the journal folder. First entry is the current system.
pub fn parse_nav_route(bytes: &[u8]) -> Option<NavRouteSnap> {
    let v: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let route = v.get("Route").and_then(|x| x.as_array())?;
    let systems =
        route.iter().filter_map(|s| s.get("StarSystem").and_then(|x| x.as_str()).map(str::to_string)).collect();
    Some(NavRouteSnap { systems })
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
            JournalEvent::MaterialDelta { items } => assert_eq!(items[0].count, 4),
            _ => panic!("expected reward"),
        }
        let l = parse_line(r#"{"timestamp":"t","event":"EngineerContribution","Type":"Commodity","Commodity":"soontillrelics","Quantity":2}"#)
            .unwrap();
        assert!(matches!(l.event, JournalEvent::Other { .. }));
    }

    #[test]
    fn loadout_modules_and_fleet_events() {
        let l = parse_line(
            r#"{"timestamp":"t","event":"Loadout","Ship":"krait_mkii","ShipID":7,"ShipName":"Kestrel","Modules":[{"Slot":"FrameShiftDrive","Item":"Int_Hyperdrive_Size5_Class5","On":true,"Engineering":{"Engineer":"Felicity Farseer","BlueprintName":"FSD_LongRange","Level":4,"Quality":0.5,"ExperimentalEffect":"special_fsd_heavy","ExperimentalEffect_Localised":"Mass Manager","Modifiers":[]}},{"Slot":"Armour","Item":"Krait_MkII_Armour_Grade1"}]}"#,
        )
        .unwrap();
        match l.event {
            JournalEvent::Loadout { ship_id, modules, .. } => {
                assert_eq!(ship_id, Some(7));
                assert_eq!(modules[0].item, "int_hyperdrive_size5_class5");
                let e = modules[0].engineering.as_ref().unwrap();
                assert_eq!((e.blueprint.as_str(), e.level, e.quality), ("FSD_LongRange", 4, 0.5));
                assert_eq!(e.experimental.as_deref(), Some("special_fsd_heavy"));
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
        let l = parse_line(r#"{"timestamp":"t","event":"SellShipOnRebuy","ShipType":"asp","SellShipId":4}"#).unwrap();
        assert_eq!(l.event, JournalEvent::ShipSold { ship_id: 4 });
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
            br#"{ "event":"Cargo", "Count":5, "Inventory":[{"Name":"gold","Name_Localised":"Gold","Count":3},{"Name":"painite","Count":2}] }"#,
        )
        .unwrap();
        assert_eq!(c.count, 5);
        assert_eq!(c.items[0], ("Gold".into(), 3));
        let r = parse_nav_route(
            br#"{ "event":"NavRoute", "Route":[{"StarSystem":"Sol"},{"StarSystem":"Barnard's Star"}] }"#,
        )
        .unwrap();
        assert_eq!(r.systems, vec!["Sol", "Barnard's Star"]);
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
        let l = parse_line(
            r#"{ "timestamp":"t", "event":"CarrierJump", "StarSystem":"Alioth", "SystemAllegiance":"Alliance", "Population":1000 }"#,
        )
        .unwrap();
        assert!(
            matches!(l.event, JournalEvent::FsdJump { from_carrier: true, ref star_system, .. } if star_system == "Alioth")
        );
    }
}
