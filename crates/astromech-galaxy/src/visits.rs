//! Faction states seen on arrival. Every `FSDJump`, `CarrierJump` and `Location` carries the system's factions
//! and their active states, which is newer than Spansh's copy for any system the commander has
//! just visited. [`crate::Galaxy::record_visits`] keeps the newest per system, and the faction
//! lookups prefer it over an older Spansh answer.

use astromech_data::spansh::Faction;
use astromech_journal::{FactionSnap, JournalEvent, JournalLine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// One system's factions as the journal last saw them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Visit {
    pub id64: i64,
    pub system: String,
    /// Journal timestamp (RFC 3339).
    pub seen: String,
    /// In Spansh's spelling ("Civil War", not "CivilWar").
    pub factions: Vec<Faction>,
}

/// "CivilWar" → "Civil War"; "None" or empty → `None`.
fn spansh_state(s: &str) -> Option<String> {
    if s.is_empty() || s.eq_ignore_ascii_case("none") {
        return None;
    }
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_ascii_uppercase() && !out.ends_with(' ') {
            out.push(' ');
        }
        out.push(c);
    }
    Some(out)
}

pub fn faction(f: &FactionSnap) -> Faction {
    Faction {
        name: f.name.clone(),
        allegiance: f.allegiance.clone(),
        state: spansh_state(&f.state),
        active_states: f.active_states.iter().filter_map(|s| spansh_state(s)).collect(),
        influence: f.influence,
    }
}

/// The newest visit per system in `lines` (`FSDJump`, `CarrierJump` and `Location` arrivals; jumps without a `SystemAddress` or factions are
/// skipped: an empty faction list is no news).
pub fn visits_from_lines<'a>(lines: impl IntoIterator<Item = &'a JournalLine>) -> Vec<Visit> {
    let mut by_id: HashMap<i64, Visit> = HashMap::new();
    for l in lines {
        let (system, id64, factions) = match &l.event {
            JournalEvent::FsdJump { star_system, system_address, factions, .. }
            | JournalEvent::Location { star_system, system_address, factions, .. } => {
                (star_system, system_address, factions)
            }
            _ => continue,
        };
        let (Some(id64), false) = (*id64, factions.is_empty()) else { continue };
        let v = Visit {
            id64,
            system: system.clone(),
            seen: l.timestamp.clone(),
            factions: factions.iter().map(faction).collect(),
        };
        match by_id.get(&id64) {
            Some(old) if old.seen >= v.seen => {}
            _ => {
                by_id.insert(id64, v);
            }
        }
    }
    let mut out: Vec<Visit> = by_id.into_values().collect();
    out.sort_by(|a, b| b.seen.cmp(&a.seen));
    out
}

/// How many of the newest journals the lookups read visits from.
pub const RECENT_JOURNALS: usize = 20;

/// Visits in the newest `max_files` journals in `dir`.
pub fn visits_from_journal_dir(dir: &Path, max_files: usize) -> Vec<Visit> {
    let logs = astromech_journal::journal_logs(dir);
    let mut lines = vec![];
    for p in logs.iter().rev().take(max_files) {
        let Ok(text) = std::fs::read_to_string(p) else { continue };
        for line in text.lines().filter(|l| crate::POSITION_EVENTS.iter().any(|e| l.contains(e))) {
            if let Ok(l) = astromech_journal::parse_line(line.trim()) {
                lines.push(l);
            }
        }
    }
    visits_from_lines(&lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn states_take_spansh_spelling() {
        assert_eq!(spansh_state("CivilWar").as_deref(), Some("Civil War"));
        assert_eq!(spansh_state("Boom").as_deref(), Some("Boom"));
        assert_eq!(spansh_state("InfrastructureFailure").as_deref(), Some("Infrastructure Failure"));
        assert_eq!(spansh_state("None"), None);
    }

    #[test]
    fn newest_visit_per_system_wins() {
        let jump = |t: &str, state: &str| {
            astromech_journal::parse_line(&format!(
                r#"{{"timestamp":"{t}","event":"FSDJump","StarSystem":"Tau Ceti","SystemAddress":7,"StarPos":[1,2,3],"Factions":[{{"Name":"Tau Ceti Gang","Allegiance":"Independent","FactionState":"{state}","Influence":0.4,"ActiveStates":[{{"State":"{state}"}}]}}]}}"#
            ))
            .unwrap()
        };
        let lines = [
            jump("2026-10-16T10:00:00Z", "Famine"),
            jump("2026-10-16T12:00:00Z", "CivilWar"),
            jump("2026-10-15T00:00:00Z", "Boom"),
        ];
        let v = visits_from_lines(&lines);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].seen, "2026-10-16T12:00:00Z");
        assert_eq!(v[0].factions[0].active_states, ["Civil War"]);
        assert_eq!(v[0].factions[0].state.as_deref(), Some("Civil War"));
    }

    /// The reader filters with `POSITION_EVENTS`, the same list a page watches: a carrier jump,
    /// which carries the system's factions like any arrival, is a visit.
    #[test]
    fn a_carrier_jump_is_a_visit() {
        let d = tempfile::tempdir().unwrap();
        let line = r#"{"timestamp":"2026-10-16T12:00:00Z","event":"CarrierJump","StarSystem":"Tau Ceti","SystemAddress":7,"StarPos":[1,2,3],"Factions":[{"Name":"Tau Ceti Gang","Allegiance":"Independent","FactionState":"Boom","Influence":0.4,"ActiveStates":[{"State":"Boom"}]}]}"#;
        std::fs::write(d.path().join("Journal.2026-10-16T110000.01.log"), format!("{line}\n")).unwrap();
        let v = visits_from_journal_dir(d.path(), 5);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].factions[0].state.as_deref(), Some("Boom"));
    }
}
