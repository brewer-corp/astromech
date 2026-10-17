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
    /// Silent running on: spoken warning, nothing pressed yet.
    NeedsConfirm(String),
    /// Pressed, with nothing in Status.json to verify it by (paste).
    Pressed(String),
}

/// Between Ctrl+V and Enter, for the search box to take the text. Pasting into the box works
/// (players do it by hand); this delay and the Enter after it are ⚠ unverified. d47 waits 1.5 s
/// before its Return, but it then steps into the results itself.
pub const PASTE_SETTLE: Duration = Duration::from_millis(500);

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
        self.execute(call, origin, false)
    }

    /// After the user confirmed a spoken warning.
    pub fn run_confirmed(&self, call: ToolCall, origin: Origin) -> Outcome {
        self.execute(call, origin, true)
    }

    /// Paste the clipboard into the galaxy map's search box: Ctrl+V, then Enter to search.
    /// The player has selected the box already (in VR, with their UI keys). Only while the
    /// galaxy map is showing: anywhere else V or Enter may be a ship binding.
    pub fn paste(&self, origin: Origin) -> Outcome {
        let say = |s: &str| self.speaker.say(s);
        let g = self.hub.snapshot();
        if !g.status_seen || g.gui_focus != Some(astromech_status::GUI_GALAXY) {
            let m = "Open the galaxy map and select its search box first.".to_string();
            say(&m);
            return Outcome::Refused(m);
        }
        let ticket = match self.policy.authorize("paste_search", Tier::T1, origin, Instant::now()) {
            Ok(t) => t,
            Err(d) => {
                say(&d.spoken());
                return Outcome::Denied(d);
            }
        };
        let r = ticket.press("Key_V", &["Key_LeftControl".to_string()]).and_then(|()| {
            std::thread::sleep(PASTE_SETTLE);
            ticket.press("Key_Enter", &[])
        });
        if let Err(e) = r {
            say(&e.spoken());
            return Outcome::PressFailed(e);
        }
        (self.log)("paste_search: Ctrl+V, Enter sent to the galaxy map");
        say("Pasted.");
        Outcome::Pressed("Pasted.".into())
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
