//! Species prediction against bodies whose species are known.
//!
//! `fixtures/spansh-bio-bodies.json` is a trimmed Spansh bodies search (17 Oct 2026): 48 bodies
//! with biological signals whose every species has been sampled and reported (the landmarks),
//! picked so that together they cover every species seen in a thousand bodies around Sol, Colonia
//! and three other regions. Each body's conditions go through the prediction; the species found
//! there must be among the candidates, at the value Spansh reports, inside its genus's range.

use astromech_exobiology::catalog;
use astromech_exobiology::predict::{self, Body};
use serde_json::Value;

const BODIES: &str = include_str!("fixtures/spansh-bio-bodies.json");

/// Species whose value in Spansh's codex data differs from the wiki's table (the catalogue's
/// source): (species, Spansh, catalogue). The wiki marks Bacterium Nebulus's value as changed;
/// Spansh has the old one.
const SPANSH_DIFFERS: &[(&str, i64, i64)] =
    &[("Bacterium Nebulus", 5_289_900, 9_116_600), ("Tussock Ventusa", 3_227_700, 3_277_700)];

/// Spansh's "Thin Carbon dioxide-rich" as the journal's `AtmosphereType` `CarbonDioxideRich`.
fn atmosphere_type(spansh: &str) -> String {
    let a = spansh.trim_start_matches("Thin ").trim_start_matches("Thick ");
    if a == "No atmosphere" {
        return "None".into();
    }
    let (a, rich) = match a.strip_suffix("-rich") {
        Some(a) => (a, "Rich"),
        None => (a, ""),
    };
    let words: String = a
        .split([' ', '-'])
        .map(|w| {
            let mut c = w.chars();
            c.next().map(|f| f.to_uppercase().chain(c).collect::<String>()).unwrap_or_default()
        })
        .collect();
    words + rich
}

/// Spansh's body subtype as the journal's `PlanetClass`.
fn planet_class(spansh: &str) -> String {
    match spansh {
        "High metal content world" => "High metal content body",
        "Rocky Ice world" => "Rocky ice body",
        other => other,
    }
    .into()
}

/// Spansh's star subtype ("K (Yellow-Orange) Star", "T Tauri Star") as a journal `StarType`.
fn star_type(spansh: &str) -> String {
    if spansh.starts_with("T Tauri") {
        "TTS".into()
    } else if spansh.contains("White Dwarf") {
        "DA".into()
    } else if spansh.starts_with("Neutron") {
        "N".into()
    } else if spansh.starts_with("Black Hole") {
        "H".into()
    } else if spansh.starts_with("Herbig") {
        "AeBe".into()
    } else {
        spansh[..1].into()
    }
}

