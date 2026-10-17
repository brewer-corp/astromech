//! Bodies known to EDSM (`known_bodies`), part of the arrival briefing: how much of the system
//! other players have already scanned, so the commander knows whether it is worth scanning.
//!
//! - On arrival (`FSDJump`, `CarrierJump`, or a login: `Location`, once a system): "12 of 30
//!   bodies known." against EDSM's body count for the system, "12 bodies known." when EDSM has
//!   no count, and "No bodies known." when EDSM has none (or doesn't know the system). A system
//!   EDSM knows completely says nothing: most of the populated galaxy is, and it is not worth
//!   scanning for discoveries anyway.
//! - The answer comes from a [`KnownBodies`] the core plugs in, which answers from what it has
//!   and looks the system up in the background. An answer that arrives later is said on the
//!   clock tick, up to a minute after the arrival; a lookup that fails or is still out by then
//!   says nothing. The pipeline thread never waits on the network.
//!
//! The in-game "undiscovered" flag is the `arrival` rule's ("Undiscovered system.").

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::JournalEvent;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

/// What EDSM knows about a system's bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Known {
    /// Bodies EDSM has.
    pub bodies: u32,
    /// The system's body count, if EDSM has it (from a discovery scan someone shared).
    pub total: Option<u32>,
}

/// Remote knowledge of a system's bodies. Called on the pipeline thread, so it must never block:
/// it answers from what it already has, and otherwise starts a lookup and says None.
pub trait KnownBodies: Send + Sync {
    /// What is known about `system`'s bodies, or None while it doesn't know yet.
    fn known(&self, system: &str) -> Option<Known>;
}

/// The lookup the core plugs in once it has one.
#[derive(Clone, Default)]
pub struct SharedKnownBodies(Arc<RwLock<Option<Arc<dyn KnownBodies>>>>);

impl SharedKnownBodies {
    pub fn set(&self, k: Arc<dyn KnownBodies>) {
        *self.0.write().unwrap() = Some(k);
    }

    pub fn get(&self) -> Option<Arc<dyn KnownBodies>> {
        self.0.read().unwrap().clone()
    }
}

impl std::fmt::Debug for SharedKnownBodies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SharedKnownBodies({})", if self.get().is_some() { "set" } else { "none" })
    }
}

/// How long after an arrival a late answer is still worth saying, seconds.
pub const LATE_S: i64 = 60;

/// The line for what is known; None when EDSM knows every body.
pub fn line(k: Known) -> Option<String> {
    let bodies = |n: u32| if n == 1 { "body" } else { "bodies" };
    match (k.bodies, k.total) {
        (0, _) => Some("No bodies known.".into()),
        (n, Some(t)) if n >= t => None,
        (n, Some(t)) => Some(format!("{n} of {t} {} known.", bodies(t))),
        (n, None) => Some(format!("{n} {} known.", bodies(n))),
    }
}

#[derive(Default)]
struct Said {
    system: Option<String>,
    /// The arrival here (a jump or a login) has been handled.
    arrived: bool,
    /// An answer still awaited: until this time (Unix seconds).
    waiting_until: Option<i64>,
}

pub struct KnownBodiesRule {
    pub known: SharedKnownBodies,
    said: Mutex<Said>,
}

impl KnownBodiesRule {
    pub fn new(known: SharedKnownBodies) -> Self {
        Self { known, said: Mutex::default() }
    }
}

fn say(k: Known) -> Option<Callout> {
    line(k).map(|text| Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
}

impl Rule for KnownBodiesRule {
    fn id(&self) -> &'static str {
        "known_bodies"
    }
    /// Its own memory decides what is new.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        let mut said = self.said.lock().unwrap();
        match input {
            Input::Journal(l) => {
                // `FsdJump` is a carrier jump too; `Location` a login or a respawn.
                let (JournalEvent::FsdJump { star_system, .. } | JournalEvent::Location { star_system, .. }) = &l.event
                else {
                    return None;
                };
                if star_system.is_empty() {
                    return None;
                }
                if said.system.as_deref() != Some(star_system.as_str()) {
                    *said = Said { system: Some(star_system.clone()), ..Default::default() };
                }
                if said.arrived {
                    return None;
                }
                said.arrived = true;
                match self.known.get()?.known(star_system) {
                    Some(k) => say(k),
                    None => {
                        let ts = chrono::DateTime::parse_from_rfc3339(&l.timestamp).ok().map(|t| t.timestamp());
                        said.waiting_until = ts.map(|t| t + LATE_S);
                        None
                    }
                }
            }
            Input::Tick { wall_secs } => {
                let until = said.waiting_until?;
                if *wall_secs > until {
                    said.waiting_until = None;
                    return None;
                }
                // Still in the system it was asked for (the state follows the journal).
                let system = said.system.clone().filter(|s| state.system.as_deref() == Some(s.as_str()))?;
                let k = self.known.get()?.known(&system)?;
                said.waiting_until = None;
                say(k)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let k = |bodies, total| Known { bodies, total };
        assert_eq!(line(k(2, Some(9))).as_deref(), Some("2 of 9 bodies known."));
        assert_eq!(line(k(12, None)).as_deref(), Some("12 bodies known."));
        assert_eq!(line(k(1, None)).as_deref(), Some("1 body known."));
        assert_eq!(line(k(0, Some(30))).as_deref(), Some("No bodies known."));
        assert_eq!(line(k(0, None)).as_deref(), Some("No bodies known."));
        assert_eq!(line(k(9, Some(9))), None, "all known: nothing to scan for");
        assert_eq!(line(k(10, Some(9))), None);
    }
}
