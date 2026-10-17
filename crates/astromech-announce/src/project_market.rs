//! Colonisation shopping at a market: on docking, how many of the commodities the colonisation
//! projects still need this market has in stock. "Sells 5 project commodities." The HUD's
//! market shows stock, not what your project is short of.
//!
//! **Timing.** The game writes `Market.json` when the commander opens the commodity market,
//! usually after `Docked`. The line is spoken once per dock, when the first live `Market.json`
//! for the market docked at arrives (the pipeline only offers a file newer than its `Market`
//! event, so an earlier visit's file is never used). Opening the market again says nothing.
//!
//! Only commodities with outstanding demand count, over every project still being built
//! ([`GameState::project_needs`]); no project, or none of its commodities in stock, says nothing.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::companion::MarketSnap;
use std::collections::BTreeSet;
use std::time::Duration;

/// How many commodities the projects still need are in stock at `market`.
pub fn count(market: &MarketSnap, state: &GameState) -> usize {
    let needed: BTreeSet<&str> = state.project_needs.values().flatten().map(String::as_str).collect();
    let stocked: BTreeSet<String> =
        market.items.iter().filter(|i| i.stock > 0).map(|i| i.name.to_ascii_lowercase()).collect();
    needed.iter().filter(|n| stocked.contains(**n)).count()
}

/// "Sells 5 project commodities."
pub fn line(count: usize) -> String {
    match count {
        1 => "Sells 1 project commodity.".into(),
        n => format!("Sells {n} project commodities."),
    }
}

pub struct ProjectMarket;

impl Rule for ProjectMarket {
    fn id(&self) -> &'static str {
        "project_market"
    }
    /// The engine already allows one per dock.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        let Input::Market(m) = input else { return None };
        // Ties the file to this dock: not a market opened from a carrier or left behind.
        if state.station_market_id != m.market_id {
            return None;
        }
        let n = count(m, state);
        (n > 0).then(|| Callout { text: line(n), priority: Priority::Normal, channels: Channels::SPEECH })
    }
}
