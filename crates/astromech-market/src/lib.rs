//! # astromech-market
//!
//! Trade lookups for the "replace the daily toolbox" mission (S2): where to sell or buy a
//! commodity near a system, best price first, with the demand or stock, pad size and data age
//! of each market. Prices come from Ardent Insight's nearby imports and exports through
//! `astromech-data` (cache, schema check, rate limit); jump counts from `astromech-galaxy`'s
//! star map. [`catalog`] names the market commodities for the app and the voice grammar;
//! [`cargo`] plans the sale of the hold, as a whole load and per commodity.

pub mod cargo;
pub mod catalog;
pub mod trade;

pub use catalog::Commodity;
pub use trade::{prices, search, Offer, Origin, Prices, Search, Side};
