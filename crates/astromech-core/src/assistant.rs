//! Transcript → command. Applies the §5.12 thresholds and handles "Did you mean …?".

use crate::actions::{Outcome, Runner};
use crate::route::{self, Desk};
use astromech_intent::{Decision, Grammar};
use astromech_proto::Origin;
use astromech_store::Store;
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

enum Pending {
    Mean { command: &'static str, heard: String, at: Instant },
    Act { command: &'static str, at: Instant },
}

/// The spoken answer for a command the grammar knows but the app can't run yet.
pub fn unavailable_reason(command: &str) -> &'static str {
    match command {
        "supercruise" => "Supercruise isn't available yet. Flight commands come later.",
        "hyperspace_jump" => "Hyperspace jumps aren't available yet. Flight commands come later.",
        "request_docking" => "Docking requests aren't available yet.",
        "plot_route" => {
            "I can't plot courses yet. Pick a route in the app, then say \"paste\" in the galaxy map's search box."
        }
        _ => "That isn't available yet.",
    }
}

pub struct Assistant {
    grammar: Mutex<Grammar>,
    pub runner: Arc<Runner>,
    pending: Mutex<Option<Pending>>,
    store: Option<Arc<Mutex<Store>>>,
    route: Option<Arc<Desk>>,
}

impl Assistant {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self {
            grammar: Mutex::new(Grammar::mvp(&["Sol"])),
            runner,
            pending: Mutex::new(None),
            store: None,
            route: None,
        }
    }

    pub fn with_store(self, store: Arc<Mutex<Store>>) -> Self {
        if let Ok(list) = store.lock().unwrap().aliases() {
            let mut g = self.grammar.lock().unwrap();
            for (heard, cmd) in list {
                if let Some(cmd) = Grammar::intern(&cmd) {
                    g.add_alias(&heard, cmd);
                }
            }
        }
        Self { store: Some(store), ..self }
    }

    pub fn with_route(self, desk: Arc<Desk>) -> Self {
        Self { route: Some(desk), ..self }
    }

    fn say(&self, s: &str) {
        self.runner.speaker.say(s);
    }

    fn learn(&self, heard: &str, command: &'static str) {
        if heard.trim().is_empty() || command == "confirm" || command == "cancel" {
            return;
        }
        self.grammar.lock().unwrap().add_alias(heard, command);
        if let Some(st) = &self.store {
            let _ = st.lock().unwrap().learn_alias(heard, command);
        }
    }

    /// Route following and paste (docs/PLOTTING.md). None: not one of these.
    fn route_command(&self, command: &'static str, origin: Origin) -> Option<Handled> {
        if command == "galaxy_map.paste" {
            return Some(Handled::Ran { command, outcome: self.runner.paste(origin) });
        }
        if !command.starts_with("route.") {
            return None;
        }
        let m = match &self.route {
            None => "Routes aren't available here.".to_string(),
            Some(d) => {
                let v = match command {
                    "route.skip" => d.view().route.is_some().then(|| d.step(true)),
                    "route.back" => d.view().route.is_some().then(|| d.step(false)),
                    "route.clear" => {
                        let had = d.view().route.is_some();
                        d.clear();
                        let m = if had { "Route cleared." } else { "There's no route to clear." };
                        self.say(m);
                        return Some(Handled::Ran { command, outcome: Outcome::Spoken(m.into()) });
                    }
                    _ => d.view().route.is_some().then(|| d.copy_next()),
                };
                match v {
                    None => "No route set. Pick one on the app's Route page.".to_string(),
                    Some(v) => match &v.next {
                        Some(n) => route::next_line(n, v.remaining, v.clipboard_error.is_none()),
                        None => "That was the last waypoint. The route is finished.".to_string(),
                    },
                }
            }
        };
        self.say(&m);
        Some(Handled::Ran { command, outcome: Outcome::Spoken(m) })
    }

    fn dispatch(&self, command: &'static str, origin: Origin, confirmed: bool) -> Handled {
        if let Some(h) = self.route_command(command, origin) {
            return h;
        }
        match astromech_tools::from_command(command) {
            Some(call) => {
                let outcome =
                    if confirmed { self.runner.run_confirmed(call, origin) } else { self.runner.run(call, origin) };
                if matches!(outcome, Outcome::NeedsConfirm(_)) {
                    *self.pending.lock().unwrap() = Some(Pending::Act { command, at: Instant::now() });
                    return Handled::Asked { command, phrase: "silent running".into() };
                }
                Handled::Ran { command, outcome }
            }
            None => {
                self.say(unavailable_reason(command));
                Handled::NotAvailable(command)
            }
        }
    }

    fn take_pending(&self) -> Option<Pending> {
        let mut p = self.pending.lock().unwrap();
        match p.as_ref() {
            Some(Pending::Mean { at, .. } | Pending::Act { at, .. }) if at.elapsed() < CONFIRM_WINDOW => p.take(),
            _ => {
                *p = None;
                None
            }
        }
    }

    pub fn handle(&self, transcript: &str, origin: Origin) -> Handled {
        if astromech_intent::is_noise_only(transcript) {
            let s = transcript.to_lowercase();
            let yes = s.contains("yeah") || s.contains("okay") || s.contains("ok");
            if yes {
                if let Some(pend) = self.take_pending() {
                    return self.resolve_yes(pend, origin);
                }
            }
            return Handled::Ignored;
        }
        let m = self.grammar.lock().unwrap().best(transcript);
        if let Some(pend) = self.take_pending() {
            if m.decision != Decision::NoMatch && m.command == "confirm" {
                return self.resolve_yes(pend, origin);
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
            Decision::Execute => self.dispatch(m.command, origin, false),
            Decision::Ask if m.command != "confirm" && m.command != "cancel" => {
                *self.pending.lock().unwrap() =
                    Some(Pending::Mean { command: m.command, heard: transcript.to_string(), at: Instant::now() });
                self.say(&format!("Did you mean {}?", m.phrase));
                Handled::Asked { command: m.command, phrase: m.phrase }
            }
            _ => {
                self.say("Didn't catch that.");
                Handled::NotUnderstood
            }
        }
    }

    fn resolve_yes(&self, pend: Pending, origin: Origin) -> Handled {
        match pend {
            Pending::Mean { command, heard, .. } => {
                self.learn(&heard, command);
                self.dispatch(command, origin, false)
            }
            Pending::Act { command, .. } => self.dispatch(command, origin, true),
        }
    }
}
