//! Biology other players reported for a system (Spansh's body signals and codex sightings, sent
//! to EDDN), for an arrival before any scan.

use crate::catalog;
use serde::Serialize;

/// A body with biology, as the remote data has it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KnownBody {
    pub name: String,
    pub body_id: Option<i64>,
    /// Biological signals.
    pub signals: u32,
    /// Genera from someone's DSS, catalogue names.
    pub genera: Vec<String>,
    /// Species someone sampled there, with their catalogue value.
    pub species: Vec<(String, i64)>,
}

impl KnownBody {
    /// Build one from what the remote data lists: genus names in any form the catalogue knows
    /// (Spansh's `Bacterial`, `Shrubs`, ...) and species names; unknown names are dropped,
    /// duplicates merged.
    pub fn new<'a>(
        name: &str,
        body_id: Option<i64>,
        signals: u32,
        genera: impl IntoIterator<Item = &'a str>,
        species: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        let mut b = KnownBody { name: name.to_string(), body_id, signals, genera: vec![], species: vec![] };
        for s in species {
            if let Some((g, sp)) = catalog::species(s) {
                if !b.species.iter().any(|(n, _)| *n == sp.name) {
                    b.species.push((sp.name.clone(), sp.value));
                }
                if !b.genera.contains(&g.name) {
                    b.genera.push(g.name.clone());
                }
            }
        }
        for g in genera {
            if let Some(g) = catalog::genus(g) {
                if !b.genera.contains(&g.name) {
                    b.genera.push(g.name.clone());
                }
            }
        }
        b
    }

    /// The most valuable thing known to be there: (what to call it, its value, exact). A
    /// sampled species is exact; a genus without one is its dearest species, not exact.
    pub fn best(&self) -> Option<(String, i64, bool)> {
        let sampled = self.species.iter().map(|(n, v)| (n.clone(), *v, true));
        let genera = self
            .genera
            .iter()
            .filter(|g| !self.species.iter().any(|(s, _)| catalog::species(s).is_some_and(|(sg, _)| sg.name == **g)))
            .filter_map(|g| catalog::genus(g).map(|c| (g.clone(), c.value_range().1, false)));
        sampled.chain(genera).max_by_key(|(_, v, _)| *v)
    }

    /// Lowest and highest value: sampled species exactly, other known genera over their range.
    pub fn value(&self) -> (i64, i64) {
        let exact: i64 = self.species.iter().map(|(_, v)| v).sum();
        self.genera
            .iter()
            .filter(|g| !self.species.iter().any(|(s, _)| catalog::species(s).is_some_and(|(sg, _)| sg.name == **g)))
            .filter_map(|g| catalog::genus(g).map(|c| c.value_range()))
            .fold((exact, exact), |(lo, hi), (a, b)| (lo + a, hi + b))
    }
}

/// Bodies with biology, most valuable first (by the middle of the value range).
pub fn ranked(mut bodies: Vec<KnownBody>) -> Vec<KnownBody> {
    bodies.retain(|b| b.signals > 0 || !b.genera.is_empty());
    bodies.sort_by(|a, b| crate::middle(b.value()).cmp(&crate::middle(a.value())).then(a.name.cmp(&b.name)));
    bodies
}
