//! The docked view: the station's facts and what its outfitting would do for the ship flown.

use crate::book::{Item, OutfittingBook};
use astromech_engineering::{data, Fitted, Ship};
use astromech_gamestate::GameState;
use astromech_journal::StationInfo;
use serde::Serialize;
use std::path::Path;

/// Everything the docked panel shows. Without a dock (`docked: false`) the rest is empty.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct DockedHere {
    pub docked: bool,
    pub station: Option<String>,
    pub system: Option<String>,
    pub market_id: Option<i64>,
    /// Services, economies, faction and its state, government, pads, distance from the star.
    #[serde(flatten)]
    pub info: StationInfo,
    /// The biggest pad the station has: `Large`, `Medium` or `Small`.
    pub max_pad: Option<&'static str>,
    /// The ship being flown.
    pub ship: Option<String>,
    /// What the station's outfitting sells; `None` when none was seen for this station yet.
    pub outfitting: Option<OutfittingHere>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OutfittingHere {
    /// When the game wrote the file this is from.
    pub seen: String,
    /// Modules for sale.
    pub items: usize,
    /// What the current build needs: see [`upgrades`].
    pub upgrades: Vec<Upgrade>,
}

/// A module on sale that beats one fitted.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Upgrade {
    pub slot: String,
    pub slot_name: String,
    /// "5C Frame Shift Drive"
    pub fitted: String,
    pub item: String,
    /// "5A Frame Shift Drive"
    pub name: String,
    pub price: i64,
}

impl DockedHere {
    /// The facts for `state`, the `ship` flown (its fitted modules decide what the build needs)
    /// and whatever `book` has for this station.
    pub fn new(state: &GameState, ship: Option<&Ship>, book: Option<&OutfittingBook>) -> Self {
        if !state.docked {
            return Self::default();
        }
        let info = state.station_info.clone().unwrap_or_default();
        let outfitting = state.station_market_id.zip(book).and_then(|(id, b)| b.station(id).ok().flatten()).map(|o| {
            let upgrades = ship.map(|s| upgrades(&s.modules, &o.items)).unwrap_or_default();
            OutfittingHere { seen: o.seen, items: o.items.len(), upgrades }
        });
        let pads = info.landing_pads;
        let max_pad = pads.and_then(|p| {
            if p.large > 0 {
                Some("Large")
            } else if p.medium > 0 {
                Some("Medium")
            } else if p.small > 0 {
                Some("Small")
            } else {
                None
            }
        });
        DockedHere {
            docked: true,
            station: state.station.clone(),
            system: state.system.clone(),
            market_id: state.station_market_id,
            info,
            max_pad,
            ship: ship.map(|s| s.name.clone().unwrap_or_else(|| s.type_name.clone())),
            outfitting,
        }
    }
}

/// Where the commander is now, from the newest journal (and the one before it, which a new
/// session's `Location` may be in when the newest is a continuation part).
pub fn state_from_journal_dir(dir: &Path) -> GameState {
    let logs = astromech_journal::journal_logs(dir);
    let mut state = GameState::default();
    for p in logs.iter().skip(logs.len().saturating_sub(2)) {
        for l in astromech_journal::parse_file(p) {
            state.apply_journal(&l.event);
        }
    }
    state
}

/// Rating A (best) to E; `None` for the ones that don't compare (armour's `I`, weapons' `F`-`H`).
fn rank(rating: Option<&str>) -> Option<u8> {
    match rating? {
        "A" => Some(5),
        "B" => Some(4),
        "C" => Some(3),
        "D" => Some(2),
        "E" => Some(1),
        _ => None,
    }
}

/// What the current build needs, by one simple rule: for each fitted module, the module on sale
/// of the same kind and size (same group, name, class and mount) with a better rating, the
/// best rating first and the cheapest of those. Empty slots are not counted (the catalog has no
/// slot sizes per hull), nor are other kinds of module.
pub fn upgrades(fitted: &[Fitted], items: &[Item]) -> Vec<Upgrade> {
    let mut out = vec![];
    for f in fitted {
        let Some(have) = data::module(&f.item) else { continue };
        let Some(have_rank) = rank(have.rating.as_deref()) else { continue };
        let best = items
            .iter()
            .filter_map(|i| {
                let m = data::module(&i.item)?;
                let better = rank(m.rating.as_deref())?;
                (m.group == have.group
                    && m.name == have.name
                    && m.class == have.class
                    && m.mount == have.mount
                    && better > have_rank)
                    .then_some((better, i))
            })
            .max_by(|(ra, a), (rb, b)| ra.cmp(rb).then(b.price.cmp(&a.price)));
        if let Some((_, i)) = best {
            out.push(Upgrade {
                slot: f.slot.clone(),
                slot_name: f.slot_name.clone(),
                fitted: have.display(),
                item: i.item.clone(),
                name: i.name.clone(),
                price: i.price,
            });
        }
    }
    out
}
