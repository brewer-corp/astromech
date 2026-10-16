//! Run one typed tool call: plan from the snapshot, look up the key, authorise, press,
//! confirm optimistically, verify in the background (ARCHITECTURE.md §5.5).

use crate::health::Shared;
use crate::hub::StateHub;
use crate::speech::Speaker;
use astromech_policy::{Denial, Policy, PressError};
use astromech_proto::{Origin, Tier};
use astromech_tools::{authorize_plan, plan_with, tool_name, Plan, ToolCall, Verify};
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
    /// No Status.json signal (throttle to zero).
    Unverified,
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
    Spoken(String),
    /// A site lookup is running off the caller's thread ("Looking that up." was said); the
    /// handle resolves to the answer once it has been spoken.
    LookingUp(JoinHandle<String>),
    /// Silent running on: spoken warning, nothing pressed yet.
    NeedsConfirm(String),
}

pub struct Runner {
    pub policy: Policy,
    pub hub: Arc<StateHub>,
    pub binds: Shared,
    pub speaker: Arc<dyn Speaker>,
    pub verify_timeout: Duration,
    /// Where verification outcomes go (the console log).
    pub log: Arc<dyn Fn(&str) + Send + Sync>,
    /// Star map and remote lookups for "nearest crystalline shards" and friends; `None` when
    /// the galaxy store couldn't be opened.
    pub galaxy: Option<Arc<astromech_galaxy::Galaxy>>,
}

impl Runner {
    /// Start the nearest-site lookup for `kind`. Spansh and EDSM can take many seconds, so the
    /// lookup runs on its own thread and speaks its answer when it arrives: the speech thread is
    /// free for the next command at once. Jumps are counted at the ship's maximum range (20 ly if
    /// unknown); the lookup's filters in the app don't apply.
    pub fn nearest_site(&self, kind: &'static str) -> Outcome {
        let g = self.hub.snapshot();
        let (Some(system), Some(pos)) = (g.system.clone(), g.star_pos) else {
            let m = "Your position isn't known yet. It comes from the next jump or login.".to_string();
            self.speaker.say(&m);
            return Outcome::Spoken(m);
        };
        let Some(galaxy) = self.galaxy.clone() else {
            let m = "Site lookups aren't available: the star map couldn't be opened.".to_string();
            self.speaker.say(&m);
            return Outcome::Spoken(m);
        };
        let range = g.max_jump_range.filter(|r| *r > 1.0).unwrap_or(20.0);
        let here = astromech_galaxy::Here { system, id64: None, pos };
        self.speaker.say("Looking that up.");
        let speaker = self.speaker.clone();
        let h = std::thread::spawn(move || {
            let m = match galaxy.sites(kind, &here, range, 1, &astromech_galaxy::Filters::default()) {
                Ok(s) => astromech_galaxy::sites::spoken(&s),
                Err(e) => format!("Couldn't look that up. {e}"),
            };
            speaker.say(&m);
            m
        });
        Outcome::LookingUp(h)
    }

    pub fn run(&self, call: ToolCall, origin: Origin) -> Outcome {
        self.execute(call, origin, false)
    }

    /// After the user confirmed a spoken warning.
    pub fn run_confirmed(&self, call: ToolCall, origin: Origin) -> Outcome {
        self.execute(call, origin, true)
    }

    fn execute(&self, call: ToolCall, origin: Origin, confirmed: bool) -> Outcome {
        let say = |s: &str| self.speaker.say(s);
        let mut planned = plan_with(call, &self.hub.snapshot());
        if confirmed {
            planned = authorize_plan(planned);
        }
        let (action, verify, confirm) = match planned {
            Plan::Already(m) => {
                say(&m);
                return Outcome::Already(m);
            }
            Plan::Refused(m) => {
                say(&m);
                return Outcome::Refused(m);
            }
            Plan::Speak(m) => {
                say(&m);
                return Outcome::Spoken(m);
            }
            Plan::Lookup(kind) => return self.nearest_site(kind),
            Plan::Confirm { prompt, .. } => {
                say(&prompt);
                return Outcome::NeedsConfirm(prompt);
            }
            Plan::Press { action, verify, confirm, .. } => (action, verify, confirm),
        };
        let noun = astromech_tools::action_noun(call);
        let combo = {
            let b = self.binds.read().unwrap();
            match b.keyboard(action) {
                Some(k) => k,
                None => {
                    let m = b.missing_reason(action, noun);
                    drop(b);
                    say(&m);
                    return Outcome::NoBinding(m);
                }
            }
        };
        let name = tool_name(call);
        let ticket = match self.policy.authorize(name, Tier::T1, origin, Instant::now()) {
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
        say(&confirm);
        let hub = self.hub.clone();
        let log = self.log.clone();
        let timeout = self.verify_timeout;
        let t0 = Instant::now();
        let h = std::thread::spawn(move || {
            let v = match verify {
                Verify::None => Verified::Unverified,
                Verify::Flag { bit, expect_set } => {
                    if hub.wait_flag(bit, expect_set, timeout, &|| ticket.is_cancelled()) {
                        Verified::Confirmed { ms: t0.elapsed().as_millis() as u64 }
                    } else if ticket.is_cancelled() {
                        Verified::Cancelled
                    } else {
                        Verified::NotConfirmed
                    }
                }
                Verify::GuiFocus(want) => {
                    if hub.wait_gui(want, timeout, &|| ticket.is_cancelled()) {
                        Verified::Confirmed { ms: t0.elapsed().as_millis() as u64 }
                    } else if ticket.is_cancelled() {
                        Verified::Cancelled
                    } else {
                        Verified::NotConfirmed
                    }
                }
            };
            log(&match &v {
                Verified::Confirmed { ms } => format!("{name}: confirmed in {ms} ms"),
                Verified::NotConfirmed => {
                    format!("{name}: not confirmed within {:.1} s (no retry)", timeout.as_secs_f64())
                }
                Verified::Cancelled => format!("{name}: verification cancelled (kill switch)"),
                Verified::Unverified => format!("{name}: sent (no status flag)"),
            });
            v
        });
        Outcome::Sent(h)
    }
}
