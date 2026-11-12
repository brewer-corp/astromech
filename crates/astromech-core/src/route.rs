//! Following a multi-stop route (docs/PLOTTING.md, layer 1). The game has no API for setting a
//! destination, so the next waypoint goes on the clipboard, ready to paste into the galaxy
//! map's search box, and is announced. Arriving in a waypoint's system (the journal's
//! `FSDJump`, `CarrierJump` or `Location`, live only: the callout engine's `route_follow` rule
//! calls [`Desk`] as a `Follower`) moves the route on and copies the stop after it.
//!
//! A route is never dropped silently. Following a new one from the window ([`Desk::set`]: Set
//! destination, the engineer trip, an import, the plotter) parks the unfinished route it
//! replaces, and Resume brings that back. Add stop ([`Desk::add`]) makes a place the next stop
//! of the route followed instead of replacing it.
//!
//! A route plotted in the galaxy map is followed too ([`Desk::plotted`]). If it ends at a
//! waypoint still ahead on the route being followed, it is a leg of that route (a neutron route's
//! next neutron star) and nothing changes. Otherwise it becomes the route, silently and without
//! the clipboard (the game has it plotted and its HUD shows the next jump), and a route it
//! replaces is parked: Resume brings it back, and so does plotting to one of its waypoints in the
//! galaxy map. Only a route of our own is parked: the game keeps its own plotted route.
//!
//! [`Route`] is the pure part. [`Desk`] owns the one active route and the parked one: it saves
//! them to `route.json`, so they survive restarts and can be set from the app window while Elite
//! is not running, and it does the clipboard writes.
//!
//! (Not to be confused with the window's `--route`, which is a page in the app.)
//!
//! The Route page's timeline is [`View::stops`]: what each source says about the route's stops
//! and legs, and nothing it doesn't say ([`StopView`] gives the contract per source).

use astromech_api::Recover;
use astromech_galaxy::plot::{Hop, Settings};
use astromech_galaxy::stops::Details;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

/// A stop on the route. Every field but `system` is optional and present only where the route's
/// source said it; `route.json` from before a field was recorded loads without it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ts_rs::TS)]
#[ts(optional_fields = nullable)]
pub struct Waypoint {
    pub system: String,
    /// What is there, e.g. the engineer's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Jumps from the waypoint before, where the route knows them: a plotted route
    /// (`astromech_galaxy::plot`), one plotted in the galaxy map and a CSV with a Jumps column
    /// do, a plain list or a lookup's stop doesn't.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jumps: Option<u32>,
    /// Light years from the waypoint before: a plotted route, and a Spansh CSV's leg distance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ly: Option<f64>,
    /// `StarClass`, as `NavRoute.json` writes it: a route plotted in the galaxy map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub star_class: Option<String>,
    /// Whether its star can be scooped, as the plotter flags it (exact routes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoopable: Option<bool>,
    /// Whether its star is a neutron star, as the plotter or a CSV's Neutron Star column says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neutron: Option<bool>,
    /// The leg's jumps, the last being this stop, as the plotter listed them (exact routes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hops: Option<Vec<Hop>>,
}

impl Waypoint {
    pub fn new(system: &str) -> Self {
        Self { system: system.trim().to_string(), ..Self::default() }
    }

    /// The leg to it is no longer the one its source described (a stop was put before it).
    fn forget_leg(&mut self) {
        self.jumps = None;
        self.ly = None;
        self.hops = None;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "FollowedRoute")]
pub struct Route {
    pub name: String,
    /// Where it came from: "engineer trip", "nearby", "import".
    #[serde(default)]
    pub source: String,
    pub waypoints: Vec<Waypoint>,
    /// Index of the next waypoint; `waypoints.len()` once the route is finished.
    #[serde(default)]
    pub next: usize,
    /// What a plotted route was plotted with, for `route_replot`. Other routes, and plotted
    /// ones saved before it was recorded, have none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub plot: Option<Settings>,
}

/// The source of a route plotted in the galaxy map.
pub const GAME: &str = "galaxy map";

fn same_system(a: &str, b: &str) -> bool {
    a.trim().eq_ignore_ascii_case(b.trim())
}

/// Two stops in one system are one jump target: "Felicity Farseer, Someone Else".
fn merge_labels(a: Option<String>, b: Option<String>) -> Option<String> {
    match (a, b) {
        (Some(a), Some(b)) if a != b => Some(format!("{a}, {b}")),
        (a, b) => a.or(b),
    }
}

impl Route {
    /// A route over `waypoints`, with blank systems dropped and consecutive stops in the same
    /// system merged (two engineers in one system are one jump target).
    pub fn new(name: &str, source: &str, waypoints: Vec<Waypoint>) -> Self {
        let mut out: Vec<Waypoint> = vec![];
        for w in waypoints {
            let w =
                Waypoint { system: w.system.trim().to_string(), label: w.label.filter(|l| !l.trim().is_empty()), ..w };
            if w.system.is_empty() {
                continue;
            }
            match out.last_mut() {
                Some(last) if same_system(&last.system, &w.system) => {
                    last.label = merge_labels(last.label.take(), w.label);
                    // A repeat of the same system is no further jump.
                    last.jumps = last.jumps.map(|j| j + w.jumps.unwrap_or(0));
                    last.ly = last.ly.map(|l| l + w.ly.unwrap_or(0.0));
                }
                _ => out.push(w),
            }
        }
        Self { name: name.trim().to_string(), source: source.to_string(), waypoints: out, next: 0, plot: None }
    }

    /// The same route, kept with what it was plotted with.
    pub fn plotted_with(self, plot: Option<Settings>) -> Self {
        Self { plot, ..self }
    }

    pub fn next_stop(&self) -> Option<&Waypoint> {
        self.waypoints.get(self.next)
    }

    /// Waypoints still to reach, the next one included.
    pub fn remaining(&self) -> usize {
        self.waypoints.len().saturating_sub(self.next)
    }

    pub fn finished(&self) -> bool {
        self.next >= self.waypoints.len()
    }

    /// Jumps to the end, the leg to the next waypoint included, when every waypoint still to
    /// reach knows its jumps; `None` when one doesn't (an import, a lookup's stop).
    pub fn jumps_left(&self) -> Option<u32> {
        self.waypoints.iter().skip(self.next).map(|w| w.jumps).sum()
    }

    /// Plotted in the galaxy map ([`GAME`]): followed silently, never copied.
    pub fn from_game(&self) -> bool {
        self.source == GAME
    }

    /// `system` is a waypoint still to reach.
    fn ahead(&self, system: &str) -> bool {
        self.waypoints.iter().skip(self.next).any(|w| same_system(&w.system, system))
    }

    /// Now in `system`: if it is the next waypoint, or a later one (stops skipped), the route
    /// moves past it. Returns the index arrived at.
    pub fn arrive(&mut self, system: &str) -> Option<usize> {
        let i = (self.next..self.waypoints.len()).find(|&i| same_system(&self.waypoints[i].system, system))?;
        self.next = i + 1;
        Some(i)
    }

    pub fn skip(&mut self) -> bool {
        if self.finished() {
            return false;
        }
        self.next += 1;
        true
    }

    pub fn back(&mut self) -> bool {
        if self.next == 0 {
            return false;
        }
        self.next -= 1;
        true
    }

    /// Make `stop` the next waypoint. In the same system as the next waypoint, or as the one just
    /// passed, it merges with it (as [`Route::new`] merges neighbours), and that one is next.
    fn add_next(&mut self, stop: Waypoint) {
        let i = self.next;
        if let Some(w) = self.waypoints.get_mut(i).filter(|w| same_system(&w.system, &stop.system)) {
            w.label = merge_labels(stop.label, w.label.take());
        } else if let Some(w) =
            i.checked_sub(1).map(|p| &mut self.waypoints[p]).filter(|w| same_system(&w.system, &stop.system))
        {
            w.label = merge_labels(w.label.take(), stop.label);
            self.next = i - 1;
        } else {
            // The stop after it is now reached from the new stop, so its leg is unknown too.
            if let Some(after) = self.waypoints.get_mut(i) {
                after.forget_leg();
            }
            self.waypoints.insert(i, stop);
        }
    }
}

