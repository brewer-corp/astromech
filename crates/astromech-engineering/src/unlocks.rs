//! The engineer unlock guide as a tech tree: every ship and suit engineer
//! (`data/catalog/engineer-unlocks.json`, from the Elite Dangerous wiki), the engineers whose
//! introduction or referral they need (the edges), and the commander's progress from the
//! journal's `EngineerProgress`. Each engineer not yet unlocked names the requirement left, and
//! the ones that can be worked on now are marked `next`.
//!
//! Ship engineers introduce the next ones once the commander's rank with them passes grade 3
//! (about half way to grade 4); suit engineers refer the next one after a task of their own.

use crate::engineers::{key, EngineerBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UnlockInfo {
    pub name: String,
    /// `ship` or `suit`.
    pub kind: String,
    pub system: String,
    /// The system needs a permit.
    #[serde(default)]
    pub permit: bool,
    /// Engineers whose introduction (ship) or referral (suit) is needed; empty: common knowledge.
    pub introduced_by: Vec<String>,
    pub invite: String,
    /// Ship engineers: what unlocks the workshop once invited.
    #[serde(default)]
    pub unlock: Option<String>,
    /// Ship engineers: how to raise rank with them.
    #[serde(default)]
    pub reputation: Option<String>,
    /// Suit engineers: the task for the referral to the next engineer.
    #[serde(default)]
    pub referral: Option<String>,
}

#[derive(Deserialize)]
struct UnlocksFile {
    engineers: Vec<UnlockInfo>,
}

pub fn catalog() -> &'static [UnlockInfo] {
    static C: OnceLock<Vec<UnlockInfo>> = OnceLock::new();
    C.get_or_init(|| {
        serde_json::from_str::<UnlocksFile>(include_str!("../../../data/catalog/engineer-unlocks.json"))
            .expect("engineer-unlocks.json")
            .engineers
    })
}

