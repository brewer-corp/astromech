//! High Grade Emissions callout (`hge`): on `FSSSignalDiscovered` for a High Grade Emissions
//! signal, "High Grade Emissions. Federation composites." The pool comes from the faction that
//! spawned it (`SpawningFaction`, looked up among the system's factions from the jump), or, when
//! the game doesn't say, from the most influential faction with a pool
//! (`astromech_materials::hge`). A material on the shopping list from that pool is named
//! instead of the pool ("High Grade Emissions. Proto Light Alloys."). Other signals stay quiet.
//! One honk reveals several at once, so a 30 s cooldown keeps it to one line.
//!
//! The arrival side (a system of over a billion people whose allegiance pool has a wanted
//! material: "Good for Imperial Shielding.") is part of [`crate::arrival`].

use crate::arrival::SharedShoppingList;
use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{FactionSnap, JournalEvent};
use astromech_materials::hge;
use std::time::Duration;

/// What to call a pool when no material on the shopping list comes from it.
pub fn pool_label(pool: &str) -> &'static str {
    match pool {
        "fed_composites" => "Federation composites",
        "empire_shielding" => "Imperial Shielding",
        "proto" => "Proto materials",
        "military" => "Military materials",
        "improvised" => "Improvised Components",
        "pharma" => "Pharmaceutical Isolators",
        _ => "",
    }
}

/// The pool of the faction that spawned the signal, else of the most influential faction with one.
pub fn pool(factions: &[FactionSnap], population: i64, spawned_by: Option<&str>) -> Option<&'static str> {
    if let Some(f) = spawned_by.and_then(|n| factions.iter().find(|f| f.name.eq_ignore_ascii_case(n))) {
        return hge::faction_pool(f, population);
    }
    let mut fs: Vec<&FactionSnap> = factions.iter().collect();
    fs.sort_by(|a, b| b.influence.total_cmp(&a.influence));
    fs.into_iter().find_map(|f| hge::faction_pool(f, population))
}

/// "High Grade Emissions. Federation composites." / "High Grade Emissions." with no pool.
pub fn line(pool: Option<&str>, wanted: &[String]) -> String {
    let Some(p) = pool else { return "High Grade Emissions.".into() };
    let mats = hge::pool_materials(p);
    let name = wanted
        .iter()
        .find(|id| mats.contains(&id.as_str()))
        .and_then(|id| astromech_materials::lookup(id))
        .map_or(pool_label(p), |s| s.name);
    format!("High Grade Emissions. {name}.")
}

pub struct Hge {
    pub shopping: SharedShoppingList,
}

impl Rule for Hge {
    fn id(&self) -> &'static str {
        "hge"
    }
    /// One honk reveals several signals at once.
    fn cooldown(&self) -> Duration {
        Duration::from_secs(30)
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        let Input::Journal(l) = input else { return None };
        let JournalEvent::FssSignal { high_grade: true, spawning_faction, .. } = &l.event else { return None };
        let p = pool(&state.system_factions, state.system_population, spawning_faction.as_deref());
        let text = line(p, &self.shopping.read().unwrap().0);
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
}
