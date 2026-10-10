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
    Ran { command: &'static str, outcome: Outcome },
    Asked { command: &'static str, phrase: String },
    NotUnderstood,
    Cancelled,
    NotAvailable(&'static str),
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
                self.say("That command isn't available yet.");
                Handled::NotAvailable(command)
            }
        }
    }

    pub fn handle(&self, transcript: &str, origin: Origin) -> Handled {
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
