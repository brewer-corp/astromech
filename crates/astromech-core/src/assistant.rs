//! Transcript → command. Applies the §5.12 thresholds and handles "Did you mean …?".

use crate::actions::{Outcome, Runner};
use astromech_galaxy::stations::Want;
use astromech_intent::{Decision, Grammar};
use astromech_proto::Origin;
use astromech_store::Store;
use astromech_tools::lookup;
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
    /// A station lookup: the answer is spoken when Spansh replies.
    Lookup {
        command: &'static str,
        want: Want,
    },
    /// A noise-only transcript ("Mm-hmm.", "Yeah."): nothing said, nothing done.
    Ignored,
}

enum Pending {
    Mean { command: &'static str, slot: Option<String>, slot_text: Option<String>, heard: String, at: Instant },
    Act { command: &'static str, at: Instant },
}

/// The spoken answer for a command the grammar knows but the app can't run yet.
pub fn unavailable_reason(command: &str) -> &'static str {
    match command {
        "supercruise" => "Supercruise is not available yet. Flight commands come later.",
        "hyperspace_jump" => "Hyperspace jumps are not available yet. Flight commands come later.",
        "request_docking" => "Docking requests are not available yet.",
        "plot_route" => "Route plotting is not available yet.",
        _ => "That command is not available yet.",
    }
}

/// The command grammar with its slot lists: market commodities for the trade questions.
pub fn grammar() -> Grammar {
    crate::lookups::grammar(&["Sol"])
}

/// Add the personal aliases learned from "Did you mean …?" answers.
pub fn add_aliases(g: &mut Grammar, store: &Store) {
    if let Ok(list) = store.aliases() {
        for (heard, cmd) in list {
            if let Some(cmd) = Grammar::intern(&cmd) {
                g.add_alias(&heard, cmd);
            }
        }
    }
}

pub struct Assistant {
    grammar: Mutex<Grammar>,
    pub runner: Arc<Runner>,
    pending: Mutex<Option<Pending>>,
    store: Option<Arc<Mutex<Store>>>,
    /// Remote data for the lookups (Spansh stations, Ardent markets); shared with the
    /// threads that answer station lookups.
    remote: Arc<astromech_data::Client>,
    /// Judges the age of market and station data.
    clock: Arc<dyn astromech_data::Clock>,
    /// The journal history, for the session summary.
    sessions: astromech_announce::session_summary::SharedSessions,
}

impl Assistant {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self {
            grammar: Mutex::new(grammar()),
            runner,
            pending: Mutex::new(None),
            store: None,
            remote: Arc::new(astromech_data::Client::new(
                astromech_data::Cache::in_memory().expect("an in-memory SQLite cache always opens"),
            )),
            clock: Arc::new(astromech_data::SystemClock),
            sessions: Default::default(),
        }
    }

    /// Answer lookups through `remote`, judging data age by `clock` (tests pass recorded
    /// answers and a fixed time).
    pub fn with_remote(self, remote: astromech_data::Client, clock: Arc<dyn astromech_data::Clock>) -> Self {
        Self { remote: Arc::new(remote), clock, ..self }
    }

    /// Answer lookups through the process's one `remote` client (shared with the star map).
    pub fn with_shared_remote(self, remote: Arc<astromech_data::Client>) -> Self {
        Self { remote, ..self }
    }

    /// Answer the session summary from `sessions` (the history the logout summary reads too).
    pub fn with_sessions(self, sessions: astromech_announce::session_summary::SharedSessions) -> Self {
        Self { sessions, ..self }
    }

    pub fn with_store(self, store: Arc<Mutex<Store>>) -> Self {
        add_aliases(&mut self.grammar.lock().unwrap(), &store.lock().unwrap());
        Self { store: Some(store), ..self }
    }

