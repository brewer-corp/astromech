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
//! - `world.jsonl`: optional, the steps (below).
//! - `net.json`: optional, the network recording: the answer to every HTTP request the core
//!   makes, its faults, and what happens to a request it doesn't hold. The format is in
//!   [`astromech_data::replay`]. Missing: every request is refused.
//! - Payloads that `file` steps and `net.json`'s `from` name by a path relative to the world
//!   (a recorded body may be an existing fixture, `../../crates/astromech-data/tests/fixtures/…`).
//!
//! # Steps: `world.jsonl`
//!
//! One JSON object per line; blank lines are skipped. The first line is the header,
//! `{"start": "2026-11-07T12:00:00Z"}`: the clock's UTC time before any step (with no
//! `world.jsonl`, the clock starts at the real time). Every later line is one step, named by its
//! kind's key:
//!
//! - `{"journal": <line>, "file"?: "Journal.<stamp>.<part>.log"}`: append the line to the journal
//!   `file` (created if new: how the game starts a journal), or to the one the last `journal` step
//!   wrote, or else to the seed's latest by name. A JSON object is written exactly as it appears
//!   in the step; a string is the line itself (any text, for a malformed line). A line whose
//!   `timestamp` is later than the clock first advances the clock to that time and waits for
//!   quiescence, then is appended: the pipeline takes a line stamped more than a few seconds
//!   after now for a clock running ahead. One stamped earlier leaves the clock alone.
//! - `{"file": "Status.json", "json": {…}}`, or `"text": "…"`, or `"from": "<payload>"`: write a
//!   companion file in the game folder whole (beside it under a name the pipeline ignores, then
//!   renamed over it); with `"partial": true`, cut off half way, as a reader can catch the game
//!   writing it.
//! - `{"game": "running" | "launcher" | "exited"}`: what the world's process list shows (the
//!   game, the launcher, neither).
//! - `{"advance": "1.5s"}`: move the clock on (`ms`, `s`, `m` or `h`).
//! - `{"mark": "jumped"}`: name this point, for `world_step`. Marks are unique.
//! - `{"net": {"url": "…", "status": 503}}`: one exchange in `net.json`'s form, added after the
//!   recording's, so it answers what it matches from the next request on (the last match
//!   answers). It settles at once.
//!
//! Reserved, and refused with "not supported yet" until their child lands: the step kinds
//! `focus`, `press`, `release`, `audio` and `say` and the header's `devices`, `reactions` and
//! `foreground` (astromech-1hh9). A malformed line, an unknown key or a step with two kinds fails
//! the load, naming the line.
//!
//! # Settling
//!
//! `world_step {to?}` runs the steps up to and including the mark `to` (none: to the end), and
//! returns `{ran, at, next}` once the core has consumed each one. A step consumed is one the
//! core has finished with, judged by what the core publishes, never by time:
//!
//! - a `journal` or `file` step, by generation: the world bumps the file's counter after each
//!   write, under a lock the pipeline takes to read the counters, so a counter read is never
//!   behind the file. The pipeline reads the counters of the files it was told of before it
//!   reads them, and publishes them once it has handled them, whether they parsed or not
//!   ([`settle::FileGens`]); the step waits until the file's published generation reaches its
//!   write. A file the pipeline ignores, or a write with no session (no pipeline), settles at
//!   once;
//! - an `advance` step, once every thread registered with the clock is parked in a clock wait
//!   again ([`ManualClock::advance`]);
//! - a `game` step sets the process list, then advances the clock by the game watch's poll
//!   (`gamewatch::POLL`) until the watch reports the game so (going from running takes two polls
//!   without it), then waits until every `activate` the watch started has ended: the session
//!   started, failed or was already there ([`settle::Sessions`]). The watch's `deactivate` runs
//!   on its own registered thread, so the advance that let it poll returned only once the
//!   session was dropped. `status` then reports the new game state.
//!
//! Each wait gives up at [`WAIT_GUARD`] (real time, a guard against a stuck core that decides
//! nothing): the step fails, and no later step runs. Threads that aren't registered with the
//! clock can still be running when a step returns (speech, a command's own thread); their
//! effects are awaited with `world_wait`.
//!
//! Two more `local` commands: `world_game` (the session's game state, as the pipeline built it;
//! null with no session) and `world_gate {hold}`, a hold on the game watch for race tests. Held,
//! the watch stops at the top of its next poll, before it reads the process list, waits there
//! until the rest of the core is parked on the clock, and the call returns `"reached"` (its
//! progress line `"held"` says the hold is on). Held again while the watch is stopped there, it
//! lets that poll go and stops at the next; `{hold: false}` lets it go on.
//!
//! The installed app has none of this: `Boundaries::settle` is None, so each hook is an `Option`
//! check (per game-watch poll, per `activate`, per file notification the pipeline handles).
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
//! The fakes: the process list is the world's (empty until a step starts the game); the opener,
//! clipboard, notifier, Start with Windows and the app window record what reaches them and touch
//! nothing; the secrets are in memory, seeded from `secrets.json`; the clock is a manual one
//! ([`World::clock`]), started at the header's time and moved only by the steps. Then it calls
//! [`app::assemble`] and [`app::start`], as the installed app does, but none of what only
//! [`app::run`] adds: no instance lock, no Start with Windows default, no update thread, no
//! window and no tray.
//!
//! The network is `net.json` (world/net.rs has the detail): the remote-data client, EDDN's upload and
//! EDSM's API are answered from it in process, and the downloads (the update feed and zip, the
//! speech models, the neutron star file) from a replay server on 127.0.0.1, at the URLs the
//! composition root rewrites ([`app::Http::downloads`]). None reaches the internet, so the
//! world's EDDN and EDSM switches are its own. A lookup never waits on the world's clock: the
//! remote-data client has no request spacing and Spansh's jobs are polled with no wait.
//!
//! Record mode, `--record-net <file>` with `--world`, sends the remote-data client's requests
//! and EDSM's reads to the real services instead, and writes their answers to `<file>` in
//! `net.json`'s form, redacted ([`astromech_data::replay`], "Redaction"). Everything else is
//! still answered from the world's recording. Never commit a recording made with a real key or
//! commander, or one holding personal data: a recording for the repository comes from public
//! endpoints, with no credentials, and is checked for redaction.
//!
//! Once the control API is up, the binary prints one line on stdout,
//! `{"control": {"port": …, "token": …}, "app_dir": "…"}`, its callers' only way to find it
//! ([`astromech_api::control::call_to`]). It runs until `quit`, or until its stdin reaches end of
//! file (the process that started it has gone, even if killed outright). `quit` is answered
//! before the core tears down. It then stops the session and the history follower, so nothing
//! writes into the temp directory, removes it and exits 0. A caller keeps its stdin open: a pipe
//! it holds.
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
//! - `http {method, url, unrecorded}`: an HTTP request, its URL with the query (key-like values
//!   `<redacted>`); `unrecorded` when the recording held no answer and it was refused.
//!
//! Two `local` commands read them, registered only in world mode, like every `world_*` command (decision astromech-unh9: the
//! window's origin is refused):
//!
//! - `world_observed {since?}`: `{observed: [...], next}`, the observations from `seq` `since`
//!   (default 0), and the `seq` the next one will have;
//! - `world_wait {match, since?}`: the first observation from `since` whose fields include every
//!   field of `match` (`{"kind": "edit"}`), waiting for it if there is none yet. The observation
//!   itself wakes the wait; if none arrives within [`WAIT_GUARD`], the call fails.

