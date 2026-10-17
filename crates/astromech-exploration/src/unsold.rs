//! Unsold exploration data: what the cartographic data and organic samples aboard would fetch,
//! folded from the journal since the commander's last death.
//!
//! - **Cartographic**: every star and planet scanned (`Scan`, once per body; nav beacon data
//!   too, which Universal Cartographics buys like any other), at its value now, mapped if the
//!   commander mapped it (`SAAScanComplete`) ([`crate::value`]). `SellExplorationData` and
//!   `MultiSellExplorationData` sell the systems they list, so their bodies drop out.
//! - **Organic**: every completed sample (`ScanOrganic` with `ScanType: Analyse`) at its species'
//!   value ([`crate::exobiology`]); a species with no known value counts 0 and is counted apart.
//!   `SellOrganicData` sells the samples it lists, one per entry.
//! - **Dying** (`Died`) loses both, so the fold starts afresh there.
//!
//! The first-logged bonus on organics and selling through a fleet carrier aren't estimated.

use crate::{exobiology, value};
use astromech_journal::{ExtraEvent as X, JournalEvent as J, JournalLine, ScanBody};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

/// The journal events the fold reads (for readers that pick lines by event, like the history).
pub const EVENTS: &[&str] = &[
    "Scan",
    "SAAScanComplete",
    "ScanOrganic",
    "SellExplorationData",
    "MultiSellExplorationData",
    "SellOrganicData",
    "Died",
];

/// Above this, losing the data hurts: the page reminds the commander to sell it.
pub const RISKY: i64 = 50_000_000;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Unsold {
    pub cartographic: i64,
    pub systems: usize,
    pub bodies: usize,
    pub organic: i64,
    pub samples: usize,
    /// Samples of a species with no known value (counted as 0).
    pub unknown_samples: usize,
    pub total: i64,
    /// The oldest scan or sample still unsold (journal timestamp).
    pub since: Option<String>,
    /// [`total`](Self::total) is at least [`RISKY`].
    pub risky: bool,
}

#[derive(Debug, Clone)]
struct Body {
    scan: ScanBody,
    mapped: Option<bool>,
    ts: String,
}

#[derive(Debug, Clone)]
struct Sample {
    species: String,
    value: Option<i64>,
    ts: String,
}

#[derive(Debug, Clone, Default)]
pub struct UnsoldFold {
    /// System name → body name → body.
    systems: BTreeMap<String, BTreeMap<String, Body>>,
    names: HashMap<i64, String>,
    samples: Vec<Sample>,
}

impl UnsoldFold {
    pub fn apply(&mut self, line: &JournalLine) {
        if line.name == "Died" {
            *self = Self::default();
            return;
        }
        let J::Extra(x) = &line.event else { return };
        match x {
            X::Scan(b) => {
                let is_body = b.star_type.is_some() || b.planet_class.as_deref().is_some_and(|c| !c.is_empty());
                let system =
                    b.star_system.clone().or_else(|| b.system_address.and_then(|a| self.names.get(&a).cloned()));
                let Some(system) = system.filter(|_| is_body) else { return };
                if let Some(a) = b.system_address {
                    self.names.insert(a, system.clone());
                }
                let bodies = self.systems.entry(system).or_default();
                let mapped = bodies.get(&b.body_name).and_then(|old| old.mapped);
                bodies.insert(b.body_name.clone(), Body { scan: (**b).clone(), mapped, ts: line.timestamp.clone() });
            }
            X::SaaScanComplete { body_name, system_address, probes_used, efficiency_target, .. } => {
                let Some(system) = system_address.and_then(|a| self.names.get(&a)) else { return };
                if let Some(b) = self.systems.get_mut(system).and_then(|s| s.get_mut(body_name)) {
                    b.mapped = Some(*efficiency_target > 0 && probes_used <= efficiency_target);
                }
            }
            X::ScanOrganic { scan_type, species, genus, .. } if scan_type == "Analyse" => {
                let value = exobiology::species_value(species, genus);
                self.samples.push(Sample { species: species.to_ascii_lowercase(), value, ts: line.timestamp.clone() });
            }
            X::ExplorationSale { systems, .. } => {
                for s in systems {
                    self.systems.remove(s);
                }
            }
            X::SellOrganicData { bio_data, .. } => {
                for sold in bio_data {
                    let species = sold.species.to_ascii_lowercase();
                    if let Some(i) = self.samples.iter().position(|s| s.species == species) {
                        self.samples.remove(i);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn unsold(&self) -> Unsold {
        let bodies: Vec<&Body> = self.systems.values().flat_map(|s| s.values()).collect();
        let cartographic = bodies.iter().filter_map(|b| value::estimate(&b.scan, b.mapped)).map(|(now, _)| now).sum();
        let organic = self.samples.iter().filter_map(|s| s.value).sum();
        let since = bodies.iter().map(|b| &b.ts).chain(self.samples.iter().map(|s| &s.ts)).min().cloned();
        let total = cartographic + organic;
        Unsold {
            cartographic,
            systems: self.systems.values().filter(|s| !s.is_empty()).count(),
            bodies: bodies.len(),
            organic,
            samples: self.samples.len(),
            unknown_samples: self.samples.iter().filter(|s| s.value.is_none()).count(),
            total,
            since,
            risky: total >= RISKY,
        }
    }
}

/// The unsold data after these journal lines (as the game wrote them; unreadable ones are
/// skipped).
pub fn from_json_lines<S: AsRef<str>>(lines: &[S]) -> Unsold {
    let mut f = UnsoldFold::default();
    for l in lines {
        if let Ok(l) = astromech_journal::parse_line(l.as_ref()) {
            f.apply(&l);
        }
    }
    f.unsold()
}
