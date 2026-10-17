//! What Vista Genomics pays for a species: `data/catalog/exobiology.json` (sources there and in
//! NOTICE.md). Looked up by the journal's species symbol (`$Codex_Ent_Stratum_07_Name;`); the
//! genera with one species value (anemones, brain trees, ...) by their genus symbol.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct Entry {
    symbol: String,
    name: String,
    value: i64,
}

#[derive(Deserialize)]
struct File {
    species: Vec<Entry>,
    genera: Vec<Entry>,
}

struct Table {
    species: HashMap<String, (String, i64)>,
    genera: HashMap<String, (String, i64)>,
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let f: File =
            serde_json::from_str(include_str!("../../../data/catalog/exobiology.json")).expect("exobiology.json");
        let map = |v: Vec<Entry>| v.into_iter().map(|e| (e.symbol.to_ascii_lowercase(), (e.name, e.value))).collect();
        Table { species: map(f.species), genera: map(f.genera) }
    })
}

/// The base value of a species (`Species`, else its `Genus`), and its name; `None` if unknown.
pub fn species_value(species: &str, genus: &str) -> Option<(&'static str, i64)> {
    let t = table();
    t.species
        .get(&species.to_ascii_lowercase())
        .or_else(|| t.genera.get(&genus.to_ascii_lowercase()))
        .map(|(n, v)| (n.as_str(), *v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn species_and_single_species_genera() {
        assert_eq!(
            species_value("$Codex_Ent_Stratum_07_Name;", "$Codex_Ent_Stratum_Genus_Name;"),
            Some(("Stratum Tectonicas", 19_010_800))
        );
        assert_eq!(species_value("$Codex_Ent_Fonticulus_05_Name;", ""), Some(("Fonticulua Fluctus", 20_000_000)));
        assert_eq!(species_value("$Codex_Ent_Bacterial_01_Name;", ""), Some(("Bacterium Aurasus", 1_000_000)));
        assert_eq!(
            species_value("$Codex_Ent_SphereABCD_01_Name;", "$Codex_Ent_Sphere_Name;").map(|s| s.1),
            Some(1_499_900)
        );
        assert_eq!(species_value("$Codex_Ent_Unknown_Name;", "$Codex_Ent_Unknown_Genus_Name;"), None);
    }
}
