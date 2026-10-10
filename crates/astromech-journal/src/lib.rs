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
    FileHeader { game_version: String, build: String, odyssey: bool },
    LoadGame { commander: String, ship: Option<String> },
    Location { star_system: String, docked: bool },
    FsdJump { star_system: String, jump_dist: f64 },
    Docked { station_name: String, star_system: String },
    Undocked { station_name: String },
    Other { event: String, raw: serde_json::Value },
}

#[derive(Debug, Clone, PartialEq)]
pub struct JournalLine {
    pub timestamp: String,
    pub event: JournalEvent,
}

#[derive(Deserialize)]
struct Head {
    timestamp: Option<String>,
    event: Option<String>,
}

fn s(v: &serde_json::Value, k: &str) -> String {
    v.get(k).and_then(|x| x.as_str()).unwrap_or_default().to_string()
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
            ship: raw.get("Ship").and_then(|x| x.as_str()).map(str::to_string),
        },
        "Location" => JournalEvent::Location {
            star_system: s(&raw, "StarSystem"),
            docked: raw.get("Docked").and_then(|x| x.as_bool()).unwrap_or(false),
        },
        "FSDJump" => JournalEvent::FsdJump {
            star_system: s(&raw, "StarSystem"),
            jump_dist: raw.get("JumpDist").and_then(|x| x.as_f64()).unwrap_or(0.0),
        },
        "Docked" => JournalEvent::Docked { station_name: s(&raw, "StationName"), star_system: s(&raw, "StarSystem") },
        "Undocked" => JournalEvent::Undocked { station_name: s(&raw, "StationName") },
        _ => JournalEvent::Other { event, raw },
    };
    Ok(JournalLine { timestamp, event: ev })
}

/// The newest `Journal.*.log` in `dir` (by file name, which sorts by date in both naming schemes
/// used since 2015 only within a scheme; the modified time breaks ties).
pub fn newest_journal(dir: &Path) -> Option<PathBuf> {
    let mut v: Vec<(std::time::SystemTime, PathBuf)> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.starts_with("Journal.") && n.ends_with(".log")
        })
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    v.sort();
    v.pop().map(|x| x.1)
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
    }
}
