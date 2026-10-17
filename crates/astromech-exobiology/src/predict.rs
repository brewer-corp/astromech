//! Species prediction from a body's detailed scan: which species could grow there and what
//! they are worth, before anyone lands (BioScan-style).
//!
//! A species is a candidate when one of its [`Conditions`] fits the body. Atmosphere, body
//! class, star class and volcanism are hard gates; gravity, temperature and pressure are soft:
//! inside the documented range the fit is 1, and it falls to 0 over a margin of 15 % of the
//! range beyond either edge (the published ranges come from sightings, so the true edge is a
//! little wider). An unknown value doesn't constrain. Rules marked unmodeled (they leave out a
//! region or nebula condition) count only on a body known to have biology.

use crate::catalog::{self, Conditions, Volcanism};
use serde::Serialize;

/// How far past a documented edge the fit tapers to 0, as a fraction of the range.
const MARGIN: f64 = 0.15;
/// m/s² per g and Pa per atmosphere as the condition tables were measured.
pub const MS2_PER_G: f64 = 9.797759;
pub const PA_PER_ATM: f64 = 101_231.656_25;

/// What the prediction reads from a body.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Body {
    /// Journal `PlanetClass`: "Rocky body".
    pub planet_class: String,
    /// Journal `AtmosphereType`; "None" or empty for no atmosphere.
    pub atmosphere_type: String,
    /// Journal `Volcanism` text, lower case; empty for none.
    pub volcanism: String,
    pub temperature_k: Option<f64>,
    pub gravity_g: Option<f64>,
    pub pressure_atm: Option<f64>,
    /// The nearest parent star's class ([`catalog::star_class`]); None if not known.
    pub star_class: Option<String>,
}

impl Body {
    /// From a journal `Scan` of a planet (gravity in m/s², pressure in Pa), with the class of
    /// its nearest parent star if known. None for a star or a body that isn't landable: only
    /// landable bodies with a thin atmosphere grow biology that can be sampled.
    pub fn from_scan(b: &astromech_journal::ScanBody, star_class: Option<String>) -> Option<Self> {
        if !b.landable {
            return None;
        }
        Some(Self {
            planet_class: b.planet_class.clone()?,
            atmosphere_type: b.atmosphere_type.clone().unwrap_or_default(),
            volcanism: b.volcanism.clone().unwrap_or_default().to_lowercase(),
            temperature_k: b.surface_temperature,
            gravity_g: b.surface_gravity.map(|g| g / MS2_PER_G),
            pressure_atm: b.surface_pressure.map(|p| p / PA_PER_ATM),
            star_class,
        })
    }
}

/// A species that could be there.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Candidate {
    pub genus: &'static str,
    pub species: &'static str,
    /// Base value in credits.
    pub value: i64,
    /// How well the body fits, 0 < fit ≤ 1 (1: inside every documented range).
    pub fit: f64,
}

fn range_fit(v: f64, lo: Option<f64>, hi: Option<f64>) -> f64 {
    let (over, span) = match (lo, hi) {
        (Some(l), _) if v < l => (l - v, hi.map_or(l.abs().max(1.0), |h| h - l)),
        (_, Some(h)) if v > h => (v - h, lo.map_or(h.abs().max(1.0), |l| h - l)),
        _ => return 1.0,
    };
    let margin = span * MARGIN;
    if margin <= 0.0 || over >= margin {
        0.0
    } else {
        1.0 - over / margin
    }
}

fn volcanism_fits(rule: Option<&Volcanism>, volcanism: &str) -> bool {
    let has = !volcanism.trim().is_empty() && !volcanism.trim().eq_ignore_ascii_case("no volcanism");
    match rule {
        None => true,
        Some(Volcanism::Word(w)) if w == "any" => has,
        Some(Volcanism::Word(w)) if w == "none" => !has,
        Some(Volcanism::Word(w)) => volcanism.contains(w.as_str()),
        Some(Volcanism::Keywords(ks)) => ks.iter().any(|k| volcanism.contains(k.as_str())),
    }
}

/// How well `body` fits `c`, 0 for not at all.
pub fn fit(c: &Conditions, body: &Body, has_biology: bool) -> f64 {
    if c.unmodeled.is_some() && !has_biology {
        return 0.0;
    }
    let atmosphere = if body.atmosphere_type.is_empty() { "None" } else { body.atmosphere_type.as_str() };
    let gate = |allowed: &Option<Vec<String>>, v: &str| allowed.as_ref().is_none_or(|a| a.iter().any(|x| x == v));
    if !gate(&c.atmosphere, atmosphere) || !gate(&c.body_types, &body.planet_class) {
        return 0.0;
    }
    if let Some(star) = &body.star_class {
        if !gate(&c.star_types, star) {
            return 0.0;
        }
    }
    if !volcanism_fits(c.volcanism.as_ref(), &body.volcanism) {
        return 0.0;
    }
    [
        (body.temperature_k, c.min_temp_k, c.max_temp_k),
        (body.gravity_g, c.min_gravity_g, c.max_gravity_g),
        (body.pressure_atm, c.min_pressure_atm, c.max_pressure_atm),
    ]
    .into_iter()
    .filter_map(|(v, lo, hi)| Some(range_fit(v?, lo, hi)))
    .fold(1.0, f64::min)
}

/// Every species that could grow on `body`, best fit first, then most valuable.
/// `has_biology`: the body is known to have biological signals (allows the unmodeled rules).
pub fn predict(body: &Body, has_biology: bool) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = catalog::genera()
        .iter()
        .flat_map(|g| g.species.iter().map(move |s| (g, s)))
        .filter_map(|(g, s)| {
            let fit = s.rules.iter().map(|c| fit(c, body, has_biology)).fold(0.0, f64::max);
            (fit > 0.0).then_some(Candidate { genus: &g.name, species: &s.name, value: s.value, fit })
        })
        .collect();
    out.sort_by(|a, b| b.fit.total_cmp(&a.fit).then(b.value.cmp(&a.value)));
    out
}

/// The lowest and highest value among `candidates` (0, 0 for none).
pub fn value_range(candidates: &[Candidate]) -> (i64, i64) {
    let v = candidates.iter().map(|c| c.value);
    (v.clone().min().unwrap_or(0), v.max().unwrap_or(0))
}
