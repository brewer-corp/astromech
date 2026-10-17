//! # astromech-exobiology
//!
//! Exobiology for the explorer (feature astromech-znhx):
//!
//! - [`surface`]: great-circle bearing and distance between two latitude/longitude points on a
//!   body of a given radius (Status.json's `Latitude`, `Longitude` and `PlanetRadius`).
//! - [`catalog`]: every genus and species with its value, colony range and growing conditions
//!   (`data/catalog/exobiology.json`).
//! - [`predict`]: which species could grow on a body, from its detailed scan.
//!
//! The system's bodies and their biology view are `astromech_exploration::{system, biology}`,
//! which use this catalogue and prediction.
//! - [`known`]: biology other players reported for a system (Spansh), for an arrival.
//! - [`sampling`]: the species being sampled, where its samples were taken, and whether the
//!   commander is now far enough from them for the next one.

pub mod catalog;
pub mod known;
pub mod predict;
pub mod sampling;
pub mod surface;

use astromech_journal::JournalEvent;
use serde::Serialize;

/// Everything exobiology keeps from the journal, for the game state.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Exobiology {
    /// The species being sampled and where.
    pub sampling: sampling::Sampling,
}

impl Exobiology {
    pub fn apply(&mut self, ev: &JournalEvent) {
        self.sampling.apply(ev);
    }

    pub fn apply_status(&mut self, s: &astromech_status::StatusSnap) {
        self.sampling.apply_status(s);
    }
}

/// The middle of a value range, for ranking.
pub fn middle((lo, hi): (i64, i64)) -> i64 {
    lo / 2 + hi / 2
}
