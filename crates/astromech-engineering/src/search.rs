//! "Where do I engineer my lasers?": the Engineers page's search by module type
//! (astromech-7did). The commander's word ("laser", "fsd", "thrusters") is looked up in
//! `data/catalog/engineer-search.json`, a small table of words to the module types (groups) of
//! `engineering.json`; each engineer who offers one of them is listed with the best grade per
//! type ([`crate::engineers::offers`]), the commander's standing with them (the unlock tree), the
//! grade their rank allows today, where they are and how far.
//!
//! Suit engineers have no grades: a word that names no ship module is matched against the
//! modifications they fit ("range", "magazine", "night vision"). A suit or on-foot weapon word
//! ("pistol", "Maverick") that matches nothing says so, since suit engineers are searched by
//! modification.
//!
//! The main answer is the best grade the commander can get today: among unlocked engineers, the
//! highest grade their rank allows, nearest first. With none unlocked it is the engineer nearest
//! to unlocking (invited, then known, then one whose introducers are all unlocked), nearest first.

use crate::data;
use crate::engineers::{self, key, offers};
use crate::unlocks::{Node, Status, Tree};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct Entry {
    words: Vec<String>,
    says: String,
    /// `[group id, short label for the grade line]` ("pl", "Pulse"), in the order shown.
    types: Vec<[String; 2]>,
}

#[derive(Debug, Deserialize)]
struct Table {
    entries: Vec<Entry>,
    suit_words: Vec<String>,
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        serde_json::from_str(include_str!("../../../data/catalog/engineer-search.json")).expect("engineer-search.json")
    })
}

/// A module type the words matched, as the grade line names it.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(rename = "EngineerSearchType")]
pub struct ModuleType {
    /// The group id in `engineering.json` ("pl").
    pub id: String,
    /// Short label for the grade line ("Pulse").
    pub label: String,
}

/// The best grade an engineer offers on one matched module type.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct TypeGrade {
    pub id: String,
    pub label: String,
    pub grade: u8,
}

/// One engineer who offers a matched module (ship) or modification (suit).
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct SearchHit {
    pub name: String,
    /// `ship` or `suit`.
    pub kind: String,
    pub system: String,
    pub permit: bool,
    /// Light years from the commander's current system; `None` when either is not known.
    pub distance_ly: Option<f64>,
    pub status: Status,
    /// Rank with this engineer (ship engineers, once unlocked).
    pub rank: Option<u8>,
    /// What is left to unlock them; `None` once unlocked.
    pub left: Option<String>,
    /// Can be worked on now (see [`Node::next`]).
    pub next: bool,
    /// Ship engineers: the best grade per matched type, in the table's order; types they don't
    /// offer are left out.
    pub grades: Vec<TypeGrade>,
    /// The best grade over `grades` (0 for suit engineers).
    pub best: u8,
    /// The grade the commander can get from them today: `best` capped by their rank once
    /// unlocked, else 0. A suit engineer, unlocked, is 1 (they fit the modification).
    pub today: u8,
    /// Suit engineers: the matched modifications they fit.
    pub mods: Vec<String>,
}

/// Why the main answer was chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
pub enum MainKind {
    /// The best grade the commander can get today.
    Today,
    /// No matched engineer is unlocked: the one nearest to unlocking.
    Unlock,
}

#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct EngineerSearch {
    /// The words as typed.
    pub query: String,
    /// What the words matched, as the page names it ("Pulse, burst and beam lasers",
    /// "Greater Range"); `None` when nothing matched.
    pub says: Option<String>,
    /// `ship` or `suit` when something matched.
    pub kind: Option<String>,
    /// The ship module types matched, in the table's order.
    pub types: Vec<ModuleType>,
    /// The suit modifications matched.
    pub mods: Vec<String>,
    /// The main answer first, then the rest: the grade available today (highest first), the
    /// best grade offered, how close to unlocking, then distance (nearest first).
    pub hits: Vec<SearchHit>,
    /// Why `hits[0]` is the main answer; `None` when there is none (no hits, or all barred).
    pub main: Option<MainKind>,
    /// Words the table knows that look like the query, when nothing matched.
    pub near: Vec<String>,
    /// The query is an on-foot weapon or suit word that matched nothing: suit engineers are
    /// searched by modification.
    pub suit_word: bool,
    /// The system distances are measured from.
    pub here: Option<String>,
}

