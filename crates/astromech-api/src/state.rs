//! The command layer's state, built once per process and shared by every caller: the keybinds
//! [`Api`] and the journal caches the window's pages poll. Each piece has its own short-held
//! lock, recovered if a handler panicked holding it ([`crate::lock`]).
//!
//! The folders come from a [`Folders`] source read on every call (the core implements it over
//! its live config), so a folder changed in onboarding or Settings applies at once: the keybinds
//! `Api` is rebuilt when the Bindings or game folder it was built from changes, and each journal
//! cache is dropped when the journal folder changes. The data folder is fixed for the process.

use crate::lock;
use astromech_journal::{journal_version, JournalCache, JournalVersion};
use astromech_keybinds::api::Api;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// The folders the state reads, as configured now.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Folders {
    /// The game's journal folder; empty if there is none, which callers report as a folder not
    /// found.
    pub journal: PathBuf,
    /// The Bindings folder, if one is set (else the keybinds code finds the default).
    pub bindings: Option<PathBuf>,
    /// The game's install folder, if one is set.
    pub game: Option<PathBuf>,
}

/// Everything the Engineering screen (and the lookups after it) reads from the journals,
/// re-read when the journals change.
pub struct JournalState {
    pub fleet: astromech_engineering::Fleet,
    /// When the next ship or module transfer under way lands: after that the fleet above is stale
    /// though no journal line was written (astromech-4irh).
    pub fleet_next_arrival: Option<i64>,
    pub ledger: astromech_materials::Ledger,
    pub engineers: Vec<astromech_engineering::EngineerView>,
    /// Every ship and suit engineer with the commander's progress and what is left to unlock.
    pub unlocks: astromech_engineering::unlocks::Tree,
    pub here: Option<astromech_galaxy::Here>,
    /// Faction states seen on arrival in the newest journals (fresher than Spansh's).
    pub visits: Vec<astromech_galaxy::Visit>,
}

impl JournalState {
    pub fn build(dir: &Path) -> Self {
        let progress = astromech_engineering::engineers::builder_from_journal_dir(dir);
        let fleet = astromech_engineering::from_journal_dir_at(dir, &astromech_data::SystemClock);
        JournalState {
            fleet: fleet.fleet,
            fleet_next_arrival: fleet.next_arrival,
            ledger: astromech_materials::replay_journal_dir(dir),
            engineers: progress.build(),
            unlocks: astromech_engineering::unlocks::tree(&progress),
            here: astromech_galaxy::here_from_journal_dir(dir),
            visits: astromech_galaxy::visits_from_journal_dir(dir, astromech_galaxy::visits::RECENT_JOURNALS),
        }
    }

    /// Rebuilt when a journal event any reader of it looks at is appended, not on every append
    /// and not on file notifications or directory metadata (astromech-ztrb). A reader added to
    /// [`JournalState::build`] must add its pattern to `engineering_journal_events`
    /// (astromech-ldiz).
    pub fn cache() -> JournalCache<Self> {
        JournalCache::new(&astromech_galaxy::engineering_journal_events())
    }

    /// The cached state for `dir`, also rebuilt when a transfer under way lands on the clock.
    pub fn get(cache: &JournalCache<Self>, dir: &Path) -> Result<Arc<Self>, String> {
        if !dir.is_dir() {
            return Err(format!("journal folder not found: {}", dir.display()));
        }
        let clock = astromech_data::SystemClock;
        Ok(cache.get_fresh(
            dir,
            |st| astromech_engineering::Settled::current(st.fleet_next_arrival, &clock),
            || Self::build(dir),
        ))
    }
}

/// A cache for one journal folder, dropped and made afresh when the folder changes.
struct ForDir<C> {
    slot: Mutex<Option<(PathBuf, Arc<C>)>>,
    make: fn(&Path) -> C,
}

impl<C> ForDir<C> {
    fn new(make: fn(&Path) -> C) -> Self {
        Self { slot: Mutex::new(None), make }
    }

    fn get(&self, dir: &Path) -> Arc<C> {
        let mut g = lock(&self.slot);
        match &*g {
            Some((d, c)) if d == dir => c.clone(),
            _ => {
                let c = Arc::new((self.make)(dir));
                *g = Some((dir.to_path_buf(), c.clone()));
                c
            }
        }
    }
}

/// A value built from the journals, kept while their [`JournalVersion`] (and any extra key) is
/// unchanged.
type Versioned<K, T> = Mutex<Option<(K, Arc<T>)>>;

fn versioned<K: PartialEq, T>(cache: &Versioned<K, T>, key: K, build: impl FnOnce() -> T) -> Arc<T> {
    let mut g = lock(cache);
    if let Some((k, v)) = &*g {
        if *k == key {
            return v.clone();
        }
    }
    let v = Arc::new(build());
    *g = Some((key, v.clone()));
    v
}

