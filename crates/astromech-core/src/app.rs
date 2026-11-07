//! The always-running core (M0, docs/V1-DESIGN.md §7): tray icon, single instance, start with
//! Windows, game detection with idle/active, the app window launched from the tray, crash
//! marker and diagnostics. [`run`] is what `astromech-core.exe` does when started without a
//! console option (and with `--background` from the Run key).

use crate::config::Config;
use crate::gamewatch::{self, Change, Game};
use crate::logging::log;
use crate::session::{self, Session, VOICE_PAUSED};
use crate::tester::{self, Source, TESTER};
use crate::update::{self, ApplyWhen, Status as UpStatus};
use anyhow::Result;
use astromech_api::reply::{Answer, Done, Nothing};
use astromech_api::{control, Exposure, Recover, Registry, Request};
use astromech_store::Paths;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use ts_rs::TS;

pub const SHELL_EXE: &str = if cfg!(windows) { "astromech-shell.exe" } else { "astromech-shell" };

#[derive(Debug, Clone, Default)]
pub struct Opts {
    /// Started by the Run key: tray only, no window.
    pub background: bool,
    /// Route for the window when a second start asks the first to open it.
    pub route: Option<String>,
    pub config: Option<PathBuf>,
    pub session: session::Opts,
}

/// Messages to the UI thread (the tray).
#[derive(Debug, Clone, PartialEq)]
pub enum Ui {
    Refresh,
    Quit,
}

/// What Settings → Files says about the values `astromech.toml` had wrong.
fn config_problems_note(problems: &[crate::config::Problem]) -> Option<String> {
    if problems.is_empty() {
        return None;
    }
    let list: Vec<String> = problems.iter().map(|p| p.to_string()).collect();
    Some(format!("These config values were ignored and are at their defaults: {}.", list.join("; ")))
}

/// What Settings → Files says when the whole file can't be read.
fn config_unreadable_note(e: &anyhow::Error) -> String {
    format!("The config file can't be read, so every setting is at its default: {e:#}")
}

/// What the config file's state is, and the file as last looked at (modified time and size).
#[derive(Default)]
pub struct ConfigWatch {
    /// Shown in Settings → Files: values of the config file that were ignored, or why it
    /// wasn't read at all. Cleared when the file is fixed (see [`Core::refresh_config`]).
    pub note: Option<String>,
    /// The whole file was unreadable, so every setting is a default.
    pub failed: bool,
    seen: Option<(std::time::SystemTime, u64)>,
}

fn file_stamp(path: &Path) -> Option<(std::time::SystemTime, u64)> {
    let m = std::fs::metadata(path).ok()?;
    Some((m.modified().ok()?, m.len()))
}

impl ConfigWatch {
    /// The state after the file was loaded: `note` is what was wrong with it, if anything.
    pub fn loaded(path: &Path, note: Option<String>, failed: bool) -> Self {
        let seen = if note.is_some() { file_stamp(path) } else { None };
        Self { note, failed, seen }
    }
}

/// Shared state; the tray reads it, the watcher and control API write it.
pub struct Core {
    pub app_dir: PathBuf,
    pub config_path: PathBuf,
    /// The config; the window's setup edits (push-to-talk, mic, folders) replace it.
    pub cfg: std::sync::RwLock<Config>,
    /// Held across a whole setup edit (read, write the file, swap `cfg`).
    pub config_lock: Mutex<()>,
    /// Kokoro for Settings' voice preview while no session runs (released when one starts, or
    /// after a minute unused).
    pub preview: crate::speech::PreviewSlot,
    /// Shown in Settings → Files, and whether the whole file was unreadable.
    pub config_watch: Mutex<ConfigWatch>,
    pub game: Mutex<Game>,
    pub session: Mutex<Option<Session>>,
    /// While a session loads (models take a few seconds).
    pub starting: AtomicBool,
    pub voice: Mutex<String>,
    pub crash: Mutex<Option<String>>,
    pub models: Mutex<Option<String>>,
    pub update: Mutex<update::Handle>,
    pub ui: Mutex<Sender<Ui>>,
    pub opts: Opts,
    /// A mic calibration in progress from the window.
    pub calibration: Mutex<Option<crate::calibration::Run>>,
    /// Onboarding finished or skipped on this PC.
    pub onboarded: AtomicBool,
    /// The journal history follower, on the configured journal folder.
    pub history: crate::history::Following,
    /// API keys (EDSM's): the OS keyring.
    pub secrets: Arc<dyn astromech_store::secrets::Secrets>,
    /// The route being followed (docs/PLOTTING.md). The core owns it, not a session, so the
    /// window can set one while Elite isn't running.
    pub route: Arc<crate::route::Desk>,
    /// The command layer's state (keybinds, journal caches), read by the window's commands.
    pub api: Arc<astromech_api::State>,
}

/// The command layer's state over `core`'s live config: its folders are read on every call,
/// so a `set_folders` applies at once. `data_dir` is fixed for the process, and with it the
/// process's one remote-data client (`remote`, its cache in `data_dir`) and the star map, whose
/// preload is the bundled `starmap.db` next to this exe. `open_external` may open folders under
/// `app_dir` and the log folder too.
fn api_state(
    core: std::sync::Weak<Core>,
    data_dir: PathBuf,
    remote: Arc<astromech_data::Client>,
    openable: Vec<PathBuf>,
) -> astromech_api::State {
    let preload = std::env::current_exe().ok().and_then(|exe| astromech_api::preload_beside(&exe));
    let folders = move || core.upgrade().map(|c| api_folders(&c.cfg())).unwrap_or_default();
    astromech_api::State::new(data_dir, remote, preload, folders).openable(openable)
}

/// The process's one remote-data client, its cache in `data_dir`.
pub fn remote_client(data_dir: &Path) -> Arc<astromech_data::Client> {
    astromech_data::Client::shared(data_dir, Arc::new(|s: &str| log(s)))
}

fn api_folders(c: &Config) -> astromech_api::Folders {
    let bindings = c.game.bindings_dir.trim();
    astromech_api::Folders {
        journal: c.journal_dir().unwrap_or_default(),
        bindings: (!bindings.is_empty()).then(|| PathBuf::from(bindings)),
        game: c.game_dir(),
    }
}

impl Core {
    pub fn cfg(&self) -> Config {
        self.cfg.read().recover().clone()
    }

    pub fn notify(&self) {
        let _ = self.ui.lock().recover().send(Ui::Refresh);
    }

    pub fn active(&self) -> bool {
        self.session.lock().recover().is_some()
    }

    /// Keep the journal history on `cfg`'s journal folder: at startup, and after every config
    /// edit (re-points the follower when the folder changed; otherwise nothing changes).
    fn follow_history(&self, cfg: &Config) {
        self.history.follow(cfg.journal_dir(), &cfg.data_dir(), Arc::new(|m: &str| log(m)));
    }

    /// Why the config file wasn't used, if it wasn't.
    pub fn config_note(&self) -> Option<String> {
        self.config_watch.lock().recover().note.clone()
    }

    /// Make a config the one in force: the window's setup edits and a fixed file both come here.
    fn install_config(&self, mut c: Config) {
        c.actions.dry_run |= self.opts.session.dry_run;
        apply_eddn_switch(&c);
        apply_edsm_switch(&c, self.secrets.as_ref());
        // Under the config lock, so concurrent edits re-point in the order they saved.
        self.follow_history(&c);
        *self.cfg.write().recover() = c;
    }

    /// Whether the whole config file was unreadable (so every setting is a default).
    pub fn config_failed(&self) -> bool {
        self.config_watch.lock().recover().failed
    }

    /// While the config file had something wrong (values ignored, or unreadable), look at it
    /// again when it changes: use what loads and update or clear the note, so a fix needs no
    /// restart. A file that loaded cleanly is never re-read here (edits to a working file apply
    /// at the next start, as before).
    pub fn refresh_config(&self) {
        // The usual case (a good file) costs one look and never waits on an edit in progress.
        if self.config_watch.lock().recover().note.is_none() {
            return;
        }
        let _g = self.config_lock.lock().unwrap_or_else(|e| e.into_inner());
        let mut w = self.config_watch.lock().recover();
        if w.note.is_none() {
            return;
        }
        let stamp = file_stamp(&self.config_path);
        if stamp.is_none() || stamp == w.seen {
            return;
        }
        w.seen = stamp;
        match Config::load_report(&self.config_path) {
            Ok((c, problems)) => {
                log(&format!("config: {} read again; using it", self.config_path.display()));
                for p in &problems {
                    log(&format!("config: ignoring {p}; using the default for it"));
                }
                w.failed = false;
                w.note = config_problems_note(&problems);
                drop(w);
                self.install_config(c);
            }
            Err(e) => {
                log(&format!("config error, still using defaults: {e:#}"));
                w.failed = true;
                w.note = Some(config_unreadable_note(&e));
            }
        }
    }

    /// The tray's one status line (also its tooltip, under "astromech").
    pub fn status_line(&self) -> String {
        self.status_line_for(TESTER.armed(Instant::now()))
    }

    /// The tray status. While the voice tester's test mode is on, push-to-talk does nothing in the
    /// game and the HUD cannot say so, so the line does.
    fn status_line_for(&self, testing: bool) -> String {
        let game = *self.game.lock().recover();
        let mut s = match (game, self.active(), self.starting.load(Ordering::SeqCst)) {
            _ if testing => "Voice tester open · push-to-talk testing only".to_string(),
            (_, _, true) => "Starting voice…".to_string(),
            (Game::Running, true, _) => format!("Elite running · {}", self.voice.lock().recover()),
            (Game::Launcher, true, _) => format!("Launcher open · {}", self.voice.lock().recover()),
            (Game::Running, false, _) => "Elite running · voice failed (see logs)".to_string(),
            _ => "Waiting for Elite".to_string(),
        };
        if VOICE_PAUSED.load(Ordering::SeqCst) {
            s = format!("{s} · paused");
        }
        if let Some(m) = self.models.lock().recover().as_ref() {
            s = format!("{s} · {m}");
        }
        s
    }

