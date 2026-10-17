//! The ores worth mining, most valuable first: one list for the prospector callouts (which name
//! at most two ores per rock, in this order) and the mining lookups (`astromech-market`'s
//! hotspot and sale lookups offer them in this order). A static order from wiki-visible prices;
//! the live price of an ore is the market lookup's job.

/// One ore: the journal symbol (lower case, as `ProspectedAsteroid` and the markets spell it)
/// and the name to say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ore {
    pub symbol: &'static str,
    pub name: &'static str,
}

const fn ore(symbol: &'static str, name: &'static str) -> Ore {
    Ore { symbol, name }
}

/// High-value ores, most valuable first.
pub const VALUE_ORDER: &[Ore] = &[
    ore("opal", "Void Opals"),
    ore("lowtemperaturediamond", "Low Temperature Diamonds"),
    ore("painite", "Painite"),
    ore("platinum", "Platinum"),
    ore("musgravite", "Musgravite"),
    ore("grandidierite", "Grandidierite"),
    ore("alexandrite", "Alexandrite"),
    ore("benitoite", "Benitoite"),
    ore("monazite", "Monazite"),
    ore("rhodplumsite", "Rhodplumsite"),
    ore("serendibite", "Serendibite"),
    ore("osmium", "Osmium"),
    ore("palladium", "Palladium"),
    ore("bromellite", "Bromellite"),
    ore("tritium", "Tritium"),
];

/// The ore with this symbol (any case).
pub fn find(symbol: &str) -> Option<&'static Ore> {
    VALUE_ORDER.iter().find(|o| o.symbol.eq_ignore_ascii_case(symbol))
}