    /// What `transcript` would do now, without doing it (the voice command tester). Reads the
    /// grammar, the game state and the binds check; never calls the runner.
    pub fn dry_run(&self, transcript: &str, source: crate::tester::Source) -> crate::tester::DryRun {
        let g = self.grammar.lock().unwrap();
        let binds = self.runner.binds.read().unwrap();
        crate::tester::dry_run(&g, transcript, &self.runner.hub.snapshot(), &binds, source)
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

    /// Look `want` up near the commander and say the answer when it arrives (off this thread:
    /// Spansh can take seconds).
    fn look_up(&self, command: &'static str, want: Want) -> Handled {
        let game = self.runner.hub.snapshot();
        let filters = {
            let st = self.store.as_ref().map(|s| s.lock().unwrap_or_else(|e| e.into_inner()));
            crate::lookups::filters_for(st.as_deref(), want.lookup(), &game)
        };
        let (remote, speaker, w) = (self.remote.clone(), self.runner.speaker.clone(), want.clone());
        let now = self.clock.now_s();
        std::thread::spawn(move || speaker.say(&crate::lookups::answer(&remote, &filters, &w, &game, now)));
        Handled::Lookup { command, want }
    }

    fn dispatch(
        &self,
        command: &'static str,
        (slot, slot_text): (Option<&str>, Option<&str>),
        origin: Origin,
        confirmed: bool,
    ) -> Handled {
        if lookup::is_lookup(command) {
            return match lookup::want(command, slot, slot_text) {
                Some(want) => self.look_up(command, want),
                None => {
                    self.say("Didn't catch what to look for.");
                    Handled::NotUnderstood
                }
            };
        }
        if astromech_tools::session::is_summary(command) {
            let game = self.runner.hub.snapshot();
            let totals = match self.sessions.get() {
                Some(h) => h.session(game.commander.as_deref(), None),
                None => Ok(None),
            };
            let lines = astromech_tools::session::answer(totals.as_ref().map(Option::as_ref).map_err(|e| *e));
            for l in &lines {
                self.say(l);
            }
            return Handled::Ran { command, outcome: Outcome::Spoken(lines.join(" ")) };
        }
        if let Some(ask) = astromech_tools::trade::from_command(command) {
            let game = self.runner.hub.snapshot();
            let filters = {
                let st = self.store.as_ref().map(|s| s.lock().unwrap_or_else(|e| e.into_inner()));
                crate::lookups::filters_for(st.as_deref(), ask.lookup(), &game)
            };
            let s = crate::trade::answer(&self.remote, &filters, ask, slot, &game, self.clock.now_s());
            self.say(&s);
            return Handled::Ran { command, outcome: Outcome::Spoken(s) };
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
                self.say("Say again.");
                Handled::NotUnderstood
            }
            Decision::Execute => self.dispatch(m.command, (m.slot.as_deref(), m.slot_text.as_deref()), origin, false),
            Decision::Ask if m.command != "confirm" && m.command != "cancel" => {
                *self.pending.lock().unwrap() = Some(Pending::Mean {
                    command: m.command,
                    slot: m.slot.clone(),
                    slot_text: m.slot_text.clone(),
                    heard: transcript.to_string(),
                    at: Instant::now(),
                });
                self.say(&format!("Did you mean {}?", m.phrase));
                Handled::Asked { command: m.command, phrase: m.phrase }
            }
            _ => {
                self.say("Say again.");
                Handled::NotUnderstood
            }
        }
    }

    fn resolve_yes(&self, pend: Pending, origin: Origin) -> Handled {
        match pend {
            Pending::Mean { command, slot, slot_text, heard, .. } => {
                // An alias stands for a command, not for its slot value: don't learn those.
                if slot.is_none() {
                    self.learn(&heard, command);
                }
                self.dispatch(command, (slot.as_deref(), slot_text.as_deref()), origin, false)
            }
            Pending::Act { command, .. } => self.dispatch(command, (None, None), origin, true),
        }
    }
}