    /// What still has to be set up before voice can work, in the order to do it.
    pub fn setup_needed(&self) -> Vec<&'static str> {
        let mut v = vec![];
        // A broken config falls back to defaults, which would otherwise read as "not set".
        self.refresh_config();
        if self.config_failed() {
            v.push("config file has an error (see logs)");
            return v;
        }
        let cfg = self.cfg();
        if cfg.controls.ptt.is_none() {
            v.push("push-to-talk not set");
        }
        if models_missing(&cfg) {
            v.push("speech models not downloaded");
        }
        v
    }

    /// The `status` reply: the core's state for the window and the tray.
    pub fn status(&self) -> Done<Status> {
        self.refresh_config();
        let (killed, toasts, mining) = match self.session.lock().recover().as_ref() {
            Some(s) => (s.runner.policy.is_killed(), s.toasts.recent(), Some(s.hub.mining())),
            None => (false, vec![], None),
        };
        let cfg = self.cfg();
        let button = |b: &Option<crate::config::Button>| b.as_ref().map(|b| format!("{} {}", b.device, b.key));
        // Each lock is taken and let go on its own: a guard held to the end of the struct below
        // would deadlock `status_line`, which takes them again.
        let game = *self.game.lock().recover();
        let voice = self.voice.lock().recover().clone();
        let crash_pending = self.crash.lock().recover().is_some();
        let models_progress = self.models.lock().recover().clone();
        let update = self.update.lock().recover().snap.clone();
        Done::new(Status {
            version: env!("CARGO_PKG_VERSION"),
            pid: std::process::id(),
            game,
            active: self.active(),
            starting: self.starting.load(Ordering::SeqCst),
            voice,
            voice_paused: session::VOICE_PAUSED.load(Ordering::SeqCst),
            killed,
            autostart: crate::autostart::is_enabled(),
            crash_pending,
            status_line: self.status_line(),
            app_dir: self.app_dir.display().to_string(),
            log_dir: crate::logging::dir().unwrap_or_else(|| Paths::current().log_dir()).display().to_string(),
            config: self.config_path.display().to_string(),
            config_note: self.config_note(),
            setup_needed: self.setup_needed(),
            ptt: button(&cfg.controls.ptt),
            kill_switch: button(&cfg.controls.kill_switch),
            mic: cfg.voice.mic.clone(),
            onboarded: self.onboarded.load(Ordering::SeqCst),
            models_missing: models_missing(&cfg),
            models_progress,
            models_dir: cfg.models_dir().display().to_string(),
            update,
            toasts,
            history: self.history.status(),
            mining,
        })
    }

    /// [`Core::status`] as JSON (diagnostics).
    pub fn status_json(&self) -> Value {
        serde_json::to_value(self.status()).unwrap_or_default()
    }

    pub fn save_diagnostics(&self) -> Result<PathBuf> {
        let log_dir = crate::logging::dir().unwrap_or_else(|| Paths::current().log_dir());
        let data = self.cfg().data_dir();
        let p = crate::diagnostics::save(&crate::diagnostics::Inputs {
            app_dir: &self.app_dir,
            data_dir: &data,
            log_dir: &log_dir,
            config: Some(&self.config_path),
            status: serde_json::to_string_pretty(&self.status_json()).unwrap_or_default(),
        })?;
        log(&format!("diagnostics saved: {}", p.display()));
        if self.crash.lock().recover().take().is_some() {
            crate::crash::acknowledge(&self.app_dir);
        }
        self.notify();
        Ok(p)
    }

    pub fn kill_switch(&self) {
        if let Some(s) = self.session.lock().recover().as_ref() {
            if s.runner.policy.is_killed() {
                s.runner.policy.rearm();
                s.speaker.say("Re-armed.");
            } else {
                s.speaker.interrupt();
                s.runner.policy.kill();
                s.speaker.say("Kill switch on. All actions stopped.");
            }
        }
        self.notify();
    }

    fn activate(self: &Arc<Self>, why: &str) {
        if self.active() || self.starting.swap(true, Ordering::SeqCst) {
            return;
        }
        log(&format!("{why}: going active (loading the binds check, game state and voice)"));
        // The session loads its own Kokoro; free the preview's. (`starting` is already set, so no
        // new preview Kokoro can load while the session does.)
        self.preview.take();
        self.notify();
        let t0 = Instant::now();
        let cfg = self.cfg();
        let r = Session::start(&cfg, &self.opts.session, self.route.clone(), &self.api).and_then(|mut s| {
            s.start_buttons(&cfg, &self.opts.session)?;
            Ok(s)
        });
        match r {
            Ok(s) => {
                log(&format!("active in {} ms: {}", t0.elapsed().as_millis(), s.voice));
                *self.voice.lock().recover() = s.voice.clone();
                *self.session.lock().recover() = Some(s);
                // A voice saved in Settings while the session loaded: use it now.
                let (old, new) = (&cfg.voice, self.cfg().voice);
                if (old.speaker.as_str(), old.speed, old.colour) != (new.speaker.as_str(), new.speed, new.colour) {
                    self.apply_voice();
                }
            }
            Err(e) => {
                log(&format!("couldn't go active: {e:#}"));
                *self.voice.lock().recover() = format!("error: {e}");
            }
        }
        self.starting.store(false, Ordering::SeqCst);
        // If the game went away while we were loading, unload again.
        if *self.game.lock().recover() == Game::NotRunning {
            self.deactivate("the game exited while loading");
        }
        self.notify();
    }

    fn deactivate(&self, why: &str) {
        if !self.starting.load(Ordering::SeqCst) {
            *self.voice.lock().recover() = "voice not loaded".into();
        }
        // Out of the lock before dropping: closing a session waits for its threads, and the
        // status, tray and kill switch all need this lock meanwhile.
        let session = self.session.lock().recover().take();
        if let Some(s) = session {
            log(&format!("{why}: going idle (unloading voice, closing the microphone)"));
            s.speaker.interrupt();
            s.runner.policy.kill();
            drop(s);
        }
        self.notify();
    }

    fn set_update(&self, f: impl FnOnce(&mut update::Handle)) {
        f(&mut self.update.lock().recover());
        self.notify();
    }

    /// A requested check (Settings → Check now, the tray's Check for updates), off the caller's
    /// thread: it goes to the network.
    pub fn check_update_in_background(self: &Arc<Self>) {
        let c = self.clone();
        std::thread::spawn(move || c.check_update(true));
    }

    /// Fetch `latest.json`, and download the zip if it is newer. Safe while Elite is running.
    pub fn check_update(self: &Arc<Self>, manual: bool) {
        {
            let mut h = self.update.lock().recover();
            if h.snap.status.busy() {
                return;
            }
            h.snap.status = UpStatus::Checking;
            h.snap.error = None;
        }
        self.notify();
        let pk = match update::public_key() {
            Ok(k) => k,
            Err(e) => {
                self.set_update(|h| {
                    h.snap.status = UpStatus::Error;
                    h.snap.error = Some(format!("{e:#}"));
                });
                log(&format!("update: {e:#}"));
                return;
            }
        };
        log(if manual { "update: checking (requested)" } else { "update: checking" });
        let settings = self.cfg().update;
        match update::fetch_and_verify(&settings, &pk) {
            Ok((m, _)) => {
                let installed = update::installed_version();
                self.set_update(|h| {
                    h.snap.latest = Some(m.version.clone());
                    h.snap.notes = m.notes.clone();
                    h.snap.last_check = Some(chrono::Utc::now().to_rfc3339());
                    h.snap.installed = installed.into();
                });
                if !update::newer(&m.version, installed) {
                    self.set_update(|h| {
                        h.snap.status = UpStatus::Idle;
                        h.snap.ready = false;
                        h.payload = None;
                    });
                    log(&format!("update: already on {installed}"));
                    return;
                }
                if settings.apply == ApplyWhen::Never {
                    self.set_update(|h| {
                        h.snap.status = UpStatus::Available;
                        h.snap.ready = false;
                    });
                    log(&format!("update: {} available (apply = never)", m.version));
                    return;
                }
                self.set_update(|h| h.snap.status = UpStatus::Downloading);
                let work = self.app_dir.join("updates");
                match update::stage(&m, &work, |p| log(&format!("update: {p}"))) {
                    Ok(payload) => {
                        self.set_update(|h| {
                            h.payload = Some(payload);
                            h.snap.ready = true;
                            h.snap.status = UpStatus::Ready;
                            h.snap.error = None;
                        });
                        log(&format!("update: {} ready to install", m.version));
                    }
                    Err(e) => {
                        self.set_update(|h| {
                            h.snap.status = UpStatus::Error;
                            h.snap.error = Some(format!("{e:#}"));
                            h.snap.ready = false;
                        });
                        log(&format!("update: download failed: {e:#}"));
                    }
                }
            }
            Err(e) => {
                self.set_update(|h| {
                    h.snap.status = UpStatus::Error;
                    h.snap.error = Some(format!("{e:#}"));
                    h.snap.last_check = Some(chrono::Utc::now().to_rfc3339());
                });
                log(&format!("update: check failed: {e:#}"));
            }
        }
    }

    /// Quit, overlay the staged payload, relaunch. Elite may still be running.
    pub fn install_update(self: &Arc<Self>) -> Answer<Nothing> {
        if self.cfg().update.apply == ApplyWhen::Never {
            return Answer::failed("updates are set to never apply ([update] apply = \"never\")");
        }
        // Claim the install under the lock, so a second click can't start another helper.
        let (payload, before) = {
            let mut h = self.update.lock().recover();
            if h.snap.status == UpStatus::Applying {
                return Answer::failed("already installing");
            }
            if !h.snap.ready {
                return Answer::failed(h.snap.error.clone().unwrap_or_else(|| "no update ready".into()));
            }
            let Some(p) = h.payload.clone() else {
                return Answer::failed("update payload missing");
            };
            let before = h.snap.status;
            h.snap.status = UpStatus::Applying;
            (p, before)
        };
        let install_dir = Paths::current().exe_dir().to_path_buf();
        // Bring the window back afterwards if it is open now (the helper closes it to replace it).
        let window_open = gamewatch::process_names().iter().any(|n| n.eq_ignore_ascii_case(SHELL_EXE));
        let after = if window_open { update::After::RelaunchWithWindow } else { update::After::Relaunch };
        self.deactivate("installing update");
        match update::spawn_apply(&payload, &install_dir, after) {
            Ok(()) => {
                self.notify();
                log("update: helper started; quitting so files can be replaced");
                // Quit a moment later, so whoever asked (the window) still gets this reply.
                let ui = self.ui.lock().recover().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(300));
                    let _ = ui.send(Ui::Quit);
                });
                Answer::ok(Nothing {})
            }
            Err(e) => {
                let m = format!("{e:#}");
                log(&format!("update: couldn't start helper: {m}"));
                self.set_update(|h| h.snap.status = before);
                Answer::failed(m)
            }
        }
    }

    fn apply_on_quit(&self) {
        if self.cfg().update.apply != ApplyWhen::OnQuit {
            return;
        }
        let h = self.update.lock().recover();
        if h.snap.status == UpStatus::Applying {
            return;
        }
        if !h.snap.ready {
            return;
        }
        let Some(payload) = h.payload.clone() else { return };
        drop(h);
        match update::spawn_apply(&payload, Paths::current().exe_dir(), update::After::Exit) {
            Ok(()) => log("update: applying on quit"),
            Err(e) => log(&format!("update: apply on quit failed: {e:#}")),
        }
    }

    /// Launch (or focus) the app window at `route`.
    pub fn open_window(&self, route: Option<&str>) -> Result<(), String> {
        let exe = std::env::var_os("ASTROMECH_SHELL")
            .map(PathBuf::from)
            .unwrap_or_else(|| Paths::current().exe_dir().join(SHELL_EXE));
        if !exe.is_file() {
            let m = format!("the app window ({}) isn't installed next to the core", exe.display());
            log(&m);
            return Err(m);
        }
        let mut c = std::process::Command::new(&exe);
        if let Some(r) = route {
            c.arg("--route").arg(r);
        }
        match c.spawn() {
            Ok(_) => {
                log(&format!("opened the app window{}", route.map(|r| format!(" at {r}")).unwrap_or_default()));
                Ok(())
            }
            Err(e) => {
                let m = format!("couldn't open the app window: {e}");
                log(&m);
                Err(m)
            }
        }
    }

    /// Setup edits from the window: write `astromech.toml`, then use it. Voice picks the new
    /// buttons and mic up the next time it starts (when Elite next starts).
    fn edit_config(&self, r: &Request) -> Answer<Applies> {
        let res = crate::setup::Edit::from_request(&r.cmd, r.args.as_ref()).and_then(|edits| {
            crate::setup::edit(&self.config_lock, &self.config_path, &edits, |c| self.install_config(c))
        });
        match res {
            Ok(()) => {
                log(&format!("config: {} saved from the window", r.cmd.trim_start_matches("set_")));
                if r.cmd == "set_voice" {
                    self.apply_voice();
                }
                self.notify();
                let now = matches!(r.cmd.as_str(), "set_voice" | "set_eddn" | "set_edsm");
                Answer::ok(Applies { applies_next_session: !now && self.active() })
            }
            Err(e) => Answer::failed(format!("{e:#}")),
        }
    }

    /// Settings → Data sharing, EDSM: the API key (to the OS keyring; empty forgets it), then
    /// the switch and commander name (to the config), then the uploader's credentials.
    fn set_edsm(&self, r: &Request) -> Answer<Applies> {
        let edits = match crate::setup::Edit::from_request(&r.cmd, r.args.as_ref()) {
            Ok(e) => e,
            Err(e) => return Answer::failed(format!("{e:#}")),
        };
        if let Some(k) = r.args.as_ref().and_then(|a| a.get("api_key")).filter(|k| !k.is_null()) {
            let Some(k) = k.as_str().map(str::trim) else {
                return Answer::failed("api_key must be text");
            };
            if let Err(e) = self.secrets.set(astromech_eddn::edsm::KEY_SECRET, Some(k).filter(|k| !k.is_empty())) {
                return Answer::failed(format!("couldn't save the API key: {e}"));
            }
            log(if k.is_empty() { "EDSM: API key removed" } else { "EDSM: API key saved in the OS keyring" });
        }
        if edits.is_empty() {
            apply_edsm_switch(&self.cfg(), self.secrets.as_ref());
            self.notify();
            return Answer::ok(Applies { applies_next_session: false });
        }
        self.edit_config(r)
    }

    /// Speak the next line in the configured voice: the running session's and the preview's.
    fn apply_voice(&self) {
        let cfg = self.cfg();
        match crate::speech::VoiceParams::from_config(&cfg.voice) {
            Ok(v) => {
                if let Some(s) = self.session.lock().recover().as_ref() {
                    s.speaker.set_voice(&v);
                }
                self.preview.set_voice(&v);
                log(&format!("voice: {} ({}, speed {})", v.speaker, v.colour.as_str(), cfg.voice.speed));
            }
            Err(e) => log(&format!("voice: {e:#}")),
        }
    }

    /// Settings' preview: say a sample line in the configured voice, through the session's
    /// speaker if voice is running, else a Kokoro started for the preview. Refused while a
    /// session loads (it would load a second Kokoro) and when speech output is off.
    fn preview_voice(&self) -> Answer<Previewed> {
        use crate::speech::PREVIEW_LINE;
        let cfg = self.cfg();
        let route = || -> Option<Result<()>> {
            if self.starting.load(Ordering::SeqCst) {
                return Some(Err(anyhow::anyhow!("voice is starting; try again in a moment")));
            }
            if let Some(s) = self.session.lock().recover().as_ref() {
                if !s.speaker.speaks_aloud() {
                    return Some(Err(anyhow::anyhow!("speech output is off (see the log)")));
                }
                s.speaker.say(PREVIEW_LINE);
                return Some(Ok(()));
            }
            if !cfg.voice.tts {
                return Some(Err(anyhow::anyhow!("speech output is off ([voice] tts = false in the config)")));
            }
            if models_missing(&cfg) {
                return Some(Err(anyhow::anyhow!("download the speech models first (Speech models, above)")));
            }
            None
        };
        let start = || -> Result<Arc<dyn crate::speech::Speaker>> {
            let v = &cfg.voice;
            let s = crate::speech::TtsSpeaker::start(
                &cfg.worker_exe(),
                &cfg.models_dir(),
                v.threads,
                &cfg.data_dir(),
                &v.speaker,
                v.speed,
                v.colour,
                vec![],
            )
            .map_err(|e| anyhow::anyhow!("couldn't start text-to-speech: {e:#}"))?;
            log("voice preview: Kokoro loaded (released after a minute unused)");
            Ok(Arc::new(s))
        };
        match self.preview.say(PREVIEW_LINE, route, start) {
            Ok(()) => Answer::ok(Previewed { line: PREVIEW_LINE }),
            Err(e) => Answer::failed(format!("{e:#}")),
        }
    }

    /// The Route page's commands besides `route_set`, `route_add` and `route_resume`.
    fn route_cmd(&self, cmd: &str) -> Done<RouteReply> {
        let d = &self.route;
        let view = match cmd {
            "route_skip" => d.step(true),
            "route_back" => d.step(false),
            "route_clear" => d.clear(),
            "route_copy" => d.copy_next(),
            _ => d.view(),
        };
        Done::new(RouteReply { view })
    }

    /// `route_set`: `{name, source, waypoints}` or, for an import, `{name, source, text}` (a
    /// Spansh CSV or one system per line).
    fn route_set(&self, a: RouteSetArgs) -> Answer<RouteReply> {
        use crate::route::{parse_list, Route};
        let waypoints = match (a.text, a.waypoints) {
            (Some(t), _) => parse_list(&t),
            (None, Some(w)) => w,
            (None, None) => return Answer::failed("bad waypoints: expected waypoints or text"),
        };
        let name = a.name.filter(|n| !n.trim().is_empty()).unwrap_or_else(|| "Route".to_string());
        self.route
            .set(Route::new(&name, &a.source.unwrap_or_default(), waypoints))
            .map(|view| RouteReply { view })
            .into()
    }

    /// `route_add`: `{system, label, source}`, a lookup result's Add stop.
    fn route_add(&self, a: RouteAddArgs) -> Answer<RouteReply> {
        let stop = crate::route::Waypoint { system: a.system.unwrap_or_default(), label: a.label, jumps: None };
        self.route.add(stop, &a.source.unwrap_or_default()).map(|view| RouteReply { view }).into()
    }

    /// The core's commands. Each one's value is its reply as the window reads it; an error the
    /// window shows is `{"ok": false, "error": …}` inside the value ([`Answer`]). `quit` and
    /// `open_window` stay with the tray and a second start (decision astromech-unh9). A command
    /// that takes nothing ignores its args, as before they were typed.
    pub fn commands(self: &Arc<Self>) -> Registry {
        use crate::calibration as cal;
        use Exposure::{Local, Window};
        let mut reg = Registry::new();
        astromech_api::commands::register(&self.api, &mut reg);
        astromech_api::lookups::register(&self.api, &mut reg);

        // Each handler gets the core and the request; `A` is what the window sends as args
        // (`String`: the request's text).
        fn add<A: TS + ?Sized + 'static, R: Serialize + TS + 'static>(
            reg: &mut Registry,
            core: &Arc<Core>,
            names: &[&str],
            exposure: Exposure,
            f: fn(&Arc<Core>, &Request) -> R,
        ) {
            for name in names {
                let c = core.clone();
                reg.add_request::<A, R>(name, exposure, move |r, _| Ok(f(&c, r)));
            }
        }
        // Args parsed as `A`; a bad one is the reply's own error, as the window reads it.
        fn parsed<A: serde::de::DeserializeOwned, R>(r: &Request, why: &str) -> Result<A, Answer<R>> {
            serde_json::from_value(r.args.clone().unwrap_or_else(|| json!({})))
                .map_err(|e| Answer::failed(format!("{why}: {e}")))
        }
        fn text(r: &Request) -> String {
            r.text.clone().unwrap_or_default()
        }

        add::<(), _>(
            &mut reg,
            self,
            &["route", "route_skip", "route_back", "route_clear", "route_copy"],
            Window,
            |c, r| c.route_cmd(&r.cmd),
        );
        add::<(), Answer<RouteReply>>(&mut reg, self, &["route_resume"], Window, |c, _| {
            c.route.resume().map(|view| RouteReply { view }).into()
        });
        add::<RouteSetArgs, _>(&mut reg, self, &["route_set"], Window, |c, r| match parsed(r, "bad waypoints") {
            Ok(a) => c.route_set(a),
            Err(e) => e,
        });
        add::<RouteAddArgs, _>(&mut reg, self, &["route_add"], Window, |c, r| match parsed(r, "bad stop") {
            Ok(a) => c.route_add(a),
            Err(e) => e,
        });
        add::<(), _>(&mut reg, self, &["status"], Window, |c, _| c.status());
        // `route` is the request's page to show, not args.
        add::<(), Answer<Nothing>>(&mut reg, self, &["open_window"], Local, |c, r| {
            c.open_window(r.route.as_deref()).map(|()| Nothing {}).into()
        });
        add::<(), _>(&mut reg, self, &["save_diagnostics"], Window, |c, _| match c.save_diagnostics() {
            Ok(p) => {
                reveal(&p);
                Answer::ok(Saved { path: p.display().to_string() })
            }
            Err(e) => Answer::failed(format!("{e:#}")),
        });
        add::<(), _>(&mut reg, self, &["ack_crash"], Window, |c, _| {
            c.crash.lock().recover().take();
            crate::crash::acknowledge(&c.app_dir);
            c.notify();
            Done::empty()
        });
        add::<(), _>(&mut reg, self, &["quit"], Local, |c, _| {
            let _ = c.ui.lock().recover().send(Ui::Quit);
            Done::empty()
        });
        add::<(), _>(&mut reg, self, &["check_update"], Window, |c, _| {
            c.check_update_in_background();
            Done::empty()
        });
        add::<(), _>(&mut reg, self, &["install_update"], Window, |c, _| c.install_update());
        add::<(), Answer<Nothing>>(&mut reg, self, &["autostart_on", "autostart_off"], Window, |c, r| {
            let on = r.cmd == "autostart_on";
            let res =
                std::env::current_exe().map_err(|e| e.to_string()).and_then(|exe| crate::autostart::set(on, &exe));
            c.notify();
            if res.is_ok() {
                log(if on { "start with Windows: on" } else { "start with Windows: off" });
            }
            res.map(|()| Nothing {}).into()
        });
        add::<(), _>(&mut reg, self, &["pause_voice", "resume_voice"], Window, |c, r| {
            VOICE_PAUSED.store(r.cmd == "pause_voice", Ordering::SeqCst);
            log(if r.cmd == "pause_voice" { "voice paused" } else { "voice resumed" });
            c.notify();
            Done::empty()
        });
        add::<(), _>(&mut reg, self, &["kill_switch"], Window, |c, _| {
            c.kill_switch();
            Done::empty()
        });
        add::<(), _>(&mut reg, self, &["download_models"], Window, |c, _| {
            download_models(c);
            Done::empty()
        });
        add::<(), _>(&mut reg, self, &["open_config"], Window, |c, _| {
            open_config(&c.config_path);
            Done::empty()
        });
        add::<(), _>(&mut reg, self, &["prospector_thresholds"], Window, |c, _| {
            crate::callouts::thresholds_view(&c.cfg().data_dir())
        });
        // The change as JSON text: `{"ore", "threshold"}` or `{"ore", "reset": true}`.
        add::<String, Answer<crate::callouts::Ores>>(&mut reg, self, &["prospector_set"], Window, |c, r| {
            let text = r.text.clone().unwrap_or_default();
            let res = std::fs::create_dir_all(c.cfg().data_dir())
                .map_err(|e| e.to_string())
                .and_then(|_| {
                    astromech_store::Store::open(&c.cfg().data_dir().join("astromech.sqlite"))
                        .map_err(|e| format!("opening the database: {e}"))
                })
                .and_then(|st| crate::callouts::change_thresholds(&st, &text));
            res.map(|t| {
                if let Some(s) = c.session.lock().recover().as_ref() {
                    *s.shared.prospector.write().recover() = t.clone();
                }
                log(&format!("prospector thresholds changed: {text}"));
                crate::callouts::thresholds_json(&t)
            })
            .into()
        });
        // The shopping list change as JSON text.
        add::<String, Answer<ShoppingSet>>(&mut reg, self, &["shopping_set"], Window, |c, r| {
            let text = r.text.clone().unwrap_or_default();
            let res = std::fs::create_dir_all(c.cfg().data_dir())
                .map_err(|e| e.to_string())
                .and_then(|_| {
                    astromech_store::Store::open(&c.cfg().data_dir().join("astromech.sqlite"))
                        .map_err(|e| format!("opening the database: {e}"))
                })
                .and_then(|st| crate::callouts::change_shopping(&st, &text));
            res.map(|list| {
                let materials = list.0.len();
                if let Some(s) = c.session.lock().recover().as_ref() {
                    *s.shared.shopping.write().recover() = list;
                }
                ShoppingSet { materials }
            })
            .into()
        });
        add::<(), _>(&mut reg, self, &["exobiology"], Window, |c, _| match c.session.lock().recover().as_ref() {
            Some(s) => {
                let known = s.shared.known_bio.get();
                Answer::Done(crate::biology::view(&s.hub.snapshot(), known.as_deref()))
            }
            None => Answer::failed("Elite isn't running."),
        });
        add::<(), _>(&mut reg, self, &["eddn_log"], Window, |c, _| {
            let cfg = c.cfg();
            eddn_log(&cfg.data_dir(), cfg.eddn.enabled, crate::pipeline::eddn_test(&cfg))
        });
        add::<(), _>(&mut reg, self, &["find_elite"], Window, |c, _| {
            Done::new(FindElite { found: crate::setup::find_elite(&c.cfg()) })
        });
        add::<Option<crate::config::Button>, _>(&mut reg, self, &["set_ptt", "set_kill_switch"], Window, |c, r| {
            c.edit_config(r)
        });
        add::<crate::setup::args::Mic, _>(&mut reg, self, &["set_mic"], Window, |c, r| c.edit_config(r));
        add::<crate::setup::args::Folders, _>(&mut reg, self, &["set_folders"], Window, |c, r| c.edit_config(r));
        add::<crate::setup::args::Voice, _>(&mut reg, self, &["set_voice"], Window, |c, r| c.edit_config(r));
        add::<crate::setup::args::Eddn, _>(&mut reg, self, &["set_eddn"], Window, |c, r| c.edit_config(r));
        add::<crate::setup::args::Callout, _>(&mut reg, self, &["set_callout"], Window, |c, r| c.edit_config(r));
        add::<(), _>(&mut reg, self, &["callouts"], Window, |c, _| crate::callouts::callouts_json(&c.cfg().callouts));
        add::<(), _>(&mut reg, self, &["edsm_status"], Window, |c, _| {
            let cfg = c.cfg();
            edsm_status(&cfg, c.secrets.as_ref())
        });
        add::<crate::setup::args::Edsm, _>(&mut reg, self, &["set_edsm"], Window, |c, r| c.set_edsm(r));
        add::<(), Answer<FlightLog>>(&mut reg, self, &["edsm_flight_log"], Window, |c, _| {
            let creds = edsm_credentials(&c.cfg().edsm, c.secrets.as_ref());
            match creds {
                Ok(Some(c)) => {
                    let api = astromech_eddn::edsm::api::HttpApi::with_timeout(
                        &format!("astromech/{}", env!("CARGO_PKG_VERSION")),
                        astromech_eddn::edsm::api::FLIGHT_LOG_TIMEOUT,
                    );
                    astromech_eddn::edsm::api::flight_log(&api, &c).map(|visits| FlightLog { visits }).into()
                }
                Ok(None) => Answer::failed("set your EDSM commander name and API key first"),
                Err(e) => Answer::failed(e),
            }
        });
        add::<(), _>(&mut reg, self, &["voice_settings"], Window, |c, _| {
            let v = c.cfg().voice;
            Done::new(VoiceSettings {
                speaker: v.speaker,
                speed: v.speed,
                colour: v.colour,
                voices: crate::speech::voice_choices(),
                speeds: [*crate::setup::SPEEDS.start(), *crate::setup::SPEEDS.end()],
            })
        });
        add::<(), _>(&mut reg, self, &["voice_preview"], Window, |c, _| c.preview_voice());
        add::<(), _>(&mut reg, self, &["mic_list"], Window, |c, _| {
            let cfg = c.cfg();
            let device = cal::configured_device(&cfg.voice.mic);
            let gate = cal::open_store(&cfg.data_dir()).ok().and_then(|s| cal::stored_gate(&s, &device));
            cal::mic_list(&cfg.voice.mic, cal::MicGate { device, min_level_dbfs: gate })
        });
        add::<(), _>(&mut reg, self, &["calibrate_start"], Window, |c, _| cal::start(&c.calibration));
        add::<(), _>(&mut reg, self, &["calibrate_record"], Window, |c, _| {
            let mic = c.cfg().voice.mic;
            cal::record(&c.calibration, &|ms| cal::record_mic(&mic, ms))
        });
        add::<(), _>(&mut reg, self, &["calibrate_finish"], Window, |c, _| {
            let data = c.cfg().data_dir();
            let v = cal::finish(&c.calibration, &|| cal::open_store(&data));
            if let Answer::Done(d) = &v {
                log(&format!(
                    "microphone: calibrated {:?} (min_level_dbfs {})",
                    d.value.device, d.value.min_level_dbfs
                ));
            }
            v
        });
        add::<(), _>(&mut reg, self, &["calibrate_cancel"], Window, |c, _| cal::cancel(&c.calibration));
        add::<(), _>(&mut reg, self, &["mic_check"], Window, |c, _| {
            let mic = c.cfg().voice.mic;
            cal::mic_check(&|ms| cal::record_mic(&mic, ms))
        });
        add::<(), _>(&mut reg, self, &["onboarding_done"], Window, |c, _| {
            let res = cal::open_store(&c.cfg().data_dir()).and_then(|s| crate::setup::mark_onboarded(&s));
            match res {
                Ok(()) => {
                    c.onboarded.store(true, Ordering::SeqCst);
                    c.notify();
                    Answer::ok(Nothing {})
                }
                Err(e) => Answer::failed(format!("{e:#}")),
            }
        });
        // The phrase to try, as text.
        add::<String, _>(&mut reg, self, &["voice_test"], Window, |c, r| {
            let text = text(r);
            let res = {
                let s = c.session.lock().recover();
                s.as_ref().map(|s| s.assistant.dry_run(&text, Source::Typed))
            };
            let result = res.unwrap_or_else(|| tester::dry_run_idle(&c.cfg(), &text));
            Done::new(Tried { result })
        });
        add::<(), _>(&mut reg, self, &["voice_test_poll"], Window, |c, _| {
            if TESTER.arm(Instant::now()) {
                log("voice tester open: push-to-talk is tested there, nothing is pressed");
            }
            let last = TESTER.last().map(|(seq, result)| Heard { seq, result });
            Done::new(Poll { listening: c.active(), last })
        });
        add::<(), _>(&mut reg, self, &["learned_phrases"], Window, |c, _| crate::learned::list(&c.cfg().data_dir()));
        // The phrase to forget, as text.
        add::<String, _>(&mut reg, self, &["learned_forget"], Window, |c, r| {
            let assistant = c.session.lock().recover().as_ref().map(|s| s.assistant.clone());
            let heard = r.text.clone().unwrap_or_default();
            log(&format!("learned phrase forgotten: {heard:?}"));
            crate::learned::forget(&c.cfg().data_dir(), assistant.as_deref(), &heard)
        });
        add::<(), _>(&mut reg, self, &["voice_test_stop"], Window, |_, _| {
            if TESTER.armed(Instant::now()) {
                log("voice tester closed: push-to-talk acts again");
            }
            TESTER.disarm();
            Done::empty()
        });
        add::<LiveRequest, Answer<Ran>>(&mut reg, self, &["voice_test_live"], Window, |c, r| {
            // The command id as text, or `{command, from}` for a lookup from a named system.
            let arg = |k: &str| r.args.as_ref().and_then(|a| a.get(k)).and_then(Value::as_str).map(str::to_string);
            let cmd = r.text.clone().or_else(|| arg("command")).unwrap_or_default();
            let from = arg("from");
            let runner = c.session.lock().recover().as_ref().map(|s| s.runner.clone());
            match runner {
                None => Answer::failed("Elite isn't running, so there's nothing to send keys to."),
                Some(runner) => {
                    log(&format!("voice tester: live run of {cmd} (from {from:?})"));
                    tester::live(&runner, &cmd, from.as_deref(), tester::SWITCH_WAIT)
                        .map(|message| Ran { message })
                        .into()
                }
            }
        });
        reg
    }

    /// One command through [`Core::commands`], as the control API runs it.
    #[cfg(test)]
    pub fn handle(self: &Arc<Self>, r: &Request) -> Value {
        self.commands().dispatch(r, &|_| {}).unwrap_or_else(|e| json!({"ok": false, "error": e}))
    }
}

