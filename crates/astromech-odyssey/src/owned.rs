//! The suits and weapons the commander owns, with their grade and fitted modifications, from the
//! journal: `BuySuit`, `BuyWeapon`, `UpgradeSuit`, `UpgradeWeapon`, `SellSuit`, `SellWeapon` and
//! the loadout events `SuitLoadout`, `SwitchSuitLoadout` and `CreateSuitLoadout` (one shape).
//!
//! Field names and symbol formats are from real Odyssey journals as collected in the community
//! schemas jixxed/ed-journal-schemas (`schemas/BuySuit`, `BuyWeapon`, `UpgradeSuit`,
//! `UpgradeWeapon`, `SellSuit`, `SellWeapon`, `SuitLoadout`, `SwitchSuitLoadout`,
//! `CreateSuitLoadout`), with the symbol and mod-name tables from EDCD/EDDI's data definitions
//! (see `scripts/gen-odyssey.py`). A suit's `Name` ends `_class<grade>`; a weapon's `Class` is its
//! grade; `SuitMods` and `WeaponMods` list the mods fitted, on purchase and sale as well as in a
//! loadout (Pioneer Supplies sells upgraded and modified gear). `Name_Localised` is not used:
//! it is optional, language-dependent, and for suits a `$UtilitySuit_Class1_Name;` token. Only
//! the suits and weapons in the catalogue are kept (not the Flight Suit). A weapon the commander
//! has never been logged with (no purchase, no loadout) is unknown.
//!
//! The loadout events also give each loadout: `LoadoutID`, `LoadoutName`, the suit by `SuitID`
//! and each weapon of `Modules` by `SlotName` and `SuitModuleID`. Each event states the whole
//! loadout, so the latest one for a `LoadoutID` replaces what was known of it. The worn loadout
//! is the one the latest `SuitLoadout` or `SwitchSuitLoadout` names (`CreateSuitLoadout` does
//! not put it on). The Flight Suit's loadout is not kept, as the Flight Suit is not. The journal
//! files hold no `RenameSuitLoadout`, `DeleteSuitLoadout`, `LoadoutEquipModule` or
//! `LoadoutRemoveModule` line, so those changes show only at the next loadout event that names
//! the loadout.

use crate::catalog::catalog;
use serde::Serialize;
use serde_json::Value;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

/// Where the crate reports what it could not map; the caller supplies it (no global sink).
pub type Log<'a> = &'a dyn Fn(&str);

/// One owned suit or weapon.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct OwnedGear {
    /// The game's id for this one item: `SuitID` for a suit, `SuitModuleID` for a weapon.
    pub id: i64,
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

/// A suit loadout: its suit and the weapon in each slot, each the owned item it is.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct SuitLoadout {
    /// The game's `LoadoutID`.
    pub id: i64,
    pub name: String,
    /// The commander has it on (the latest `SuitLoadout` or `SwitchSuitLoadout` named it).
    pub worn: bool,
    pub suit: OwnedGear,
    /// The weapons, in slot order (`PrimaryWeapon1`, `PrimaryWeapon2`, `SecondaryWeapon`).
    pub slots: Vec<LoadoutSlot>,
}

/// One weapon slot of a suit loadout.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
pub struct LoadoutSlot {
    /// The journal's `SlotName`: `PrimaryWeapon1`, `PrimaryWeapon2` or `SecondaryWeapon`.
    pub slot: String,
    pub item: OwnedGear,
}

/// The weapon slots a loadout has, in order.
const SLOTS: [&str; 3] = ["PrimaryWeapon1", "PrimaryWeapon2", "SecondaryWeapon"];

/// A loadout as the journal last stated it: the ids of its suit and of the weapon in each slot.
#[derive(Debug, Clone, PartialEq)]
struct LoadoutRec {
    name: String,
    suit: i64,
    /// (slot index in `SLOTS`, `SuitModuleID`).
    weapons: BTreeMap<usize, i64>,
}

/// The owned gear by the game's id: `SuitID` for suits, `SuitModuleID` for weapons; and the
/// loadouts by `LoadoutID`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Owned {
    suits: BTreeMap<i64, OwnedGear>,
    weapons: BTreeMap<i64, OwnedGear>,
    loadouts: BTreeMap<i64, LoadoutRec>,
    /// The `LoadoutID` the latest `SuitLoadout` or `SwitchSuitLoadout` named.
    worn: Option<i64>,
}

