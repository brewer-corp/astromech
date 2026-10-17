//! Where power commodities and data move, as data (`data/catalog/powerplay-logistics.json`):
//! for a target system of a kind (acquire, reinforce, undermine) which of the pledged power's
//! systems can supply commodities and within what distance, and where data is handed in. The
//! Spansh search that finds the actual systems is in `astromech-galaxy`. Frontier changes these
//! rules often, so the table is versioned data with its sources.

use crate::system::Work;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Where data of a kind is handed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandIn {
    /// At the target system.
    Target,
    /// At the systems that can supply commodities for the target.
    Supporting,
    /// At any system the pledged power controls.
    Controlled,
}

/// A kind of system that can supply commodities for a target.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Pickup {
    pub work: Work,
    /// Journal / Spansh `PowerplayState` values of the supplying system.
    pub states: Vec<String>,
    /// Farthest from the target; `None` when no limit is documented.
    pub max_ly: Option<f64>,
    pub uncertain: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DataDelivery {
    pub work: Work,
    pub at: HandIn,
    pub uncertain: Option<String>,
}

#[derive(Deserialize)]
struct LogisticsFile {
    version: u32,
    fetched: String,
    pickups: Vec<Pickup>,
    data_delivery: Vec<DataDelivery>,
    controlled_states: Vec<String>,
    warnings: Vec<String>,
}

fn file() -> &'static LogisticsFile {
    static F: OnceLock<LogisticsFile> = OnceLock::new();
    F.get_or_init(|| {
        serde_json::from_str(include_str!("../../../data/catalog/powerplay-logistics.json"))
            .expect("powerplay-logistics.json")
    })
}

/// The systems that can supply commodities for a `work` target.
pub fn pickups(work: Work) -> impl Iterator<Item = &'static Pickup> {
    file().pickups.iter().filter(move |p| p.work == work)
}

/// Where data for a `work` target is handed in.
pub fn data_delivery(work: Work) -> Option<&'static DataDelivery> {
    file().data_delivery.iter().find(|d| d.work == work)
}

/// States of a system the power controls.
pub fn controlled_states() -> &'static [String] {
    &file().controlled_states
}

/// Cautions to show with the lookup.
pub fn warnings() -> &'static [String] {
    &file().warnings
}

/// The table's version and fetch date.
pub fn version() -> (u32, &'static str) {
    (file().version, &file().fetched)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquisition_supply_follows_the_20_and_30_ly_rule() {
        let p: Vec<_> = pickups(Work::Acquire).map(|p| (p.states[0].as_str(), p.max_ly)).collect();
        assert_eq!(p, [("Fortified", Some(20.0)), ("Stronghold", Some(30.0))]);
        assert!(pickups(Work::Reinforce).all(|p| p.states == ["Stronghold"]));
        assert_eq!(data_delivery(Work::Reinforce).unwrap().at, HandIn::Target);
        assert!(version().0 >= 1 && !warnings().is_empty());
    }
}
