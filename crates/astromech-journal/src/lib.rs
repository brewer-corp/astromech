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
    },
    Location {
        star_system: String,
        body: Option<String>,
        docked: bool,
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
        },
        "Location" => JournalEvent::Location {
            star_system: s(&raw, "StarSystem"),
            body: opt_s(&raw, "Body"),
            docked: raw.get("Docked").and_then(|x| x.as_bool()).unwrap_or(false),
        },
        "ApproachBody" => JournalEvent::ApproachBody { star_system: s(&raw, "StarSystem"), body: s(&raw, "Body") },
        "LeaveBody" => JournalEvent::LeaveBody { star_system: s(&raw, "StarSystem"), body: s(&raw, "Body") },
        "FSDJump" => JournalEvent::FsdJump {
            star_system: s(&raw, "StarSystem"),
            jump_dist: raw.get("JumpDist").and_then(|x| x.as_f64()).unwrap_or(0.0),
        },
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

/// Journal logs in `dir`, oldest first (file name, then modified time).
pub fn journal_logs(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<(String, std::time::SystemTime, PathBuf)> = std::fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| is_journal_name(&e.file_name().to_string_lossy()))
        .filter_map(|e| {
            Some((e.file_name().to_string_lossy().into_owned(), e.metadata().ok()?.modified().ok()?, e.path()))
        })
        .collect();
    v.sort();
    v.into_iter().map(|x| x.2).collect()
}

/// The newest `Journal.*.log` in `dir` (by file name, which sorts by date in both naming schemes
/// used since 2015 only within a scheme; the modified time breaks ties).
pub fn newest_journal(dir: &Path) -> Option<PathBuf> {
    journal_logs(dir).pop()
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
        assert_eq!(l.event, JournalEvent::FsdJump { star_system: "Sol".into(), jump_dist: 8.5 });
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
}
