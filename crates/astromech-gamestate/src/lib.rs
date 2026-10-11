//! # astromech-gamestate
//!
//! The reducer: folds journal events and Status.json snapshots into one [`GameState`]
//! (ARCHITECTURE.md §5.2). Pure and synchronous so it can be replay-tested.

use astromech_journal::{CargoSnap, JournalEvent, NavRouteSnap};
use astromech_materials::Ledger;
use astromech_status::{StatusSnap, FLAG_DOCKED, FLAG_GEAR, FLAG_SUPERCRUISE};
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct GameState {
    pub commander: Option<String>,
    pub ship: Option<String>,
    pub ship_name: Option<String>,
    pub ship_ident: Option<String>,
    pub system: Option<String>,
    pub body: Option<String>,
    pub station: Option<String>,
    pub docked: bool,
    pub gear_down: bool,
    pub supercruise: bool,
    pub status_flags: u64,
    /// False until a Status.json with Flags has been read (main menu / game not running).
    pub status_seen: bool,
    pub gui_focus: Option<u64>,
    pub fuel_main: Option<f64>,
    pub fuel_reservoir: Option<f64>,
    pub fuel_capacity_main: Option<f64>,
    pub cargo_tonnes: Option<f64>,
    pub cargo_capacity: Option<i64>,
    pub cargo_items: Vec<(String, i64)>,
    pub legal_state: Option<String>,
    pub credits: Option<i64>,
    pub rebuy: Option<i64>,
    pub hull_health: Option<f64>,
    pub max_jump_range: Option<f64>,
    pub route: Vec<String>,
    /// Journal file currently followed.
    pub journal_file: Option<String>,
    /// Ship engineering materials (login snapshot + later deltas).
    pub materials: Ledger,
}

impl GameState {
    /// Flags for planning, `None` if no Status.json has been seen.
    pub fn flags(&self) -> Option<u64> {
        self.status_seen.then_some(self.status_flags)
    }

    pub fn apply_journal(&mut self, ev: &JournalEvent) {
        match ev {
            JournalEvent::LoadGame { commander, ship, ship_name, ship_ident, credits } => {
                self.commander = Some(commander.clone());
                self.ship = ship.clone();
                if ship_name.is_some() {
                    self.ship_name = ship_name.clone();
                }
                if ship_ident.is_some() {
                    self.ship_ident = ship_ident.clone();
                }
                self.credits = *credits;
            }
            JournalEvent::Loadout {
                ship,
                ship_name,
                ship_ident,
                hull_health,
                cargo_capacity,
                max_jump_range,
                fuel_capacity_main,
                rebuy,
            } => {
                if ship.is_some() {
                    self.ship = ship.clone();
                }
                self.ship_name = ship_name.clone();
                self.ship_ident = ship_ident.clone();
                self.hull_health = *hull_health;
                self.cargo_capacity = *cargo_capacity;
                self.max_jump_range = *max_jump_range;
                self.fuel_capacity_main = *fuel_capacity_main;
                self.rebuy = *rebuy;
            }
            JournalEvent::Location { star_system, body, docked, .. } => {
                self.system = Some(star_system.clone());
                self.body = body.clone();
                self.docked = *docked;
            }
            JournalEvent::ApproachBody { star_system, body } => {
                self.system = Some(star_system.clone());
                self.body = Some(body.clone());
            }
            JournalEvent::LeaveBody { .. } => self.body = None,
            JournalEvent::FsdJump { star_system, from_carrier, .. } => {
                self.system = Some(star_system.clone());
                if !from_carrier {
                    self.station = None;
                    self.body = None;
                }
            }
            JournalEvent::Docked { station_name, star_system } => {
                self.station = Some(station_name.clone());
                self.system = Some(star_system.clone());
                self.docked = true;
            }
            JournalEvent::Undocked { .. } => self.docked = false,
            _ => {}
        }
        self.materials.apply("", ev);
    }
    pub fn apply_status(&mut self, s: &StatusSnap) {
        self.status_flags = s.flags;
        self.status_seen = true;
        self.gui_focus = s.gui_focus;
        self.gear_down = s.flags & FLAG_GEAR != 0;
        self.supercruise = s.flags & FLAG_SUPERCRUISE != 0;
        self.docked = s.flags & FLAG_DOCKED != 0;
        if s.fuel_main.is_some() {
            self.fuel_main = s.fuel_main;
        }
        if s.fuel_reservoir.is_some() {
            self.fuel_reservoir = s.fuel_reservoir;
        }
        if s.cargo.is_some() {
            self.cargo_tonnes = s.cargo;
        }
        if s.legal_state.is_some() {
            self.legal_state = s.legal_state.clone();
        }
        if s.body_name.is_some() {
            self.body = s.body_name.clone();
        }
        if s.balance.is_some() {
            self.credits = s.balance;
        }
    }

    pub fn apply_cargo(&mut self, c: &CargoSnap) {
        self.cargo_tonnes = Some(c.count as f64);
        self.cargo_items = c.items.clone();
    }

    pub fn apply_route(&mut self, r: &NavRouteSnap) {
        self.route = r.systems.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn folds_events() {
        let mut g = GameState::default();
        for l in [
            r#"{"timestamp":"t","event":"LoadGame","Commander":"Test","Ship":"krait_mkii","Credits":50}"#,
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Shinrarta Dezhra","JumpDist":12.0}"#,
            r#"{"timestamp":"t","event":"Docked","StationName":"Jameson Memorial","StarSystem":"Shinrarta Dezhra"}"#,
        ] {
            g.apply_journal(&astromech_journal::parse_line(l).unwrap().event);
        }
        assert_eq!(g.station.as_deref(), Some("Jameson Memorial"));
        assert!(g.docked);
        assert_eq!(g.credits, Some(50));
        g.apply_status(&StatusSnap { flags: FLAG_GEAR, gui_focus: None, ..Default::default() });
        assert!(g.gear_down && !g.docked);
        g.apply_route(&NavRouteSnap { systems: vec!["Shinrarta Dezhra".into(), "Sol".into()] });
        assert_eq!(g.route.len(), 2);
        g.apply_journal(
            &astromech_journal::parse_line(
                r#"{"timestamp":"t","event":"Materials","Raw":[{"Name":"iron","Count":4}],"Manufactured":[],"Encoded":[]}"#,
            )
            .unwrap()
            .event,
        );
        assert_eq!(g.materials.count("iron"), 4);
        g.apply_journal(
            &astromech_journal::parse_line(
                r#"{"timestamp":"t","event":"CarrierJump","StarSystem":"Alioth","SystemAllegiance":"Alliance"}"#,
            )
            .unwrap()
            .event,
        );
        assert_eq!(g.system.as_deref(), Some("Alioth"));
        assert_eq!(g.station.as_deref(), Some("Jameson Memorial"));
    }
}
