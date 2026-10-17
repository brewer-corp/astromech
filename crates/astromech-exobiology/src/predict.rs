//! Species prediction from a body's detailed scan: which species could grow there and what
//! they are worth, before anyone lands.
//!
//! A species is a candidate when one of its [`Conditions`] (where it has been seen growing,
//! measured from Spansh's bodies; see `scripts/gen-exobiology.py`) fits the body. Atmosphere
//! type, body class, the nearest parent star's class and the kind of volcanism are hard gates.
//! Gravity, temperature and pressure are soft: inside the observed range the fit is 1, and it
//! falls to 0 over a margin of 15 % of the range's width beyond either edge (of the bound itself
//! when the range is a single value), since the sightings only sample where a species grows. An
//! unknown value doesn't constrain.

use crate::catalog::{self, Conditions};
use serde::Serialize;

/// How far past an observed edge the fit tapers to 0, as a fraction of the range's width.
const MARGIN: f64 = 0.15;
/// Standard gravity (m/s² per g) and the standard atmosphere (Pa), the units of the conditions.
pub const MS2_PER_G: f64 = 9.80665;
pub const PA_PER_ATM: f64 = 101_325.0;

/// What the prediction reads from a body.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Body {
    /// Journal `PlanetClass`: "Rocky body".
    pub planet_class: String,
    /// Journal `AtmosphereType`; "None" or empty for no atmosphere.
    pub atmosphere_type: String,
    /// Journal `Volcanism` text; empty for none.
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
            volcanism: b.volcanism.clone().unwrap_or_default(),
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
    /// How well the body fits, 0 < fit ≤ 1 (1: inside every observed range).
    pub fit: f64,
}

/// The kind of volcanism in a journal `Volcanism` text: its material, without minor/major and
/// geysers/magma: "minor water geysers volcanism" is "water", "Carbon Dioxide Geysers" is
/// "carbon dioxide", "" and "No volcanism" are "none".
pub fn volcanism_kind(text: &str) -> String {
    let words: Vec<String> = text
        .to_lowercase()
        .split_whitespace()
        .filter(|w| !matches!(*w, "minor" | "major" | "geysers" | "magma" | "vapour" | "no" | "volcanism"))
        .map(str::to_string)
        .collect();
    if words.is_empty() {
        "none".into()
    } else {
        words.join(" ")
    }
}

/// 1 inside `[lo, hi]`; past an edge, falling to 0 over [`MARGIN`] of the range's width (of the
/// bound itself for a one-sided or single-value range).
fn range_fit(v: f64, lo: Option<f64>, hi: Option<f64>) -> f64 {
    let (over, edge) = match (lo, hi) {
        (Some(l), _) if v < l => (l - v, l),
        (_, Some(h)) if v > h => (v - h, h),
        _ => return 1.0,
    };
    let width = match (lo, hi) {
        (Some(l), Some(h)) if h > l => h - l,
        _ => edge.abs(),
    };
    let margin = width * MARGIN;
    if margin <= 0.0 || over >= margin {
        0.0
    } else {
        1.0 - over / margin
    }
}

/// How well `body` fits `c`, 0 for not at all.
pub fn fit(c: &Conditions, body: &Body) -> f64 {
    let atmosphere = if body.atmosphere_type.is_empty() { "None" } else { body.atmosphere_type.as_str() };
    let gate = |allowed: &Option<Vec<String>>, v: &str| allowed.as_ref().is_none_or(|a| a.iter().any(|x| x == v));
    if !gate(&c.atmosphere, atmosphere) || !gate(&c.body_types, &body.planet_class) {
        return 0.0;
    }
    if body.star_class.as_deref().is_some_and(|star| !gate(&c.star_types, star)) {
        return 0.0;
    }
    if !gate(&c.volcanism, &volcanism_kind(&body.volcanism)) {
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
pub fn predict(body: &Body) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = catalog::genera()
        .iter()
        .flat_map(|g| g.species.iter().map(move |s| (g, s)))
        .filter_map(|(g, s)| {
            let fit = s.rules.iter().map(|c| fit(c, body)).fold(0.0, f64::max);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn margins_scale_with_the_range_or_the_bound() {
        // Two-sided: 15 % of the width (10 K) is 1.5 K.
        assert_eq!(range_fit(175.0, Some(170.0), Some(180.0)), 1.0);
        assert_eq!(range_fit(180.75, Some(170.0), Some(180.0)), 0.5);
        assert_eq!(range_fit(181.5, Some(170.0), Some(180.0)), 0.0);
        // One-sided pressure of 0.0135 atm: the margin is 15 % of the bound, about 0.002 atm,
        // not a fixed 0.15 atm.
        assert!(range_fit(0.0145, None, Some(0.0135)) > 0.0);
        assert_eq!(range_fit(0.016, None, Some(0.0135)), 0.0);
        assert_eq!(range_fit(0.1, None, Some(0.0135)), 0.0);
        // A single observed value: 15 % of it either way.
        assert!(range_fit(0.105, Some(0.1), Some(0.1)) > 0.0);
        assert_eq!(range_fit(0.12, Some(0.1), Some(0.1)), 0.0);
    }

    #[test]
    fn volcanism_kinds() {
        for (text, kind) in [
            ("", "none"),
            ("No volcanism", "none"),
            ("minor water geysers volcanism", "water"),
            ("Major Water Geysers", "water"),
            ("carbon dioxide geysers volcanism", "carbon dioxide"),
            ("Minor Silicate Vapour Geysers", "silicate"),
            ("metallic magma volcanism", "metallic"),
        ] {
            assert_eq!(volcanism_kind(text), kind, "{text}");
        }
    }
}
