//! Powerplay arrival: on a jump (`FSDJump`, `CarrierJump`) into a system with Powerplay
//! activity, say its state: "Zachary Hudson fortified, 40 percent." "Reinforcement 10,
//! undermining 20." and, for a pledged commander, what it means for them ("Undermining
//! target."). The game shows this only in the system and galaxy maps, never on the HUD.
//!
//! Off by default ([`Rule::default_enabled`]); `[callouts.powerplay_arrival] enabled = true`
//! turns it on. While on, it owns the "Undermining target." / "Acquisition target." line, so the
//! `arrival` rule leaves it out ([`Shared::powerplay_spoken`]). The line is
//! `astromech_powerplay::arrival_line`, said a sentence at a time (a long list a clause at a time)
//! to keep each within the word budget.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::JournalEvent;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Whether `powerplay_arrival` is on, set by the engine from the settings; `arrival` reads it.
pub type SharedFlag = Arc<AtomicBool>;

/// A sentence longer than this (the callout word budget) is split at its commas.
const MAX_WORDS: usize = 8;

pub struct PowerplayArrival {
    pub spoken: SharedFlag,
}

impl Rule for PowerplayArrival {
    fn id(&self) -> &'static str {
        "powerplay_arrival"
    }
    /// One jump, one system: nothing to hold back.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn default_enabled(&self) -> bool {
        false
    }
    fn configured(&self, enabled: bool) {
        self.spoken.store(enabled, Ordering::Relaxed);
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        self.react_all(input, state).into_iter().next()
    }
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        let Input::Journal(l) = input else { return vec![] };
        if !matches!(l.event, JournalEvent::FsdJump { .. }) {
            return vec![];
        }
        let Some(line) = astromech_powerplay::arrival_line(&l.event, state.powerplay.power.as_deref()) else {
            return vec![];
        };
        sentences(&line)
            .into_iter()
            .map(|text| Callout { text, priority: Priority::Normal, channels: Channels::SPEECH })
            .collect()
    }
}

/// An arrival line as the sentences to say, each within the word budget.
pub fn sentences(line: &str) -> Vec<String> {
    let mut out = vec![];
    for sentence in line.split_inclusive(". ").map(str::trim) {
        // A long list (several powers acquiring) is said a clause at a time.
        if sentence.split_whitespace().count() > MAX_WORDS {
            out.extend(sentence.trim_end_matches('.').split(", ").map(|c| format!("{c}.")));
        } else {
            out.push(sentence.to_string());
        }
    }
    out
}
