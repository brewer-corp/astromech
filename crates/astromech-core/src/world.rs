//! World mode: `astromech-core --world <dir>` runs the real core with no window or tray against
//! a world, a directory that stands in for the PC around the core (decision astromech-e1a5).
//! Tests, the dev bridge and the journey driver start the binary this way and talk to it over
//! the control API.
//!
//! # A world on disk
//!
//! - `astromech.toml`: the config. Its folders are replaced by the temp copy's (below); every
//!   other setting is the world's. Missing: the defaults.
//! - `game/`: the journal folder's seed (journals, Status.json and the other companion files).
//! - `bindings/`: the Bindings folder's seed (`.binds` files).
//! - `secrets.json`: optional, the secrets the core finds saved, by name, as
//!   `{"edsm-api-key": "…"}`.
//!
//! `net.json` (the network recording) and `world.jsonl` (the steps) are the format's too; their
//! loaders come with the recorded-network and script children of astromech-vucf.
//!
//! # What world mode runs
//!
//! [`run`] copies the world into a fresh temp directory, so the world itself is never changed,
//! and builds the core's [`Boundaries`] there:
//!
//! - `world/`: the copy. `world/astromech.toml` is the config the core loads, rewritten to name
//!   `data/`, `models/`, `world/game/`, `world/bindings/` and `elite/` (the game's install
//!   folder) as its folders;
//! - `exe/` and `local/`: the exe folder and the per-user folder, so the app folder (with
//!   `core.json`) is `local/astromech/data` and its logs are under it. There is no user profile,
//!   so nothing falls back to the user's own folders.
//!
//! The config also turns EDDN and EDSM uploading off, whatever the world says, until the
//! recorded network answers them (astromech-xk0n): their uploaders send over real HTTP.
//!
//! The fakes: the process list is the world's (empty until a step starts the game); the opener,
//! clipboard, notifier, Start with Windows and the app window record what reaches them and touch
//! nothing; the secrets are in memory, seeded from `secrets.json`; every remote-data lookup is
//! refused and recorded as unrecorded; the clock is a manual one ([`World::clock`]), started at
//! the real time and moved only by the script, which the remote-data client shares. Then it calls [`app::assemble`] and [`app::start`], as the installed
//! app does, but none of what only [`app::run`] adds: no instance lock, no Start with Windows
//! default, no update thread, no window and no tray.
//!
//! Once the control API is up, the binary prints one line on stdout,
//! `{"control": {"port": …, "token": …}, "app_dir": "…"}`, its callers' only way to find it
//! ([`astromech_api::control::call_to`]). It runs until `quit`, or until its stdin reaches end of
//! file (the process that started it has gone, even if killed outright), then removes the temp
//! directory and exits 0. A caller keeps its stdin open: a pipe it holds.
//!
//! # Observations
//!
//! What reaches a fake is recorded in order ([`Observed`]), each as a JSON object with its
//! `seq` (from 0) and `kind`:
//!
//! - `open {target}`: a URL or folder opened; `reveal {path}`: a file shown in the file
//!   manager; `edit {path}`: a file opened in the editor (`open_config`);
//! - `clipboard {text}`; `notify {text}`;
//! - `autostart {on, command}`: Start with Windows set, with the command line it would run
//!   (null when off);
//! - `window {route}`: the app window opened (`open_window`), at a page or null;
//! - `http {url, path, unrecorded}`: a remote-data request.
//!
//! Two `local` commands read them, registered only in world mode (decision astromech-unh9: the
//! window's origin is refused):
//!
//! - `world_observed {since?}`: `{observed: [...], next}`, the observations from `seq` `since`
//!   (default 0), and the `seq` the next one will have;
//! - `world_wait {match, since?}`: the first observation from `since` whose fields include every
//!   field of `match` (`{"kind": "edit"}`), waiting for it if there is none yet. The observation
//!   itself wakes the wait; if none arrives within [`WAIT_GUARD`], the call fails.

