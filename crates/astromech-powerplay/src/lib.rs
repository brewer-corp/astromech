//! # astromech-powerplay
//!
//! Powerplay 2.0 from the journal: the commander's standing ([`standing`]) and the current
//! system's state ([`system`]), with the rank rules kept as data ([`rules`],
//! `data/catalog/powerplay.json`). Pure, so it can be replay-tested; the game state and the app
//! window fold the same events.

pub mod rules;
pub mod standing;
pub mod system;

pub use rules::{rules, Rules};
pub use standing::{standing_from_journal_dir, standing_from_lines, NextRank, Standing, StandingView};
pub use system::{PowerShare, SystemPower, Work};

use astromech_journal::{JournalEvent, JournalLine};
use serde::Serialize;
use std::path::Path;

/// The arrival line for a jump (`FSDJump` / `CarrierJump`) into a system with Powerplay
/// activity; `None` otherwise. `Location` (login, respawn) is not an arrival.
pub fn arrival_line(ev: &JournalEvent, pledged: Option<&str>) -> Option<String> {
    match ev {
        JournalEvent::FsdJump { .. } => SystemPower::from_event(ev).flatten().map(|s| s.spoken(pledged)),
        _ => None,
    }
}

/// Everything the app's Powerplay page reads from the journals.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Journal {
    pub standing: Standing,
    /// The current system, from the last `Location` / `FSDJump` / `CarrierJump`.
    pub here: Option<String>,
    /// Its Powerplay state; `None` when it has no Powerplay activity.
    pub system: Option<SystemPower>,
}

/// The current system as the page shows it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SystemView {
    #[serde(flatten)]
    pub power: SystemPower,
    /// What it offers the pledged power.
    pub work: Option<Work>,
    pub summary: String,
}

/// The page's data as of now.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Overview {
    pub standing: StandingView,
    pub here: Option<String>,
    pub system: Option<SystemView>,
}

impl Journal {
    pub fn apply(&mut self, l: &JournalLine) {
        self.standing.apply(&l.timestamp, &l.event);
        if let Some(s) = SystemPower::from_event(&l.event) {
            if let JournalEvent::FsdJump { star_system, .. } | JournalEvent::Location { star_system, .. } = &l.event {
                self.here = Some(star_system.clone());
            }
            self.system = s;
        }
    }

    pub fn overview(&self) -> Overview {
        let pledged = self.standing.power.as_deref();
        Overview {
            standing: self.standing.view(chrono::Utc::now()),
            here: self.here.clone(),
            system: self.system.as_ref().map(|s| SystemView {
                work: s.work_for(pledged),
                summary: s.summary(),
                power: s.clone(),
            }),
        }
    }
}

/// Journal lines that can change the page, found without parsing every line.
fn may_matter(line: &str) -> bool {
    ["\"Powerplay", "\"LoadGame\"", "\"FSDJump\"", "\"CarrierJump\"", "\"Location\""].iter().any(|e| line.contains(e))
}

/// Replay every journal in `dir`, oldest first.
pub fn from_journal_dir(dir: &Path) -> Journal {
    let mut j = Journal::default();
    for p in astromech_journal::journal_logs(dir) {
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        for line in text.lines().filter(|l| may_matter(l)) {
            if let Ok(l) = astromech_journal::parse_line(line.trim()) {
                j.apply(&l);
            }
        }
    }
    j
}