/// The colonisation view's key: the journals and `Cargo.json`'s bytes (a small file, compared
/// whole so a change shows even where its mtime can't be read).
type ColonisationKey = (JournalVersion, Option<Vec<u8>>);

/// The keybinds `Api` and the folders it was built from.
struct Binds {
    from: (Option<PathBuf>, Option<PathBuf>),
    api: Api,
}

pub struct State {
    data_dir: PathBuf,
    folders: Box<dyn Fn() -> Folders + Send + Sync>,
    /// The connected controllers, keyboards and mice, read afresh for each `binds_state`.
    devices: Box<dyn Fn() -> astromech_keybinds::devices::Inventory + Send + Sync>,
    binds: Mutex<Option<Binds>>,
    materials: ForDir<Versioned<JournalVersion, astromech_materials::Inventory>>,
    odyssey: ForDir<Versioned<JournalVersion, astromech_odyssey::JournalState>>,
    powerplay: ForDir<Mutex<astromech_powerplay::Follower>>,
    engineering: ForDir<JournalCache<JournalState>>,
    exploration: ForDir<Mutex<astromech_exploration::SystemViewCache>>,
    /// Keyed by the history in the data folder, which is fixed, so it is never dropped.
    unsold: Mutex<astromech_history::UnsoldCache>,
    colonisation: ForDir<Versioned<ColonisationKey, Vec<astromech_colonisation::ProjectView>>>,
}

impl State {
    /// `data_dir`: the core's data folder for this process. `folders`: read on every call.
    pub fn new(data_dir: PathBuf, folders: impl Fn() -> Folders + Send + Sync + 'static) -> Self {
        Self {
            data_dir,
            folders: Box::new(folders),
            devices: Box::new(astromech_keybinds::devices::enumerate_live),
            binds: Mutex::new(None),
            materials: ForDir::new(|_| Mutex::new(None)),
            odyssey: ForDir::new(|_| Mutex::new(None)),
            powerplay: ForDir::new(|d| Mutex::new(astromech_powerplay::Follower::new(d))),
            engineering: ForDir::new(|_| JournalState::cache()),
            exploration: ForDir::new(|_| Mutex::new(Default::default())),
            unsold: Mutex::new(Default::default()),
            colonisation: ForDir::new(|_| Mutex::new(None)),
        }
    }

    /// Devices from `devices` instead of the live ones (a test's controllers).
    #[cfg(test)]
    pub(crate) fn with_devices(
        mut self,
        devices: impl Fn() -> astromech_keybinds::devices::Inventory + Send + Sync + 'static,
    ) -> Self {
        self.devices = Box::new(devices);
        self
    }

