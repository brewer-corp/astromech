//! # astromech-tools
//!
//! The typed tool registry (ARCHITECTURE.md §5.13): every voice-exposed query becomes a tier-0
//! tool; commands are registered with a tier and go through `astromech-policy`.
//!
//! **Rule:** depends on `astromech-policy`, never on `astromech-input` directly.

use astromech_policy::{Decision, Policy};
use astromech_proto::{Origin, Tier};
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: &'static str,
    pub tier: Tier,
    pub summary: &'static str,
}

#[derive(Default)]
pub struct Registry {
    tools: BTreeMap<&'static str, ToolSpec>,
}

impl Registry {
    pub fn register(&mut self, spec: ToolSpec) {
        self.tools.insert(spec.name, spec);
    }
    pub fn get(&self, name: &str) -> Option<&ToolSpec> {
        self.tools.get(name)
    }
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.tools.keys().copied()
    }
    /// Look the tool up and ask the policy layer. Execution itself happens in the core.
    pub fn authorize(&self, policy: &mut Policy, name: &str, origin: Origin) -> Decision {
        match self.get(name) {
            Some(t) => policy.decide(t.name, t.tier, origin, Instant::now()),
            None => Decision::Deny(format!("unknown tool {name}")),
        }
    }
}

/// The MVP's first tools.
pub fn mvp() -> Registry {
    let mut r = Registry::default();
    r.register(ToolSpec { name: "ship.landing_gear", tier: Tier::T1, summary: "Deploy or retract the landing gear" });
    r.register(ToolSpec { name: "ship.hardpoints", tier: Tier::T1, summary: "Deploy or retract hardpoints" });
    r.register(ToolSpec { name: "state.get", tier: Tier::T0, summary: "Current game state" });
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_goes_through_policy() {
        let r = mvp();
        let mut p = Policy::default();
        assert_eq!(r.authorize(&mut p, "ship.landing_gear", Origin::Voice), Decision::Allow);
        assert!(matches!(r.authorize(&mut p, "self.destruct", Origin::Voice), Decision::Deny(_)));
        assert_eq!(r.names().count(), 3);
    }
}
