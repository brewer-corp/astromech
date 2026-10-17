//! Arrival briefing: on a jump (`FSDJump`, `CarrierJump`), say only what the HUD doesn't show
//! and only what matters for the commander's goals. Star class, security, allegiance and the
//! system name are on screen, so they are never spoken.
//!
//! - **A faction state the shopping list wants**: "Boom here. Proto Heat Radiators." The
//!   shopping list is what the Engineering plans still have to collect ([`ShoppingList`]);
//!   states count when a material on it comes from them: Boom, War, Civil War, Civil Unrest
//!   and Outbreak for their High Grade Emissions pools (`astromech_materials::hge`), Famine for
//!   wake scans. One state, the most influential faction's first, and the first material on the
//!   list it gives.
//! - **What the system means for the pledged power**: "Undermining target." or "Acquisition
//!   target." Reinforcing a system of one's own power is the normal case and stays quiet.
//! - **Undiscovered**: the arrival star's automatic scan (`Scan`, `AutoScan`, 0 ls) with
//!   `WasDiscovered` false gives "Undiscovered system." as its own line just after the jump.
//!
//! A system with none of these gets nothing. Bodies known to EDSM, times visited and known
//! biology are not part of the briefing yet.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, FactionSnap, JournalEvent};
use astromech_materials::hge;
use astromech_powerplay::{SystemPower, Work};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Material ids (`astromech_materials` catalogue, lower case) still to collect, most wanted
/// first. The core keeps it from the app's Engineering plans.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShoppingList(pub Vec<String>);

/// Shared with the core, which replaces it when a plan changes.
pub type SharedShoppingList = Arc<RwLock<ShoppingList>>;

pub struct Arrival {
    pub shopping: SharedShoppingList,
}

/// The faction states that can matter, with the source id (`astromech_materials` notes'
/// `query`) a wanted material must list for the state to count.
fn offers(f: &FactionSnap, population: i64) -> Vec<(&'static str, &'static str)> {
    let mut v = vec![];
    match hge::faction_pool(f, population) {
        Some("proto") => v.push(("Boom", "proto")),
        Some("military") if hge::in_state(f, "Civil War") => v.push(("Civil war", "military")),
        Some("military") => v.push(("War", "military")),
        Some("improvised") => v.push(("Civil unrest", "improvised")),
        Some("pharma") => v.push(("Outbreak", "pharma")),
        _ => {}
    }
    if hge::in_state(f, "Famine") {
        v.push(("Famine", "famine"));
    }
    v
}

/// The first material on `wanted` that comes from the source `query`, by name.
fn wanted_from(wanted: &[String], query: &str) -> Option<&'static str> {
    wanted.iter().find_map(|id| {
        let spec = astromech_materials::lookup(id)?;
        hge::sources_for(spec).iter().any(|n| n.query.as_deref() == Some(query)).then_some(spec.name)
    })
}

/// "Boom here. Proto Heat Radiators." for the most influential faction whose state gives a
/// material on `wanted`; None if no state does.
pub fn state_line(factions: &[FactionSnap], population: i64, wanted: &[String]) -> Option<String> {
    let mut fs: Vec<&FactionSnap> = factions.iter().collect();
    fs.sort_by(|a, b| b.influence.total_cmp(&a.influence));
    fs.iter().flat_map(|f| offers(f, population)).find_map(|(state, query)| {
        let name = wanted_from(wanted, query)?;
        Some(format!("{state} here. {name}."))
    })
}

/// What the system offers the pledged power, when that is news.
pub fn powerplay_line(system: &SystemPower, pledged: Option<&str>) -> Option<&'static str> {
    match system.work_for(pledged)? {
        Work::Undermine => Some("Undermining target."),
        Work::Acquire => Some("Acquisition target."),
        Work::Reinforce => None,
    }
}

/// The briefing for a jump: the state line, then the Powerplay line; None if neither applies.
pub fn line(
    factions: &[FactionSnap],
    population: i64,
    wanted: &[String],
    system: Option<&SystemPower>,
    pledged: Option<&str>,
) -> Option<String> {
    let parts: Vec<String> =
        [state_line(factions, population, wanted), system.and_then(|s| powerplay_line(s, pledged)).map(str::to_string)]
            .into_iter()
            .flatten()
            .collect();
    (!parts.is_empty()).then(|| parts.join(" "))
}

pub const UNDISCOVERED: &str = "Undiscovered system.";

impl Rule for Arrival {
    fn id(&self) -> &'static str {
        "arrival"
    }
    /// One jump, one system: nothing to hold back.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        let Input::Journal(l) = input else { return None };
        let text = match &l.event {
            JournalEvent::FsdJump { factions, population, .. } => {
                let wanted = self.shopping.read().unwrap();
                line(factions, *population, &wanted.0, state.system_power.as_ref(), state.powerplay.power.as_deref())?
            }
            JournalEvent::Extra(ExtraEvent::Scan(b))
                if b.scan_type.eq_ignore_ascii_case("AutoScan")
                    && b.distance_from_arrival_ls == Some(0.0)
                    && b.was_discovered == Some(false) =>
            {
                UNDISCOVERED.to_string()
            }
            _ => return None,
        };
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
}
