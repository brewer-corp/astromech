//! # astromech-odyssey
//!
//! Odyssey on-foot micro-resources (ARCHITECTURE.md §18.7, "Odyssey micro-resources"): the
//! inventory from `ShipLocker.json` and `Backpack.json`, suit and weapon grade upgrades and
//! modifications with what each is missing, a shopping list over several picks, the on-foot
//! engineers with their unlock status and referral chain, and bartender values. Game data is
//! `data/catalog/odyssey.json` (`scripts/gen-odyssey.py`).

pub mod catalog;
pub mod engineers;
pub mod inventory;
pub mod shopping;

pub use catalog::catalog;
pub use engineers::{progress_from_journal_dir, EngineerLine, Progress};
pub use inventory::{Holdings, ItemLine};
pub use shopping::{shopping, Pick, Shopping};

use serde::Serialize;
use std::path::Path;

/// A suit or weapon in the blueprint picker.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GearRef {
    pub id: String,
    pub name: String,
    /// `suit` or `weapon`.
    pub kind: &'static str,
    pub maker: Option<String>,
}

/// A modification in the blueprint picker.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModRef {
    pub id: String,
    pub name: String,
    pub kind: String,
    /// Weapon makers with their own cost; empty when one cost fits all.
    pub makers: Vec<String>,
}

/// Everything the Odyssey page shows except a shopping list.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OdysseyView {
    pub source: String,
    pub locker_ts: Option<String>,
    pub backpack_ts: Option<String>,
    pub items: Vec<ItemLine>,
    pub engineers: Vec<EngineerLine>,
    pub gear: Vec<GearRef>,
    pub mods: Vec<ModRef>,
    /// Where the game data is from, when it was fetched, and what it is unsure of.
    pub data_source: String,
    pub data_date: String,
    pub data_uncertain: Vec<String>,
}

pub fn view(h: &Holdings, progress: &Progress, source: &str) -> OdysseyView {
    let c = catalog();
    let gear = c
        .suits
        .iter()
        .map(|g| (g, "suit"))
        .chain(c.weapons.iter().map(|g| (g, "weapon")))
        .map(|(g, kind)| GearRef { id: g.id.clone(), name: g.name.clone(), kind, maker: g.maker.clone() })
        .collect();
    let mods = c
        .mods
        .iter()
        .map(|m| ModRef {
            id: m.id.clone(),
            name: m.name.clone(),
            kind: m.kind.clone(),
            makers: m.costs.iter().filter_map(|c| c.maker.clone()).collect(),
        })
        .collect();
    OdysseyView {
        source: source.to_string(),
        locker_ts: h.locker_ts.clone(),
        backpack_ts: h.backpack_ts.clone(),
        items: inventory::item_lines(h),
        engineers: engineers::lines(progress),
        gear,
        mods,
        data_source: c.source.clone(),
        data_date: c.generated.clone(),
        data_uncertain: c.uncertain.clone(),
    }
}

