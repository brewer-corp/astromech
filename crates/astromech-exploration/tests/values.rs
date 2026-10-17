//! Body values against published estimates for real bodies of every class.
//!
//! `fixtures/edsm-values.json` (source in the file): fifteen systems' bodies with their class,
//! mass and terraform state, and EDSM's published estimates: each system's bodies at their scan
//! value (`estimatedValue`) and with every planet mapped (`estimatedValueMapped`), and each
//! valuable body's mapped value (`valueMax`). EDSM's figures
//! are for a body already discovered and mapped by someone else, without the efficiency or
//! Odyssey bonus, so they are compared on those terms. Between them the systems hold every star
//! value class (main sequence, brown dwarfs, Wolf-Rayet, white dwarfs, a neutron star, a black
//! hole) and every planet value class (metal-rich, high metal content and terraformable,
//! rocky and terraformable, icy, rocky ice, Earth-like, water world and terraformable, ammonia
//! world, gas giants of class I, II, III, V and with life).

use astromech_exploration::value::{body_value, Terms};
use astromech_journal::ScanBody;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    systems: Vec<System>,
}

#[derive(Deserialize)]
struct System {
    system: String,
    edsm_total: Option<i64>,
    edsm_total_mapped: Option<i64>,
    edsm_mapped: Vec<(String, i64)>,
    bodies: Vec<Body>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Body {
    body_name: String,
    star_type: Option<String>,
    stellar_mass: Option<f64>,
    planet_class: Option<String>,
    #[serde(rename = "MassEM")]
    mass_em: Option<f64>,
    terraform_state: Option<String>,
}

impl Body {
    fn scan(&self) -> ScanBody {
        ScanBody {
            scan_type: "Detailed".into(),
            body_name: self.body_name.clone(),
            star_type: self.star_type.clone(),
            stellar_mass: self.stellar_mass,
            planet_class: self.planet_class.clone(),
            mass_em: self.mass_em,
            terraform_state: self.terraform_state.clone(),
            was_discovered: Some(true),
            was_mapped: Some(true),
            ..Default::default()
        }
    }
}

const EDSM: Terms = Terms { first_discovery: false, first_mapped: false, mapping: None, odyssey: false };

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("fixtures/edsm-values.json")).unwrap()
}

