//! System biology summary (`biology`): what the system's biology is worth, ranked by value, from
//! `astromech_exploration::biology` over the game state's system bodies (FSS counts, DSS genus
//! lists, species predictions),
//! and on arrival from what other players reported (Spansh).
//!
//! - **On arrival** (`FSDJump`, `CarrierJump`, or a login: `Location`, once a system), before any
//!   scan: "Known biology on 3 bodies." and the most valuable known thing: "A 2: Stratum
//!   Tectonicas, 19 M." It comes from a [`KnownBiology`] the core plugs in, which answers from
//!   what it has and looks the system up in the background; an answer that arrives later is
//!   said on the clock tick, until the first FSS or DSS biology result in the system or a minute
//!   after the arrival, whichever is first. A failed lookup is tried again within that minute.
//!   A login into a system whose own FSS or DSS results are already in says nothing.
//!   Part of the arrival briefing: it follows `arrival`'s lines.
//! - **After the FSS**: leaving the FSS scanner (Status.json's `GuiFocus` leaves 9), "Biology on
//!   3 bodies." and the most valuable body's best genus: "A 2: Stratum, up to 19 M." (the top
//!   of the predicted range; a species the conditions single out is named). Only when it
//!   changed since it was last said in the system.
//! - **After the DSS** (`SAASignalsFound` with genera): that body's best genus, now from its
//!   real genus list: "A 2: Stratum, up to 19 M."
//!
//! Per-body signal counts are never spoken: the FSS and the system map show them. Values are
//! Vista Genomics base values, before the first-logged bonus.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_exobiology::known::{self, KnownBody};
use astromech_exploration::biology;
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, JournalEvent};
use astromech_status::GUI_FSS;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

/// Remote knowledge of a system's biology. Called on the pipeline thread, so it must never
/// block: it answers from what it already has, and otherwise starts a lookup and says None.
pub trait KnownBiology: Send + Sync {
    /// The bodies with biology in `system_address` (empty: none known), or None while it
    /// doesn't know yet. `star_pos` helps the lookup.
    fn known(&self, system_address: i64, star_pos: Option<[f64; 3]>) -> Option<Vec<KnownBody>>;
}

/// The lookup the core plugs in once it has one.
#[derive(Clone, Default)]
pub struct SharedKnownBio(Arc<RwLock<Option<Arc<dyn KnownBiology>>>>);

impl SharedKnownBio {
    pub fn set(&self, k: Arc<dyn KnownBiology>) {
        *self.0.write().unwrap() = Some(k);
    }

    pub fn get(&self) -> Option<Arc<dyn KnownBiology>> {
        self.0.read().unwrap().clone()
    }
}

impl std::fmt::Debug for SharedKnownBio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SharedKnownBio({})", if self.get().is_some() { "set" } else { "none" })
    }
}

/// How long after a jump a late remote answer is still worth saying, seconds.
pub const LATE_KNOWN_S: i64 = 60;

/// "19 M", "1.4 M": credits in millions, one decimal under ten million.
pub fn millions(v: i64) -> String {
    let m = v as f64 / 1e6;
    if m >= 9.95 {
        format!("{} M", m.round())
    } else {
        let s = format!("{m:.1}");
        format!("{} M", s.trim_end_matches(".0"))
    }
}

/// "Biology on 3 bodies." / "Known biology on 1 body."
pub fn count_line(bodies: usize, known: bool) -> String {
    let what = if known { "Known biology" } else { "Biology" };
    let s = if bodies == 1 { "body" } else { "bodies" };
    format!("{what} on {bodies} {s}.")
}

/// "A 2: Stratum, up to 19 M." for the body `short` (its name without the system's), the best
/// `label` (genus or species), its value and whether that value is exact. To keep to the
/// eight-word budget on a long body name, "up to" is left out, then a species is called by its
/// genus.
pub fn body_line(short: &str, label: &str, value: i64, exact: bool) -> String {
    let words = |l: &str| short.split_whitespace().count() + l.split_whitespace().count() + 2;
    let label = match astromech_exobiology::catalog::species(label) {
        Some((g, _)) if words(label) > 8 => g.name.as_str(),
        _ => label,
    };
    let up_to = if exact || words(label) + 2 > 8 { "" } else { "up to " };
    format!("{short}: {label}, {up_to}{}.", millions(value))
}

/// "A 2" for "Synuefe BY-F d12-39 A 2" in that system; the whole name otherwise.
pub fn short_name<'a>(body: &'a str, system: Option<&str>) -> &'a str {
    system
        .and_then(|s| body.strip_prefix(s)?.strip_prefix(' '))
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .unwrap_or(body)
}

