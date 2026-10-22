//! The current mining run: tons refined (one per `MiningRefined`) and tons per hour.
//!
//! A run starts at the first prospector or collector `LaunchDrone`, `ProspectedAsteroid` or
//! `MiningRefined` after the last `Docked` or `LoadGame` (docking is where the haul is sold),
//! and its rate is measured from then to the latest journal line. Kept apart from [`crate::GameState`] because it needs the journal timestamps.

use astromech_journal::{ExtraEvent, JournalEvent, JournalLine};
use chrono::{DateTime, FixedOffset};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MiningRun {
    /// When the run started (journal time).
    pub started: Option<DateTime<FixedOffset>>,
    /// The latest journal time seen since the run started (the rate is measured to here, so a
    /// replay gives what live did).
    pub last: Option<DateTime<FixedOffset>>,
    /// Tons refined, by commodity symbol (`platinum`, from `$platinum_name;`).
    pub refined: BTreeMap<String, u32>,
}

impl MiningRun {
    pub fn apply(&mut self, line: &JournalLine) {
        if matches!(line.name.as_str(), "Docked" | "LoadGame") {
            *self = Self::default();
            return;
        }
        if self.started.is_some() {
            self.last = DateTime::parse_from_rfc3339(&line.timestamp).ok().or(self.last);
        }
        let JournalEvent::Extra(event) = &line.event else { return };
        let mining = match event {
            // Repair, fuel, hatchbreaker and the other limpets aren't mining.
            ExtraEvent::LaunchDrone { kind } => matches!(kind.as_str(), "Prospector" | "Collection"),
            ExtraEvent::ProspectedAsteroid { .. } | ExtraEvent::MiningRefined { .. } => true,
            _ => false,
        };
        if !mining {
            return;
        }
        if self.started.is_none() {
            self.started = DateTime::parse_from_rfc3339(&line.timestamp).ok();
            self.last = self.started;
        }
        if let ExtraEvent::MiningRefined { commodity } = event {
            let sym = commodity.trim_start_matches('$').trim_end_matches(';').trim_end_matches("_name");
            *self.refined.entry(sym.to_ascii_lowercase()).or_default() += 1;
        }
    }

    /// Tons refined this run.
    pub fn total(&self) -> u32 {
        self.refined.values().sum()
    }

    /// Tons per hour from the start of the run to the latest journal line.
    pub fn per_hour(&self) -> Option<f64> {
        self.tons_per_hour(self.last?)
    }

    /// Tons per hour from the start of the run to `at`, or None before a minute has passed.
    pub fn tons_per_hour(&self, at: DateTime<FixedOffset>) -> Option<f64> {
        let secs = (at - self.started?).num_seconds();
        (secs >= 60).then(|| self.total() as f64 * 3600.0 / secs as f64)
    }
}