/// Where the commander stands with an engineer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Unlocked,
    /// Invited (or met): the workshop still needs its unlock task.
    Invited,
    /// Known: the invitation is next.
    Known,
    /// Not known yet: needs an introduction or referral first.
    Locked,
    Barred,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Node {
    pub name: String,
    pub kind: String,
    pub system: String,
    pub permit: bool,
    /// Column in the tree: 0 for common knowledge, else one more than the deepest introducer.
    pub depth: u32,
    /// The edges: who introduces or refers this engineer.
    pub introduced_by: Vec<String>,
    pub status: Status,
    pub rank: Option<u8>,
    /// What is left to unlock this engineer; `None` once unlocked.
    pub left: Option<String>,
    /// Can be worked on now (known, invited, or every introducer unlocked).
    pub next: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

fn status_of(progress: Option<&str>, common: bool) -> Status {
    match progress.map(|p| p.to_ascii_lowercase()) {
        Some(p) if p == "unlocked" => Status::Unlocked,
        Some(p) if p == "invited" || p == "acquainted" => Status::Invited,
        Some(p) if p == "known" => Status::Known,
        Some(p) if p == "barred" => Status::Barred,
        // Common-knowledge engineers are known to everyone, logged or not.
        _ if common => Status::Known,
        _ => Status::Locked,
    }
}

/// The tree for the commander's progress (`EngineerProgress`, newest per engineer).
pub fn tree(progress: &EngineerBuilder) -> Tree {
    let cat = catalog();
    let by_key: HashMap<String, &UnlockInfo> = cat.iter().map(|e| (key(&e.name), e)).collect();
    let status: HashMap<String, (Status, Option<u8>)> = cat
        .iter()
        .map(|e| {
            let p = progress.progress(&e.name);
            (key(&e.name), (status_of(p.map(|x| x.0), e.introduced_by.is_empty()), p.and_then(|x| x.1)))
        })
        .collect();
    let unlocked = |n: &str| status.get(&key(n)).is_some_and(|s| s.0 == Status::Unlocked);

    fn depth(name: &str, by_key: &HashMap<String, &UnlockInfo>, seen: u32) -> u32 {
        let Some(e) = by_key.get(&key(name)) else { return 0 };
        if seen > 8 {
            return 0;
        }
        e.introduced_by.iter().map(|r| depth(r, by_key, seen + 1) + 1).max().unwrap_or(0)
    }

    let nodes = cat
        .iter()
        .map(|e| {
            let (st, rank) = status[&key(&e.name)];
            let ship = e.kind == "ship";
            let left = match st {
                Status::Unlocked => None,
                Status::Barred => Some("Barred from this workshop.".into()),
                Status::Invited if ship => {
                    Some(format!("Unlock at {}: {}.", e.system, e.unlock.as_deref().unwrap_or("see the workshop")))
                }
                Status::Invited => Some(format!("Visit them at {} to unlock.", e.system)),
                Status::Known => Some(format!("Invitation: {}.", e.invite)),
                Status::Locked => {
                    let parts: Vec<String> = e
                        .introduced_by
                        .iter()
                        .map(|r| {
                            if !unlocked(r) {
                                format!("Unlock {r} first.")
                            } else if ship {
                                format!("Introduction from {r}: raise your rank with them past grade 3.")
                            } else {
                                let task = by_key.get(&key(r)).and_then(|x| x.referral.as_deref());
                                format!("Referral from {r}: {}.", task.unwrap_or("their referral task"))
                            }
                        })
                        .collect();
                    Some(parts.join(" "))
                }
            };
            let next = match st {
                Status::Known | Status::Invited => true,
                Status::Locked => e.introduced_by.iter().all(|r| unlocked(r)),
                Status::Unlocked | Status::Barred => false,
            };
            Node {
                name: e.name.clone(),
                kind: e.kind.clone(),
                system: e.system.clone(),
                permit: e.permit,
                depth: depth(&e.name, &by_key, 0),
                introduced_by: e.introduced_by.clone(),
                status: st,
                rank,
                left,
                next,
            }
        })
        .collect();
    Tree { nodes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_ship_and_suit_engineer_has_unlock_data() {
        let ship: Vec<&str> = catalog().iter().filter(|e| e.kind == "ship").map(|e| e.name.as_str()).collect();
        let suit = catalog().iter().filter(|e| e.kind == "suit").count();
        assert_eq!((ship.len(), suit), (25, 13));
        // every ship engineer the trip planner knows has unlock data, and the reverse
        for e in crate::engineers::catalog() {
            assert!(ship.iter().any(|s| key(s) == key(&e.name)), "{} has no unlock data", e.name);
        }
        for e in catalog() {
            assert!(!e.invite.is_empty() && !e.system.is_empty(), "{}", e.name);
            if e.kind == "ship" {
                assert!(e.unlock.is_some(), "{}", e.name);
            }
        }
    }

    #[test]
    fn tree_marks_progress_and_what_is_left() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/unlocks");
        let t = tree(&crate::engineers::builder_from_journal_dir(&dir));
        let n = |name: &str| t.nodes.iter().find(|x| x.name == name).unwrap_or_else(|| panic!("{name}"));
        let row = |name: &str| {
            let x = n(name);
            (x.status, x.left.clone(), x.next)
        };
        assert_eq!(row("Felicity Farseer"), (Status::Unlocked, None, false));
        assert_eq!(n("Felicity Farseer").rank, Some(5));
        // the newer single-engineer event wins: Liz Ryder went from invited to unlocked
        assert_eq!(row("Liz Ryder"), (Status::Unlocked, None, false));
        assert_eq!(
            row("Marco Qwent"),
            (Status::Known, Some("Invitation: Ally reputation with Sirius Corporation.".into()), true)
        );
        assert_eq!(row("Kit Fowler"), (Status::Invited, Some("Visit them at Capoya to unlock.".into()), true));
        // locked, introducer unlocked: the introduction is what's left, and it can be done now
        assert_eq!(
            row("Hera Tani"),
            (Status::Locked, Some("Introduction from Liz Ryder: raise your rank with them past grade 3.".into()), true)
        );
        assert_eq!(
            row("Terra Velasquez"),
            (Status::Locked, Some("Referral from Jude Navarro: Provide 5 Genetic Repair Meds.".into()), true)
        );
        // locked behind an engineer who isn't unlocked yet
        assert_eq!(row("Lori Jameson"), (Status::Locked, Some("Unlock Marco Qwent first.".into()), false));
        assert_eq!(row("Yarden Bond"), (Status::Locked, Some("Unlock Kit Fowler first.".into()), false));
        // never logged but common knowledge: known, invitation next
        assert_eq!(
            row("Tod \"The Blaster\" McQuinn"),
            (Status::Known, Some("Invitation: Earn 15 bounty vouchers.".into()), true)
        );
        assert_eq!(row("Hero Ferrari").0, Status::Known);
        assert_eq!(row("Wellington Beck"), (Status::Locked, Some("Unlock Hero Ferrari first.".into()), false));
        // Yi Shen needs all three Colonia referrals
        let yi = n("Yi Shen");
        assert_eq!(
            yi.left.as_deref(),
            Some("Unlock Baltanos first. Unlock Eleanor Bresa first. Unlock Rosa Dayette first.")
        );
        // the tree's columns follow the introductions
        assert_eq!((n("Elvira Martuuk").depth, n("Marco Qwent").depth, n("Lori Jameson").depth), (0, 1, 2));
        assert_eq!(n("Lori Jameson").introduced_by, ["Marco Qwent"]);
        assert_eq!(n("Yi Shen").depth, 1);
    }
}
