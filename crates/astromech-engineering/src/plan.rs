//! Build plan → materials: what each target costs, the total against the inventory, trader swaps
//! from surplus, and where to farm the rest.
//!
//! Since 3.0 a grade must be completed before the next one opens, each roll costs one full grade
//! recipe, and the number of rolls per grade depends on engineer rank and luck. [`Rolls`] is an
//! estimate the user can change. A different blueprint starts again from grade 1.

use crate::data;
use crate::fleet::{Applied, Ship};
use astromech_materials::catalog::{self, Kind, Spec};
use astromech_materials::{Ledger, SourceNote};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Estimated rolls to complete grades 1–5.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rolls(pub [u32; 5]);

impl Default for Rolls {
    fn default() -> Self {
        Rolls([1, 2, 3, 4, 5])
    }
}

impl Rolls {
    pub fn of(&self, grade: u8) -> u32 {
        self.0.get(grade.saturating_sub(1) as usize).copied().unwrap_or(1).max(1)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub ship_id: i64,
    pub targets: Vec<Target>,
    #[serde(default)]
    pub rolls: Rolls,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub slot: String,
    pub blueprint: String,
    pub grade: u8,
    #[serde(default)]
    pub experimental: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlanResult {
    pub ship_id: i64,
    pub rolls: Rolls,
    pub targets: Vec<TargetCost>,
    /// Every material the plan uses, rarest first.
    pub materials: Vec<MatLine>,
    pub trades: Vec<Trade>,
    /// Still short after the suggested trades.
    pub farm: Vec<MatLine>,
    pub snapshot_ts: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TargetCost {
    pub slot: String,
    pub slot_name: String,
    pub module: String,
    pub blueprint: String,
    pub blueprint_name: String,
    pub grade: u8,
    pub experimental: Option<String>,
    pub experimental_name: Option<String>,
    /// Current state of the module, if engineered.
    pub from: Option<Applied>,
    /// (grade, rolls) still to do.
    pub steps: Vec<(u8, u32)>,
    pub experimental_needed: bool,
    pub materials: Vec<(String, i64)>,
    /// Engineers who can apply the top grade.
    pub engineers: Vec<String>,
    /// Why nothing is needed, or what is wrong with the target.
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MatLine {
    pub id: String,
    pub name: String,
    pub kind: Option<Kind>,
    pub group: String,
    pub grade: u8,
    pub need: i64,
    pub have: i64,
    pub short: i64,
    pub capacity: i64,
    /// Needs more than fits in storage at once: craft in batches.
    pub over_cap: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceNote>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Trade {
    pub give_id: String,
    pub give_name: String,
    pub give: i64,
    pub get_id: String,
    pub get_name: String,
    pub get: i64,
    pub trader: String,
}

/// Cost of one target against the ship's current module.
pub fn cost_target(ship: &Ship, t: &Target, rolls: &Rolls) -> TargetCost {
    let fitted = ship.modules.iter().find(|m| m.slot.eq_ignore_ascii_case(&t.slot));
    let bp = data::blueprint(&t.blueprint);
    let ex = t.experimental.as_deref().filter(|e| !e.is_empty()).and_then(data::experimental);
    let mut out = TargetCost {
        slot: t.slot.clone(),
        slot_name: fitted.map(|m| m.slot_name.clone()).unwrap_or_else(|| crate::fleet::slot_name(&t.slot)),
        module: fitted.map(|m| m.name.clone()).unwrap_or_default(),
        blueprint: bp.map(|(id, _)| id.to_string()).unwrap_or_else(|| t.blueprint.clone()),
        blueprint_name: bp.map(|(_, b)| b.name.clone()).unwrap_or_else(|| t.blueprint.clone()),
        grade: t.grade,
        experimental: ex.map(|(id, _)| id.to_string()),
        experimental_name: ex.map(|(_, e)| e.name.clone()),
        from: fitted.and_then(|m| m.engineering.clone()),
        steps: vec![],
        experimental_needed: false,
        materials: vec![],
        engineers: vec![],
        note: None,
    };
    let Some(m) = fitted else {
        out.note = Some("No module in this slot.".into());
        return out;
    };
    let Some((bp_id, bp)) = bp else {
        out.note = Some(format!("Unknown blueprint {}.", t.blueprint));
        return out;
    };
    let allowed = m.group.as_deref().and_then(data::group);
    if !allowed.is_some_and(|g| g.blueprints.iter().any(|b| b == bp_id)) {
        out.note = Some(format!("{} cannot take {}.", m.name, bp.name));
        return out;
    }
    let target = t.grade.clamp(1, bp.max_grade());
    out.grade = target;
    let same = m.engineering.as_ref().filter(|e| e.blueprint.eq_ignore_ascii_case(bp_id));
    let (start, done_q) = match same {
        Some(e) if e.grade >= 1 => (e.grade, e.quality),
        _ => (1, 0.0),
    };
    if same.is_none() && m.engineering.is_some() {
        out.note = Some("Replaces the current blueprint; starts again at grade 1.".into());
    }
    let mut mats: BTreeMap<String, i64> = BTreeMap::new();
    if start <= target {
        for g in start..=target {
            let r = rolls.of(g);
            let n = if g == start && same.is_some() { ((1.0 - done_q) * r as f64).ceil().max(0.0) as u32 } else { r };
            if n == 0 {
                continue;
            }
            out.steps.push((g, n));
            if let Some(rec) = bp.grade(g) {
                for (id, c) in &rec.materials {
                    *mats.entry(id.clone()).or_default() += c * n as i64;
                }
            }
        }
    } else {
        out.note = Some(format!("Already grade {start}, above the target."));
    }
    if let Some((ex_id, e)) = ex {
        let has = m
            .engineering
            .as_ref()
            .and_then(|a| a.experimental.as_deref())
            .is_some_and(|x| x.eq_ignore_ascii_case(ex_id));
        let fits = allowed.is_some_and(|g| g.experimentals.iter().any(|x| x == ex_id));
        if !fits {
            out.note = Some(format!("{} cannot take {}.", m.name, e.name));
        } else if !has || same.is_none() {
            out.experimental_needed = true;
            for (id, c) in &e.materials {
                *mats.entry(id.clone()).or_default() += c;
            }
        }
    }
    out.engineers = bp.grade(target).map(|r| r.engineers.clone()).unwrap_or_default();
    if out.steps.is_empty() && !out.experimental_needed && out.note.is_none() {
        out.note = Some("Already done.".into());
    }
    out.materials = mats.into_iter().collect();
    out
}

pub fn evaluate(ship: &Ship, plan: &Plan, ledger: &Ledger) -> PlanResult {
    let targets: Vec<TargetCost> = plan.targets.iter().map(|t| cost_target(ship, t, &plan.rolls)).collect();
    let mut need: BTreeMap<String, i64> = BTreeMap::new();
    for t in &targets {
        for (id, n) in &t.materials {
            *need.entry(id.clone()).or_default() += n;
        }
    }
    let mut materials: Vec<MatLine> = need.iter().map(|(id, n)| mat_line(id, *n, ledger)).collect();
    materials.sort_by(|a, b| b.grade.cmp(&a.grade).then_with(|| a.name.cmp(&b.name)));
    let (trades, left) = suggest_trades(&need, ledger);
    let mut farm: Vec<MatLine> = materials
        .iter()
        .filter_map(|m| {
            let s = left.get(&m.id).copied().unwrap_or(0);
            (s > 0).then(|| MatLine { short: s, ..m.clone() })
        })
        .collect();
    farm.sort_by(|a, b| b.grade.cmp(&a.grade).then_with(|| b.short.cmp(&a.short)));
    PlanResult {
        ship_id: plan.ship_id,
        rolls: plan.rolls,
        targets,
        materials,
        trades,
        farm,
        snapshot_ts: ledger.snapshot_ts.clone(),
    }
}

fn mat_line(id: &str, need: i64, ledger: &Ledger) -> MatLine {
    let spec = catalog::lookup(id);
    let have = ledger.count(id);
    let grade = spec.map(|s| s.grade).unwrap_or(1);
    let capacity = catalog::capacity(grade);
    MatLine {
        id: id.to_string(),
        name: spec.map(|s| ledger.display_name(s)).unwrap_or_else(|| id.to_string()),
        kind: spec.map(|s| s.kind),
        group: spec.map(|s| s.group.to_string()).unwrap_or_default(),
        grade,
        need,
        have,
        short: (need - have).max(0),
        capacity,
        over_cap: need > capacity,
        sources: spec.map(astromech_materials::hge::sources_for).unwrap_or_default(),
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Material trader exchange for one trade block: give `.0` of `from` for `.1` of `to`.
/// 6:1 per grade up, 1:3 per grade down, another ×6 when the category changes
/// (same rule as EDDiscovery's trader panel).
pub fn rate(from: &Spec, to: &Spec) -> Option<(i64, i64)> {
    if from.kind != to.kind || !from.on_trader || !to.on_trader || from.id == to.id {
        return None;
    }
    let cross: i64 = if from.group != to.group { 6 } else { 1 };
    let (give, get) = if from.grade < to.grade {
        (6i64.pow((to.grade - from.grade) as u32) * cross, 1)
    } else if from.grade > to.grade {
        (cross, 3i64.pow((from.grade - to.grade) as u32))
    } else {
        (cross, 1)
    };
    let g = gcd(give, get);
    Some((give / g, get / g))
}

/// Value kept per unit traded (1.0 = lossless). Higher is better.
fn efficiency(from: &Spec, to: &Spec, give: i64, get: i64) -> f64 {
    let v = |s: &Spec| 6f64.powi(s.grade as i32 - 1);
    (v(to) * get as f64) / (v(from) * give as f64)
}

/// Greedy trader plan: rarest shortfall first, each from the donor that loses the least value,
/// preferring donors closest to their cap. Only spends what the plan itself does not need.
pub fn suggest_trades(need: &BTreeMap<String, i64>, ledger: &Ledger) -> (Vec<Trade>, BTreeMap<String, i64>) {
    let mut spare: BTreeMap<&'static str, i64> = BTreeMap::new();
    for s in catalog::catalog().iter().filter(|s| s.on_trader) {
        let n = ledger.count(s.id) - need.get(s.id).copied().unwrap_or(0);
        if n > 0 {
            spare.insert(s.id, n);
        }
    }
    let mut short: Vec<(&'static Spec, i64)> = need
        .iter()
        .filter_map(|(id, n)| {
            let s = catalog::lookup(id)?;
            let gap = n - ledger.count(id);
            (gap > 0).then_some((s, gap))
        })
        .collect();
    short.sort_by(|a, b| b.0.grade.cmp(&a.0.grade).then_with(|| a.0.id.cmp(b.0.id)));
    let mut left: BTreeMap<String, i64> =
        need.iter().map(|(id, n)| (id.clone(), (n - ledger.count(id)).max(0))).filter(|(_, n)| *n > 0).collect();
    let mut trades = vec![];
    for (to, mut gap) in short {
        let mut donors: Vec<(&'static Spec, i64, i64, f64, f64)> = spare
            .iter()
            .filter_map(|(id, n)| {
                let from = catalog::lookup(id)?;
                let (give, get) = rate(from, to)?;
                let fill = *n as f64 / catalog::capacity(from.grade) as f64;
                Some((from, give, get, efficiency(from, to, give, get), fill))
            })
            .collect();
        donors.sort_by(|a, b| {
            b.3.partial_cmp(&a.3)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal))
        });
        for (from, give, get, _, _) in donors {
            if gap <= 0 {
                break;
            }
            let avail = spare.get(from.id).copied().unwrap_or(0);
            let blocks = ((gap + get - 1) / get).min(avail / give);
            if blocks <= 0 {
                continue;
            }
            *spare.entry(from.id).or_default() -= blocks * give;
            let got = blocks * get;
            gap -= got;
            trades.push(Trade {
                give_id: from.id.to_string(),
                give_name: ledger.display_name(from),
                give: blocks * give,
                get_id: to.id.to_string(),
                get_name: ledger.display_name(to),
                get: got,
                trader: to.kind.trader().to_string(),
            });
        }
        left.insert(to.id.to_string(), gap.max(0));
    }
    left.retain(|_, n| *n > 0);
    (trades, left)
}
