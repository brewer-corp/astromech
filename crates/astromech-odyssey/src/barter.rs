//! A bartender trade plan for the short assets on a shopping list: which surplus assets (held,
//! and needed by no pick) to trade, how much barter value they give, and what is still short
//! afterwards. The wiki gives a barter value for assets only, so only assets are traded (given
//! or received); goods and data stay on the "still short" list. The plan assumes a bartender
//! swaps equal barter value for equal barter value and holds whatever is asked for; the game
//! has not been checked on either (astromech-ha4m's validation issue).

use crate::catalog::catalog;
use crate::inventory::Holdings;
use crate::shopping::Need;
use serde::Serialize;
use std::collections::BTreeMap;

/// Some of one surplus asset given in a swap.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Give {
    pub id: String,
    pub name: String,
    pub count: i64,
    /// Barter value of one.
    pub barter: i64,
}

/// One short asset covered (wholly or partly) from surplus.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Swap {
    pub get_id: String,
    pub get_name: String,
    pub get: i64,
    /// Barter value of what is received.
    pub barter_wanted: i64,
    pub gives: Vec<Give>,
    /// Barter value of what is given; more than `barter_wanted` when the last item overshoots.
    pub barter_given: i64,
}

/// What is still short after the swaps.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Left {
    pub id: String,
    pub name: String,
    pub category: &'static str,
    pub short: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct BarterPlan {
    pub swaps: Vec<Swap>,
    /// Everything still short after the swaps, the assets no surplus covered and any goods or
    /// data (the wiki gives no barter value for those).
    pub left: Vec<Left>,
    /// Total barter value given.
    pub barter_given: i64,
}

