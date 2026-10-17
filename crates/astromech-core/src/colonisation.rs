//! Colonisation questions by voice ("how is the build going?", "what is left to deliver?",
//! "where can I buy steel for the depot?"): the projects come from the journals, replayed the
//! way the app's Colonisation page reads them (`astromech_colonisation::from_journal_dir`, with
//! the hold from `Cargo.json`); where to buy is the page's Ardent lookup
//! (`astromech_market::list::plan_search`) for that one commodity, from the commander's
//! system, with the page's remembered filters (`colonisation.buy`). Nothing is pressed.

use astromech_colonisation::ProjectView;
use astromech_galaxy::{Filters, Galaxy};
use astromech_gamestate::GameState;
use astromech_tools::colonisation::{self as words, Ask};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// What the projects were replayed from: the journal files (count, newest, its length) and
/// `Cargo.json`'s bytes. A change to any of them replays again.
type Key = (usize, Option<PathBuf>, u64, Option<Vec<u8>>);

/// The tracker's projects, replayed from the journal folder when it changes.
#[derive(Default)]
pub struct Projects {
    dir: Option<PathBuf>,
    cached: Mutex<Option<(Key, Arc<Vec<ProjectView>>)>>,
}

impl Projects {
    pub fn new(dir: PathBuf) -> Self {
        Projects { dir: Some(dir), cached: Mutex::default() }
    }

    /// Every project in the journals, being built first (none without a journal folder).
    pub fn get(&self) -> Arc<Vec<ProjectView>> {
        let Some(dir) = &self.dir else { return Arc::default() };
        let logs = astromech_journal::journal_logs(dir);
        let newest = logs.last().cloned();
        let len = newest.as_deref().and_then(|p| std::fs::metadata(p).ok()).map_or(0, |m| m.len());
        let key = (logs.len(), newest, len, std::fs::read(dir.join("Cargo.json")).ok());
        let mut cached = self.cached.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((k, v)) = cached.as_ref() {
            if *k == key {
                return v.clone();
            }
        }
        let v = Arc::new(astromech_colonisation::from_journal_dir(dir));
        *cached = Some((key, v.clone()));
        v
    }
}

/// The spoken answer to a colonisation question about the project docked at, else the one
/// last delivered to. `commodity` is the grammar slot's value (a catalogue name) for
/// [`Ask::Buy`].
pub fn answer(
    ask: Ask,
    projects: &[ProjectView],
    commodity: Option<&str>,
    lookup: Option<(&Galaxy, &Filters)>,
    game: &GameState,
    now_s: i64,
) -> String {
    let docked = game.station_market_id.filter(|_| game.docked);
    let Some(v) = words::pick(projects, docked) else {
        return words::NO_PROJECT.into();
    };
    match ask {
        Ask::Progress => words::progress(v),
        Ask::Remaining => words::remaining(v),
        Ask::Buy => {
            let need = match words::need(v, commodity.unwrap_or_default()) {
                Ok(n) => n,
                Err(why) => return why,
            };
            let Some((galaxy, filters)) = lookup else {
                return words::NO_STAR_MAP.into();
            };
            let (Some(system), Some(pos)) = (game.system.clone(), game.star_pos) else {
                return words::NO_POSITION.into();
            };
            let here = astromech_galaxy::Here { system, id64: None, pos };
            let range = game.max_jump_range.filter(|r| *r > 1.0).unwrap_or(20.0);
            let max_ly = filters.max_distance_ly.unwrap_or(astromech_market::trade::DEFAULT_DISTANCE_LY);
            match astromech_market::list::plan_search(
                galaxy,
                &here,
                range,
                std::slice::from_ref(&need),
                filters,
                12,
                now_s,
            ) {
                Ok(plan) => words::buy(&need, &plan, max_ly, now_s),
                Err(e) => e.to_string(),
            }
        }
    }
}
