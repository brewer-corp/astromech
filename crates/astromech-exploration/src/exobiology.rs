//! What Vista Genomics pays for a species, for the unsold organic data total. The values are the
//! exobiology catalogue's (`data/catalog/exobiology.json`, `astromech_exobiology::catalog`; sources
//! there and in NOTICE.md), looked up by the journal's species symbol
//! (`$Codex_Ent_Stratum_07_Name;`); a Horizons-era genus (anemones, brain trees, ...) by its genus
//! symbol, at the lowest value of its species.

/// The base value of a species (`Species`, else its `Genus`); `None` if unknown.
pub fn species_value(species: &str, genus: &str) -> Option<i64> {
    astromech_exobiology::catalog::sample_value(species, genus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn species_and_horizons_genera() {
        assert_eq!(species_value("$Codex_Ent_Stratum_07_Name;", "$Codex_Ent_Stratum_Genus_Name;"), Some(19_010_800));
        // The wiki's value; EDR and ExplorerLite say 20,000,000 (listed as uncertain).
        assert_eq!(species_value("$Codex_Ent_Fonticulus_05_Name;", ""), Some(16_777_215));
        assert_eq!(species_value("$Codex_Ent_Bacterial_01_Name;", ""), Some(1_000_000));
        assert_eq!(species_value("$Codex_Ent_Bacterial_07_Name;", ""), Some(1_949_000));
        assert_eq!(species_value("$Codex_Ent_SphereABCD_01_Name;", "$Codex_Ent_Sphere_Name;"), Some(1_499_900));
        assert_eq!(species_value("$Codex_Ent_SphereABCD_01_Name;", "$Codex_Ent_Sphere_Genus_Name;"), Some(1_499_900));
        assert_eq!(species_value("$Codex_Ent_Unknown_Name;", "$Codex_Ent_Unknown_Genus_Name;"), None);
        // An Odyssey genus alone doesn't say which species: no value.
        assert_eq!(species_value("$Codex_Ent_Unknown_Name;", "$Codex_Ent_Stratum_Genus_Name;"), None);
    }

    /// Names and symbols as written, with no stray spaces ("Bacterium Tela " had one in EDR).
    #[test]
    fn catalog_text_is_trimmed() {
        for g in astromech_exobiology::catalog::genera() {
            for s in &g.species {
                for v in [Some(&s.name), s.symbol.as_ref()].into_iter().flatten() {
                    assert_eq!(*v, v.split_whitespace().collect::<Vec<_>>().join(" "), "{v:?}");
                }
            }
        }
        assert!(astromech_exobiology::catalog::species("Bacterium Tela").is_some());
    }
}