/// The EDDN switch after a config edit: set the flag the feed and its worker read, and when
/// sharing goes off, empty the queue now. The worker would discard it on its next pass, but it
/// may be asleep for minutes, and switched back on by then it would send the old messages.
fn apply_eddn_switch(c: &Config) {
    let was = crate::pipeline::EDDN_SHARING.swap(c.eddn.enabled, Ordering::SeqCst);
    let p = c.data_dir().join(astromech_eddn::DB_FILE);
    if was && !c.eddn.enabled && p.is_file() {
        let now = astromech_eddn::worker::wall_now();
        if let Err(e) = astromech_eddn::Outbox::open(&p).and_then(|o| o.discard_all(now)) {
            log(&format!("EDDN outbox: {e}"));
        }
    }
}

/// The EDSM credentials from Settings: the commander name from the config and the API key
/// from the keyring. `None` if either is missing.
pub fn edsm_credentials(
    c: &crate::config::Edsm,
    secrets: &dyn astromech_store::secrets::Secrets,
) -> Result<Option<astromech_eddn::edsm::Credentials>, String> {
    let commander = c.commander.trim();
    if commander.is_empty() {
        return Ok(None);
    }
    let key = secrets.get(astromech_eddn::edsm::KEY_SECRET)?;
    Ok(key
        .filter(|k| !k.is_empty())
        .map(|api_key| astromech_eddn::edsm::Credentials { commander: commander.to_owned(), api_key }))
}

