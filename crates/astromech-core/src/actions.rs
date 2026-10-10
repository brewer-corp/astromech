//! Run one typed tool call: plan from the snapshot, look up the key, authorise, press,
//! confirm optimistically, verify in the background (ARCHITECTURE.md §5.5).

use crate::health::Shared;
use crate::hub::StateHub;
use crate::speech::Speaker;
use astromech_policy::{Denial, Policy, PressError};
use astromech_proto::Origin;
use astromech_tools::{plan, Plan, ToolCall};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verified {
    /// Status.json showed the expected flag after this many presses.
    Confirmed { presses: u32 },
    /// Still unchanged after the retry.
    Failed,
    /// Kill switch.
    Cancelled,
    /// The retry press itself failed (focus lost etc.).
    RetryBlocked(PressError),
}

#[derive(Debug)]
pub enum Outcome {
    Already(String),
    Refused(String),
    NoBinding(String),
    Denied(Denial),
    PressFailed(PressError),
    /// Pressed; the handle resolves when verification ends.
    Sent(JoinHandle<Verified>),
}

pub struct Runner {
    pub policy: Policy,
    pub hub: Arc<StateHub>,
    pub binds: Shared,
    pub speaker: Arc<dyn Speaker>,
    pub verify_timeout: Duration,
    pub retries: u32,
}

impl Runner {
    pub fn run(&self, call: ToolCall, origin: Origin) -> Outcome {
        let spec = call.toggle.spec();
        let say = |s: &str| self.speaker.say(s);
        let (action, flag, expect, confirm, failure) = match plan(call, self.hub.snapshot().flags()) {
            Plan::Already(m) => {
                say(&m);
                return Outcome::Already(m);
            }
            Plan::Refused(m) => {
                say(&m);
                return Outcome::Refused(m);
            }
            Plan::Press { action, flag, expect_set, confirm, failure } => (action, flag, expect_set, confirm, failure),
        };
        let combo = {
            let b = self.binds.read().unwrap();
            match b.keyboard(action) {
                Some(k) => k,
                None => {
                    let m = b.missing_reason(action, spec.noun);
                    drop(b);
                    say(&m);
                    return Outcome::NoBinding(m);
                }
            }
        };
        let ticket = match self.policy.authorize(spec.name, spec.tier, origin, Instant::now()) {
            Ok(t) => t,
            Err(d) => {
                say(&d.spoken());
                return Outcome::Denied(d);
            }
        };
        if let Err(e) = ticket.press(&combo.key, &combo.modifiers) {
            say(&e.spoken());
            return Outcome::PressFailed(e);
        }
        // Optimistic confirmation.
        say(&confirm);
        let hub = self.hub.clone();
        let speaker = self.speaker.clone();
        let timeout = self.verify_timeout;
        let retries = self.retries;
        let h = std::thread::spawn(move || {
            let mut presses = 1;
            loop {
                if hub.wait_flag(flag, expect, timeout, &|| ticket.is_cancelled()) {
                    return Verified::Confirmed { presses };
                }
                if ticket.is_cancelled() {
                    return Verified::Cancelled;
                }
                if presses > retries {
                    speaker.say(&failure);
                    return Verified::Failed;
                }
                // Toggle actions: re-read right before retrying so a late change isn't undone.
                let now = hub.snapshot();
                if now.status_seen && (now.status_flags & flag != 0) == expect {
                    return Verified::Confirmed { presses };
                }
                if let Err(e) = ticket.press(&combo.key, &combo.modifiers) {
                    if e == PressError::Cancelled {
                        return Verified::Cancelled;
                    }
                    speaker.say(&format!("{failure} {}", e.spoken()));
                    return Verified::RetryBlocked(e);
                }
                presses += 1;
            }
        });
        Outcome::Sent(h)
    }
}
