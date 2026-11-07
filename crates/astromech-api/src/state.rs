//! The command layer's state, built once per process and shared by every caller: the keybinds
//! [`Api`], the journal caches the window's pages poll, the process's one remote-data client,
//! the star map ([`GalaxyHolder`]), the neutron star file and the one-at-a-time flags of the long
//! jobs. Each piece has its own short-held lock, recovered if a handler panicked holding it
//! ([`crate::lock`]). A long job (a route search, a plot) holds a flag, not a lock, so other
//! callers aren't kept waiting.
//!
//! The folders come from a [`Folders`] source read on every call (the core implements it over
//! its live config), so a folder changed in onboarding or Settings applies at once: the keybinds
//! `Api` is rebuilt when the Bindings or game folder it was built from changes, and each journal
//! cache is dropped when the journal folder changes. The data folder is fixed for the process,
//! so the client, the star map and the neutron star file in it are never reopened.

use crate::lock;
use astromech_journal::{journal_version, JournalCache, JournalVersion};
use astromech_keybinds::api::Api;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// The star map preload's file name: the bundled `starmap.db`, which the installer puts next to
/// the app's executables.
pub const PRELOAD: &str = "starmap.db";

/// The bundled star map preload for a process running `exe`: the `starmap.db` next to it, if
/// there is one. Without one the star map opens with no preload, as a dev build's does.
pub fn preload_beside(exe: &Path) -> Option<PathBuf> {
    let p = exe.with_file_name(PRELOAD);
    p.is_file().then_some(p)
}

/// The local star map, opened on first use and then kept. A failed open is not remembered: the
/// next call tries again. The window's lookups and the session's voice lookups share one.
pub struct GalaxyHolder {
    open: Box<dyn Fn() -> anyhow::Result<astromech_galaxy::Galaxy> + Send + Sync>,
    slot: Mutex<Option<Arc<astromech_galaxy::Galaxy>>>,
}

impl GalaxyHolder {
    /// A star map that `open` opens when it is first asked for.
    pub fn new(open: impl Fn() -> anyhow::Result<astromech_galaxy::Galaxy> + Send + Sync + 'static) -> Self {
        Self { open: Box::new(open), slot: Mutex::new(None) }
    }

    /// The star map in `db` (seeded once from `preload`, if there is one), answering remote
    /// requests through `remote`. Opening it logs which database and preload it used, or why
    /// it couldn't.
    pub fn at(db: PathBuf, preload: Option<PathBuf>, remote: Arc<astromech_data::Client>) -> Self {
        Self::new(move || {
            let opened = astromech_galaxy::Galaxy::open(&db, preload.as_deref(), remote.clone());
            let from = preload.as_ref().map_or("no preload".into(), |p| format!("preload {}", p.display()));
            match &opened {
                Ok(_) => astromech_log::log(&format!("star map: opened {}, {from}", db.display())),
                Err(e) => astromech_log::log(&format!(
                    "star map: {} not opened ({from}), site lookups are off: {e:#}",
                    db.display()
                )),
            }
            opened
        })
    }

    /// A star map that is open already (a test's, over a store in memory).
    pub fn opened(galaxy: astromech_galaxy::Galaxy) -> Self {
        Self { open: Box::new(|| anyhow::bail!("opened already")), slot: Mutex::new(Some(Arc::new(galaxy))) }
    }

    /// The star map, opening it if it isn't open yet.
    pub fn get(&self) -> Result<Arc<astromech_galaxy::Galaxy>, String> {
        let mut g = lock(&self.slot);
        if let Some(x) = g.as_ref() {
            return Ok(x.clone());
        }
        let opened = Arc::new((self.open)().map_err(|e| format!("{e:#}"))?);
        *g = Some(opened.clone());
        Ok(opened)
    }
}

/// Holds one of the state's one-at-a-time flags while its job runs and clears it when the job
/// ends, however it ends.
pub(crate) struct Busy<'a>(&'a AtomicBool);

impl<'a> Busy<'a> {
    /// `None` when the flag is held already.
    pub(crate) fn take(flag: &'a AtomicBool) -> Option<Self> {
        (!flag.swap(true, Ordering::SeqCst)).then_some(Busy(flag))
    }
}

impl Drop for Busy<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