use crate::app::{self, Assembled, Boundaries, Opener};
use crate::logging::log;
use anyhow::{Context, Result};
use astromech_api::{Exposure, Recover, Registry};
use astromech_proto::ManualClock;
use astromech_store::secrets::{MemorySecrets, Secrets};
use astromech_store::Paths;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use ts_rs::TS;

/// How long `world_wait` waits for a match before it fails the call: a guard against a test
/// hanging, never how a test decides anything.
pub const WAIT_GUARD: Duration = Duration::from_secs(30);

/// What reached the world's fakes, in order.
#[derive(Default)]
pub struct Observed {
    log: Mutex<Vec<Value>>,
    added: Condvar,
    /// Told the log's length each time a wait parks, still holding the log's lock.
    #[cfg(test)]
    parking: Mutex<Option<std::sync::mpsc::Sender<usize>>>,
}

impl Observed {
    /// Record one observation: `fields` (an object) with its `seq` and `kind`.
    pub fn record(&self, kind: &str, fields: Value) {
        let mut log = self.log.lock().recover();
        let mut o = match fields {
            Value::Object(m) => m,
            _ => Map::new(),
        };
        o.insert("seq".into(), json!(log.len()));
        o.insert("kind".into(), json!(kind));
        log.push(Value::Object(o));
        self.added.notify_all();
    }

    /// The observations from `seq` `since`, and the `seq` the next one will have.
    pub fn since(&self, since: usize) -> (Vec<Value>, usize) {
        let log = self.log.lock().recover();
        (log.get(since..).unwrap_or_default().to_vec(), log.len())
    }

    /// The first observation from `since` with every field of `pattern`, waiting for it to be
    /// recorded; an error if none is within `guard`.
    pub fn wait(&self, pattern: &Map<String, Value>, since: usize, guard: Duration) -> Result<Value, String> {
        // The guard is real time: it only stops a stuck call, and decides nothing in a world.
        let deadline = Instant::now() + guard;
        let mut log = self.log.lock().recover();
        let mut from = since;
        loop {
            if let Some(o) = log.iter().skip(from).find(|o| matches(o, pattern)) {
                return Ok(o.clone());
            }
            from = from.max(log.len());
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Err(format!(
                    "world_wait: nothing matching {} was observed within {} s (from seq {since}; {} observed)",
                    Value::Object(pattern.clone()),
                    guard.as_secs_f32(),
                    log.len()
                ));
            }
            #[cfg(test)]
            if let Some(tx) = self.parking.lock().recover().as_ref() {
                let _ = tx.send(log.len());
            }
            log = self.added.wait_timeout(log, left).recover().0;
        }
    }
}

fn matches(o: &Value, pattern: &Map<String, Value>) -> bool {
    pattern.iter().all(|(k, v)| o.get(k) == Some(v))
}

/// The opener: records what it is asked to open, and opens nothing.
struct WorldOpener(Arc<Observed>);

impl Opener for WorldOpener {
    fn open(&self, target: &str) -> Result<(), String> {
        self.0.record("open", json!({ "target": target }));
        Ok(())
    }
    fn reveal(&self, path: &Path) {
        self.0.record("reveal", json!({ "path": path }));
    }
    fn edit(&self, path: &Path) {
        self.0.record("edit", json!({ "path": path }));
    }
}

/// The platform's notifications, recorded.
struct WorldNotifier(Arc<Observed>);

impl crate::callouts::Notifier for WorldNotifier {
    fn notify(&self, text: &str) {
        self.0.record("notify", json!({ "text": text }));
    }
}

/// Start with Windows: the world's own Run value, recorded when set.
struct WorldAutostart {
    observed: Arc<Observed>,
    run: Mutex<Option<String>>,
}

