//! The suits and weapons the commander owns, with their grade and fitted modifications, from the
//! journal: `BuySuit`, `BuyWeapon`, `UpgradeSuit`, `UpgradeWeapon`, `SellSuit`, `SellWeapon` and
//! `SuitLoadout` (event shapes from Frontier's Journal manual and the EDCD journal docs). Only
//! the suits and weapons in the catalogue are kept (not the Flight Suit). Only `SuitLoadout` lists fitted modifications, so a weapon the commander owns but has
//! in no loadout shows no mods.

use crate::catalog::catalog;
use serde::Serialize;
use serde_json::Value;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

/// One owned suit or weapon.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OwnedGear {
    /// Catalogue suit or weapon id (`maverick`, `karmaar50`).
    pub gear: String,
    pub name: String,
    /// `suit` or `weapon`.
    pub kind: &'static str,
    pub maker: Option<String>,
    /// 1 to 5.
    pub grade: u8,
    /// Catalogue modification ids fitted to it.
    pub mods: Vec<String>,
}

/// The owned gear by the game's id: `SuitID` for suits, `SuitModuleID` for weapons.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Owned {
    suits: BTreeMap<i64, OwnedGear>,
    weapons: BTreeMap<i64, OwnedGear>,
}

/// Suits are named `<type>_class<grade>`, e.g. `utilitysuit_class3`.
fn suit_gear(name: &str, localised: Option<&str>) -> Option<&'static str> {
    let by_name = match name.to_ascii_lowercase().split("_class").next().unwrap_or("") {
        "utilitysuit" => Some("maverick"),
        "tacticalsuit" => Some("dominator"),
        "explorationsuit" => Some("artemis"),
        _ => None,
    };
    let c = catalog();
    by_name
        .and_then(|id| c.suits.iter().find(|g| g.id == id))
        .or_else(|| localised.and_then(|l| c.suits.iter().find(|g| g.name.eq_ignore_ascii_case(l))))
        .map(|g| g.id.as_str())
}

/// Weapons are matched by their localised name (the journal gives it for every one).
fn weapon_gear(localised: Option<&str>) -> Option<&'static str> {
    localised.and_then(|l| catalog().weapons.iter().find(|g| g.name.eq_ignore_ascii_case(l))).map(|g| g.id.as_str())
}

fn str_of<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str)
}

fn int_of(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(Value::as_i64)
}

fn grade_of(v: Option<i64>) -> Option<u8> {
    v.and_then(|g| u8::try_from(g).ok()).filter(|g| (1..=5).contains(g))
}