/// A suit engineer's modifications, by name ("Greater Range").
pub struct SuitOffer<'a> {
    pub engineer: &'a str,
    pub mods: Vec<&'a str>,
}

/// Lowercase, single spaces, no surrounding space.
fn norm(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// The table entry for the words, also with a trailing "s" dropped; then a module type named in
/// full by the catalog ("fragment cannon"). Returns what it says and the types.
fn ship_match(q: &str) -> Option<(String, Vec<ModuleType>)> {
    let find = |w: &str| table().entries.iter().find(|e| e.words.iter().any(|x| x == w));
    let e = find(q).or_else(|| q.strip_suffix('s').and_then(find));
    if let Some(e) = e {
        let types = e.types.iter().map(|[id, label]| ModuleType { id: id.clone(), label: label.clone() }).collect();
        return Some((e.says.clone(), types));
    }
    let singular = q.strip_suffix('s').unwrap_or(q);
    data::catalog()
        .groups
        .iter()
        .find(|(_, g)| {
            let n = g.name.to_lowercase();
            n == q || n == singular
        })
        .map(|(id, g)| (g.name.clone(), vec![ModuleType { id: id.clone(), label: g.name.clone() }]))
}

/// Whether every word of `q` starts a word of `name`, in order ("night vision", "range",
/// "mag" → "Magazine Size").
fn words_start(q: &str, name: &str) -> bool {
    let n = norm(name);
    let mut words = n.split(' ');
    q.split(' ').all(|w| words.any(|x| x.starts_with(w)))
}

/// Edit distance, for the near matches.
fn edits(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut prev = row[0];
        row[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cur = row[j + 1];
            row[j + 1] = (prev + usize::from(ca != *cb)).min(row[j] + 1).min(cur + 1);
            prev = cur;
        }
    }
    row[b.len()]
}

