//! # astromech-station
//!
//! What the station you are docked at offers, and where you saw a module before.
//!
//! - [`DockedHere`]: the station's services, economies, faction and state and pads (from the
//!   `Docked` event, as [`astromech_gamestate::GameState`] folds it), and which modules its
//!   outfitting sells that would upgrade the ship you are flying ([`upgrades`]).
//! - [`OutfittingBook`]: every `Outfitting.json` the core saw, in its own SQLite file
//!   (`outfitting.sqlite`), searched offline by module ("where did I see a 5A FSD?").
//!
//! The book is not part of `history.sqlite` on purpose. That file is rebuilt from the journals
//! (delete it and re-import), but the game overwrites `Outfitting.json` at every visit and no
//! journal line holds the module list, so a snapshot that is not stored is gone for good.

mod book;
mod here;

pub use book::{Item, OutfittingBook, Seen, StationOutfitting, DB_FILE};
pub use here::{state_from_journal_dir, upgrades, DockedHere, OutfittingHere, Upgrade};
