//! Blueprints the commander picks (a suit or weapon upgrade over a grade range, or a
//! modification) and what they cost against the inventory: per pick what is missing, and one
//! shopping list for them all.

use crate::catalog::{catalog, Materials};
use crate::engineers::{unlocked, Progress};
use crate::inventory::{category_title, Holdings};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Pick {
    /// Upgrade a suit or weapon from grade `from` to grade `to` (each step in between).
    Upgrade { gear: String, from: u8, to: u8 },
    /// A modification; `maker` picks the weapon maker's cost for weapon mods.
    Mod {
        id: String,
        #[serde(default)]
        maker: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Need {
    pub id: String,
    pub name: String,
    /// `goods`, `assets`, `data` or `consumables`.
    pub category: &'static str,
    pub need: i64,
    pub have: i64,
    pub short: i64,
    /// Barter value of one, for assets a bartender trades.
    pub barter: Option<i64>,
}

/// An engineer who fits a picked modification.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FitBy {
    pub name: String,
    pub system: String,
    pub unlocked: bool,
    /// The wiki's sources disagree on whether this engineer fits it.
    pub unconfirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PickResult {
    pub pick: Pick,
    pub title: String,
    pub lines: Vec<Need>,
    pub credits: i64,
    /// Some step's price is not known, so `credits` is a lower bound.
    pub credits_partial: bool,
    /// Engineers who fit it (modifications); upgrades are bought at Pioneer Supplies.
    pub fitted_by: Vec<FitBy>,
    /// Nothing is short.
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Shopping {
    pub picks: Vec<PickResult>,
    /// Every pick's materials together, against the same inventory.
    pub total: Vec<Need>,
    pub credits: i64,
    pub credits_partial: bool,
    /// Barter value of the short assets: what a bartender would want for them.
    pub barter_short: i64,
}

fn needs(m: &Materials, h: &Holdings) -> Vec<Need> {
    let c = catalog();
    let mut out: Vec<Need> = m
        .iter()
        .filter(|(_, n)| **n > 0)
        .map(|(id, &need)| {
            let have = h.have(id);
            let item = c.items.get(id);
            Need {
                id: id.clone(),
                name: c.item_name(id),
                category: category_title(item.map_or("", |i| i.category.as_str())),
                need,
                have,
                short: (need - have).max(0),
                barter: item.and_then(|i| i.barter),
            }
        })
        .collect();
    // Short first, then by name.
    out.sort_by(|a, b| (b.short > 0).cmp(&(a.short > 0)).then_with(|| a.name.cmp(&b.name)));
    out
}

fn add(into: &mut Materials, m: &Materials) {
    for (k, v) in m {
        *into.entry(k.clone()).or_default() += v;
    }
}

fn maker_title(maker: &str) -> &str {
    match maker {
        "kinematic" => "Kinematic Armaments",
        "manticore" => "Manticore",
        "takada" => "Takada",
        other => other,
    }
}

/// The materials, credits (and whether some are unknown), title and fitters of one pick.
fn cost(pick: &Pick, progress: &Progress) -> Result<(Materials, i64, bool, String, Vec<FitBy>), String> {
    let c = catalog();
    let mut m = Materials::new();
    let mut credits = 0;
    let mut partial = false;
    match pick {
        Pick::Upgrade { gear, from, to } => {
            let g = c.gear(gear).ok_or_else(|| format!("unknown suit or weapon `{gear}`"))?;
            if !(1..=4).contains(from) || !(2..=5).contains(to) || from >= to {
                return Err(format!("{}: can't upgrade from grade {from} to {to}", g.name));
            }
            for u in g.upgrades.iter().filter(|u| u.grade > *from && u.grade <= *to) {
                add(&mut m, &u.materials);
                credits += u.credits.unwrap_or(0);
                partial |= u.credits.is_none();
            }
            Ok((m, credits, partial, format!("{} grade {from} → {to}", g.name), vec![]))
        }
        Pick::Mod { id, maker } => {
            let md = c.modification(id).ok_or_else(|| format!("unknown modification `{id}`"))?;
            let cost = md.cost(maker.as_deref()).ok_or_else(|| match maker {
                Some(mk) => format!("{}: no cost for {} weapons", md.name, maker_title(mk)),
                None => format!("{}: pick the weapon maker", md.name),
            })?;
            add(&mut m, &cost.materials);
            credits += cost.credits.unwrap_or(0);
            partial |= cost.credits.is_none();
            let title = match (&cost.maker, md.kind.as_str()) {
                (Some(mk), _) => format!("{} ({} weapons)", md.name, maker_title(mk)),
                (None, "suit") => format!("{} (suit)", md.name),
                (None, _) => format!("{} (weapon)", md.name),
            };
            let fitted_by = c
                .engineers
                .iter()
                .filter(|e| e.mods.contains(&md.id))
                .map(|e| FitBy {
                    name: e.name.clone(),
                    system: e.system.clone(),
                    unlocked: unlocked(progress, e.id),
                    unconfirmed: e.unconfirmed_mods.contains(&md.id),
                })
                .collect();
            Ok((m, credits, partial, title, fitted_by))
        }
    }
}

/// What each pick needs against `h`, and the shopping list for all of them.
pub fn shopping(picks: &[Pick], h: &Holdings, progress: &Progress) -> Result<Shopping, String> {
    let mut all = Materials::new();
    let mut credits = 0;
    let mut credits_partial = false;
    let mut results = Vec::with_capacity(picks.len());
    for p in picks {
        let (m, cr, partial, title, fitted_by) = cost(p, progress)?;
        add(&mut all, &m);
        credits += cr;
        credits_partial |= partial;
        let lines = needs(&m, h);
        results.push(PickResult {
            pick: p.clone(),
            title,
            ready: lines.iter().all(|l| l.short == 0),
            lines,
            credits: cr,
            credits_partial: partial,
            fitted_by,
        });
    }
    let total = needs(&all, h);
    let barter_short = total.iter().filter_map(|l| l.barter.map(|b| b * l.short)).sum();
    Ok(Shopping { picks: results, total, credits, credits_partial, barter_short })
}
