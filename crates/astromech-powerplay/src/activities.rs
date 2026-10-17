//! What earns merits in a system, as data (`data/catalog/powerplay-activities.json`): which
//! activities count for the pledged power given what the system is to it (acquire, conflict,
//! reinforce, undermine) and its Powerplay state, with each power's preferred (ethos)
//! activities and the controlling power's weak spots. Frontier rebalances these often, so the
//! table is versioned data with its sources and fetch date; rules nobody has checked in game
//! carry an `uncertain` note.

use crate::system::{SystemPower, Work};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

/// What the system is to the pledged power, as the rules table names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Context {
    Acquire,
    /// An acquisition system where two or more powers, mine included, passed the conflict
    /// threshold; adds to the acquire rules.
    Conflict,
    Reinforce,
    Undermine,
}

#[derive(Deserialize)]
struct Activity {
    id: String,
    name: String,
    #[serde(default)]
    odyssey: bool,
}

#[derive(Deserialize)]
struct Rule {
    work: Context,
    activity: String,
    /// Journal `PowerplayState` values the rule is limited to; empty for all.
    #[serde(default)]
    states: Vec<String>,
    /// Controlling powers the rule is limited to; empty for all.
    #[serde(default)]
    controlled_by: Vec<String>,
    how: String,
    #[serde(default)]
    favoured_by: Vec<String>,
    uncertain: Option<String>,
    /// Listed before everything else (a warning that decides whether the rest count).
    #[serde(default)]
    first: bool,
    /// Earns no merits at the moment; kept so a fix is a one-line data change.
    suspended: Option<String>,
}

#[derive(Deserialize)]
struct Commodities {
    acquire: String,
    reinforce: String,
    undermine: String,
}

#[derive(Deserialize)]
struct ActivitiesFile {
    version: u32,
    fetched: String,
    conflict_progress: f64,
    activities: Vec<Activity>,
    rules: Vec<Rule>,
    weak_to: HashMap<String, Vec<String>>,
    commodities: HashMap<String, Commodities>,
}

/// One thing to do here for merits.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Suggestion {
    /// Stable id from the rules table, e.g. `bounty-hunting`.
    pub activity: String,
    pub name: String,
    /// Where and how, with the pledged power's commodity names filled in.
    pub how: String,
    /// An on-foot (Odyssey) activity.
    pub odyssey: bool,
    /// The pledged power's ethos favours it (bonus merits).
    pub favoured: bool,
    /// The controlling power is weak to it (undermining only).
    pub weak_spot: bool,
    /// Why the rule is unverified, if it is.
    pub uncertain: Option<String>,
}

/// The rules table's version and fetch date, shown with the suggestions.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RulesVersion {
    pub version: u32,
    pub fetched: String,
}

fn file() -> &'static ActivitiesFile {
    static F: OnceLock<ActivitiesFile> = OnceLock::new();
    F.get_or_init(|| {
        serde_json::from_str(include_str!("../../../data/catalog/powerplay-activities.json"))
            .expect("powerplay-activities.json")
    })
}

/// The bundled table's version and fetch date.
pub fn rules_version() -> RulesVersion {
    let f = file();
    RulesVersion { version: f.version, fetched: f.fetched.clone() }
}

impl SystemPower {
    /// Two or more powers, `me` among them, past the conflict threshold in an unoccupied system.
    fn in_conflict(&self, me: &str, threshold: f64) -> bool {
        let past: Vec<&str> =
            self.acquisition.iter().filter(|a| a.progress >= threshold).map(|a| a.power.as_str()).collect();
        past.len() >= 2 && past.contains(&me)
    }