/// Waypoints from pasted text: a Spansh CSV export (any router), or one system name per line.
///
/// A CSV gives what its columns give, as Spansh's site writes them (its `downloadCSV` column
/// lists, read 12 Nov 2026): the "System Name" column; "Jumps" (neutron, road to riches,
/// tourist and others) as each stop's jumps; the leg's light years from "Distance" (exact,
/// fleet carrier, colonisation) or, in the neutron plotter's export, "Distance To Arrival"
/// (elsewhere that column is a body's distance in light seconds, so it is read only with a
/// "Distance Remaining" column and no "Body Name"); and "Neutron Star" (Yes or No). No export
/// has a star class or a scoopable flag. Rows repeating the system before them (road to riches:
/// one row per body, each with the system's jumps) are one stop, from its first row.
pub fn parse_list(text: &str) -> Vec<Waypoint> {
    let mut lines = text.lines().map(str::trim).filter(|l| !l.is_empty()).peekable();
    let Some(first) = lines.peek() else { return vec![] };
    let header = split_csv(first);
    let col = |names: &[&str]| header.iter().position(|h| names.iter().any(|n| h.eq_ignore_ascii_case(n)));
    let out: Vec<Waypoint> = match col(&["system name", "system"]) {
        Some(c) if header.len() > 1 || first.contains('"') => {
            let jumps = col(&["jumps"]);
            let leg_ly = col(&["distance"]).or_else(|| {
                (col(&["distance remaining"]).is_some() && col(&["body name"]).is_none())
                    .then(|| col(&["distance to arrival"]))
                    .flatten()
            });
            let neutron = col(&["neutron star"]);
            let mut out: Vec<Waypoint> = vec![];
            for l in lines.skip(1) {
                let f = split_csv(l);
                let Some(system) = f.get(c) else { continue };
                let cell = |i: Option<usize>| i.and_then(|i| f.get(i)).map(String::as_str);
                let w = Waypoint {
                    jumps: cell(jumps).and_then(|j| j.parse().ok()),
                    ly: cell(leg_ly).and_then(|d| d.parse().ok()),
                    neutron: cell(neutron).and_then(|n| {
                        if n.eq_ignore_ascii_case("yes") {
                            Some(true)
                        } else if n.eq_ignore_ascii_case("no") {
                            Some(false)
                        } else {
                            None
                        }
                    }),
                    ..Waypoint::new(system)
                };
                if !out.last().is_some_and(|last| same_system(&last.system, &w.system)) {
                    out.push(w);
                }
            }
            out
        }
        _ => lines.map(|l| Waypoint::new(l.trim_matches('"'))).collect(),
    };
    Route::new("", "", out).waypoints
}

/// One CSV line, with double-quoted fields ("" is a literal quote).
fn split_csv(line: &str) -> Vec<String> {
    let mut out = vec![];
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted && chars.peek() == Some(&'"') => {
                cur.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut cur).trim().to_string()),
            c => cur.push(c),
        }
    }
    out.push(cur.trim().to_string());
    out
}

/// What the route did after a jump, for the spoken line.
#[derive(Debug, Clone, PartialEq)]
pub enum Moved {
    /// Reached a waypoint; `next` is now on the clipboard (`copied` false if that failed).
    Arrived { at: Waypoint, next: Waypoint, left: usize, copied: bool },
    /// Reached the last waypoint.
    Finished { at: Waypoint },
}

// Spoken lines, in the callout copy guide's register (docs/V1-DESIGN.md §6.5): short, no
// contractions, no "I". `tests/copy.rs` checks them.

/// A voice route command with no route set.
pub const NO_ROUTE: &str = "No route to follow. Plot one in the app.";
pub const CLEARED: &str = "Route cleared.";
pub const NOTHING_TO_CLEAR: &str = "No route to clear.";
/// Past the last waypoint (arriving in it, or skipping past it).
pub const COMPLETE: &str = "Route complete.";
pub const FINISHED: &str = "Final waypoint reached. Route complete.";
/// "paste" outside the galaxy map.
pub const PASTE_REFUSED: &str = "Open the galaxy map and select its search box first.";
pub const PASTED: &str = "Pasted.";
/// The galaxy map closed between Ctrl+V and Enter: Enter is not sent.
pub const PASTE_STOPPED: &str = "Galaxy map closed. Search not sent.";

/// Every fixed line above, for the speech cache and the copy check.
pub const FIXED_LINES: &[&str] =
    &[NO_ROUTE, CLEARED, NOTHING_TO_CLEAR, COMPLETE, FINISHED, PASTE_REFUSED, PASTED, PASTE_STOPPED];

/// The callout on arrival and the reply to "skip" or "back": "Next waypoint. Deciat." The
/// clipboard holds it; "Not copied." says when it does not. The name is a sentence of its own,
/// which the word budget does not count (V1-DESIGN §6.5): a procedural name alone ("Col 285
/// Sector AB-C d14-123") can outrun a callout's budget.
pub fn next_line(next: &Waypoint, copied: bool) -> String {
    let mut s = format!("Next waypoint. {}.", next.system);
    if !copied {
        s += " Not copied.";
    }
    s
}

/// The answer to "next waypoint": "Next waypoint. Deciat. For Felicity Farseer. 3 waypoints
/// left." (`left` counts the next one.) The label says what is there (an engineer, "neutron
/// boost"); it follows the name in a sentence of its own, so the name stays a sentence the word
/// budget does not count (V1-DESIGN §6.5), however long.
pub fn answer_line(next: &Waypoint, left: usize, copied: bool) -> String {
    let mut s = format!("Next waypoint. {}.", next.system);
    if let Some(l) = &next.label {
        s += &format!(" For {l}.");
    }
    if !copied {
        s += " Not copied.";
    }
    if left > 1 {
        s += &format!(" {left} waypoints left.");
    }
    s
}

impl Moved {
    pub fn spoken(&self) -> String {
        match self {
            Moved::Arrived { next, copied, .. } => next_line(next, *copied),
            Moved::Finished { .. } => FINISHED.to_string(),
        }
    }
}

/// The voice commands for the route (`astromech-intent`'s MVP grammar ids).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Say the next waypoint and copy it again.
    Next,
    Skip,
    Back,
    Clear,
    /// Ctrl+V and Enter into the galaxy map's search box ([`crate::actions::Runner::paste`]).
    Paste,
}

impl Command {
    pub fn from_id(id: &str) -> Option<Self> {
        Some(match id {
            "route.next" => Self::Next,
            "route.skip" => Self::Skip,
            "route.back" => Self::Back,
            "route.clear" => Self::Clear,
            "galaxy_map.paste" => Self::Paste,
            _ => return None,
        })
    }

    /// What the command would do, for the voice command tester.
    pub fn describe(self) -> &'static str {
        match self {
            Self::Next => "Would say the next waypoint of the route and copy it to the clipboard again.",
            Self::Skip => "Would skip the next waypoint and copy the one after it.",
            Self::Back => "Would go back one waypoint and copy it.",
            Self::Clear => "Would stop following the route.",
            Self::Paste => "Would press Ctrl+V, then Enter: pastes the clipboard into the galaxy map's search box.",
        }
    }
}

/// Do a route command (not [`Command::Paste`]) on `desk` and return what to say.
pub fn command_reply(desk: &Desk, cmd: Command) -> String {
    if !desk.has_route() {
        return if cmd == Command::Clear { NOTHING_TO_CLEAR } else { NO_ROUTE }.to_string();
    }
    let v = match cmd {
        Command::Clear => {
            desk.clear();
            return CLEARED.to_string();
        }
        Command::Skip => desk.step(true),
        Command::Back => desk.step(false),
        Command::Next | Command::Paste => desk.copy_next(),
    };
    let copied = v.clipboard_error.is_none();
    match (&v.next, cmd) {
        (None, _) => COMPLETE.to_string(),
        (Some(n), Command::Next) => answer_line(n, v.remaining, copied),
        (Some(n), _) => next_line(n, copied),
    }
}

/// The route as the app window shows it.
#[derive(Debug, Clone, Serialize, ts_rs::TS)]
#[ts(rename = "RouteView")]
pub struct View {
    pub route: Option<Route>,
    /// Last system the journal put us in.
    pub here: Option<String>,
    pub next: Option<Waypoint>,
    pub remaining: usize,
    /// Jumps to the end ([`Route::jumps_left`]), when the route knows them.
    pub jumps_left: Option<u32>,
    /// Why the last clipboard write failed, if it did.
    pub clipboard_error: Option<String>,
    /// The route a galaxy-map plot or a route set from the window replaced, to resume.
    pub parked: Option<Route>,
    /// The timeline: the route's stops in order, each with the leg to it.
    pub stops: Vec<StopView>,
    /// Where the commander is on the leg being flown (the one to `next`), when its jumps are
    /// listed: how many of `stops[route.next].hops` are behind them, from the journal's
    /// current system (0: still in the system the list starts from, before its first jump). None
    /// when the leg's jumps aren't listed, or the commander isn't on them.
    pub position: Option<usize>,
}

/// Whether a star can be fuel-scooped: from its class where one is known (the KGBFOAM classes,
/// as `astromech_announce::fuel::scoopable` has it), else from the plotter's flag, else unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ts_rs::TS)]
#[serde(rename_all = "lowercase")]
pub enum Scoop {
    Yes,
    No,
    Unknown,
}

