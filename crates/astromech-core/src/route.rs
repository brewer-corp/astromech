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

use astromech_api::Recover;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Waypoint {
    pub system: String,
    /// What is there, e.g. the engineer's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Jumps from the waypoint before, where the route knows them: a plotted route
    /// (`astromech_galaxy::plot`) and one plotted in the galaxy map do, an import or a lookup's
    /// stop doesn't. `route.json` from before it was recorded loads without it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jumps: Option<u32>,
}

impl Waypoint {
    pub fn new(system: &str) -> Self {
        Self { system: system.trim().to_string(), label: None, jumps: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    /// Where it came from: "engineer trip", "nearby", "import".
    #[serde(default)]
    pub source: String,
    pub waypoints: Vec<Waypoint>,
    /// Index of the next waypoint; `waypoints.len()` once the route is finished.
    #[serde(default)]
    pub next: usize,
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
                }
                _ => out.push(w),
            }
        }
        Self { name: name.trim().to_string(), source: source.to_string(), waypoints: out, next: 0 }
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
            // The stop after it is now reached from the new stop, so its jumps are unknown too.
            if let Some(after) = self.waypoints.get_mut(i) {
                after.jumps = None;
            }
            self.waypoints.insert(i, stop);
        }
    }
}

/// Waypoints from pasted text: a Spansh CSV export (any router; the "System Name" column, with
/// repeats for road-to-riches bodies merged), or one system name per line.
pub fn parse_list(text: &str) -> Vec<Waypoint> {
    let mut lines = text.lines().map(str::trim).filter(|l| !l.is_empty()).peekable();
    let Some(first) = lines.peek() else { return vec![] };
    let header = split_csv(first);
    let col = header.iter().position(|h| h.eq_ignore_ascii_case("system name") || h.eq_ignore_ascii_case("system"));
    let out: Vec<Waypoint> = match col {
        Some(c) if header.len() > 1 || first.contains('"') => {
            lines.skip(1).filter_map(|l| split_csv(l).get(c).map(|s| Waypoint::new(s))).collect()
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize)]
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
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Saved {
    route: Option<Route>,
    #[serde(default)]
    here: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parked: Option<Route>,
}

pub type Clip = Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>;

/// The one active route.
pub struct Desk {
    path: Option<PathBuf>,
    state: Mutex<(Saved, Option<String>)>,
    clip: Clip,
}

impl Desk {
    /// Load `path` (a missing or broken file is no route) and copy with `clip`.
    pub fn open(path: Option<PathBuf>, clip: Clip) -> Self {
        let saved = path
            .as_ref()
            .and_then(|p| std::fs::read(p).ok())
            .and_then(|b| serde_json::from_slice::<Saved>(&b).ok())
            .unwrap_or_default();
        Self { path, state: Mutex::new((saved, None)), clip }
    }

    /// The real one: `<data>/route.json` and the Windows clipboard.
    pub fn in_dir(data: &std::path::Path) -> Self {
        let _ = std::fs::create_dir_all(data);
        Self::open(Some(data.join("route.json")), Box::new(crate::clipboard::set_text))
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

    fn view_of(st: &(Saved, Option<String>)) -> View {
        let r = st.0.route.as_ref();
        View {
            route: r.cloned(),
            here: st.0.here.clone(),
            next: r.and_then(|r| r.next_stop().cloned()),
            remaining: r.map(|r| r.remaining()).unwrap_or(0),
            jumps_left: r.and_then(Route::jumps_left),
            clipboard_error: st.1.clone(),
            parked: st.0.parked.clone(),
        }
    }

    pub fn view(&self) -> View {
        Self::view_of(&self.state.lock().recover())
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
        Self::view_of(st)
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
        Ok(Self::view_of(&st))
    }

    /// Stop following the route, and forget a parked one.
    pub fn clear(&self) -> View {
        let mut st = self.state.lock().recover();
        if st.0.route.take().is_some() | st.0.parked.take().is_some() {
            crate::logging::log("route: cleared");
        }
        st.1 = None;
        self.save(&st.0);
        Self::view_of(&st)
    }

    /// Skip (`forward`) or go back one waypoint, and copy the new next one.
    pub fn step(&self, forward: bool) -> View {
        let mut st = self.state.lock().recover();
        let moved = st.0.route.as_mut().map(|r| if forward { r.skip() } else { r.back() }).unwrap_or(false);
        if moved {
            self.copy(&mut st);
            self.save(&st.0);
        }
        Self::view_of(&st)
    }

    /// Copy the next waypoint again (something else took the clipboard).
    pub fn copy_next(&self) -> View {
        let mut st = self.state.lock().recover();
        self.copy(&mut st);
        Self::view_of(&st)
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
        Ok(Self::view_of(&st))
    }

    /// The commander plotted `systems` in the galaxy map (live; the current system first). A
    /// plot to a waypoint ahead on the route followed is a leg of it: nothing changes. A plot to
    /// a waypoint ahead on the parked route resumes that. Anything else is followed as a route
    /// from the game, and an unfinished route of our own that it replaces is parked.
    pub fn plotted(&self, systems: &[String]) {
        let Some(dest) = systems.last().filter(|_| systems.len() > 1) else { return };
        let mut st = self.state.lock().recover();
        if st.0.route.as_ref().is_some_and(|r| !r.from_game() && r.ahead(dest)) {
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
        // The galaxy map's route has every jump as a waypoint.
        let jumps = systems[1..].iter().map(|s| Waypoint { jumps: Some(1), ..Waypoint::new(s) }).collect();
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
    fn plotted(&self, systems: &[String]) {
        Desk::plotted(self, systems);
    }
    fn cleared(&self) {
        self.cleared_in_game();
    }
}

#[cfg(test)]
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
        d.plotted(&systems.iter().map(|s| s.to_string()).collect::<Vec<_>>());
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
        Waypoint { system: system.into(), label: Some(label.into()), jumps: None }
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
}
