//! # astromech-exploration
//!
//! Exploration from the journal, for the app's Exploration page and the callouts:
//!
//! - [`value`]: what a star or planet is worth to Universal Cartographics, scanned, mapped and
//!   first discovered (the community's formulae, with their sources).
//! - [`system`]: the live system view, the current system's bodies as they are scanned.
//! - [`unsold`]: the cartographic and organic data aboard and what it would fetch.
//! - [`biology`]: the system's biology ranked by value, genera known or predicted with their
//!   likely species, over [`system`]'s bodies.
//! - [`exobiology`]: what Vista Genomics pays for a species (`data/catalog/exobiology.json`).
//!
//! Pure folds over [`astromech_journal::JournalLine`]s, so they are replay-tested.

pub mod biology;
pub mod exobiology;
pub mod system;
pub mod unsold;
pub mod value;

pub use system::{BodyView, SystemFold, SystemView};
pub use unsold::{Unsold, UnsoldFold};
