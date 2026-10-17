//! What Vista Genomics pays for a species, for the unsold organic data total. The values are the
//! exobiology catalogue's (`data/catalog/exobiology.json`, `astromech_exobiology::catalog`; sources
//! there and in NOTICE.md), looked up by the journal's species symbol
//! (`$Codex_Ent_Stratum_07_Name;`, `$Codex_Ent_SphereABCD_01_Name;` for a Horizons-era one).

/// The base value of a species by its `Species` symbol; `None` if unknown.
pub fn species_value(species: &str) -> Option<i64> {
    astromech_exobiology::catalog::sample_value(species)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn species_by_symbol_horizons_era_too() {
        assert_eq!(species_value("$Codex_Ent_Stratum_07_Name;"), Some(19_010_800));
        // The wiki says 16,777,215 (2^24-1); the catalogue fixes it (listed as uncertain).
        assert_eq!(species_value("$Codex_Ent_Fonticulus_05_Name;"), Some(20_000_000));
        assert_eq!(species_value("$Codex_Ent_Bacterial_01_Name;"), Some(1_000_000));
        assert_eq!(species_value("$Codex_Ent_Bacterial_07_Name;"), Some(1_949_000));
        // Horizons-era species by their own symbols: Croceum Anemone, an Aureum Brain Tree.
        assert_eq!(species_value("$Codex_Ent_SphereABCD_01_Name;"), Some(3_399_800));
        assert_eq!(species_value("$Codex_Ent_SeedEFGH_01_Name;"), Some(3_565_100));
        assert_eq!(species_value("$Codex_Ent_Unknown_Name;"), None);
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
