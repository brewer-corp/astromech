//! # astromech-market
//!
//! Trade lookups for the "replace the daily toolbox" mission (S2): where to sell or buy a
//! commodity near a system or anywhere in the galaxy, best price first, with the demand or
//! stock, pad size and data age of each market. Prices come from Ardent Insight's nearby and
//! galaxy-wide imports and exports through `astromech-data` (cache, schema check, rate limit);
//! jump counts from `astromech-galaxy`'s star map. [`catalog`] names the market commodities for
//! the app and the voice grammar; [`cargo`] plans the sale of the hold, as a whole load and per
//! commodity; [`list`] finds where to buy a shopping list (colonisation), stations stocking most
//! of it first; [`here`] sets the docked market (`Market.json`) and the hold against the best
//! nearby prices; [`route`] finds single-hop and loop trade routes (Spansh's trade router, the
//! way back from Ardent markets).

pub mod cargo;
pub mod catalog;
pub mod here;
pub mod list;
pub mod route;
pub mod trade;

pub use catalog::Commodity;
pub use trade::{anywhere, prices, search, Offer, Origin, Prices, Search, Side};
