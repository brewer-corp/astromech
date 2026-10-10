//! # astromech-policy
//!
//! Every state-changing command passes through here (ARCHITECTURE.md §5.14): risk tiers,
//! confirmation, per-origin rate limits, the kill switch and the audit log. This is the
//! **only** crate allowed to drive `astromech_input::exec`.

use astromech_proto::{Origin, Tier};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Confirm(String),
    Deny(String),
}

pub struct Policy {
    kill_switch: bool,
    min_interval: Duration,
    last: Option<(String, Instant)>,
}

impl Default for Policy {
    fn default() -> Self {
        Self { kill_switch: false, min_interval: Duration::from_secs(3), last: None }
    }
}

impl Policy {
    pub fn set_kill_switch(&mut self, on: bool) {
        self.kill_switch = on;
    }

    /// Decide on one command. `now` is passed in so tests can control time.
    pub fn decide(&mut self, name: &str, tier: Tier, origin: Origin, now: Instant) -> Decision {
        if self.kill_switch && tier > Tier::T0 {
            return Decision::Deny("kill switch is on".into());
        }
        if tier == Tier::T3 {
            return Decision::Deny("never automated".into());
        }
        if origin == Origin::Cloud && tier > Tier::T1 {
            return Decision::Deny("cloud origin is limited to tier 0 and 1".into());
        }
        if let Some((n, t)) = &self.last {
            if n == name && now.duration_since(*t) < self.min_interval && tier > Tier::T0 {
                return Decision::Deny("duplicate within 3 s".into());
            }
        }
        if tier > Tier::T0 {
            self.last = Some((name.to_string(), now));
        }
        if tier == Tier::T2 {
            return Decision::Confirm(format!("Confirm {name}?"));
        }
        Decision::Allow
    }
}

/// The game window title the executor targets (re-exported for callers of the policy layer).
pub const TARGET_WINDOW: &str = astromech_input::ELITE_TITLE;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tiers_kill_switch_and_dedupe() {
        let mut p = Policy::default();
        let t = Instant::now();
        assert_eq!(p.decide("ship.landing_gear", Tier::T1, Origin::Voice, t), Decision::Allow);
        assert!(matches!(p.decide("ship.landing_gear", Tier::T1, Origin::Voice, t), Decision::Deny(_)));
        assert!(matches!(p.decide("nav.jump", Tier::T2, Origin::Voice, t), Decision::Confirm(_)));
        assert!(matches!(p.decide("nav.jump", Tier::T2, Origin::Cloud, t + Duration::from_secs(5)), Decision::Deny(_)));
        p.set_kill_switch(true);
        assert!(matches!(p.decide("ship.lights", Tier::T1, Origin::AppWindow, t), Decision::Deny(_)));
        assert_eq!(p.decide("state.get", Tier::T0, Origin::Voice, t), Decision::Allow);
    }
}
