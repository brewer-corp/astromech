//! The current system's biology from the journal: biological signal counts per body from the
//! FSS (`FSSBodySignals`), the genus list from the DSS (`SAASignalsFound.Genuses`), each
//! landable body's conditions from its `Scan` (with the nearest parent star's class) for the
//! species prediction, and the species confirmed by sampling (`ScanOrganic`).
//!
//! It starts again on arrival in another system (`FSDJump`, `CarrierJump`, a `Location`
//! elsewhere). Events for another system (by `SystemAddress`) are ignored.

use crate::catalog;
use crate::predict::{self, Candidate};
use astromech_journal::{BodySignals, ExtraEvent, JournalEvent};
use serde::Serialize;
use std::collections::BTreeMap;

const BIOLOGICAL: &str = "$SAA_SignalType_Biological;";

/// What the journal said about one body.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Body {
    pub body_id: i64,
    pub name: String,
    /// Biological signals (FSS or DSS); None until either reports the body.
    pub signals: Option<u32>,
    /// Genera from the DSS, catalogue names ("Stratum"); an unknown symbol as the game wrote it.
    pub genera: Vec<String>,
    /// Conditions for the prediction (landable planets only).
    pub conditions: Option<predict::Body>,
    /// `Parents` from the scan, nearest first.
    pub parents: Vec<(String, i64)>,
    /// Species seen by sampling, by genus.
    pub species: BTreeMap<String, String>,
}

/// One genus on a body: known from the DSS (or sampling), or predicted from the conditions.
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
    pub body_id: i64,
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

/// The current system's biology.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct SystemBio {
    pub system_address: Option<i64>,
    pub system: Option<String>,
    /// Star class ([`catalog::star_class`]) by BodyID.
    pub stars: BTreeMap<i64, String>,
    pub bodies: BTreeMap<i64, Body>,
}

/// "Stratum" for a DSS genus symbol; the symbol's middle part if the catalogue doesn't know it.
fn genus_name(symbol: &str) -> String {
    catalog::genus(symbol).map(|g| g.name.clone()).unwrap_or_else(|| {
        symbol.trim_start_matches("$Codex_Ent_").trim_end_matches(';').trim_end_matches("_Name").replace('_', " ")
    })
}

impl SystemBio {
    fn arrive(&mut self, address: Option<i64>, name: &str) {
        if address.is_some() && address == self.system_address {
            return;
        }
        *self = SystemBio { system_address: address, system: Some(name.to_string()), ..Default::default() };
    }

    /// Whether an event for `address` is about this system (no address: assume so).
    fn here(&self, address: Option<i64>) -> bool {
        address.is_none() || self.system_address.is_none() || address == self.system_address
    }

    fn body(&mut self, id: i64, name: &str) -> &mut Body {
        let b = self.bodies.entry(id).or_insert_with(|| Body { body_id: id, ..Default::default() });
        if !name.is_empty() {
            b.name = name.to_string();
        }
        b
    }

    fn signals(&mut self, s: &BodySignals, dss: bool) {
        let (Some(id), true) = (s.body_id, self.here(s.system_address)) else { return };
        let count = s.signals.iter().filter(|x| x.kind == BIOLOGICAL).map(|x| x.count.max(0) as u32).sum::<u32>();
        let b = self.body(id, &s.body_name);
        if count > 0 || dss {
            b.signals = Some(count);
        }
        if dss && !s.genuses.is_empty() {
            b.genera = s.genuses.iter().map(|g| genus_name(g)).collect();
        }
    }