/// The synthetic fixtures (app window sample): a ShipLocker, a Backpack and a journal.
pub fn sample() -> (Holdings, Progress) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    (Holdings::from_journal_dir(&dir), progress_from_journal_dir(&dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line<'a>(v: &'a OdysseyView, id: &str) -> &'a ItemLine {
        v.items.iter().find(|i| i.id == id).unwrap_or_else(|| panic!("{id}"))
    }

    #[test]
    fn shiplocker_fixture_gives_the_inventory() {
        let (h, p) = sample();
        assert_eq!(h.locker_ts.as_deref(), Some("2026-10-14T12:05:00Z"));
        let v = view(&h, &p, "Sample");
        let g = line(&v, "graphene");
        assert_eq!((g.category, g.locker, g.backpack), ("assets", 7, 2));
        assert_eq!((g.barter, g.value), (Some(23), Some(1300)));
        let s = line(&v, "suitschematic");
        assert_eq!((s.category, s.locker), ("goods", 3));
        let m = line(&v, "manufacturinginstructions");
        assert_eq!((m.category, m.locker), ("data", 4));
        assert_eq!(line(&v, "healthpack").category, "consumables");
        // Mission items are listed apart and not counted.
        let mon = line(&v, "healthmonitor");
        assert_eq!((mon.locker, mon.mission), (1, 2));
        assert_eq!(h.have("healthmonitor"), 1);
        // Something the catalogue doesn't know still shows, under the file's category.
        let x = line(&v, "notarealthing");
        assert_eq!((x.name.as_str(), x.category, x.locker), ("Not A Real Thing", "assets", 5));
        // Everything in the catalogue is listed, held or not.
        assert!(v.items.len() > 190);
    }

    #[test]
    fn chosen_blueprint_shows_whats_missing() {
        let (h, p) = sample();
        // Maverick grade 1 → 2 needs 1 Suit Schematic, 1 Health Monitor, 1 Manufacturing
        // Instructions, 2 Carbon Fibre Plating, 2 Graphene and 600,000 CR.
        let pick = Pick::Upgrade { gear: "maverick".into(), from: 1, to: 2 };
        let s = shopping(std::slice::from_ref(&pick), &h, &p).unwrap();
        let r = &s.picks[0];
        assert_eq!(r.title, "Maverick Suit grade 1 → 2");
        assert_eq!(r.credits, 600_000);
        let short: Vec<(&str, i64)> =
            r.lines.iter().filter(|l| l.short > 0).map(|l| (l.id.as_str(), l.short)).collect();
        // The fixture holds 0 Carbon Fibre Plating; everything else is covered.
        assert_eq!(short, [("carbonfibreplating", 2)]);
        assert!(!r.ready);
        // Grade 1 → 5 adds up every step.
        let all = shopping(&[Pick::Upgrade { gear: "maverick".into(), from: 1, to: 5 }], &h, &p).unwrap();
        let graphene = all.picks[0].lines.iter().find(|l| l.id == "graphene").unwrap();
        let steps: i64 = catalog().gear("maverick").unwrap().upgrades.iter().map(|u| u.materials["graphene"]).sum();
        assert_eq!((graphene.need, graphene.have), (steps, 9));
    }

    #[test]
    fn modifications_and_the_shopping_list() {
        let (h, p) = sample();
        let picks = [
            Pick::Mod { id: "nightvision".into(), maker: None },
            Pick::Mod { id: "headshotdamage".into(), maker: Some("takada".into()) },
            Pick::Upgrade { gear: "karmap15".into(), from: 2, to: 3 },
        ];
        let s = shopping(&picks, &h, &p).unwrap();
        assert_eq!(s.picks[1].title, "Headshot Damage (Takada weapons)");
        // Night Vision: Oden Geiger and Yi Shen fit it; the fixture has Oden unlocked.
        let fit: Vec<(&str, bool)> = s.picks[0].fitted_by.iter().map(|f| (f.name.as_str(), f.unlocked)).collect();
        assert_eq!(fit, [("Oden Geiger", true), ("Yi Shen", false)]);
        // The list adds every pick's needs together and compares once.
        let sum = |id: &str| s.picks.iter().flat_map(|r| &r.lines).filter(|l| l.id == id).map(|l| l.need).sum::<i64>();
        for l in &s.total {
            assert_eq!(l.need, sum(&l.id), "{}", l.id);
            assert_eq!(l.short, (l.need - h.have(&l.id)).max(0));
        }
        assert_eq!(s.credits, s.picks.iter().map(|r| r.credits).sum::<i64>());
        let barter: i64 = s.total.iter().filter_map(|l| l.barter.map(|b| b * l.short)).sum();
        assert!(barter > 0);
        assert_eq!(s.barter_short, barter);
        // Bad picks name what's wrong.
        let e = shopping(&[Pick::Mod { id: "headshotdamage".into(), maker: None }], &h, &p).unwrap_err();
        assert_eq!(e, "Headshot Damage: pick the weapon maker");
        let e = shopping(&[Pick::Upgrade { gear: "maverick".into(), from: 3, to: 3 }], &h, &p).unwrap_err();
        assert!(e.contains("from grade 3 to 3"), "{e}");
        assert!(shopping(&[Pick::Upgrade { gear: "nope".into(), from: 1, to: 2 }], &h, &p).is_err());
    }

    #[test]
    fn engineer_unlocks_and_referrals() {
        let (h, p) = sample();
        let v = view(&h, &p, "Sample");
        let e = |n: &str| v.engineers.iter().find(|e| e.name == n).unwrap();
        // Domino Green is unlocked; her referral reveals Kit Fowler, still unknown.
        let dg = e("Domino Green");
        assert_eq!(dg.progress, "Unlocked");
        assert_eq!(dg.refers, ["Kit Fowler"]);
        assert_eq!(dg.next, "Referral to Kit Fowler: Provide 5 Push");
        assert_eq!(e("Kit Fowler").next, "Needs a referral from Domino Green");
        assert_eq!(e("Hero Ferrari").progress, "Invited");
        assert_eq!(e("Hero Ferrari").next, "Visit Siris to unlock");
        assert!(e("Jude Navarro").next.starts_with("To get invited: "));
        // Oden Geiger unlocked and refers nobody: nothing left to do.
        assert_eq!(e("Oden Geiger").progress, "Unlocked");
        assert_eq!(e("Oden Geiger").next, "");
        assert_eq!(e("Yi Shen").next, "Needs a referral from Baltanos, Eleanor Bresa and Rosa Dayette");
        // Ship engineers in the same event are not listed.
        assert_eq!(v.engineers.len(), 13);
    }

    #[test]
    fn missing_files_mean_an_empty_inventory() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("ShipLocker.json"), "{\"timestamp\":").unwrap();
        let h = Holdings::from_journal_dir(d.path());
        assert_eq!(h, Holdings::default());
        assert!(progress_from_journal_dir(d.path()).is_empty());
    }
}
