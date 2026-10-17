//! # astromech-announce
//!
//! The event → reaction engine (ARCHITECTURE.md §5.15). Rules look at journal lines and
//! Status.json changes and may answer with a [`Callout`]: a short line for speech, an in-app
//! toast and/or a Windows notification. The [`Engine`] decides what actually fires:
//!
//! - **Replay is silent.** While not live ([`Engine::set_live`]; the pipeline says which events
//!   are: not the startup catch-up of the journal and Status.json, not a journal re-read as a
//!   replay) no rule runs. The engine still tracks docking and combat from those events so quiet
//!   is right at once.
//! - **Per-rule toggle and cooldown** from [`Settings`] (`[callouts.<rule id>]` in
//!   `astromech.toml`). A disabled rule is never asked. A rule that fired stays silent for its
//!   cooldown, so a burst of the same event gives one callout; only a callout of higher
//!   priority than the one that started the cooldown gets through.
//! - **Quiet during docking and combat** (see [`Quiet`]): speech is dropped unless the callout
//!   is [`Priority::Urgent`]; toasts and notifications still go.
//! - **Status.json and NavRoute.json are deduplicated**: a rewrite with the same contents is not
//!   an event, nor is a cleared route.
//!
//! Speech that fires goes through a [`SpeechQueue`]: highest priority first, stale lines
//! dropped, low priority dropped first when it is full. Rules mostly decide from the event and the
//! game state after it, never the replayed history; the few that remember something between
//! events (`missions`: which expiry warnings it gave) keep it themselves.
//!
//! The rules that ship are in [`builtin`]: [`prospector`], [`scan`], [`arrival`], [`powerplay_arrival`] (off by default), [`docking`], [`fuel`], [`route`], [`hge`], [`materials_cap`], [`missions`], [`project_market`], [`signals`], [`discovery`] and [`session_summary`].

use astromech_gamestate::GameState;
use astromech_journal::companion::MarketSnap;
use astromech_journal::{JournalLine, NavRouteSnap};
use astromech_status::{StatusSnap, FLAG_BEING_INTERDICTED, FLAG_IN_DANGER};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

/// How much a callout matters. Higher goes first; only `Urgent` speaks while it is quiet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Low,
    Normal,
    High,
    /// Speaks even during docking and combat (e.g. a cargo scan with stolen goods aboard).
    Urgent,
}

/// Where a callout goes. Any combination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Channels {
    pub speech: bool,
    /// A toast in the app window (and the overlay, once it has one).
    pub toast: bool,
    /// A Windows notification.
    pub notification: bool,
}

impl Channels {
    pub const SPEECH: Channels = Channels { speech: true, toast: false, notification: false };
    pub const TOAST: Channels = Channels { speech: false, toast: true, notification: false };
    pub const NOTIFICATION: Channels = Channels { speech: false, toast: false, notification: true };

    pub fn is_empty(&self) -> bool {
        !(self.speech || self.toast || self.notification)
    }
}

impl std::ops::BitOr for Channels {
    type Output = Channels;
    fn bitor(self, o: Channels) -> Channels {
        Channels {
            speech: self.speech || o.speech,
            toast: self.toast || o.toast,
            notification: self.notification || o.notification,
        }
    }
}

/// What a rule wants said or shown.
#[derive(Debug, Clone, PartialEq)]
pub struct Callout {
    pub text: String,
    pub priority: Priority,
    pub channels: Channels,
}

/// A callout that fired, with the rule that made it and when.
#[derive(Debug, Clone, PartialEq)]
pub struct Reaction {
    pub rule: &'static str,
    pub text: String,
    pub priority: Priority,
    pub channels: Channels,
    pub at: Instant,
}

