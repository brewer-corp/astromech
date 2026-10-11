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

/// Background check after a press. Logged only: never spoken, never retried (a second press
/// of a toggle could undo a late change).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verified {
    /// Status.json showed the expected flag this long after the press.
    Confirmed { ms: u64 },
    /// No change within `verify_timeout`.
    NotConfirmed,
    /// Kill switch.
    Cancelled,
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
    /// Where verification outcomes go (the console log).
    pub log: Arc<dyn Fn(&str) + Send + Sync>,
}

impl Runner {
    pub fn run(&self, call: ToolCall, origin: Origin) -> Outcome {
        let spec = call.toggle.spec();
        let say = |s: &str| self.speaker.say(s);
        let (action, flag, expect, confirm) = match plan(call, self.hub.snapshot().flags()) {
            Plan::Already(m) => {
                say(&m);
                return Outcome::Already(m);
            }
            Plan::Refused(m) => {
                say(&m);
                return Outcome::Refused(m);
            }
            Plan::Press { action, flag, expect_set, confirm, .. } => (action, flag, expect_set, confirm),
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
        let log = self.log.clone();
        let timeout = self.verify_timeout;
        let name = spec.name;
        let t0 = Instant::now();
        let h = std::thread::spawn(move || {
            let v = if hub.wait_flag(flag, expect, timeout, &|| ticket.is_cancelled()) {
                Verified::Confirmed { ms: t0.elapsed().as_millis() as u64 }
            } else if ticket.is_cancelled() {
                Verified::Cancelled
            } else {
                Verified::NotConfirmed
            };
            log(&match &v {
                Verified::Confirmed { ms } => format!("{name}: confirmed in {ms} ms"),
                Verified::NotConfirmed => {
                    format!("{name}: not confirmed within {:.1} s (no retry)", timeout.as_secs_f64())
                }
                Verified::Cancelled => format!("{name}: verification cancelled (kill switch)"),
            });
            v
        });
        Outcome::Sent(h)
    }
}
