//! What Vista Genomics pays for a species: `data/catalog/exobiology.json` (sources there and in
//! NOTICE.md). Looked up by the journal's species symbol (`$Codex_Ent_Stratum_07_Name;`); the
//! genera with one species value (anemones, brain trees, ...) by their genus symbol.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// `name` is in the file for people reading it; lookups go by symbol.
#[derive(Deserialize)]
struct Entry {
    symbol: String,
    value: i64,
}

#[derive(Deserialize)]
struct File {
    species: Vec<Entry>,
    genera: Vec<Entry>,
}

struct Table {
    species: HashMap<String, i64>,
    genera: HashMap<String, i64>,
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let f: File =
            serde_json::from_str(include_str!("../../../data/catalog/exobiology.json")).expect("exobiology.json");
        let map = |v: Vec<Entry>| v.into_iter().map(|e| (e.symbol.to_ascii_lowercase(), e.value)).collect();
        Table { species: map(f.species), genera: map(f.genera) }
    })
}

/// The base value of a species (`Species`, else its `Genus`); `None` if unknown.
pub fn species_value(species: &str, genus: &str) -> Option<i64> {
    let t = table();
    t.species.get(&species.to_ascii_lowercase()).or_else(|| t.genera.get(&genus.to_ascii_lowercase())).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn species_and_single_species_genera() {
        assert_eq!(species_value("$Codex_Ent_Stratum_07_Name;", "$Codex_Ent_Stratum_Genus_Name;"), Some(19_010_800));
        assert_eq!(species_value("$Codex_Ent_Fonticulus_05_Name;", ""), Some(20_000_000));
        assert_eq!(species_value("$Codex_Ent_Bacterial_01_Name;", ""), Some(1_000_000));
        assert_eq!(species_value("$Codex_Ent_Bacterial_07_Name;", ""), Some(1_949_000));
        assert_eq!(species_value("$Codex_Ent_SphereABCD_01_Name;", "$Codex_Ent_Sphere_Name;"), Some(1_499_900));
        assert_eq!(species_value("$Codex_Ent_Unknown_Name;", "$Codex_Ent_Unknown_Genus_Name;"), None);
    }

    /// Names and symbols as written, with no stray spaces ("Bacterium Tela " had one).
    #[test]
    fn catalog_text_is_trimmed() {
        let f: serde_json::Value = serde_json::from_str(include_str!("../../../data/catalog/exobiology.json")).unwrap();
        for e in f["species"].as_array().unwrap().iter().chain(f["genera"].as_array().unwrap()) {
            for k in ["symbol", "name"] {
                let v = e[k].as_str().unwrap();
                assert_eq!(v, v.split_whitespace().collect::<Vec<_>>().join(" "), "{k} {v:?}");
            }
        }
        assert!(f.to_string().contains(r#""name":"Bacterium Tela""#));
    }
}
