//! # astromech-powerplay
//!
//! Powerplay 2.0 from the journal: the commander's standing ([`standing`]) with the rank rules
//! kept as data ([`rules`], `data/catalog/powerplay.json`). Pure, so it can be replay-tested;
//! the game state and the app window fold the same events.

pub mod rules;
pub mod standing;

pub use rules::{rules, Rules};
pub use standing::{standing_from_journal_dir, standing_from_lines, NextRank, Standing, StandingView};

use serde::Serialize;
use std::path::Path;

/// Everything the app's Powerplay page reads from the journals.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Journal {
    pub standing: Standing,
}

/// The page's data as of now.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Overview {
    pub standing: StandingView,
}

impl Journal {
    pub fn overview(&self) -> Overview {
        Overview { standing: self.standing.view(chrono::Utc::now()) }
    }
}

/// Replay every journal in `dir`, oldest first.
pub fn from_journal_dir(dir: &Path) -> Journal {
    Journal { standing: standing_from_journal_dir(dir) }
}
