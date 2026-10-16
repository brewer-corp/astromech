//! Market commodities by journal symbol (`data/catalog/commodities.json`): the ordinary
//! commodities stations trade, with their in-game English names and categories, plus spoken
//! forms the voice grammar also accepts ("LTDs"). Rare goods are not listed; a symbol from
//! `Cargo.json` still works in a lookup without being in the catalogue. Ardent Insight and the
//! journal use the same lowercase symbols ("lowtemperaturediamond").

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, serde::Serialize)]
pub struct Commodity {
    /// Journal and Ardent symbol, lowercase: "lowtemperaturediamond".
    pub symbol: String,
    /// In-game English name: "Low Temperature Diamonds".
    pub name: String,
    pub category: String,
    /// Other names a commander says ("LTDs").
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Every catalogue commodity, by category then name as in the file.
pub fn all() -> &'static [Commodity] {
    static ALL: OnceLock<Vec<Commodity>> = OnceLock::new();
    ALL.get_or_init(|| {
        serde_json::from_str(include_str!("../../../data/catalog/commodities.json"))
            .expect("data/catalog/commodities.json is valid")
    })
}

/// The commodity with this symbol (any case).
pub fn by_symbol(symbol: &str) -> Option<&'static Commodity> {
    all().iter().find(|c| c.symbol.eq_ignore_ascii_case(symbol.trim()))
}

/// The commodity a commander typed or said: its name, an alias or its symbol (any case,
/// spaces and punctuation ignored).
pub fn find(text: &str) -> Option<&'static Commodity> {
    let key = squash(text);
    if key.is_empty() {
        return None;
    }
    all().iter().find(|c| squash(&c.name) == key || c.symbol == key || c.aliases.iter().any(|a| squash(a) == key))
}

/// A symbol's name: the catalogue's, else the symbol itself.
pub fn name_of(symbol: &str) -> String {
    by_symbol(symbol).map(|c| c.name.clone()).unwrap_or_else(|| symbol.to_string())
}

/// (spoken form, canonical name) pairs for the voice grammar's `{commodity}` slot.
pub fn spoken_names() -> Vec<(&'static str, &'static str)> {
    all()
        .iter()
        .flat_map(|c| {
            std::iter::once((c.name.as_str(), c.name.as_str()))
                .chain(c.aliases.iter().map(move |a| (a.as_str(), c.name.as_str())))
        })
        .collect()
}

fn squash(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_has_the_mined_goods_and_unique_symbols() {
        let all = all();
        assert!(all.len() > 150, "{}", all.len());
        let mut syms: Vec<&str> = all.iter().map(|c| c.symbol.as_str()).collect();
        syms.sort();
        let n = syms.len();
        syms.dedup();
        assert_eq!(n, syms.len(), "duplicate symbols");
        for s in ["painite", "platinum", "lowtemperaturediamond", "opal", "tritium", "gold", "monazite"] {
            assert!(by_symbol(s).is_some(), "{s}");
        }
        assert!(all.iter().all(|c| c.symbol == c.symbol.to_lowercase() && !c.name.is_empty()));
    }

    #[test]
    fn find_by_name_alias_or_symbol() {
        assert_eq!(find("Low Temperature Diamonds").unwrap().symbol, "lowtemperaturediamond");
        assert_eq!(find("ltds").unwrap().symbol, "lowtemperaturediamond");
        assert_eq!(find("void opals").unwrap().symbol, "opal");
        assert_eq!(find("meta alloys").unwrap().symbol, "metaalloys");
        assert_eq!(find("PAINITE").unwrap().name, "Painite");
        assert!(find("banana").is_none() && find("  ").is_none());
        assert_eq!(name_of("opal"), "Void Opal");
        assert_eq!(name_of("someraregood"), "someraregood");
        assert!(spoken_names().contains(&("LTDs", "Low Temperature Diamonds")));
    }
}