    /// The keybinds state with the devices connected now. The core runs from login for days,
    /// so the devices the `Api` was built with go stale; the window asks for this when a page
    /// loads, as it used to when the window opened.
    pub fn binds_state(&self) -> Result<astromech_keybinds::api::State, String> {
        self.binds(false, |a| {
            a.inventory = (self.devices)();
            Ok(a.state())
        })
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// The journal folder as configured now (it may not exist).
    pub fn journal_dir(&self) -> PathBuf {
        (self.folders)().journal
    }

    /// The journal folder, or an error naming it if it isn't there.
    fn journals(&self) -> Result<PathBuf, String> {
        let dir = self.journal_dir();
        if !dir.is_dir() {
            return Err(format!("journal folder not found: {}", dir.display()));
        }
        Ok(dir)
    }

    fn history(&self) -> PathBuf {
        self.data_dir.join("history.sqlite")
    }

    /// Run `f` on the keybinds `Api`, built on first use and again when the Bindings or game
    /// folder changed. `fresh` drops the one there is first (`binds_reset`).
    pub fn binds<T>(&self, fresh: bool, f: impl FnOnce(&mut Api) -> anyhow::Result<T>) -> Result<T, String> {
        let f_now = (self.folders)();
        let from = (f_now.bindings, f_now.game);
        let mut g = lock(&self.binds);
        if fresh || g.as_ref().is_some_and(|b| b.from != from) {
            *g = None;
        }
        if g.is_none() {
            let api = Api::discover(from.0.clone(), from.1.as_deref(), self.data_dir.clone())
                .map_err(|e| format!("{e:#}"))?;
            *g = Some(Binds { from, api });
        }
        f(&mut g.as_mut().expect("built above").api).map_err(|e| format!("{e:#}"))
    }

    /// Engineering materials from the journal, laid out like the in-game traders.
    pub fn materials(&self) -> Result<Arc<astromech_materials::Inventory>, String> {
        let dir = self.journals()?;
        // Not directory metadata: on Windows that misses appends to the open journal (astromech-txzq).
        let version = journal_version(&dir);
        Ok(versioned(&self.materials.get(&dir), version, || astromech_materials::from_journal_dir(&dir)))
    }

    /// The ShipLocker and Backpack (small files, read every call) and the on-foot engineer
    /// progress and owned gear from the journal, re-read when the journal changes.
    pub fn odyssey(&self) -> Result<(astromech_odyssey::Holdings, Arc<astromech_odyssey::JournalState>), String> {
        let dir = self.journals()?;
        let holdings = astromech_odyssey::Holdings::from_journal_dir(&dir);
        let version = journal_version(&dir);
        let state =
            versioned(&self.odyssey.get(&dir), version, || astromech_odyssey::journal_state(&dir, &astromech_log::log));
        Ok((holdings, state))
    }

    /// The Powerplay journal, followed: each read takes only the lines added since the last one
    /// (the page polls every 2 s while the game appends).
    pub fn powerplay(&self) -> Result<astromech_powerplay::Journal, String> {
        let dir = self.journals()?;
        let follower = self.powerplay.get(&dir);
        let mut f = lock(&follower);
        Ok(f.refresh().clone())
    }

    /// The fleet, materials, engineers and position from the journals.
    pub fn engineering(&self) -> Result<Arc<JournalState>, String> {
        let dir = self.journal_dir();
        JournalState::get(&self.engineering.get(&dir), &dir)
    }

    /// The current system's bodies as scanned, with their estimated values, from the newest
    /// journals, with the bodies scanned on earlier visits and the commander's landings on them
    /// from the history (when there is one).
    pub fn exploration_system(&self) -> astromech_exploration::SystemView {
        let dir = self.journal_dir();
        let live = lock(&self.exploration.get(&dir)).fold(&dir);
        // The history adds the bodies scanned on earlier visits and the landings on every poll
        // (the cache holds only the current visit's fold); without it (best effort) the view is
        // the live one.
        match astromech_history::History::open_read_only(&self.history()) {
            Ok(h) => h.system_view(live),
            Err(_) => live.view(),
        }
    }

    /// The latest commander's unsold cartographic and organic data from the history; `None`
    /// before the first commander or without a history yet.
    pub fn exploration_unsold(&self) -> Result<Option<astromech_history::UnsoldData>, String> {
        lock(&self.unsold).unsold_at(&self.history(), None).map_err(|e| e.to_string())
    }

    /// Colonisation projects from the journals, being built first, each with what is left to
    /// buy; replayed only when the journals or `Cargo.json` change.
    pub fn colonisation(&self) -> Result<Arc<Vec<astromech_colonisation::ProjectView>>, String> {
        let dir = self.journals()?;
        let key = (journal_version(&dir), std::fs::read(dir.join("Cargo.json")).ok());
        Ok(versioned(&self.colonisation.get(&dir), key, || astromech_colonisation::from_journal_dir(&dir)))
    }

    /// The history the core keeps (`history.sqlite`, read only).
    pub fn open_history(&self) -> Result<astromech_history::History, String> {
        let db = self.history();
        if !db.is_file() {
            return Err("no journal history yet: the core builds it from the journals once it runs".into());
        }
        astromech_history::History::open_read_only(&db).map_err(|e| e.to_string())
    }

    /// How often the Exploration page's journals were parsed (for tests).
    #[cfg(test)]
    fn exploration_parses(&self) -> usize {
        lock(&self.exploration.get(&self.journal_dir())).parses()
    }
}

/// Ship engineers: where they are, access and rank, and where the commander is.
#[derive(Serialize)]
pub struct Engineers {
    pub engineers: Vec<astromech_engineering::EngineerView>,
    pub here: Option<astromech_galaxy::Here>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// A journal folder with one journal and a state reading it.
    struct World {
        _d: tempfile::TempDir,
        journals: PathBuf,
        state: State,
    }

    const START: &str = r#"{ "timestamp":"2026-11-01T10:00:00Z", "event":"Fileheader", "part":1, "gameversion":"4.1.0", "build":"r1" }
{ "timestamp":"2026-11-01T10:00:01Z", "event":"LoadGame", "Commander":"Test", "FID":"F1", "Ship":"sidewinder", "ShipID":1, "Credits":1000 }
"#;

    fn world() -> World {
        let d = tempfile::tempdir().unwrap();
        let journals = d.path().join("journals");
        std::fs::create_dir_all(&journals).unwrap();
        std::fs::write(journals.join("Journal.2026-11-01T100000.01.log"), START).unwrap();
        let (j, data) = (journals.clone(), d.path().join("data"));
        let state = State::new(data, move || Folders { journal: j.clone(), ..Folders::default() });
        World { _d: d, journals, state }
    }

