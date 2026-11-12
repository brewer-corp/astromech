//! Route follower (`route_follow`, docs/PLOTTING.md): arriving in a system (`FSDJump`,
//! `CarrierJump`, `Location`) hands it to the route the core follows. Arriving in a waypoint
//! moves the route on, copies the next waypoint to the clipboard and says it: "Next waypoint.
//! Deciat." The core owns the route, the clipboard and the lines ([`Follower`]); this rule only
//! decides when, so the engine's rules hold for it: nothing moves the route or touches the
//! clipboard unless the event is live (a replayed journal only tells it where the commander
//! is), `[callouts.route_follow]` turns it off (it still tells the route where the commander
//! is), and speech waits out combat and docking (the clipboard and the toast don't).
//!
//! A route plotted in the galaxy map (a new `NavRoute.json`, live) and `NavRouteClear` are
//! handed over too: the follower decides whether the plot is a leg of its route or a new one. A
//! `NavRouteClear` read while the route file still has hops is ignored: either a newer plot is in
//! the file, or the file's clear (an empty `NavRoute.json` after one with hops) is still to come.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, JournalEvent};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// The route the core follows.
pub trait Follower: Send + Sync {
    /// The commander arrived in `system` (live): move the route on if it is a waypoint (copying
    /// the next one) and return the line to say, if any.
    fn arrived(&self, system: &str) -> Option<String>;
    /// A replayed event put the commander in `system`: remember it, nothing else (the route
    /// doesn't move and the clipboard isn't written).
    fn here(&self, system: &str);
    /// The commander plotted a route in the galaxy map (live): `NavRoute.json`'s systems with
    /// their star classes, the current system first.
    fn plotted(&self, hops: &[astromech_journal::RouteHop]);
    /// The commander cleared the galaxy map's route (live `NavRouteClear`).
    fn cleared(&self);
}

/// The follower the core plugs in. Shared like the visit history.
#[derive(Clone, Default)]
pub struct SharedFollower(Arc<RwLock<Option<Arc<dyn Follower>>>>);

impl SharedFollower {
    pub fn set(&self, f: Arc<dyn Follower>) {
        *self.0.write().unwrap() = Some(f);
    }

    fn get(&self) -> Option<Arc<dyn Follower>> {
        self.0.read().unwrap().clone()
    }
}

impl std::fmt::Debug for SharedFollower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SharedFollower({})", if self.get().is_some() { "set" } else { "none" })
    }
}

pub struct RouteFollow {
    pub follower: SharedFollower,
}

/// The system an arrival event puts the commander in.
fn arrival<'a>(input: &Input<'a>) -> Option<&'a str> {
    let Input::Journal(l) = input else { return None };
    match &l.event {
        JournalEvent::FsdJump { star_system, .. } | JournalEvent::Location { star_system, .. } => {
            Some(star_system.as_str()).filter(|s| !s.is_empty())
        }
        _ => None,
    }
}