/// One event for the rules.
#[derive(Debug, Clone, Copy)]
pub enum Input<'a> {
    Journal(&'a JournalLine),
    /// A Status.json read that differs from the previous one (`before` is None for the first).
    Status {
        now: &'a StatusSnap,
        before: Option<&'a StatusSnap>,
    },
    /// A newly plotted route: `NavRoute.json` with contents different from the last read, not
    /// empty (a cleared route isn't an event).
    Route(&'a NavRouteSnap),
    /// A fresh `Market.json` for the market docked at, once per dock (see [`Engine::market`]).
    Market(&'a MarketSnap),
    /// The clock moved on: `wall_secs` is the time now in seconds since the Unix epoch.
    Tick {
        wall_secs: i64,
    },
}

/// A reaction rule. Decides from the event and the state after it; a rule that must remember
/// what it already said (once-only warnings) keeps that itself, behind `&self`.
pub trait Rule: Send {
    /// Stable id, used in `[callouts.<id>]`.
    fn id(&self) -> &'static str;
    /// Cooldown unless the settings give one.
    fn cooldown(&self) -> Duration;
    /// Whether the rule runs unless the settings say otherwise. Most do; a rule that is off
    /// until the commander asks for it returns false.
    fn default_enabled(&self) -> bool {
        true
    }
    /// Told once by the engine whether the rule is enabled (settings applied), for rules that
    /// others adjust to.
    fn configured(&self, _enabled: bool) {}
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout>;
    /// Several lines for one event (each its own sentence within the word budget). They share
    /// the rule's cooldown. By default, [`react`](Rule::react)'s one line.
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        self.react(input, state).into_iter().collect()
    }
    /// The cooldown holds back only a repeat of the same line, not a different one from the
    /// same rule (distinct warnings that must each be heard). Default: any line.
    fn cooldown_per_line(&self) -> bool {
        false
    }
}

pub mod arrival;
pub mod discovery;
pub mod docking;
pub mod fuel;
pub mod hge;
pub mod materials_cap;
pub mod missions;
pub mod powerplay_arrival;
pub mod project_market;
pub mod prospector;
pub mod route;
pub mod scan;
pub mod session_summary;
pub mod signals;

/// What the core keeps for the rules and may change while they run.
#[derive(Debug, Clone, Default)]
pub struct Shared {
    /// Prospector thresholds (Settings changes them).
    pub prospector: prospector::SharedThresholds,
    /// What the Engineering plans still have to collect (the app sends it).
    pub shopping: arrival::SharedShoppingList,
    /// Whether the `powerplay_arrival` rule is on (the engine sets it), so `arrival` leaves out
    /// the Powerplay line it would repeat.
    pub powerplay_spoken: powerplay_arrival::SharedFlag,
    /// The commander's visit history, for the arrival briefing (the core plugs it in).
    pub visits: arrival::SharedVisits,
    /// The commander's session history, for the session summary (the core plugs it in).
    pub sessions: session_summary::SharedSessions,
}

/// The rules that ship.
pub fn builtin(shared: Shared) -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(prospector::Prospector { thresholds: shared.prospector }),
        Box::new(scan::Scan),
        Box::new(arrival::Arrival {
            shopping: shared.shopping.clone(),
            visits: shared.visits.clone(),
            powerplay_spoken: shared.powerplay_spoken.clone(),
        }),
        Box::new(powerplay_arrival::PowerplayArrival { spoken: shared.powerplay_spoken.clone() }),
        Box::new(docking::Docking),
        Box::new(fuel::Scoopable),
        Box::new(fuel::FuelRange),
        Box::new(route::Route),
        Box::new(hge::Hge { shopping: shared.shopping.clone() }),
        Box::new(materials_cap::MaterialsCap),
        Box::new(missions::Missions::default()),
        Box::new(project_market::ProjectMarket),
        Box::new(signals::Signals::default()),
        Box::new(discovery::Discovery::default()),
        Box::new(session_summary::SessionSummary::new(shared.sessions.clone())),
    ]
}

/// Per-rule settings, `[callouts.<rule id>]` in `astromech.toml`. Missing means the default.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RuleSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_s: Option<f64>,
}

