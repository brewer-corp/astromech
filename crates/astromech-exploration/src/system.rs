//! The live system view: the current system's bodies as they are scanned, with what each is
//! worth ([`crate::value`]).
//!
//! Folded from the journal: a jump (`FSDJump`, `CarrierJump`) or a login (`Location`) into
//! another system starts afresh. Then `FSSDiscoveryScan` (the honk: how many bodies there are),
//! `Scan` (a star or planet, from the FSS, the DSS, a nav beacon or the automatic scans on
//! arrival; belt clusters are left out), `FSSBodySignals` and `SAASignalsFound` (biological
//! and geological signals), `SAAScanComplete` (mapped, and whether within the efficiency
//! target) and `FSSAllBodiesFound`.
//!
//! "Worth mapping" is a planet not mapped yet whose value mapped efficiently is at least
//! [`WORTH_MAPPING`](crate::value::WORTH_MAPPING). "First footfall likely" is a landable body
//! nobody had discovered (`WasDiscovered: false`), in a system with no population, that the
//! scan doesn't say someone has walked on (`WasFootfalled`). The journal never says who walked
//! first, so it is only a hint.

use crate::value;
use astromech_journal::{ExtraEvent as X, JournalEvent as J, JournalLine, ScanBody};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

/// Standard gravity, m/s²: the journal's `SurfaceGravity` over this is in g.
const G: f64 = 9.80665;

/// One scanned body as the view shows it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BodyView {
    pub name: String,
    /// The name without the system's (`A 2` in `Sol A 2`); the whole name when it isn't prefixed.
    pub short_name: String,
    pub body_id: Option<i64>,
    pub star: bool,
    /// The journal's class (`StarType` or `PlanetClass`) and how it reads ("Earth-like world").
    pub class: String,
    pub class_name: String,
    pub terraformable: bool,
    pub landable: bool,
    pub rings: bool,
    /// Surface gravity in g (planets).
    pub gravity_g: Option<f64>,
    pub distance_ls: Option<f64>,
    /// Someone had discovered / mapped / walked on it before (`None`: the scan didn't say).
    pub was_discovered: Option<bool>,
    pub was_mapped: Option<bool>,
    pub was_footfalled: Option<bool>,
    /// The commander mapped it, and whether within the efficiency target.
    pub mapped: bool,
    pub mapped_efficiently: bool,
    pub bio_signals: i64,
    pub geo_signals: i64,
    /// Estimated value now (scanned, and mapped if it was) and mapped efficiently.
    pub value: i64,
    pub value_mapped: i64,
    pub first_discovery: bool,
    pub worth_mapping: bool,
    pub first_footfall_likely: bool,
    /// The commander's landings here before, from the history (the view's reader fills it in).
    pub landings: u32,
}

/// The current system and its bodies.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct SystemView {
    pub system: Option<String>,
    pub system_address: Option<i64>,
    pub population: i64,
    /// From the honk (`FSSDiscoveryScan` `BodyCount`), if the commander honked here.
    pub body_count: Option<i64>,
    /// `FSSAllBodiesFound`.
    pub all_found: bool,
    pub bodies: Vec<BodyView>,
    /// The bodies' value now, and with every planet worth mapping mapped efficiently.
    pub value: i64,
    pub value_if_mapped: i64,
}

#[derive(Debug, Clone, Default)]
struct Body {
    scan: Option<ScanBody>,
    /// Mapped by the commander: within the efficiency target or not.
    mapped: Option<bool>,
    bio: i64,
    geo: i64,
}

/// The fold behind [`SystemView`].
#[derive(Debug, Clone, Default)]
pub struct SystemFold {
    system: Option<String>,
    address: Option<i64>,
    population: i64,
    body_count: Option<i64>,
    all_found: bool,
    bodies: BTreeMap<String, Body>,
}

impl SystemFold {
    fn enter(&mut self, name: Option<&str>, address: Option<i64>, population: Option<i64>) {
        let same = match (address, self.address) {
            (Some(a), Some(b)) => a == b,
            _ => name.is_some() && name == self.system.as_deref(),
        };
        if !same {
            *self = Self::default();
        }
        if let Some(n) = name {
            self.system = Some(n.to_string());
        }
        self.address = address.or(self.address);
        if let Some(p) = population {
            self.population = p;
        }
    }

