//! What Universal Cartographics pays for a body: the community's exploration value formulae.
//!
//! Source: MattG's "Exploration value formulae" thread on the Frontier forums
//! (forums.frontier.co.uk/threads/exploration-value-formulae.232000/, values since 3.3), as
//! implemented in EDDiscovery's `EstimatedValues.cs` (Apache-2.0) and EDDI's `Body.cs`
//! (Apache-2.0). Written from the formulae, not copied. Checked against EDSM's published
//! estimates for real bodies of every class (`tests/values.rs`).
//!
//! - A star is worth `k + m·k / 66.25` (`m` in solar masses): `k` is 1200, 14057 for a white
//!   dwarf, 22628 for a neutron star or black hole, 33.5678 for the galactic centre's black hole.
//! - A planet is worth `max(500, k + k·q·m^0.2)` with `q = 0.56591828` (`m` in Earth masses).
//!   `k` by class: metal-rich 21790 (+65631 terraformable), high metal content and class II gas
//!   giant 9654 (+100677), water world 64831 (+116295), Earth-like world 64831 + 116295 (always),
//!   ammonia world 96932, class I gas giant 1656, everything else 300 (+93328). Terraformable
//!   means any `TerraformState` (`Terraformable`, `Terraforming`, `Terraformed`).
//! - Mapping (the Detailed Surface Scanner) multiplies a planet by 3.699622554 when it was
//!   neither discovered nor mapped before, 8.0956 when it was discovered but not mapped, and
//!   10/3 otherwise; since Odyssey the mapped value then gains 30% (at least 555 credits), and
//!   mapping within the efficiency target another 25%.
//! - Being the first to discover a body (`WasDiscovered: false`) multiplies the result by 2.6.
//! - Except a planet that was undiscovered yet already mapped (`WasMapped: true`): the old
//!   populated bubble, where EDDI's `Body.cs` (`isPreOdysseyBubble`) gives neither the 2.6 nor
//!   the Odyssey 30%. It is valued as discovered and mapped (10/3), the efficiency bonus kept.
//!   Stars have no such rule.
//!
//! What it doesn't know: whether a body really is the commander's first discovery (the journal
//! says only whether someone else had it before the scan), selling through a fleet carrier
//! (25% less) and the Legacy-mode rules. Every journal astromech reads is from the live game,
//! so the Odyssey bonus always applies.

use astromech_journal::ScanBody;

const STAR_DIVIDER: f64 = 66.25;
const PLANET_Q: f64 = 0.565_918_28;
const PLANET_POWER: f64 = 0.2;
const MIN_PLANET: f64 = 500.0;
const FIRST_DISCOVERY: f64 = 2.6;
const MAP_FIRST_DISCOVERED_AND_MAPPED: f64 = 3.699_622_554;
const MAP_FIRST_MAPPED: f64 = 8.0956;
const MAP_ALREADY_MAPPED: f64 = 10.0 / 3.0;
const EFFICIENT: f64 = 1.25;

/// A planet worth this much once mapped (efficiently) is worth flying to with the probes.
pub const WORTH_MAPPING: i64 = 500_000;

/// How a body was or would be mapped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mapping {
    /// Within the efficiency target (`ProbesUsed <= EfficiencyTarget`).
    pub efficient: bool,
}

/// What is known about a body's history and the scan being valued.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Terms {
    /// Nobody had discovered it before (`WasDiscovered: false`).
    pub first_discovery: bool,
    /// Nobody had mapped it before (`WasMapped: false`).
    pub first_mapped: bool,
    /// Mapped, or valued as if it were.
    pub mapping: Option<Mapping>,
    /// The Odyssey bonus on mapping (all live-game sales). Off only to compare with estimates
    /// made without it.
    pub odyssey: bool,
}

impl Terms {
    /// The terms the journal gives for a scan, not mapped.
    pub fn from_scan(b: &ScanBody) -> Self {
        Self {
            first_discovery: b.was_discovered == Some(false),
            first_mapped: b.was_mapped == Some(false),
            mapping: None,
            odyssey: true,
        }
    }

    /// A planet the journal calls undiscovered but already mapped lies in the pre-Odyssey
    /// populated bubble, where EDDI (`Body.cs`, `isPreOdysseyBubble = isFirstDiscoverer &&
    /// !isFirstMapper`) gives neither the first-discovery x2.6 nor the Odyssey 30%.
    pub fn pre_odyssey_bubble(self) -> bool {
        self.first_discovery && !self.first_mapped
    }

    pub fn mapped(self, efficient: bool) -> Self {
        Self { mapping: Some(Mapping { efficient }), ..self }
    }
}

/// `k` for a star type (`StarType`).
pub fn star_k(star_type: &str) -> f64 {
    match star_type {
        "N" | "H" => 22628.0,
        "SupermassiveBlackHole" => 33.5678,
        t if is_white_dwarf(t) => 14057.0,
        _ => 1200.0,
    }
}

