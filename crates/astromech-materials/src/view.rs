//! Trader-style grids for the app window.

use crate::catalog::{self, Kind, Spec};
use crate::ledger::Ledger;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Inventory {
    pub source: String,
    pub snapshot_ts: Option<String>,
    pub kinds: Vec<KindView>,
    pub other: Vec<Cell>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KindView {
    pub id: String,
    pub title: String,
    pub trader: String,
    pub grades: Vec<u8>,
    pub groups: Vec<GroupView>,
    pub held: i64,
    pub capacity: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GroupView {
    pub name: String,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Cell {
    pub id: String,
    pub name: String,
    pub kind: Kind,
    pub group: String,
    pub grade: u8,
    pub rarity: String,
    pub count: i64,
    pub capacity: i64,
    pub trader: bool,
}

impl Cell {
    fn from_spec(ledger: &Ledger, s: &Spec) -> Self {
        Cell {
            id: s.id.to_string(),
            name: ledger.display_name(s),
            kind: s.kind,
            group: s.group.to_string(),
            grade: s.grade,
            rarity: catalog::rarity(s.grade).to_string(),
            count: ledger.count(s.id),
            capacity: catalog::capacity(s.grade),
            trader: s.on_trader,
        }
    }
}

pub fn trader_view(ledger: &Ledger, source: impl Into<String>) -> Inventory {
    let kinds =
        [Kind::Raw, Kind::Manufactured, Kind::Encoded].into_iter().map(|kind| kind_view(ledger, kind)).collect();
    let mut other: Vec<Cell> = catalog::catalog()
        .iter()
        .filter(|s| !s.on_trader)
        .map(|s| Cell::from_spec(ledger, s))
        .filter(|c| c.count > 0)
        .collect();
    for (id, name, n) in ledger.unknown() {
        other.push(Cell {
            id: id.clone(),
            name,
            kind: Kind::Manufactured,
            group: "Unknown".into(),
            grade: 1,
            rarity: catalog::rarity(1).into(),
            count: n,
            capacity: catalog::capacity(1),
            trader: false,
        });
    }
    other.sort_by(|a, b| a.group.cmp(&b.group).then_with(|| a.name.cmp(&b.name)));
    Inventory { source: source.into(), snapshot_ts: ledger.snapshot_ts.clone(), kinds, other }
}

fn kind_view(ledger: &Ledger, kind: Kind) -> KindView {
    let max_g = kind.max_grade();
    let grades: Vec<u8> = (1..=max_g).collect();
    let groups = catalog::groups(kind)
        .into_iter()
        .map(|g| GroupView {
            name: g.to_string(),
            cells: grades
                .iter()
                .filter_map(|&grade| {
                    catalog::catalog()
                        .iter()
                        .find(|s| s.kind == kind && s.group == g && s.grade == grade && s.on_trader)
                })
                .map(|s| Cell::from_spec(ledger, s))
                .collect(),
        })
        .collect();
    let (held, capacity) = ledger.kind_totals(kind);
    KindView {
        id: kind.as_str().into(),
        title: kind.title().into(),
        trader: kind.trader().into(),
        grades,
        groups,
        held,
        capacity,
    }
}

pub fn from_journal_dir(dir: &Path) -> Inventory {
    let ledger = crate::ledger::replay_journal_dir(dir);
    let source = match &ledger.snapshot_ts {
        Some(ts) => format!("journal snapshot {ts}"),
        None => format!("no Materials event in {}", dir.display()),
    };
    trader_view(&ledger, source)
}