/// All `[callouts.*]` tables, by rule id.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Settings(pub BTreeMap<String, RuleSettings>);

/// Why speech is held back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quiet {
    /// After `DockingGranted` (the approach) until `Docked`, `DockingCancelled`,
    /// `DockingDenied`, `DockingTimeout`, or leaving (`Undocked`, `SupercruiseEntry`,
    /// `FSDJump`, `Died`, a new game). The grant itself is not quiet, so its pad line speaks.
    Docking,
    /// Status.json says in danger or being interdicted, or an `UnderAttack` in the last
    /// [`ATTACK_QUIET`].
    Combat,
}

/// How long one `UnderAttack` keeps it quiet.
pub const ATTACK_QUIET: Duration = Duration::from_secs(30);

const DOCKING_STARTS: &[&str] = &["DockingGranted"];
const DOCKING_ENDS: &[&str] = &[
    "Docked",
    "DockingCancelled",
    "DockingDenied",
    "DockingTimeout",
    "Undocked",
    "SupercruiseEntry",
    "FSDJump",
    "Died",
    "LoadGame",
    "Shutdown",
];

struct Slot {
    rule: Box<dyn Rule>,
    enabled: bool,
    cooldown: Duration,
    /// When the rule last fired, and at what priority.
    fired: Option<(Instant, Priority)>,
    /// For [`Rule::cooldown_per_line`] rules: each line said within the cooldown, when and at
    /// what priority.
    said: Vec<(String, Instant, Priority)>,
}

pub struct Engine {
    slots: Vec<Slot>,
    problems: Vec<String>,
    live: bool,
    docking: bool,
    danger_flags: bool,
    attacked_until: Option<Instant>,
    last_status: Option<StatusSnap>,
    last_route: Option<NavRouteSnap>,
    /// The market a live `Market.json` was already handled for since the last dock event.
    market_done: Option<i64>,
}

impl Engine {
    pub fn new(rules: Vec<Box<dyn Rule>>, settings: &Settings) -> Self {
        let mut problems = vec![];
        let slots: Vec<Slot> = rules
            .into_iter()
            .map(|rule| {
                let s = settings.0.get(rule.id()).cloned().unwrap_or_default();
                let cooldown = match s.cooldown_s.map(|c| (c, Duration::try_from_secs_f64(c))) {
                    None => rule.cooldown(),
                    Some((_, Ok(d))) => d,
                    Some((c, Err(_))) => {
                        let d = rule.cooldown();
                        problems.push(format!(
                            "[callouts.{}] cooldown_s = {c} is not a usable number of seconds; using {} s",
                            rule.id(),
                            d.as_secs_f64()
                        ));
                        d
                    }
                };
                let enabled = s.enabled.unwrap_or_else(|| rule.default_enabled());
                rule.configured(enabled);
                Slot { enabled, cooldown, fired: None, said: vec![], rule }
            })
            .collect();
        for id in settings.0.keys().filter(|id| !slots.iter().any(|s| s.rule.id() == id.as_str())) {
            problems.push(format!("[callouts.{id}] names no callout; ignored"));
        }
        Self {
            slots,
            problems,
            live: false,
            docking: false,
            danger_flags: false,
            attacked_until: None,
            last_status: None,
            last_route: None,
            market_done: None,
        }
    }

    /// What was wrong with the settings (unknown rule ids, unusable cooldowns), for the log.
    pub fn setting_problems(&self) -> &[String] {
        &self.problems
    }

    /// Whether the next events are live (may fire) or replayed history. The pipeline decides
    /// per event: replayed journal lines and the startup catch-up are not live.
    pub fn set_live(&mut self, live: bool) {
        self.live = live;
    }

    pub fn quiet(&self, now: Instant) -> Option<Quiet> {
        if self.danger_flags || self.attacked_until.is_some_and(|t| now < t) {
            Some(Quiet::Combat)
        } else if self.docking {
            Some(Quiet::Docking)
        } else {
            None
        }
    }