impl Rule for RouteFollow {
    fn id(&self) -> &'static str {
        "route_follow"
    }
    /// Each arrival is its own news.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        let f = self.follower.get()?;
        match input {
            // The game rewrites `NavRoute.json` empty on a clear, so the file alone orders plots
            // and clears, whichever of it and the journal line is read first.
            Input::Route(r) if r.hops.is_empty() => {
                f.cleared();
                return None;
            }
            Input::Route(r) => {
                f.plotted(&r.hops);
                return None;
            }
            // The clear is ignored while the route file last read still has hops: `NavRoute.json`
            // is read on Status.json changes, ahead of journal lines that lag, so a re-plot after
            // the clear may already be in it (and if the empty file is still to come, its read
            // clears).
            Input::Journal(l) if matches!(l.event, JournalEvent::Extra(ExtraEvent::NavRouteClear)) => {
                if state.route_hops.is_empty() {
                    f.cleared();
                }
                return None;
            }
            _ => {}
        }
        let text = f.arrived(arrival(input)?)?;
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
    fn replayed(&self, input: &Input, _state: &GameState) {
        if let (Some(f), Some(system)) = (self.follower.get(), arrival(input)) {
            f.here(system);
        }
    }
    /// Turned off, the route doesn't follow, but where the commander is stays current.
    fn while_off(&self, input: &Input, state: &GameState) {
        self.replayed(input, state);
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;
    use crate::{Engine, Settings};
    use std::sync::Mutex;
    use std::time::Instant;

    #[derive(Default)]
    struct Log(Mutex<Vec<String>>);
    impl Follower for Log {
        fn arrived(&self, system: &str) -> Option<String> {
            self.0.lock().unwrap().push(format!("arrived {system}"));
            Some(format!("Next waypoint. {system}."))
        }
        fn here(&self, system: &str) {
            self.0.lock().unwrap().push(format!("here {system}"));
        }
        fn plotted(&self, hops: &[astromech_journal::RouteHop]) {
            let systems: Vec<&str> = hops.iter().map(|h| h.system.as_str()).collect();
            self.0.lock().unwrap().push(format!("plotted {}", systems.join(">")));
        }
        fn cleared(&self) {
            self.0.lock().unwrap().push("cleared".into());
        }
    }

    fn line(json: &str) -> astromech_journal::JournalLine {
        astromech_journal::parse_line(json).unwrap()
    }

    #[test]
    fn only_live_arrivals_move_the_route() {
        let log = Arc::new(Log::default());
        let shared = SharedFollower::default();
        shared.set(log.clone());
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &Settings::default());
        let g = GameState::default();
        let jump = line(r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Kuk","JumpDist":9.0}"#);
        assert!(e.journal(&jump, &g, Instant::now()).is_empty(), "replayed: silent");
        e.set_live(true);
        let fired = e.journal(&jump, &g, Instant::now());
        assert_eq!(fired.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(), ["Next waypoint. Kuk."]);
        let carrier = line(r#"{"timestamp":"t","event":"CarrierJump","StarSystem":"Sol"}"#);
        let other = line(r#"{"timestamp":"t","event":"Docked","StationName":"X","StationType":"Coriolis"}"#);
        e.journal(&carrier, &g, Instant::now());
        e.journal(&other, &g, Instant::now());
        assert_eq!(*log.0.lock().unwrap(), ["here Kuk", "arrived Kuk", "arrived Sol"]);
    }

    /// Off, the route doesn't move and nothing is said, but the follower still learns where the
    /// commander is, replayed or live (the Route page's "you are here", and a route set later
    /// starting after the system the commander is in).
    #[test]
    fn the_toggle_turns_following_off_but_not_where_you_are() {
        let log = Arc::new(Log::default());
        let shared = SharedFollower::default();
        shared.set(log.clone());
        let off: Settings = toml::from_str("[route_follow]\nenabled = false\n").unwrap();
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &off);
        let g = GameState::default();
        let replayed = line(r#"{"timestamp":"t","event":"Location","StarSystem":"Sol"}"#);
        assert!(e.journal(&replayed, &g, Instant::now()).is_empty());
        e.set_live(true);
        let jump = line(r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Kuk","JumpDist":9.0}"#);
        assert!(e.journal(&jump, &g, Instant::now()).is_empty());
        assert_eq!(*log.0.lock().unwrap(), ["here Sol", "here Kuk"]);
    }

    /// Galaxy-map plots and clears reach the follower only live and with following on, and say
    /// nothing themselves.
    #[test]
    fn galaxy_map_plots_and_clears_are_handed_over_live() {
        use astromech_journal::NavRouteSnap;
        let log = Arc::new(Log::default());
        let shared = SharedFollower::default();
        shared.set(log.clone());
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &Settings::default());
        let g = GameState::default();
        let snap = |s: &[&str]| NavRouteSnap {
            systems: s.iter().map(|x| x.to_string()).collect(),
            hops: s
                .iter()
                .map(|x| astromech_journal::RouteHop { system: x.to_string(), ..Default::default() })
                .collect(),
        };
        let clear = line(r#"{"timestamp":"t","event":"NavRouteClear","Route":[]}"#);
        assert!(e.route(&snap(&["Sol", "Kuk"]), &g, Instant::now()).is_empty(), "replayed");
        e.journal(&clear, &g, Instant::now());
        e.set_live(true);
        assert!(e.route(&snap(&["Sol", "Wolf 359", "Kuk"]), &g, Instant::now()).is_empty());
        assert!(e.journal(&clear, &g, Instant::now()).is_empty());
        assert_eq!(*log.0.lock().unwrap(), ["plotted Sol>Wolf 359>Kuk", "cleared"]);

        let log = Arc::new(Log::default());
        let shared = SharedFollower::default();
        shared.set(log.clone());
        let off: Settings = toml::from_str("[route_follow]\nenabled = false\n").unwrap();
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &off);
        e.set_live(true);
        e.route(&snap(&["Sol", "Kuk"]), &g, Instant::now());
        e.journal(&clear, &g, Instant::now());
        assert!(log.0.lock().unwrap().is_empty(), "following off: the galaxy map's route isn't mirrored");
    }

    /// A clear then a re-plot, with the journal read late: the new route file (read on a
    /// Status.json change) reaches the follower before the clear's journal line. The clear is
    /// older than the route the file holds, so it is ignored (astromech-k1fw).
    #[test]
    fn a_clear_read_after_a_newer_plot_is_ignored() {
        let log = Arc::new(Log::default());
        let shared = SharedFollower::default();
        shared.set(log.clone());
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &Settings::default());
        e.set_live(true);
        let snap = |s: &[&str]| astromech_journal::NavRouteSnap {
            systems: s.iter().map(|x| x.to_string()).collect(),
            hops: s
                .iter()
                .map(|x| astromech_journal::RouteHop { system: x.to_string(), ..Default::default() })
                .collect(),
        };
        let mut g = GameState::default();
        let a = snap(&["Sol", "Kuk"]);
        g.apply_route(&a);
        e.route(&a, &g, Instant::now());
        // The game cleared A and plotted B; the file (B) is read before the journal's lines.
        let b = snap(&["Sol", "Wolf 359", "Deciat"]);
        g.apply_route(&b);
        e.route(&b, &g, Instant::now());
        let clear = line(r#"{"timestamp":"t","event":"NavRouteClear","Route":[]}"#);
        e.journal(&clear, &g, Instant::now());
        assert_eq!(*log.0.lock().unwrap(), ["plotted Sol>Kuk", "plotted Sol>Wolf 359>Deciat"]);

        // A clear whose file was read first (now empty) still clears.
        let empty = snap(&[]);
        g.apply_route(&empty);
        e.route(&empty, &g, Instant::now());
        e.journal(&clear, &g, Instant::now());
        assert_eq!(log.0.lock().unwrap()[2..], ["cleared", "cleared"]);
    }

    /// A clear whose journal line is read before the game rewrites the file: the line is
    /// ignored (the file still has the route), and the empty file read after it clears.
    #[test]
    fn a_clear_read_before_its_empty_file_clears_when_the_file_is_read() {
        let log = Arc::new(Log::default());
        let shared = SharedFollower::default();
        shared.set(log.clone());
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &Settings::default());
        e.set_live(true);
        let snap = |s: &[&str]| astromech_journal::NavRouteSnap {
            systems: s.iter().map(|x| x.to_string()).collect(),
            hops: s
                .iter()
                .map(|x| astromech_journal::RouteHop { system: x.to_string(), ..Default::default() })
                .collect(),
        };
        let mut g = GameState::default();
        let a = snap(&["Sol", "Kuk"]);
        g.apply_route(&a);
        e.route(&a, &g, Instant::now());
        let clear = line(r#"{"timestamp":"t","event":"NavRouteClear","Route":[]}"#);
        e.journal(&clear, &g, Instant::now());
        assert_eq!(*log.0.lock().unwrap(), ["plotted Sol>Kuk"], "the file still has the route");
        let empty = snap(&[]);
        g.apply_route(&empty);
        e.route(&empty, &g, Instant::now());
        assert_eq!(*log.0.lock().unwrap(), ["plotted Sol>Kuk", "cleared"]);
        // The empty file read again is not news.
        e.route(&empty, &g, Instant::now());
        assert_eq!(log.0.lock().unwrap().len(), 2);
    }
}
