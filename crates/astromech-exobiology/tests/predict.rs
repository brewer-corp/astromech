//! Species prediction against bodies whose species are known, and each condition's gate.
//!
//! `fixtures/spansh-held-out.json` is written by `scripts/gen-exobiology.py`: bodies from
//! Spansh's bodies search with the species players sampled there, from the quarter of the
//! sample (by a hash of the id64) held out of the conditions in `data/catalog/exobiology.json`. So the
//! recall measured here is on bodies the conditions never saw.

use astromech_exobiology::catalog::{self, Conditions};
use astromech_exobiology::predict::{self, Body};
use serde_json::Value;

const HELD_OUT: &str = include_str!("fixtures/spansh-held-out.json");

/// A held-out body in the journal's terms (the fixture is already converted).
fn body(b: &Value) -> Body {
    Body {
        planet_class: b["planet_class"].as_str().unwrap_or_default().into(),
        atmosphere_type: b["atmosphere_type"].as_str().unwrap_or("None").into(),
        // The kind ("water", "none") reads back as itself; unknown is no volcanism text.
        volcanism: b["volcanism"].as_str().unwrap_or_default().into(),
        temperature_k: b["temperature_k"].as_f64(),
        gravity_g: b["gravity_g"].as_f64(),
        pressure_atm: b["pressure_atm"].as_f64(),
        star_class: b["star"].as_str().map(str::to_string),
    }
}

#[test]
fn recall_on_held_out_bodies() {
    let all: Value = serde_json::from_str(HELD_OUT).unwrap();
    let bodies = all["bodies"].as_array().unwrap();
    assert!(bodies.len() >= 300, "{}", bodies.len());
    let (mut hits, mut total, mut candidates) = (0, 0, 0);
    let mut missed = std::collections::BTreeMap::<&str, u32>::new();
    for b in bodies {
        let c = predict::predict(&body(b));
        candidates += c.len();
        for s in b["species"].as_array().unwrap().iter().filter_map(Value::as_str) {
            let Some((g, sp)) = catalog::species(s) else { continue };
            if sp.rules.is_empty() {
                continue; // Horizons-era: never predicted
            }
            total += 1;
            match c.iter().find(|c| c.species == sp.name) {
                Some(found) => {
                    hits += 1;
                    assert_eq!((found.genus, found.value), (g.name.as_str(), sp.value));
                }
                None => *missed.entry(&sp.name).or_default() += 1,
            }
        }
    }
    let recall = hits as f64 / total as f64;
    let per_body = candidates as f64 / bodies.len() as f64;
    eprintln!("recall {hits}/{total} = {recall:.3}; {per_body:.1} candidates per body; missed {missed:?}");
    assert!(total >= 600, "{total} sightings");
    assert!(recall >= RECALL, "recall {recall:.3} below {RECALL}");
    assert!(per_body <= CANDIDATES, "{per_body:.1} candidates per body, more than {CANDIDATES}");
}

/// What the held-out bodies must reach (measured by the script and this test, then rounded down).
const RECALL: f64 = 0.98;
const CANDIDATES: f64 = 12.5;

/// A condition measured from enough bodies to gate on star and volcanism, and a body in the
/// middle of it.
fn gated() -> (&'static Conditions, Body) {
    let c = catalog::genera()
        .iter()
        .flat_map(|g| &g.species)
        .flat_map(|s| &s.rules)
        .find(|c| {
            c.star_types.as_ref().is_some_and(|s| s.len() < 8)
                && c.volcanism.as_ref().is_some_and(|v| v.len() < 6)
                && c.min_pressure_atm.zip(c.max_pressure_atm).is_some_and(|(lo, hi)| hi > lo && lo > 0.0)
        })
        .expect("a condition with star, volcanism and pressure");
    let mid = |lo: Option<f64>, hi: Option<f64>| lo.zip(hi).map(|(l, h)| (l + h) / 2.0);
    let body = Body {
        planet_class: c.body_types.as_ref().unwrap()[0].clone(),
        atmosphere_type: c.atmosphere.as_ref().unwrap()[0].clone(),
        volcanism: c.volcanism.as_ref().unwrap()[0].clone(),
        temperature_k: mid(c.min_temp_k, c.max_temp_k),
        gravity_g: mid(c.min_gravity_g, c.max_gravity_g),
        pressure_atm: mid(c.min_pressure_atm, c.max_pressure_atm),
        star_class: Some(c.star_types.as_ref().unwrap()[0].clone()),
    };
    (c, body)
}