    /// A journal line, already applied to `state`.
    pub fn journal(&mut self, line: &JournalLine, state: &GameState, now: Instant) -> Vec<Reaction> {
        // The event is judged against the mode before it: `DockingGranted` (the pad) is not
        // "during docking"; what follows it is.
        let mut quiet = self.quiet(now);
        let name = line.name.as_str();
        if DOCKING_STARTS.contains(&name) {
            self.docking = true;
        } else if DOCKING_ENDS.contains(&name) {
            self.docking = false;
            self.market_done = None;
            // Arriving ends the approach: what `Docked` says (trader suggestions) is not "during
            // docking". Combat still holds it back.
            if name == "Docked" {
                quiet = self.quiet(now);
            }
        }
        if name == "UnderAttack" && self.live {
            self.attacked_until = Some(now + ATTACK_QUIET);
        }
        self.fire(&Input::Journal(line), state, now, quiet)
    }

    /// A Status.json read, already applied to `state`. Unchanged contents are ignored.
    pub fn status(&mut self, snap: &StatusSnap, state: &GameState, now: Instant) -> Vec<Reaction> {
        if self.last_status.as_ref() == Some(snap) {
            return vec![];
        }
        let quiet = self.quiet(now);
        self.danger_flags = snap.flags & (FLAG_IN_DANGER | FLAG_BEING_INTERDICTED) != 0;
        let before = self.last_status.replace(snap.clone());
        self.fire(&Input::Status { now: snap, before: before.as_ref() }, state, now, quiet)
    }

    /// `NavRoute.json` was read, already applied to `state`. The same route again, or an empty
    /// one (cleared), is not an event.
    pub fn route(&mut self, route: &NavRouteSnap, state: &GameState, now: Instant) -> Vec<Reaction> {
        if self.last_route.as_ref() == Some(route) {
            return vec![];
        }
        self.last_route = Some(route.clone());
        if route.hops.is_empty() {
            return vec![];
        }
        let quiet = self.quiet(now);
        self.fire(&Input::Route(route), state, now, quiet)
    }

    /// A fresh `Market.json`, already applied to `state` (the pipeline only offers a file newer
    /// than the `Market` event that announced it). Handled once per dock: a rewrite while still
    /// docked there (opening the market again) is not an event. Only a live file counts, so the
    /// catch-up replay of the same dock doesn't use up the dock's one chance.
    pub fn market(&mut self, market: &MarketSnap, state: &GameState, now: Instant) -> Vec<Reaction> {
        let Some(id) = market.market_id else { return vec![] };
        if !self.live || self.market_done == Some(id) {
            return vec![];
        }
        self.market_done = Some(id);
        let quiet = self.quiet(now);
        self.fire(&Input::Market(market), state, now, quiet)
    }

    /// The clock moved on (`wall_secs`: seconds since the Unix epoch), for rules that remind by
    /// time. The pipeline calls it about once a second; replay tests inject their own time.
    pub fn tick(&mut self, wall_secs: i64, state: &GameState, now: Instant) -> Vec<Reaction> {
        let quiet = self.quiet(now);
        self.fire(&Input::Tick { wall_secs }, state, now, quiet)
    }