impl Scoop {
    fn of(star_class: Option<&str>, flag: Option<bool>) -> Self {
        match star_class.map(astromech_announce::fuel::scoopable).or(flag) {
            Some(true) => Scoop::Yes,
            Some(false) => Scoop::No,
            None => Scoop::Unknown,
        }
    }
}

/// Whether a star is a neutron star: from its class where known, else the plotter's flag.
fn neutron_of(star_class: Option<&str>, flag: Option<bool>) -> Option<bool> {
    star_class.map(|c| c == "N").or(flag)
}

/// Where a leg's listed jumps come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
pub enum HopsFrom {
    /// The route the commander plotted in the galaxy map (`NavRoute.json`), for the leg being
    /// flown: systems and star classes.
    NavRoute,
    /// The plotter's own list (exact routes): systems with Spansh's scoopable and neutron flags.
    Plotter,
}

/// A jump of a leg, as its source lists it.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(rename = "RouteJump")]
pub struct JumpView {
    pub system: String,
    pub star_class: Option<String>,
    pub scoopable: Scoop,
    pub neutron: Option<bool>,
}

impl JumpView {
    fn of(h: &Hop) -> Self {
        let class = h.star_class.as_deref();
        JumpView {
            system: h.system.clone(),
            star_class: h.star_class.clone(),
            scoopable: Scoop::of(class, h.scoopable),
            neutron: neutron_of(class, h.neutron),
        }
    }
}

/// A stop on the timeline and the leg to it. What each source gives, and nothing more:
///
/// - Neutron routes (this PC's plotter, Spansh's): the stop's system and whether it is a neutron
///   star; the leg's jump count and light years. No star class, and no jumps listed.
/// - Exact routes (Spansh): every jump is a stop, so each leg is the one jump to its stop, listed
///   with Spansh's scoopable and neutron flags (`hops_from: plotter`; the destination has no
///   scoopable flag, which Spansh sets false at both ends). No star class.
/// - A route plotted in the galaxy map: every jump is a stop, with its star class from
///   `NavRoute.json`; each leg is one jump, not listed.
/// - An imported CSV: what its columns give ([`parse_list`]): jumps, light years and the neutron
///   flag where the export has them. A plain list, or a lookup's stop: the system alone.
///
/// The leg being flown (to `route.next`) lists its jumps from `NavRoute.json` instead, when the
/// route the commander plotted in the galaxy map ends at this stop (`hops_from: nav_route`): its
/// systems with their star classes, which give the stop its class too. Otherwise a leg lists the
/// plotter's jumps if it has them, and is count-only if not. A scoopable that neither a class
/// nor a flag gives is `unknown`.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(rename = "RouteStop")]
pub struct StopView {
    pub system: String,
    pub label: Option<String>,
    pub star_class: Option<String>,
    pub scoopable: Scoop,
    pub neutron: Option<bool>,
    /// The leg's jumps (0 for a plotted route's first stop).
    pub jumps: Option<u32>,
    pub ly: Option<f64>,
    /// The leg's jumps in order, the last being this stop, where a source lists them.
    pub hops: Option<Vec<JumpView>>,
    pub hops_from: Option<HopsFrom>,
    /// An inhabited system's details from Spansh, once `route_details` has looked it up.
    pub details: Option<Details>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Saved {
    route: Option<Route>,
    #[serde(default)]
    here: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parked: Option<Route>,
    /// The route last plotted in the galaxy map (`NavRoute.json`, live), the system it was
    /// plotted from first: the systems and star classes of the leg being flown.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    nav: Vec<Hop>,
}

/// The timeline of `r` ([`View::stops`]) and the commander's place on the leg being flown.
fn timeline(
    r: &Route,
    nav: &[Hop],
    here: Option<&str>,
    details: &HashMap<String, Option<Details>>,
) -> (Vec<StopView>, Option<usize>) {
    let mut position = None;
    let stops = r
        .waypoints
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let flown = i == r.next;
            let nav = nav.last().filter(|d| flown && nav.len() > 1 && same_system(&d.system, &w.system)).map(|_| nav);
            let (start, hops, from) = match (nav, &w.hops) {
                (Some(nav), _) => (Some(nav[0].system.as_str()), Some(&nav[1..]), Some(HopsFrom::NavRoute)),
                (None, Some(h)) => {
                    (i.checked_sub(1).map(|p| r.waypoints[p].system.as_str()), Some(&h[..]), Some(HopsFrom::Plotter))
                }
                (None, None) => (None, None, None),
            };
            if let (true, Some(start), Some(hops), Some(here)) = (flown, start, hops, here) {
                let systems = std::iter::once(start).chain(hops.iter().map(|h| h.system.as_str()));
                position = systems.collect::<Vec<_>>().iter().rposition(|s| same_system(s, here));
            }
            let class = w.star_class.clone().or_else(|| nav.and_then(|n| n.last()).and_then(|h| h.star_class.clone()));
            StopView {
                system: w.system.clone(),
                label: w.label.clone(),
                scoopable: Scoop::of(class.as_deref(), w.scoopable),
                neutron: neutron_of(class.as_deref(), w.neutron),
                star_class: class,
                jumps: w.jumps,
                ly: w.ly,
                hops: hops.map(|h| h.iter().map(JumpView::of).collect()),
                hops_from: from,
                details: details.get(&key(&w.system)).cloned().flatten(),
            }
        })
        .collect();
    (stops, position)
}

/// A system's key in the details cache.
fn key(system: &str) -> String {
    system.trim().to_ascii_lowercase()
}

pub type Clip = Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>;

/// The one active route.
pub struct Desk {
    path: Option<PathBuf>,
    state: Mutex<(Saved, Option<String>)>,
    clip: Clip,
    /// Stops' details from Spansh by [`key`], for this run: None for a system looked up and found
    /// uninhabited or unknown, or whose lookup failed (it is not asked again).
    details: Mutex<HashMap<String, Option<Details>>>,
}

impl Desk {
    /// Load `path` (a missing or broken file is no route) and copy with `clip`.
    pub fn open(path: Option<PathBuf>, clip: Clip) -> Self {
        let saved = path
            .as_ref()
            .and_then(|p| std::fs::read(p).ok())
            .and_then(|b| serde_json::from_slice::<Saved>(&b).ok())
            .unwrap_or_default();
        Self { path, state: Mutex::new((saved, None)), clip, details: Mutex::default() }
    }

    /// The core's one: `<data>/route.json`, copying with `clip` (the installed app's is the
    /// Windows clipboard, [`crate::clipboard::set_text`]).
    pub fn in_dir(data: &std::path::Path, clip: Clip) -> Self {
        let _ = std::fs::create_dir_all(data);
        Self::open(Some(data.join("route.json")), clip)
    }

    fn save(&self, s: &Saved) {
        if let Some(p) = &self.path {
            if let Ok(t) = serde_json::to_string_pretty(s) {
                let tmp = p.with_extension("json.tmp");
                if std::fs::write(&tmp, t).is_ok() {
                    let _ = std::fs::rename(&tmp, p);
                }
            }
        }
    }

    fn copy(&self, st: &mut (Saved, Option<String>)) -> bool {
        let Some(w) = st.0.route.as_ref().and_then(|r| r.next_stop()) else { return false };
        match (self.clip)(&w.system) {
            Ok(()) => {
                st.1 = None;
                true
            }
            Err(e) => {
                crate::logging::log(&format!("route: couldn't copy {} to the clipboard: {e}", w.system));
                st.1 = Some(e);
                false
            }
        }
    }

    fn view_of(&self, st: &(Saved, Option<String>)) -> View {
        let r = st.0.route.as_ref();
        let (stops, position) =
            r.map(|r| timeline(r, &st.0.nav, st.0.here.as_deref(), &self.details.lock().recover())).unwrap_or_default();
        View {
            route: r.cloned(),
            here: st.0.here.clone(),
            next: r.and_then(|r| r.next_stop().cloned()),
            remaining: r.map(|r| r.remaining()).unwrap_or(0),
            jumps_left: r.and_then(Route::jumps_left),
            clipboard_error: st.1.clone(),
            parked: st.0.parked.clone(),
            stops,
            position,
        }
    }

    pub fn view(&self) -> View {
        self.view_of(&self.state.lock().recover())
    }

    /// The route's stops not looked up yet, now counted as looked up so each is asked for once
    /// ([`Desk::put_details`] gives the answer).
    pub fn take_details_wanted(&self) -> Vec<String> {
        let st = self.state.lock().recover();
        let mut d = self.details.lock().recover();
        let mut want = vec![];
        for w in st.0.route.iter().flat_map(|r| &r.waypoints) {
            if let std::collections::hash_map::Entry::Vacant(e) = d.entry(key(&w.system)) {
                e.insert(None);
                want.push(w.system.clone());
            }
        }
        want
    }