/// `suit_nightvision` or `weapon_stability` → the catalogue modification of that kind.
fn mod_ids(v: Option<&Value>, kind: &str) -> Vec<String> {
    let prefix = format!("{kind}_");
    let mut out: Vec<String> = v
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter_map(|m| {
            let m = m.to_ascii_lowercase();
            let id = m.strip_prefix(&prefix).unwrap_or(&m);
            catalog().modification(id).filter(|md| md.kind == kind).map(|md| md.id.clone())
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

impl Owned {
    fn new_gear(gear: &str, grade: u8) -> Option<OwnedGear> {
        let g = catalog().gear(gear)?;
        Some(OwnedGear {
            gear: g.id.clone(),
            name: g.name.clone(),
            kind: if catalog().is_suit(gear) { "suit" } else { "weapon" },
            maker: g.maker.clone(),
            grade,
            mods: vec![],
        })
    }

    fn slot<'a>(map: &'a mut BTreeMap<i64, OwnedGear>, id: i64, gear: &str, grade: u8) -> Option<&'a mut OwnedGear> {
        match map.entry(id) {
            Entry::Occupied(o) => Some(o.into_mut()),
            Entry::Vacant(v) => Some(v.insert(Self::new_gear(gear, grade)?)),
        }
    }

    /// Apply one journal line (already parsed as JSON); other events are ignored.
    pub fn apply(&mut self, ev: &Value) {
        let Some(event) = str_of(ev, "event") else { return };
        match event {
            "BuySuit" => {
                let name = str_of(ev, "Name").unwrap_or("");
                if let (Some(id), Some(gear)) = (int_of(ev, "SuitID"), suit_gear(name, str_of(ev, "Name_Localised"))) {
                    self.suits.remove(&id);
                    Self::slot(&mut self.suits, id, gear, 1);
                }
            }
            "UpgradeSuit" => {
                let name = str_of(ev, "Name").unwrap_or("");
                if let (Some(id), Some(gear), Some(grade)) =
                    (int_of(ev, "SuitID"), suit_gear(name, str_of(ev, "Name_Localised")), grade_of(int_of(ev, "Class")))
                {
                    if let Some(s) = Self::slot(&mut self.suits, id, gear, grade) {
                        s.grade = grade;
                    }
                }
            }
            "SellSuit" => {
                if let Some(id) = int_of(ev, "SuitID") {
                    self.suits.remove(&id);
                }
            }
            "BuyWeapon" => {
                if let (Some(id), Some(gear)) = (int_of(ev, "SuitModuleID"), weapon_gear(str_of(ev, "Name_Localised")))
                {
                    self.weapons.remove(&id);
                    Self::slot(&mut self.weapons, id, gear, 1);
                }
            }
            "UpgradeWeapon" => {
                if let (Some(id), Some(gear), Some(grade)) = (
                    int_of(ev, "SuitModuleID"),
                    weapon_gear(str_of(ev, "Name_Localised")),
                    grade_of(int_of(ev, "Class")),
                ) {
                    if let Some(w) = Self::slot(&mut self.weapons, id, gear, grade) {
                        w.grade = grade;
                    }
                }
            }
            "SellWeapon" => {
                if let Some(id) = int_of(ev, "SuitModuleID") {
                    self.weapons.remove(&id);
                }
            }
            "SuitLoadout" => self.loadout(ev),
            _ => {}
        }
    }

    /// A `SuitLoadout` states the suit's grade and mods, and each weapon's.
    fn loadout(&mut self, ev: &Value) {
        let name = str_of(ev, "SuitName").unwrap_or("");
        if let (Some(id), Some(gear)) = (int_of(ev, "SuitID"), suit_gear(name, str_of(ev, "SuitName_Localised"))) {
            let grade = name.rsplit_once("_class").and_then(|(_, n)| n.parse::<i64>().ok());
            if let Some(grade) = grade_of(grade) {
                if let Some(s) = Self::slot(&mut self.suits, id, gear, grade) {
                    s.grade = grade;
                    s.mods = mod_ids(ev.get("SuitMods"), "suit");
                }
            }
        }
        for m in ev.get("Modules").and_then(Value::as_array).into_iter().flatten() {
            let (Some(id), Some(gear), Some(grade)) = (
                int_of(m, "SuitModuleID"),
                weapon_gear(str_of(m, "ModuleName_Localised")),
                grade_of(int_of(m, "Class")),
            ) else {
                continue;
            };
            if let Some(w) = Self::slot(&mut self.weapons, id, gear, grade) {
                w.grade = grade;
                w.mods = mod_ids(m.get("WeaponMods"), "weapon");
            }
        }
    }

    /// Every owned suit then weapon, by catalogue order, then grade.
    pub fn gear(&self) -> Vec<OwnedGear> {
        let c = catalog();
        let pos = |g: &OwnedGear| c.suits.iter().chain(&c.weapons).position(|x| x.id == g.gear);
        let mut v: Vec<OwnedGear> = self.suits.values().chain(self.weapons.values()).cloned().collect();
        v.sort_by(|a, b| pos(a).cmp(&pos(b)).then(a.grade.cmp(&b.grade)));
        v
    }

    pub fn is_empty(&self) -> bool {
        self.suits.is_empty() && self.weapons.is_empty()
    }
}
