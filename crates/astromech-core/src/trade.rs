//! Trade questions by voice ("where can I sell painite?", "where should I sell my cargo?"):
//! the lookup runs on Ardent Insight through `astromech-market`, from the commander's current
//! system, with the same remembered filters as the app's Trade page
//! (`search.filters.trade.sell` / `.buy` / `.cargo` in `astromech-store`; the pad follows the
//! current ship until the commander picks one). The hold comes from `Cargo.json`. The answer
//! is spoken by the assistant; nothing is pressed.

use astromech_data::Client;
use astromech_galaxy::Filters;
use astromech_gamestate::GameState;
use astromech_market::{cargo, catalog, prices, trade::DEFAULT_DISTANCE_LY, Origin};
use astromech_tools::trade::{answer as spoken, cargo_answer, Ask};

/// Jump range for the estimated jump counts when the ship's isn't known.
const DEFAULT_RANGE_LY: f64 = 20.0;

/// Said when a trade question names no commodity the catalogue knows.
pub const WHICH_COMMODITY: &str = "Which commodity? Say, for example, where to sell gold.";

/// The spoken answer to a trade question, from the current system or the system named in
/// `from` ([`crate::lookups::named_origin`], located on Spansh). `commodity` is the grammar's
/// slot value (a catalogue name).
pub fn answer(
    client: &Client,
    filters: &Filters,
    ask: Ask,
    commodity: Option<&str>,
    game: &GameState,
    from: Option<&str>,
    now_s: i64,
) -> String {
    let named = match crate::lookups::named_origin(game, from, |n| astromech_galaxy::locate_remote(client, n)) {
        Ok(n) => n,
        Err(m) => return m,
    };
    let origin = match (&named, game.system.as_deref()) {
        (Some(h), _) => Origin::from(h),
        (None, Some(system)) => Origin { system: system.to_string(), pos: None },
        (None, None) => return astromech_tools::lookup::NO_POSITION.into(),
    };
    let range = game.max_jump_range.filter(|r| *r > 1.0).unwrap_or(DEFAULT_RANGE_LY);
    let max_ly = filters.max_distance_ly.unwrap_or(DEFAULT_DISTANCE_LY);
    let said = if ask == Ask::SellCargo {
        match cargo::plan(client, &origin, &game.cargo_inventory, filters, range, now_s) {
            Ok(p) => cargo_answer(&p, max_ly, now_s),
            Err(e) => e.to_string(),
        }
    } else {
        let Some(c) = commodity.and_then(catalog::find) else {
            return WHICH_COMMODITY.into();
        };
        match prices(client, &origin, ask.side(), &[c.symbol.as_str()], filters, range, now_s) {
            Ok(p) => spoken(ask, &c.name, &p[0], max_ly, now_s),
            Err(e) => e.to_string(),
        }
    };
    crate::lookups::from_named(named.as_ref(), said)
}
