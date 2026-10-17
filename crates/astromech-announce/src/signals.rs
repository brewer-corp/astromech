//! FSS signal callouts (`signals`): on `FSSSignalDiscovered`, one line per kind of signal the
//! HUD doesn't announce on its own: Thargoid signals, conflict zones (with intensity), Powerplay
//! signals and megaships. Each line is said once per system, so three conflict zones of the
//! same intensity give one line. Stations and carriers (`IsStation`) stay quiet, and High Grade
//! Emissions have their own rule ([`crate::hge`]).
//!
//! Journal values (from public journal parsers, not from our own logs):
//! - `SignalType` "Combat" with `SignalName` `$Warzone_PointRace_{Low,Med,High}:#index=1;`
//!   (conflict zone), "Megaship" and "Titan" (SrvSurvey `JournalEntry.cs` samples; ed-journal's
//!   `SignalType` list).
//! - `$Warzone_TG`, `$Warzone_TG_{Low,Med,High,VeryHigh}` (Thargoid conflict zone) and
//!   `$Warzone_Powerplay_Med` (Powerplay conflict zone) from EDDI `SignalSource.cs`.
//! - `USSType` `$USS_Type_NonHuman;` (Thargoid) and `$USS_Type_Power*;`, e.g.
//!   `$USS_Type_PowerEmissions;` (Powerplay), also EDDI.
//!
//! Unverified: that "Titan" is only ever a Thargoid Titan, and the Powerplay conflict zone
//! intensities other than "Med".

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::JournalEvent;
use std::sync::Mutex;
use std::time::Duration;

/// The line for a signal, or None when it isn't one we call out.
pub fn line(signal_type: &str, signal_name: Option<&str>) -> Option<String> {
    let name = signal_name.unwrap_or("").trim_start_matches('$').to_ascii_lowercase();
    let intensity = |prefix: &str| {
        let level = name.strip_prefix(prefix)?.trim_start_matches('_');
        let level = level.split([':', ';']).next().unwrap_or("");
        match level {
            "low" => Some("low"),
            "med" | "medium" => Some("medium"),
            "high" => Some("high"),
            "veryhigh" => Some("very high"),
            _ => None,
        }
    };
    let with = |what: &str, level: Option<&str>| match level {
        Some(l) => format!("{what}, {l} intensity."),
        None => format!("{what}."),
    };
    if name.starts_with("warzone_tg") {
        Some(with("Thargoid conflict zone", intensity("warzone_tg")))
    } else if name.starts_with("warzone_powerplay") {
        Some(with("Powerplay conflict zone", intensity("warzone_powerplay")))
    } else if name.starts_with("warzone_") {
        Some(with("Conflict zone", intensity("warzone_pointrace")))
    } else if signal_type.eq_ignore_ascii_case("$USS_Type_NonHuman;") {
        Some("Thargoid signal.".into())
    } else if signal_type.to_ascii_lowercase().starts_with("$uss_type_power") {
        Some("Powerplay signal.".into())
    } else if signal_type == "Titan" {
        Some("Thargoid Titan in system.".into())
    } else if signal_type == "Megaship" {
        Some("Megaship in system.".into())
    } else {
        None
    }
}

/// The lines said so far in one system.
#[derive(Default)]
struct Said {
    system: Option<i64>,
    lines: Vec<String>,
}

#[derive(Default)]
pub struct Signals {
    said: Mutex<Said>,
}

impl Rule for Signals {
    fn id(&self) -> &'static str {
        "signals"
    }
    /// Once per system is the rule's own; a honk's signals each get their say.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, _state: &GameState) -> Option<Callout> {
        let Input::Journal(l) = input else { return None };
        let mut said = self.said.lock().unwrap();
        let text = match &l.event {
            JournalEvent::FsdJump { .. } => {
                *said = Said::default();
                return None;
            }
            JournalEvent::FssSignal { uss_type, signal_name, is_station: false, system_address, .. } => {
                if system_address.is_some() && said.system != *system_address {
                    *said = Said { system: *system_address, lines: vec![] };
                }
                line(uss_type, signal_name.as_deref())?
            }
            _ => return None,
        };
        if said.lines.contains(&text) {
            return None;
        }
        said.lines.push(text.clone());
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH })
    }
}