    /// Fold a journal event.
    pub fn apply(&mut self, ev: &JournalEvent) {
        match ev {
            JournalEvent::FsdJump { star_system, system_address, .. } => self.arrive(*system_address, star_system),
            JournalEvent::Location { star_system, system_address, .. } => self.arrive(*system_address, star_system),
            JournalEvent::Extra(ExtraEvent::FssBodySignals(s)) => self.signals(s, false),
            JournalEvent::Extra(ExtraEvent::SaaSignalsFound(s)) => self.signals(s, true),
            JournalEvent::Extra(ExtraEvent::Scan(b)) if self.here(b.system_address) => {
                let Some(id) = b.body_id else { return };
                if let Some(t) = &b.star_type {
                    self.stars.insert(id, catalog::star_class(t));
                } else if let Some(c) = predict::Body::from_scan(b, None) {
                    let body = self.body(id, &b.body_name);
                    body.conditions = Some(c);
                    body.parents = b.parents.clone();
                }
            }
            JournalEvent::Extra(ExtraEvent::ScanOrganic {
                genus,
                genus_localised,
                species,
                species_localised,
                body,
                system_address,
                ..
            }) if self.here(*system_address) => {
                let Some(id) = body else { return };
                let g = catalog::genus(genus)
                    .or_else(|| genus_localised.as_deref().and_then(catalog::genus))
                    .map(|g| g.name.clone())
                    .or_else(|| genus_localised.clone())
                    .unwrap_or_else(|| genus_name(genus));
                let sp = species_localised.clone().unwrap_or_else(|| species.clone());
                self.body(*id, "").species.insert(g, sp);
            }
            _ => {}
        }
    }

    /// The class of the nearest parent star of `b`, if that star was scanned.
    fn star_of(&self, b: &Body) -> Option<String> {
        b.parents.iter().find(|(kind, _)| kind == "Star").and_then(|(_, id)| self.stars.get(id).cloned())
    }

    /// One body's biology, or None if nothing says it has any.
    pub fn body_bio(&self, b: &Body) -> Option<BodyBio> {
        let has = b.signals.is_some_and(|n| n > 0) || !b.genera.is_empty() || !b.species.is_empty();
        if !has {
            return None;
        }
        let candidates: Vec<Candidate> = b
            .conditions
            .as_ref()
            .map(|c| {
                let c = predict::Body { star_class: self.star_of(b), ..c.clone() };
                predict::predict(&c, true)
            })
            .unwrap_or_default();
        let genus_bio = |genus: &str, known: bool| -> GenusBio {
            let cat = catalog::genus(genus);
            let sampled = b.species.get(genus).and_then(|s| catalog::species(s));
            let species: Vec<Candidate> = match sampled {
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
        let mut known: Vec<String> = b.genera.clone();
        for g in b.species.keys() {
            if !known.contains(g) {
                known.push(g.clone());
            }
        }
        let mut genera: Vec<GenusBio> = known.iter().map(|g| genus_bio(g, true)).collect();
        let mut predicted: Vec<GenusBio> = vec![];
        if b.genera.is_empty() {
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
        if b.genera.is_empty() {
            // Before the DSS: the signal count says how many genera there are; the cheapest and
            // dearest that many of the predicted ones bound the value.
            let left = b.signals.unwrap_or(0).saturating_sub(genera.len() as u32) as usize;
            let mut lows: Vec<i64> = predicted.iter().map(|g| g.value.0).collect();
            lows.sort();
            value.0 += lows.iter().take(left).sum::<i64>();
            value.1 += predicted.iter().take(left).map(|g| g.value.1).sum::<i64>();
        }
        genera.extend(predicted);
        Some(BodyBio {
            body_id: b.body_id,
            name: b.name.clone(),
            signals: b.signals,
            mapped: !b.genera.is_empty(),
            genera,
            value,
        })
    }

    /// Every body with biology, most valuable first: by the middle of its value range, so a
    /// body with a sure 19 M ranks above one that might reach 25 M or be worth 5 M.
    pub fn ranked(&self) -> Vec<BodyBio> {
        let mut v: Vec<BodyBio> = self.bodies.values().filter_map(|b| self.body_bio(b)).collect();
        v.sort_by(|a, b| crate::middle(b.value).cmp(&crate::middle(a.value)).then(a.name.cmp(&b.name)));
        v
    }
}