mod net;
mod script;
pub mod settle;
mod steps;

pub use settle::Settle;

use crate::app::{self, Assembled, Boundaries, Core, Opener};
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

/// A world, copied into its temp directory, and the state its fakes share.
pub struct World {
    dir: tempfile::TempDir,
    observed: Arc<Observed>,
    /// The running processes' exe names the game watch sees.
    processes: Arc<Mutex<Vec<String>>>,
    secrets: Vec<(String, String)>,
    /// The core's clock: time moves only when the script advances it.
    clock: Arc<ManualClock>,
    /// What the core publishes for the steps to settle on.
    settle: Arc<Settle>,
    /// `world.jsonl`'s steps, run by `world_step`.
    steps: Arc<Mutex<steps::Steps>>,
    /// `net.json`, answering every HTTP request, and its server.
    net: net::Net,
}

impl World {
    /// Copy the world in `src` to a fresh temp directory, and point its config's folders there.
    /// `record`: record mode's file (`--record-net`).
    pub fn load(src: &Path, record: Option<&Path>) -> Result<Self> {
        anyhow::ensure!(src.is_dir(), "no world folder at {}", src.display());
        let script = match std::fs::read_to_string(src.join(script::FILE)) {
            Ok(t) => Some(script::Script::parse(&t, src)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e).context("reading world.jsonl"),
        };
        let dir =
            tempfile::Builder::new().prefix("astromech-world-").tempdir().context("making the world's temp folder")?;
        // At the header's start; with no steps, at the real time, so the world's seed files are
        // as old as they are on disk.
        #[allow(clippy::disallowed_methods)]
        let start = script.as_ref().map_or_else(std::time::SystemTime::now, |s| s.start);
        let clock = Arc::new(ManualClock::new(start));
        let (processes, settle) = (Arc::<Mutex<Vec<String>>>::default(), Arc::<Settle>::default());
        let observed = Arc::<Observed>::default();
        let net = net::Net::load(src, record, &observed)?;
        let game_dir = dir.path().join("world").join("game");
        let steps =
            steps::Steps::new(script, game_dir, clock.clone(), processes.clone(), settle.clone(), net.replay().clone());
        let w =
            Self { dir, observed, processes, secrets: vec![], clock, settle, steps: Arc::new(Mutex::new(steps)), net };
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
        for t in ["game", "voice"] {
            if !doc.contains_key(t) {
                doc[t] = toml_edit::table();
            }
        }
        doc["game"]["journal_dir"] = path(self.world().join("game"));
        doc["game"]["bindings_dir"] = path(self.world().join("bindings"));
        doc["game"]["game_dir"] = path(self.root().join("elite"));
        doc["voice"]["models_dir"] = path(self.root().join("models"));
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
        let remote = self.net.remote(self.clock.clone());
        let processes = self.processes.clone();
        let (clip, commands) = (o.clone(), o.clone());
        let (steps, settle) = (self.steps.clone(), self.settle.clone());
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
            http: self.net.http(),
            more_commands: Some(Arc::new(move |core, reg| register(&commands, &steps, &settle, core, reg))),
            clock: self.clock.clone(),
            settle: Some(self.settle.clone()),
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

#[derive(Deserialize, TS)]
struct StepTo {
    #[serde(default)]
    to: Option<String>,
}

#[derive(Deserialize, TS)]
struct Hold {
    hold: bool,
}

/// The world's commands, all `local`.
fn register(
    observed: &Arc<Observed>,
    steps: &Arc<Mutex<steps::Steps>>,
    settle: &Arc<Settle>,
    core: &Arc<Core>,
    reg: &mut Registry,
) {
    let (s, c) = (steps.clone(), core.clone());
    reg.add_typed("world_step", Exposure::Local, move |a: Option<StepTo>, _| {
        s.lock().recover().run(a.and_then(|a| a.to).as_deref(), &c)
    });
    let c = core.clone();
    reg.add_typed("world_game", Exposure::Local, move |_: (), _| {
        let state = c.session.lock().recover().as_ref().map(|s| s.hub.snapshot());
        serde_json::to_value(state).map_err(|e| e.to_string())
    });
    let s = settle.clone();
    reg.add_typed("world_gate", Exposure::Local, move |a: Hold, call| {
        if !a.hold {
            s.gate.release();
            return Ok("released");
        }
        s.gate.hold(WAIT_GUARD, || call.progress(json!("held")))?;
        Ok("reached")
    });
    let o = observed.clone();
    reg.add_typed("world_observed", Exposure::Local, move |a: Option<Since>, _| {
        let (observed, next) = o.since(a.map_or(0, |a| a.since));
        Ok(json!({ "observed": observed, "next": next }))
    });
    let o = observed.clone();
    reg.add_typed("world_wait", Exposure::Local, move |a: WaitFor, _| o.wait(&a.pattern, a.since, WAIT_GUARD));
}

/// Spansh's jobs polled with no wait between polls: a lookup never waits for the world's clock.
const NO_WAIT: astromech_data::spansh::Polling =
    astromech_data::spansh::Polling { every: Duration::ZERO, timeout: Duration::from_secs(180) };

/// Run the core in the world at `src` until `quit`. Returns the exit code. `record`: record
/// mode's file (`--record-net`).
pub fn run(src: &Path, session: crate::session::Opts, record: Option<&Path>) -> i32 {
    match serve(src, session, record) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("world mode: {e:#}");
            1
        }
    }
}

