//! Following a multi-stop route (docs/PLOTTING.md, layer 1). The game has no API for setting a
//! destination, so the next waypoint goes on the clipboard, ready to paste into the galaxy
//! map's search box, and is announced. Arriving in a waypoint's system (the journal's
//! `FSDJump`, `CarrierJump` or `Location`) moves the route on and copies the stop after it.
//!
//! [`Route`] is the pure part. [`Desk`] owns the one active route: it saves it to
//! `route.json`, so it survives restarts and can be set from the app window while Elite is not
//! running, and it does the clipboard writes.
//!
//! (Not to be confused with the window's `--route`, which is a page in the app.)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Waypoint {
    pub system: String,
    /// What is there, e.g. the engineer's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl Waypoint {
    pub fn new(system: &str) -> Self {
        Self { system: system.trim().to_string(), label: None }
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

fn same_system(a: &str, b: &str) -> bool {
    a.trim().eq_ignore_ascii_case(b.trim())
}

impl Route {
    /// A route over `waypoints`, with blank systems dropped and consecutive stops in the same
    /// system merged (two engineers in one system are one jump target).
    pub fn new(name: &str, source: &str, waypoints: Vec<Waypoint>) -> Self {
        let mut out: Vec<Waypoint> = vec![];
        for w in waypoints {
            let w = Waypoint { system: w.system.trim().to_string(), label: w.label.filter(|l| !l.trim().is_empty()) };
            if w.system.is_empty() {
                continue;
            }
            match out.last_mut() {
                Some(last) if same_system(&last.system, &w.system) => {
                    last.label = match (last.label.take(), w.label) {
                        (Some(a), Some(b)) if a != b => Some(format!("{a}, {b}")),
                        (a, b) => a.or(b),
                    };
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

fn named(w: &Waypoint) -> String {
    match &w.label {
        Some(l) => format!("{}, for {l}", w.system),
        None => w.system.clone(),
    }
}

/// "Next waypoint: Deciat, for Felicity Farseer. It's on your clipboard. 3 waypoints left."
pub fn next_line(next: &Waypoint, left: usize, copied: bool) -> String {
    let mut s = format!("Next waypoint: {}.", named(next));
    s += if copied { " It's on your clipboard." } else { " I couldn't copy it to the clipboard." };
    if left > 1 {
        s += &format!(" {left} waypoints left.");
    }
    s
}

impl Moved {
    pub fn spoken(&self) -> String {
        match self {
            Moved::Arrived { next, left, copied, .. } => next_line(next, *left, *copied),
            Moved::Finished { at } => format!("{} reached. That was the last waypoint.", at.system),
        }
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
    /// Why the last clipboard write failed, if it did.
    pub clipboard_error: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Saved {
    route: Option<Route>,
    #[serde(default)]
    here: Option<String>,
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
            clipboard_error: st.1.clone(),
        }
    }

    pub fn view(&self) -> View {
        Self::view_of(&self.state.lock().unwrap())
    }

    /// Start following `route`. If we are already in one of its systems it starts after it.
    /// The next waypoint is copied straight away.
    pub fn set(&self, mut route: Route) -> Result<View, String> {
        if route.waypoints.is_empty() {
            return Err("the route has no systems".into());
        }
        let mut st = self.state.lock().unwrap();
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
        st.0.route = Some(route);
        self.copy(&mut st);
        self.save(&st.0);
        Ok(Self::view_of(&st))
    }

    pub fn clear(&self) -> View {
        let mut st = self.state.lock().unwrap();
        if st.0.route.take().is_some() {
            crate::logging::log("route: cleared");
        }
        st.1 = None;
        self.save(&st.0);
        Self::view_of(&st)
    }

    /// Skip (`forward`) or go back one waypoint, and copy the new next one.
    pub fn step(&self, forward: bool) -> View {
        let mut st = self.state.lock().unwrap();
        let moved = st.0.route.as_mut().map(|r| if forward { r.skip() } else { r.back() }).unwrap_or(false);
        if moved {
            self.copy(&mut st);
            self.save(&st.0);
        }
        Self::view_of(&st)
    }

    /// Copy the next waypoint again (something else took the clipboard).
    pub fn copy_next(&self) -> View {
        let mut st = self.state.lock().unwrap();
        self.copy(&mut st);
        Self::view_of(&st)
    }

    /// The journal put us in `system`.
    pub fn on_system(&self, system: &str) -> Option<Moved> {
        let mut st = self.state.lock().unwrap();
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
                g.lock().unwrap().push(s.to_string());
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
        assert_eq!(got.lock().unwrap().as_slice(), ["Alpha Centauri"]);
        assert_eq!(d.on_system("Wolf 359"), None);
        let m = d.on_system("alpha centauri").unwrap();
        assert!(matches!(&m, Moved::Arrived { next, left: 1, copied: true, .. } if next.system == "Barnard's Star"));
        assert_eq!(m.spoken(), "Next waypoint: Barnard's Star. It's on your clipboard.");
        assert!(matches!(d.on_system("Barnard's Star"), Some(Moved::Finished { .. })));
        assert_eq!(d.view().remaining, 0);
        assert_eq!(got.lock().unwrap().len(), 2);
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
        assert_eq!(got.lock().unwrap().as_slice(), ["A", "B", "B"]);
        assert!(d.clear().route.is_none());
        assert!(d.set(route(&[" ", ""])).is_err());
    }

    #[test]
    fn clipboard_failure_is_reported() {
        let d = Desk::open(None, Box::new(|_| Err("busy".into())));
        let v = d.set(route(&["A", "B"])).unwrap();
        assert_eq!(v.clipboard_error.as_deref(), Some("busy"));
        let m = d.on_system("A").unwrap();
        assert!(m.spoken().contains("couldn't copy"));
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

    #[test]
    fn merges_stops_in_one_system() {
        let r = Route::new(
            "trip",
            "engineer trip",
            vec![
                Waypoint { system: "Deciat".into(), label: Some("Felicity Farseer".into()) },
                Waypoint { system: "deciat".into(), label: Some("Someone Else".into()) },
                Waypoint { system: "Kuk".into(), label: Some("Ram Tah".into()) },
            ],
        );
        assert_eq!(r.waypoints.len(), 2);
        assert_eq!(r.waypoints[0].label.as_deref(), Some("Felicity Farseer, Someone Else"));
        assert_eq!(
            next_line(&r.waypoints[1], 2, true),
            "Next waypoint: Kuk, for Ram Tah. It's on your clipboard. 2 waypoints left."
        );
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