impl crate::autostart::Autostart for WorldAutostart {
    fn get(&self) -> Option<String> {
        self.run.lock().recover().clone()
    }
    fn set(&self, on: bool, exe: &Path) -> Result<(), String> {
        let command = on.then(|| crate::autostart::command_line(exe));
        *self.run.lock().recover() = command.clone();
        self.observed.record("autostart", json!({ "on": on, "command": command }));
        Ok(())
    }
}

/// The remote-data transport: refuses every request and records it as unrecorded.
struct Unrecorded(Arc<Observed>);

impl astromech_data::Transport for Unrecorded {
    fn send(
        &self,
        url: &str,
        req: &astromech_data::Request,
    ) -> Result<astromech_data::Response, astromech_data::TransportError> {
        self.0.record("http", json!({ "url": url, "path": req.path, "unrecorded": true }));
        Err(astromech_data::TransportError::Offline(format!("this world has no recording for {url}{}", req.path)))
    }
}

/// A world, copied into its temp directory, and the state its fakes share.
pub struct World {
    dir: tempfile::TempDir,
    observed: Arc<Observed>,
    /// The running processes' exe names the game watch sees.
    processes: Arc<Mutex<Vec<String>>>,
    secrets: Vec<(String, String)>,
    /// The core's clock: time moves only when the script advances it (astromech-i1ev).
    clock: Arc<ManualClock>,
}

impl World {
    /// Copy the world in `src` to a fresh temp directory, and point its config's folders there.
    pub fn load(src: &Path) -> Result<Self> {
        anyhow::ensure!(src.is_dir(), "no world folder at {}", src.display());
        let dir =
            tempfile::Builder::new().prefix("astromech-world-").tempdir().context("making the world's temp folder")?;
        // It starts at the real time, so the world's seed files are as old as they are on disk.
        #[allow(clippy::disallowed_methods)]
        let clock = Arc::new(ManualClock::new(std::time::SystemTime::now()));
        let w = Self { dir, observed: Default::default(), processes: Default::default(), secrets: vec![], clock };
        copy_dir(src, &w.world()).with_context(|| format!("copying the world {}", src.display()))?;
        let secrets = match std::fs::read_to_string(w.world().join("secrets.json")) {
            Ok(t) => serde_json::from_str::<std::collections::BTreeMap<String, String>>(&t)
                .context("secrets.json: expected {\"name\": \"value\", …}")?
                .into_iter()
                .collect(),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => vec![],
            Err(e) => return Err(e).context("reading secrets.json"),
        };
        for d in ["game", "bindings"] {
            std::fs::create_dir_all(w.world().join(d))?;
        }
        for d in ["exe", "local", "data", "models", "elite"] {
            std::fs::create_dir_all(w.root().join(d))?;
        }
        w.write_config()?;
        Ok(Self { secrets, ..w })
    }

    /// The core's clock, which the script's `advance` moves.
    pub fn clock(&self) -> &Arc<ManualClock> {
        &self.clock
    }

    /// The temp directory.
    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    /// The world's copy in it.
    fn world(&self) -> PathBuf {
        self.root().join("world")
    }

    /// The config the core loads.
    pub fn config_path(&self) -> PathBuf {
        self.world().join(astromech_store::paths::CONFIG_FILE)
    }

