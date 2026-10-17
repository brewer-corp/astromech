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

/// What the projects were replayed from: the journals and `Cargo.json`'s bytes (a small file,
/// compared whole so a change shows even where its mtime can't be read). A change to either
/// replays again.
type Key = (astromech_journal::JournalVersion, Option<Vec<u8>>);

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

    /// The journal folder the projects are read from.
    pub fn dir(&self) -> Option<&std::path::Path> {
        self.dir.as_deref()
    }

    /// Every project in the journals, being built first (none without a journal folder).
    pub fn get(&self) -> Arc<Vec<ProjectView>> {
        let Some(dir) = &self.dir else { return Arc::default() };
        let key = (astromech_journal::journal_version(dir), std::fs::read(dir.join("Cargo.json")).ok());
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
                return astromech_tools::lookup::NO_STAR_MAP.into();
            };
            let (Some(system), Some(pos)) = (game.system.clone(), game.star_pos) else {
                return astromech_tools::lookup::NO_POSITION.into();
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

#[cfg(test)]
mod tests {
    use super::*;

    const START: &str = r#"{"timestamp":"2026-10-16T10:00:00Z","event":"ColonisationConstructionDepot","MarketID":1,"ConstructionProgress":0.1,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$steel_name;","Name_Localised":"Steel","RequiredAmount":100,"ProvidedAmount":0,"Payment":1000}]}"#;
    const LATER: &str = r#"{"timestamp":"2026-10-16T10:05:00Z","event":"ColonisationConstructionDepot","MarketID":1,"ConstructionProgress":0.5,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$steel_name;","Name_Localised":"Steel","RequiredAmount":100,"ProvidedAmount":50,"Payment":1000}]}"#;

    fn provided(p: &Projects) -> Vec<i64> {
        p.get().iter().flat_map(|v| v.project.resources.iter().map(|r| r.provided)).collect()
    }

    /// The projects are replayed again when the journal grows or `Cargo.json` changes, and not
    /// otherwise (the cache is the same allocation).
    #[test]
    fn the_projects_are_replayed_when_the_journal_or_cargo_json_change() {
        let d = tempfile::tempdir().unwrap();
        let log = d.path().join("Journal.2026-10-16T100000.01.log");
        std::fs::write(&log, format!("{START}\n")).unwrap();
        let p = Projects::new(d.path().to_path_buf());

        let first = p.get();
        assert_eq!(provided(&p), [0]);
        assert!(Arc::ptr_eq(&first, &p.get()), "unchanged: served from the cache");

        std::fs::write(&log, format!("{START}\n{LATER}\n")).unwrap();
        assert_eq!(provided(&p), [50], "an appended line replays");

        let before = p.get();
        std::fs::write(d.path().join("Cargo.json"), r#"{"Inventory":[]}"#).unwrap();
        assert!(!Arc::ptr_eq(&before, &p.get()), "a new Cargo.json replays");
        let after = p.get();
        std::fs::write(d.path().join("Cargo.json"), r#"{"Inventory":[{"Name":"steel","Count":7}]}"#).unwrap();
        assert!(!Arc::ptr_eq(&after, &p.get()), "an edited Cargo.json replays");

        std::fs::write(d.path().join("Journal.2026-10-16T110000.01.log"), "").unwrap();
        let before = p.get();
        std::fs::write(d.path().join("Journal.2026-10-16T120000.01.log"), "").unwrap();
        assert!(!Arc::ptr_eq(&before, &p.get()), "a new journal file replays");
    }
}