/// The EDSM switch at start and after an edit: the credentials while uploading is on (switched
/// on, with a commander name and an API key), and when it goes off, the queue is emptied now
/// (as for EDDN).
pub fn apply_edsm_switch(c: &Config, secrets: &dyn astromech_store::secrets::Secrets) {
    let creds = if c.edsm.enabled {
        edsm_credentials(&c.edsm, secrets).unwrap_or_else(|e| {
            log(&format!("EDSM: {e}"));
            None
        })
    } else {
        None
    };
    let on = creds.is_some();
    let was = crate::pipeline::EDSM.set(creds);
    let p = c.data_dir().join(astromech_eddn::edsm::DB_FILE);
    if was && !on && p.is_file() {
        let now = astromech_eddn::worker::wall_now();
        if let Err(e) = astromech_eddn::Outbox::open(&p).and_then(|o| o.with_label("edsm").discard_all(now)) {
            log(&format!("EDSM outbox: {e}"));
        }
    }
}

/// EDSM status for Settings → Data sharing: the switch, the commander name, whether an API key
/// is saved (never the key), whether uploading is on, and the queue and send log as for EDDN.
pub fn edsm_status(c: &Config, secrets: &dyn astromech_store::secrets::Secrets) -> Answer<Outbox<EdsmHead>> {
    let (has_key, key_error) = match secrets.get(astromech_eddn::edsm::KEY_SECRET) {
        Ok(k) => (k.is_some_and(|k| !k.is_empty()), None),
        Err(e) => (false, Some(e)),
    };
    let head = EdsmHead {
        enabled: c.edsm.enabled,
        commander: c.edsm.commander.clone(),
        has_key,
        key_error,
        uploading: crate::pipeline::EDSM.get().is_some(),
    };
    outbox_status(&c.data_dir().join(astromech_eddn::edsm::DB_FILE), head)
}

/// EDDN status for Settings → Data sharing and the send log for Diagnostics: the switch, queue
/// depth, last send and last error (a failed send, not a message discarded here), the gateway's answers and (apart, so they can't push those
/// off screen) the messages never queued. Reads `eddn.sqlite` only if it exists, so asking never
/// creates it.
pub fn eddn_log(data_dir: &Path, enabled: bool, test: bool) -> Answer<Outbox<EddnHead>> {
    outbox_status(&data_dir.join(astromech_eddn::DB_FILE), EddnHead { enabled, test })
}

/// `head` with an outbox's queue depth, last send, last error, send log and skips. Reads the
/// file only if it exists, so asking never creates it.
fn outbox_status<H>(p: &Path, head: H) -> Answer<Outbox<H>> {
    use astromech_eddn::Outcome::{Discarded, Dropped, GaveUp, Rejected, Retry, Sent};
    if !p.is_file() {
        return Answer::ok(Outbox {
            head,
            pending: 0,
            last_sent: None,
            last_error: None,
            log: vec![],
            skipped: vec![],
        });
    }
    let read = || -> Result<Outbox<H>> {
        let o = astromech_eddn::Outbox::open(p)?;
        // EDSM: errors logged before the commander name or key last changed are out of date.
        let cleared = o.meta(astromech_eddn::edsm::ERRORS_CLEARED)?.and_then(|t| t.parse::<i64>().ok());
        Ok(Outbox {
            pending: o.pending()?,
            last_sent: o.recent_of(&[Sent], 1)?.pop(),
            last_error: o.recent_of_after(&[Rejected, Retry, GaveUp], cleared.unwrap_or(0), 1)?.pop(),
            log: o.recent_of(&[Sent, Rejected, Retry, GaveUp, Discarded], 50)?,
            skipped: o.recent_of(&[Dropped], 50)?,
            head,
        })
    };
    read().map_err(|e| format!("reading {}: {e}", p.display())).into()
}

/// The core's replies that are not another module's (astromech-se44): their JSON is what the
/// window has always read (`core_replies_keep_their_json`).
mod replies {
    use super::*;

    /// The `status` reply.
    #[derive(Debug, Serialize, ts_rs::TS)]
    #[ts(rename = "CoreStatus")]
    pub struct Status {
        pub version: &'static str,
        pub pid: u32,
        pub game: Game,
        pub active: bool,
        pub starting: bool,
        pub voice: String,
        pub voice_paused: bool,
        pub killed: bool,
        pub autostart: bool,
        pub crash_pending: bool,
        pub status_line: String,
        pub app_dir: String,
        pub log_dir: String,
        pub config: String,
        pub config_note: Option<String>,
        pub setup_needed: Vec<&'static str>,
        /// "device key", e.g. "231D3201 Joy_4".
        pub ptt: Option<String>,
        pub kill_switch: Option<String>,
        pub mic: String,
        pub onboarded: bool,
        pub models_missing: bool,
        pub models_progress: Option<String>,
        pub models_dir: String,
        pub update: update::Snapshot,
        pub toasts: Vec<crate::callouts::Toast>,
        pub history: Option<crate::history::Progress>,
        /// The session's mining run; `null` while Elite isn't running.
        pub mining: Option<crate::hub::Mining>,
    }

    /// A config edit's reply: whether it applies only from the next game session.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Applies {
        pub applies_next_session: bool,
    }

    /// `voice_preview`: the line being said.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Previewed {
        pub line: &'static str,
    }

    /// The Route page's commands: the route as it is now.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct RouteReply {
        pub view: crate::route::View,
    }

    /// `route_set`: a name and a source, and the waypoints or, for an import, the text (a
    /// Spansh CSV or one system per line).
    #[derive(Debug, Deserialize, ts_rs::TS)]
    #[ts(optional_fields = nullable)]
    pub struct RouteSetArgs {
        pub name: Option<String>,
        pub source: Option<String>,
        pub text: Option<String>,
        pub waypoints: Option<Vec<crate::route::Waypoint>>,
    }

    /// `route_add`: a lookup result's Add stop.
    #[derive(Debug, Deserialize, ts_rs::TS)]
    #[ts(optional_fields = nullable)]
    pub struct RouteAddArgs {
        pub system: Option<String>,
        pub label: Option<String>,
        pub source: Option<String>,
    }

    /// `save_diagnostics`: the zip written.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Saved {
        pub path: String,
    }

    /// `shopping_set`: how many materials the list has now.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct ShoppingSet {
        pub materials: usize,
    }

    /// `find_elite`.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct FindElite {
        pub found: crate::setup::Found,
    }

    /// `edsm_flight_log`.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct FlightLog {
        pub visits: Vec<astromech_eddn::edsm::api::Visit>,
    }

    /// `voice_settings`: the configured voice and what Settings may pick.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct VoiceSettings {
        pub speaker: String,
        pub speed: f32,
        pub colour: astromech_voice::colour::Colour,
        pub voices: Vec<crate::speech::VoiceChoice>,
        /// The slowest and fastest speed.
        pub speeds: [f32; 2],
    }

    /// `voice_test`: what the phrase would do.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Tried {
        pub result: tester::DryRun,
    }

    /// `voice_test_poll`: whether voice is listening, and the last phrase push-to-talk heard.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Poll {
        pub listening: bool,
        pub last: Option<Heard>,
    }

    /// A phrase heard in the voice tester, numbered so the page shows each once.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Heard {
        pub seq: u64,
        pub result: tester::DryRun,
    }

    /// `voice_test_live`: the command id as the request's text, or a lookup from a named system.
    #[derive(Debug, Serialize, ts_rs::TS)]
    #[serde(untagged)]
    pub enum LiveRequest {
        Command(String),
        From(LiveFrom),
    }

    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct LiveFrom {
        pub command: String,
        pub from: String,
    }

    /// `voice_test_live`: what was done.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Ran {
        pub message: String,
    }

    /// An outbox's state with `H` (the switch and what else the service shows): queue depth,
    /// the last send, the last error, the send log and the messages never queued.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct Outbox<H> {
        #[serde(flatten)]
        pub head: H,
        pub pending: usize,
        pub last_sent: Option<astromech_eddn::outbox::LogEntry>,
        pub last_error: Option<astromech_eddn::outbox::LogEntry>,
        pub log: Vec<astromech_eddn::outbox::LogEntry>,
        pub skipped: Vec<astromech_eddn::outbox::LogEntry>,
    }

    /// `eddn_log`'s switch and whether messages go to the test schemas.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct EddnHead {
        pub enabled: bool,
        pub test: bool,
    }

    /// `edsm_status`'s settings: never the key, only whether one is saved.
    #[derive(Debug, Serialize, ts_rs::TS)]
    pub struct EdsmHead {
        pub enabled: bool,
        pub commander: String,
        pub has_key: bool,
        pub key_error: Option<String>,
        pub uploading: bool,
    }
}
pub use replies::*;

/// Open a URL or folder with the default handler.
pub fn open_path(target: &str) {
    #[cfg(windows)]
    let r = std::process::Command::new("explorer.exe").arg(target).spawn();
    #[cfg(not(windows))]
    let r = std::process::Command::new("xdg-open").arg(target).spawn();
    if let Err(e) = r {
        log(&format!("couldn't open {target}: {e}"));
    }
}

/// Show a file in Explorer (selected).
pub fn reveal(p: &Path) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("explorer.exe").arg(format!("/select,{}", p.display())).spawn();
    }
    #[cfg(not(windows))]
    open_path(&p.parent().unwrap_or(p).display().to_string());
}

/// Write the sample config if there is no config yet. The sample changes no setting: its
/// examples are comments, so a fresh install gets the same defaults as with no file.
fn write_sample_config(path: &Path) {
    if !path.exists() {
        if let Some(d) = path.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let _ = std::fs::write(path, crate::config::SAMPLE);
        log(&format!("wrote a sample config to {}; edit [controls], then quit and restart astromech", path.display()));
    }
}

/// Create the config from the sample if it doesn't exist, then open it in the editor.
pub fn open_config(path: &Path) {
    write_sample_config(path);
    #[cfg(windows)]
    let r = std::process::Command::new("notepad.exe").arg(path).spawn();
    #[cfg(not(windows))]
    let r = std::process::Command::new("xdg-open").arg(path).spawn();
    if let Err(e) = r {
        log(&format!("couldn't open {}: {e}", path.display()));
    }
}

/// Download the speech models in the background, showing progress in the status line.
pub fn download_models(core: &Arc<Core>) {
    if core.models.lock().recover().is_some() {
        return;
    }
    *core.models.lock().recover() = Some("downloading speech models…".into());
    core.notify();
    let c = core.clone();
    let _ = std::thread::Builder::new().name("models".into()).spawn(move || {
        use astromech_voice::models;
        let base = c.cfg().models_dir();
        let mut err = None;
        for a in models::missing(&base) {
            let mut last = u64::MAX;
            let r = models::download(&base, &a, &mut |p| {
                let pct = (p.done * 100).checked_div(p.total).unwrap_or(0);
                if pct != last {
                    last = pct;
                    *c.models.lock().recover() = Some(format!("{} {:?} {pct}%", a.label, p.phase));
                    c.notify();
                }
            });
            if let Err(e) = r {
                err = Some(format!("{e:#}"));
                break;
            }
        }
        match err {
            None => {
                log(&format!("speech models ready in {}", base.display()));
                remove_retired_models(&c.cfg());
            }
            Some(e) => log(&format!("speech model download failed: {e}")),
        }
        *c.models.lock().recover() = None;
        c.notify();
    });
}

/// Delete models an update replaced (Kokoro v0.19) once Kokoro v1.0 is completely downloaded,
/// but only from astromech's own models folder: a folder `models_dir` points at may belong to
/// another tool (the S9 test still uses v0.19). Returns the folders removed.
pub fn remove_retired_models(cfg: &Config) -> Vec<PathBuf> {
    use astromech_voice::models;
    let dir = cfg.models_dir();
    if !cfg.voice.models_dir.trim().is_empty() {
        if models::has_retired(&dir) && models::find_tts(&dir).is_ok() {
            log(&format!(
                "Kokoro v0.19 in {} is no longer used; left in place because models_dir points at that folder",
                dir.display()
            ));
        }
        return vec![];
    }
    let gone = models::remove_retired(&dir);
    for d in &gone {
        log(&format!("removed the replaced speech model {}", d.display()));
    }
    gone
}

pub fn models_missing(cfg: &Config) -> bool {
    !astromech_voice::models::missing(&cfg.models_dir()).is_empty()
}

fn watch_game(core: Arc<Core>) {
    let mut w = gamewatch::Watcher::default();
    let jdir = core.cfg().journal_dir();
    let mut journal_seen = jdir.as_deref().and_then(gamewatch::newest_journal).map(|(_, t)| t);
    loop {
        let names = gamewatch::process_names();
        let change = w.poll(&names);
        *core.game.lock().recover() = w.game;
        match change {
            Some(Change::LauncherOpened) => {
                log("Elite launcher (EDLaunch.exe) opened");
                let c = core.clone();
                std::thread::spawn(move || c.activate("pre-warming for the launcher"));
            }
            Some(Change::GameStarted) => {
                log(&format!("Elite started ({})", gamewatch::GAME_EXE));
                let c = core.clone();
                std::thread::spawn(move || c.activate("Elite running"));
            }
            Some(Change::GameExited) => {
                log("Elite exited");
                core.deactivate("Elite exited");
            }
            Some(Change::LauncherClosed) => {
                log("Elite launcher closed without starting the game");
                core.deactivate("launcher closed");
            }
            None => {}
        }
        if w.game == Game::Running {
            if let Some((p, t)) = jdir.as_deref().and_then(gamewatch::newest_journal) {
                if journal_seen.is_none_or(|s| t > s) {
                    if journal_seen.is_none_or(|s| s < t - Duration::from_secs(60)) {
                        log(&format!(
                            "journal active ({}): game session confirmed",
                            p.file_name().unwrap_or_default().to_string_lossy()
                        ));
                    }
                    journal_seen = Some(t);
                }
            }
        }
        if change.is_some() {
            core.notify();
        }
        std::thread::sleep(gamewatch::POLL);
    }
}