/// Opens a URL in the browser or a folder in the file manager (explorer.exe on Windows).
pub type Opener = Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>;

fn open_with_the_system(target: &str) -> Result<(), String> {
    #[cfg(windows)]
    let r = std::process::Command::new("explorer.exe").arg(target).spawn();
    #[cfg(not(windows))]
    let r = std::process::Command::new("xdg-open").arg(target).spawn();
    r.map(|_| ()).map_err(|e| e.to_string())
}

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
    /// The process's one remote-data client (Spansh, Ardent, EDSM).
    remote: Arc<astromech_data::Client>,
    galaxy: Arc<GalaxyHolder>,
    /// The neutron star file in the data folder, loaded on the first local neutron plot and kept
    /// (about 80 MB), and loaded again when a download replaces it.
    pub(crate) neutron: astromech_galaxy::neutron::Kept,
    /// Where the neutron star file is published.
    pub(crate) neutron_url: String,
    /// A trade route search is running (one Spansh job at a time, even if the page is left and
    /// opened again while the first still polls).
    pub(crate) route_busy: AtomicBool,
    /// A route is being plotted for the follower (one at a time, the same way).
    pub(crate) plot_busy: AtomicBool,
    /// How a route plot polls Spansh.
    pub(crate) plot_polling: astromech_data::spansh::Polling,
    /// Folders `open_external` may open (with what is under them), besides the data folder.
    openable: Vec<PathBuf>,
    opener: Opener,
}