fn body(b: &Value) -> Body {
    let star = b["parents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["type"] == "Star")
        .map(|p| catalog::star_class(&star_type(p["subtype"].as_str().unwrap())));
    let volcanism = match b["volcanism_type"].as_str() {
        None | Some("No volcanism") => String::new(),
        Some(v) => format!("{} volcanism", v.to_lowercase()),
    };
    Body {
        planet_class: planet_class(b["subtype"].as_str().unwrap()),
        atmosphere_type: atmosphere_type(b["atmosphere"].as_str().unwrap_or("No atmosphere")),
        volcanism,
        temperature_k: b["surface_temperature"].as_f64(),
        gravity_g: b["gravity"].as_f64(),
        pressure_atm: b["surface_pressure"].as_f64(),
        star_class: star,
    }
}

#[test]
fn the_prediction_includes_every_species_found_there_and_its_value() {
    let all: Value = serde_json::from_str(BODIES).unwrap();
    let bodies = all["results"].as_array().unwrap();
    assert_eq!(bodies.len(), 48);
    let mut species_checked = std::collections::BTreeSet::new();
    for b in bodies {
        let name = b["name"].as_str().unwrap();
        let candidates = predict::predict(&body(b), true);
        for l in b["landmarks"].as_array().unwrap() {
            let (genus, species, value) =
                (l["type"].as_str().unwrap(), l["subtype"].as_str().unwrap(), l["value"].as_i64().unwrap());
            let c = candidates
                .iter()
                .find(|c| c.species == species)
                .unwrap_or_else(|| panic!("{name}: {species} not predicted; got {candidates:?}"));
            assert_eq!(c.genus, genus, "{name}: {species}");
            match SPANSH_DIFFERS.iter().find(|d| d.0 == species) {
                Some(&(_, spansh, ours)) => assert_eq!((value, c.value), (spansh, ours), "{name}: {species}"),
                None => assert_eq!(c.value, value, "{name}: {species} value differs from Spansh's"),
            }
            let value = c.value;
            let of_genus: Vec<_> = candidates.iter().filter(|c| c.genus == genus).cloned().collect();
            let (lo, hi) = predict::value_range(&of_genus);
            assert!(lo <= value && value <= hi, "{name}: {species} {value} outside {lo}..{hi}");
            species_checked.insert(species.to_string());
        }
    }
    assert_eq!(species_checked.len(), 70);
}

#[test]
fn the_prediction_rules_out_species_that_cannot_grow_there() {
    let all: Value = serde_json::from_str(BODIES).unwrap();
    // A high metal content world with a thin carbon dioxide atmosphere (36 Ophiuchi A 3, Stratum
    // Tectonicas): no ammonia-only or icy-body species.
    let b = all["results"].as_array().unwrap().iter().find(|b| b["name"] == "36 Ophiuchi A 3").unwrap();
    let c = predict::predict(&body(b), true);
    for none in ["Bacterium Alcyoneum", "Fonticulua Campestris", "Electricae Pluma", "Aleoida Laminiae"] {
        assert!(!c.iter().any(|c| c.species == none), "{none} predicted on {}", b["name"]);
    }
    // Nothing grows without an atmosphere.
    let airless = Body { atmosphere_type: "None".into(), ..body(b) };
    assert!(predict::predict(&airless, true).is_empty());
}

#[test]
fn soft_ranges_taper_and_hard_gates_do_not() {
    // Stratum Tectonicas: high metal content body, thin CO2, from 165 K.
    let tectonicas = |t: f64| Body {
        planet_class: "High metal content body".into(),
        atmosphere_type: "CarbonDioxide".into(),
        volcanism: String::new(),
        temperature_k: Some(t),
        gravity_g: Some(0.3),
        pressure_atm: Some(0.05),
        star_class: Some("K".into()),
    };
    let fit =
        |t| predict::predict(&tectonicas(t), true).iter().find(|c| c.species == "Stratum Tectonicas").map(|c| c.fit);
    assert_eq!(fit(200.0), Some(1.0));
    assert!(fit(1000.0).is_none(), "far too hot");
    let star_y = Body { star_class: Some("Y".into()), ..tectonicas(200.0) };
    let unknown_star = Body { star_class: None, ..tectonicas(200.0) };
    assert_eq!(
        predict::predict(&star_y, true).iter().any(|c| c.species == "Stratum Tectonicas"),
        predict::predict(&unknown_star, true).iter().any(|c| c.species == "Stratum Tectonicas"),
        "Stratum Tectonicas has no star condition"
    );
}

#[test]
fn the_catalogue_knows_names_from_the_journal_and_spansh() {
    for (name, genus) in [
        ("$Codex_Ent_Stratum_Genus_Name;", "Stratum"),
        ("$Codex_Ent_Clepeus_Genus_Name;", "Clypeus"),
        ("$Codex_Ent_Shrubs_Genus_Name;", "Frutexa"),
        ("$Codex_Ent_Brancae_Name;", "Brain Tree"),
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
    assert_eq!(catalog::species("Roseum Brain Tree").unwrap().0.name, "Brain Tree");
    assert_eq!(catalog::genus("Stratum").unwrap().value_range(), (1_362_000, 19_010_800));
    for (t, class) in [("DAB", "D"), ("K", "K"), ("A_BlueWhiteSuperGiant", "A"), ("TTS", "TTS"), ("N", "N")] {
        assert_eq!(catalog::star_class(t), class);
    }
}