#[test]
fn each_condition_gates() {
    let (c, b) = gated();
    assert_eq!(predict::fit(c, &b), 1.0, "{c:?} {b:?}");
    let not = |what: &str, b: Body| assert_eq!(predict::fit(c, &b), 0.0, "{what}: {c:?} {b:?}");
    not("atmosphere", Body { atmosphere_type: "Helium".into(), ..b.clone() });
    not("no atmosphere", Body { atmosphere_type: "None".into(), ..b.clone() });
    let other_class = ["Rocky body", "Icy body", "High metal content body", "Rocky ice body"]
        .into_iter()
        .find(|k| !c.body_types.as_ref().unwrap().iter().any(|t| t == k))
        .unwrap();
    not("body class", Body { planet_class: other_class.into(), ..b.clone() });
    let other_star = ["O", "B", "A", "F", "G", "K", "M", "L", "T", "Y", "D", "N", "H", "TTS", "AeBe", "W"]
        .into_iter()
        .find(|k| !c.star_types.as_ref().unwrap().iter().any(|t| t == k))
        .unwrap();
    not("star", Body { star_class: Some(other_star.into()), ..b.clone() });
    let other_volcanism =
        ["none", "water", "carbon dioxide", "silicate", "metallic", "rocky", "ammonia", "methane", "nitrogen"]
            .into_iter()
            .find(|k| !c.volcanism.as_ref().unwrap().iter().any(|v| v == k))
            .unwrap();
    not("volcanism", Body { volcanism: format!("minor {other_volcanism} geysers volcanism"), ..b.clone() });
    let (lo, hi) = (c.min_pressure_atm.unwrap(), c.max_pressure_atm.unwrap());
    not("pressure", Body { pressure_atm: Some(hi + (hi - lo)), ..b.clone() });
    not("pressure", Body { pressure_atm: Some(lo - (hi - lo)), ..b.clone() });
    let (lo, hi) = (c.min_temp_k.unwrap(), c.max_temp_k.unwrap());
    not("temperature", Body { temperature_k: Some(hi + (hi - lo) + 1.0), ..b.clone() });
    let (lo, hi) = (c.min_gravity_g.unwrap(), c.max_gravity_g.unwrap());
    not("gravity", Body { gravity_g: Some(hi + (hi - lo) + 0.01), ..b.clone() });
    // An unknown star doesn't rule it out.
    assert_eq!(predict::fit(c, &Body { star_class: None, ..b.clone() }), 1.0);
}

#[test]
fn the_catalogue_knows_names_from_the_journal_and_spansh() {
    for (name, genus) in [
        ("$Codex_Ent_Stratum_Genus_Name;", "Stratum"),
        ("$Codex_Ent_Clepeus_Genus_Name;", "Clypeus"),
        ("$Codex_Ent_Shrubs_Genus_Name;", "Frutexa"),
        ("$Codex_Ent_Brancae_Name;", "Brain Tree"),
        // Elite Observatory has seen brain trees under this genus symbol too.
        ("$Codex_Ent_Seed_Name;", "Brain Tree"),
        ("Bacterial", "Bacterium"),
        ("Fonticulus", "Fonticulua"),
        ("Brain Trees", "Brain Tree"),
        ("tussock", "Tussock"),
    ] {
        assert_eq!(catalog::genus(name).map(|g| g.name.as_str()), Some(genus), "{name}");
    }
    assert_eq!(catalog::genus("Thargoid Barnacle").map(|g| &g.name), None);
    let (g, s) = catalog::species("Stratum Tectonicas").unwrap();
    assert_eq!((g.name.as_str(), s.value, g.colony_range_m), ("Stratum", 19_010_800, 500));
    assert_eq!(catalog::species("Fonticulua Fluctus").unwrap().1.value, 20_000_000);
    assert_eq!(catalog::species("Roseum Brain Tree").unwrap().0.name, "Brain Tree");
    // Every species, Horizons-era ones included, has its journal symbol.
    for g in catalog::genera() {
        for s in &g.species {
            assert!(s.symbol.is_some(), "{} has no symbol", s.name);
        }
    }
    let by_symbol = |s: &str| catalog::species_by_symbol(s).map(|(_, s)| s.name.as_str());
    assert_eq!(by_symbol("$Codex_Ent_SphereABCD_01_Name;"), Some("Croceum Anemone"));
    assert_eq!(by_symbol("$Codex_Ent_Seed_Name;"), Some("Roseum Brain Tree"));
    assert_eq!(by_symbol("$Codex_Ent_TubeABCD_02_Name;"), Some("Albidum Sinuous Tuber"));
    assert_eq!(by_symbol("$Codex_Ent_Vents_Name;"), Some("Amphora Plant"));
    assert_eq!(catalog::genus("Stratum").unwrap().value_range(), (1_362_000, 19_010_800));
    for (t, class) in [
        ("DAB", "D"),
        ("K", "K"),
        ("A_BlueWhiteSuperGiant", "A"),
        ("TTS", "TTS"),
        ("N", "N"),
        ("WNC", "W"),
        ("CN", "C"),
        ("MS", "S"),
        ("SupermassiveBlackHole", "H"),
    ] {
        assert_eq!(catalog::star_class(t), class, "{t}");
    }
}
