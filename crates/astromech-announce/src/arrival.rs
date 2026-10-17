//! Arrival briefing: on a jump (`FSDJump`, `CarrierJump`), say only what the HUD doesn't show
//! and only what matters for the commander's goals. Star class, security, allegiance and the
//! system name are on screen, so they are never spoken.
//!
//! - **A faction state the shopping list wants**: "Boom here. Proto Heat Radiators." The
//!   shopping list is what the Engineering plans still have to collect ([`ShoppingList`]);
//!   states count when a material on it comes from them: Boom, War, Civil War, Civil Unrest
//!   and Outbreak for their High Grade Emissions pools (`astromech_materials::hge`), Famine for
//!   wake scans. One state, the most influential faction's first, and the first material on the
//!   list it gives. In a system of over a billion people, a Federation or Empire faction's High
//!   Grade Emissions pool counts too: "Good for Imperial Shielding."
//! - **What the system means for the pledged power**: "Undermining target." or "Acquisition
//!   target." Reinforcing a system of one's own power is the normal case and stays quiet. Left out when the
//!   `powerplay_arrival` callout is on, which says it with the rest of the system's state.
//! - **Undiscovered**: the arrival star's automatic scan (`Scan`, `AutoScan`, 0 ls) with
//!   `WasDiscovered` false gives "Undiscovered system." as its own line just after the jump.
//!
//! - **Been here before**: "Visited 2 times. Last here 3 days ago." from the history ledgers,
//!   as its own line after the others. The arrival being announced is not counted, and a
//!   system the commander has never been to says nothing.
//!
//! A system with none of these gets nothing. Known biology (Spansh) follows these lines from its
//! own callout, [`crate::biology`]; bodies known to EDSM are not part of the briefing yet.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, FactionSnap, JournalEvent};
use astromech_materials::hge;
use astromech_powerplay::{SystemPower, Work};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// One material to collect: its id (`astromech_materials` catalogue, lower case) and the count
/// to hold when it is done (what the plan had plus what it still had to collect).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Want {
    pub id: String,
    pub until: i64,
}

/// What the Engineering plans still have to collect, most wanted first. The core keeps it from
/// the app; the rules read it against the materials held now ([`ShoppingList::needed`]), so a
/// material drops out once collected, whether or not the app is open.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShoppingList(pub Vec<Want>);

impl ShoppingList {
    /// A list that wants each material until a count it never reaches (tests, previews).
    pub fn wanting<S: AsRef<str>>(ids: &[S]) -> Self {
        Self(ids.iter().map(|id| Want { id: id.as_ref().to_string(), until: i64::MAX }).collect())
    }

    /// The ids still short of their count in `state`'s materials, in list order.
    pub fn needed(&self, state: &GameState) -> Vec<String> {
        self.0.iter().filter(|w| state.materials.count(&w.id) < w.until).map(|w| w.id.clone()).collect()
    }
}

/// Shared with the core, which replaces it when a plan changes.
pub type SharedShoppingList = Arc<RwLock<ShoppingList>>;

/// What the history holds about a system from before an arrival.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Past {
    /// Earlier arrivals in the system.
    pub visits: u32,
    /// When the latest of them was (journal timestamp).
    pub last: String,
}

/// The commander's visit history, as the rules need it. The pipeline thread calls it on every
/// jump and must never block, so it answers from an index or a query with a short timeout, and
/// `None` when it can't (no history yet, database busy).
pub trait VisitHistory: Send + Sync {
    /// The arrivals of commander `commander` in `system_address` strictly before `before` (the
    /// journal timestamp of the arrival being announced, which is therefore not counted).
    fn before(&self, commander: &str, system_address: i64, before: &str) -> Option<Past>;
}

/// The history the core plugs in once it has one. Shared like the shopping list.
#[derive(Clone, Default)]
pub struct SharedVisits(Arc<RwLock<Option<Arc<dyn VisitHistory>>>>);

impl SharedVisits {
    pub fn set(&self, history: Arc<dyn VisitHistory>) {
        *self.0.write().unwrap() = Some(history);
    }

    fn get(&self) -> Option<Arc<dyn VisitHistory>> {
        self.0.read().unwrap().clone()
    }
}

impl std::fmt::Debug for SharedVisits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SharedVisits({})", if self.get().is_some() { "set" } else { "none" })
    }
}