/// Up to five words the table knows, or modification names, that look like `q`: one starts
/// with the other, or a few letters apart.
fn near(q: &str, suits: &[SuitOffer]) -> Vec<String> {
    let mut words: Vec<String> = table().entries.iter().filter_map(|e| e.words.first().cloned()).collect();
    for s in suits {
        for m in &s.mods {
            let m = norm(m);
            if !words.contains(&m) {
                words.push(m);
            }
        }
    }
    let close = (q.chars().count() / 3).clamp(1, 2);
    let mut scored: Vec<(usize, String)> = words
        .into_iter()
        .filter_map(|w| {
            let all: Vec<&String> = table()
                .entries
                .iter()
                .find(|e| e.words.first() == Some(&w))
                .map_or(vec![], |e| e.words.iter().collect());
            let d = std::iter::once(&w)
                .chain(all)
                .map(|x| if x.starts_with(q) || (q.len() >= 3 && x.contains(q)) { 0 } else { edits(q, x) })
                .min()?;
            (d <= close).then_some((d, w))
        })
        .collect();
    scored.sort();
    scored.into_iter().take(5).map(|(_, w)| w).collect()
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// How close to unlocking: lower is closer.
fn unlock_order(h: &SearchHit) -> u8 {
    match h.status {
        Status::Unlocked => 0,
        Status::Invited => 1,
        Status::Known => 2,
        Status::Locked if h.next => 3,
        Status::Locked => 4,
        Status::Barred => 5,
    }
}

fn nearer(a: &SearchHit, b: &SearchHit) -> Ordering {
    match (a.distance_ly, b.distance_ly) {
        (Some(x), Some(y)) => x.total_cmp(&y),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
    .then_with(|| a.name.cmp(&b.name))
}

fn hit(n: &Node, here: Option<[f64; 3]>) -> SearchHit {
    let coords = engineers::info(&n.name).map(|e| e.coords);
    SearchHit {
        name: n.name.clone(),
        kind: n.kind.clone(),
        system: n.system.clone(),
        permit: n.permit,
        distance_ly: here.zip(coords).map(|(h, c)| (distance(h, c) * 100.0).round() / 100.0),
        status: n.status,
        rank: n.rank,
        left: n.left.clone(),
        next: n.next,
        grades: vec![],
        best: 0,
        today: 0,
        mods: vec![],
    }
}

/// The search for `query`, against the commander's standing (`tree`), from `here` (the current
/// system's name and position), with the suit engineers' modifications.
pub fn search(query: &str, tree: &Tree, here: Option<(&str, [f64; 3])>, suits: &[SuitOffer]) -> EngineerSearch {
    let q = norm(query);
    let pos = here.map(|h| h.1);
    let mut out = EngineerSearch {
        query: query.to_string(),
        says: None,
        kind: None,
        types: vec![],
        mods: vec![],
        hits: vec![],
        main: None,
        near: vec![],
        suit_word: false,
        here: here.map(|h| h.0.to_string()),
    };
    if q.is_empty() {
        return out;
    }
    let ship_nodes = tree.nodes.iter().filter(|n| n.kind == "ship");
    if let Some((says, types)) = ship_match(&q) {
        // Every engineer the catalog names on a matched type, with their best grade per type.
        let mut per: BTreeMap<String, Vec<TypeGrade>> = BTreeMap::new();
        for t in &types {
            let Some(g) = data::group(&t.id) else { continue };
            for e in g.all_engineers() {
                let grade = g.blueprints.iter().map(|b| offers(e, b, &t.id)).max().unwrap_or(0);
                if grade > 0 {
                    per.entry(key(e)).or_default().push(TypeGrade { id: t.id.clone(), label: t.label.clone(), grade });
                }
            }
        }
        for n in ship_nodes {
            let Some(grades) = per.remove(&key(&n.name)) else { continue };
            let mut h = hit(n, pos);
            h.best = grades.iter().map(|g| g.grade).max().unwrap_or(0);
            if n.status == Status::Unlocked {
                h.today = h.best.min(n.rank.unwrap_or(1).max(1));
            }
            h.grades = grades;
            out.hits.push(h);
        }
        out.says = Some(says);
        out.kind = Some("ship".into());
        out.types = types;
    } else {
        let mut mods: Vec<String> = vec![];
        for s in suits {
            let fits: Vec<&str> = s.mods.iter().copied().filter(|m| words_start(&q, m)).collect();
            if fits.is_empty() {
                continue;
            }
            for m in &fits {
                if !mods.iter().any(|x| x == m) {
                    mods.push((*m).to_string());
                }
            }
            if let Some(n) = tree.nodes.iter().find(|n| n.kind == "suit" && key(&n.name) == key(s.engineer)) {
                let mut h = hit(n, pos);
                if n.status == Status::Unlocked {
                    h.today = 1;
                }
                h.mods = fits.iter().map(|m| (*m).to_string()).collect();
                out.hits.push(h);
            }
        }
        if !mods.is_empty() {
            mods.sort();
            out.says = Some(mods.join(", "));
            out.kind = Some("suit".into());
            out.mods = mods;
        }
    }
    if out.kind.is_none() {
        out.near = near(&q, suits);
        out.suit_word = q.split(' ').any(|w| table().suit_words.iter().any(|s| s == w));
        return out;
    }
    out.hits.sort_by(|a, b| {
        b.today
            .cmp(&a.today)
            .then_with(|| b.best.cmp(&a.best))
            .then_with(|| unlock_order(a).cmp(&unlock_order(b)))
            .then_with(|| nearer(a, b))
    });
    out.main = match out.hits.first() {
        Some(h) if h.today > 0 => Some(MainKind::Today),
        Some(h) if h.status != Status::Barred => Some(MainKind::Unlock),
        _ => None,
    };
    if out.main == Some(MainKind::Unlock) {
        // With none unlocked, the main answer is the nearest to unlock, whatever grade it reaches.
        let i = (0..out.hits.len())
            .min_by(|&a, &b| {
                let (x, y) = (&out.hits[a], &out.hits[b]);
                unlock_order(x).cmp(&unlock_order(y)).then_with(|| nearer(x, y))
            })
            .unwrap_or(0);
        let h = out.hits.remove(i);
        out.hits.insert(0, h);
    }
    out
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;

    /// Every module type in the table is in the catalog, and every engineer the catalog names
    /// on one resolves to a located engineer and an unlock node: the search joins them by name.
    #[test]
    fn the_table_and_the_catalogs_agree() {
        let unlocks = crate::unlocks::catalog();
        for e in &table().entries {
            assert!(!e.types.is_empty(), "{:?}", e.words);
            for [id, _] in &e.types {
                let g = data::group(id).unwrap_or_else(|| panic!("{id} is not a module type"));
                for name in g.all_engineers() {
                    assert!(engineers::info(name).is_some(), "{name} ({id}) has no location");
                    assert!(unlocks.iter().any(|u| key(&u.name) == key(name)), "{name} ({id}) has no unlock data");
                }
            }
        }
        let mut words: Vec<&String> = table().entries.iter().flat_map(|e| &e.words).collect();
        let n = words.len();
        words.sort();
        words.dedup();
        assert_eq!(words.len(), n, "a word maps to two entries");
    }

    fn tree() -> Tree {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/unlocks");
        crate::unlocks::tree(&crate::engineers::builder_from_journal_dir(&dir))
    }

    /// Shinrarta Dezhra.
    const HERE: (&str, [f64; 3]) = ("Shinrarta Dezhra", [55.71875, 17.59375, 27.15625]);

    fn row(s: &EngineerSearch) -> Vec<(String, u8, u8)> {
        s.hits.iter().map(|h| (h.name.clone(), h.best, h.today)).collect()
    }

    #[test]
    fn a_module_word_ranks_what_the_commander_can_get_today() {
        let t = tree();
        // tests/unlocks: Farseer unlocked at rank 5, Martuuk at rank 2, Ryder at 1.
        let s = search(" FSD ", &t, Some(HERE), &[]);
        assert_eq!(s.says.as_deref(), Some("Frame shift drives"));
        assert_eq!(s.main, Some(MainKind::Today));
        let r = row(&s);
        // Farseer gives grade 5 today; Martuuk reaches 5 but rank 2 allows 2.
        assert_eq!(r[0], ("Felicity Farseer".into(), 5, 5));
        assert_eq!(r[1], ("Elvira Martuuk".into(), 5, 2));
        assert!(r[2..].iter().all(|h| h.2 == 0), "{r:?}");
        assert!(s.hits[0].distance_ly.is_some_and(|d| d > 0.0));
        assert_eq!(s.hits[0].grades, vec![TypeGrade { id: "fsd".into(), label: "FSD".into(), grade: 5 }]);
        // plural, mixed case and spacing still match
        assert_eq!(search("Lasers", &t, None, &[]).types.len(), 3);
    }

    #[test]
    fn with_none_unlocked_the_main_answer_is_the_nearest_to_unlock() {
        let t = tree();
        let s = search("laser", &t, Some(HERE), &[]);
        assert_eq!(s.main, Some(MainKind::Unlock));
        assert!(s.hits.iter().all(|h| h.today == 0));
        // The Dweller is common knowledge (known); Broo Tarquin and Mel Brandon need introductions.
        assert_eq!(s.hits[0].name, "The Dweller");
        assert_eq!(s.hits[0].status, Status::Known);
        let dweller = &s.hits[0].grades;
        assert_eq!(
            dweller.iter().map(|g| (g.label.as_str(), g.grade)).collect::<Vec<_>>(),
            [("Pulse", 4), ("Burst", 3), ("Beam", 3)]
        );
    }

    #[test]
    fn suit_engineers_match_by_modification() {
        let t = tree();
        let suits = [
            SuitOffer { engineer: "Domino Green", mods: vec!["Greater Range", "Stability"] },
            SuitOffer { engineer: "Oden Geiger", mods: vec!["Night Vision"] },
        ];
        let s = search("range", &t, Some(HERE), &suits);
        assert_eq!((s.kind.as_deref(), s.mods.clone()), (Some("suit"), vec!["Greater Range".to_string()]));
        assert_eq!(s.hits.iter().map(|h| h.name.as_str()).collect::<Vec<_>>(), ["Domino Green"]);
        assert_eq!(search("night vision", &t, None, &suits).hits[0].name, "Oden Geiger");
        // a weapon word that is no modification: nothing, and the page says why
        let p = search("pistol", &t, None, &suits);
        assert!(p.hits.is_empty() && p.suit_word && p.kind.is_none());
    }

    #[test]
    fn an_unknown_word_lists_near_matches() {
        let s = search("lazer", &tree(), None, &[]);
        assert!(s.kind.is_none() && s.hits.is_empty() && s.main.is_none());
        assert!(s.near.contains(&"laser".to_string()), "{:?}", s.near);
        assert!(!s.suit_word);
        assert!(search("", &tree(), None, &[]).near.is_empty());
    }
}
