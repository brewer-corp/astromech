//! The sampling helper: the species being sampled, where each sample was taken, and how far the
//! commander now is from them against the genus's colony range.
//!
//! The genetic sampler writes `ScanOrganic`: `Log` for the first sample of a species, `Sample`
//! for the next, `Analyse` once the third is done. A species needs three samples, each at least
//! the genus's colony range from the others. The journal line has no position, so a sample is
//! placed at the last position Status.json gave (`Latitude`, `Longitude`, `PlanetRadius`).
//! Logging another species starts over (the sampler holds one species at a time), and so does a
//! jump.

use crate::catalog;
use crate::surface::{self, LatLon};
use astromech_journal::{ExtraEvent, JournalEvent};
use astromech_status::StatusSnap;
use serde::Serialize;

/// Where the commander is on a body, from Status.json.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Position {
    pub lat: f64,
    pub lon: f64,
    /// The body's radius in metres, when Status.json gives it.
    pub radius_m: Option<f64>,
}

/// One species being sampled.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Progress {
    pub system_address: Option<i64>,
    pub body_id: Option<i64>,
    /// Catalogue genus name ("Stratum"), or the journal's if the catalogue doesn't know it.
    pub genus: String,
    pub species: String,
    /// Samples taken, 1 to 3.
    pub count: u8,
    /// Where each sample was taken (None: no position was known then).
    pub at: Vec<Option<Position>>,
    /// The genus's colony range in metres, if known.
    pub colony_range_m: Option<u32>,
    /// `Analyse`: the three samples are done.
    pub complete: bool,
}

/// How far the commander is from the samples taken so far.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Clearance {
    /// The next sample's number (2 or 3).
    pub next: u8,
    /// Distance to the nearest earlier sample, metres.
    pub nearest_m: f64,
    /// Bearing from here to that sample, degrees.
    pub bearing_deg: f64,
    pub colony_range_m: u32,
    /// Every earlier sample is at least the colony range away.
    pub far_enough: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Sampling {
    /// The last position Status.json gave; None away from a body's surface.
    pub position: Option<Position>,
    /// The species being sampled (kept once complete until the next one starts).
    pub current: Option<Progress>,
}

impl Sampling {
    pub fn apply_status(&mut self, s: &StatusSnap) {
        self.position = s.lat_lon().map(|(lat, lon)| Position { lat, lon, radius_m: s.planet_radius });
    }

    pub fn apply(&mut self, ev: &JournalEvent) {
        match ev {
            JournalEvent::FsdJump { .. } => self.current = None,
            JournalEvent::Extra(ExtraEvent::ScanOrganic {
                scan_type,
                genus,
                genus_localised,
                species,
                species_localised,
                body,
                system_address,
                ..
            }) => {
                let species = species_localised.clone().unwrap_or_else(|| species.clone());
                let same = self.current.as_ref().is_some_and(|p| {
                    p.species == species && p.body_id == *body && p.system_address == *system_address && !p.complete
                });
                let kind = scan_type.as_str();
                if kind == "Log" || !same {
                    let g = catalog::genus(genus).or_else(|| genus_localised.as_deref().and_then(catalog::genus));
                    // A `Sample` without its `Log` (the log started mid-way): at least the second.
                    let count = match kind {
                        "Log" => 1,
                        "Analyse" => 3,
                        _ => 2,
                    };
                    let mut at = vec![None; count as usize - 1];
                    at.push(self.position);
                    self.current = Some(Progress {
                        system_address: *system_address,
                        body_id: *body,
                        genus: g.map(|g| g.name.clone()).or_else(|| genus_localised.clone()).unwrap_or(genus.clone()),
                        species,
                        count,
                        at,
                        colony_range_m: g.map(|g| g.colony_range_m),
                        complete: kind == "Analyse",
                    });
                    return;
                }
                let Some(p) = self.current.as_mut() else { return };
                match kind {
                    "Sample" if p.count < 3 => {
                        p.count += 1;
                        p.at.push(self.position);
                    }
                    "Analyse" => {
                        p.count = 3;
                        p.complete = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// How far the commander is from the samples of the species in progress, if one is in
    /// progress, the position and radius are known, and so is every earlier sample's place.
    pub fn clearance(&self) -> Option<Clearance> {
        let p = self.current.as_ref().filter(|p| !p.complete && p.count < 3)?;
        let here = self.position?;
        let radius = here.radius_m?;
        let range = p.colony_range_m?;
        let from = LatLon::new(here.lat, here.lon);
        let nearest =
            p.at.iter()
                .map(|a| a.map(|a| LatLon::new(a.lat, a.lon)))
                .collect::<Option<Vec<_>>>()?
                .into_iter()
                .map(|a| surface::guidance(from, a, radius))
                .min_by(|a, b| a.distance_m.total_cmp(&b.distance_m))?;
        Some(Clearance {
            next: p.count + 1,
            nearest_m: nearest.distance_m,
            bearing_deg: nearest.bearing_deg,
            colony_range_m: range,
            far_enough: nearest.distance_m >= f64::from(range),
        })
    }
}