/// `utilitysuit_class3` → (`maverick`, 3).
fn suit_of(name: &str) -> Option<(&'static str, Option<u8>)> {
    let name = name.to_ascii_lowercase();
    let (symbol, grade) = match name.split_once("_class") {
        Some((s, g)) => (s, g.parse::<u8>().ok()),
        None => (name.as_str(), None),
    };
    catalog().suits.iter().find(|g| g.journal == symbol).map(|g| (g.id.as_str(), grade))
}

fn weapon_of(symbol: &str) -> Option<&'static str> {
    let symbol = symbol.to_ascii_lowercase();
    catalog().weapons.iter().find(|g| g.journal == symbol).map(|g| g.id.as_str())
}

fn str_of<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str)
}

fn int_of(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(Value::as_i64)
}

fn grade_of(g: Option<i64>) -> Option<u8> {
    g.and_then(|g| u8::try_from(g).ok()).filter(|g| (1..=5).contains(g))
}

/// A journal mod name the catalogue does not have; each is reported once.
fn report_unknown_mod(name: &str, log: Log) {
    static SEEN: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
    if SEEN.lock().is_ok_and(|mut s| s.insert(name.to_string())) {
        log(&format!("odyssey: unknown {name} in the journal (not in data/catalog/odyssey.json)"));
    }
}

