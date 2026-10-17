//! The current system's biology, ranked by value: each body's genera (from the DSS, or predicted
//! from its detailed scan before the DSS) with the species they could be and what they are worth.
//!
//! A view over [`SystemFold`], which tracks the system's bodies (scans, FSS and DSS signals,
//! genus lists, samples); the species catalogue and the prediction are
//! `astromech_exobiology::{catalog, predict}`.

use crate::system::{BodyRecord, SystemFold};
use astromech_exobiology::catalog;
use astromech_exobiology::predict::{self, Candidate};
use serde::Serialize;
use std::collections::BTreeMap;

/// One genus on a body: known from the DSS (or a sample), or predicted from the conditions.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenusBio {
    pub genus: String,
    /// The DSS or a sample says it's there; false: predicted only.
    pub known: bool,
    /// The species it could be, best fit first (one when sampled). Empty if the conditions
    /// don't single any out.
    pub species: Vec<Candidate>,
    /// Lowest and highest base value it could be worth.
    pub value: (i64, i64),
    /// The least distance between samples, metres (None for a genus not in the catalogue).
    pub colony_range_m: Option<u32>,
}

/// A body with biology: what's there, what it's worth.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BodyBio {
    pub body_id: Option<i64>,
    pub name: String,
    /// Biological signals, if the FSS or DSS has reported them.
    pub signals: Option<u32>,
    /// The genus list came from the DSS.
    pub mapped: bool,
    /// Known genera first, then predicted ones, each most valuable first.
    pub genera: Vec<GenusBio>,
    /// What the body's biology is worth, lowest to highest: the known genera's ranges summed,
    /// or before the DSS, the `signals` cheapest and dearest predicted genera.
    pub value: (i64, i64),
}

impl BodyBio {
    /// The most valuable genus: (what to call it, the top of its value, exact). Exact when the
    /// DSS or a sample says the genus is there and only one species fits; it is then named by
    /// its species, otherwise by the genus.
    pub fn best(&self) -> Option<(String, i64, bool)> {
        let g = self.genera.iter().filter(|g| g.known || !self.mapped).max_by_key(|g| g.value.1)?;
        Some(match g.species.as_slice() {
            [one] if g.known => (one.species.to_string(), g.value.1, true),
            _ => (g.genus.clone(), g.value.1, false),
        })
    }
}

/// "Stratum" for a genus symbol; the symbol's middle part if the catalogue doesn't know it.
fn genus_name(symbol: &str) -> String {
    catalog::genus(symbol).map(|g| g.name.clone()).unwrap_or_else(|| {
        symbol.trim_start_matches("$Codex_Ent_").trim_end_matches(';').trim_end_matches("_Name").replace('_', " ")
    })
}

/// Star class by BodyID, from the scanned stars.
fn stars(fold: &SystemFold) -> BTreeMap<i64, String> {
    fold.records()
        .filter_map(|r| {
            let s = r.scan?;
            Some((s.body_id?, catalog::star_class(s.star_type.as_deref()?)))
        })
        .collect()
}

/// One body's biology, or None if nothing says it has any.
fn body_bio(r: &BodyRecord, stars: &BTreeMap<i64, String>) -> Option<BodyBio> {
    let dss: Vec<String> = r.genuses.iter().map(|g| genus_name(g)).collect();
    let sampled: BTreeMap<String, &str> = r.sampled.iter().map(|(g, s)| (genus_name(g), s.as_str())).collect();
    if r.bio_signals <= 0 && dss.is_empty() && sampled.is_empty() {
        return None;
    }
    let candidates: Vec<Candidate> = r
        .scan
        .and_then(|s| {
            let star = s.parents.iter().find(|(kind, _)| kind == "Star").and_then(|(_, id)| stars.get(id).cloned());
            predict::Body::from_scan(s, star)
        })
        .map(|c| predict::predict(&c))
        .unwrap_or_default();
    let genus_bio = |genus: &str, known: bool| -> GenusBio {
        let cat = catalog::genus(genus);
        let species: Vec<Candidate> = match sampled.get(genus).and_then(|s| catalog::species(s)) {
            Some((g, s)) => vec![Candidate { genus: &g.name, species: &s.name, value: s.value, fit: 1.0 }],
            None => candidates.iter().filter(|c| c.genus == genus).cloned().collect(),
        };
        let value = if species.is_empty() {
            cat.map_or((0, 0), catalog::Genus::value_range)
        } else {
            predict::value_range(&species)
        };
        GenusBio { genus: genus.to_string(), known, species, value, colony_range_m: cat.map(|g| g.colony_range_m) }
    };
    let mut known = dss.clone();
    for g in sampled.keys() {
        if !known.contains(g) {
            known.push(g.clone());
        }
    }
    let mut genera: Vec<GenusBio> = known.iter().map(|g| genus_bio(g, true)).collect();
    let mut predicted: Vec<GenusBio> = vec![];
    if dss.is_empty() {
        for c in &candidates {
            if !known.iter().any(|k| k == c.genus) && !predicted.iter().any(|p| p.genus == c.genus) {
                predicted.push(genus_bio(c.genus, false));
            }
        }
    }
    let by_value = |a: &GenusBio, b: &GenusBio| b.value.1.cmp(&a.value.1).then(b.value.0.cmp(&a.value.0));
    genera.sort_by(by_value);
    predicted.sort_by(by_value);
    let mut value = genera.iter().fold((0, 0), |(lo, hi), g| (lo + g.value.0, hi + g.value.1));
    let signals = (r.bio_signals > 0).then_some(r.bio_signals as u32);
    if dss.is_empty() {
        // Before the DSS: the signal count says how many genera there are; the cheapest and
        // dearest that many of the predicted ones bound the value.
        let left = signals.unwrap_or(0).saturating_sub(genera.len() as u32) as usize;
        let mut lows: Vec<i64> = predicted.iter().map(|g| g.value.0).collect();
        lows.sort();
        value.0 += lows.iter().take(left).sum::<i64>();
        value.1 += predicted.iter().take(left).map(|g| g.value.1).sum::<i64>();
    }
    genera.extend(predicted);
    Some(BodyBio { body_id: r.body_id, name: r.name.to_string(), signals, mapped: !dss.is_empty(), genera, value })
}

/// Every body with biology, most valuable first: by the middle of its value range, so a body
/// with a sure 19 M ranks above one that might reach 25 M or be worth 5 M.
pub fn ranked(fold: &SystemFold) -> Vec<BodyBio> {
    let stars = stars(fold);
    let mut v: Vec<BodyBio> = fold.records().filter_map(|r| body_bio(&r, &stars)).collect();
    v.sort_by(|a, b| {
        astromech_exobiology::middle(b.value).cmp(&astromech_exobiology::middle(a.value)).then(a.name.cmp(&b.name))
    });
    v
}

/// One body's biology by its BodyID.
pub fn body(fold: &SystemFold, body_id: i64) -> Option<BodyBio> {
    let stars = stars(fold);
    fold.records().find(|r| r.body_id == Some(body_id)).and_then(|r| body_bio(&r, &stars))
}