    /// What earns `pledged` merits here: pinned rules (the stronghold carrier warning) first,
    /// then favoured and weak-spot activities, then table order. Empty when not pledged or the system offers the power nothing.
    pub fn suggestions(&self, pledged: Option<&str>) -> Vec<Suggestion> {
        let (Some(me), Some(work)) = (pledged, self.work_for(pledged)) else { return vec![] };
        let f = file();
        let contexts: &[Context] = match work {
            Work::Reinforce => &[Context::Reinforce],
            Work::Undermine => &[Context::Undermine],
            Work::Acquire if self.in_conflict(me, f.conflict_progress) => &[Context::Acquire, Context::Conflict],
            Work::Acquire => &[Context::Acquire],
        };
        let controller = self.controlling_power.as_deref();
        let weak = controller.and_then(|c| f.weak_to.get(c));
        let goods = f.commodities.get(me);
        let fill = |how: &str| match goods {
            Some(g) => how
                .replace("{acquire}", &g.acquire)
                .replace("{reinforce}", &g.reinforce)
                .replace("{undermine}", &g.undermine),
            None => how
                .replace("{acquire}", "your power's acquisition commodity")
                .replace("{reinforce}", "its reinforcement commodity")
                .replace("{undermine}", "its undermining commodity"),
        };
        let mut out: Vec<(bool, Suggestion)> = f
            .rules
            .iter()
            .filter(|r| contexts.contains(&r.work) && r.suspended.is_none())
            .filter(|r| r.states.is_empty() || self.state.as_ref().is_some_and(|s| r.states.contains(s)))
            .filter(|r| {
                r.controlled_by.is_empty() || controller.is_some_and(|c| r.controlled_by.iter().any(|p| p == c))
            })
            .map(|r| {
                let a = f.activities.iter().find(|a| a.id == r.activity).expect("rule names a listed activity");
                let s = Suggestion {
                    activity: a.id.clone(),
                    name: a.name.clone(),
                    how: fill(&r.how),
                    odyssey: a.odyssey,
                    favoured: r.favoured_by.iter().any(|p| p == me),
                    weak_spot: work == Work::Undermine && weak.is_some_and(|w| w.contains(&a.id)),
                    uncertain: r.uncertain.clone(),
                };
                (r.first, s)
            })
            .collect();
        out.sort_by_key(|(first, s)| (!first, !(s.favoured || s.weak_spot)));
        out.into_iter().map(|(_, s)| s).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules;

    /// The table is consistent: rules name listed activities and real powers, every power has
    /// weak spots and commodities, and each kind of system has rules.
    #[test]
    fn the_table_hangs_together() {
        let f = file();
        let powers: Vec<&str> = rules().powers().collect();
        let ids: Vec<&str> = f.activities.iter().map(|a| a.id.as_str()).collect();
        for r in &f.rules {
            assert!(ids.contains(&r.activity.as_str()), "unknown activity {}", r.activity);
            for p in r.favoured_by.iter().chain(&r.controlled_by) {
                assert!(powers.contains(&p.as_str()), "unknown power {p}");
            }
            for s in &r.states {
                assert!(["Exploited", "Fortified", "Stronghold"].contains(&s.as_str()), "unknown state {s}");
            }
        }
        for p in &powers {
            assert!(f.commodities.contains_key(*p), "no commodities for {p}");
            let w = f.weak_to.get(*p).unwrap_or_else(|| panic!("no weak spots for {p}"));
            assert!(w.iter().all(|a| ids.contains(&a.as_str())));
        }
        assert_eq!(f.weak_to.len(), 12);
        assert_eq!(f.commodities.len(), 12);
        for c in [Context::Acquire, Context::Conflict, Context::Reinforce, Context::Undermine] {
            assert!(f.rules.iter().any(|r| r.work == c), "no rules for {c:?}");
        }
        assert!(f.fetched.len() == 10 && f.version >= 1);
        // Every weak spot can show up: some undermining rule applies against that power.
        for (p, acts) in &f.weak_to {
            for a in acts {
                assert!(
                    f.rules.iter().any(|r| r.work == Context::Undermine
                        && &r.activity == a
                        && (r.controlled_by.is_empty() || r.controlled_by.contains(p))),
                    "{p} is weak to {a}, but no undermining rule suggests it"
                );
            }
        }
    }

    fn unoccupied(shares: &[(&str, f64)]) -> SystemPower {
        SystemPower {
            system: "Test".into(),
            controlling_power: None,
            state: Some("Unoccupied".into()),
            control_progress: None,
            reinforcement: None,
            undermining: None,
            contesting: vec![],
            acquisition: shares.iter().map(|(p, x)| crate::PowerShare { power: (*p).into(), progress: *x }).collect(),
        }
    }

    #[test]
    fn conflict_needs_two_powers_past_the_threshold_with_mine_among_them() {
        let me = "Li Yong-Rui";
        let conflict = |s: &SystemPower| s.suggestions(Some(me)).iter().any(|x| x.activity == "scan-ships");
        assert!(conflict(&unoccupied(&[(me, 0.3), ("Pranav Antal", 0.31)])));
        // Two others past it, mine below: acquiring, but not in the conflict.
        assert!(!conflict(&unoccupied(&[("Pranav Antal", 0.5), ("Felicia Winters", 0.4), (me, 0.1)])));
        // Mine past it alone.
        assert!(!conflict(&unoccupied(&[(me, 0.9), ("Pranav Antal", 0.1)])));
        assert!(!unoccupied(&[(me, 0.9)]).suggestions(Some(me)).is_empty());
    }
}