/// Plan swaps for the short assets in `total` (the shopping list) from the surplus in `h`:
/// items held beyond what `total` needs. The most valuable short asset is covered first, from
/// the most valuable surplus asset down, so the fewest items change hands; only whole items
/// are received, and nothing is given that would not buy one.
pub fn plan(total: &[Need], h: &Holdings) -> BarterPlan {
    let c = catalog();
    let need: BTreeMap<&str, i64> = total.iter().map(|n| (n.id.as_str(), n.need)).collect();
    // Surplus assets, most valuable first.
    let mut spare: Vec<(String, i64, i64)> = c
        .items
        .iter()
        .filter_map(|(id, i)| {
            let b = i.barter.filter(|b| *b > 0)?;
            let n = h.have(id) - need.get(id.as_str()).copied().unwrap_or(0);
            (n > 0).then(|| (id.clone(), n, b))
        })
        .collect();
    spare.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));

    let mut short: Vec<&Need> = total.iter().filter(|n| n.short > 0 && n.barter.is_some_and(|b| b > 0)).collect();
    short.sort_by(|a, b| b.barter.cmp(&a.barter).then_with(|| a.id.cmp(&b.id)));

    let mut swaps = vec![];
    let mut got: BTreeMap<&str, i64> = BTreeMap::new();
    for want in short {
        let b_to = want.barter.unwrap_or(1);
        let pool: i64 = spare.iter().map(|(_, n, b)| n * b).sum();
        let get = want.short.min(pool / b_to);
        if get <= 0 {
            continue;
        }
        let wanted = get * b_to;
        let mut owed = wanted;
        let mut gives: Vec<Give> = vec![];
        // Spare is sorted most valuable first: take whole multiples from the most valuable
        // item that fits in what is owed; when none does, one of the cheapest that covers it.
        while owed > 0 {
            let fit = spare.iter().position(|(_, n, b)| *n > 0 && *b <= owed);
            let (i, count) = match fit {
                Some(i) => (i, (owed / spare[i].2).min(spare[i].1)),
                None => (spare.iter().rposition(|(_, n, _)| *n > 0).expect("the pool covers it"), 1),
            };
            let (id, n, b) = &mut spare[i];
            *n -= count;
            owed -= count * *b;
            match gives.iter_mut().find(|g| g.id == *id) {
                Some(g) => g.count += count,
                None => gives.push(Give { id: id.clone(), name: c.item_name(id), count, barter: *b }),
            }
        }
        let barter_given = gives.iter().map(|g| g.count * g.barter).sum();
        got.insert(want.id.as_str(), get);
        swaps.push(Swap {
            get_id: want.id.clone(),
            get_name: want.name.clone(),
            get,
            barter_wanted: wanted,
            gives,
            barter_given,
        });
    }

    let left = total
        .iter()
        .filter_map(|n| {
            let short = n.short - got.get(n.id.as_str()).copied().unwrap_or(0);
            (short > 0).then(|| Left { id: n.id.clone(), name: n.name.clone(), category: n.category, short })
        })
        .collect();
    let barter_given = swaps.iter().map(|s| s.barter_given).sum();
    BarterPlan { swaps, left, barter_given }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Materials;
    use crate::shopping::{shopping, Pick};

    fn held(items: &[(&str, i64)]) -> Holdings {
        let locker: Materials = items.iter().map(|(k, v)| (k.to_string(), *v)).collect();
        Holdings { locker, ..Default::default() }
    }

    fn gives(s: &Swap) -> Vec<(&str, i64)> {
        s.gives.iter().map(|g| (g.id.as_str(), g.count)).collect()
    }

    #[test]
    fn fixture_surplus_pays_for_the_short_asset_and_goods_stay_short() {
        let (h, p) = crate::sample();
        // Maverick 1 → 3 is short 7 Carbon Fibre Plating (11 barter each, 77 in all), plus goods
        // and data. The locker holds spare Graphene (2, 23 each), Weapon Component (1, 18),
        // Tungsten Carbide (7, 11) and Circuit Switch (1, 4) beyond what the picks need.
        let picks = [
            Pick::Upgrade { gear: "maverick".into(), from: 1, to: 3 },
            Pick::Mod { id: "nightvision".into(), maker: None },
            Pick::Upgrade { gear: "karmap15".into(), from: 1, to: 2 },
        ];
        let s = shopping(&picks, &h, &p.progress);
        assert_eq!(s.barter_short, 77);
        let t = &s.trade;
        assert_eq!(t.swaps.len(), 1);
        let sw = &t.swaps[0];
        assert_eq!((sw.get_id.as_str(), sw.get, sw.barter_wanted), ("carbonfibreplating", 7, 77));
        // Most valuable first: 2 Graphene (46) + 1 Weapon Component (18) leaves 13, then
        // 1 Tungsten Carbide (11) leaves 2, which the cheapest spare (Circuit Switch, 4) covers.
        assert_eq!(gives(sw), [("graphene", 2), ("weaponcomponent", 1), ("tungstencarbide", 1), ("circuitswitch", 1)]);
        assert_eq!((sw.barter_given, t.barter_given), (79, 79));
        // Carbon Fibre Plating is covered; the goods and data are still short, untouched.
        let left: Vec<(&str, i64)> = t.left.iter().map(|l| (l.id.as_str(), l.short)).collect();
        assert!(!left.iter().any(|(id, _)| *id == "carbonfibreplating"));
        let before: Vec<(&str, i64)> = s
            .total
            .iter()
            .filter(|n| n.short > 0 && n.id != "carbonfibreplating")
            .map(|n| (n.id.as_str(), n.short))
            .collect();
        assert_eq!(left, before);
        assert!(t.left.iter().all(|l| l.category != "assets"));
    }

    #[test]
    fn a_plan_never_gives_more_than_is_spare_and_covers_what_it_can() {
        let c = catalog();
        // 5 Graphene held, 1 needed → 4 spare (92 barter). Short 5 Tungsten Carbide (11 each,
        // 55) and 1 Weapon Component (18) with nothing else spare.
        let h = held(&[("graphene", 5)]);
        let total = |h: &Holdings, id: &str, need: i64| Need {
            id: id.into(),
            name: c.item_name(id),
            category: "assets",
            need,
            have: h.have(id),
            short: (need - h.have(id)).max(0),
            barter: c.items[id].barter,
        };
        let list = [total(&h, "graphene", 1), total(&h, "tungstencarbide", 5), total(&h, "weaponcomponent", 1)];
        let t = plan(&list, &h);
        // Weapon Component (18, the dearer) is first: one Graphene (23) is the only way to pay.
        // The 3 left (69) pay for the 5 Tungsten Carbide short (55): all three, as 2 Graphene
        // leave 9 owed and the next Graphene is the cheapest that covers it.
        let by: Vec<(&str, i64, Vec<(&str, i64)>)> =
            t.swaps.iter().map(|s| (s.get_id.as_str(), s.get, gives(s))).collect();
        assert_eq!(by, [("weaponcomponent", 1, vec![("graphene", 1)]), ("tungstencarbide", 5, vec![("graphene", 3)])]);
        assert_eq!(t.barter_given, 23 + 69);
        assert!(t.left.iter().all(|l| l.id == "graphene" || l.short == 0));
        let given: i64 = t.swaps.iter().flat_map(|s| &s.gives).filter(|g| g.id == "graphene").map(|g| g.count).sum();
        assert!(given <= 4);
        // Not enough spare to buy even one: nothing is traded.
        let poor = held(&[("circuitswitch", 3), ("graphene", 1)]);
        let list = [total(&poor, "graphene", 1), total(&poor, "circuitswitch", 2), total(&poor, "weaponcomponent", 1)];
        let none = plan(&list, &poor);
        assert!(none.swaps.is_empty());
        assert_eq!(none.left.iter().map(|l| l.id.as_str()).collect::<Vec<_>>(), ["weaponcomponent"]);
    }
}