/// The lines for what other players reported: the count and the best body. None if nothing.
pub fn known_lines(bodies: Vec<KnownBody>, system: Option<&str>) -> Vec<String> {
    let ranked = known::ranked(bodies);
    let Some(top) = ranked.first() else { return vec![] };
    let mut v = vec![count_line(ranked.len(), true)];
    if let Some((label, value, exact)) = top.best() {
        v.push(body_line(short_name(&top.name, system), &label, value, exact));
    }
    v
}

/// The lines for the journal's view of the system: the count and the best body.
pub fn summary_lines(state: &GameState) -> Vec<String> {
    let sys = &state.system_bodies;
    let ranked = biology::ranked(sys);
    let Some(top) = ranked.first() else { return vec![] };
    let mut v = vec![count_line(ranked.len(), false)];
    if let Some((label, value, exact)) = top.best() {
        v.push(body_line(short_name(&top.name, sys.system()), &label, value, exact));
    }
    v
}

/// What was said in the current system.
#[derive(Default)]
struct Said {
    system: Option<i64>,
    /// The arrival here (a jump or a login) has been handled: the known biology said or awaited.
    arrived: bool,
    /// A remote answer still awaited: until this time (Unix seconds).
    waiting_until: Option<i64>,
    /// The last FSS summary said.
    summary: Vec<String>,
    /// DSS lines said.
    mapped: Vec<String>,
}

pub struct Biology {
    pub known: SharedKnownBio,
    said: Mutex<Said>,
}

impl Biology {
    pub fn new(known: SharedKnownBio) -> Self {
        Self { known, said: Mutex::default() }
    }

    fn ask(&self, state: &GameState) -> Option<Vec<KnownBody>> {
        let addr = state.system_bodies.system_address()?;
        self.known.get()?.known(addr, state.star_pos)
    }
}

impl Rule for Biology {
    fn id(&self) -> &'static str {
        "biology"
    }
    /// Its own memory decides what is new.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        self.react_all(input, state).into_iter().next()
    }
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        let say = |lines: Vec<String>| -> Vec<Callout> {
            lines
                .into_iter()
                .map(|text| Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
                .collect()
        };
        let sys = &state.system_bodies;
        let mut said = self.said.lock().unwrap();
        if said.system != sys.system_address() {
            *said = Said { system: sys.system_address(), ..Default::default() };
        }
        // Journal results in the system replace the remote ones.
        let scanned = !biology::ranked(sys).is_empty();
        if scanned {
            said.waiting_until = None;
        }
        match input {
            Input::Journal(l) => match &l.event {
                // `FsdJump` is a carrier jump too; `Location` a login or a respawn, said once a
                // system.
                JournalEvent::FsdJump { .. } | JournalEvent::Location { .. } if !said.arrived => {
                    said.arrived = true;
                    // A relog into a system already scanned (the scans in the startup catch-up).
                    if scanned {
                        return vec![];
                    }
                    match self.ask(state) {
                        Some(bodies) => say(known_lines(bodies, sys.system())),
                        None => {
                            let ts = chrono::DateTime::parse_from_rfc3339(&l.timestamp).ok().map(|t| t.timestamp());
                            said.waiting_until = ts.map(|t| t + LATE_KNOWN_S);
                            vec![]
                        }
                    }
                }
                JournalEvent::Extra(ExtraEvent::SaaSignalsFound(s)) if !s.genuses.is_empty() => {
                    let Some(b) = s.body_id.and_then(|id| biology::body(sys, id)) else {
                        return vec![];
                    };
                    let Some((label, value, exact)) = b.best() else { return vec![] };
                    let line = body_line(short_name(&b.name, sys.system()), &label, value, exact);
                    if said.mapped.contains(&line) {
                        return vec![];
                    }
                    said.mapped.push(line.clone());
                    say(vec![line])
                }
                _ => vec![],
            },
            Input::Status { now, before } => {
                let left_fss = before.is_some_and(|b| b.gui_focus == Some(GUI_FSS)) && now.gui_focus != Some(GUI_FSS);
                if !left_fss {
                    return vec![];
                }
                let lines = summary_lines(state);
                if lines.is_empty() || lines == said.summary {
                    return vec![];
                }
                said.summary = lines.clone();
                say(lines)
            }
            Input::Tick { wall_secs } => {
                let Some(until) = said.waiting_until else { return vec![] };
                if *wall_secs > until {
                    said.waiting_until = None;
                    return vec![];
                }
                match self.ask(state) {
                    Some(bodies) => {
                        said.waiting_until = None;
                        say(known_lines(bodies, sys.system()))
                    }
                    None => vec![],
                }
            }
            _ => vec![],
        }
    }
}
