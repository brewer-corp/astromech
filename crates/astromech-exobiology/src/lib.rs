//! # astromech-exobiology
//!
//! Exobiology for the explorer (feature astromech-znhx):
//!
//! - [`surface`]: great-circle bearing and distance between two latitude/longitude points on a
//!   body of a given radius (Status.json's `Latitude`, `Longitude` and `PlanetRadius`).
//! - [`catalog`]: every genus and species with its value, colony range and growing conditions
//!   (`data/catalog/exobiology.json`).
//! - [`predict`]: which species could grow on a body, from its detailed scan.
//! - [`system`]: the current system's biology from the journal (FSS counts, DSS genus lists,
//!   scans, samples), each body's genera with their likely species and value.

pub mod catalog;
pub mod predict;
pub mod surface;
pub mod system;

use astromech_journal::JournalEvent;
use serde::Serialize;

/// Everything exobiology keeps from the journal, for the game state.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Exobiology {
    /// The current system's biology.
    pub system: system::SystemBio,
}

impl Exobiology {
    pub fn apply(&mut self, ev: &JournalEvent) {
        self.system.apply(ev);
    }
}
