//! # astromech-powerplay
//!
//! Powerplay 2.0 from the journal: the commander's standing ([`standing`]) and the current
//! system's state ([`system`]), with the rank rules kept as data ([`rules`],
//! `data/catalog/powerplay.json`) and what earns merits in each kind of system ([`activities`],
//! `data/catalog/powerplay-activities.json`) and where power commodities and data move ([`logistics`],
//! `data/catalog/powerplay-logistics.json`). Pure, so it can be replay-tested; the game state and the app
//! window fold the same events.

pub mod activities;
pub mod logistics;
pub mod rules;
pub mod standing;
pub mod system;

pub use activities::{commodity, rules_version, RulesVersion, Suggestion};
pub use rules::{rules, Rules};
pub use standing::{standing_from_journal_dir, standing_from_lines, NextRank, Standing, StandingView};
pub use system::{PowerShare, SystemPower, Work};

use astromech_journal::JournalTail;
use astromech_journal::{JournalEvent, JournalLine};
use serde::Serialize;
use std::path::{Path, PathBuf};

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
    /// What earns the pledged power merits here.
    pub suggestions: Vec<Suggestion>,
    /// The activity table the suggestions come from.
    pub activity_rules: RulesVersion,
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
                suggestions: s.suggestions(pledged),
                activity_rules: rules_version(),
                power: s.clone(),
            }),
        }
    }
}

/// Journal lines that can change the page, found without parsing every line.
fn may_matter(line: &str) -> bool {
    ["\"Powerplay", "\"LoadGame\"", "\"FSDJump\"", "\"CarrierJump\"", "\"Location\"", "\"Fileheader\"", "\"Missions\""]
        .iter()
        .any(|e| line.contains(e))
}

/// Replay every journal in `dir`, oldest first.
pub fn from_journal_dir(dir: &Path) -> Journal {
    let mut f = Follower::new(dir);
    f.refresh();
    f.journal
}

/// [`Journal`] kept up to date for a page that polls: each [`Follower::refresh`] reads only the
/// lines the game added since the last one (the newest file grows; new files start), instead
/// of every journal again. It replays from the start when the files read so far are no longer
/// the oldest ones in the folder (one deleted, an older one added) or the newest one shrank.
#[derive(Debug)]
pub struct Follower {
    dir: PathBuf,
    /// The files read, oldest first; the last is still being followed.
    read: Vec<PathBuf>,
    tail: Option<JournalTail>,
    journal: Journal,
}

impl Follower {
    pub fn new(dir: &Path) -> Self {
        Self { dir: dir.to_path_buf(), read: vec![], tail: None, journal: Journal::default() }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Read what is new and return the journal as of now.
    pub fn refresh(&mut self) -> &Journal {
        let logs = astromech_journal::journal_logs(&self.dir);
        if !logs.starts_with(&self.read) {
            *self = Self::new(&self.dir);
        }
        if let Some(tail) = &mut self.tail {
            let restarts = tail.restarts();
            let lines = tail.read_new();
            if tail.restarts() != restarts {
                // The newest file shrank: its lines were replaced, so replay everything.
                *self = Self::new(&self.dir);
                return self.refresh();
            }
            apply_lines(&mut self.journal, &lines);
        }
        for path in &logs[self.read.len()..] {
            let mut tail = JournalTail::new(path.clone(), 0);
            apply_lines(&mut self.journal, &tail.read_new());
            self.read.push(path.clone());
            self.tail = Some(tail);
        }
        &self.journal
    }
}

fn apply_lines(j: &mut Journal, lines: &[String]) {
    for line in lines.iter().filter(|l| may_matter(l)) {
        if let Ok(l) = astromech_journal::parse_line(line) {
            j.apply(&l);
        }
    }
}
