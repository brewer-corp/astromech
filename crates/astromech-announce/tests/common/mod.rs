//! A live engine with the built-in rules, fed synthetic journal lines (and companion files)
//! the way the pipeline feeds it: the game state takes each line first.

#![allow(dead_code)]

use astromech_announce::*;
use astromech_gamestate::GameState;
use astromech_journal::companion::{parse_fresh, CompanionKind};
use astromech_journal::{parse_line, parse_nav_route};
use std::time::{Duration, Instant};

pub struct Run {
    pub engine: Engine,
    pub state: GameState,
    pub shared: Shared,
    t0: Instant,
    n: u64,
}

impl Run {
    pub fn new() -> Self {
        Self::with(Shared::default(), &Settings::default())
    }

    pub fn with(shared: Shared, settings: &Settings) -> Self {
        let mut engine = Engine::new(builtin(shared.clone()), settings);
        engine.set_live(true);
        Self { engine, state: GameState::default(), shared, t0: Instant::now(), n: 0 }
    }

    /// Everything that fired for one journal line, one second after the previous line.
    pub fn feed_all(&mut self, line: &str) -> Vec<Reaction> {
        let j = parse_line(line).unwrap();
        self.state.apply_line(&j);
        self.n += 1;
        self.engine.journal(&j, &self.state, self.t0 + Duration::from_secs(self.n))
    }

    /// The game wrote `NavRoute.json` (the pipeline reads it into the state).
    pub fn route(&mut self, json: &str) {
        self.state.apply_route(&parse_nav_route(json.as_bytes()).unwrap());
    }

    /// The game wrote a new `NavRoute.json`: into the state, then to the engine. Spoken lines.
    pub fn plot(&mut self, json: &str) -> Vec<String> {
        let snap = parse_nav_route(json.as_bytes()).unwrap();
        self.state.apply_route(&snap);
        self.n += 1;
        let out = self.engine.route(&snap, &self.state, self.t0 + Duration::from_secs(self.n));
        out.into_iter().filter(|r| r.channels.speech).map(|r| r.text).collect()
    }

    /// The game wrote `Market.json` after a `Market` event stamped `event_ts`: read the way the
    /// pipeline does (a file older than its event is not used), into the state, then to the
    /// engine. Spoken lines.
    pub fn market(&mut self, json: &str, event_ts: &str) -> Vec<String> {
        let Ok(file) = parse_fresh(CompanionKind::Market, Some(json.as_bytes()), event_ts) else { return vec![] };
        self.state.apply_companion(file);
        self.n += 1;
        let m = self.state.companions.market.clone().unwrap();
        let out = self.engine.market(&m, &self.state, self.t0 + Duration::from_secs(self.n));
        out.into_iter().filter(|r| r.channels.speech).map(|r| r.text).collect()
    }

    /// Spoken lines for one or more journal lines (one per text line).
    pub fn spoken(&mut self, lines: &str) -> Vec<String> {
        let mut out = vec![];
        for l in lines.lines().map(str::trim).filter(|l| !l.is_empty()) {
            out.extend(self.feed_all(l).into_iter().filter(|r| r.channels.speech).map(|r| r.text));
        }
        out
    }
}
