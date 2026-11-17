//! The engineer unlock guide as a tech tree: every ship and suit engineer
//! (`data/catalog/engineer-unlocks.json`, from the Elite Dangerous wiki), the engineers whose
//! introduction or referral they need (the edges), and the commander's progress from the
//! journal's `EngineerProgress`. Each engineer not yet unlocked names the requirement left, and
//! the ones that can be worked on now are marked `next`.
//!
//! Each engineer also carries what they work on: a ship engineer's module types with the best
//! grade they offer (as the search works it out), a suit engineer's modifications.
//!
//! Ship engineers introduce the next ones once the commander's rank with them passes grade 3
//! (about half way to grade 4); suit engineers refer the next one after a task of their own.

use crate::data;
use crate::engineers::{best_grade, key, EngineerBuilder};
use crate::search::{type_label, type_order, SuitOffer};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ts_rs::TS)]
#[ts(rename = "EngineerStatus")]
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

/// Something an engineer works on: a ship module type with the best grade they offer on it, or
/// a suit modification.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct WorksOn {
    /// The module type's short label ("Pulse", "FSD"), or the modification's name.
    pub label: String,
    /// Ship engineers: the best grade they offer on it. Suit engineers: 0 (no grades).
    pub grade: u8,
    /// Suit engineers: the wiki disagrees with itself on whether they fit this modification.
    pub unsure: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
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
    /// What can be done next toward unlocking this engineer; `None` once unlocked.
    pub left: Option<String>,
    /// Every step from here to the workshop, in order, starting with what `left` says; empty
    /// once unlocked.
    pub steps: Vec<String>,
    /// Can be worked on now (known, invited, or every introducer unlocked).
    pub next: bool,
    /// Ship engineers: each module type they offer, best grade first, then in the search table's
    /// order. Suit engineers: their modifications, in `suits`' order.
    pub works_on: Vec<WorksOn>,
}

#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
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

/// What the ship engineer `name` works on: every module type they offer, with the best grade.
fn ship_works_on(name: &str) -> Vec<WorksOn> {
    let mut out: Vec<((usize, usize), WorksOn)> = data::catalog()
        .groups
        .keys()
        .filter_map(|id| {
            let grade = best_grade(name, id);
            (grade > 0).then(|| (type_order(id), WorksOn { label: type_label(id), grade, unsure: false }))
        })
        .collect();
    out.sort_by(|(oa, a), (ob, b)| b.grade.cmp(&a.grade).then(oa.cmp(ob)).then_with(|| a.label.cmp(&b.label)));
    out.into_iter().map(|(_, w)| w).collect()
}

/// What the suit engineer `name` works on, from `suits` (the Odyssey data's modifications).
fn suit_works_on(name: &str, suits: &[SuitOffer]) -> Vec<WorksOn> {
    let k = key(name);
    suits.iter().find(|s| key(s.engineer) == k).map_or_else(Vec::new, |s| {
        s.mods.iter().map(|m| WorksOn { label: (*m).to_string(), grade: 0, unsure: s.unsure.contains(m) }).collect()
    })
}