/// `SuitMods` / `WeaponMods` → the catalogue modification ids of `kind`.
fn mod_ids(v: Option<&Value>, kind: &str, log: Log) -> Vec<String> {
    let mut out: Vec<String> = v
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter_map(|m| {
            let lower = m.to_ascii_lowercase();
            let found = catalog().mods.iter().find(|md| md.kind == kind && md.journal == lower);
            if found.is_none() {
                report_unknown_mod(m, log);
            }
            found.map(|md| md.id.clone())
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

impl Owned {
    fn new_gear(id: i64, gear: &str, grade: u8, mods: Vec<String>) -> Option<OwnedGear> {
        let g = catalog().gear(gear)?;
        Some(OwnedGear {
            id,
            gear: g.id.clone(),
            name: g.name.clone(),
            kind: if catalog().is_suit(gear) { "suit" } else { "weapon" },
            maker: g.maker.clone(),
            grade,
            mods,
        })
    }

    /// Record `gear` under `id`, replacing what was there.
    fn set(map: &mut BTreeMap<i64, OwnedGear>, id: i64, gear: &str, grade: u8, mods: Vec<String>) -> bool {
        match Self::new_gear(id, gear, grade, mods) {
            Some(g) => {
                map.insert(id, g);
                true
            }
            None => false,
        }
    }

    /// Set the grade of what is under `id`, adding it (no mods known) if it was not seen.
    fn regrade(map: &mut BTreeMap<i64, OwnedGear>, id: i64, gear: &str, grade: u8) {
        match map.entry(id) {
            Entry::Occupied(mut o) => o.get_mut().grade = grade,
            Entry::Vacant(v) => {
                if let Some(g) = Self::new_gear(id, gear, grade, vec![]) {
                    v.insert(g);
                }
            }
        }
    }

    /// Apply one journal line (already parsed as JSON); other events are ignored.
    pub fn apply(&mut self, ev: &Value, log: Log) {
        let Some(event) = str_of(ev, "event") else { return };
        match event {
            "BuySuit" | "UpgradeSuit" => {
                let Some((gear, from_name)) = suit_of(str_of(ev, "Name").unwrap_or("")) else { return };
                let Some(id) = int_of(ev, "SuitID") else { return };
                let grade = if event == "UpgradeSuit" {
                    grade_of(int_of(ev, "Class")).or_else(|| grade_of(from_name.map(i64::from)))
                } else {
                    grade_of(from_name.map(i64::from))
                };
                let Some(grade) = grade else { return };
                if event == "BuySuit" {
                    Self::set(&mut self.suits, id, gear, grade, mod_ids(ev.get("SuitMods"), "suit", log));
                } else {
                    Self::regrade(&mut self.suits, id, gear, grade);
                }
            }
            "SellSuit" => {
                if let Some(id) = int_of(ev, "SuitID") {
                    self.suits.remove(&id);
                }
            }
            "BuyWeapon" | "UpgradeWeapon" => {
                let (Some(id), Some(gear), Some(grade)) = (
                    int_of(ev, "SuitModuleID"),
                    weapon_of(str_of(ev, "Name").unwrap_or("")),
                    grade_of(int_of(ev, "Class")),
                ) else {
                    return;
                };
                if event == "BuyWeapon" {
                    Self::set(&mut self.weapons, id, gear, grade, mod_ids(ev.get("WeaponMods"), "weapon", log));
                } else {
                    Self::regrade(&mut self.weapons, id, gear, grade);
                }
            }
            "SellWeapon" => {
                if let Some(id) = int_of(ev, "SuitModuleID") {
                    self.weapons.remove(&id);
                }
            }
            "SuitLoadout" | "SwitchSuitLoadout" | "CreateSuitLoadout" => {
                self.loadout(ev, log);
                if event != "CreateSuitLoadout" {
                    self.worn = int_of(ev, "LoadoutID");
                }
            }
            _ => {}
        }
    }

    /// A loadout event states the suit's grade and mods, and each weapon's, which are recorded
    /// as owned gear whether or not a purchase was seen; and the loadout itself, kept when its
    /// suit is one the catalogue has (not the Flight Suit).
    fn loadout(&mut self, ev: &Value, log: Log) {
        let mut suit = None;
        if let (Some(id), Some((gear, Some(grade)))) =
            (int_of(ev, "SuitID"), suit_of(str_of(ev, "SuitName").unwrap_or("")))
        {
            if let Some(grade) = grade_of(Some(i64::from(grade))) {
                if Self::set(&mut self.suits, id, gear, grade, mod_ids(ev.get("SuitMods"), "suit", log)) {
                    suit = Some(id);
                }
            }
        }
        let mut weapons = BTreeMap::new();
        for m in ev.get("Modules").and_then(Value::as_array).into_iter().flatten() {
            let (Some(id), Some(gear), Some(grade)) = (
                int_of(m, "SuitModuleID"),
                weapon_of(str_of(m, "ModuleName").unwrap_or("")),
                grade_of(int_of(m, "Class")),
            ) else {
                continue;
            };
            if Self::set(&mut self.weapons, id, gear, grade, mod_ids(m.get("WeaponMods"), "weapon", log)) {
                if let Some(slot) =
                    str_of(m, "SlotName").and_then(|s| SLOTS.iter().position(|x| x.eq_ignore_ascii_case(s)))
                {
                    weapons.insert(slot, id);
                }
            }
        }
        let Some(loadout) = int_of(ev, "LoadoutID") else { return };
        match suit {
            Some(suit) => {
                let name = str_of(ev, "LoadoutName").unwrap_or_default().to_string();
                self.loadouts.insert(loadout, LoadoutRec { name, suit, weapons });
            }
            None => {
                self.loadouts.remove(&loadout);
            }
        }
    }

    /// The loadouts in `LoadoutID` order, each item as the owned gear it is. A loadout whose suit
    /// has been sold since is left out, as is a weapon sold since.
    pub fn loadouts(&self) -> Vec<SuitLoadout> {
        self.loadouts
            .iter()
            .filter_map(|(&id, l)| {
                Some(SuitLoadout {
                    id,
                    name: l.name.clone(),
                    worn: self.worn == Some(id),
                    suit: self.suits.get(&l.suit)?.clone(),
                    slots: l
                        .weapons
                        .iter()
                        .filter_map(|(&slot, w)| {
                            Some(LoadoutSlot { slot: SLOTS[slot].to_string(), item: self.weapons.get(w)?.clone() })
                        })
                        .collect(),
                })
            })
            .collect()
    }

    /// Every owned suit then weapon, by catalogue order, then grade.
    pub fn gear(&self) -> Vec<OwnedGear> {
        let c = catalog();
        let pos = |g: &OwnedGear| c.suits.iter().chain(&c.weapons).position(|x| x.id == g.gear);
        let mut v: Vec<OwnedGear> = self.suits.values().chain(self.weapons.values()).cloned().collect();
        v.sort_by(|a, b| pos(a).cmp(&pos(b)).then(a.grade.cmp(&b.grade)));
        v
    }
}