#[test]
fn every_valuable_body_matches_edsm_mapped_value() {
    let mut checked = 0;
    for s in fixture().systems {
        for (name, published) in &s.edsm_mapped {
            let b = s.bodies.iter().find(|b| &b.body_name == name).unwrap_or_else(|| panic!("{name} not in fixture"));
            let ours = body_value(&b.scan(), EDSM.mapped(false)).unwrap();
            assert!(
                (ours - published).abs() <= 1,
                "{} {name} ({:?}): ours {ours}, EDSM {published}",
                s.system,
                b.planet_class
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 20);
}

#[test]
fn every_system_total_matches_edsm_scan_value() {
    let mut checked = 0;
    for s in fixture().systems {
        let Some(published) = s.edsm_total else { continue };
        let ours: i64 = s.bodies.iter().map(|b| body_value(&b.scan(), EDSM).unwrap()).sum();
        // EDSM rounds each body; the sum of our rounded values may differ by a credit or two.
        assert!((ours - published).abs() <= 2, "{}: ours {ours}, EDSM {published}", s.system);
        checked += 1;
    }
    assert_eq!(checked, 14);
}

/// Small planets once mapped: the 500 cr minimum applies after the mapping multiplier, not
/// before (EDDI's `Body.cs` `estimateBodyValue` does this; EDDiscovery floors first). EDSM's
/// `estimatedValueMapped` decides it: these systems hold over a hundred rocky, icy and rocky ice
/// bodies under the floor, and flooring after mapping matches every total exactly, while
/// flooring first comes out high in each (Togher by 13,859, HIP 34707 by 5,051).
#[test]
fn every_system_mapped_total_matches_edsm() {
    let mut checked = 0;
    for s in fixture().systems {
        // Sol's EDSM body list differs from Spansh's (no scan total either).
        let Some(published) = s.edsm_total_mapped else { continue };
        let ours: i64 = s
            .bodies
            .iter()
            .map(|b| {
                let terms = if b.planet_class.is_some() { EDSM.mapped(false) } else { EDSM };
                body_value(&b.scan(), terms).unwrap()
            })
            .sum();
        assert!((ours - published).abs() <= 2, "{}: ours {ours}, EDSM {published}", s.system);
        checked += 1;
    }
    assert_eq!(checked, 14);
}

/// A mapped 1 Earth-mass icy body: `300 + 300·q` is 470, under the floor, but mapped it is
/// 470 × 10/3 = 1566 on EDSM's terms (the rule `every_system_mapped_total_matches_edsm` checks),
/// and 1566 + 555 = 2121 with the Odyssey bonus, as EDDI gives. Flooring first gave 1667 and 2222.
#[test]
fn a_mapped_small_icy_body_is_floored_after_mapping() {
    use astromech_exploration::value::planet_value;
    assert_eq!(planet_value("Icy body", false, Some(1.0), EDSM), 500);
    assert_eq!(planet_value("Icy body", false, Some(1.0), EDSM.mapped(false)), 1566);
    let live = Terms { odyssey: true, ..EDSM };
    assert_eq!(planet_value("Icy body", false, Some(1.0), live.mapped(false)), 2121);
}

/// The fixture covers every value class (the test is only as good as its examples).
#[test]
fn the_examples_cover_every_value_class() {
    use astromech_exploration::value::{planet_k, star_k};
    let f = fixture();
    let bodies: Vec<&Body> = f.systems.iter().flat_map(|s| &s.bodies).collect();
    let star_ks: std::collections::BTreeSet<i64> =
        bodies.iter().filter_map(|b| b.star_type.as_deref()).map(|t| star_k(t) as i64).collect();
    assert_eq!(star_ks.into_iter().collect::<Vec<_>>(), [1200, 14057, 22628]);
    let tf = |b: &&&Body| b.terraform_state.as_deref().is_some_and(|t| !t.is_empty());
    let planet_ks: std::collections::BTreeSet<(i64, bool)> = bodies
        .iter()
        .filter(|b| b.planet_class.is_some())
        .map(|b| (planet_k(b.planet_class.as_deref().unwrap(), tf(&b)) as i64, tf(&b)))
        .collect();
    for want in [
        (21790, false),         // metal-rich
        (9654, false),          // high metal content, class II gas giant
        (9654 + 100677, true),  // terraformable high metal content
        (64831, false),         // water world
        (64831 + 116295, true), // terraformable water world, Earth-like world
        (96932, false),         // ammonia world
        (1656, false),          // class I gas giant
        (300, false),           // rocky, icy, rocky ice, other gas giants
        (300 + 93328, true),    // terraformable rocky
    ] {
        assert!(planet_ks.contains(&want), "no example for k {want:?}");
    }
}

/// First discovery and first mapping, against a published worked example: ED Scout's test of its
/// appraiser (github.com/joncage/ed-scout, `EDScoutCore/test_BodyAppraiser.py`, MIT): a
/// terraformable high metal content world of 0.18679 Earth masses, first discovered and first
/// mapped, is worth 1,490,667 ± 15,000 (a Horizons-era figure: no Odyssey or efficiency bonus).
#[test]
fn first_discovered_and_mapped_matches_the_published_example() {
    let b = ScanBody {
        body_name: "Pro Eurl TE-H d10-15 6".into(),
        planet_class: Some("High metal content body".into()),
        mass_em: Some(0.18679),
        terraform_state: Some("Terraformable".into()),
        was_discovered: Some(false),
        was_mapped: Some(false),
        ..Default::default()
    };
    let horizons = Terms { first_discovery: true, first_mapped: true, mapping: None, odyssey: false }.mapped(false);
    let v = body_value(&b, horizons).unwrap();
    assert!((v - 1_490_667).abs() <= 15_000, "{v}");
    // Since Odyssey: +30% for mapping, ×1.25 within the efficiency target.
    let odyssey = Terms { odyssey: true, ..horizons };
    assert_eq!(body_value(&b, odyssey).unwrap(), (v as f64 * 1.3).round() as i64);
    assert_eq!(body_value(&b, odyssey.mapped(true)).unwrap(), (v as f64 * 1.3 * 1.25).round() as i64);
    // Only scanned, first discovered: ×2.6 of the scan value.
    let scanned = body_value(&b, Terms::from_scan(&b)).unwrap();
    let plain = body_value(&b, EDSM).unwrap();
    assert!((scanned as f64 / plain as f64 - 2.6).abs() < 1e-4);
}

/// Discovered by someone else but never mapped pays the first-mapper multiplier, and a star's
/// first discovery is ×2.6.
#[test]
fn first_mapped_only_and_stars() {
    use astromech_exploration::value::{planet_value, star_value};
    let t = Terms { first_discovery: false, first_mapped: true, mapping: None, odyssey: false };
    let base = planet_value("Water world", false, Some(1.0), Terms { first_mapped: false, ..t });
    let mapped = planet_value("Water world", false, Some(1.0), t.mapped(false));
    assert!((mapped as f64 / base as f64 - 8.0956).abs() < 1e-4, "{mapped} / {base}");
    assert_eq!(star_value("G", Some(1.0), false), 1218);
    assert_eq!(star_value("G", Some(1.0), true), 3167);
}

/// A planet the journal calls undiscovered but already mapped is in the pre-Odyssey populated
/// bubble. EDDI (`DataDefinitions/Body.cs`, `estimateBodyValue`, `isPreOdysseyBubble =
/// isFirstDiscoverer && !isFirstMapper`) values it as a discovered, mapped body: the 10/3
/// multiplier, no Odyssey 30% and no x2.6 first-discovery bonus; the efficiency bonus stays.
#[test]
fn undiscovered_but_mapped_bodies_get_no_discovery_bonuses() {
    use astromech_exploration::value::planet_value;
    let b = ScanBody {
        body_name: "Bubble 1".into(),
        planet_class: Some("Water world".into()),
        mass_em: Some(1.0),
        was_discovered: Some(false),
        was_mapped: Some(true),
        ..Default::default()
    };
    let terms = Terms::from_scan(&b);
    assert!(terms.first_discovery && !terms.first_mapped);
    let base = planet_value("Water world", false, Some(1.0), EDSM);
    // EDDI: (k + k q m^0.2) x 3.3333333333 [x 1.25 if efficient].
    let eddi_mapped = (base as f64 * 3.333_333_333_3).round() as i64;
    assert!((body_value(&b, terms.mapped(false)).unwrap() - eddi_mapped).abs() <= 1);
    let eddi_efficient = (base as f64 * 3.333_333_333_3 * 1.25).round() as i64;
    assert!((body_value(&b, terms.mapped(true)).unwrap() - eddi_efficient).abs() <= 1);
    // Scanned only: no x2.6 either.
    assert_eq!(body_value(&b, terms).unwrap(), base);
    // A star in the same state keeps its first-discovery bonus (EDDI's star value has no such rule).
    use astromech_exploration::value::star_value;
    assert_eq!(star_value("G", Some(1.0), true), 3167);
}
