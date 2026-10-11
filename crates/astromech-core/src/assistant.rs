//! Transcript → command. Applies the §5.12 thresholds and handles "Did you mean …?".

use crate::actions::{Outcome, Runner};
use astromech_intent::{Decision, Grammar};
use astromech_proto::Origin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// How long a "Did you mean …?" stays open for a yes/no.
pub const CONFIRM_WINDOW: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub enum Handled {
    Ran {
        command: &'static str,
        outcome: Outcome,
    },
    Asked {
        command: &'static str,
        phrase: String,
    },
    NotUnderstood,
    Cancelled,
    NotAvailable(&'static str),
    /// A noise-only transcript ("Mm-hmm.", "Yeah."): nothing said, nothing done.
    Ignored,
}

/// The spoken answer for a command the grammar knows but the app can't run yet.
pub fn unavailable_reason(command: &str) -> &'static str {
    match command {
        "supercruise" => "I can't do that yet. Supercruise isn't in this preview; flight commands come later.",
        "hyperspace_jump" => {
            "I can't do that yet. Hyperspace jumps aren't in this preview; flight commands come later."
        }
        "request_docking" => "I can't do that yet. Docking requests aren't in this preview.",
        "plot_route" => "I can't do that yet. Route plotting isn't in this preview.",
        _ => "I can't do that yet.",
    }
}

pub struct Assistant {
    pub grammar: Grammar,
    pub runner: Arc<Runner>,
    pending: Mutex<Option<(&'static str, Instant)>>,
}

impl Assistant {
    pub fn new(runner: Arc<Runner>) -> Self {
        // No system list yet in this slice (route plotting isn't implemented).
        Self { grammar: Grammar::mvp(&["Sol"]), runner, pending: Mutex::new(None) }
    }

    fn say(&self, s: &str) {
        self.runner.speaker.say(s);
    }

    fn dispatch(&self, command: &'static str, origin: Origin) -> Handled {
        match astromech_tools::from_command(command) {
            Some(call) => Handled::Ran { command, outcome: self.runner.run(call, origin) },
            None => {
                self.say(unavailable_reason(command));
                Handled::NotAvailable(command)
            }
        }
    }

    pub fn handle(&self, transcript: &str, origin: Origin) -> Handled {
        if astromech_intent::is_noise_only(transcript) {
            // "Yeah."/"Okay." answers an open "Did you mean …?"; otherwise it's noise.
            let s = transcript.to_lowercase();
            let yes = s.contains("yeah") || s.contains("okay") || s.contains("ok");
            let mut p = self.pending.lock().unwrap();
            if yes {
                if let Some((cmd, t)) = *p {
                    if t.elapsed() < CONFIRM_WINDOW {
                        *p = None;
                        drop(p);
                        return self.dispatch(cmd, origin);
                    }
                }
            }
            return Handled::Ignored;
        }
        let m = self.grammar.best(transcript);
        let pending = self.pending.lock().unwrap().take().filter(|(_, t)| t.elapsed() < CONFIRM_WINDOW);
        if let Some((cmd, _)) = pending {
            if m.decision != Decision::NoMatch && m.command == "confirm" {
                return self.dispatch(cmd, origin);
            }
            if m.decision != Decision::NoMatch && m.command == "cancel" {
                self.say("Cancelled.");
                return Handled::Cancelled;
            }
        }
        match m.decision {
            Decision::Execute if m.command == "confirm" || m.command == "cancel" => {
                self.say("Didn't catch that.");
                Handled::NotUnderstood
            }
            Decision::Execute => self.dispatch(m.command, origin),
            Decision::Ask if m.command != "confirm" && m.command != "cancel" => {
                *self.pending.lock().unwrap() = Some((m.command, Instant::now()));
                self.say(&format!("Did you mean {}?", m.phrase));
                Handled::Asked { command: m.command, phrase: m.phrase }
            }
            _ => {
                self.say("Didn't catch that.");
                Handled::NotUnderstood
            }
        }
    }
}
