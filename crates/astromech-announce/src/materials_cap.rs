//! Material storage cap. The game shows no warning when a material is full, so say so once,
//! on the collect that fills it: "Arsenic at 300/300." Docking at a station with a material
//! trader then names what to trade down to free the slot: "Trader here. Trade down
//! Arsenic."
//!
//! Caps and grades are the materials catalogue's (`astromech_materials::catalog`). A grade 1
//! material cannot go down, so it is traded up instead. The trader's category (raw,
//! manufactured, encoded) follows from the station's economy, which the line does not name:
//! the commander sees which trader it is. Exchange rates are not spoken (the Engineering plan
//! has them: 6 per grade up, 1 for 3 per grade down).

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::JournalEvent;
use astromech_materials::catalog;
use std::collections::BTreeSet;
use std::sync::Mutex;
use std::time::Duration;

/// The most trade suggestions for one docking.
const MAX_SUGGESTIONS: usize = 2;

/// The materials ledger stops at the cap (the game logs the whole pickup but keeps no more), so
/// the count after an event can't say whether it was full before. The rule keeps which trader
/// materials were full after the last journal line, live or replayed.
#[derive(Default)]
pub struct MaterialsCap {
    full: Mutex<BTreeSet<&'static str>>,
}

impl MaterialsCap {
    /// Note where materials stand after each journal line (any line: the ledger alone knows
    /// which events change them).
    fn remember(&self, input: &Input, state: &GameState) {
        if let Input::Journal(_) = input {
            *self.full.lock().unwrap() = catalog::catalog()
                .iter()
                .filter(|s| s.on_trader && state.materials.count(s.id) >= catalog::capacity(s.grade))
                .map(|s| s.id)
                .collect();
        }
    }
}

impl Rule for MaterialsCap {
    fn id(&self) -> &'static str {
        "materials_cap"
    }
    /// Once per material reaching the cap; the delta tells it, not the clock.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        self.react_all(input, state).into_iter().next()
    }
    fn replayed(&self, input: &Input, state: &GameState) {
        self.remember(input, state);
    }
    fn while_off(&self, input: &Input, state: &GameState) {
        self.remember(input, state);
    }
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        let out = self.callouts(input, state);
        self.remember(input, state);
        out
    }
}

impl MaterialsCap {
    fn callouts(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        let Input::Journal(l) = input else { return vec![] };
        let line =
            |text: String| Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST };
        match &l.event {
            JournalEvent::MaterialDelta { items } => items
                .iter()
                .filter(|it| it.count > 0)
                .filter_map(|it| {
                    let spec = catalog::lookup(&it.name).filter(|s| s.on_trader)?;
                    let (now, cap) = (state.materials.count(spec.id), catalog::capacity(spec.grade));
                    // It reached the cap with this event, not before.
                    let was_full = self.full.lock().unwrap().contains(spec.id);
                    (now >= cap && !was_full)
                        .then(|| line(format!("{} at {cap}/{cap}.", state.materials.display_name(spec))))
                })
                .collect(),
            JournalEvent::Docked { station, .. } if station.services.iter().any(|s| s == "materialtrader") => {
                catalog::catalog()
                    .iter()
                    .filter(|s| s.on_trader && state.materials.count(s.id) >= catalog::capacity(s.grade))
                    .take(MAX_SUGGESTIONS)
                    .map(|s| {
                        let way = if s.grade == 1 { "up" } else { "down" };
                        line(format!("Trader here. Trade {way} {}.", state.materials.display_name(s)))
                    })
                    .collect()
            }
            _ => vec![],
        }
    }
}