    /// The world's config with every folder under the temp directory.
    fn write_config(&self) -> Result<()> {
        let p = self.config_path();
        let text = match std::fs::read_to_string(&p) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(e).context("reading the world's astromech.toml"),
        };
        let mut doc: toml_edit::DocumentMut = text.parse().context("the world's astromech.toml")?;
        let path = |p: PathBuf| toml_edit::value(p.display().to_string());
        doc["data_dir"] = path(self.root().join("data"));
        for t in ["game", "voice", "eddn", "edsm"] {
            if !doc.contains_key(t) {
                doc[t] = toml_edit::table();
            }
        }
        doc["game"]["journal_dir"] = path(self.world().join("game"));
        doc["game"]["bindings_dir"] = path(self.world().join("bindings"));
        doc["game"]["game_dir"] = path(self.root().join("elite"));
        doc["voice"]["models_dir"] = path(self.root().join("models"));
        doc["eddn"]["enabled"] = toml_edit::value(false);
        doc["edsm"]["enabled"] = toml_edit::value(false);
        std::fs::write(&p, doc.to_string()).context("writing the world's config")
    }

    /// The core's boundaries in this world: its folders under the temp directory, and the fakes.
    pub fn boundaries(&self) -> Boundaries {
        let local = self.root().join("local");
        let paths = Paths::new(self.root().join("exe"), move |k| (k == "LOCALAPPDATA").then(|| local.clone().into()));
        let secrets = MemorySecrets::default();
        for (k, v) in &self.secrets {
            let _ = secrets.set(k, Some(v));
        }
        let o = self.observed.clone();
        let remote = Arc::new(
            astromech_data::Client::with_transport(
                astromech_data::Cache::in_memory().expect("an in-memory cache"),
                Arc::new(Unrecorded(o.clone())),
            )
            .limiter(astromech_data::Limiter::none())
            .clock(self.clock.clone()),
        );
        let processes = self.processes.clone();
        let (clip, commands) = (o.clone(), o.clone());
        Boundaries {
            paths,
            processes: Arc::new(move || processes.lock().recover().clone()),
            opener: Arc::new(WorldOpener(o.clone())),
            clipboard: Arc::new(move |t: &str| {
                clip.record("clipboard", json!({ "text": t }));
                Ok(())
            }),
            notifier: Arc::new(WorldNotifier(o.clone())),
            secrets: Arc::new(secrets),
            autostart: Arc::new(WorldAutostart { observed: o.clone(), run: Mutex::new(None) }),
            window: Arc::new(move |route: Option<&str>| {
                o.record("window", json!({ "route": route }));
                Ok(())
            }),
            remote: Arc::new(move |_: &Path| remote.clone()),
            more_commands: Some(Arc::new(move |_, reg| register(&commands, reg))),
            clock: self.clock.clone(),
        }
    }
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let dst = to.join(e.file_name());
        if e.file_type()?.is_dir() {
            copy_dir(&e.path(), &dst)?;
        } else {
            std::fs::copy(e.path(), dst)?;
        }
    }
    Ok(())
}

#[derive(Deserialize, TS)]
struct Since {
    #[serde(default)]
    since: usize,
}

#[derive(Deserialize, TS)]
struct WaitFor {
    #[serde(rename = "match")]
    pattern: Map<String, Value>,
    #[serde(default)]
    since: usize,
}

/// `world_observed` and `world_wait`, both `local`.
fn register(observed: &Arc<Observed>, reg: &mut Registry) {
    let o = observed.clone();
    reg.add_typed("world_observed", Exposure::Local, move |a: Option<Since>, _| {
        let (observed, next) = o.since(a.map_or(0, |a| a.since));
        Ok(json!({ "observed": observed, "next": next }))
    });
    let o = observed.clone();
    reg.add_typed("world_wait", Exposure::Local, move |a: WaitFor, _| o.wait(&a.pattern, a.since, WAIT_GUARD));
}

/// Run the core in the world at `src` until `quit`. Returns the exit code.
pub fn run(src: &Path, session: crate::session::Opts) -> i32 {
    match serve(src, session) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("world mode: {e:#}");
            1
        }
    }
}

