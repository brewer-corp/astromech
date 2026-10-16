//! Prospector callouts (`ProspectedAsteroid`): speak only rocks worth mining. A rock whose
//! high-value ore is at or above that ore's threshold gets one short line naming the ore and
//! its percentage ("Platinum 34%."); a motherlode always gets a line ("Motherlode. Void
//! Opals."). Everything else stays quiet.
//!
//! Thresholds are per ore: [`ORES`] gives the defaults (a static value order until the mining
//! sale advisor exists); the commander's changes are [`Thresholds`] (stored by the core, edited
//! in Settings). An ore set to `None` is never called out.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, JournalEvent};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// One ore astromech knows: the journal symbol (lower case), the name to say, and the default
/// threshold in percent (None: only as a motherlode).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ore {
    pub symbol: &'static str,
    pub name: &'static str,
    pub default: Option<f64>,
}

/// High-value ores, most valuable first (static order, wiki-visible names). Lines name at most
/// two ores, in this order.
pub const ORES: &[Ore] = &[
    Ore { symbol: "opal", name: "Void Opals", default: None },
    Ore { symbol: "lowtemperaturediamond", name: "Low Temperature Diamonds", default: Some(25.0) },
    Ore { symbol: "painite", name: "Painite", default: Some(25.0) },
    Ore { symbol: "platinum", name: "Platinum", default: Some(20.0) },
    Ore { symbol: "musgravite", name: "Musgravite", default: None },
    Ore { symbol: "grandidierite", name: "Grandidierite", default: None },
    Ore { symbol: "alexandrite", name: "Alexandrite", default: None },
    Ore { symbol: "benitoite", name: "Benitoite", default: None },
    Ore { symbol: "monazite", name: "Monazite", default: None },
    Ore { symbol: "rhodplumsite", name: "Rhodplumsite", default: None },
    Ore { symbol: "serendibite", name: "Serendibite", default: None },
    Ore { symbol: "osmium", name: "Osmium", default: Some(25.0) },
    Ore { symbol: "palladium", name: "Palladium", default: Some(30.0) },
    Ore { symbol: "bromellite", name: "Bromellite", default: Some(30.0) },
    Ore { symbol: "tritium", name: "Tritium", default: Some(30.0) },
];

pub fn ore(symbol: &str) -> Option<&'static Ore> {
    let s = symbol.to_ascii_lowercase();
    ORES.iter().find(|o| o.symbol == s)
}

/// The commander's changes to the default thresholds, by ore symbol. `None` turns an ore off.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Thresholds(pub BTreeMap<String, Option<f64>>);

impl Thresholds {
    /// The threshold in force for `symbol` (percent), or None if it isn't called out.
    pub fn get(&self, symbol: &str) -> Option<f64> {
        let o = ore(symbol)?;
        match self.0.get(o.symbol) {
            Some(v) => *v,
            None => o.default,
        }
    }

    /// Set (`Some`) or turn off (`None`) an ore. Errors name what was wrong.
    pub fn set(&mut self, symbol: &str, threshold: Option<f64>) -> Result<(), String> {
        let o = ore(symbol).ok_or_else(|| format!("unknown ore {symbol:?}"))?;
        if let Some(t) = threshold {
            if !(0.0..=100.0).contains(&t) {
                return Err(format!("{}: threshold {t} is not a percentage (0 to 100)", o.name));
            }
        }
        self.0.insert(o.symbol.to_string(), threshold);
        Ok(())
    }

    /// Back to the default for `symbol`.
    pub fn reset(&mut self, symbol: &str) {
        if let Some(o) = ore(symbol) {
            self.0.remove(o.symbol);
        }
    }
}

/// Shared with the core, which changes it when Settings does.
pub type SharedThresholds = Arc<RwLock<Thresholds>>;

pub struct Prospector {
    pub thresholds: SharedThresholds,
}

/// "Platinum 34%." / "Motherlode. Void Opals. Platinum 34%." or None for a rock not worth it.
pub fn line(materials: &[(&str, f64)], motherlode: Option<&str>, t: &Thresholds) -> Option<String> {
    let mut parts = vec![];
    if let Some(m) = motherlode.filter(|m| !m.is_empty()) {
        let name = ore(m).map(|o| o.name.to_string()).unwrap_or_else(|| capitalise(m));
        parts.push(format!("Motherlode. {name}."));
    }
    let mut hits: Vec<(usize, &Ore, f64)> = materials
        .iter()
        .filter_map(|(sym, p)| {
            let i = ORES.iter().position(|o| o.symbol.eq_ignore_ascii_case(sym))?;
            (*p >= t.get(sym)?).then_some((i, &ORES[i], *p))
        })
        .collect();
    hits.sort_by_key(|h| h.0);
    for (_, o, p) in hits.into_iter().take(2) {
        parts.push(format!("{} {:.0}%.", o.name, p.floor()));
    }
    (!parts.is_empty()).then(|| parts.join(" "))
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

impl Rule for Prospector {
    fn id(&self) -> &'static str {
        "prospector"
    }
    /// Each prospector limpet reports a different rock, so no cooldown by default.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, _: &GameState) -> Option<Callout> {
        let Input::Journal(l) = input else { return None };
        let JournalEvent::Extra(ExtraEvent::ProspectedAsteroid { materials, motherlode_material, .. }) = &l.event
        else {
            return None;
        };
        let m: Vec<(&str, f64)> = materials.iter().map(|m| (m.name.as_str(), m.proportion)).collect();
        let t = self.thresholds.read().unwrap();
        let text = line(&m, motherlode_material.as_deref(), &t)?;
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_default_override_and_off() {
        let mut t = Thresholds::default();
        assert_eq!(t.get("Platinum"), Some(20.0));
        assert_eq!(t.get("opal"), None);
        assert_eq!(t.get("iron"), None);
        t.set("platinum", Some(35.0)).unwrap();
        t.set("painite", None).unwrap();
        assert_eq!((t.get("platinum"), t.get("painite")), (Some(35.0), None));
        assert!(t.set("platinum", Some(120.0)).unwrap_err().contains("0 to 100"));
        assert!(t.set("iron", Some(5.0)).unwrap_err().contains("unknown ore"));
        t.reset("platinum");
        assert_eq!(t.get("platinum"), Some(20.0));
    }
}