    /// Spansh's answer for stops [`Desk::take_details_wanted`] gave; the others stay without.
    pub fn put_details(&self, found: Vec<(String, Details)>) {
        let mut d = self.details.lock().recover();
        for (system, details) in found {
            d.insert(key(&system), Some(details));
        }
    }

    pub fn has_route(&self) -> bool {
        self.state.lock().recover().0.route.is_some()
    }

    /// Start following `route`. An unfinished route of our own that it replaces is parked (in
    /// place of an older parked one), for Resume; otherwise a parked route stays parked. If we
    /// are already in its first system it starts after it. The next waypoint is copied straight
    /// away.
    pub fn set(&self, route: Route) -> Result<View, String> {
        if route.waypoints.is_empty() {
            return Err("the route has no systems".into());
        }
        let mut st = self.state.lock().recover();
        Ok(self.follow(&mut st, route))
    }

    fn follow(&self, st: &mut (Saved, Option<String>), mut route: Route) -> View {
        route.next = 0;
        if let Some(h) = st.0.here.clone() {
            // Only the start counts: being in a later stop shouldn't skip the ones before it.
            if same_system(&route.waypoints[0].system, &h) {
                route.next = 1;
            }
        }
        crate::logging::log(&format!(
            "route: following {:?} ({} waypoints, from {})",
            route.name,
            route.waypoints.len(),
            if route.source.is_empty() { "?" } else { &route.source }
        ));
        if let Some(old) = st.0.route.take().filter(|r| !r.from_game() && !r.finished()) {
            crate::logging::log(&format!("route: parked {:?}", old.name));
            st.0.parked = Some(old);
        }
        st.0.route = Some(route);
        self.copy(st);
        self.save(&st.0);
        self.view_of(st)
    }

    /// Add stop: make `stop` the next waypoint of the route followed and copy it. Already in its
    /// system, it counts as reached (as a new route's first stop does) and the stop after it is
    /// next. With no route of our own to add to (none, a finished one, or the galaxy map's), it
    /// is followed as a one-stop route named after its system, from `source`.
    pub fn add(&self, stop: Waypoint, source: &str) -> Result<View, String> {
        let stop = Waypoint {
            system: stop.system.trim().to_string(),
            label: stop.label.filter(|l| !l.trim().is_empty()),
            ..stop
        };
        if stop.system.is_empty() {
            return Err("the stop has no system".into());
        }
        let mut st = self.state.lock().recover();
        let Some(r) = st.0.route.as_mut().filter(|r| !r.from_game() && !r.finished()) else {
            let route = Route::new(&stop.system.clone(), source, vec![stop]);
            return Ok(self.follow(&mut st, route));
        };
        crate::logging::log(&format!("route: added {} as the next stop of {:?}", stop.system, r.name));
        r.add_next(stop);
        let here = st.0.here.clone();
        let r = st.0.route.as_mut().expect("the route added to");
        if here.is_some_and(|h| r.next_stop().is_some_and(|w| same_system(&w.system, &h))) {
            r.next += 1;
        }
        self.copy(&mut st);
        self.save(&st.0);
        Ok(self.view_of(&st))
    }

    /// Stop following the route, and forget a parked one.
    pub fn clear(&self) -> View {
        let mut st = self.state.lock().recover();
        if st.0.route.take().is_some() | st.0.parked.take().is_some() {
            crate::logging::log("route: cleared");
        }
        st.1 = None;
        self.save(&st.0);
        self.view_of(&st)
    }

    /// Skip (`forward`) or go back one waypoint, and copy the new next one.
    pub fn step(&self, forward: bool) -> View {
        let mut st = self.state.lock().recover();
        let moved = st.0.route.as_mut().map(|r| if forward { r.skip() } else { r.back() }).unwrap_or(false);
        if moved {
            self.copy(&mut st);
            self.save(&st.0);
        }
        self.view_of(&st)
    }

    /// Copy the next waypoint again (something else took the clipboard).
    pub fn copy_next(&self) -> View {
        let mut st = self.state.lock().recover();
        self.copy(&mut st);
        self.view_of(&st)
    }

    /// Follow the parked route again and copy its next waypoint. Already in a waypoint ahead (flown
    /// there while it was parked), it resumes after that one.
    pub fn resume(&self) -> Result<View, String> {
        let mut st = self.state.lock().recover();
        let mut r = st.0.parked.take().ok_or("there is no route to resume")?;
        if let Some(h) = st.0.here.clone() {
            r.arrive(&h);
        }
        crate::logging::log(&format!("route: resumed {:?}", r.name));
        st.0.route = Some(r);
        self.copy(&mut st);
        self.save(&st.0);
        Ok(self.view_of(&st))
    }

    /// The commander plotted `hops` in the galaxy map (live, `NavRoute.json`; the current system
    /// first). Its systems and star classes are kept for the leg being flown ([`View::stops`]).
    /// A plot to a waypoint ahead on the route followed is a leg of it: the route doesn't
    /// change. A plot to a waypoint ahead on the parked route resumes that. Anything else is
    /// followed as a route from the game, and an unfinished route of our own that it replaces is
    /// parked.
    pub fn plotted(&self, hops: &[astromech_journal::RouteHop]) {
        let systems: Vec<&str> = hops.iter().map(|h| h.system.as_str()).collect();
        let Some(dest) = systems.last().copied().filter(|_| systems.len() > 1) else { return };
        let mut st = self.state.lock().recover();
        st.0.nav = hops
            .iter()
            .map(|h| Hop { system: h.system.clone(), star_class: h.star_class.clone(), scoopable: None, neutron: None })
            .collect();
        if st.0.route.as_ref().is_some_and(|r| !r.from_game() && r.ahead(dest)) {
            self.save(&st.0);
            return;
        }
        if st.0.parked.as_ref().is_some_and(|r| r.ahead(dest)) {
            let mut r = st.0.parked.take();
            if let (Some(r), Some(h)) = (r.as_mut(), st.0.here.clone()) {
                r.arrive(&h);
            }
            st.0.route = r;
            st.1 = None;
            crate::logging::log(&format!("route: galaxy map plotted to {dest}, resumed the parked route"));
            self.save(&st.0);
            return;
        }
        // The galaxy map's route has every jump as a waypoint, with its star class.
        let jumps = hops[1..]
            .iter()
            .map(|h| Waypoint { jumps: Some(1), star_class: h.star_class.clone(), ..Waypoint::new(&h.system) })
            .collect();
        let game = Route::new(&format!("Route to {dest}"), GAME, jumps);
        if game.waypoints.is_empty() {
            return;
        }
        if let Some(old) = st.0.route.take().filter(|r| !r.from_game() && !r.finished()) {
            crate::logging::log(&format!("route: parked {:?}", old.name));
            st.0.parked = Some(old);
        }
        crate::logging::log(&format!(
            "route: following the galaxy map's route to {dest} ({} jumps)",
            game.waypoints.len()
        ));
        st.0.route = Some(game);
        st.1 = None;
        self.save(&st.0);
    }

    /// The galaxy map's route was cleared: stop following it, if that is the route followed. A
    /// route of our own stays (its leg was cleared, or the commander arrived).
    pub fn cleared_in_game(&self) {
        let mut st = self.state.lock().recover();
        if !st.0.nav.is_empty() {
            st.0.nav.clear();
            self.save(&st.0);
        }
        if st.0.route.as_ref().is_some_and(Route::from_game) {
            st.0.route = None;
            st.1 = None;
            crate::logging::log("route: the galaxy map's route was cleared");
            self.save(&st.0);
        }
    }

    /// A replayed journal put us in `system`: remember where we are, but don't move the route
    /// or write the clipboard (only live arrivals do, [`Desk::on_system`]).
    pub fn set_here(&self, system: &str) {
        let mut st = self.state.lock().recover();
        if st.0.here.as_deref() != Some(system) {
            st.0.here = Some(system.to_string());
            self.save(&st.0);
        }
    }

    /// A live arrival in `system`: moves the route on if it is a waypoint (copying the next). A
    /// route from the game moves on without a copy or a line.
    pub fn on_system(&self, system: &str) -> Option<Moved> {
        let mut st = self.state.lock().recover();
        let changed = st.0.here.as_deref() != Some(system);
        st.0.here = Some(system.to_string());
        let arrived =
            st.0.route.as_mut().and_then(|r| r.arrive(system).map(|i| (r.waypoints[i].clone(), r.remaining())));
        let Some((at, left)) = arrived else {
            if changed {
                self.save(&st.0);
            }
            return None;
        };
        let next = st.0.route.as_ref().and_then(|r| r.next_stop().cloned());
        if st.0.route.as_ref().is_some_and(Route::from_game) {
            self.save(&st.0);
            return None;
        }
        let copied = next.is_some() && self.copy(&mut st);
        self.save(&st.0);
        crate::logging::log(&format!(
            "route: reached {}{}",
            at.system,
            next.as_ref().map(|n| format!("; next {}", n.system)).unwrap_or_else(|| "; route finished".into())
        ));
        Some(match next {
            Some(next) => Moved::Arrived { at, next, left, copied },
            None => Moved::Finished { at },
        })
    }
}

