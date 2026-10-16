//! # astromech-gamestate
//!
//! The reducer: folds journal events and Status.json snapshots into one [`GameState`]
//! (ARCHITECTURE.md §5.2). Pure and synchronous so it can be replay-tested.

use astromech_journal::companion::{
    CompanionFile, FcMaterialsSnap, InventorySnap, MarketSnap, ModulesInfoSnap, OutfittingSnap, ShipyardSnap,
};
use astromech_journal::{CargoSnap, JournalEvent, NavRouteSnap};
use astromech_materials::Ledger;
use astromech_status::{StatusSnap, FLAG_DOCKED, FLAG_GEAR, FLAG_SUPERCRUISE};
use serde::Serialize;
use std::sync::Arc;

/// A snapshot shared by every copy of the state. The hub clones and compares the whole state on
/// every update, so a 300-line market must not be copied or compared by value each time; two
/// are equal when they are the same read of the file.
#[derive(Debug)]
pub struct Shared<T>(pub Arc<T>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> std::ops::Deref for Shared<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// The latest fresh snapshot of each companion file (`Market.json`, ...). Not serialised to the
/// UI: they are large and consumers read them from the state directly.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Companions {
    pub market: Option<Shared<MarketSnap>>,
    pub outfitting: Option<Shared<OutfittingSnap>>,
    pub shipyard: Option<Shared<ShipyardSnap>>,
    pub modules_info: Option<Shared<ModulesInfoSnap>>,
    pub ship_locker: Option<Shared<InventorySnap>>,
    pub backpack: Option<Shared<InventorySnap>>,
    pub fc_materials: Option<Shared<FcMaterialsSnap>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct GameState {
    pub commander: Option<String>,
    pub ship: Option<String>,
    pub ship_name: Option<String>,
    pub ship_ident: Option<String>,
    pub system: Option<String>,
    pub body: Option<String>,
    pub station: Option<String>,
    /// `MarketID` of the station docked at; ties `Market.json` and friends to this dock.
    pub station_market_id: Option<i64>,
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
    /// `Cargo.json`'s lines with their symbols (for "where should I sell my cargo?").
    pub cargo_inventory: Vec<astromech_journal::CargoItem>,
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
    #[serde(skip)]
    pub companions: Companions,
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
                ..
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
            JournalEvent::Location { star_system, body, docked, station_name, market_id, .. } => {
                self.system = Some(star_system.clone());
                self.body = body.clone();
                self.docked = *docked;
                // Logging in docked: `Location` carries the station, there is no `Docked`.
                if *docked {
                    if station_name.is_some() {
                        self.station = station_name.clone();
                    }
                    self.station_market_id = *market_id;
                } else {
                    self.station = None;
                    self.station_market_id = None;
                }
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
                    self.station_market_id = None;
                    self.body = None;
                }
            }
            JournalEvent::Docked { station_name, star_system, market_id, .. } => {
                self.station = Some(station_name.clone());
                self.station_market_id = *market_id;
                self.system = Some(star_system.clone());
                self.docked = true;
            }
            JournalEvent::Undocked { .. } => {
                self.docked = false;
                self.station_market_id = None;
            }
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
        self.cargo_inventory = c.inventory.clone();
    }

    /// Keep a companion file that passed the freshness check (see `astromech_journal::companion`).
    pub fn apply_companion(&mut self, f: CompanionFile) {
        let c = &mut self.companions;
        match f {
            CompanionFile::Market(x) => c.market = Some(Shared(Arc::new(x))),
            CompanionFile::Outfitting(x) => c.outfitting = Some(Shared(Arc::new(x))),
            CompanionFile::Shipyard(x) => c.shipyard = Some(Shared(Arc::new(x))),
            CompanionFile::ModulesInfo(x) => c.modules_info = Some(Shared(Arc::new(x))),
            CompanionFile::ShipLocker(x) => c.ship_locker = Some(Shared(Arc::new(x))),
            CompanionFile::Backpack(x) => c.backpack = Some(Shared(Arc::new(x))),
            CompanionFile::FcMaterials(x) => c.fc_materials = Some(Shared(Arc::new(x))),
        }
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
