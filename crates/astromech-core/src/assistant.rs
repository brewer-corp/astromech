//! Transcript → command. Applies the §5.12 thresholds and handles "Did you mean …?".

use crate::actions::{Outcome, Runner};
use crate::route::{self, Desk};
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
        "plot_route" => "Plotting is not available. Plot in the app, then paste.",
        _ => "That command is not available yet.",
    }
}

/// The command grammar with its slot lists: market commodities for the trade questions.
pub fn grammar() -> Grammar {
    crate::lookups::grammar(&["Sol"])
}

/// Yes answers a phrase needs before it runs a key-pressing command without asking.
pub const KEY_CONFIRMATIONS: u32 = 2;

/// Commands that press a key in the game (the T1 tools and paste), as opposed to questions and
/// lookups that only speak.
pub fn presses_keys(command: &str) -> bool {
    use astromech_tools::ToolCall;
    command == "galaxy_map.paste"
        || matches!(
            astromech_tools::from_command(command),
            Some(ToolCall::Toggle { .. } | ToolCall::Screen(_) | ToolCall::ThrottleZero)
        )
}

/// Whether a phrase confirmed `count` times for `command` runs it without asking: never for an
/// ask-only phrase (a word from the mishearing table, "I'll stop", "Pays"); after
/// [`KEY_CONFIRMATIONS`] yes answers for a command that presses a key; after one otherwise.
pub fn alias_runs(heard: &str, command: &str, count: u32) -> bool {
    !astromech_intent::is_ask_only(heard) && count >= confirmations_needed(command)
}

fn confirmations_needed(command: &str) -> u32 {
    if presses_keys(command) {
        KEY_CONFIRMATIONS
    } else {
        1
    }
}

/// Add the personal aliases learned from "Did you mean …?" answers that [run](alias_runs).
pub fn add_aliases(g: &mut Grammar, store: &Store) {
    if let Ok(list) = store.aliases() {
        for (heard, cmd, count) in list {
            if let Some(cmd) = Grammar::intern(&cmd) {
                if alias_runs(&heard, cmd, count) {
                    g.add_alias(&heard, cmd);
                }
            }
        }
    }
}

pub struct Assistant {
    grammar: Mutex<Grammar>,
    pub runner: Arc<Runner>,
    pending: Mutex<Option<Pending>>,
    store: Option<Arc<Mutex<Store>>>,
    /// Yes answers per heard phrase when there is no store (heard → command, count).
    confirmations: Mutex<std::collections::HashMap<String, (&'static str, u32)>>,
    /// Judges the age of market and station data.
    clock: Arc<dyn astromech_data::Clock>,
    /// The journal history, for the session summary.
    sessions: astromech_announce::session_summary::SharedSessions,
    /// The colonisation projects, replayed from the journals.
    projects: Arc<crate::colonisation::Projects>,
    /// The current system and the unsold data, from the journals and the history.
    exploration: Arc<crate::exploration::Exploration>,
    /// The route the core follows (none: the route commands say so).
    route: Option<Arc<Desk>>,
}

impl Assistant {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self {
            grammar: Mutex::new(grammar()),
            runner,
            pending: Mutex::new(None),
            store: None,
            confirmations: Default::default(),
            clock: Arc::new(astromech_data::SystemClock),
            sessions: Default::default(),
            projects: Default::default(),
            exploration: Default::default(),
            route: None,
        }
    }

    /// Answer the route commands from `desk`, the route the core follows.
    pub fn with_route(self, desk: Arc<Desk>) -> Self {
        Self { route: Some(desk), ..self }
    }

    /// Answer the colonisation and exploration questions from the journals in `dir`.
    pub fn with_journals(self, dir: std::path::PathBuf) -> Self {
        Self {
            projects: Arc::new(crate::colonisation::Projects::new(dir.clone())),
            exploration: Arc::new(self.exploration.with_journals(dir)),
            ..self
        }
    }

    /// Answer the exploration questions with the journal history in `db` (`history.sqlite`):
    /// the unsold data, and what earlier visits scanned in the current system.
    pub fn with_history(self, db: std::path::PathBuf) -> Self {
        Self { exploration: Arc::new(self.exploration.with_history(db)), ..self }
    }