/// The callout engine's `route_follow` rule drives the desk: it is asked only for live
/// arrivals, so a replayed journal never moves the route or writes the clipboard.
impl astromech_announce::route_follow::Follower for Desk {
    fn arrived(&self, system: &str) -> Option<String> {
        self.on_system(system).map(|m| m.spoken())
    }
    fn here(&self, system: &str) {
        self.set_here(system);
    }
    fn plotted(&self, hops: &[astromech_journal::RouteHop]) {
        Desk::plotted(self, hops);
    }
    fn cleared(&self) {
        self.cleared_in_game();
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;
    use std::sync::Arc;

    fn desk() -> (Desk, Arc<Mutex<Vec<String>>>) {
        let got = Arc::new(Mutex::new(vec![]));
        let g = got.clone();
        let d = Desk::open(
            None,
            Box::new(move |s| {
                g.lock().recover().push(s.to_string());
                Ok(())
            }),
        );
        (d, got)
    }

    fn route(systems: &[&str]) -> Route {
        Route::new("test", "import", systems.iter().map(|s| Waypoint::new(s)).collect())
    }

    #[test]
    fn follows_and_copies() {
        let (d, got) = desk();
        d.on_system("Sol");
        let v = d.set(route(&["Sol", "Alpha Centauri", "Barnard's Star"])).unwrap();
        // Already in the first stop: it starts at the second.
        assert_eq!(v.next.unwrap().system, "Alpha Centauri");
        assert_eq!(got.lock().recover().as_slice(), ["Alpha Centauri"]);
        assert_eq!(d.on_system("Wolf 359"), None);
        let m = d.on_system("alpha centauri").unwrap();
        assert!(matches!(&m, Moved::Arrived { next, left: 1, copied: true, .. } if next.system == "Barnard's Star"));
        assert_eq!(m.spoken(), "Next waypoint. Barnard's Star.");
        let m = d.on_system("Barnard's Star").unwrap();
        assert!(matches!(m, Moved::Finished { .. }));
        assert_eq!(m.spoken(), FINISHED);
        assert_eq!(d.view().remaining, 0);
        assert_eq!(got.lock().recover().len(), 2);
    }

    #[test]
    fn a_later_stop_skips_ahead() {
        let (d, _) = desk();
        d.set(route(&["A", "B", "C", "D"])).unwrap();
        assert!(matches!(d.on_system("C"), Some(Moved::Arrived { ref next, left: 1, .. }) if next.system == "D"));
        // Being in a later stop when the route is set does not skip.
        let (d, _) = desk();
        d.on_system("C");
        assert_eq!(d.set(route(&["A", "B", "C"])).unwrap().next.unwrap().system, "A");
    }

    #[test]
    fn skip_back_clear() {
        let (d, got) = desk();
        d.set(route(&["A", "B"])).unwrap();
        assert_eq!(d.step(true).next.unwrap().system, "B");
        assert_eq!(d.step(true).next, None);
        assert_eq!(d.step(true).next, None);
        assert_eq!(d.step(false).next.unwrap().system, "B");
        assert_eq!(got.lock().recover().as_slice(), ["A", "B", "B"]);
        assert!(d.clear().route.is_none());
        assert!(d.set(route(&[" ", ""])).is_err());
    }

    #[test]
    fn clipboard_failure_is_reported() {
        let d = Desk::open(None, Box::new(|_| Err("busy".into())));
        let v = d.set(route(&["A", "B"])).unwrap();
        assert_eq!(v.clipboard_error.as_deref(), Some("busy"));
        let m = d.on_system("A").unwrap();
        assert_eq!(m.spoken(), "Next waypoint. B. Not copied.");
    }

    fn plot_in_game(d: &Desk, systems: &[&str]) {
        let hops: Vec<astromech_journal::RouteHop> = systems
            .iter()
            .map(|s| astromech_journal::RouteHop { system: s.to_string(), ..Default::default() })
            .collect();
        d.plotted(&hops);
    }

    /// A Spansh neutron route's stops are neutron stars; the commander plots each leg in the
    /// galaxy map. A plot to a stop ahead (the next, or one further on) keeps the route.
    #[test]
    fn a_galaxy_map_plot_to_a_waypoint_ahead_is_a_leg() {
        let (d, got) = desk();
        d.on_system("Sol");
        d.set(route(&["Sol", "N1", "N2", "Colonia"])).unwrap();
        plot_in_game(&d, &["Sol", "A", "B", "N1"]);
        plot_in_game(&d, &["Sol", "A", "n2"]);
        let v = d.view();
        assert_eq!(v.route.as_ref().unwrap().name, "test");
        assert_eq!(v.next.unwrap().system, "N1");
        assert!(v.parked.is_none());
        assert_eq!(got.lock().recover().as_slice(), ["N1"], "a leg doesn't touch the clipboard");
    }

    /// Anything else is a route of its own: followed silently, without the clipboard, and the
    /// route it replaced is parked until Resume or a plot back onto it.
    #[test]
    fn a_detour_in_the_galaxy_map_parks_the_route() {
        let (d, got) = desk();
        d.on_system("Sol");
        d.set(route(&["Sol", "N1", "N2", "Colonia"])).unwrap();
        plot_in_game(&d, &["Sol", "Wolf 359", "Station Star"]);
        let v = d.view();
        let r = v.route.unwrap();
        assert!(r.from_game());
        assert_eq!(r.name, "Route to Station Star");
        assert_eq!(r.waypoints.iter().map(|w| w.system.as_str()).collect::<Vec<_>>(), ["Wolf 359", "Station Star"]);
        assert_eq!(v.parked.as_ref().unwrap().name, "test");
        assert_eq!(d.on_system("Wolf 359"), None, "silent");
        assert_eq!(d.view().next.unwrap().system, "Station Star", "but it moves on");
        assert_eq!(got.lock().recover().as_slice(), ["N1"], "and never copies");

        // A second detour replaces the game's route; the parked route stays ours.
        plot_in_game(&d, &["Wolf 359", "Ross 128"]);
        assert_eq!(d.view().route.unwrap().name, "Route to Ross 128");
        assert_eq!(d.view().parked.unwrap().name, "test");

        // Plotting to a waypoint of the parked route resumes it, without a copy (it is plotted).
        plot_in_game(&d, &["Wolf 359", "X", "N1"]);
        let v = d.view();
        assert_eq!(v.route.unwrap().name, "test");
        assert!(v.parked.is_none());
        assert_eq!(got.lock().recover().len(), 1);

        // Flown to a waypoint on a detour, then plotting onward: it resumes after that waypoint.
        plot_in_game(&d, &["Wolf 359", "Ross 128"]);
        d.on_system("N1");
        plot_in_game(&d, &["N1", "Y", "Colonia"]);
        assert_eq!(d.view().next.unwrap().system, "N2");
    }

    #[test]
    fn resume_brings_the_parked_route_back_and_copies() {
        let (d, got) = desk();
        d.set(route(&["A", "B"])).unwrap();
        plot_in_game(&d, &["Sol", "Elsewhere"]);
        let v = d.resume().unwrap();
        assert_eq!(v.route.unwrap().name, "test");
        assert!(v.parked.is_none());
        assert_eq!(got.lock().recover().as_slice(), ["A", "A"]);
        assert!(d.resume().is_err());
        // Flown to a waypoint while it was parked: Resume carries on after it.
        d.set(route(&["A", "B", "C"])).unwrap();
        plot_in_game(&d, &["Sol", "Elsewhere"]);
        d.on_system("B");
        assert_eq!(d.resume().unwrap().next.unwrap().system, "C");
        // A route set from the window replaces the galaxy map's route, which is not parked, so
        // the parked route stays; clearing drops it.
        plot_in_game(&d, &["Sol", "Elsewhere"]);
        assert_eq!(d.set(route(&["C"])).unwrap().parked.unwrap().name, "test");
        assert!(d.clear().parked.is_none());
    }

    fn named(name: &str, systems: &[&str]) -> Route {
        Route::new(name, "import", systems.iter().map(|s| Waypoint::new(s)).collect())
    }

    fn stop(system: &str, label: &str) -> Waypoint {
        Waypoint { label: Some(label.into()), ..Waypoint::new(system) }
    }

    fn systems(v: &View) -> Vec<String> {
        v.route.as_ref().unwrap().waypoints.iter().map(|w| w.system.clone()).collect()
    }

    /// Set destination (or any route set from the window) parks the unfinished route it
    /// replaces, so Resume brings it back: never a silent replace (decision astromech-bi94).
    #[test]
    fn setting_a_route_parks_the_unfinished_one_it_replaces() {
        let (d, got) = desk();
        d.set(named("trip", &["A", "B"])).unwrap();
        let v = d.set(named("Deciat", &["Deciat"])).unwrap();
        assert_eq!(v.next.unwrap().system, "Deciat");
        assert_eq!(v.parked.as_ref().unwrap().name, "trip");
        // A newer replacement parks its route in place of the older parked one.
        let v = d.set(named("Kuk", &["Kuk"])).unwrap();
        assert_eq!(v.parked.as_ref().unwrap().name, "Deciat");
        let v = d.resume().unwrap();
        assert_eq!(v.route.unwrap().name, "Deciat");
        assert!(v.parked.is_none(), "one route stays parked");
        assert_eq!(got.lock().recover().as_slice(), ["A", "Deciat", "Kuk", "Deciat"]);
    }

    #[test]
    fn a_finished_route_is_not_parked_by_a_new_one() {
        let (d, _) = desk();
        d.set(named("old", &["A", "B"])).unwrap();
        d.set(named("trip", &["A"])).unwrap();
        d.on_system("A");
        let v = d.set(named("next", &["C"])).unwrap();
        assert_eq!(v.parked.unwrap().name, "old", "the finished trip does not displace the parked route");
    }

    /// Add stop makes the place the next waypoint, copies it, and leaves the rest of the route.
    #[test]
    fn add_stop_inserts_the_next_waypoint() {
        let (d, got) = desk();
        d.on_system("A");
        d.set(named("trip", &["A", "B", "C"])).unwrap();
        let v = d.add(stop("X", "Interstellar Factors"), "stations.service").unwrap();
        assert_eq!(systems(&v), ["A", "X", "B", "C"]);
        assert_eq!(v.next, Some(stop("X", "Interstellar Factors")));
        assert_eq!(v.remaining, 3);
        assert_eq!(v.route.as_ref().unwrap().name, "trip");
        assert!(v.parked.is_none(), "adding replaces nothing");
        assert_eq!(got.lock().recover().as_slice(), ["B", "X"]);
        // Arriving there moves on to the route's own next stop.
        assert!(matches!(d.on_system("X"), Some(Moved::Arrived { ref next, .. }) if next.system == "B"));
    }

    /// A stop in the same system as the next waypoint, or the one just passed (skipped, here),
    /// merges with it as `Route::new` merges neighbours, and that waypoint is next.
    #[test]
    fn add_stop_merges_with_a_neighbour_in_its_system() {
        let (d, got) = desk();
        d.set(Route::new(
            "trip",
            "engineer trip",
            vec![stop("A", "Farseer"), stop("B", "Ram Tah"), stop("C", "Palin")],
        ))
        .unwrap();
        d.step(true);
        let v = d.add(stop("b", "Material Trader"), "").unwrap();
        assert_eq!(systems(&v), ["A", "B", "C"]);
        assert_eq!(v.next, Some(stop("B", "Material Trader, Ram Tah")));
        let v = d.add(stop("A", "Interstellar Factors"), "").unwrap();
        assert_eq!(systems(&v), ["A", "B", "C"]);
        assert_eq!(v.next, Some(stop("A", "Farseer, Interstellar Factors")), "back to the stop just passed");
        assert_eq!(v.remaining, 3);
        assert_eq!(got.lock().recover().as_slice(), ["A", "B", "B", "A"]);
    }

    /// A stop in the system the commander is in counts as reached, as a new route's first stop
    /// does: the route stays on its next stop instead of waiting for a jump into where they are.
    #[test]
    fn add_stop_where_you_are_counts_as_reached() {
        let (d, got) = desk();
        d.on_system("A");
        d.set(named("trip", &["A", "B"])).unwrap();
        let v = d.add(stop("a", "Interstellar Factors"), "").unwrap();
        assert_eq!(systems(&v), ["A", "B"]);
        assert_eq!(v.route.as_ref().unwrap().waypoints[0].label.as_deref(), Some("Interstellar Factors"));
        assert_eq!(v.next.unwrap().system, "B");
        d.on_system("Wolf 359");
        let v = d.add(Waypoint::new("Wolf 359"), "").unwrap();
        assert_eq!(systems(&v), ["A", "Wolf 359", "B"]);
        assert_eq!(v.next.unwrap().system, "B");
        assert_eq!(got.lock().recover().as_slice(), ["B", "B", "B"]);
    }

    /// With no route of our own to add to, Add stop follows the place on its own; a finished or
    /// galaxy-map route it replaces is not parked, and a parked route stays.
    #[test]
    fn add_stop_with_no_route_starts_a_one_stop_route() {
        let (d, got) = desk();
        let v = d.add(stop("Deciat", "Garay Terminal"), "search").unwrap();
        let r = v.route.unwrap();
        assert_eq!((r.name.as_str(), r.source.as_str()), ("Deciat", "search"));
        assert_eq!(v.next, Some(stop("Deciat", "Garay Terminal")));
        assert!(v.parked.is_none());

        // Finished: replaced, not added to.
        d.on_system("Deciat");
        let v = d.add(Waypoint::new("Kuk"), "search").unwrap();
        assert_eq!(systems(&v), ["Kuk"]);
        assert!(v.parked.is_none());

        // The galaxy map's route: replaced, not added to, and the parked route of ours stays.
        plot_in_game(&d, &["Deciat", "Wolf 359", "Sol"]);
        assert_eq!(d.view().parked.unwrap().name, "Kuk");
        let v = d.add(Waypoint::new("Shinrarta Dezhra"), "search").unwrap();
        assert_eq!(systems(&v), ["Shinrarta Dezhra"]);
        assert_eq!(v.parked.unwrap().name, "Kuk");
        assert_eq!(got.lock().recover().as_slice(), ["Deciat", "Kuk", "Shinrarta Dezhra"]);

        assert!(d.add(Waypoint::new("  "), "search").is_err());
    }

    #[test]
    fn a_finished_route_is_not_parked_and_a_galaxy_map_clear_drops_only_its_own() {
        let (d, _) = desk();
        d.set(route(&["A"])).unwrap();
        d.on_system("A");
        plot_in_game(&d, &["A", "B"]);
        assert!(d.view().parked.is_none());
        d.cleared_in_game();
        assert!(d.view().route.is_none());

        d.set(route(&["A", "C"])).unwrap();
        d.cleared_in_game();
        assert_eq!(d.view().route.unwrap().name, "test", "a cleared leg keeps our route");
        // A one-system route (only where we are) is no route.
        plot_in_game(&d, &["A"]);
        assert_eq!(d.view().route.unwrap().name, "test");
    }

    #[test]
    fn the_parked_route_survives_a_restart() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("route.json");
        let d = Desk::open(Some(p.clone()), Box::new(|_| Ok(())));
        d.set(route(&["A", "B"])).unwrap();
        plot_in_game(&d, &["Sol", "Elsewhere"]);
        drop(d);
        let v = Desk::open(Some(p), Box::new(|_| Ok(()))).view();
        assert!(v.route.unwrap().from_game());
        assert_eq!(v.parked.unwrap().name, "test");
    }