/// `D`, `DA`, `DAB`, ..., `DX`: the white dwarf types.
pub fn is_white_dwarf(star_type: &str) -> bool {
    star_type.starts_with('D') && star_type.len() <= 3
}

/// A star's value (`StellarMass` in solar masses; 1 if the scan didn't say).
pub fn star_value(star_type: &str, solar_mass: Option<f64>, first_discovery: bool) -> i64 {
    let k = star_k(star_type);
    let v = k + solar_mass.unwrap_or(1.0) * k / STAR_DIVIDER;
    (v * if first_discovery { FIRST_DISCOVERY } else { 1.0 }).round() as i64
}

/// `k` for a planet class (`PlanetClass`), with the terraforming bonus when it applies.
pub fn planet_k(planet_class: &str, terraformable: bool) -> f64 {
    let (k, bonus) = match planet_class {
        "Metal rich body" => (21790.0, 65631.0),
        "High metal content body" | "Sudarsky class II gas giant" => (9654.0, 100677.0),
        "Water world" => (64831.0, 116295.0),
        // Always valued as terraformable.
        "Earthlike body" => return 64831.0 + 116295.0,
        "Ammonia world" => (96932.0, 0.0),
        "Sudarsky class I gas giant" => (1656.0, 0.0),
        _ => (300.0, 93328.0),
    };
    if terraformable {
        k + bonus
    } else {
        k
    }
}

/// A planet's value (`MassEM` in Earth masses; 1 if the scan didn't say).
pub fn planet_value(planet_class: &str, terraformable: bool, mass_em: Option<f64>, terms: Terms) -> i64 {
    let k = planet_k(planet_class, terraformable);
    let base = (k + k * PLANET_Q * mass_em.unwrap_or(1.0).powf(PLANET_POWER)).max(MIN_PLANET);
    let mut v = base;
    let bubble = terms.pre_odyssey_bubble();
    if let Some(m) = terms.mapping {
        let mult = if terms.first_discovery && terms.first_mapped {
            MAP_FIRST_DISCOVERED_AND_MAPPED
        } else if terms.first_mapped {
            MAP_FIRST_MAPPED
        } else {
            MAP_ALREADY_MAPPED
        };
        v *= mult;
        if terms.odyssey && !bubble {
            v += (v * 0.3).max(555.0);
        }
        if m.efficient {
            v *= EFFICIENT;
        }
    }
    if terms.first_discovery && !bubble {
        v *= FIRST_DISCOVERY;
    }
    v.round() as i64
}

/// Any `TerraformState` but none counts: `Terraformable`, `Terraforming`, `Terraformed`.
pub fn terraformable(b: &ScanBody) -> bool {
    b.terraform_state.as_deref().is_some_and(|t| !t.is_empty() && !t.eq_ignore_ascii_case("Not terraformable"))
}

/// A scanned body's value on the terms given; `None` for belt clusters and other scans that are
/// neither a star nor a planet.
pub fn body_value(b: &ScanBody, terms: Terms) -> Option<i64> {
    if let Some(t) = &b.star_type {
        return Some(star_value(t, b.stellar_mass, terms.first_discovery));
    }
    let class = b.planet_class.as_deref().filter(|c| !c.is_empty())?;
    Some(planet_value(class, terraformable(b), b.mass_em, terms))
}

/// What a scanned body is worth now (`mapped`: the commander mapped it, efficiently or not) and
/// at most (mapped efficiently; a star's value doesn't change).
pub fn estimate(b: &ScanBody, mapped: Option<bool>) -> Option<(i64, i64)> {
    let terms = Terms::from_scan(b);
    let now = body_value(
        b,
        match mapped {
            Some(efficient) => terms.mapped(efficient),
            None => terms,
        },
    )?;
    let best = if b.star_type.is_some() { now } else { body_value(b, terms.mapped(true))?.max(now) };
    Some((now, best))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_dwarfs_are_the_d_types() {
        for t in ["D", "DA", "DAB", "DAO", "DAZ", "DAV", "DB", "DBZ", "DBV", "DO", "DOV", "DQ", "DC", "DCV", "DX"] {
            assert!(is_white_dwarf(t), "{t}");
        }
        for t in ["G", "M_RedGiant", "DAVID"] {
            assert!(!is_white_dwarf(t), "{t}");
        }
    }

    #[test]
    fn tiny_bodies_are_worth_at_least_500() {
        let t = Terms { first_discovery: false, first_mapped: false, mapping: None, odyssey: true };
        assert_eq!(planet_value("Icy body", false, Some(0.000_01), t), 500);
        // The floor applies before mapping: 500 × 10/3, plus the Odyssey 555.
        assert_eq!(planet_value("Icy body", false, Some(0.000_01), t.mapped(false)), 1667 + 555);
    }
}