impl State {
    /// `data_dir`: the core's data folder for this process. `remote`: the process's one
    /// remote-data client. `preload`: the star map preload ([`preload_beside`]). `folders`: read
    /// on every call.
    pub fn new(
        data_dir: PathBuf,
        remote: Arc<astromech_data::Client>,
        preload: Option<PathBuf>,
        folders: impl Fn() -> Folders + Send + Sync + 'static,
    ) -> Self {
        let galaxy = GalaxyHolder::at(data_dir.join("galaxy.db"), preload, remote.clone());
        Self {
            neutron: astromech_galaxy::neutron::Kept::new(&data_dir),
            neutron_url: astromech_galaxy::neutron::RELEASE_URL.into(),
            route_busy: AtomicBool::new(false),
            plot_busy: AtomicBool::new(false),
            plot_polling: Default::default(),
            openable: vec![],
            opener: Box::new(open_with_the_system),
            remote,
            galaxy: Arc::new(galaxy),
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

    /// Folders `open_external` may open besides the data folder (the core's app and log folders).
    pub fn openable(mut self, folders: Vec<PathBuf>) -> Self {
        self.openable = folders;
        self
    }

    /// Poll Spansh for a route plot this way instead of every few seconds (a test's recorded
    /// answers need no wait).
    pub fn plot_polling(mut self, polling: astromech_data::spansh::Polling) -> Self {
        self.plot_polling = polling;
        self
    }

    /// Open URLs and folders with `opener` instead of the system's (a test's).
    #[cfg(test)]
    pub(crate) fn with_opener(mut self, opener: impl Fn(&str) -> Result<(), String> + Send + Sync + 'static) -> Self {
        self.opener = Box::new(opener);
        self
    }

    /// Fetch the neutron star file from `url` instead of the `neutron` pre-release (a test's
    /// server).
    #[cfg(test)]
    pub(crate) fn with_neutron_url(mut self, url: &str) -> Self {
        self.neutron_url = url.into();
        self
    }

    /// The process's one remote-data client.
    pub fn remote(&self) -> &Arc<astromech_data::Client> {
        &self.remote
    }

    /// The star map, shared with the session's voice lookups.
    pub fn galaxy_holder(&self) -> &Arc<GalaxyHolder> {
        &self.galaxy
    }

    /// The star map, opened on first use.
    pub fn galaxy(&self) -> Result<Arc<astromech_galaxy::Galaxy>, String> {
        self.galaxy.get()
    }

    /// Open `target` if it is a link the app opens (the project's GitHub pages, Coriolis, EDSY,
    /// Spansh, Inara) or a folder under the data folder or another [`State::openable`] one;
    /// "not allowed" otherwise.
    pub fn open_external(&self, target: &str) -> Result<(), String> {
        let ok_url = astromech_engineering::links::is_openable_url(target);
        let ok_dir = {
            let p = Path::new(target);
            p.is_dir()
                && p.canonicalize().is_ok_and(|p| {
                    std::iter::once(&self.data_dir)
                        .chain(&self.openable)
                        .filter_map(|r| r.canonicalize().ok())
                        .any(|r| p.starts_with(r))
                })
        };
        if !ok_url && !ok_dir {
            return Err("not allowed".into());
        }
        (self.opener)(target)
    }

    /// The settings store, `astromech.sqlite` in the data folder (saved search filters).
    pub fn settings(&self) -> Result<astromech_store::Store, String> {
        std::fs::create_dir_all(&self.data_dir).map_err(|e| e.to_string())?;
        astromech_store::Store::open(&self.data_dir.join("astromech.sqlite")).map_err(|e| e.to_string())
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
pub(crate) mod tests {
    use super::*;
    use std::io::Write;

    /// A client whose every request fails, for tests that look nothing up.
    pub(crate) fn offline() -> Arc<astromech_data::Client> {
        struct Offline;
        impl astromech_data::Transport for Offline {
            fn send(
                &self,
                url: &str,
                _: &astromech_data::Request,
            ) -> Result<astromech_data::Response, astromech_data::TransportError> {
                Err(astromech_data::TransportError::Offline(format!("no network in this test: {url}")))
            }
        }
        Arc::new(
            astromech_data::Client::with_transport(astromech_data::Cache::in_memory().unwrap(), Arc::new(Offline))
                .limiter(astromech_data::Limiter::none()),
        )
    }

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
        let state = State::new(data, offline(), None, move || Folders { journal: j.clone(), ..Folders::default() });
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
        let state = State::new(d.path().join("data"), offline(), None, move || Folders {
            journal: lock(&n).clone(),
            ..Folders::default()
        });
        let a = state.materials().unwrap();
        *lock(&now) = two;
        let b = state.materials().unwrap();
        assert!(!Arc::ptr_eq(&a, &b));
        *lock(&now) = one;
        assert!(!Arc::ptr_eq(&a, &state.materials().unwrap()), "the first folder's cache was dropped");
    }

    #[test]
    fn the_preload_is_the_starmap_db_next_to_the_exe() {
        let d = tempfile::tempdir().unwrap();
        let exe = d.path().join("astromech-core.exe");
        assert_eq!(preload_beside(&exe), None, "none there: no preload, as the session opened it");
        std::fs::write(d.path().join("starmap.db"), "").unwrap();
        assert_eq!(preload_beside(&exe), Some(d.path().join("starmap.db")));
    }

    /// A star map that won't open is tried again on the next call, and kept once open.
    #[test]
    fn a_star_map_that_failed_to_open_is_opened_on_the_next_call() {
        let tries = Arc::new(Mutex::new(0));
        let t = tries.clone();
        let holder = GalaxyHolder::new(move || {
            let mut n = lock(&t);
            *n += 1;
            anyhow::ensure!(*n > 1, "the disk is busy");
            Ok(astromech_galaxy::Galaxy::with_remote(astromech_galaxy::Store::in_memory()?, offline()))
        });
        assert_eq!(holder.get().err().as_deref(), Some("the disk is busy"));
        let g = holder.get().unwrap();
        assert!(Arc::ptr_eq(&g, &holder.get().unwrap()));
        assert_eq!(*lock(&tries), 2);
    }

    /// Opening the star map names the database and the preload, or says there is none.
    #[test]
    fn opening_the_star_map_logs_its_database_and_preload() {
        crate::tests::test_log();
        let d = tempfile::tempdir().unwrap();
        let db = d.path().join("galaxy.db");
        GalaxyHolder::at(db.clone(), None, offline()).get().unwrap();
        let line = format!("star map: opened {}, no preload", db.display());
        assert!(crate::tests::logged().contains(&line), "{}", crate::tests::logged());

        let (db, pre) = (d.path().join("other.db"), d.path().join("starmap.db"));
        astromech_galaxy::Store::open(&pre).unwrap();
        GalaxyHolder::at(db.clone(), Some(pre.clone()), offline()).get().unwrap();
        let line = format!("star map: opened {}, preload {}", db.display(), pre.display());
        assert!(crate::tests::logged().contains(&line), "{}", crate::tests::logged());
    }
}