    #[test]
    fn survives_a_restart() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("route.json");
        let d = Desk::open(Some(p.clone()), Box::new(|_| Ok(())));
        d.set(route(&["A", "B", "C"])).unwrap();
        d.on_system("A");
        drop(d);
        let d = Desk::open(Some(p), Box::new(|_| Ok(())));
        let v = d.view();
        assert_eq!(v.next.unwrap().system, "B");
        assert_eq!(v.here.as_deref(), Some("A"));
    }

    /// `route.json` written before waypoints recorded their jumps still loads: the route is
    /// followed as before and says how many stops are left, not jumps.
    #[test]
    fn a_route_saved_without_jumps_still_loads() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("route.json");
        std::fs::write(
            &p,
            r#"{"route":{"name":"Sol to Maia","source":"exact plotter","waypoints":[{"system":"Sol"},{"system":"Tascheter Sector MN-T b3-6","label":"fuel"},{"system":"Maia"}],"next":1},"here":"Sol"}"#,
        )
        .unwrap();
        let v = Desk::open(Some(p.clone()), Box::new(|_| Ok(()))).view();
        assert_eq!(v.next.as_ref().unwrap().system, "Tascheter Sector MN-T b3-6");
        assert_eq!(v.next.unwrap().jumps, None);
        assert_eq!((v.remaining, v.jumps_left), (2, None));
        // Saving it again doesn't write jumps it never had.
        let d = Desk::open(Some(p.clone()), Box::new(|_| Ok(())));
        d.step(true);
        assert!(!std::fs::read_to_string(&p).unwrap().contains("jumps"));
    }

    fn legs(stops: &[(&str, u32)]) -> Route {
        let w = stops.iter().map(|&(s, j)| Waypoint { jumps: Some(j), ..Waypoint::new(s) }).collect();
        Route::new("Sol to Colonia", "neutron plotter", w)
    }

    /// A plotted route knows each leg's jumps, so the strip can say "N jumps left"; a route with
    /// a stop that doesn't (an import, a lookup's Add stop) has only its stops to count.
    #[test]
    fn counts_the_jumps_left_where_every_stop_knows_them() {
        let (d, _) = desk();
        d.on_system("Sol");
        // A neutron route: legs of 10 jumps and 1, then 6 to the end.
        let v = d.set(legs(&[("Sol", 0), ("N1", 10), ("N2", 1), ("Colonia", 6)])).unwrap();
        assert_eq!((v.remaining, v.jumps_left), (3, Some(17)));
        d.on_system("N1");
        assert_eq!(d.view().jumps_left, Some(7));
        // A repeat of a system in a row is one stop and no extra jump.
        let r = legs(&[("Sol", 0), ("A", 3), ("a", 0), ("B", 2)]);
        assert_eq!((r.waypoints.len(), r.jumps_left()), (3, Some(5)));

        // An import knows no jumps.
        assert_eq!(d.set(route(&["Sol", "A", "B"])).unwrap().jumps_left, None);

        // Add stop puts a stop of unknown jumps ahead, and the leg after it starts there now.
        d.set(legs(&[("N1", 0), ("N2", 1), ("Colonia", 6)])).unwrap();
        assert_eq!(d.view().jumps_left, Some(7));
        let v = d.add(stop("Deciat", "Felicity Farseer"), "services").unwrap();
        assert_eq!((v.remaining, v.jumps_left), (3, None));
        d.on_system("Deciat");
        let v = d.view();
        assert_eq!(v.next.as_ref().unwrap().system, "N2");
        assert_eq!(v.jumps_left, None, "N2's jumps were from N1, not from Deciat");

        // A route plotted in the galaxy map has every jump as a waypoint.
        plot_in_game(&d, &["Deciat", "W1", "W2", "Ross 128"]);
        assert_eq!(d.view().jumps_left, Some(3));
        d.on_system("W1");
        assert_eq!(d.view().jumps_left, Some(2));
    }

    /// `[callouts.route_follow] enabled = false`: jumping doesn't move the route or copy, but the
    /// desk still knows where the commander is, so the Route page's "you are here" is right and a
    /// route set now starts after the system the commander is in.
    #[test]
    fn with_following_off_jumps_still_update_where_you_are() {
        use astromech_announce::route_follow::{RouteFollow, SharedFollower};
        use astromech_announce::{Engine, Settings};
        let (d, got) = desk();
        let d = Arc::new(d);
        d.set(route(&["A", "B"])).unwrap();
        let shared = SharedFollower::default();
        shared.set(d.clone());
        let off: Settings = toml::from_str("[route_follow]\nenabled = false\n").unwrap();
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &off);
        e.set_live(true);
        let g = astromech_gamestate::GameState::default();
        let mut jump = |system: &str| {
            let l = astromech_journal::parse_line(&format!(
                r#"{{"timestamp":"t","event":"FSDJump","StarSystem":"{system}","JumpDist":9.0}}"#
            ))
            .unwrap();
            e.journal(&l, &g, std::time::Instant::now())
        };
        assert!(jump("A").is_empty(), "nothing said");
        let v = d.view();
        assert_eq!(v.here.as_deref(), Some("A"));
        assert_eq!(v.next.unwrap().system, "A", "the route did not move");
        assert_eq!(got.lock().recover().len(), 1, "only the copy when the route was set");
        let v = d.set(route(&["A", "C"])).unwrap();
        assert_eq!(v.next.unwrap().system, "C", "a route set now starts after where you are");
    }

    #[test]
    fn merges_stops_in_one_system() {
        let r = Route::new(
            "trip",
            "engineer trip",
            vec![stop("Deciat", "Felicity Farseer"), stop("deciat", "Someone Else"), stop("Kuk", "Ram Tah")],
        );
        assert_eq!(r.waypoints.len(), 2);
        assert_eq!(r.waypoints[0].label.as_deref(), Some("Felicity Farseer, Someone Else"));
        assert_eq!(answer_line(&r.waypoints[1], 2, true), "Next waypoint. Kuk. For Ram Tah. 2 waypoints left.");
    }

    #[test]
    fn parses_spansh_csv_and_plain_lists() {
        let neutron = "\"System Name\",\"Distance To Arrival\",\"Distance Remaining\",\"Neutron Star\",\"Jumps\"\n\
                       \"Sol\",\"0\",\"22000\",\"No\",\"0\"\n\
                       \"Col 285 Sector AB-C d12-34\",\"400\",\"21600\",\"Yes\",\"6\"\n";
        let w = parse_list(neutron);
        assert_eq!(w.iter().map(|w| w.system.as_str()).collect::<Vec<_>>(), ["Sol", "Col 285 Sector AB-C d12-34"]);
        // Road to riches: one row per body, merged per system.
        let r2r = "System Name,Body Name,Body Subtype\nHIP 1,HIP 1 A 1,Water world\nHIP 1,HIP 1 A 2,Earth-like world\nHIP 2,HIP 2 3,Ammonia world\n";
        assert_eq!(parse_list(r2r).len(), 2);
        let plain = "Sol\n\n  Shinrarta Dezhra \n\"Colonia\"\n";
        assert_eq!(
            parse_list(plain).iter().map(|w| w.system.as_str()).collect::<Vec<_>>(),
            ["Sol", "Shinrarta Dezhra", "Colonia"]
        );
        assert_eq!(split_csv(r#""a ""b""",c"#), ["a \"b\"", "c"]);
        assert!(parse_list("").is_empty());
    }

    /// NavRoute.json for the leg Sol to PSR J1752-2806, as the game writes it: the systems Spansh's
    /// exact plotter gave for that leg (12 Nov 2026), with each one's main star from Spansh's
    /// system pages as the journal names the classes.
    const NAV: &[(&str, &str)] = &[
        ("Sol", "G"),
        ("Core Sys Sector RT-R b4-5", "M"),
        ("Alrai Sector LH-V b2-4", "M"),
        ("Erh Longo", "M"),
        ("Scorpii Sector GR-W c1-35", "G"),
        ("Col 285 Sector VB-H b25-6", "M"),
        ("Col 285 Sector DO-D b27-9", "M"),
        ("Col 285 Sector LA-A b29-3", "TTS"),
        ("Praea Euq RD-T b3-5", "TTS"),
        ("Col 359 Sector ZL-K b10-5", "M"),
        ("Col 359 Sector GP-P c6-35", "TTS"),
        ("PSR J1752-2806", "N"),
    ];

    fn nav(hops: &[(&str, &str)]) -> Vec<astromech_journal::RouteHop> {
        hops.iter()
            .map(|&(s, c)| astromech_journal::RouteHop {
                system: s.into(),
                star_class: (!c.is_empty()).then(|| c.into()),
                star_pos: None,
            })
            .collect()
    }

    /// The neutron route Spansh plotted (astromech-data's spansh-route-result.json): Sol, 10
    /// jumps to PSR J1752-2806, 1 to Colonia.
    fn spansh_neutron() -> Route {
        let w = |s: &str, jumps: u32, ly: f64, neutron: Option<bool>| Waypoint {
            jumps: Some(jumps),
            ly: Some(ly),
            neutron,
            ..Waypoint::new(s)
        };
        Route::new(
            "Sol to Colonia",
            "neutron plotter",
            vec![
                w("Sol", 0, 0.0, None),
                w("PSR J1752-2806", 10, 407.486400990496, Some(true)),
                w("Colonia", 1, 113.169815881102, Some(false)),
            ],
        )
    }

    /// The leg being flown lists its jumps from NavRoute.json when the commander's galaxy-map
    /// route ends at the next stop: systems, star classes, scoopable from the class, and the
    /// commander's place on it from the journal. The leg after it keeps only its count.
    #[test]
    fn the_flown_leg_lists_its_jumps_from_the_galaxy_map() {
        let (d, _) = desk();
        d.on_system("Sol");
        let v = d.set(spansh_neutron()).unwrap();
        assert_eq!((v.stops[1].hops.as_ref(), v.position), (None, None), "no galaxy-map route yet: count only");

        d.plotted(&nav(NAV));
        let v = d.view();
        assert_eq!(v.route.as_ref().unwrap().name, "Sol to Colonia", "a leg of the route");
        let leg = &v.stops[1];
        assert_eq!(leg.hops_from, Some(HopsFrom::NavRoute));
        let hops = leg.hops.as_ref().unwrap();
        assert_eq!(hops.len(), 11, "the game's 11 jumps, though the plotter counted 10 at its range");
        assert_eq!(leg.jumps, Some(10));
        assert_eq!(
            hops[0],
            JumpView {
                system: "Core Sys Sector RT-R b4-5".into(),
                star_class: Some("M".into()),
                scoopable: Scoop::Yes,
                neutron: Some(false)
            }
        );
        assert_eq!((hops[6].star_class.as_deref(), hops[6].scoopable), (Some("TTS"), Scoop::No));
        assert_eq!(
            (leg.star_class.as_deref(), leg.scoopable, leg.neutron),
            (Some("N"), Scoop::No, Some(true)),
            "the stop's class too"
        );
        assert_eq!(v.position, Some(0), "in Sol, before the first jump");
        assert_eq!((v.stops[2].hops.as_ref(), v.stops[2].jumps, v.stops[2].star_class.as_ref()), (None, Some(1), None));

        // Jumping along the leg: the journal's system is the place on it.
        assert_eq!(d.on_system("Core Sys Sector RT-R b4-5"), None);
        d.on_system("Alrai Sector LH-V b2-4");
        assert_eq!(d.view().position, Some(2));
        // Off the listed jumps: no place.
        d.on_system("Wolf 359");
        assert_eq!(d.view().position, None);

        // Arriving: the next leg has no galaxy-map route ending at its stop, so it is count-only.
        d.on_system("PSR J1752-2806");
        let v = d.view();
        assert_eq!((v.stops[2].hops.as_ref(), v.position), (None, None));
        // The stop just reached keeps nothing from the old galaxy-map route either.
        assert_eq!(v.stops[1].hops, None);
    }

    /// A galaxy-map route that ends elsewhere lists nothing on the leg, and a jump with no star
    /// class (RouteHop.star_class is optional) is unknown, not guessed. A clear forgets it.
    #[test]
    fn a_galaxy_map_route_lists_a_leg_only_when_it_ends_at_the_next_stop() {
        let (d, _) = desk();
        d.on_system("Sol");
        d.set(spansh_neutron()).unwrap();
        // A plot to Colonia, a stop further on, is a leg of the route but not the leg flown.
        d.plotted(&nav(&[("Sol", "G"), ("Somewhere", "K"), ("Colonia", "")]));
        assert_eq!(d.view().stops.iter().filter(|s| s.hops.is_some()).count(), 0);

        d.plotted(&nav(&[("Sol", "G"), ("Unscanned", ""), ("PSR J1752-2806", "N")]));
        let v = d.view();
        let hops = v.stops[1].hops.as_ref().unwrap();
        assert_eq!((hops[0].star_class.as_ref(), hops[0].scoopable, hops[0].neutron), (None, Scoop::Unknown, None));
        d.cleared_in_game();
        assert_eq!(d.view().stops[1].hops, None);
    }

    /// The galaxy map's own route has every jump as a stop, each with its star class.
    #[test]
    fn a_galaxy_map_route_gives_each_stop_its_star_class() {
        let (d, _) = desk();
        d.on_system("Sol");
        d.plotted(&nav(&NAV[..4]));
        let v = d.view();
        let classes: Vec<_> =
            v.stops.iter().map(|s| (s.system.as_str(), s.star_class.as_deref(), s.scoopable)).collect();
        assert_eq!(
            classes,
            [
                ("Core Sys Sector RT-R b4-5", Some("M"), Scoop::Yes),
                ("Alrai Sector LH-V b2-4", Some("M"), Scoop::Yes),
                ("Erh Longo", Some("M"), Scoop::Yes)
            ]
        );
    }

    /// Spansh's CSV exports (its site's column lists, here over the recorded neutron and exact
    /// jobs in astromech-data's fixtures) give what their columns give.
    #[test]
    fn a_csv_import_gives_what_its_columns_give() {
        let w = parse_list(include_str!("../tests/fixtures/spansh-neutron-sol-colonia.csv"));
        let got: Vec<_> = w.iter().map(|w| (w.system.as_str(), w.jumps, w.ly, w.neutron)).collect();
        assert_eq!(
            got,
            [
                ("Sol", Some(0), Some(0.0), Some(false)),
                ("PSR J1752-2806", Some(10), Some(407.486400990496), Some(true)),
                ("Colonia", Some(1), Some(113.169815881102), Some(false))
            ]
        );
        // The exact plotter's export: every jump, its distance and neutron flag, but no jump count
        // column, no star class and no scoopable flag.
        let w = parse_list(include_str!("../tests/fixtures/spansh-exact-sol-maia.csv"));
        assert_eq!(w.len(), 10);
        assert_eq!((w[1].jumps, w[1].ly, w[1].neutron), (None, Some(38.2943229675293), Some(false)));
        assert_eq!(w[8].neutron, Some(true), "98 k Tauri");
        assert!(w.iter().all(|w| w.star_class.is_none() && w.scoopable.is_none() && w.hops.is_none()));
        let (d, _) = desk();
        let v = d.set(Route::new("Import", "import", w)).unwrap();
        assert!(v.stops.iter().all(|s| s.hops.is_none() && s.scoopable == Scoop::Unknown));
        // Road to riches: "Distance To Arrival" is a body's light seconds, not a leg.
        let r2r = "\"System Name\",\"Body Name\",\"Body Subtype\",\"Is Terraformable\",\"Distance To Arrival\",\"Estimated Scan Value\",\"Estimated Mapping Value\",\"Jumps\"\n\
                   \"HIP 1\",\"HIP 1 A 1\",\"Water world\",\"No\",\"1200\",\"1\",\"2\",\"3\"\n\
                   \"HIP 1\",\"HIP 1 A 2\",\"Earth-like world\",\"No\",\"1500\",\"1\",\"2\",\"3\"\n";
        let w = parse_list(r2r);
        assert_eq!((w.len(), w[0].jumps, w[0].ly), (1, Some(3), None), "one stop, its jumps counted once");
    }

    /// Each stop is looked up once; a stop Spansh doesn't answer for, or a failed lookup, has no
    /// details and isn't asked for again.
    #[test]
    fn stop_details_are_looked_up_once() {
        let (d, _) = desk();
        d.set(spansh_neutron()).unwrap();
        assert_eq!(d.take_details_wanted(), ["Sol", "PSR J1752-2806", "Colonia"]);
        assert!(d.take_details_wanted().is_empty(), "asked once");
        let sol = Details {
            population: 18320926115,
            allegiance: Some("Federation".into()),
            government: Some("Democracy".into()),
            security: Some("High".into()),
            faction: Some("Mother Gaia".into()),
            faction_state: Some("Expansion".into()),
        };
        d.put_details(vec![("Sol".into(), sol.clone())]);
        let v = d.view();
        assert_eq!(v.stops[0].details.as_ref(), Some(&sol));
        assert_eq!((v.stops[1].details.as_ref(), v.stops[2].details.as_ref()), (None, None));
        // A new route asks only for its new stops.
        d.set(route(&["Sol", "Deciat"])).unwrap();
        assert_eq!(d.take_details_wanted(), ["Deciat"]);
    }

    /// A plotted route keeps its plot settings across a restart; one saved before they were
    /// recorded loads without them.
    #[test]
    fn plot_settings_are_kept_and_optional() {
        use astromech_galaxy::plot::{Engine, Plotter};
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("route.json");
        let settings = Settings {
            to: "Colonia".into(),
            plotter: Plotter::Neutron,
            engine: Engine::Local,
            efficiency: 60,
            refuel_every_scoopable: false,
            range: 39.87,
        };
        let d = Desk::open(Some(p.clone()), Box::new(|_| Ok(())));
        d.set(spansh_neutron().plotted_with(Some(settings.clone()))).unwrap();
        drop(d);
        let v = Desk::open(Some(p.clone()), Box::new(|_| Ok(()))).view();
        assert_eq!(v.route.unwrap().plot, Some(settings));
        std::fs::write(
            &p,
            r#"{"route":{"name":"Sol to Colonia","source":"neutron plotter","waypoints":[{"system":"Sol","jumps":0},{"system":"Colonia","jumps":40}],"next":1},"here":"Sol"}"#,
        )
        .unwrap();
        let v = Desk::open(Some(p), Box::new(|_| Ok(()))).view();
        let r = v.route.unwrap();
        assert_eq!((r.plot, r.waypoints.len(), v.stops[1].jumps), (None, 2, Some(40)));
    }
}
