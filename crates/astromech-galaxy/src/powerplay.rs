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

/// A system to collect from or deliver to.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LogisticsPoint {
    pub name: String,
    pub id64: i64,
    /// Distance from the target system.
    pub ly: f64,
    pub state: Option<String>,
    /// The commodity to collect here; `None` for delivery points.
    pub commodity: Option<String>,
    /// The farthest from the target this system may be, if the rules say.
    pub limit_ly: Option<f64>,
    pub uncertain: Option<String>,
}

/// Where to collect power commodities for a target system, and where to deliver them and the
/// matching power data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Logistics {
    pub work: Target,
    pub power: String,
    pub target: Here,
    pub fetched: String,
    /// Supplying systems of the pledged power, nearest the target first.
    pub collect: Vec<LogisticsPoint>,
    /// Commodities go to the Power Contact in the target system.
    pub deliver: Vec<LogisticsPoint>,
    /// Where data is handed in (target, supporting or controlled systems).
    pub deliver_data: Vec<LogisticsPoint>,
    pub warnings: Vec<String>,
}

impl Target {
    fn work(self) -> astromech_powerplay::Work {
        use astromech_powerplay::Work;
        match self {
            Target::Acquire => Work::Acquire,
            Target::Reinforce => Work::Reinforce,
            Target::Undermine => Work::Undermine,
        }
    }
}

impl Galaxy {
    /// Where `power` collects commodities for `work` at `target` (a system the power can do that
    /// work in; pick one from [`Galaxy::powerplay_targets`]) and where it delivers them and the
    /// data. Rules without a documented distance limit search within `within_ly`.
    pub fn powerplay_logistics(
        &self,
        work: Target,
        power: &str,
        target: &Here,
        within_ly: f64,
        limit: usize,
    ) -> Result<Logistics> {
        use astromech_powerplay::logistics::{self, HandIn};
        let w = work.work();
        let pickups: Vec<_> = logistics::pickups(w).collect();
        let radius = pickups.iter().map(|p| p.max_ly.unwrap_or(within_ly)).fold(0.0, f64::max);
        let delivers_data = logistics::data_delivery(w);
        let anywhere = matches!(delivers_data.map(|d| d.at), Some(HandIn::Controlled));
        let (states, radius): (Vec<&str>, f64) = if anywhere {
            (logistics::controlled_states().iter().map(String::as_str).collect(), within_ly.max(radius))
        } else {
            let mut s: Vec<&str> = pickups.iter().flat_map(|p| p.states.iter().map(String::as_str)).collect();
            s.dedup();
            (s, radius)
        };
        let _ = self.ensure_covered(target.pos);
        let q = Search::near(Reference::Coords(target.pos))
            .within(radius)
            .size(SEARCH_SIZE)
            .power_state(&states)
            .controlling_power(&[power]);
        let page = self.remote.spansh_systems(&q)?;
        // Spansh ignores filters it doesn't know: re-check, and keep the target out of its own supply.
        let mine: Vec<(System, f64)> = page
            .value
            .results
            .into_iter()
            .filter(|s| s.controlling_power.as_deref() == Some(power) && s.id64 != target.id64.unwrap_or(-1))
            .filter(|s| s.name != target.system)
            .map(|s| {
                let ly = dist(target.pos, s.pos());
                (s, ly)
            })
            .collect();
        let point =
            |s: &System, ly: f64, commodity: Option<String>, p: Option<&astromech_powerplay::logistics::Pickup>| {
                LogisticsPoint {
                    name: s.name.clone(),
                    id64: s.id64,
                    ly,
                    state: s.power_state.clone(),
                    commodity,
                    limit_ly: p.and_then(|p| p.max_ly),
                    uncertain: p.and_then(|p| p.uncertain.clone()),
                }
            };
        let mut collect: Vec<LogisticsPoint> = vec![];
        for (s, ly) in &mine {
            let state = s.power_state.as_deref().unwrap_or("");
            let rule =
                pickups.iter().find(|p| p.states.iter().any(|x| x == state) && ly <= &p.max_ly.unwrap_or(within_ly));
            if let Some(p) = rule {
                collect.push(point(s, *ly, astromech_powerplay::commodity(power, w).map(String::from), Some(p)));
            }
        }
        collect.sort_by(|a, b| a.ly.total_cmp(&b.ly));
        collect.truncate(limit.max(1));

        let me = LogisticsPoint {
            name: target.system.clone(),
            id64: target.id64.unwrap_or_default(),
            ly: 0.0,
            state: None,
            commodity: None,
            limit_ly: None,
            uncertain: None,
        };
        let deliver_data = match delivers_data {
            Some(d) => match d.at {
                HandIn::Target => vec![LogisticsPoint { uncertain: d.uncertain.clone(), ..me.clone() }],
                HandIn::Supporting => collect.iter().map(|c| LogisticsPoint { commodity: None, ..c.clone() }).collect(),
                HandIn::Controlled => {
                    let mut v: Vec<LogisticsPoint> = mine
                        .iter()
                        .filter(|(s, ly)| {
                            *ly <= radius
                                && s.power_state.as_ref().is_some_and(|x| logistics::controlled_states().contains(x))
                        })
                        .map(|(s, ly)| LogisticsPoint { uncertain: d.uncertain.clone(), ..point(s, *ly, None, None) })
                        .collect();
                    v.sort_by(|a, b| a.ly.total_cmp(&b.ly));
                    v.truncate(limit.max(1));
                    v
                }
            },
            None => vec![],
        };
        Ok(Logistics {
            work,
            power: power.to_string(),
            target: target.clone(),
            fetched: crate::iso(page.fetched_at),
            collect,
            deliver: vec![me],
            deliver_data,
            warnings: logistics::warnings().to_vec(),
        })
    }
}
