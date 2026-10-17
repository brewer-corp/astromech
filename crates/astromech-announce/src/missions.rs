//! Mission reminders (`missions`), the part of EDMC's massacre tracker that speaks:
//!
//! - **Expiry**: "Mission expires within an hour." and "Mission expires within 10 minutes."
//!   Driven by [`Input::Tick`] (the pipeline ticks with the wall clock), once per mission per
//!   threshold. Missions crossing a threshold on the same tick share one line ("3 missions
//!   expire within an hour."). The game announces neither.
//! - **Target here**: on `FSDJump`, "Mission target in this system." when an active mission's
//!   destination is the system, or "Mission target faction here." when a mission without a
//!   destination names a target faction that is in the system's faction list.
//!
//! Going live after the catch-up (which is silent and keeps no history) a mission already past
//! a threshold gets one line on the first live tick, only for the tightest threshold it has
//! passed (inside 10 minutes: the 10 minute line, not also the hour one); hence "within". Expired
//! missions say nothing. A line dropped by the engine's quiet (combat, docking) is not repeated.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::JournalEvent;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

/// Warning thresholds in seconds before expiry, loosest first.
const THRESHOLDS: [i64; 2] = [3600, 600];

#[derive(Default)]
pub struct Missions {
    /// Per mission id, the tightest threshold already warned (index into [`THRESHOLDS`] + 1).
    warned: Mutex<HashMap<i64, usize>>,
}

/// "Mission expires within an hour." / "3 missions expire within 10 minutes."
pub fn expiry_line(n: usize, level: usize) -> String {
    let when = if level == 1 { "an hour" } else { "10 minutes" };
    if n == 1 {
        format!("Mission expires within {when}.")
    } else {
        format!("{n} missions expire within {when}.")
    }
}

pub fn target_line(faction: bool) -> &'static str {
    if faction {
        "Mission target faction here."
    } else {
        "Mission target in this system."
    }
}

fn callout(text: String) -> Callout {
    Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST }
}

impl Rule for Missions {
    fn id(&self) -> &'static str {
        "missions"
    }
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        self.react_all(input, state).into_iter().next()
    }
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        match input {
            Input::Tick { wall_secs } => {
                let mut warned = self.warned.lock().unwrap();
                warned.retain(|id, _| state.missions.iter().any(|m| m.id == *id));
                let mut counts = [0usize; 2];
                for m in &state.missions {
                    let Some(left) = m.expires.map(|e| e - wall_secs).filter(|l| *l > 0) else { continue };
                    let Some(level) = THRESHOLDS.iter().rposition(|t| left <= *t).map(|i| i + 1) else { continue };
                    if warned.get(&m.id).is_none_or(|w| *w < level) {
                        warned.insert(m.id, level);
                        counts[level - 1] += 1;
                    }
                }
                counts
                    .iter()
                    .enumerate()
                    .filter(|(_, n)| **n > 0)
                    .map(|(i, n)| callout(expiry_line(*n, i + 1)))
                    .collect()
            }
            Input::Journal(l) if matches!(l.event, JournalEvent::FsdJump { .. }) => {
                let Some(system) = &state.system else { return vec![] };
                let here = |s: &Option<String>| s.as_ref().is_some_and(|s| s.eq_ignore_ascii_case(system));
                if state.missions.iter().any(|m| here(&m.destination_system)) {
                    return vec![callout(target_line(false).into())];
                }
                let in_system = |f: &str| state.system_factions.iter().any(|x| x.name.eq_ignore_ascii_case(f));
                let faction_here = state
                    .missions
                    .iter()
                    .any(|m| m.destination_system.is_none() && m.target_faction.as_deref().is_some_and(in_system));
                if faction_here {
                    vec![callout(target_line(true).into())]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }
}
