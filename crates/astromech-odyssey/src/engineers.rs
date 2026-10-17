//! On-foot engineers: what they fit, the commander's access from the journal (`EngineerProgress`
//! lists them alongside the ship engineers, ids 4000xx), and the referral chain that reveals
//! them.

use crate::catalog::{catalog, Engineer};
use astromech_journal::JournalEvent;
use serde::Serialize;
use std::collections::BTreeMap;

/// Engineer id → `Known`, `Invited`, `Acquainted` or `Unlocked`, as last logged.
pub type Progress = BTreeMap<i64, String>;

pub fn apply(progress: &mut Progress, ev: &JournalEvent) {
    let JournalEvent::EngineerProgress { engineers } = ev else { return };
    for e in engineers {
        let id = e.engineer_id.or_else(|| catalog().engineers.iter().find(|x| x.name == e.engineer).map(|x| x.id));
        if let Some(id) = id.filter(|id| catalog().engineers.iter().any(|x| x.id == *id)) {
            progress.insert(id, e.progress.clone());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModRef {
    pub id: String,
    pub name: String,
    /// `suit` or `weapon`.
    pub kind: String,
    /// The wiki's sources disagree on whether this engineer fits it.
    pub unconfirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EngineerLine {
    pub id: i64,
    pub name: String,
    pub system: String,
    pub colonia: bool,
    /// From the journal, or `Unknown` when the game has not mentioned them.
    pub progress: String,
    pub mods: Vec<ModRef>,
    pub referred_by: Vec<String>,
    /// Engineers this one's referral reveals.
    pub refers: Vec<String>,
    pub invite: String,
    pub referral_task: Option<String>,
    /// What to do next for this engineer, or empty when nothing is left.
    pub next: String,
}

pub fn progress_of(progress: &Progress, e: &Engineer) -> String {
    progress.get(&e.id).cloned().unwrap_or_else(|| "Unknown".into())
}

pub fn unlocked(progress: &Progress, id: i64) -> bool {
    progress.get(&id).is_some_and(|p| p.eq_ignore_ascii_case("Unlocked"))
}

fn and_list(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

pub fn lines(progress: &Progress) -> Vec<EngineerLine> {
    let c = catalog();
    c.engineers
        .iter()
        .map(|e| {
            let state = progress_of(progress, e);
            let refers: Vec<String> =
                c.engineers.iter().filter(|x| x.referred_by.contains(&e.name)).map(|x| x.name.clone()).collect();
            let unrevealed: Vec<String> = c
                .engineers
                .iter()
                .filter(|x| refers.contains(&x.name) && progress_of(progress, x) == "Unknown")
                .map(|x| x.name.clone())
                .collect();
            let next = match state.as_str() {
                "Unlocked" => match (&e.referral_task, unrevealed.is_empty()) {
                    (Some(task), false) => format!("Referral to {}: {task}", and_list(&unrevealed)),
                    _ => String::new(),
                },
                "Invited" | "Acquainted" => format!("Visit {} to unlock", e.system),
                "Known" => format!("To get invited: {}", e.invite),
                _ if e.referred_by.is_empty() => format!("To get invited: {}", e.invite),
                _ => format!("Needs a referral from {}", and_list(&e.referred_by)),
            };
            EngineerLine {
                id: e.id,
                name: e.name.clone(),
                system: e.system.clone(),
                colonia: e.colonia,
                progress: state,
                mods: e
                    .mods
                    .iter()
                    .filter_map(|m| c.modification(m))
                    .map(|m| ModRef {
                        id: m.id.clone(),
                        name: m.name.clone(),
                        kind: m.kind.clone(),
                        unconfirmed: e.unconfirmed_mods.contains(&m.id),
                    })
                    .collect(),
                referred_by: e.referred_by.clone(),
                refers,
                invite: e.invite.clone(),
                referral_task: e.referral_task.clone(),
                next,
            }
        })
        .collect()
}
