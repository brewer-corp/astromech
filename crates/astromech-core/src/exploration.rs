//! Exploration questions by voice ("what is this system worth?", "is this system worth
//! mapping?", "which organics haven't I scanned here?", "how much is my unsold data worth?"),
//! read the way the app's Exploration pages read them: the current system folded from the
//! newest journals (`astromech_exploration::SystemViewCache`) with what earlier visits scanned
//! there from the history (`History::add_earlier_visits`), its biology ranked over that fold
//! (`astromech_exploration::biology::ranked`), and the unsold data from the history
//! (`astromech_history::UnsoldCache`, the latest commander). Nothing is pressed.

use astromech_announce::session_summary::Unreadable;
use astromech_exploration::{SystemFold, SystemViewCache};
use astromech_history::{History, UnsoldCache};
use astromech_tools::exploration::{self as words, Ask};
use std::path::PathBuf;
use std::sync::Mutex;

/// Where the answers are read from, with the folds remembered while those are unchanged.
#[derive(Default)]
pub struct Exploration {
    journals: Option<PathBuf>,
    history: Option<PathBuf>,
    view: Mutex<SystemViewCache>,
    unsold: Mutex<UnsoldCache>,
}

impl Exploration {
    /// Read the current system from the journals in `dir`.
    pub fn with_journals(&self, dir: PathBuf) -> Self {
        Exploration { journals: Some(dir), history: self.history.clone(), ..Default::default() }
    }

    /// Read earlier visits and the unsold data from the history database `db`.
    pub fn with_history(&self, db: PathBuf) -> Self {
        Exploration { journals: self.journals.clone(), history: Some(db), ..Default::default() }
    }

    /// The current system: this visit from the journals, earlier visits from the history (best
    /// effort: without one, this visit alone).
    fn system(&self) -> SystemFold {
        let Some(dir) = &self.journals else { return SystemFold::default() };
        let mut fold = self.view.lock().unwrap_or_else(|e| e.into_inner()).fold(dir);
        if let Some(h) =
            self.history.as_deref().filter(|db| db.is_file()).and_then(|db| History::open_read_only(db).ok())
        {
            h.add_earlier_visits(&mut fold);
        }
        fold
    }

    /// The latest commander's unsold data: `Ok(None)` without a history (or a commander in it).
    fn unsold(&self) -> Result<Option<astromech_exploration::Unsold>, Unreadable> {
        let Some(db) = &self.history else { return Ok(None) };
        let mut cache = self.unsold.lock().unwrap_or_else(|e| e.into_inner());
        cache.unsold_at(db, None).map(|u| u.map(|u| u.unsold)).map_err(|_| Unreadable)
    }
}

/// The spoken answer to an exploration question.
pub fn answer(ask: Ask, ex: &Exploration) -> String {
    match ask {
        Ask::SystemValue => words::system_value(&ex.system().view()),
        Ask::WorthMapping => words::worth_mapping(&ex.system().view()),
        Ask::Biology => {
            let fold = ex.system();
            words::biology(&astromech_exploration::biology::ranked(&fold), &fold.view())
        }
        Ask::Unsold => words::unsold(ex.unsold().as_ref().map(Option::as_ref).map_err(|e| *e)),
    }
}