pub struct Arrival {
    pub shopping: SharedShoppingList,
    pub visits: SharedVisits,
    /// `powerplay_arrival` is on and says the Powerplay line itself.
    pub powerplay_spoken: crate::powerplay_arrival::SharedFlag,
}

/// "Visited 2 times. Last here 3 days ago." for an arrival at `now` after `past`. The gap is
/// between journal timestamps, so a replay says what a live run would. None if the history
/// holds no earlier arrival or a timestamp doesn't parse.
pub fn visits_line(past: &Past, now: &str) -> Option<String> {
    if past.visits == 0 {
        return None;
    }
    let times = match past.visits {
        1 => "once".to_string(),
        n => format!("{n} times"),
    };
    let secs = |t: &str| chrono::DateTime::parse_from_rfc3339(t).ok().map(|d| d.timestamp());
    let gap = (secs(now)? - secs(&past.last)?).max(0);
    let (n, unit) = match gap {
        0..=59 => return Some(format!("Visited {times}. Last here moments ago.")),
        60..=3599 => (gap / 60, "minute"),
        3600..=86_399 => (gap / 3600, "hour"),
        86_400..=2_591_999 => (gap / 86_400, "day"),
        2_592_000..=31_103_999 => (gap / 2_592_000, "month"),
        _ => (gap / 31_104_000, "year"),
    };
    let s = if n == 1 { "" } else { "s" };
    Some(format!("Visited {times}. Last here {n} {unit}{s} ago."))
}

/// High Grade Emissions from the allegiance pools spawn reliably only in systems this big.
pub const HGE_POPULATION: i64 = 1_000_000_000;

/// What a faction offers: the state to name (None: the system is simply good for it, an
/// allegiance pool in a big system) and the source id (`astromech_materials` notes' `query`)
/// a wanted material must list for it to count.
fn offers(f: &FactionSnap, population: i64) -> Vec<(Option<&'static str>, &'static str)> {
    let mut v = vec![];
    match hge::faction_pool(f, population) {
        Some("proto") => v.push((Some("Boom"), "proto")),
        Some("military") if hge::in_state(f, "Civil War") => v.push((Some("Civil war"), "military")),
        Some("military") => v.push((Some("War"), "military")),
        Some("improvised") => v.push((Some("Civil unrest"), "improvised")),
        Some("pharma") => v.push((Some("Outbreak"), "pharma")),
        Some(pool @ ("fed_composites" | "empire_shielding")) if population >= HGE_POPULATION => v.push((None, pool)),
        _ => {}
    }
    if hge::in_state(f, "Famine") {
        v.push((Some("Famine"), "famine"));
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
/// material on `wanted`, or "Good for Imperial Shielding." for an allegiance pool in a system
/// of over a billion people; None if nothing does.
pub fn state_line(factions: &[FactionSnap], population: i64, wanted: &[String]) -> Option<String> {
    let mut fs: Vec<&FactionSnap> = factions.iter().collect();
    fs.sort_by(|a, b| b.influence.total_cmp(&a.influence));
    fs.iter().flat_map(|f| offers(f, population)).find_map(|(state, query)| {
        let name = wanted_from(wanted, query)?;
        Some(match state {
            Some(state) => format!("{state} here. {name}."),
            None => format!("Good for {name}."),
        })
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
        self.react_all(input, state).into_iter().next()
    }
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        let Input::Journal(l) = input else { return vec![] };
        let say = |text| Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST };
        let text = match &l.event {
            JournalEvent::FsdJump { factions, population, system_address, from_carrier, .. } => {
                let wanted = self.shopping.read().unwrap().needed(state);
                let system = state.system_power.as_ref().filter(|_| !self.powerplay_spoken.load(Ordering::Relaxed));
                let briefing = line(factions, *population, &wanted, system, state.powerplay.power.as_deref());
                let been = (|| {
                    // Home again by carrier is not news.
                    if *from_carrier {
                        return None;
                    }
                    let past =
                        self.visits.get()?.before(state.commander.as_deref()?, (*system_address)?, &l.timestamp)?;
                    visits_line(&past, &l.timestamp)
                })();
                return briefing.into_iter().chain(been).map(say).collect();
            }
            JournalEvent::Extra(ExtraEvent::Scan(b))
                if b.scan_type.eq_ignore_ascii_case("AutoScan")
                    && b.distance_from_arrival_ls == Some(0.0)
                    && b.was_discovered == Some(false) =>
            {
                UNDISCOVERED.to_string()
            }
            _ => return vec![],
        };
        vec![say(text)]
    }
}