fn serve(src: &Path, session: crate::session::Opts) -> Result<()> {
    let world = World::load(src)?;
    let b = world.boundaries();
    let app_dir = b.paths.app_dir().to_path_buf();
    crate::logging::init(&b.paths.log_dir(), false);
    log(&format!("world mode: {} in {} (pid {})", src.display(), world.root().display(), std::process::id()));
    let opts = app::Opts { config: Some(world.config_path()), session, ..Default::default() };
    let Assembled { core, ui } = app::assemble(&opts, b);
    let started = app::start(&core);
    let server = started.server.as_ref().context("the control API didn't start")?;
    let line = json!({ "control": server.endpoint.contact(), "app_dir": app_dir });
    {
        let mut out = std::io::stdout().lock();
        writeln!(out, "{line}")?;
        out.flush()?;
    }
    // Quit when the starter goes: its end of stdin closes, even when it is killed.
    let ui_tx = core.ui.lock().recover().clone();
    std::thread::Builder::new().name("world-stdin".into()).spawn(move || {
        let _ = std::io::copy(&mut std::io::stdin().lock(), &mut std::io::sink());
        log("world mode: stdin closed");
        let _ = ui_tx.send(app::Ui::Quit);
    })?;
    app::headless(&ui);
    log("world mode: quitting");
    core.deactivate("quit");
    drop(started);
    let root = world.root().to_path_buf();
    if let Err(e) = world.dir.close() {
        // On Windows the core's open files (its databases, its log) keep the folder.
        eprintln!("world mode: couldn't remove {}: {e}", root.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pattern(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn observations_are_numbered_in_order_and_read_from_a_seq() {
        let o = Observed::default();
        o.record("open", json!({"target": "https://example.org"}));
        o.record("clipboard", json!({"text": "Sol"}));
        let (all, next) = o.since(0);
        assert_eq!(next, 2);
        assert_eq!(all[0], json!({"seq": 0, "kind": "open", "target": "https://example.org"}));
        assert_eq!(all[1], json!({"seq": 1, "kind": "clipboard", "text": "Sol"}));
        assert_eq!(o.since(1).0, vec![all[1].clone()]);
        assert_eq!(o.since(5), (vec![], 2));
    }

    #[test]
    fn a_wait_finds_a_past_match_from_its_seq_and_skips_earlier_ones() {
        let o = Observed::default();
        o.record("clipboard", json!({"text": "A"}));
        o.record("clipboard", json!({"text": "B"}));
        let got = o.wait(&pattern(json!({"kind": "clipboard"})), 1, Duration::ZERO).unwrap();
        assert_eq!(got["text"], "B");
        let e = o.wait(&pattern(json!({"kind": "clipboard", "text": "A"})), 1, Duration::ZERO).unwrap_err();
        assert!(e.contains("nothing matching") && e.contains("2 observed"), "{e}");
    }

    /// The waiter tells the test it is parking while it holds the log's lock, so each record
    /// below happens while it is parked in the wait, and only the recording can wake it: its
    /// guard is far longer than the test.
    #[test]
    fn a_wait_is_woken_by_the_observation() {
        let o = Arc::new(Observed::default());
        let (tx, parked) = std::sync::mpsc::channel();
        *o.parking.lock().unwrap() = Some(tx);
        let w = o.clone();
        let waiter =
            std::thread::spawn(move || w.wait(&pattern(json!({"kind": "notify"})), 0, Duration::from_secs(600)));
        assert_eq!(parked.recv().unwrap(), 0);
        o.record("clipboard", json!({"text": "not this"}));
        // Woken by it, it didn't match, and it parks again.
        assert_eq!(parked.recv().unwrap(), 1);
        o.record("notify", json!({"text": "Fuel low"}));
        assert_eq!(waiter.join().unwrap().unwrap(), json!({"seq": 1, "kind": "notify", "text": "Fuel low"}));
    }

    #[test]
    fn a_wait_with_no_match_fails_at_its_guard() {
        let o = Observed::default();
        o.record("open", json!({"target": "x"}));
        let e = o.wait(&pattern(json!({"kind": "edit"})), 0, Duration::from_millis(1)).unwrap_err();
        assert!(e.starts_with("world_wait: nothing matching {\"kind\":\"edit\"}"), "{e}");
    }

    #[test]
    fn a_loaded_world_names_only_its_temp_folders_and_leaves_the_world_alone() {
        let src = tempfile::tempdir().unwrap();
        std::fs::write(
            src.path().join("astromech.toml"),
            "data_dir = '/elsewhere'\n[eddn]\nenabled = true\n[voice]\ntts = false\n",
        )
        .unwrap();
        std::fs::create_dir(src.path().join("game")).unwrap();
        std::fs::write(src.path().join("game").join("Journal.2026-11-07T120000.01.log"), "{}\n").unwrap();
        std::fs::write(src.path().join("secrets.json"), r#"{"edsm-api-key": "k3y"}"#).unwrap();
        let w = World::load(src.path()).unwrap();
        let root = w.root().to_path_buf();
        let cfg = crate::config::Config::load(&w.config_path()).unwrap();
        let b = w.boundaries();
        assert_eq!(cfg.data_dir(&b.paths), root.join("data"));
        assert_eq!(cfg.journal_dir(&b.paths), Some(root.join("world").join("game")));
        assert_eq!(cfg.bindings_dir(), Some(root.join("world").join("bindings")));
        assert_eq!(cfg.game_dir(), Some(root.join("elite")));
        assert_eq!(cfg.models_dir(&b.paths), root.join("models"));
        assert!(!cfg.eddn.enabled && !cfg.edsm.enabled, "uploading is off whatever the world says");
        assert!(!cfg.voice.tts, "the world's other settings stay");
        assert!(b.paths.app_dir().starts_with(&root) && b.paths.log_dir().starts_with(&root));
        assert!(b.paths.exe_dir().starts_with(&root) && !b.paths.portable());
        assert!(root.join("world/game/Journal.2026-11-07T120000.01.log").is_file());
        assert_eq!(b.secrets.get(astromech_eddn::edsm::KEY_SECRET), Ok(Some("k3y".into())));
        // The world itself is as it was.
        let src_cfg = std::fs::read_to_string(src.path().join("astromech.toml")).unwrap();
        assert_eq!(src_cfg, "data_dir = '/elsewhere'\n[eddn]\nenabled = true\n[voice]\ntts = false\n");
        assert!(!src.path().join("bindings").exists());
        drop(b);
        drop(w);
        assert!(!root.exists(), "the temp folder goes with the world");
    }

    #[test]
    fn the_fakes_record_what_reaches_them() {
        let src = tempfile::tempdir().unwrap();
        let w = World::load(src.path()).unwrap();
        let b = w.boundaries();
        b.opener.open("https://www.edsm.net").unwrap();
        b.opener.reveal(Path::new("diag.zip"));
        b.opener.edit(Path::new("astromech.toml"));
        (b.clipboard)("Colonia").unwrap();
        b.notifier.notify("Fuel low");
        assert_eq!(b.autostart.get(), None);
        b.autostart.set(true, Path::new("astromech-core")).unwrap();
        assert_eq!(b.autostart.get(), Some(crate::autostart::command_line(Path::new("astromech-core"))));
        b.autostart.set(false, Path::new("astromech-core")).unwrap();
        (b.window)(Some("settings")).unwrap();
        assert!((b.processes)().is_empty());
        let client = (b.remote)(Path::new("unused"));
        assert!(client.send(astromech_data::Service::Spansh, &astromech_data::Request::get("/api/x")).is_err());
        let kinds: Vec<Value> = w.observed.since(0).0.iter().map(|o| o["kind"].clone()).collect();
        assert_eq!(
            kinds,
            ["open", "reveal", "edit", "clipboard", "notify", "autostart", "autostart", "window", "http"]
        );
        let (all, _) = w.observed.since(0);
        assert_eq!(all[5]["on"], true);
        assert_eq!(all[6], json!({"seq": 6, "kind": "autostart", "on": false, "command": null}));
        assert_eq!(all[7]["route"], "settings");
        assert_eq!(all[8]["unrecorded"], true);
    }
}