    /// An event of the current system, or of one the fold hasn't seen arriving in.
    fn here(&mut self, address: Option<i64>) {
        if address.is_some() && self.address.is_some() && address != self.address {
            *self = Self::default();
        }
        self.address = self.address.or(address);
    }

    pub fn apply(&mut self, line: &JournalLine) {
        match &line.event {
            J::FsdJump { star_system, system_address, population, .. } => {
                self.enter(Some(star_system), *system_address, Some(*population))
            }
            J::Location { star_system, system_address, population, .. } => {
                self.enter(Some(star_system), *system_address, Some(*population))
            }
            J::Extra(x) => match x {
                X::FssDiscoveryScan { body_count, system_address, system_name, .. } => {
                    self.here(*system_address);
                    if self.system.is_none() {
                        self.system = system_name.clone();
                    }
                    self.body_count = Some(*body_count);
                }
                X::FssAllBodiesFound { system_address, .. } => {
                    self.here(*system_address);
                    self.all_found = true;
                }
                X::Scan(b) => {
                    let is_body = b.star_type.is_some() || b.planet_class.as_deref().is_some_and(|c| !c.is_empty());
                    if !is_body {
                        return;
                    }
                    self.here(b.system_address);
                    if self.system.is_none() {
                        self.system = b.star_system.clone();
                    }
                    self.bodies.entry(b.body_name.clone()).or_default().scan = Some((**b).clone());
                }
                X::FssBodySignals(s) | X::SaaSignalsFound(s) => {
                    self.here(s.system_address);
                    let body = self.bodies.entry(s.body_name.clone()).or_default();
                    let count = |kind: &str| s.signals.iter().filter(|c| c.kind.contains(kind)).map(|c| c.count).sum();
                    body.bio = count("Biological");
                    body.geo = count("Geological");
                }
                X::SaaScanComplete { body_name, system_address, probes_used, efficiency_target, .. } => {
                    self.here(*system_address);
                    let efficient = *efficiency_target > 0 && probes_used <= efficiency_target;
                    self.bodies.entry(body_name.clone()).or_default().mapped = Some(efficient);
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn view(&self) -> SystemView {
        let mut bodies: Vec<BodyView> =
            self.bodies.values().filter_map(|b| body_view(b, self.system.as_deref(), self.population)).collect();
        bodies.sort_by(|a, b| {
            a.body_id.unwrap_or(i64::MAX).cmp(&b.body_id.unwrap_or(i64::MAX)).then(a.name.cmp(&b.name))
        });
        let value = bodies.iter().map(|b| b.value).sum();
        let value_if_mapped = bodies.iter().map(|b| if b.worth_mapping { b.value_mapped } else { b.value }).sum();
        SystemView {
            system: self.system.clone(),
            system_address: self.address,
            population: self.population,
            body_count: self.body_count,
            all_found: self.all_found,
            bodies,
            value,
            value_if_mapped,
        }
    }
}

fn body_view(b: &Body, system: Option<&str>, population: i64) -> Option<BodyView> {
    let s = b.scan.as_ref()?;
    let (value, value_mapped) = value::estimate(s, b.mapped)?;
    let star = s.star_type.is_some();
    let class = s.star_type.clone().or_else(|| s.planet_class.clone()).unwrap_or_default();
    let terraformable = !star && value::terraformable(s);
    let short_name = system
        .and_then(|sys| s.body_name.strip_prefix(sys))
        .and_then(|rest| rest.strip_prefix(' '))
        .filter(|r| !r.is_empty())
        .unwrap_or(&s.body_name)
        .to_string();
    let first_discovery = s.was_discovered == Some(false);
    Some(BodyView {
        name: s.body_name.clone(),
        short_name,
        body_id: s.body_id,
        star,
        class_name: class_name(&class, s.subclass),
        class,
        terraformable,
        landable: s.landable,
        rings: s.ring_count > 0,
        gravity_g: if star { None } else { s.surface_gravity.map(|g| g / G) },
        distance_ls: s.distance_from_arrival_ls,
        was_discovered: s.was_discovered,
        was_mapped: s.was_mapped,
        was_footfalled: s.was_footfalled,
        mapped: b.mapped.is_some(),
        mapped_efficiently: b.mapped == Some(true),
        bio_signals: b.bio,
        geo_signals: b.geo,
        value,
        value_mapped,
        first_discovery,
        worth_mapping: !star && b.mapped.is_none() && value_mapped >= value::WORTH_MAPPING,
        first_footfall_likely: s.landable && first_discovery && population == 0 && s.was_footfalled != Some(true),
        landings: 0,
    })
}

/// How a journal class reads: "Earth-like world", "Class II gas giant", "K star", "White dwarf
/// (DA)". Unknown classes read as the journal wrote them.
pub fn class_name(class: &str, subclass: Option<i64>) -> String {
    let fixed = match class {
        "Metal rich body" => "Metal-rich body",
        "High metal content body" => "High metal content world",
        "Rocky ice body" => "Rocky ice world",
        "Earthlike body" => "Earth-like world",
        "Gas giant with water based life" => "Gas giant with water-based life",
        "Gas giant with ammonia based life" => "Gas giant with ammonia-based life",
        "Sudarsky class I gas giant" => "Class I gas giant",
        "Sudarsky class II gas giant" => "Class II gas giant",
        "Sudarsky class III gas giant" => "Class III gas giant",
        "Sudarsky class IV gas giant" => "Class IV gas giant",
        "Sudarsky class V gas giant" => "Class V gas giant",
        "Helium rich gas giant" => "Helium-rich gas giant",
        "N" => "Neutron star",
        "H" => "Black hole",
        "SupermassiveBlackHole" => "Supermassive black hole",
        "TTS" => "T Tauri star",
        "AeBe" => "Herbig Ae/Be star",
        "A_BlueWhiteSuperGiant" => "A supergiant",
        "B_BlueWhiteSuperGiant" => "B supergiant",
        "F_WhiteSuperGiant" => "F supergiant",
        "G_WhiteSuperGiant" => "G supergiant",
        "M_RedSuperGiant" => "M supergiant",
        "M_RedGiant" => "M giant",
        "K_OrangeGiant" => "K giant",
        "X" => "Exotic star",
        "RoguePlanet" => "Rogue planet",
        "Nebula" | "StellarRemnantNebula" => "Nebula",
        "L" | "T" | "Y" => return format!("Brown dwarf ({class})"),
        "O" | "B" | "A" | "F" | "G" | "K" | "M" => {
            return format!("{class}{} star", subclass.map(|s| s.to_string()).unwrap_or_default())
        }
        c if value::is_white_dwarf(c) => return format!("White dwarf ({c})"),
        c if c.starts_with('W') => "Wolf-Rayet star",
        "MS" | "S" => "S-type star",
        c if c.starts_with('C') => "Carbon star",
        c => c,
    };
    fixed.to_string()
}

/// The current system from the newest journal and the one before it (a continuation part may
/// start mid-system).
pub fn from_journal_dir(dir: &Path) -> SystemView {
    let logs = astromech_journal::journal_logs(dir);
    let mut f = SystemFold::default();
    for p in logs.iter().skip(logs.len().saturating_sub(2)) {
        for l in astromech_journal::parse_file(p) {
            f.apply(&l);
        }
    }
    f.view()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_names() {
        assert_eq!(class_name("Earthlike body", None), "Earth-like world");
        assert_eq!(class_name("Sudarsky class II gas giant", None), "Class II gas giant");
        assert_eq!(class_name("G", Some(2)), "G2 star");
        assert_eq!(class_name("DAB", Some(5)), "White dwarf (DAB)");
        assert_eq!(class_name("WNC", None), "Wolf-Rayet star");
        assert_eq!(class_name("Y", None), "Brown dwarf (Y)");
        assert_eq!(class_name("Something new", None), "Something new");
    }
}
