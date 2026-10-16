//! On-foot micro-resources the commander holds: `ShipLocker.json` plus `Backpack.json` (the game
//! rewrites both in the journal folder whenever they change). Mission items (with a `MissionID`)
//! are shown but never counted towards a blueprint: they go back when the mission ends.

use crate::catalog::{catalog, Materials};
use astromech_journal::companion::{parse_fresh, CompanionFile, CompanionKind, InventorySnap};
use astromech_journal::MicroResource;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

/// What the two companion files say, by item symbol.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Holdings {
    pub locker: Materials,
    pub backpack: Materials,
    /// Mission items, in either place.
    pub mission: Materials,
    /// Symbols the catalogue does not know: the file's display name and category.
    pub unknown: BTreeMap<String, (String, String)>,
    pub locker_ts: Option<String>,
    pub backpack_ts: Option<String>,
}

impl Holdings {
    pub fn from_snaps(locker: Option<&InventorySnap>, backpack: Option<&InventorySnap>) -> Self {
        let mut h = Holdings {
            locker_ts: locker.map(|s| s.timestamp.clone()),
            backpack_ts: backpack.map(|s| s.timestamp.clone()),
            ..Default::default()
        };
        for (snap, into_backpack) in [(locker, false), (backpack, true)] {
            for m in snap.map(|s| s.items.as_slice()).unwrap_or_default() {
                h.add(m, into_backpack);
            }
        }
        h
    }

    fn add(&mut self, m: &MicroResource, into_backpack: bool) {
        let id = m.name.to_ascii_lowercase();
        if !catalog().items.contains_key(&id) {
            let name = m.name_localised.clone().unwrap_or_else(|| m.name.clone());
            self.unknown.insert(id.clone(), (name, m.category.to_ascii_lowercase()));
        }
        let bucket = if m.mission_id.is_some() {
            &mut self.mission
        } else if into_backpack {
            &mut self.backpack
        } else {
            &mut self.locker
        };
        *bucket.entry(id).or_default() += m.count;
    }

    /// Usable count of `id`: locker plus backpack, not mission items.
    pub fn have(&self, id: &str) -> i64 {
        self.locker.get(id).copied().unwrap_or(0) + self.backpack.get(id).copied().unwrap_or(0)
    }

    /// Read both files from the journal folder. A file that is missing or caught half-written
    /// counts as empty; the next poll reads it again.
    pub fn from_journal_dir(dir: &Path) -> Self {
        let read = |kind: CompanionKind| {
            let bytes = std::fs::read(dir.join(kind.file_name())).ok()?;
            // No event to compare with: any readable file is the latest the game wrote.
            match parse_fresh(kind, Some(&bytes), "") {
                Ok(CompanionFile::ShipLocker(s) | CompanionFile::Backpack(s)) => Some(s),
                _ => None,
            }
        };
        Self::from_snaps(read(CompanionKind::ShipLocker).as_ref(), read(CompanionKind::Backpack).as_ref())
    }
}

/// One micro-resource on the inventory page.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ItemLine {
    pub id: String,
    pub name: String,
    /// `goods`, `assets`, `data` or `consumables` (the in-game tab names).
    pub category: &'static str,
    pub locker: i64,
    pub backpack: i64,
    pub mission: i64,
    /// Bartender barter value (assets only).
    pub barter: Option<i64>,
    /// Credits a bartender pays for one.
    pub value: Option<i64>,
}

pub fn category_title(fdev: &str) -> &'static str {
    match fdev {
        "item" | "items" => "goods",
        "component" | "components" => "assets",
        "data" => "data",
        _ => "consumables",
    }
}

/// Every catalogue item, plus anything held the catalogue does not know, sorted by name.
pub fn item_lines(h: &Holdings) -> Vec<ItemLine> {
    let c = catalog();
    let get = |m: &Materials, id: &str| m.get(id).copied().unwrap_or(0);
    let mut out: Vec<ItemLine> = c
        .items
        .keys()
        .chain(h.unknown.keys())
        .map(|id| {
            let item = c.items.get(id);
            let (name, category) = match (item, h.unknown.get(id)) {
                (Some(i), _) => (i.name.clone(), i.category.as_str()),
                (None, Some((n, cat))) => (n.clone(), cat.as_str()),
                (None, None) => (id.clone(), ""),
            };
            ItemLine {
                id: id.clone(),
                name,
                category: category_title(category),
                locker: get(&h.locker, id),
                backpack: get(&h.backpack, id),
                mission: get(&h.mission, id),
                barter: item.and_then(|i| i.barter),
                value: item.and_then(|i| i.value),
            }
        })
        .collect();
    out.sort_by_key(|a| a.name.to_lowercase());
    out
}