/// Run the core until "Quit". Returns the exit code.
pub fn run(opts: Opts) -> i32 {
    let paths = Paths::current();
    let app_dir = paths.app_dir().to_path_buf();
    let _ = std::fs::create_dir_all(&app_dir);
    crate::logging::init(&paths.log_dir(), false);
    crate::crash::install(&app_dir);

    // Single instance: a second start opens the window of the first, then exits.
    let Some(_guard) = crate::instance::acquire(&app_dir) else {
        if !opts.background {
            match control::call_short(&app_dir, "open_window", opts.route.as_deref()) {
                Ok(_) => log("already running: asked it to open the window"),
                Err(e) => log(&format!("already running, but it didn't answer: {e:#}")),
            }
        }
        return 0;
    };
    log(&format!(
        "astromech-core {} starting{} (pid {})",
        env!("CARGO_PKG_VERSION"),
        if opts.background { " in the background" } else { "" },
        std::process::id()
    ));

    let config_path = opts.config.clone().unwrap_or_else(|| paths.config_file());
    let config_exists = config_path.is_file();
    let (mut cfg, config_watch) = if config_exists {
        match Config::load_report(&config_path) {
            Ok((c, problems)) => {
                log(&format!("config: {}", config_path.display()));
                for p in &problems {
                    log(&format!("config: ignoring {p}; using the default for it"));
                }
                (c, ConfigWatch::loaded(&config_path, config_problems_note(&problems), false))
            }
            Err(e) => {
                log(&format!("config error, using defaults: {e:#}"));
                (Config::default(), ConfigWatch::loaded(&config_path, Some(config_unreadable_note(&e)), true))
            }
        }
    } else {
        log(&format!("config: {} not found; using defaults (Settings → Open config file)", config_path.display()));
        (Config::default(), ConfigWatch::default())
    };
    if opts.session.dry_run {
        cfg.actions.dry_run = true;
    }

    if !paths.portable() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(m) = crate::autostart::apply_default(&app_dir, &exe) {
                log(&m);
            }
        }
    }

    let crash = crate::crash::pending(&app_dir);
    if crash.is_some() {
        log("astromech crashed last time; the tray offers Save diagnostics");
    }

    let mut update_handle = update::Handle::new();
    let apply_err = update::apply_error_file(&app_dir);
    if let Ok(e) = std::fs::read_to_string(&apply_err) {
        let _ = std::fs::remove_file(&apply_err);
        log(&format!("update: the last install failed: {e}"));
        update_handle.snap.apply_error = Some(e.trim().to_string());
    }

    crate::pipeline::EDDN_SHARING.store(cfg.eddn.enabled, Ordering::SeqCst);
    let secrets: Arc<dyn astromech_store::secrets::Secrets> = Arc::new(astromech_store::secrets::OsKeyring::new());
    apply_edsm_switch(&cfg, secrets.as_ref());
    let route = Arc::new(crate::route::Desk::in_dir(&cfg.data_dir()));
    let data_dir = cfg.data_dir();
    // Saved binding overrides used to live in the app dir whatever `data_dir` said; carry them
    // across once (never overwriting). A failed copy only means the old overrides aren't seen.
    if let Err(e) = paths.carry_over("device-overrides.json", &data_dir) {
        log(&format!("could not carry device-overrides.json over to {}: {e}", data_dir.display()));
    }
    let (ui_tx, ui_rx) = channel();
    let openable = vec![app_dir.clone(), paths.log_dir()];
    let remote = remote_client(&data_dir);
    let core = Arc::new_cyclic(|me| Core {
        api: Arc::new(api_state(me.clone(), data_dir, remote, openable)),
        app_dir: app_dir.clone(),
        config_path,
        onboarded: AtomicBool::new(
            crate::calibration::open_store(&cfg.data_dir())
                .map(|s| crate::setup::onboarded_at_start(&s, config_exists, &cfg))
                .unwrap_or(false),
        ),
        config_lock: Mutex::new(()),
        preview: Default::default(),
        cfg: std::sync::RwLock::new(cfg),
        config_watch: Mutex::new(config_watch),
        game: Mutex::new(Game::NotRunning),
        session: Mutex::new(None),
        starting: AtomicBool::new(false),
        voice: Mutex::new("voice not loaded".into()),
        crash: Mutex::new(crash),
        models: Mutex::new(None),
        update: Mutex::new(update_handle),
        ui: Mutex::new(ui_tx),
        opts: opts.clone(),
        calibration: Mutex::new(None),
        history: Default::default(),
        secrets,
        route,
    });

    let _server = match control::serve(&app_dir, Arc::new(core.commands())) {
        Ok(s) => {
            log(&format!("control API on 127.0.0.1:{}", s.endpoint.port));
            Some(s)
        }
        Err(e) => {
            log(&format!("control API unavailable: {e:#}"));
            None
        }
    };

    // An update from Kokoro v0.19: fetch v1.0 (the old model can't serve v1.0 voices), and
    // remove v0.19 once v1.0 is there.
    let models_dir = core.cfg().models_dir();
    if astromech_voice::models::has_retired(&models_dir) {
        if astromech_voice::models::find_tts(&models_dir).is_ok() {
            remove_retired_models(&core.cfg());
        } else {
            log("text-to-speech: Kokoro v0.19 was replaced by v1.0; downloading it");
            download_models(&core);
        }
    }

    // Under the config lock: a `set_folders` the control API serves meanwhile isn't undone.
    {
        let _g = core.config_lock.lock().unwrap_or_else(|e| e.into_inner());
        core.follow_history(&core.cfg());
    }
    if core.history.status().is_none() {
        log("history: no journal folder; set game.journal_dir");
    }

    let c = core.clone();
    std::thread::Builder::new().name("gamewatch".into()).spawn(move || watch_game(c)).expect("spawn game watcher");

    // Release the voice preview's Kokoro, and what the command layer loaded (the neutron star
    // file, the journal caches, the in-memory star map), once they have sat unused.
    let c = core.clone();
    std::thread::Builder::new()
        .name("preview-idle".into())
        .spawn(move || loop {
            std::thread::sleep(Duration::from_secs(10));
            if c.preview.release_idle(Instant::now(), crate::speech::PREVIEW_IDLE) {
                log("voice preview: Kokoro released (unused for a minute)");
            }
            c.api.release_idle(Instant::now(), astromech_api::state::IDLE);
        })
        .expect("spawn preview reaper");

    if core.cfg().update.check {
        let c = core.clone();
        std::thread::Builder::new()
            .name("update".into())
            .spawn(move || {
                std::thread::sleep(update::STARTUP_DELAY);
                loop {
                    c.check_update(false);
                    std::thread::sleep(update::CHECK_EVERY);
                }
            })
            .expect("spawn update checker");
    }

    if !opts.background && !paths.portable() {
        let _ = core.open_window(opts.route.as_deref());
    }

    let code = ui_loop(core.clone(), ui_rx);
    log("quitting");
    core.deactivate("quit");
    core.apply_on_quit();
    code
}

#[cfg(windows)]
fn ui_loop(core: Arc<Core>, rx: Receiver<Ui>) -> i32 {
    match crate::tray::run(core.clone(), &rx) {
        Ok(()) => 0,
        Err(e) => {
            log(&format!("tray icon failed: {e:#}; running without it until Quit (via the app window)"));
            headless(&rx)
        }
    }
}

#[cfg(not(windows))]
fn ui_loop(_core: Arc<Core>, rx: Receiver<Ui>) -> i32 {
    log("no tray on this platform; running headless (quit via the control API or Ctrl+C)");
    headless(&rx)
}

fn headless(rx: &Receiver<Ui>) -> i32 {
    while let Ok(m) = rx.recv() {
        if m == Ui::Quit {
            break;
        }
    }
    0
}

