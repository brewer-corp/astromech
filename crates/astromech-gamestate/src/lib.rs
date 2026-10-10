//! # astromech-gamestate
//!
//! The reducer: folds journal events and Status.json snapshots into one [`GameState`]
//! (ARCHITECTURE.md §5.2). Pure and synchronous so it can be replay-tested.

use astromech_journal::JournalEvent;
use astromech_status::{StatusSnap, FLAG_DOCKED, FLAG_GEAR, FLAG_SUPERCRUISE};
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct GameState {
    pub commander: Option<String>,
    pub ship: Option<String>,
    pub system: Option<String>,
    pub station: Option<String>,
    pub docked: bool,
    pub gear_down: bool,
    pub supercruise: bool,
    pub status_flags: u64,
    /// False until a Status.json with Flags has been read (main menu / game not running).
    pub status_seen: bool,
    pub gui_focus: Option<u64>,
    /// Journal file currently followed.
    pub journal_file: Option<String>,
}

impl GameState {
    /// Flags for planning, `None` if no Status.json has been seen.
    pub fn flags(&self) -> Option<u64> {
        self.status_seen.then_some(self.status_flags)
    }

    pub fn apply_journal(&mut self, ev: &JournalEvent) {
        match ev {
            JournalEvent::LoadGame { commander, ship } => {
                self.commander = Some(commander.clone());
                self.ship = ship.clone();
            }
            JournalEvent::Location { star_system, docked } => {
                self.system = Some(star_system.clone());
                self.docked = *docked;
            }
            JournalEvent::FsdJump { star_system, .. } => {
                self.system = Some(star_system.clone());
                self.station = None;
            }
            JournalEvent::Docked { station_name, star_system } => {
                self.station = Some(station_name.clone());
                self.system = Some(star_system.clone());
                self.docked = true;
            }
            JournalEvent::Undocked { .. } => self.docked = false,
            _ => {}
        }
    }
    pub fn apply_status(&mut self, s: &StatusSnap) {
        self.status_flags = s.flags;
        self.status_seen = true;
        self.gui_focus = s.gui_focus;
        self.gear_down = s.flags & FLAG_GEAR != 0;
        self.supercruise = s.flags & FLAG_SUPERCRUISE != 0;
        self.docked = s.flags & FLAG_DOCKED != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn folds_events() {
        let mut g = GameState::default();
        for l in [
            r#"{"timestamp":"t","event":"LoadGame","Commander":"Test","Ship":"krait_mkii"}"#,
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Shinrarta Dezhra","JumpDist":12.0}"#,
            r#"{"timestamp":"t","event":"Docked","StationName":"Jameson Memorial","StarSystem":"Shinrarta Dezhra"}"#,
        ] {
            g.apply_journal(&astromech_journal::parse_line(l).unwrap().event);
        }
        assert_eq!(g.station.as_deref(), Some("Jameson Memorial"));
        assert!(g.docked);
        g.apply_status(&StatusSnap { flags: FLAG_GEAR, gui_focus: None });
        assert!(g.gear_down && !g.docked);
    }
}
