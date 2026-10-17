//! Exobiology sampling helper (`sampling`): after a sample (`ScanOrganic`), "Far enough for
//! sample 2." once Status.json puts the commander at least the genus's colony range from every
//! earlier sample of the species (`astromech_exobiology::sampling`). Said once per sample: walking
//! back inside the range and out again doesn't repeat it, and a cue already due when the app
//! restarts (the catch-up) isn't said after it. The genetic sampler shows the samples
//! taken; how far is far enough is the part players count off by hand.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, JournalEvent};
use std::sync::Mutex;
use std::time::Duration;

/// "Far enough for sample 2."
pub fn line(next: u8) -> String {
    format!("Far enough for sample {next}.")
}

/// Which sample was last cued: (system, body, species, next sample). Cleared by every sample
/// taken, so a species logged again on the same body is cued afresh.
type Cued = (Option<i64>, Option<i64>, String, u8);

#[derive(Default)]
pub struct Sampling {
    cued: Mutex<Option<Cued>>,
}

impl Sampling {
    /// The next sample's number when the cue for it is due now, remembering it as cued.
    fn due(&self, input: &Input, state: &GameState) -> Option<u8> {
        let mut cued = self.cued.lock().unwrap();
        match input {
            Input::Journal(l) if matches!(l.event, JournalEvent::Extra(ExtraEvent::ScanOrganic { .. })) => {
                *cued = None;
                None
            }
            Input::Status { .. } => {
                let s = &state.exobiology.sampling;
                let c = s.clearance().filter(|c| c.far_enough)?;
                let p = s.current.as_ref()?;
                let key = (p.system_address, p.body_id, p.species.clone(), c.next);
                if cued.as_ref() == Some(&key) {
                    return None;
                }
                *cued = Some(key);
                Some(c.next)
            }
            _ => None,
        }
    }
}

impl Rule for Sampling {
    fn id(&self) -> &'static str {
        "sampling"
    }
    /// Once per sample is the rule's own.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        let next = self.due(input, state)?;
        Some(Callout { text: line(next), priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
    /// The catch-up after a restart: a cue already due then counts as said.
    fn replayed(&self, input: &Input, state: &GameState) {
        self.due(input, state);
    }
}