/// The tree for the commander's progress (`EngineerProgress`, newest per engineer), with the
/// suit engineers' modifications (`suits`, from the Odyssey data).
pub fn tree(progress: &EngineerBuilder, suits: &[SuitOffer]) -> Tree {
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
            // The introductions (ship) or referrals (suit) this engineer needs.
            let intro = |r: &String| {
                if ship {
                    format!("Introduction from {r}: raise your rank with them past grade 3.")
                } else {
                    let task = by_key.get(&key(r)).and_then(|x| x.referral.as_deref());
                    format!("Referral from {r}: {}.", task.unwrap_or("their referral task"))
                }
            };
            let invite = format!("Invitation: {}.", e.invite);
            let unlock = if ship {
                format!("Unlock at {}: {}.", e.system, e.unlock.as_deref().unwrap_or("see the workshop"))
            } else {
                format!("Visit them at {} to unlock.", e.system)
            };
            // What can be done next, in words.
            let left = match st {
                Status::Unlocked => None,
                Status::Barred => Some("Barred from this workshop.".to_string()),
                Status::Invited => Some(unlock.clone()),
                Status::Known => Some(invite.clone()),
                Status::Locked => Some(
                    e.introduced_by
                        .iter()
                        .map(|r| if unlocked(r) { intro(r) } else { format!("Unlock {r} first.") })
                        .collect::<Vec<_>>()
                        .join(" "),
                ),
            };
            // Every step from here to the workshop, in order.
            let steps: Vec<String> = match st {
                Status::Unlocked => vec![],
                Status::Barred => vec!["Barred from this workshop.".into()],
                Status::Invited => vec![unlock],
                Status::Known => vec![invite, unlock],
                Status::Locked => {
                    let first = e.introduced_by.iter().filter(|r| !unlocked(r)).map(|r| format!("Unlock {r} first."));
                    first.chain(e.introduced_by.iter().map(intro)).chain([invite, unlock]).collect()
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
                steps,
                next,
                works_on: if ship { ship_works_on(&e.name) } else { suit_works_on(&e.name, suits) },
            }
        })
        .collect();
    Tree { nodes }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
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
        let t = tree(&crate::engineers::builder_from_journal_dir(&dir), &[]);
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
        // every step from here to the workshop, starting with what `left` says
        assert_eq!(
            n("Hera Tani").steps,
            [
                "Introduction from Liz Ryder: raise your rank with them past grade 3.",
                "Invitation: Imperial Navy rank of Outsider or higher.",
                "Unlock at Kuwemaki: Provide 50 Kamitra Cigars.",
            ]
        );
        assert_eq!(
            n("Lori Jameson").steps[..2],
            ["Unlock Marco Qwent first.", "Introduction from Marco Qwent: raise your rank with them past grade 3."]
        );
        assert_eq!(n("Kit Fowler").steps, ["Visit them at Capoya to unlock."]);
        assert!(n("Felicity Farseer").steps.is_empty());
    }

    #[test]
    fn with_no_progress_the_common_knowledge_engineers_are_known_and_the_rest_not_introduced() {
        let t = tree(&EngineerBuilder::default(), &[]);
        for n in &t.nodes {
            let want = if n.introduced_by.is_empty() { Status::Known } else { Status::Locked };
            assert_eq!((n.status, n.next), (want, want == Status::Known), "{}", n.name);
        }
    }

    /// (grade, label) of what `name` works on, in order.
    fn works(t: &Tree, name: &str) -> Vec<(u8, String)> {
        let n = t.nodes.iter().find(|x| x.name == name).unwrap_or_else(|| panic!("{name}"));
        n.works_on.iter().map(|w| (w.grade, w.label.clone())).collect()
    }

    #[test]
    fn a_ship_engineer_works_on_each_module_type_with_the_best_grade() {
        let t = tree(&EngineerBuilder::default(), &[]);
        let w = |g: u8, l: &str| (g, l.to_string());
        // Best grade first, then in the search table's order; labels from the table, a type that
        // is one of several in an entry named after the entry's first ("Advanced multi-cannon").
        let cases: &[(&str, Vec<(u8, String)>)] = &[
            ("The Dweller", vec![w(5, "Distributor"), w(4, "Pulse"), w(3, "Burst"), w(3, "Beam")]),
            ("Selene Jean", vec![w(5, "Armour"), w(5, "Hull reinforcement")]),
            (
                "Tod \"The Blaster\" McQuinn",
                vec![
                    w(5, "Multi-cannon"),
                    w(5, "Advanced multi-cannon"),
                    w(5, "Rail gun"),
                    w(3, "Fragment cannon"),
                    w(2, "Cannon"),
                ],
            ),
            (
                "Didi Vatermann",
                vec![w(5, "Booster"), w(3, "Shield"), w(3, "Bi-weave shield"), w(3, "Prismatic shield")],
            ),
        ];
        for (name, want) in cases {
            assert_eq!(&works(&t, name), want, "{name}");
        }
        // the grades are the search's: Martuuk reaches FSD grade 5 there too
        assert!(works(&t, "Elvira Martuuk").contains(&w(5, "FSD")));
    }

    #[test]
    fn every_offered_module_type_has_a_label_of_its_own() {
        let mut labels: HashMap<String, String> = HashMap::new();
        for id in data::catalog().groups.keys() {
            let l = type_label(id);
            assert!(!l.is_empty(), "{id}");
            if let Some(other) = labels.insert(l.clone(), id.clone()) {
                panic!("{id} and {other} are both {l}");
            }
        }
        // every ship engineer works on something
        let t = tree(&EngineerBuilder::default(), &[]);
        for n in t.nodes.iter().filter(|n| n.kind == "ship") {
            assert!(!n.works_on.is_empty(), "{}", n.name);
        }
    }

    #[test]
    fn a_suit_engineer_works_on_their_modifications() {
        let suits = [
            SuitOffer {
                engineer: "Hero Ferrari",
                mods: vec!["Faster Handling", "Increased Sprint Duration"],
                unsure: vec!["Increased Sprint Duration"],
            },
            SuitOffer { engineer: "Domino Green", mods: vec!["Greater Range"], unsure: vec![] },
        ];
        let t = tree(&EngineerBuilder::default(), &suits);
        let cases: &[(&str, Vec<(&str, bool)>)] = &[
            ("Hero Ferrari", vec![("Faster Handling", false), ("Increased Sprint Duration", true)]),
            ("Domino Green", vec![("Greater Range", false)]),
            // not in the Odyssey data: nothing listed
            ("Kit Fowler", vec![]),
        ];
        for (name, want) in cases {
            let n = t.nodes.iter().find(|x| x.name == *name).unwrap_or_else(|| panic!("{name}"));
            let got: Vec<(&str, bool)> = n.works_on.iter().map(|w| (w.label.as_str(), w.unsure)).collect();
            assert_eq!(&got, want, "{name}");
            assert!(n.works_on.iter().all(|w| w.grade == 0), "{name}");
        }
    }
}