    /// Judge the age of market and station data by `clock` (tests pass the time their
    /// recorded answers were made). Lookups go through the runner's `remote` client.
    pub fn with_clock(self, clock: Arc<dyn astromech_data::Clock>) -> Self {
        Self { clock, ..self }
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

    /// Count a yes to "Did you mean `command`?" after `heard`; the phrase runs without asking
    /// once [`alias_runs`] says so. Ask-only phrases are not even counted.
    fn learn(&self, heard: &str, command: &'static str) {
        let heard = heard.trim();
        if heard.is_empty() || command == "confirm" || command == "cancel" || astromech_intent::is_ask_only(heard) {
            return;
        }
        let count = match &self.store {
            Some(st) => st.lock().unwrap().learn_alias(heard, command).unwrap_or(0),
            None => {
                let mut c = self.confirmations.lock().unwrap();
                let e = c.entry(heard.to_string()).or_insert((command, 0));
                if e.0 != command {
                    *e = (command, 0);
                }
                e.1 += 1;
                e.1
            }
        };
        // Added once, on the yes that makes it run (after that it no longer asks).
        if count == confirmations_needed(command) {
            self.grammar.lock().unwrap().add_alias(heard, command);
        }
    }

    /// Forget a learned phrase: it asks again next time (and is dropped from the saved list).
    /// True if it was one.
    pub fn forget(&self, heard: &str) -> bool {
        let in_grammar = self.grammar.lock().unwrap().remove_alias(heard);
        self.confirmations.lock().unwrap().remove(heard.trim());
        let in_store = self.store.as_ref().is_some_and(|st| st.lock().unwrap().forget_alias(heard).unwrap_or(false));
        in_grammar || in_store
    }

    /// Look `want` up near the commander and say the answer when it arrives (off this thread:
    /// Spansh can take seconds).
    fn look_up(&self, command: &'static str, want: Want) -> Handled {
        let game = self.runner.hub.snapshot();
        let filters = {
            let st = self.store.as_ref().map(|s| s.lock().unwrap_or_else(|e| e.into_inner()));
            crate::lookups::filters_for(st.as_deref(), want.lookup(), &game)
        };
        let (remote, speaker, w) = (self.runner.remote.clone(), self.runner.speaker.clone(), want.clone());
        let now = self.clock.now_s();
        std::thread::spawn(move || speaker.say(&crate::lookups::answer(&remote, &filters, &w, &game, now)));
        Handled::Lookup { command, want }
    }

    /// Look up the pledged power's nearest target of `target`'s kind, with the app's remembered
    /// filters for that search, and say the answer when it arrives (off this thread).
    fn targets_lookup(&self, target: astromech_galaxy::Target) -> Outcome {
        use astromech_tools::targets;
        let game = self.runner.hub.snapshot();
        let said = |m: &str| {
            self.say(m);
            Outcome::Spoken(m.into())
        };
        let galaxy = match (&game.powerplay.power, self.runner.galaxy.clone()) {
            (None, _) => return said(targets::NOT_PLEDGED),
            (_, None) => return said(lookup::NO_STAR_MAP),
            (Some(_), Some(g)) => g,
        };
        if game.system.is_none() || game.star_pos.is_none() {
            return said(lookup::NO_POSITION);
        }
        let filters = {
            let st = self.store.as_ref().map(|s| s.lock().unwrap_or_else(|e| e.into_inner()));
            crate::lookups::filters_for(st.as_deref(), targets::lookup(target), &game)
        };
        self.say("Looking that up.");
        let speaker = self.runner.speaker.clone();
        Outcome::LookingUp(std::thread::spawn(move || {
            let m = crate::lookups::targets_answer(&galaxy, &filters, target, &game);
            speaker.say(&m);
            m
        }))
    }

    /// Look up the faction-state systems for `q` near the commander, with the app's remembered
    /// filters for that lookup, and say the answer when it arrives (off this thread).
    fn faction_lookup(&self, command: &'static str, q: &'static astromech_galaxy::Query) -> Outcome {
        let game = self.runner.hub.snapshot();
        let Some(galaxy) = self.runner.galaxy.clone() else {
            self.say(lookup::NO_STAR_MAP);
            return Outcome::Spoken(lookup::NO_STAR_MAP.into());
        };
        if game.system.is_none() || game.star_pos.is_none() {
            self.say(lookup::NO_POSITION);
            return Outcome::Spoken(lookup::NO_POSITION.into());
        }
        let filters = {
            let st = self.store.as_ref().map(|s| s.lock().unwrap_or_else(|e| e.into_inner()));
            crate::lookups::filters_for(st.as_deref(), command, &game)
        };
        self.say("Looking that up.");
        let (speaker, now) = (self.runner.speaker.clone(), self.clock.now_s());
        let journals = self.projects.dir().map(std::path::Path::to_path_buf);
        Outcome::LookingUp(std::thread::spawn(move || {
            let m = crate::lookups::faction_answer(&galaxy, &filters, q, &game, journals.as_deref(), now);
            speaker.say(&m);
            m
        }))
    }

    /// Route following and paste (docs/PLOTTING.md). None: not one of these.
    fn route_command(&self, command: &'static str, origin: Origin) -> Option<Handled> {
        let cmd = route::Command::from_id(command)?;
        if cmd == route::Command::Paste {
            return Some(Handled::Ran { command, outcome: self.runner.paste(origin) });
        }
        let m = match &self.route {
            None => route::NO_ROUTE.to_string(),
            Some(d) => route::command_reply(d, cmd),
        };
        self.say(&m);
        Some(Handled::Ran { command, outcome: Outcome::Spoken(m) })
    }

    fn dispatch(
        &self,
        command: &'static str,
        (slot, slot_text): (Option<&str>, Option<&str>),
        origin: Origin,
        confirmed: bool,
    ) -> Handled {
        if let Some(h) = self.route_command(command, origin) {
            return h;
        }
        if lookup::is_lookup(command) {
            return match lookup::want(command, slot, slot_text) {
                Some(want) => self.look_up(command, want),
                None => {
                    self.say(lookup::NOT_CAUGHT);
                    Handled::NotUnderstood
                }
            };
        }
        if let Some(q) = astromech_tools::faction::from_command(command) {
            return Handled::Ran { command, outcome: self.faction_lookup(command, q) };
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
        if let Some(target) = astromech_tools::targets::from_command(command) {
            return Handled::Ran { command, outcome: self.targets_lookup(target) };
        }
        if let Some(ask) = astromech_tools::trade::from_command(command) {
            let game = self.runner.hub.snapshot();
            let filters = {
                let st = self.store.as_ref().map(|s| s.lock().unwrap_or_else(|e| e.into_inner()));
                crate::lookups::filters_for(st.as_deref(), ask.lookup(), &game)
            };
            let s = crate::trade::answer(&self.runner.remote, &filters, ask, slot, &game, self.clock.now_s());
            self.say(&s);
            return Handled::Ran { command, outcome: Outcome::Spoken(s) };
        }
        if let Some(ask) = astromech_tools::colonisation::from_command(command) {
            return Handled::Ran { command, outcome: self.colonisation(ask, slot) };
        }
        if let Some(ask) = astromech_tools::exploration::from_command(command) {
            return Handled::Ran { command, outcome: self.exploration(ask) };
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

    /// Answer a colonisation question off this thread (a replay of the journals can take a
    /// while); says "Looking that up." first.
    fn colonisation(&self, ask: astromech_tools::colonisation::Ask, commodity: Option<&str>) -> Outcome {
        let game = self.runner.hub.snapshot();
        let filters = {
            let st = self.store.as_ref().map(|s| s.lock().unwrap_or_else(|e| e.into_inner()));
            crate::lookups::filters_for(st.as_deref(), astromech_market::list::LOOKUP, &game)
        };
        // Every question can replay the journals (they grow all through play), as well as search.
        self.say("Looking that up.");
        let (projects, galaxy, speaker) =
            (self.projects.clone(), self.runner.galaxy.clone(), self.runner.speaker.clone());
        let (commodity, now) = (commodity.map(str::to_string), self.clock.now_s());
        Outcome::LookingUp(std::thread::spawn(move || {
            let lookup = galaxy.as_deref().map(|g| (g, &filters));
            let m = crate::colonisation::answer(ask, &projects.get(), commodity.as_deref(), lookup, &game, now);
            speaker.say(&m);
            m
        }))
    }

    /// Answer an exploration question off this thread (the journals and the history are read,
    /// and folded again when they changed); says "Looking that up." first.
    fn exploration(&self, ask: astromech_tools::exploration::Ask) -> Outcome {
        self.say("Looking that up.");
        let (exploration, speaker) = (self.exploration.clone(), self.runner.speaker.clone());
        Outcome::LookingUp(std::thread::spawn(move || {
            let m = crate::exploration::answer(ask, &exploration);
            speaker.say(&m);
            m
        }))
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
