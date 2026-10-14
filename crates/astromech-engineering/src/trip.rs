//! Which engineers to visit, and in what order, to finish a plan.
//!
//! Jobs come from the plan: the remaining grade rolls of each target and each experimental.
//! A grade job can be done remotely only through an engineer at rank 5 whose pinned blueprint it
//! is (remote crafting earns no reputation, so anything below rank 5 is done in person).
//! Experimentals are always in person. Every in-person job can be done by any unlocked engineer
//! who offers it; experimentals by any unlocked engineer who works on that module type.
//!
//! Choosing engineers and order is a group travelling-salesman problem: visit at least one
//! engineer per job, as few jumps as possible. Plans are small, so it is solved exactly by
//! dynamic programming over (jobs covered, current engineer), up to 16 distinct candidate sets;
//! larger plans fall back to a greedy route.

use crate::data;
use crate::engineers::{self, EngineerView};
use crate::fleet::Ship;
use crate::plan::TargetCost;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

/// One leg: light years, jumps at the ship's range, and whether the count came from the star
/// map (exact) or `ceil(ly / range)`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Hop {
    pub ly: f64,
    pub jumps: u32,
    pub exact: bool,
}

/// Each stop costs this many jumps' worth on top of the travel (landing, docking, the base's
/// supercruise leg), so the solver prefers fewer stops when the jump counts are close.
const STOP_COST: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Job {
    pub slot: String,
    pub slot_name: String,
    pub module: String,
    /// `grades` or `experimental`.
    pub kind: String,
    pub blueprint: String,
    /// "Increased range G3 → G5", "Mass Manager".
    pub what: String,
    pub grade: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Stop {
    pub engineer: String,
    pub system: String,
    pub rank: Option<u8>,
    /// From the previous stop (or the start).
    pub hop: Hop,
    pub jobs: Vec<Job>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RemoteJob {
    pub engineer: String,
    pub job: Job,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Blocked {
    pub job: Job,
    pub reason: String,
    /// Engineers who could do it once unlocked, with their progress.
    pub unlock: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Trip {
    pub start: Option<String>,
    pub range: f64,
    pub stops: Vec<Stop>,
    pub remote: Vec<RemoteJob>,
    pub blocked: Vec<Blocked>,
    pub total_jumps: u32,
    /// Every leg was counted on the star map (not estimated).
    pub exact: bool,
}

fn job_from(t: &TargetCost, kind: &str, ship: &Ship) -> Job {
    let module =
        ship.modules.iter().find(|m| m.slot == t.slot).map(|m| m.name.clone()).unwrap_or_else(|| t.module.clone());
    let (what, grade) = if kind == "grades" {
        let from = t.steps.first().map(|s| s.0).unwrap_or(t.grade);
        let to = t.steps.last().map(|s| s.0).unwrap_or(t.grade);
        let what = if from == to {
            format!("{} G{to}", t.blueprint_name)
        } else {
            format!("{} G{from} → G{to}", t.blueprint_name)
        };
        (what, to)
    } else {
        (t.experimental_name.clone().unwrap_or_default(), 0)
    };
    Job {
        slot: t.slot.clone(),
        slot_name: t.slot_name.clone(),
        module,
        kind: kind.into(),
        blueprint: t.blueprint.clone(),
        what,
        grade,
        note: None,
    }
}

/// Engineers who work on this module group (any blueprint, any grade).
fn group_engineers(group: &str) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    if let Some(g) = data::group(group) {
        for b in &g.blueprints {
            if let Some((_, bp)) = data::blueprint(b) {
                for r in &bp.grades {
                    for e in &r.engineers {
                        if !out.iter().any(|x| engineers::key(x) == engineers::key(e)) {
                            out.push(e.clone());
                        }
                    }
                }
            }
        }
    }
    out
}

/// Plan the trip. `start` is the commander's position, `hop` counts jumps between two points.
pub fn plan_trip(
    ship: &Ship,
    targets: &[TargetCost],
    engs: &[EngineerView],
    pins: &HashMap<String, String>,
    start: Option<(&str, [f64; 3])>,
    range: f64,
    hop: &dyn Fn([f64; 3], [f64; 3]) -> Hop,
) -> Trip {
    let by_key: HashMap<String, &EngineerView> = engs.iter().map(|e| (engineers::key(&e.name), e)).collect();
    let mut remote = vec![];
    let mut blocked = vec![];
    // (job, candidate engineer keys)
    let mut jobs: Vec<(Job, Vec<String>)> = vec![];
    for t in targets.iter().filter(|t| t.note.as_deref().is_none_or(|n| !n.contains("cannot take"))) {
        if !t.steps.is_empty() {
            let mut job = job_from(t, "grades", ship);
            let via_pin = engs.iter().find(|e| {
                e.unlocked()
                    && e.rank == Some(5)
                    && pins.get(&engineers::key(&e.name)).is_some_and(|b| b.eq_ignore_ascii_case(&t.blueprint))
                    && engineers::offers(&e.name, &t.blueprint) >= job.grade
            });
            if let Some(e) = via_pin {
                remote.push(RemoteJob { engineer: e.name.clone(), job });
            } else {
                let offer: Vec<&EngineerView> =
                    engs.iter().filter(|e| engineers::offers(&e.name, &t.blueprint) >= job.grade).collect();
                let cands: Vec<String> =
                    offer.iter().filter(|e| e.unlocked()).map(|e| engineers::key(&e.name)).collect();
                if cands.is_empty() {
                    blocked.push(Blocked {
                        reason: format!("No unlocked engineer offers {} G{}.", t.blueprint_name, job.grade),
                        unlock: offer.iter().map(|e| (e.name.clone(), e.progress.clone())).collect(),
                        job,
                    });
                } else {
                    let low: Vec<String> = offer
                        .iter()
                        .filter(|e| e.unlocked() && e.rank.unwrap_or(0) < job.grade)
                        .map(|e| format!("{} (rank {})", e.name, e.rank.unwrap_or(0)))
                        .collect();
                    if low.len() == cands.len() {
                        job.note = Some(format!("Rank rises as you craft in person: {}.", low.join(", ")));
                    }
                    jobs.push((job, cands));
                }
            }
        }
        if t.experimental_needed {
            let job = job_from(t, "experimental", ship);
            let group =
                ship.modules.iter().find(|m| m.slot == t.slot).and_then(|m| m.group.clone()).unwrap_or_default();
            let all = group_engineers(&group);
            let cands: Vec<String> = all
                .iter()
                .filter(|n| by_key.get(&engineers::key(n)).is_some_and(|e| e.unlocked()))
                .map(|n| engineers::key(n))
                .collect();
            if cands.is_empty() {
                blocked.push(Blocked {
                    reason: format!(
                        "Experimentals are applied in person; no unlocked engineer works on {}.",
                        job.module
                    ),
                    unlock: all
                        .iter()
                        .map(|n| {
                            (n.clone(), by_key.get(&engineers::key(n)).map(|e| e.progress.clone()).unwrap_or_default())
                        })
                        .collect(),
                    job,
                });
            } else {
                jobs.push((job, cands));
            }
        }
    }

    // Nodes: candidate engineers.
    let mut nodes: Vec<&EngineerView> = vec![];
    for (_, c) in &jobs {
        for k in c {
            if let Some(e) = by_key.get(k) {
                if !nodes.iter().any(|n| n.name == e.name) {
                    nodes.push(e);
                }
            }
        }
    }
    // Distinct candidate sets → groups, each a bitmask over nodes.
    let node_ix = |k: &str| nodes.iter().position(|n| engineers::key(&n.name) == k);
    let mut group_masks: Vec<u64> = vec![];
    let mut job_group: Vec<usize> = vec![];
    for (_, c) in &jobs {
        let m = c.iter().filter_map(|k| node_ix(k)).fold(0u64, |m, i| m | (1 << i));
        let g = group_masks.iter().position(|&x| x == m).unwrap_or_else(|| {
            group_masks.push(m);
            group_masks.len() - 1
        });
        job_group.push(g);
    }
    let n = nodes.len();
    let leg = |a: Option<usize>, b: usize| -> Hop {
        match (a, start) {
            (Some(i), _) => hop(nodes[i].coords, nodes[b].coords),
            (None, Some((_, p))) => hop(p, nodes[b].coords),
            (None, None) => Hop { ly: 0.0, jumps: 0, exact: true },
        }
    };
    let order = if jobs.is_empty() {
        vec![]
    } else if group_masks.len() <= 16 && n <= 40 {
        solve_exact(n, &group_masks, &leg)
    } else {
        solve_greedy(n, &group_masks, &leg)
    };

    // Assign each job to the first stop that can do it.
    let mut stops: Vec<Stop> = vec![];
    let mut done = vec![false; jobs.len()];
    let mut prev: Option<usize> = None;
    for &i in &order {
        let picked: Vec<usize> =
            (0..jobs.len()).filter(|&j| !done[j] && group_masks[job_group[j]] & (1 << i) != 0).collect();
        let mine: Vec<Job> = picked
            .iter()
            .map(|&j| {
                done[j] = true;
                jobs[j].0.clone()
            })
            .collect();
        if mine.is_empty() {
            continue;
        }
        stops.push(Stop {
            engineer: nodes[i].name.clone(),
            system: nodes[i].system.clone(),
            rank: nodes[i].rank,
            hop: leg(prev, i),
            jobs: mine,
        });
        prev = Some(i);
    }
    Trip {
        start: start.map(|s| s.0.to_string()),
        range,
        total_jumps: stops.iter().map(|s| s.hop.jumps).sum(),
        exact: stops.iter().all(|s| s.hop.exact),
        stops,
        remote,
        blocked,
    }
}

fn covers(groups: &[u64], node: usize) -> u32 {
    groups.iter().enumerate().filter(|(_, m)| *m & (1 << node) != 0).fold(0u32, |a, (g, _)| a | (1 << g))
}

/// Exact: dp[covered groups][last node] = fewest jumps (+ stop cost).
fn solve_exact(n: usize, groups: &[u64], leg: &dyn Fn(Option<usize>, usize) -> Hop) -> Vec<usize> {
    let g = groups.len();
    let full: u32 = (1 << g) - 1;
    let cov: Vec<u32> = (0..n).map(|i| covers(groups, i)).collect();
    let start: Vec<u32> = (0..n).map(|i| leg(None, i).jumps + STOP_COST).collect();
    let mut d = vec![vec![0u32; n]; n];
    for (a, row) in d.iter_mut().enumerate() {
        for (b, c) in row.iter_mut().enumerate() {
            if a != b {
                *c = leg(Some(a), b).jumps + STOP_COST;
            }
        }
    }
    let states = 1usize << g;
    let inf = u32::MAX;
    let mut dp = vec![inf; states * n];
    let mut from = vec![usize::MAX; states * n];
    for i in 0..n {
        let m = cov[i] as usize;
        if start[i] < dp[m * n + i] {
            dp[m * n + i] = start[i];
        }
    }
    for m in 0..states {
        for i in 0..n {
            let c = dp[m * n + i];
            if c == inf || m as u32 == full {
                continue;
            }
            for j in 0..n {
                if cov[j] as usize & !m == 0 {
                    continue; // only stops that cover something new
                }
                let nm = m | cov[j] as usize;
                let nc = c + d[i][j];
                if nc < dp[nm * n + j] {
                    dp[nm * n + j] = nc;
                    from[nm * n + j] = m * n + i;
                }
            }
        }
    }
    let Some(end) = (0..n).filter(|&i| dp[full as usize * n + i] != inf).min_by_key(|&i| dp[full as usize * n + i])
    else {
        return vec![];
    };
    let mut path = vec![];
    let mut s = full as usize * n + end;
    loop {
        path.push(s % n);
        let f = from[s];
        if f == usize::MAX {
            break;
        }
        s = f;
    }
    path.reverse();
    path
}

/// Greedy: next stop = most new groups per jump.
fn solve_greedy(n: usize, groups: &[u64], leg: &dyn Fn(Option<usize>, usize) -> Hop) -> Vec<usize> {
    let mut left: BTreeMap<usize, u64> = groups.iter().copied().enumerate().collect();
    let mut at: Option<usize> = None;
    let mut path = vec![];
    while !left.is_empty() {
        let best = (0..n)
            .filter_map(|i| {
                let gain = left.values().filter(|m| *m & (1 << i) != 0).count();
                (gain > 0).then(|| (i, gain as f64 / (leg(at, i).jumps + STOP_COST) as f64))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1));
        let Some((i, _)) = best else { break };
        left.retain(|_, m| *m & (1 << i) == 0);
        path.push(i);
        at = Some(i);
    }
    path
}
