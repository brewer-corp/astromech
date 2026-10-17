//! Route follower (`route_follow`, docs/PLOTTING.md): arriving in a system (`FSDJump`,
//! `CarrierJump`, `Location`) hands it to the route the core follows. Arriving in a waypoint
//! moves the route on, copies the next waypoint to the clipboard and says it: "Next waypoint.
//! Deciat." The core owns the route, the clipboard and the lines ([`Follower`]); this rule only
//! decides when, so the engine's rules hold for it: nothing moves the route or touches the
//! clipboard unless the event is live (a replayed journal only tells it where the commander
//! is), `[callouts.route_follow]` turns it off, and speech waits out combat and docking (the
//! clipboard and the toast don't).

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::JournalEvent;
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
    fn react(&self, input: &Input, _state: &GameState) -> Option<Callout> {
        let text = self.follower.get()?.arrived(arrival(input)?)?;
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
    fn replayed(&self, input: &Input, _state: &GameState) {
        if let (Some(f), Some(system)) = (self.follower.get(), arrival(input)) {
            f.here(system);
        }
    }
}

#[cfg(test)]
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

    #[test]
    fn the_toggle_turns_it_off() {
        let log = Arc::new(Log::default());
        let shared = SharedFollower::default();
        shared.set(log.clone());
        let off: Settings = toml::from_str("[route_follow]\nenabled = false\n").unwrap();
        let mut e = Engine::new(vec![Box::new(RouteFollow { follower: shared })], &off);
        e.set_live(true);
        let jump = line(r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Kuk","JumpDist":9.0}"#);
        assert!(e.journal(&jump, &GameState::default(), Instant::now()).is_empty());
        assert!(log.0.lock().unwrap().is_empty());
    }
}