fn serve(src: &Path, session: crate::session::Opts, record: Option<&Path>) -> Result<()> {
    let world = World::load(src, record)?;
    let b = world.boundaries();
    let app_dir = b.paths.app_dir().to_path_buf();
    crate::logging::init(&b.paths.log_dir(), false);
    log(&format!("world mode: {} in {} (pid {})", src.display(), world.root().display(), std::process::id()));
    if let Some(r) = record {
        log(&format!("world mode: recording lookups and EDSM's reads from the real services to {}", r.display()));
    }
    let opts = app::Opts { config: Some(world.config_path()), session, ..Default::default() };
    let recording = world.net.recording();
    let Assembled { core, ui } = app::assemble_with(&opts, b, |s| if recording { s } else { s.plot_polling(NO_WAIT) });
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
    // Stop what writes into the temp folder before removing it: the session (and its threads)
    // and the history follower, whose first open of its database may still be running. The
    // control API still answers meanwhile; a `set_folders` after the stop starts no follower
    // (finding astromech-8t2s).
    core.deactivate("quit");
    core.history.stop();
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
        let w = World::load(src.path(), None).unwrap();
        let root = w.root().to_path_buf();
        let cfg = crate::config::Config::load(&w.config_path()).unwrap();
        let b = w.boundaries();
        assert_eq!(cfg.data_dir(&b.paths), root.join("data"));
        assert_eq!(cfg.journal_dir(&b.paths), Some(root.join("world").join("game")));
        assert_eq!(cfg.bindings_dir(), Some(root.join("world").join("bindings")));
        assert_eq!(cfg.game_dir(), Some(root.join("elite")));
        assert_eq!(cfg.models_dir(&b.paths), root.join("models"));
        assert!(cfg.eddn.enabled && !cfg.edsm.enabled, "the world's own sharing switches: the recording answers them");
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

    /// Each HTTP path of the core, as world mode builds it, is answered from the world's one
    /// `net.json`, and each request is observed: the remote-data client, EDDN's upload and EDSM's
    /// API in process, and the downloads through the replay server at the URL the core fetches.
    #[test]
    fn every_http_path_is_answered_from_the_one_recording() {
        use astromech_voice::models;
        let src = tempfile::tempdir().unwrap();
        std::fs::write(
            src.path().join("net.json"),
            r#"{"exchanges": [
                {"url": "/api/systems/field_values/system_names", "text": "{}"},
                {"url": "eddn.edcd.io:4430/upload/", "method": "POST", "text": "OK"},
                {"url": "/api-logs-v1/get-logs", "status": 503},
                {"url": "/releases/download/neutron/", "fault": "offline"},
                {"url": "/latest.json", "status": 404},
                {"url": "/sherpa-onnx/releases/download/asr-models/", "status": 503}
            ]}"#,
        )
        .unwrap();
        let w = World::load(src.path(), None).unwrap();
        let b = w.boundaries();
        let client = (b.remote)(Path::new("unused"));
        let names = astromech_data::Request::get("/api/systems/field_values/system_names").query("q", "Sol");
        assert_eq!(client.send(astromech_data::Service::Spansh, &names).unwrap(), "{}");
        assert_eq!((b.http.eddn)().post(b"gz").unwrap().body, "OK");
        let logs =
            (b.http.edsm)(Duration::from_secs(10)).get(astromech_eddn::edsm::api::LOGS_URL, &[("apiKey", "k3y")]);
        assert_eq!(logs.unwrap().status, 503);
        let base = (b.http.downloads)(astromech_galaxy::neutron::RELEASE_URL);
        assert!(base.starts_with("http://127.0.0.1:"), "{base}");
        let neutron = astromech_data::download::text(&format!("{base}/x.sha256"), false).unwrap_err();
        assert!(matches!(neutron, astromech_data::download::DownloadError::Offline(_)), "{neutron:?}");
        let feed = (b.http.downloads)(crate::update::DEFAULT_FEED);
        assert!(format!("{:#}", crate::update::http_get(&feed, false).unwrap_err()).contains("404"));
        let dir = w.root().join("models");
        let e = models::download_from(&dir, &models::STT, &(b.http.downloads)(&models::STT.url()), false, &mut |_| {});
        assert!(format!("{:#}", e.unwrap_err()).contains("503"));
        assert!(client.send(astromech_data::Service::Edsm, &astromech_data::Request::get("/api-v1/system")).is_err());
        let seen: Vec<(String, String, bool)> = w
            .observed
            .since(0)
            .0
            .iter()
            .map(|o| (o["method"].as_str().unwrap().into(), o["url"].as_str().unwrap().into(), o["unrecorded"] == true))
            .collect();
        let url = |s: &str| s.to_string();
        assert_eq!(
            seen,
            [
                (url("GET"), url("https://spansh.co.uk/api/systems/field_values/system_names?q=Sol"), false),
                (url("POST"), url(astromech_eddn::UPLOAD_URL), false),
                (url("GET"), url("https://www.edsm.net/api-logs-v1/get-logs?apiKey=<redacted>"), false),
                (url("GET"), format!("{}/x.sha256", astromech_galaxy::neutron::RELEASE_URL), false),
                (url("GET"), url(crate::update::DEFAULT_FEED), false),
                (url("GET"), models::STT.url(), false),
                (url("GET"), url("https://www.edsm.net/api-v1/system"), true),
            ]
        );
    }

    #[test]
    fn the_clock_starts_at_the_headers_time_and_a_bad_step_fails_the_load() {
        use astromech_proto::Clock;
        let src = tempfile::tempdir().unwrap();
        std::fs::write(src.path().join("world.jsonl"), "{\"start\": \"2026-11-07T12:00:00Z\"}\n{\"mark\": \"a\"}\n")
            .unwrap();
        let w = World::load(src.path(), None).unwrap();
        assert_eq!(w.clock().now_s(), 1_794_052_800, "2026-11-07T12:00:00Z");
        std::fs::write(
            src.path().join("world.jsonl"),
            "{\"start\": \"2026-11-07T12:00:00Z\"}\n{\"focus\": \"Elite\"}\n",
        )
        .unwrap();
        let e = format!("{:#}", World::load(src.path(), None).err().unwrap());
        assert_eq!(e, "world.jsonl line 2: the step kind `focus` is not supported yet (astromech-1hh9)");
    }

    #[test]
    fn the_fakes_record_what_reaches_them() {
        let src = tempfile::tempdir().unwrap();
        let w = World::load(src.path(), None).unwrap();
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