    fn fire(&mut self, input: &Input, state: &GameState, now: Instant, quiet: Option<Quiet>) -> Vec<Reaction> {
        if !self.live {
            return vec![];
        }
        let mut out = vec![];
        for s in self.slots.iter_mut().filter(|s| s.enabled) {
            let mut cs = s.rule.react_all(input, state);
            // A higher priority isn't held back by a cooldown a lower one started (a shown-only
            // crime scan must not swallow the urgent stolen-goods line seconds later).
            let held =
                |t: Instant, p: Priority, c: &Callout| c.priority <= p && now.saturating_duration_since(t) < s.cooldown;
            if s.rule.cooldown_per_line() {
                s.said.retain(|(_, t, _)| now.saturating_duration_since(*t) < s.cooldown);
                cs.retain(|c| !s.said.iter().any(|(text, t, p)| *text == c.text && held(*t, *p, c)));
            }
            let Some(top) = cs.iter().map(|c| c.priority).max() else { continue };
            if !s.rule.cooldown_per_line()
                && s.fired.is_some_and(|(t, p)| top <= p && now.saturating_duration_since(t) < s.cooldown)
            {
                continue;
            }
            for c in cs {
                let mut channels = c.channels;
                if quiet.is_some() && c.priority < Priority::Urgent {
                    channels.speech = false;
                }
                if channels.is_empty() {
                    continue;
                }
                s.fired = Some((now, top));
                if s.rule.cooldown_per_line() {
                    s.said.retain(|(text, _, _)| *text != c.text);
                    s.said.push((c.text.clone(), now, c.priority));
                }
                out.push(Reaction { rule: s.rule.id(), text: c.text, priority: c.priority, channels, at: now });
            }
        }
        out
    }
}

/// Spoken callouts waiting for the speaker: highest priority first, then oldest. Lines older
/// than `max_age` are dropped (late is wrong); when full, the lowest priority goes first.
#[derive(Debug)]
pub struct SpeechQueue {
    items: Vec<Reaction>,
    cap: usize,
    max_age: Duration,
}

impl Default for SpeechQueue {
    fn default() -> Self {
        Self::new(8, Duration::from_secs(10))
    }
}

impl SpeechQueue {
    pub fn new(cap: usize, max_age: Duration) -> Self {
        Self { items: vec![], cap: cap.max(1), max_age }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push(&mut self, r: Reaction) {
        self.items.push(r);
        if self.items.len() > self.cap {
            // Lowest priority; among those the oldest (items are in arrival order).
            let lowest = self.items.iter().map(|r| r.priority).min().unwrap_or(Priority::Low);
            if let Some(i) = self.items.iter().position(|r| r.priority == lowest) {
                self.items.remove(i);
            }
        }
    }

    pub fn pop(&mut self, now: Instant) -> Option<Reaction> {
        let max_age = self.max_age;
        self.items.retain(|r| now.saturating_duration_since(r.at) <= max_age);
        let top = self.items.iter().map(|r| r.priority).max()?;
        let i = self.items.iter().position(|r| r.priority == top)?;
        Some(self.items.remove(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(text: &str, priority: Priority, at: Instant) -> Reaction {
        Reaction { rule: "t", text: text.into(), priority, channels: Channels::SPEECH, at }
    }

    #[test]
    fn queue_orders_by_priority_then_age_and_drops_stale_and_low() {
        let t0 = Instant::now();
        let mut q = SpeechQueue::new(3, Duration::from_secs(10));
        q.push(r("low", Priority::Low, t0));
        q.push(r("normal 1", Priority::Normal, t0));
        q.push(r("urgent", Priority::Urgent, t0));
        q.push(r("normal 2", Priority::Normal, t0)); // full: "low" goes
        assert_eq!(q.len(), 3);
        let order: Vec<String> = std::iter::from_fn(|| q.pop(t0)).map(|r| r.text).collect();
        assert_eq!(order, ["urgent", "normal 1", "normal 2"]);
        q.push(r("old", Priority::High, t0));
        assert_eq!(q.pop(t0 + Duration::from_secs(11)), None);
        assert!(q.is_empty());
    }

    #[test]
    fn settings_read_from_toml() {
        let s: Settings = toml::from_str("[prospector]\nenabled = false\n[scan]\ncooldown_s = 2.5\n").unwrap();
        assert_eq!(s.0["prospector"].enabled, Some(false));
        assert_eq!(s.0["scan"].cooldown_s, Some(2.5));
        assert_eq!(toml::to_string(&s).unwrap(), "[prospector]\nenabled = false\n\n[scan]\ncooldown_s = 2.5\n");
    }
}