#[cfg(test)]
mod every_window_command;

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_voice::{catalog, download, models};

    /// Held by tests that edit the config through a core: every edit sets the process-wide
    /// EDDN switch. Also held by tests that set the process-wide voice pause or voice tester,
    /// which `status` reports.
    pub(super) static CONFIG_EDITS: Mutex<()> = Mutex::new(());

    /// A core as `run` builds it, on the config file `text` in `dir`, without its threads.
    fn core(dir: &Path, text: &str) -> Arc<Core> {
        core_with(dir, text, offline())
    }

    /// A client whose every request fails, for tests that look nothing up.
    pub(super) fn offline() -> Arc<astromech_data::Client> {
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
        // With no spacing between requests: a refused one is sent at once, not after a wait.
        let client =
            astromech_data::Client::with_transport(astromech_data::Cache::in_memory().unwrap(), Arc::new(Offline));
        Arc::new(client.limiter(astromech_data::Limiter::none()))
    }

    const NO_WAIT: astromech_data::spansh::Polling =
        astromech_data::spansh::Polling { every: Duration::ZERO, timeout: Duration::from_secs(5) };

    /// [`core`] with its lookups going through `remote`.
    fn core_with(dir: &Path, text: &str, remote: Arc<astromech_data::Client>) -> Arc<Core> {
        core_with_state(dir, text, remote, |s| s)
    }

    /// [`core_with`], with the command layer's state passed through `state` (a test's opener
    /// or devices).
    pub(super) fn core_with_state(
        dir: &Path,
        text: &str,
        remote: Arc<astromech_data::Client>,
        state: impl FnOnce(astromech_api::State) -> astromech_api::State,
    ) -> Arc<Core> {
        let config_path = dir.join("astromech.toml");
        std::fs::write(&config_path, text).unwrap();
        let data_dir = Config::load(&config_path).unwrap().data_dir();
        Arc::new_cyclic(|me| Core {
            // Recorded Spansh answers need no wait between polls.
            api: Arc::new(state(api_state(me.clone(), data_dir, remote, vec![]).plot_polling(NO_WAIT))),
            app_dir: dir.to_path_buf(),
            cfg: std::sync::RwLock::new(Config::load(&config_path).unwrap()),
            config_path,
            config_lock: Mutex::new(()),
            preview: Default::default(),
            config_watch: Mutex::new(ConfigWatch::default()),
            game: Mutex::new(Game::NotRunning),
            session: Mutex::new(None),
            starting: AtomicBool::new(false),
            voice: Mutex::new(String::new()),
            crash: Mutex::new(None),
            models: Mutex::new(None),
            update: Mutex::new(update::Handle::new()),
            ui: Mutex::new(channel().0),
            opts: Opts::default(),
            calibration: Mutex::new(None),
            onboarded: AtomicBool::new(false),
            history: Default::default(),
            secrets: Arc::new(astromech_store::secrets::MemorySecrets::default()),
            route: Arc::new(crate::route::Desk::open(None, Box::new(|_| Ok(())))),
        })
    }

    #[test]
    fn the_sample_config_written_by_open_config_changes_no_setting() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("data").join("astromech.toml");
        write_sample_config(&p);
        let written = std::fs::read_to_string(&p).unwrap();
        assert_eq!(written, crate::config::SAMPLE);
        let loaded = Config::load(&p).unwrap();
        assert_eq!(toml::to_string(&loaded).unwrap(), toml::to_string(&Config::default()).unwrap());
        assert!(loaded.controls.kill_switch.is_none() && loaded.controls.ptt.is_none());
        // The kill switch example is there, as a comment only.
        assert!(written.lines().any(|l| l.starts_with("# kill_switch = ") && l.contains("Key_ScrollLock")));
        assert!(!written.lines().any(|l| l.trim_start().starts_with("kill_switch")));
        // An existing config is left alone.
        std::fs::write(&p, "data_dir = \"x\"").unwrap();
        write_sample_config(&p);
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "data_dir = \"x\"");
    }

    /// The Route page's commands, through the control API: set from waypoints or pasted text,
    /// skip, go back, clear, resume; a route with no systems is an error.
    #[test]
    fn route_commands_over_the_control_api() {
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        let call = |cmd: &str, args: Option<Value>| {
            core.handle(&Request { token: String::new(), cmd: cmd.into(), route: None, text: None, args, origin: None })
        };
        assert_eq!(call("route", None)["view"]["route"], Value::Null);
        let v = call(
            "route_set",
            Some(json!({"name": "Trip", "source": "engineer trip", "waypoints": [
                {"system": "Deciat", "label": "Felicity Farseer"}, {"system": "Kuk"}]})),
        );
        assert_eq!(v["view"]["next"]["system"], "Deciat", "{v}");
        assert_eq!(v["view"]["route"]["source"], "engineer trip");
        assert_eq!(v["view"]["jumps_left"], Value::Null, "an engineer trip knows no jumps");
        assert_eq!(call("route_skip", None)["view"]["next"]["system"], "Kuk");
        assert_eq!(call("route_back", None)["view"]["remaining"], 2);
        let v = call("route_set", Some(json!({"text": "System Name,Jumps\nSol,0\nColonia,40\n"})));
        assert_eq!(v["view"]["route"]["name"], "Route");
        assert_eq!(v["view"]["route"]["waypoints"][1]["system"], "Colonia", "{v}");
        assert_eq!(call("route_clear", None)["view"]["route"], Value::Null);
        assert_eq!(call("route_set", Some(json!({"text": " "})))["ok"], false);
        // Resume: the route a galaxy-map plot parked.
        assert_eq!(call("route_resume", None)["ok"], false);
        call("route_set", Some(json!({"text": "Sol\nColonia"})));
        core.route.plotted(&["Sol".into(), "Wolf 359".into()]);
        assert_eq!(call("route", None)["view"]["parked"]["waypoints"][1]["system"], "Colonia");
        let v = call("route_resume", None);
        assert_eq!(v["view"]["route"]["waypoints"][1]["system"], "Colonia", "{v}");
        assert_eq!(v["view"]["parked"], Value::Null);
    }

    /// A lookup result's two actions: Add stop (`route_add`) makes it the next stop and copies
    /// it; Set destination (`route_set` with one stop) parks the route it replaces for Resume.
    #[test]
    fn add_stop_and_set_destination_over_the_control_api() {
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        let call = |cmd: &str, args: Value| {
            core.handle(&Request {
                token: String::new(),
                cmd: cmd.into(),
                route: None,
                text: None,
                args: Some(args),
                origin: None,
            })
        };
        let trip =
            json!({"name": "Trip", "source": "engineer trip", "waypoints": [{"system": "Deciat"}, {"system": "Kuk"}]});
        call("route_set", trip);
        let v = call(
            "route_add",
            json!({"system": "Shinrarta Dezhra", "label": "Interstellar Factors", "source": "search"}),
        );
        assert_eq!(v["view"]["next"], json!({"system": "Shinrarta Dezhra", "label": "Interstellar Factors"}), "{v}");
        assert_eq!(v["view"]["route"]["name"], "Trip");
        assert_eq!(v["view"]["remaining"], 3);
        assert_eq!(call("route_add", json!({"system": " "}))["ok"], false);

        let v = call("route_set", json!({"name": "Sol", "source": "search", "waypoints": [{"system": "Sol"}]}));
        assert_eq!(v["view"]["next"]["system"], "Sol");
        assert_eq!(v["view"]["parked"]["name"], "Trip", "{v}");
        let v = call("route_resume", json!({}));
        assert_eq!(v["view"]["next"]["system"], "Shinrarta Dezhra", "{v}");

        call("route_clear", json!({}));
        let v = call("route_add", json!({"system": "Colonia", "source": "search"}));
        assert_eq!(v["view"]["route"]["name"], "Colonia", "{v}");
        assert_eq!(v["view"]["remaining"], 1);
    }

    /// Test mode is on the tray status line, over whatever the game is doing.
    #[test]
    fn the_status_line_says_when_push_to_talk_is_only_testing() {
        // The line also says when voice is paused, which another test sets for a moment.
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        assert_eq!(core.status_line_for(false), "Waiting for Elite");
        assert_eq!(core.status_line_for(true), "Voice tester open · push-to-talk testing only");
        *core.game.lock().recover() = Game::Running;
        assert!(core.status_line_for(false).starts_with("Elite running"));
        assert_eq!(core.status_line_for(true), "Voice tester open · push-to-talk testing only");
    }

    /// Spansh's answers, recorded (astromech-data's fixtures): the submit, then the polls in turn.
    struct RecordedJob(Mutex<Vec<(u16, &'static str)>>);

    impl astromech_data::Transport for RecordedJob {
        fn send(
            &self,
            _url: &str,
            _req: &astromech_data::Request,
        ) -> Result<astromech_data::Response, astromech_data::TransportError> {
            let (status, body) = self.0.lock().recover().remove(0);
            Ok(astromech_data::Response { status, body: body.into() })
        }
    }

    /// A neutron and an exact route plotted from recorded Spansh jobs go to the follower as the
    /// app window sends them (`route_set` with the plotted route): the commander, in Sol, gets
    /// the first stop after it on the clipboard, jumping into it moves the route on, and "next
    /// waypoint" says what to do there.
    #[test]
    fn plotted_routes_load_into_the_follower() {
        use astromech_galaxy::plot::{plot, Ask, Plotter, Ship};
        macro_rules! fixture {
            ($name:literal) => {
                include_str!(concat!("../../astromech-data/tests/fixtures/", $name))
            };
        }
        let fsd = astromech_ship::Fsd {
            optimal_mass: 1470.0,
            max_fuel_per_jump: 5.0,
            fuel_multiplier: 0.012,
            fuel_power: 2.45,
            base_mass: 400.63,
            tank_size: 32.0,
            reserve_size: 0.63,
            range_boost: 0.0,
            supercharge_multiplier: 4,
        };
        let ship = Ship { name: "Asp".into(), range: fsd.range(0.0), fsd, cargo: 0.0, unresolved: vec![] };
        let plotted = |plotter: Plotter, to: &str, answers: Vec<(u16, &'static str)>| {
            let client = astromech_data::Client::with_transport(
                astromech_data::Cache::in_memory().unwrap(),
                Arc::new(RecordedJob(Mutex::new(answers))),
            )
            .limiter(astromech_data::Limiter::none());
            let ask = Ask { plotter, to: to.into(), efficiency: 60, refuel_every_scoopable: true };
            serde_json::to_value(plot(&client, "Sol", &ask, &ship, NO_WAIT).unwrap()).unwrap()
        };

        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        let set = |args: Value| {
            core.handle(&Request {
                token: String::new(),
                cmd: "route_set".into(),
                route: None,
                text: None,
                args: Some(args),
                origin: None,
            })
        };
        core.route.set_here("Sol");

        let neutron = plotted(
            Plotter::Neutron,
            "Colonia",
            vec![
                (202, fixture!("spansh-route-queued.json")),
                (202, fixture!("spansh-route-pending.json")),
                (200, fixture!("spansh-route-result.json")),
            ],
        );
        let v = set(neutron);
        assert_eq!(v["view"]["route"]["name"], "Sol to Colonia", "{v}");
        assert_eq!(v["view"]["route"]["source"], "neutron plotter");
        assert_eq!(v["view"]["next"], json!({"system": "PSR J1752-2806", "label": "neutron boost", "jumps": 10}));
        // Each stop brought its leg's jumps: 10 to the neutron star, 1 on to Colonia.
        assert_eq!((v["view"]["remaining"].clone(), v["view"]["jumps_left"].clone()), (json!(2), json!(11)));

        let exact = plotted(
            Plotter::Exact,
            "Maia",
            vec![
                (202, fixture!("spansh-exact-submit.json")),
                (202, fixture!("spansh-exact-pending.json")),
                (200, fixture!("spansh-exact-result.json")),
            ],
        );
        let v = set(exact);
        assert_eq!(v["view"]["route"]["source"], "exact plotter", "{v}");
        assert_eq!(v["view"]["next"], json!({"system": "Tascheter Sector MN-T b3-6", "label": "fuel", "jumps": 1}));
        assert_eq!((v["view"]["remaining"].clone(), v["view"]["jumps_left"].clone()), (json!(9), json!(9)));
        let moved = core.route.on_system("Tascheter Sector MN-T b3-6").unwrap();
        assert_eq!(moved.spoken(), "Next waypoint. Tascheter Sector LS-T a3-0.");
        for s in [
            "Tascheter Sector LS-T a3-0",
            "Arietis Sector LD-H a11-1",
            "Arietis Sector UE-O a7-1",
            "Arietis Sector EG-V a3-1",
        ] {
            core.route.on_system(s);
        }
        assert_eq!(
            crate::route::command_reply(&core.route, crate::route::Command::Next),
            "Next waypoint. Hyades Sector DB-W b2-6. For fuel. 4 waypoints left."
        );
    }

    /// First run with no journal folder found: onboarding sets one (`set_folders`) and the
    /// history imports it without a restart (astromech-5t34).
    #[test]
    fn a_journal_folder_set_after_startup_is_imported() {
        use crate::history::tests::{journal_dir, wait_for};
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let data = d.path().join("data");
        // The configured folder doesn't exist: the first-run case, and never the real default
        // under %USERPROFILE% (a PC with journals there would import them).
        let missing = d.path().join("no journals here");
        let core = core(
            d.path(),
            &format!("data_dir = '{}'\n[game]\njournal_dir = '{}'\n", data.display(), missing.display()),
        );
        core.follow_history(&core.cfg());
        assert_eq!(core.cfg().journal_dir(), Some(missing));
        wait_for("the empty first import", || core.history.status().is_some_and(|p| p.ready));
        assert_eq!(core.history.status().unwrap().events, 0);

        let j = journal_dir();
        let r = core.handle(&Request {
            token: String::new(),
            cmd: "set_folders".into(),
            route: None,
            text: None,
            args: Some(json!({"journal_dir": j.path().display().to_string()})),
            origin: None,
        });
        assert_eq!(r["ok"], json!(true), "{r}");
        wait_for("the journals set in onboarding", || core.history.status().is_some_and(|p| p.ready && p.events == 52));
        let h = astromech_history::History::open_read_only(&data.join(crate::history::DB_FILE)).unwrap();
        assert_eq!(h.event_count().unwrap(), 52);
        assert_eq!(core.status_json()["history"]["events"], json!(52), "the app sees it in the status reply");
    }

    #[test]
    fn retired_models_stay_in_a_folder_models_dir_points_at() {
        let d = tempfile::tempdir().unwrap();
        let m = d.path();
        for a in [catalog::RETIRED_TTS[0], models::TTS] {
            let root = models::model_root(m, &a);
            std::fs::create_dir_all(root.join("espeak-ng-data")).unwrap();
            for f in ["model.onnx", "voices.bin", "tokens.txt"] {
                std::fs::write(root.join(f), "x").unwrap();
            }
            std::fs::write(download::marker_in(m, &a), a.sha256).unwrap();
        }
        let mut cfg = Config::default();
        cfg.voice.models_dir = m.display().to_string();
        assert!(remove_retired_models(&cfg).is_empty());
        assert!(models::has_retired(m), "another tool's v0.19 is left alone");
        assert!(models::find_tts(m).is_ok());
    }

    #[test]
    fn eddn_log_reads_the_outbox_without_creating_it() {
        let eddn_log = |d: &Path, on, test| serde_json::to_value(super::eddn_log(d, on, test)).unwrap();
        let d = tempfile::tempdir().unwrap();
        let v = eddn_log(d.path(), false, true);
        assert_eq!(
            (&v["ok"], &v["enabled"], &v["test"], &v["pending"]),
            (&json!(true), &json!(false), &json!(true), &json!(0))
        );
        assert!(v["last_error"].is_null() && v["log"] == json!([]));
        assert!(!d.path().join(astromech_eddn::DB_FILE).exists());
        let o = astromech_eddn::Outbox::open(&d.path().join(astromech_eddn::DB_FILE)).unwrap();
        let r = "https://eddn.edcd.io/schemas/journal/1";
        o.enqueue(r, b"m", 10).unwrap();
        o.log(11, r, astromech_eddn::Outcome::Sent, "").unwrap();
        o.log(12, r, astromech_eddn::Outcome::Retry, "HTTP 503: down; retry in 60 s").unwrap();
        o.log(13, r, astromech_eddn::Outcome::Dropped, "beta").unwrap();
        let v = eddn_log(d.path(), true, false);
        assert_eq!(v["pending"], 1);
        assert_eq!((&v["last_sent"]["at"], &v["last_error"]["at"]), (&json!(11), &json!(12)));
        assert_eq!(v["last_error"]["detail"], "HTTP 503: down; retry in 60 s", "the status shows the last failed send");
        let outcomes: Vec<&Value> = v["log"].as_array().unwrap().iter().map(|e| &e["outcome"]).collect();
        assert_eq!(outcomes, [&json!("retry"), &json!("sent")], "skips are listed apart");
        assert_eq!(v["skipped"][0]["detail"], "beta");

        // Messages discarded here (switched off, outbox full) are listed, but aren't send errors.
        o.discard_all(14).unwrap();
        for i in 0..astromech_eddn::outbox::MAX_QUEUED + 1 {
            o.enqueue(r, b"m", 15 + i).unwrap();
        }
        let v = eddn_log(d.path(), true, false);
        assert_eq!(v["last_error"]["at"], 12, "{}", v["last_error"]);
        let details: Vec<&Value> = v["log"].as_array().unwrap().iter().take(2).map(|e| &e["detail"]).collect();
        assert_eq!(details, [&json!("outbox full: 1 oldest dropped"), &json!("switched off: 1 queued discarded")]);

        // EDSM: changing the commander name or key makes the errors before it out of date.
        o.set_meta(astromech_eddn::edsm::ERRORS_CLEARED, &o.last_log_id().unwrap().to_string()).unwrap();
        assert!(eddn_log(d.path(), true, false)["last_error"].is_null());
        // An error from the same second, logged after the change (a key that is still wrong), shows.
        let at = v["log"][0]["at"].as_i64().unwrap();
        o.log(at, r, astromech_eddn::Outcome::Retry, "still wrong").unwrap();
        assert_eq!(eddn_log(d.path(), true, false)["last_error"]["detail"], "still wrong");
    }

    fn set_edsm(core: &Arc<Core>, args: Value) -> Value {
        core.handle(&Request {
            token: String::new(),
            cmd: "set_edsm".into(),
            route: None,
            text: None,
            args: Some(args),
            origin: None,
        })
    }

    #[test]
    fn edsm_settings_keep_the_key_in_the_keyring_and_upload_only_with_both() {
        use crate::pipeline::EDSM;
        use astromech_eddn::edsm::{DB_FILE, KEY_SECRET};
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let missing = d.path().join("no journals here");
        let core = core(
            d.path(),
            &format!("data_dir = '{}'\n[game]\njournal_dir = '{}'\n", d.path().display(), missing.display()),
        );
        let status = |core: &Arc<Core>| {
            core.handle(&Request {
                token: String::new(),
                cmd: "edsm_status".into(),
                route: None,
                text: None,
                args: None,
                origin: None,
            })
        };
        let v = status(&core);
        assert_eq!(
            (&v["enabled"], &v["has_key"], &v["uploading"], &v["pending"]),
            (&json!(false), &json!(false), &json!(false), &json!(0))
        );
        assert!(!d.path().join(DB_FILE).exists(), "asking doesn't create the queue");

        // Switched on with a name but no key: not uploading.
        assert_eq!(set_edsm(&core, json!({"enabled": true, "commander": "Jameson"}))["ok"], true);
        assert!(core.cfg().edsm.enabled && EDSM.get().is_none());
        assert_eq!(status(&core)["uploading"], false);

        // The key goes to the keyring only, and uploading starts at once.
        assert_eq!(set_edsm(&core, json!({"api_key": " s3cret-key "}))["ok"], true);
        assert_eq!(core.secrets.get(KEY_SECRET).unwrap().as_deref(), Some("s3cret-key"));
        let file = std::fs::read_to_string(&core.config_path).unwrap();
        assert!(!file.contains("s3cret"), "{file}");
        assert_eq!(EDSM.get().map(|c| (c.commander, c.api_key)), Some(("Jameson".into(), "s3cret-key".into())));
        let v = status(&core);
        assert_eq!((&v["has_key"], &v["uploading"], &v["commander"]), (&json!(true), &json!(true), &json!("Jameson")));
        assert!(!v.to_string().contains("s3cret"), "the status never shows the key");

        // Switched off: the queue goes at once.
        let o = astromech_eddn::Outbox::open(&d.path().join(DB_FILE)).unwrap();
        o.enqueue("FSDJump", b"{}", 10).unwrap();
        assert_eq!(set_edsm(&core, json!({"enabled": false}))["ok"], true);
        assert!(EDSM.get().is_none());
        assert_eq!(o.pending().unwrap(), 0);
        assert_eq!(o.recent(1).unwrap()[0].detail, "switched off: 1 queued discarded");

        // An empty key forgets it.
        set_edsm(&core, json!({"enabled": true}));
        assert!(EDSM.get().is_some());
        set_edsm(&core, json!({"api_key": ""}));
        assert_eq!(core.secrets.get(KEY_SECRET).unwrap(), None);
        assert!(EDSM.get().is_none());
        assert_eq!(set_edsm(&core, json!({}))["ok"], false);
        EDSM.set(None);
    }

    fn set_eddn(core: &Arc<Core>, enabled: bool) -> Value {
        core.handle(&Request {
            token: String::new(),
            cmd: "set_eddn".into(),
            route: None,
            text: None,
            args: Some(json!({ "enabled": enabled })),
            origin: None,
        })
    }

    #[test]
    fn set_eddn_flips_the_shared_switch_and_switching_off_empties_the_queue_at_once() {
        use crate::pipeline::EDDN_SHARING;
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let missing = d.path().join("no journals here");
        let core = core(
            d.path(),
            &format!("data_dir = '{}'\n[game]\njournal_dir = '{}'\n", d.path().display(), missing.display()),
        );
        assert_eq!(set_eddn(&core, true)["ok"], true);
        assert!(EDDN_SHARING.load(Ordering::SeqCst));
        assert!(core.cfg().eddn.enabled);

        // Queued while the gateway is down: the worker sleeps until the retry is due.
        let o = astromech_eddn::Outbox::open(&d.path().join(astromech_eddn::DB_FILE)).unwrap();
        let r = "https://eddn.edcd.io/schemas/journal/1";
        o.enqueue(r, b"a", 10).unwrap();
        o.enqueue(r, b"b", 10).unwrap();

        assert_eq!(set_eddn(&core, false)["ok"], true);
        assert!(!EDDN_SHARING.load(Ordering::SeqCst));
        assert_eq!(o.pending().unwrap(), 0, "discarded straight away, not on the worker's next pass");
        assert_eq!(o.recent(1).unwrap()[0].detail, "switched off: 2 queued discarded");

        // Back on before the worker wakes: nothing from before is left to send.
        set_eddn(&core, true);
        assert!(EDDN_SHARING.load(Ordering::SeqCst));
        assert_eq!(o.pending().unwrap(), 0);
        set_eddn(&core, false);
        assert_eq!(o.recent(10).unwrap().len(), 1, "an empty queue writes no row");
    }

    #[test]
    fn fixing_ignored_values_updates_then_clears_the_note_without_a_restart() {
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        // As `run` finds it: one bad value among good ones.
        let bad = "[controls]\nptt = { device = \"231D3201\", key = \"Joy_4\" }\nrearm_hold_ms = \"slow\"\n";
        std::fs::write(&core.config_path, bad).unwrap();
        let (c, problems) = Config::load_report(&core.config_path).unwrap();
        *core.cfg.write().recover() = c;
        let note = config_problems_note(&problems);
        *core.config_watch.lock().recover() = ConfigWatch::loaded(&core.config_path, note, false);
        let n = core.status_json()["config_note"].as_str().unwrap().to_string();
        assert!(n.contains("These config values were ignored") && n.contains("line 3: controls.rearm_hold_ms"), "{n}");
        // The good values are in force, and setup is not blocked by a note about one value.
        assert!(!core.setup_needed().iter().any(|m| m.contains("config file")));
        // A different mistake: the note follows the file.
        std::fs::write(&core.config_path, format!("{bad}debounce_ms = true\n")).unwrap();
        let n = core.status_json()["config_note"].as_str().unwrap().to_string();
        assert!(n.contains("controls.rearm_hold_ms") && n.contains("controls.debounce_ms"), "{n}");
        // Fixed: the note clears and the file's values are in force.
        std::fs::write(
            &core.config_path,
            "[controls]\nrearm_hold_ms = 800\nptt = { device = \"231D3201\", key = \"Joy_4\" }\n",
        )
        .unwrap();
        let st = core.status_json();
        assert!(st["config_note"].is_null(), "{st}");
        assert_eq!(core.cfg().controls.rearm_hold_ms, 800);
        // Broken beyond TOML, then fixed again.
        *core.config_watch.lock().recover() = ConfigWatch::loaded(&core.config_path, Some("x".into()), false);
        std::fs::write(&core.config_path, "[controls\nptt = ").unwrap();
        assert_eq!(core.setup_needed(), ["config file has an error (see logs)"]);
        assert!(core.config_note().unwrap().contains("can't be read"));
        std::fs::write(&core.config_path, "[voice]\nthreads = 2\n").unwrap();
        assert!(!core.setup_needed().iter().any(|m| m.contains("config file")));
        assert!(core.config_note().is_none() && !core.config_failed());
        assert_eq!(core.cfg().voice.threads, 2);
    }

    #[test]
    fn fixing_a_broken_config_clears_the_error_without_a_restart() {
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        std::fs::write(&core.config_path, "[controls\nptt = ").unwrap();
        // As `run` finds it: unreadable, so defaults and a note.
        *core.config_watch.lock().recover() =
            ConfigWatch::loaded(&core.config_path, Some("config error (see logs)".into()), true);
        assert_eq!(core.status_json()["config_note"], "config error (see logs)");
        assert_eq!(core.setup_needed(), ["config file has an error (see logs)"]);
        // Still broken, edited: still an error.
        std::fs::write(&core.config_path, "[controls\nptt = {").unwrap();
        assert_eq!(core.setup_needed(), ["config file has an error (see logs)"]);
        // Fixed in the editor: the note clears and the file's settings are in force.
        std::fs::write(
            &core.config_path,
            "[controls]\nptt = { device = \"231D3201\", key = \"Joy_4\" }\n[voice]\nmic = \"Example USB\"\n",
        )
        .unwrap();
        let st = core.status_json();
        assert!(st["config_note"].is_null(), "{st}");
        assert_eq!(st["ptt"], "231D3201 Joy_4");
        assert!(!core.setup_needed().iter().any(|m| m.contains("config file")), "{:?}", core.setup_needed());
        assert_eq!(core.cfg().voice.mic, "Example USB");
    }

    /// What a reply looks like, ignoring values: each object's keys, recursively. Arrays count
    /// as arrays (their items vary), and a value that may be null or not is just a value.
    fn shape(v: &Value) -> Value {
        match v {
            Value::Object(m) => Value::Object(m.iter().map(|(k, v)| (k.clone(), shape(v))).collect()),
            Value::Array(_) => json!("array"),
            _ => json!("value"),
        }
    }

    /// The window's fake core (`app/src/preview/fakecore.ts`) answers the setup and calibration
    /// commands with hand-written JSON. This pins the real replies' shape in
    /// `app/src/lib/core-reply-shapes.json`; the app's `fakecore.contract.test.ts` holds the
    /// fake to the same file, so neither can change alone. Refresh the file after a deliberate
    /// change with `ASTROMECH_UPDATE_SHAPES=1 cargo test -p astromech-core reply_shapes`.
    /// It compares keys of the success replies in one scenario; not value types, array items,
    /// error replies or the words a field may hold (a `verdict`, say).
    #[test]
    fn setup_and_calibration_reply_shapes_match_the_window_fake() {
        use crate::calibration::{handle, Clip};
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let journals = d.path().join("journals");
        std::fs::create_dir_all(&journals).unwrap();
        let core = core(
            d.path(),
            &format!("data_dir = '{}'\n[game]\njournal_dir = '{}'\n", d.path().display(), journals.display()),
        );
        let ask = |cmd: &str| {
            core.handle(&Request {
                token: String::new(),
                cmd: cmd.into(),
                route: None,
                text: None,
                args: None,
                origin: None,
            })
        };

        let tone = |ms: usize, amp: f32| -> Vec<f32> { (0..16 * ms).map(|i| amp * (i as f32 * 0.2).sin()).collect() };
        let clips = [vec![0.0004f32; 16 * 2000], tone(900, 0.02), tone(700, 0.03), tone(1100, 0.015)];
        let next = std::cell::Cell::new(0usize);
        let record = |_ms: u64| {
            let i = next.get().min(clips.len() - 1);
            next.set(next.get() + 1);
            Ok(Clip { device: "Microphone (Example USB Mic)".into(), rate: 16000, samples: clips[i].clone() })
        };
        let run = Mutex::new(None);
        let store = || crate::calibration::open_store(d.path());
        let mut got = serde_json::Map::new();
        got.insert("find_elite".into(), shape(&ask("find_elite")));
        got.insert("mic_list".into(), shape(&ask("mic_list")));
        got.insert("calibrate_start".into(), shape(&handle("calibrate_start", &run, &record, &store)));
        got.insert("calibrate_record".into(), shape(&handle("calibrate_record", &run, &record, &store)));
        for _ in 0..3 {
            handle("calibrate_record", &run, &record, &store);
        }
        got.insert("calibrate_finish".into(), shape(&handle("calibrate_finish", &run, &record, &store)));
        got.insert("calibrate_cancel".into(), shape(&handle("calibrate_cancel", &run, &record, &store)));
        got.insert("mic_check".into(), shape(&handle("mic_check", &run, &record, &store)));
        for (cmd, v) in &got {
            assert_eq!(v["ok"], "value", "{cmd} must answer ok");
        }
        let got = Value::Object(got);

        let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/src/lib/core-reply-shapes.json");
        if std::env::var_os("ASTROMECH_UPDATE_SHAPES").is_some() {
            std::fs::write(&file, serde_json::to_string_pretty(&got).unwrap() + "\n").unwrap();
        }
        let want: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
        assert_eq!(
            got, want,
            "the core's reply shapes changed: update core-reply-shapes.json (see this test) and fakecore.ts"
        );
    }

    /// Done-when for astromech-6siv: folders set in onboarding or Settings (`set_folders`, over
    /// the control API) are what the window's keybinds and journal commands read next, with no
    /// restart.
    #[test]
    fn the_window_reads_the_folders_set_folders_saved() {
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let folder = |name: &str, preset: &str, journal: &str| {
            let (b, j) = (d.path().join(name).join("Bindings"), d.path().join(name).join("journals"));
            std::fs::create_dir_all(&b).unwrap();
            std::fs::create_dir_all(&j).unwrap();
            std::fs::write(
                b.join(format!("{preset}.4.2.binds")),
                format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<Root PresetName=\"{preset}\" MajorVersion=\"4\" MinorVersion=\"2\">\n</Root>\n"),
            )
            .unwrap();
            std::fs::write(j.join("Journal.2026-11-01T100000.01.log"), journal).unwrap();
            (b, j)
        };
        let header = r#"{ "timestamp":"2026-11-01T10:00:00Z", "event":"Fileheader", "part":1, "gameversion":"4.1.0", "build":"r1" }"#;
        let (old_b, old_j) = folder("old", "Before", &format!("{header}\n"));
        let iron = r#"{ "timestamp":"2026-11-01T10:01:00Z", "event":"MaterialCollected", "Category":"Raw", "Name":"iron", "Count":3 }"#;
        let (new_b, new_j) = folder("new", "After", &format!("{header}\n{iron}\n"));
        let core = core(
            d.path(),
            &format!(
                "data_dir = '{}'\n[game]\njournal_dir = '{}'\nbindings_dir = '{}'\n",
                d.path().join("data").display(),
                old_j.display(),
                old_b.display()
            ),
        );
        let app = d.path().join("app");
        let _srv = control::serve(&app, Arc::new(core.commands())).unwrap();
        let ask = |cmd: &str, args: Option<Value>| {
            let req = Request { args, origin: Some(astromech_proto::Origin::AppWindow), ..Request::new(cmd) };
            control::call(&app, req, None, &mut |_| {}).unwrap()
        };
        let preset = |v: Value| v["presets"][0]["name"].clone();
        let materials = |dir: &Path| serde_json::to_value(astromech_materials::from_journal_dir(dir)).unwrap();
        assert_eq!(preset(ask("binds_state", None)), "Before");
        assert_eq!(ask("materials_inventory", None), materials(&old_j));

        let set = json!({"bindings_dir": new_b.display().to_string(), "journal_dir": new_j.display().to_string()});
        assert_eq!(ask("set_folders", Some(set))["ok"], true);
        assert_eq!(preset(ask("binds_state", None)), "After");
        let now = ask("materials_inventory", None);
        assert_eq!(now, materials(&new_j));
        assert_ne!(now, materials(&old_j));
    }

    /// A handler that panics while it holds the session lock answers with an error, and the
    /// commands that take that lock after it (`status`, `exobiology`) answer as normal.
    #[test]
    fn a_panic_under_the_session_lock_leaves_the_core_working() {
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        let mut reg = core.commands();
        let c = core.clone();
        reg.add("panic_holding_the_session", Exposure::Window, move |_, _| {
            let _held = c.session.lock();
            panic!("a handler's bug")
        });
        let app = d.path().join("app");
        let _srv = control::serve(&app, Arc::new(reg)).unwrap();
        let ask = |cmd: &str| {
            let req = Request { origin: Some(astromech_proto::Origin::AppWindow), ..Request::new(cmd) };
            control::call(&app, req, None, &mut |_| {})
        };
        let e = ask("panic_holding_the_session").unwrap_err();
        assert!(e.contains("a handler's bug"), "{e}");
        assert!(core.session.is_poisoned());
        let status = ask("status").unwrap();
        assert_eq!((&status["ok"], &status["active"]), (&json!(true), &json!(false)), "{status}");
        assert_eq!(ask("exobiology").unwrap(), json!({"ok": false, "error": "Elite isn't running."}));
    }

    /// The window's commands read and write the core's data folder, the one its config names,
    /// fixed at startup: a filter saved there and the history there are what come back.
    #[test]
    fn the_window_reads_the_data_folder_the_config_names() {
        let d = tempfile::tempdir().unwrap();
        let data = d.path().join("elsewhere");
        std::fs::create_dir_all(&data).unwrap();
        let saved = astromech_galaxy::Filters { max_distance_ly: Some(42), ..Default::default() };
        let store = astromech_store::Store::open(&data.join("astromech.sqlite")).unwrap();
        astromech_galaxy::filters::save(&store, "trade.sell", &saved).unwrap();
        drop(store);
        let journals = Path::new(env!("CARGO_MANIFEST_DIR")).join("../astromech-history/tests/fixtures");
        let mut h = astromech_history::History::open(&data.join("history.sqlite")).unwrap();
        h.import(&journals, &Default::default()).unwrap();
        drop(h);

        let core = core(d.path(), &format!("data_dir = '{}'\n", data.display()));
        assert_ne!(core.cfg().data_dir(), Config::default().data_dir());
        let app = d.path().join("app");
        let _srv = control::serve(&app, Arc::new(core.commands())).unwrap();
        let ask = |cmd: &str, args: Value| {
            let req =
                Request { args: Some(args), origin: Some(astromech_proto::Origin::AppWindow), ..Request::new(cmd) };
            control::call(&app, req, None, &mut |_| {}).unwrap()
        };
        let filters = ask("search_filters", json!({"lookup": "trade.sell"}));
        assert_eq!(filters["max_distance_ly"], 42, "{filters}");
        let logbook = ask("logbook", json!({"query": "Alpha", "cmdr": null}));
        let want = astromech_history::History::open_read_only(&data.join("history.sqlite"))
            .unwrap()
            .logbook(None, "Alpha", 200)
            .unwrap();
        assert_eq!(logbook, serde_json::to_value(want).unwrap());
        assert!(logbook.to_string().contains("Alpha Centauri"), "{logbook}");
    }

    /// `route_plot` plots from recorded Spansh answers for the journal's ship and hands the route
    /// to the follower through the registry: the Route page's `route` shows it.
    #[test]
    fn a_plotted_route_is_the_one_the_follower_shows() {
        macro_rules! fixture {
            ($name:literal) => {
                include_str!(concat!("../../astromech-data/tests/fixtures/", $name))
            };
        }
        let d = tempfile::tempdir().unwrap();
        let journals = d.path().join("journals");
        std::fs::create_dir_all(&journals).unwrap();
        std::fs::write(
            journals.join("Journal.2026-11-01T100000.01.log"),
            r#"{ "timestamp":"2026-11-01T10:00:00Z", "event":"Fileheader", "part":1, "gameversion":"4.2.0", "build":"r1" }
{ "timestamp":"2026-11-01T10:00:01Z", "event":"LoadGame", "Commander":"Test", "FID":"F1", "Ship":"asp", "ShipID":1, "Credits":1000 }
{ "timestamp":"2026-11-01T10:00:02Z", "event":"Loadout", "Ship":"asp", "ShipID":1, "ShipName":"", "ShipIdent":"", "FuelCapacity":{"Main":32.0,"Reserve":0.63}, "UnladenMass":400.6, "MaxJumpRange":30.0, "Modules":[{"Slot":"PowerPlant","Item":"int_powerplant_size4_class5","On":true,"Priority":0},{"Slot":"FrameShiftDrive","Item":"int_hyperdrive_size5_class5","On":true,"Priority":0}] }
{ "timestamp":"2026-11-01T10:00:03Z", "event":"Location", "StarSystem":"Sol", "SystemAddress":10477373803, "StarPos":[0.0,0.0,0.0], "Docked":false }
"#,
        )
        .unwrap();
        let answers = vec![
            (202, fixture!("spansh-exact-submit.json")),
            (202, fixture!("spansh-exact-pending.json")),
            (200, fixture!("spansh-exact-result.json")),
        ];
        let remote = astromech_data::Client::with_transport(
            astromech_data::Cache::in_memory().unwrap(),
            Arc::new(RecordedJob(Mutex::new(answers))),
        )
        .limiter(astromech_data::Limiter::none());
        let config = format!(
            "data_dir = '{}'\n[game]\njournal_dir = '{}'\n",
            d.path().join("data").display(),
            journals.display()
        );
        let core = core_with(d.path(), &config, Arc::new(remote));
        let app = d.path().join("app");
        let _srv = control::serve(&app, Arc::new(core.commands())).unwrap();
        let ask = |cmd: &str, args: Option<Value>| {
            let req = Request { args, origin: Some(astromech_proto::Origin::AppWindow), ..Request::new(cmd) };
            control::call(&app, req, None, &mut |_| {})
        };
        let ship = ask("route_plot_ship", None).unwrap();
        assert_eq!(ship["here"], "Sol", "{ship}");
        assert_eq!(ship["ship_error"], Value::Null, "{ship}");

        let ask_maia =
            json!({"ask": {"plotter": "exact", "to": "Maia", "efficiency": 60, "refuel_every_scoopable": true}});
        let plotted = ask("route_plot", Some(ask_maia)).unwrap();
        assert_eq!(plotted["plotted"]["source"], "exact plotter", "{plotted}");
        let shown = ask("route", None).unwrap();
        assert_eq!(shown["view"], plotted["view"]);
        assert_eq!(shown["view"]["route"]["source"], "exact plotter", "{shown}");
        assert_eq!(shown["view"]["route"]["name"], plotted["plotted"]["name"]);
        assert_eq!(shown["view"]["route"]["name"], "Sol to Maia");
        assert_eq!(shown["view"]["remaining"], 10, "{shown}");
    }

    fn call(core: &Arc<Core>, cmd: &str, args: Option<Value>) -> Value {
        core.handle(&Request { token: String::new(), cmd: cmd.into(), route: None, text: None, args, origin: None })
    }

    fn callout(v: &Value, id: &str) -> Value {
        v["callouts"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap().clone()
    }

    #[test]
    fn callout_settings_round_trip_through_the_config_file() {
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "# my notes\n[callouts.hge]\ncooldown_s = 12 # long honks\n");
        let list = call(&core, "callouts", None);
        assert_eq!(list["ok"], true);
        assert_eq!(callout(&list, "scan")["enabled"], true);
        assert_eq!(callout(&list, "scan")["cooldown_s"], 30.0);
        assert_eq!(callout(&list, "hge")["cooldown_s"], 12.0, "a hand-written cooldown shows");
        assert_eq!(callout(&list, "powerplay_arrival")["enabled"], false, "off until switched on");

        // Off, a cooldown, and the default-off one on: written to [callouts.<id>], and the
        // engine a session builds from the config honours them.
        assert_eq!(call(&core, "set_callout", Some(json!({"id": "scan", "enabled": false})))["ok"], true);
        assert_eq!(call(&core, "set_callout", Some(json!({"id": "scan", "cooldown_s": 90})))["ok"], true);
        assert_eq!(call(&core, "set_callout", Some(json!({"id": "powerplay_arrival", "enabled": true})))["ok"], true);
        let text = std::fs::read_to_string(&core.config_path).unwrap();
        assert!(text.contains("# my notes") && text.contains("# long honks"), "{text}");
        let on_disk = Config::load(&core.config_path).unwrap();
        let s = &on_disk.callouts.0;
        assert_eq!((s["scan"].enabled, s["scan"].cooldown_s), (Some(false), Some(90.0)));
        assert_eq!(s["powerplay_arrival"].enabled, Some(true));
        assert_eq!(s["hge"].cooldown_s, Some(12.0));
        let list = call(&core, "callouts", None);
        assert_eq!(
            (callout(&list, "scan")["enabled"].clone(), callout(&list, "scan")["cooldown_s"].clone()),
            (json!(false), json!(90.0))
        );
        assert_eq!(callout(&list, "powerplay_arrival")["enabled"], true);
        let engine =
            astromech_announce::Engine::new(astromech_announce::builtin(Default::default()), &on_disk.callouts);
        assert!(engine.setting_problems().is_empty(), "{:?}", engine.setting_problems());

        // Back to the callout's own cooldown removes the key; a bad id or value changes nothing.
        call(&core, "set_callout", Some(json!({"id": "scan", "cooldown_s": null})));
        assert_eq!(Config::load(&core.config_path).unwrap().callouts.0["scan"].cooldown_s, None);
        let before = std::fs::read_to_string(&core.config_path).unwrap();
        for bad in [
            json!({"id": "nope", "enabled": false}),
            json!({"id": "scan", "cooldown_s": -1}),
            json!({"id": "scan", "cooldown_s": "soon"}),
            json!({"id": "scan"}),
        ] {
            assert_eq!(call(&core, "set_callout", Some(bad.clone()))["ok"], false, "{bad}");
        }
        assert_eq!(std::fs::read_to_string(&core.config_path).unwrap(), before);
    }

    /// Typed replies keep the JSON the window reads (astromech-se44): a scripted run of the core's
    /// own commands equals `tests/fixtures/core-replies.json`, recorded from the `json!` replies
    /// before they were typed. Every folder the config names is under the test's temporary one,
    /// which is masked (as `<dir>`, with `/` separators) on any host. What this machine itself
    /// answers is masked too: the pid, the version, the log folder, start with Windows and an
    /// installed game. The microphone commands are left out on Windows, where they list and
    /// record real devices. Refresh it after a deliberate change with
    /// `ASTROMECH_UPDATE_REPLIES=1 cargo test -p astromech-core --lib replies_keep`.
    #[test]
    fn core_replies_keep_their_json() {
        let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
        let d = tempfile::tempdir().unwrap();
        let journals = d.path().join("journals");
        std::fs::create_dir_all(&journals).unwrap();
        let core = core(
            d.path(),
            &format!(
                "data_dir = '{0}'\n[game]\njournal_dir = '{1}'\nbindings_dir = '{0}/bindings'\ngame_dir = '{0}/game'\n\
                 [voice]\nmodels_dir = '{0}/models'\n",
                d.path().display(),
                journals.display()
            ),
        );
        let ask = |cmd: &str, args: Option<Value>, text: Option<&str>| {
            core.handle(&Request { args, text: text.map(str::to_string), ..Request::new(cmd) })
        };
        let mut script: Vec<(&str, Option<Value>, Option<&str>)> = vec![
            ("status", None, None),
            ("callouts", None, None),
            ("set_callout", Some(json!({"id": "scan", "enabled": false})), None),
            ("set_callout", Some(json!({"id": "nope", "enabled": false})), None),
            ("set_mic", Some(json!({"mic": "USB"})), None),
            ("set_ptt", Some(json!({"device": "", "key": ""})), None),
            ("prospector_thresholds", None, None),
            ("prospector_set", None, Some(r#"{"ore":"platinum","threshold":35}"#)),
            ("prospector_set", None, Some("not json")),
            ("shopping_set", None, Some("[]")),
            ("voice_settings", None, None),
            ("voice_preview", None, None),
            ("learned_phrases", None, None),
            ("learned_forget", None, Some("nothing")),
            ("find_elite", None, None),
            ("mic_list", None, None),
            ("calibrate_start", None, None),
            ("calibrate_finish", None, None),
            ("calibrate_cancel", None, None),
            ("mic_check", None, None),
            ("eddn_log", None, None),
            ("set_eddn", Some(json!({"enabled": "yes"})), None),
            ("edsm_status", None, None),
            ("edsm_flight_log", None, None),
            ("set_edsm", Some(json!({})), None),
            ("exobiology", None, None),
            ("voice_test", None, Some("deploy hardpoints")),
            ("voice_test_live", None, Some("hardpoints.deploy")),
            ("install_update", None, None),
            ("kill_switch", None, None),
            ("ack_crash", None, None),
            ("onboarding_done", None, None),
            ("route", None, None),
            (
                "route_set",
                Some(
                    json!({"name": "Trip", "source": "search", "waypoints": [{"system": "Sol"}, {"system": "Colonia", "label": "x", "jumps": 3}]}),
                ),
                None,
            ),
            ("route_add", Some(json!({"system": "Deciat", "label": "Farseer", "source": "engineer trip"})), None),
            ("route_skip", None, None),
            ("route_back", None, None),
            ("route_set", Some(json!({"text": " "})), None),
            ("route_clear", None, None),
            ("route_resume", Some(json!({})), None),
        ];
        // The microphone commands answer from this machine's devices on Windows.
        let host_devices = |cmd: &str| cfg!(windows) && (cmd == "mic_list" || cmd == "mic_check");
        script.retain(|(cmd, _, _)| !host_devices(cmd));
        // Any path, with `/` separators and the temporary folder as `<dir>`.
        fn mask(v: &mut Value, dir: &str) {
            match v {
                Value::String(s) => *s = s.replace('\\', "/").replace(dir, "<dir>"),
                Value::Array(a) => a.iter_mut().for_each(|v| mask(v, dir)),
                Value::Object(m) => m.values_mut().for_each(|v| mask(v, dir)),
                _ => {}
            }
        }
        let dir = d.path().display().to_string().replace('\\', "/");
        let mut got = vec![];
        for (cmd, args, text) in script {
            let mut v = ask(cmd, args.clone(), text);
            for k in ["pid", "version", "log_dir", "autostart"] {
                if v.get(k).is_some() {
                    v[k] = json!("<masked>");
                }
            }
            if let Some(found) = v.get_mut("found") {
                found["install"] = json!("<masked>");
                found["install_source"] = json!("<masked>");
            }
            mask(&mut v, &dir);
            got.push(json!({"cmd": cmd, "args": args, "text": text, "reply": v}));
        }
        let got = Value::Array(got);
        let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/core-replies.json");
        if std::env::var_os("ASTROMECH_UPDATE_REPLIES").is_some() {
            std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(&file, serde_json::to_string_pretty(&got).unwrap() + "\n").unwrap();
        }
        let mut want: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
        want.as_array_mut().unwrap().retain(|e| !host_devices(e["cmd"].as_str().unwrap_or_default()));
        for (g, w) in got.as_array().unwrap().iter().zip(want.as_array().unwrap()) {
            assert_eq!(g, w, "a reply's JSON changed");
        }
        assert_eq!(got, want);
    }

    /// Where a command's args or reply may hold a `serde_json::Value` (ts-rs's `JsonValue`,
    /// which tells the window nothing), each with why. `bindings_use_json_value_only_where_allowed`
    /// fails for any other, and for an entry no longer needed.
    const JSON_VALUES: &[(&str, &str, &str)] = &[(
        "route_plot",
        "reply",
        "its `view` is route_set's, which astromech-api reads through Call::run and can't name: the \
         route follower's types are the core's",
    )];

    /// The registry's TypeScript types (astromech-se44) are the checked-in `app/src/lib/api/`
    /// files: `types/` (ts-rs) and `commands.ts`. After changing a command's args or reply,
    /// update them with `ASTROMECH_UPDATE_BINDINGS=1 cargo test -p astromech-core --lib bindings`.
    #[test]
    fn bindings_are_the_checked_in_typescript() {
        let d = tempfile::tempdir().unwrap();
        let reg = core(d.path(), "").commands();
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/src/lib/api");
        if let Err(e) = astromech_api::bindings::check_or_write(&reg, &dir) {
            panic!("{e}");
        }
    }

    #[test]
    fn bindings_use_json_value_only_where_allowed() {
        let d = tempfile::tempdir().unwrap();
        let found = astromech_api::bindings::json_values(&core(d.path(), "").commands());
        let allowed: Vec<(String, &str)> = JSON_VALUES.iter().map(|(c, p, _)| (c.to_string(), *p)).collect();
        let not_allowed: Vec<_> = found.iter().filter(|f| !allowed.contains(f)).collect();
        assert!(
            not_allowed.is_empty(),
            "a serde_json::Value (JsonValue) in {not_allowed:?}: give it a type, or add it to JSON_VALUES with why"
        );
        let stale: Vec<_> = allowed.iter().filter(|a| !found.contains(a)).collect();
        assert!(stale.is_empty(), "JSON_VALUES lists {stale:?}, which no longer holds a JsonValue: remove it");
    }
}
