//! Trade questions by voice ("where can I sell painite?"): the lookup runs on Ardent Insight
//! through `astromech-market`, from the commander's current system, with the same remembered
//! filters as the app's Trade page (`search.filters.trade.sell` / `.buy` in `astromech-store`;
//! the pad follows the current ship until the commander picks one). The answer is spoken by
//! the assistant; nothing is pressed.

use astromech_data::Client;
use astromech_galaxy::{filters, Filters, Pad};
use astromech_gamestate::GameState;
use astromech_market::{catalog, prices, trade::DEFAULT_DISTANCE_LY, Origin};
use astromech_store::Store;
use astromech_tools::trade::{answer as spoken, Ask};

/// Jump range for the estimated jump counts when the ship's isn't known.
const DEFAULT_RANGE_LY: f64 = 20.0;

/// The lookup's remembered filters for the current ship (defaults if none are stored).
pub fn filters_for(store: Option<&Store>, ask: Ask, game: &GameState) -> Filters {
    let pad = game.ship.as_deref().and_then(Pad::for_ship);
    store
        .and_then(|s| filters::load(s, ask.side().lookup(), pad).ok())
        .unwrap_or_else(|| Filters::default().for_ship(pad))
}

/// The spoken answer to a trade question. `commodity` is the grammar's slot value (a
/// catalogue name).
pub fn answer(
    client: &Client,
    filters: &Filters,
    ask: Ask,
    commodity: Option<&str>,
    game: &GameState,
    now_s: i64,
) -> String {
    let Some(c) = commodity.and_then(catalog::find) else {
        return "Which commodity? Say, for example, where can I sell gold.".into();
    };
    let Some(system) = game.system.as_deref() else {
        return "I don't know where you are yet.".into();
    };
    let origin = Origin { system: system.to_string(), pos: None };
    let range = game.max_jump_range.filter(|r| *r > 1.0).unwrap_or(DEFAULT_RANGE_LY);
    match prices(client, &origin, ask.side(), &[c.symbol.as_str()], filters, range, now_s) {
        Ok(p) => spoken(ask, &c.name, &p[0], filters.max_distance_ly.unwrap_or(DEFAULT_DISTANCE_LY), now_s),
        Err(e) => e.to_string(),
    }
}
