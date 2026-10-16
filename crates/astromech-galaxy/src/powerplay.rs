//! Nearby Powerplay targets for the pledged power, over Spansh's systems search (filter shapes
//! in `astromech_data::spansh`): systems to acquire (unoccupied, the power active there),
//! reinforce (controlled by the power) and undermine (controlled by another power). Spansh
//! ignores filters it doesn't know, so every hit is re-checked in [`Target::matches`].

use crate::{dist, estimate, Facts, Filters, Galaxy, Here, Hop};
use anyhow::Result;
use astromech_data::spansh::{Reference, Search, System};
use serde::{Deserialize, Serialize};

/// What kind of work to look for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    Acquire,
    Reinforce,
    Undermine,
}

const CONTROLLED: &[&str] = &["Exploited", "Fortified", "Stronghold"];

impl Target {
    /// The Spansh search for `power`'s targets of this kind within `within_ly` of `origin`.
    pub fn search(self, power: &str, origin: [f64; 3], within_ly: f64, size: usize) -> Search {
        let q = Search::near(Reference::Coords(origin)).within(within_ly).size(size);
        match self {
            Target::Acquire => q.power_state(&["Unoccupied"]).power(&[power]),
            Target::Reinforce => q.controlling_power(&[power]),
            Target::Undermine => {
                let others: Vec<&str> = astromech_powerplay::rules().powers().filter(|p| *p != power).collect();
                q.power_state(CONTROLLED).controlling_power(&others)
            }
        }
    }

    pub fn matches(self, power: &str, s: &System) -> bool {
        let controller = s.controlling_power.as_deref();
        match self {
            Target::Acquire => {
                controller.is_none()
                    && s.power_state.as_deref() == Some("Unoccupied")
                    && s.power.iter().any(|p| p == power)
            }
            Target::Reinforce => controller == Some(power),
            Target::Undermine => controller.is_some_and(|c| c != power),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TargetHit {
    pub name: String,
    pub id64: i64,
    #[serde(flatten)]
    pub hop: Hop,
    pub controlling_power: Option<String>,
    pub state: Option<String>,
    /// Controlled systems: progress through the state's band, a fraction.
    pub control_progress: Option<f64>,
    pub reinforcement: Option<i64>,
    pub undermining: Option<i64>,
    /// Unoccupied systems: the pledged power's acquisition progress.
    pub acquisition: Option<f64>,
    /// Powers active there other than the controller.
    pub others: Vec<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Targets {
    pub target: Target,
    pub power: String,
    pub here: Here,
    pub range: f64,
    pub within_ly: f64,
    /// When Spansh was asked (cached up to `astromech_data::ttl::SEARCH`).
    pub fetched: String,
    pub hits: Vec<TargetHit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

/// Results asked of Spansh per search; the nearest `limit` are kept after re-checking.
const SEARCH_SIZE: usize = 50;

impl Galaxy {
    /// `power`'s nearest targets of kind `target` within `within_ly` of `here`, nearest first,
    /// with jump counts at `range`.
    pub fn powerplay_targets(
        &self,
        target: Target,
        power: &str,
        here: &Here,
        range: f64,
        within_ly: f64,
        limit: usize,
        filters: &Filters,
    ) -> Result<Targets> {
        // Fill-in is best effort: without it the jump counts are estimates.
        let _ = self.ensure_covered(here.pos);
        let page = self.remote.spansh_systems(&target.search(power, here.pos, within_ly, SEARCH_SIZE))?;
        let now = crate::now_s();
        let n = page.value.results.len();
        let mut matched: Vec<System> = page.value.results.into_iter().filter(|s| target.matches(power, s)).collect();
        let warning = (n > 0 && matched.is_empty())
            .then(|| "Spansh returned systems that don't fit the search; its filters may have changed.".to_string());
        matched.retain(|s| filters.keeps(&Facts { updated_at: s.updated_at.as_deref(), ..Facts::default() }, now));
        {
            let mut st = self.store();
            let _ = st.insert_systems(matched.iter().map(|s| (s.id64, s.name.as_str(), s.pos())));
        }
        let map = self.map().ok();
        let mut hits: Vec<TargetHit> = matched
            .into_iter()
            .map(|s| {
                let hop = match &map {
                    Some(m) => m.hop(here.pos, s.pos(), range),
                    None => {
                        let ly = dist(here.pos, s.pos());
                        Hop { ly, jumps: estimate(ly, range), exact: false }
                    }
                };
                TargetHit {
                    acquisition: s.power_conflict_progress.iter().find(|c| c.power == power).map(|c| c.progress),
                    others: s
                        .power
                        .iter()
                        .filter(|p| Some(p.as_str()) != s.controlling_power.as_deref())
                        .cloned()
                        .collect(),
                    name: s.name,
                    id64: s.id64,
                    hop,
                    controlling_power: s.controlling_power,
                    state: s.power_state,
                    control_progress: s.power_state_control_progress,
                    reinforcement: s.power_state_reinforcement,
                    undermining: s.power_state_undermining,
                    updated_at: s.updated_at,
                }
            })
            .collect();
        hits.sort_by(|a, b| a.hop.ly.total_cmp(&b.hop.ly));
        hits.truncate(limit.max(1));
        Ok(Targets {
            target,
            power: power.to_string(),
            here: here.clone(),
            range,
            within_ly,
            fetched: crate::iso(page.fetched_at),
            hits,
            warning,
        })
    }
}