    impl World {
        fn append(&self, line: &str) {
            let mut f = std::fs::OpenOptions::new()
                .append(true)
                .open(self.journals.join("Journal.2026-11-01T100000.01.log"))
                .unwrap();
            writeln!(f, "{line}").unwrap();
        }
    }

    const MUSIC: &str = r#"{ "timestamp":"2026-11-01T10:01:00Z", "event":"Music", "MusicTrack":"NoTrack" }"#;
    const JUMP: &str = r#"{ "timestamp":"2026-11-01T10:02:00Z", "event":"FSDJump", "StarSystem":"Sol", "SystemAddress":10477373803, "StarPos":[0.0,0.0,0.0] }"#;

    #[test]
    fn materials_are_kept_until_the_journal_grows() {
        let w = world();
        let a = w.state.materials().unwrap();
        assert!(Arc::ptr_eq(&a, &w.state.materials().unwrap()));
        w.append(r#"{ "timestamp":"2026-11-01T10:01:00Z", "event":"MaterialCollected", "Category":"Raw", "Name":"iron", "Count":3 }"#);
        let b = w.state.materials().unwrap();
        assert!(!Arc::ptr_eq(&a, &b));
        assert!(Arc::ptr_eq(&b, &w.state.materials().unwrap()));
    }

    #[test]
    fn odyssey_state_is_kept_until_the_journal_grows() {
        let w = world();
        let (_, a) = w.state.odyssey().unwrap();
        assert!(Arc::ptr_eq(&a, &w.state.odyssey().unwrap().1));
        w.append(MUSIC);
        assert!(!Arc::ptr_eq(&a, &w.state.odyssey().unwrap().1));
    }

    #[test]
    fn powerplay_follows_what_the_journal_adds() {
        let w = world();
        let before = w.state.powerplay().unwrap();
        assert_eq!(before.standing.power, None);
        assert_eq!(before, w.state.powerplay().unwrap());
        w.append(r#"{ "timestamp":"2026-11-01T10:01:00Z", "event":"Powerplay", "Power":"Zachary Hudson", "Rank":2, "Merits":120, "TimePledged":1000 }"#);
        assert_eq!(w.state.powerplay().unwrap().standing.power.as_deref(), Some("Zachary Hudson"));
    }

    #[test]
    fn the_engineering_state_is_rebuilt_only_for_an_event_it_reads() {
        let w = world();
        let a = w.state.engineering().unwrap();
        w.append(MUSIC);
        assert!(Arc::ptr_eq(&a, &w.state.engineering().unwrap()));
        w.append(JUMP);
        let b = w.state.engineering().unwrap();
        assert!(!Arc::ptr_eq(&a, &b));
        assert_eq!(b.here.as_ref().map(|h| h.system.as_str()), Some("Sol"));
    }

    #[test]
    fn the_system_view_is_folded_again_only_after_an_append() {
        let w = world();
        w.state.exploration_system();
        w.state.exploration_system();
        assert_eq!(w.state.exploration_parses(), 1);
        w.append(JUMP);
        w.state.exploration_system();
        assert_eq!(w.state.exploration_parses(), 2);
    }

    #[test]
    fn colonisation_is_kept_until_the_journal_or_the_cargo_changes() {
        let w = world();
        let a = w.state.colonisation().unwrap();
        assert!(Arc::ptr_eq(&a, &w.state.colonisation().unwrap()));
        w.append(MUSIC);
        let b = w.state.colonisation().unwrap();
        assert!(!Arc::ptr_eq(&a, &b));
        std::fs::write(w.journals.join("Cargo.json"), r#"{"Vessel":"Ship","Count":0,"Inventory":[]}"#).unwrap();
        assert!(!Arc::ptr_eq(&b, &w.state.colonisation().unwrap()));
    }

    #[test]
    fn a_cache_is_dropped_when_the_journal_folder_changes() {
        let d = tempfile::tempdir().unwrap();
        let (one, two) = (d.path().join("one"), d.path().join("two"));
        for dir in [&one, &two] {
            std::fs::create_dir_all(dir).unwrap();
            std::fs::write(dir.join("Journal.2026-11-01T100000.01.log"), START).unwrap();
        }
        let now = Arc::new(Mutex::new(one.clone()));
        let n = now.clone();
        let state =
            State::new(d.path().join("data"), move || Folders { journal: lock(&n).clone(), ..Folders::default() });
        let a = state.materials().unwrap();
        *lock(&now) = two;
        let b = state.materials().unwrap();
        assert!(!Arc::ptr_eq(&a, &b));
        *lock(&now) = one;
        assert!(!Arc::ptr_eq(&a, &state.materials().unwrap()), "the first folder's cache was dropped");
    }
}
