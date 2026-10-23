//! The always-running core (M0, docs/V1-DESIGN.md §7): tray icon, single instance, start with
//! Windows, game detection with idle/active, the app window launched from the tray, crash
//! marker and diagnostics. [`run`] is what `astromech-core.exe` does when started without a
//! console option (and with `--background` from the Run key).

use crate::config::Config;
use crate::control::{self, Request};
use crate::gamewatch::{self, Change, Game};
use crate::logging::log;
use crate::session::{self, Session, VOICE_PAUSED};
use crate::tester::{self, Source, TESTER};
use crate::update::{self, ApplyWhen, Status as UpStatus};
use anyhow::Result;
use astromech_store::Paths;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

/// The config file this run uses (`--config`, else the default), told to the app window so it
/// reads the same one (`Paths::config_in_use`).
fn choose_config(opts: &Opts, paths: &Paths) -> PathBuf {
    let path = opts.config.clone().unwrap_or_else(|| paths.config_file());
    if let Err(e) = paths.publish_config(&path) {
        log(&format!("couldn't note the config path for the window: {e}"));
    }
    path
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
}

impl Core {
    pub fn cfg(&self) -> Config {
        self.cfg.read().unwrap().clone()
    }

    pub fn notify(&self) {
        let _ = self.ui.lock().unwrap().send(Ui::Refresh);
    }

    pub fn active(&self) -> bool {
        self.session.lock().unwrap().is_some()
    }

    /// Keep the journal history on `cfg`'s journal folder: at startup, and after every config
    /// edit (re-points the follower when the folder changed; otherwise nothing changes).
    fn follow_history(&self, cfg: &Config) {
        self.history.follow(cfg.journal_dir(), &cfg.data_dir(), Arc::new(|m: &str| log(m)));
    }

    /// Why the config file wasn't used, if it wasn't.
    pub fn config_note(&self) -> Option<String> {
        self.config_watch.lock().unwrap().note.clone()
    }

    /// Make a config the one in force: the window's setup edits and a fixed file both come here.
    fn install_config(&self, mut c: Config) {
        c.actions.dry_run |= self.opts.session.dry_run;
        apply_eddn_switch(&c);
        apply_edsm_switch(&c, self.secrets.as_ref());
        // Under the config lock, so concurrent edits re-point in the order they saved.
        self.follow_history(&c);
        *self.cfg.write().unwrap() = c;
    }

    /// Whether the whole config file was unreadable (so every setting is a default).
    pub fn config_failed(&self) -> bool {
        self.config_watch.lock().unwrap().failed
    }

    /// While the config file had something wrong (values ignored, or unreadable), look at it
    /// again when it changes: use what loads and update or clear the note, so a fix needs no
    /// restart. A file that loaded cleanly is never re-read here (edits to a working file apply
    /// at the next start, as before).
    pub fn refresh_config(&self) {
        // The usual case (a good file) costs one look and never waits on an edit in progress.
        if self.config_watch.lock().unwrap().note.is_none() {
            return;
        }
        let _g = self.config_lock.lock().unwrap_or_else(|e| e.into_inner());
        let mut w = self.config_watch.lock().unwrap();
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
        let game = *self.game.lock().unwrap();
        let mut s = match (game, self.active(), self.starting.load(Ordering::SeqCst)) {
            _ if testing => "Voice tester open · push-to-talk testing only".to_string(),
            (_, _, true) => "Starting voice…".to_string(),
            (Game::Running, true, _) => format!("Elite running · {}", self.voice.lock().unwrap()),
            (Game::Launcher, true, _) => format!("Launcher open · {}", self.voice.lock().unwrap()),
            (Game::Running, false, _) => "Elite running · voice failed (see logs)".to_string(),
            _ => "Waiting for Elite".to_string(),
        };
        if VOICE_PAUSED.load(Ordering::SeqCst) {
            s = format!("{s} · paused");
        }
        if let Some(m) = self.models.lock().unwrap().as_ref() {
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

    pub fn status_json(&self) -> Value {
        self.refresh_config();
        let (killed, toasts, mining) = match self.session.lock().unwrap().as_ref() {
            Some(s) => (s.runner.policy.is_killed(), s.toasts.recent(), Some(s.hub.mining_json())),
            None => (false, vec![], None),
        };
        let cfg = self.cfg();
        let button = |b: &Option<crate::config::Button>| b.as_ref().map(|b| format!("{} {}", b.device, b.key));
        json!({
            "ok": true,
            "version": env!("CARGO_PKG_VERSION"),
            "pid": std::process::id(),
            "game": *self.game.lock().unwrap(),
            "active": self.active(),
            "starting": self.starting.load(Ordering::SeqCst),
            "voice": *self.voice.lock().unwrap(),
            "voice_paused": session::VOICE_PAUSED.load(Ordering::SeqCst),
            "killed": killed,
            "autostart": crate::autostart::is_enabled(),
            "crash_pending": self.crash.lock().unwrap().is_some(),
            "status_line": self.status_line(),
            "app_dir": self.app_dir.display().to_string(),
            "log_dir": crate::logging::dir().unwrap_or_else(|| Paths::current().log_dir()).display().to_string(),
            "config": self.config_path.display().to_string(),
            "config_note": self.config_note(),
            "setup_needed": self.setup_needed(),
            "ptt": button(&cfg.controls.ptt),
            "kill_switch": button(&cfg.controls.kill_switch),
            "mic": cfg.voice.mic,
            "onboarded": self.onboarded.load(Ordering::SeqCst),
            "models_missing": models_missing(&cfg),
            "models_progress": *self.models.lock().unwrap(),
            "models_dir": cfg.models_dir().display().to_string(),
            "update": &self.update.lock().unwrap().snap,
            "toasts": toasts,
            "history": self.history.status(),
            "mining": mining,
        })
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
        if self.crash.lock().unwrap().take().is_some() {
            crate::crash::acknowledge(&self.app_dir);
        }
        self.notify();
        Ok(p)
    }

    pub fn kill_switch(&self) {
        if let Some(s) = self.session.lock().unwrap().as_ref() {
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
        let r = Session::start(&cfg, &self.opts.session, self.route.clone()).and_then(|mut s| {
            s.start_buttons(&cfg, &self.opts.session)?;
            Ok(s)
        });
        match r {
            Ok(s) => {
                log(&format!("active in {} ms: {}", t0.elapsed().as_millis(), s.voice));
                *self.voice.lock().unwrap() = s.voice.clone();
                *self.session.lock().unwrap() = Some(s);
                // A voice saved in Settings while the session loaded: use it now.
                let (old, new) = (&cfg.voice, self.cfg().voice);
                if (old.speaker.as_str(), old.speed, old.colour) != (new.speaker.as_str(), new.speed, new.colour) {
                    self.apply_voice();
                }
            }
            Err(e) => {
                log(&format!("couldn't go active: {e:#}"));
                *self.voice.lock().unwrap() = format!("error: {e}");
            }
        }
        self.starting.store(false, Ordering::SeqCst);
        // If the game went away while we were loading, unload again.
        if *self.game.lock().unwrap() == Game::NotRunning {
            self.deactivate("the game exited while loading");
        }
        self.notify();
    }

    fn deactivate(&self, why: &str) {
        if !self.starting.load(Ordering::SeqCst) {
            *self.voice.lock().unwrap() = "voice not loaded".into();
        }
        // Out of the lock before dropping: closing a session waits for its threads, and the
        // status, tray and kill switch all need this lock meanwhile.
        let session = self.session.lock().unwrap().take();
        if let Some(s) = session {
            log(&format!("{why}: going idle (unloading voice, closing the microphone)"));
            s.speaker.interrupt();
            s.runner.policy.kill();
            drop(s);
        }
        self.notify();
    }

    fn set_update(&self, f: impl FnOnce(&mut update::Handle)) {
        f(&mut self.update.lock().unwrap());
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
            let mut h = self.update.lock().unwrap();
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
    pub fn install_update(self: &Arc<Self>) -> Value {
        if self.cfg().update.apply == ApplyWhen::Never {
            return json!({"ok": false, "error": "updates are set to never apply ([update] apply = \"never\")"});
        }
        // Claim the install under the lock, so a second click can't start another helper.
        let (payload, before) = {
            let mut h = self.update.lock().unwrap();
            if h.snap.status == UpStatus::Applying {
                return json!({"ok": false, "error": "already installing"});
            }
            if !h.snap.ready {
                return json!({"ok": false, "error": h.snap.error.clone().unwrap_or_else(|| "no update ready".into())});
            }
            let Some(p) = h.payload.clone() else {
                return json!({"ok": false, "error": "update payload missing"});
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
                let ui = self.ui.lock().unwrap().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(300));
                    let _ = ui.send(Ui::Quit);
                });
                json!({"ok": true})
            }
            Err(e) => {
                let m = format!("{e:#}");
                log(&format!("update: couldn't start helper: {m}"));
                self.set_update(|h| h.snap.status = before);
                json!({"ok": false, "error": m})
            }
        }
    }

    fn apply_on_quit(&self) {
        if self.cfg().update.apply != ApplyWhen::OnQuit {
            return;
        }
        let h = self.update.lock().unwrap();
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
    fn edit_config(&self, r: &Request) -> Value {
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
                json!({"ok": true, "applies_next_session": !now && self.active()})
            }
            Err(e) => json!({"ok": false, "error": format!("{e:#}")}),
        }
    }

    /// Settings → Data sharing, EDSM: the API key (to the OS keyring; empty forgets it), then
    /// the switch and commander name (to the config), then the uploader's credentials.
    fn set_edsm(&self, r: &Request) -> Value {
        let edits = match crate::setup::Edit::from_request(&r.cmd, r.args.as_ref()) {
            Ok(e) => e,
            Err(e) => return json!({"ok": false, "error": format!("{e:#}")}),
        };
        if let Some(k) = r.args.as_ref().and_then(|a| a.get("api_key")).filter(|k| !k.is_null()) {
            let Some(k) = k.as_str().map(str::trim) else {
                return json!({"ok": false, "error": "api_key must be text"});
            };
            if let Err(e) = self.secrets.set(astromech_eddn::edsm::KEY_SECRET, Some(k).filter(|k| !k.is_empty())) {
                return json!({"ok": false, "error": format!("couldn't save the API key: {e}")});
            }
            log(if k.is_empty() { "EDSM: API key removed" } else { "EDSM: API key saved in the OS keyring" });
        }
        if edits.is_empty() {
            apply_edsm_switch(&self.cfg(), self.secrets.as_ref());
            self.notify();
            return json!({"ok": true, "applies_next_session": false});
        }
        self.edit_config(r)
    }

    /// Speak the next line in the configured voice: the running session's and the preview's.
    fn apply_voice(&self) {
        let cfg = self.cfg();
        match crate::speech::VoiceParams::from_config(&cfg.voice) {
            Ok(v) => {
                if let Some(s) = self.session.lock().unwrap().as_ref() {
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
    fn preview_voice(&self) -> Value {
        use crate::speech::PREVIEW_LINE;
        let cfg = self.cfg();
        let route = || -> Option<Result<()>> {
            if self.starting.load(Ordering::SeqCst) {
                return Some(Err(anyhow::anyhow!("voice is starting; try again in a moment")));
            }
            if let Some(s) = self.session.lock().unwrap().as_ref() {
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
            Ok(()) => json!({"ok": true, "line": PREVIEW_LINE}),
            Err(e) => json!({"ok": false, "error": format!("{e:#}")}),
        }
    }

    /// The Route page's commands. `route_set` takes `{name, source, waypoints}` or, for an
    /// import, `{name, source, text}` (a Spansh CSV or one system per line).
    fn route_cmd(&self, r: &Request) -> Value {
        use crate::route::{parse_list, Route, Waypoint};
        let d = &self.route;
        let v = match r.cmd.as_str() {
            "route_set" => {
                let a = r.args.clone().unwrap_or_default();
                let s = |k: &str| a.get(k).and_then(|x| x.as_str()).unwrap_or_default().to_string();
                let waypoints: Vec<Waypoint> = match a.get("text").and_then(|t| t.as_str()) {
                    Some(t) => parse_list(t),
                    None => match serde_json::from_value(a.get("waypoints").cloned().unwrap_or_default()) {
                        Ok(w) => w,
                        Err(e) => return json!({"ok": false, "error": format!("bad waypoints: {e}")}),
                    },
                };
                let name = if s("name").trim().is_empty() { "Route".to_string() } else { s("name") };
                match d.set(Route::new(&name, &s("source"), waypoints)) {
                    Ok(v) => v,
                    Err(e) => return json!({"ok": false, "error": e}),
                }
            }
            "route_skip" => d.step(true),
            "route_back" => d.step(false),
            "route_clear" => d.clear(),
            "route_copy" => d.copy_next(),
            "route_resume" => match d.resume() {
                Ok(v) => v,
                Err(e) => return json!({"ok": false, "error": e}),
            },
            _ => d.view(),
        };
        json!({"ok": true, "view": v})
    }

    pub fn handle(self: &Arc<Self>, r: &Request) -> Value {
        match r.cmd.as_str() {
            "route" | "route_set" | "route_skip" | "route_back" | "route_clear" | "route_copy" | "route_resume" => {
                self.route_cmd(r)
            }
            "status" => self.status_json(),
            "open_window" => match self.open_window(r.route.as_deref()) {
                Ok(()) => json!({"ok": true}),
                Err(e) => json!({"ok": false, "error": e}),
            },
            "save_diagnostics" => match self.save_diagnostics() {
                Ok(p) => {
                    reveal(&p);
                    json!({"ok": true, "path": p.display().to_string()})
                }
                Err(e) => json!({"ok": false, "error": format!("{e:#}")}),
            },
            "ack_crash" => {
                self.crash.lock().unwrap().take();
                crate::crash::acknowledge(&self.app_dir);
                self.notify();
                json!({"ok": true})
            }
            "quit" => {
                let _ = self.ui.lock().unwrap().send(Ui::Quit);
                json!({"ok": true})
            }
            "check_update" => {
                self.check_update_in_background();
                json!({"ok": true})
            }
            "install_update" => self.install_update(),
            "autostart_on" | "autostart_off" => {
                let on = r.cmd == "autostart_on";
                let res =
                    std::env::current_exe().map_err(|e| e.to_string()).and_then(|exe| crate::autostart::set(on, &exe));
                self.notify();
                match res {
                    Ok(()) => {
                        log(if on { "start with Windows: on" } else { "start with Windows: off" });
                        json!({"ok": true})
                    }
                    Err(e) => json!({"ok": false, "error": e}),
                }
            }
            "pause_voice" | "resume_voice" => {
                VOICE_PAUSED.store(r.cmd == "pause_voice", Ordering::SeqCst);
                log(if r.cmd == "pause_voice" { "voice paused" } else { "voice resumed" });
                self.notify();
                json!({"ok": true})
            }
            "kill_switch" => {
                self.kill_switch();
                json!({"ok": true})
            }
            "download_models" => {
                download_models(self);
                json!({"ok": true})
            }
            "open_config" => {
                open_config(&self.config_path);
                json!({"ok": true})
            }
            "prospector_thresholds" => crate::callouts::thresholds_view(&self.cfg().data_dir()),
            "prospector_set" => {
                let text = r.text.clone().unwrap_or_default();
                let res = std::fs::create_dir_all(self.cfg().data_dir())
                    .map_err(|e| e.to_string())
                    .and_then(|_| {
                        astromech_store::Store::open(&self.cfg().data_dir().join("astromech.sqlite"))
                            .map_err(|e| format!("opening the database: {e}"))
                    })
                    .and_then(|st| crate::callouts::change_thresholds(&st, &text));
                match res {
                    Ok(t) => {
                        if let Some(s) = self.session.lock().unwrap().as_ref() {
                            *s.shared.prospector.write().unwrap() = t.clone();
                        }
                        log(&format!("prospector thresholds changed: {text}"));
                        crate::callouts::thresholds_json(&t)
                    }
                    Err(e) => json!({"ok": false, "error": e}),
                }
            }
            "shopping_set" => {
                let text = r.text.clone().unwrap_or_default();
                let res = std::fs::create_dir_all(self.cfg().data_dir())
                    .map_err(|e| e.to_string())
                    .and_then(|_| {
                        astromech_store::Store::open(&self.cfg().data_dir().join("astromech.sqlite"))
                            .map_err(|e| format!("opening the database: {e}"))
                    })
                    .and_then(|st| crate::callouts::change_shopping(&st, &text));
                match res {
                    Ok(list) => {
                        let n = list.0.len();
                        if let Some(s) = self.session.lock().unwrap().as_ref() {
                            *s.shared.shopping.write().unwrap() = list;
                        }
                        json!({"ok": true, "materials": n})
                    }
                    Err(e) => json!({"ok": false, "error": e}),
                }
            }
            "exobiology" => match self.session.lock().unwrap().as_ref() {
                Some(s) => {
                    let known = s.shared.known_bio.get();
                    crate::biology::view(&s.hub.snapshot(), known.as_deref())
                }
                None => json!({"ok": false, "error": "Elite isn't running."}),
            },
            "eddn_log" => {
                let cfg = self.cfg();
                eddn_log(&cfg.data_dir(), cfg.eddn.enabled, crate::pipeline::eddn_test(&cfg))
            }
            "find_elite" => json!({"ok": true, "found": crate::setup::find_elite(&self.cfg())}),
            "set_ptt" | "set_kill_switch" | "set_mic" | "set_folders" | "set_voice" | "set_eddn" | "set_callout" => {
                self.edit_config(r)
            }
            "callouts" => crate::callouts::callouts_json(&self.cfg().callouts),
            "edsm_status" => {
                let cfg = self.cfg();
                edsm_status(&cfg, self.secrets.as_ref())
            }
            "set_edsm" => self.set_edsm(r),
            "edsm_flight_log" => {
                let creds = edsm_credentials(&self.cfg().edsm, self.secrets.as_ref());
                match creds {
                    Ok(Some(c)) => {
                        let api = astromech_eddn::edsm::api::HttpApi::with_timeout(
                            &format!("astromech/{}", env!("CARGO_PKG_VERSION")),
                            astromech_eddn::edsm::api::FLIGHT_LOG_TIMEOUT,
                        );
                        match astromech_eddn::edsm::api::flight_log(&api, &c) {
                            Ok(visits) => json!({"ok": true, "visits": visits}),
                            Err(e) => json!({"ok": false, "error": e}),
                        }
                    }
                    Ok(None) => json!({"ok": false, "error": "set your EDSM commander name and API key first"}),
                    Err(e) => json!({"ok": false, "error": e}),
                }
            }
            "voice_settings" => {
                let v = self.cfg().voice;
                json!({
                    "ok": true,
                    "speaker": v.speaker,
                    "speed": v.speed,
                    "colour": v.colour,
                    "voices": crate::speech::voice_choices(),
                    "speeds": [crate::setup::SPEEDS.start(), crate::setup::SPEEDS.end()],
                })
            }
            "voice_preview" => self.preview_voice(),
            "mic_list" => {
                let cfg = self.cfg();
                let mut v = crate::calibration::mic_list(&cfg.voice.mic);
                let device = crate::calibration::configured_device(&cfg.voice.mic);
                let gate = crate::calibration::open_store(&cfg.data_dir())
                    .ok()
                    .and_then(|s| crate::calibration::stored_gate(&s, &device));
                v["calibration"] = json!({"device": device, "min_level_dbfs": gate});
                v
            }
            "calibrate_start" | "calibrate_record" | "calibrate_finish" | "calibrate_cancel" | "mic_check" => {
                let cfg = self.cfg();
                let record = |ms| crate::calibration::record_mic(&cfg.voice.mic, ms);
                let store = || crate::calibration::open_store(&cfg.data_dir());
                let v = crate::calibration::handle(&r.cmd, &self.calibration, &record, &store);
                if r.cmd == "calibrate_finish" && v["ok"] == true {
                    log(&format!("microphone: calibrated {} (min_level_dbfs {})", v["device"], v["min_level_dbfs"]));
                }
                v
            }
            "onboarding_done" => {
                let res = crate::calibration::open_store(&self.cfg().data_dir())
                    .and_then(|s| crate::setup::mark_onboarded(&s));
                match res {
                    Ok(()) => {
                        self.onboarded.store(true, Ordering::SeqCst);
                        self.notify();
                        json!({"ok": true})
                    }
                    Err(e) => json!({"ok": false, "error": format!("{e:#}")}),
                }
            }
            "voice_test" => {
                let text = r.text.clone().unwrap_or_default();
                let res = {
                    let s = self.session.lock().unwrap();
                    s.as_ref().map(|s| s.assistant.dry_run(&text, Source::Typed))
                };
                let res = res.unwrap_or_else(|| tester::dry_run_idle(&self.cfg(), &text));
                json!({"ok": true, "result": res})
            }
            "voice_test_poll" => {
                if TESTER.arm(Instant::now()) {
                    log("voice tester open: push-to-talk is tested there, nothing is pressed");
                }
                let last = TESTER.last().map(|(seq, r)| json!({"seq": seq, "result": r}));
                json!({"ok": true, "listening": self.active(), "last": last})
            }
            "learned_phrases" => crate::learned::list(&self.cfg().data_dir()),
            "learned_forget" => {
                let assistant = self.session.lock().unwrap().as_ref().map(|s| s.assistant.clone());
                let heard = r.text.clone().unwrap_or_default();
                log(&format!("learned phrase forgotten: {heard:?}"));
                crate::learned::forget(&self.cfg().data_dir(), assistant.as_deref(), &heard)
            }
            "voice_test_stop" => {
                if TESTER.armed(Instant::now()) {
                    log("voice tester closed: push-to-talk acts again");
                }
                TESTER.disarm();
                json!({"ok": true})
            }
            "voice_test_live" => {
                let cmd = r.text.clone().unwrap_or_default();
                let runner = self.session.lock().unwrap().as_ref().map(|s| s.runner.clone());
                match runner {
                    None => json!({"ok": false, "error": "Elite isn't running, so there's nothing to send keys to."}),
                    Some(runner) => {
                        log(&format!("voice tester: live run of {cmd}"));
                        match tester::live(&runner, &cmd, tester::SWITCH_WAIT) {
                            Ok(m) => json!({"ok": true, "message": m}),
                            Err(e) => json!({"ok": false, "error": e}),
                        }
                    }
                }
            }
            c => json!({"ok": false, "error": format!("unknown command {c:?}")}),
        }
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
pub fn edsm_status(c: &Config, secrets: &dyn astromech_store::secrets::Secrets) -> Value {
    let (has_key, key_error) = match secrets.get(astromech_eddn::edsm::KEY_SECRET) {
        Ok(k) => (k.is_some_and(|k| !k.is_empty()), None),
        Err(e) => (false, Some(e)),
    };
    let base = json!({"ok": true, "enabled": c.edsm.enabled, "commander": c.edsm.commander, "has_key": has_key,
        "key_error": key_error, "uploading": crate::pipeline::EDSM.get().is_some()});
    outbox_status(&c.data_dir().join(astromech_eddn::edsm::DB_FILE), base)
}

/// EDDN status for Settings → Data sharing and the send log for Diagnostics: the switch, queue
/// depth, last send and last error (a failed send, not a message discarded here), the gateway's answers and (apart, so they can't push those
/// off screen) the messages never queued. Reads `eddn.sqlite` only if it exists, so asking never
/// creates it.
pub fn eddn_log(data_dir: &Path, enabled: bool, test: bool) -> Value {
    let base = json!({"ok": true, "enabled": enabled, "test": test});
    outbox_status(&data_dir.join(astromech_eddn::DB_FILE), base)
}

/// `base` with an outbox's queue depth, last send, last error, send log and skips. Reads the
/// file only if it exists, so asking never creates it.
fn outbox_status(p: &Path, mut base: Value) -> Value {
    use astromech_eddn::Outcome::{Discarded, Dropped, GaveUp, Rejected, Retry, Sent};
    for (k, v) in [
        ("pending", json!(0)),
        ("last_sent", Value::Null),
        ("last_error", Value::Null),
        ("log", json!([])),
        ("skipped", json!([])),
    ] {
        base[k] = v;
    }
    if !p.is_file() {
        return base;
    }
    let read = || -> Result<Value> {
        let o = astromech_eddn::Outbox::open(p)?;
        let mut v = base.clone();
        v["pending"] = json!(o.pending()?);
        v["last_sent"] = json!(o.recent_of(&[Sent], 1)?.pop());
        // EDSM: errors logged before the commander name or key last changed are out of date.
        let cleared = o.meta(astromech_eddn::edsm::ERRORS_CLEARED)?.and_then(|t| t.parse::<i64>().ok());
        v["last_error"] = json!(o.recent_of_after(&[Rejected, Retry, GaveUp], cleared.unwrap_or(0), 1)?.pop());
        v["log"] = json!(o.recent_of(&[Sent, Rejected, Retry, GaveUp, Discarded], 50)?);
        v["skipped"] = json!(o.recent_of(&[Dropped], 50)?);
        Ok(v)
    };
    read().unwrap_or_else(|e| json!({"ok": false, "error": format!("reading {}: {e}", p.display())}))
}

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
    if core.models.lock().unwrap().is_some() {
        return;
    }
    *core.models.lock().unwrap() = Some("downloading speech models…".into());
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
                    *c.models.lock().unwrap() = Some(format!("{} {:?} {pct}%", a.label, p.phase));
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
        *c.models.lock().unwrap() = None;
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
        *core.game.lock().unwrap() = w.game;
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
            match control::call(&app_dir, "open_window", opts.route.as_deref()) {
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

    let config_path = choose_config(&opts, &paths);
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
    let (ui_tx, ui_rx) = channel();
    let core = Arc::new(Core {
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

    let c = core.clone();
    let _server = match control::serve(&app_dir, Arc::new(move |r: &Request| c.handle(r))) {
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

    // Release the voice preview's Kokoro once it has sat unused.
    let c = core.clone();
    std::thread::Builder::new()
        .name("preview-idle".into())
        .spawn(move || loop {
            std::thread::sleep(Duration::from_secs(10));
            if c.preview.release_idle(Instant::now(), crate::speech::PREVIEW_IDLE) {
                log("voice preview: Kokoro released (unused for a minute)");
            }
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
mod tests {
    use super::*;
    use astromech_voice::{catalog, download, models};

    /// Held by tests that edit the config through a core: every edit sets the process-wide
    /// EDDN switch.
    static CONFIG_EDITS: Mutex<()> = Mutex::new(());

    /// A core as `run` builds it, on the config file `text` in `dir`, without its threads.
    fn core(dir: &Path, text: &str) -> Arc<Core> {
        let config_path = dir.join("astromech.toml");
        std::fs::write(&config_path, text).unwrap();
        Arc::new(Core {
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
            core.handle(&Request { token: String::new(), cmd: cmd.into(), route: None, text: None, args })
        };
        assert_eq!(call("route", None)["view"]["route"], Value::Null);
        let v = call(
            "route_set",
            Some(json!({"name": "Trip", "source": "engineer trip", "waypoints": [
                {"system": "Deciat", "label": "Felicity Farseer"}, {"system": "Kuk"}]})),
        );
        assert_eq!(v["view"]["next"]["system"], "Deciat", "{v}");
        assert_eq!(v["view"]["route"]["source"], "engineer trip");
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

    /// Test mode is on the tray status line, over whatever the game is doing.
    #[test]
    fn the_status_line_says_when_push_to_talk_is_only_testing() {
        let d = tempfile::tempdir().unwrap();
        let core = core(d.path(), "");
        assert_eq!(core.status_line_for(false), "Waiting for Elite");
        assert_eq!(core.status_line_for(true), "Voice tester open · push-to-talk testing only");
        *core.game.lock().unwrap() = Game::Running;
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
            let (status, body) = self.0.lock().unwrap().remove(0);
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
        let no_wait = astromech_data::spansh::Polling { every: Duration::ZERO, timeout: Duration::from_secs(5) };
        let plotted = |plotter: Plotter, to: &str, answers: Vec<(u16, &'static str)>| {
            let client = astromech_data::Client::with_transport(
                astromech_data::Cache::in_memory().unwrap(),
                Arc::new(RecordedJob(Mutex::new(answers))),
            )
            .limiter(astromech_data::Limiter::none());
            let ask = Ask { plotter, to: to.into(), efficiency: 60, refuel_every_scoopable: true };
            serde_json::to_value(plot(&client, "Sol", &ask, &ship, no_wait).unwrap()).unwrap()
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
        assert_eq!(v["view"]["next"], json!({"system": "PSR J1752-2806", "label": "neutron boost"}));
        assert_eq!(v["view"]["remaining"], 2);

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
        assert_eq!(v["view"]["next"], json!({"system": "Tascheter Sector MN-T b3-6", "label": "fuel"}));
        assert_eq!(v["view"]["remaining"], 9);
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
        *core.cfg.write().unwrap() = c;
        let note = config_problems_note(&problems);
        *core.config_watch.lock().unwrap() = ConfigWatch::loaded(&core.config_path, note, false);
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
        *core.config_watch.lock().unwrap() = ConfigWatch::loaded(&core.config_path, Some("x".into()), false);
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
        *core.config_watch.lock().unwrap() =
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

    /// The window's fake core (`app/src/lib/fakecore.ts`) answers the setup and calibration
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
            core.handle(&Request { token: String::new(), cmd: cmd.into(), route: None, text: None, args: None })
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

    /// Done-when for astromech-c9em: a Bindings folder pasted in onboarding (`set_folders`, which
    /// the core writes to its config) is what the window reads, in the default, portable and
    /// `--config` layouts.
    #[test]
    fn a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout() {
        use astromech_store::window::WindowConfig;
        let env = |app: &Path| {
            let app = app.to_path_buf();
            move |k: &str| (k == "XDG_DATA_HOME").then(|| app.clone().into_os_string())
        };
        let paste = |config: &Path, dir: &str| {
            crate::setup::write(config, &[crate::setup::Edit::BindingsDir(dir.into())]).unwrap();
        };
        // `--config` somewhere else, with the window's Paths made afresh as the shell does.
        for layout in ["default", "portable", "--config"] {
            let (exe, app, elsewhere) =
                (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
            if layout == "portable" {
                std::fs::write(exe.path().join("astromech.toml"), "").unwrap();
            }
            let paths = || astromech_store::Paths::new(exe.path().into(), env(app.path()));
            let mut opts = Opts::default();
            if layout == "--config" {
                opts.config = Some(elsewhere.path().join("mine.toml"));
            }
            let used = choose_config(&opts, &paths());
            paste(&used, "D:\\Pasted\\Bindings");
            let seen = WindowConfig::load(&paths());
            assert_eq!(seen.game.bindings_dir, "D:\\Pasted\\Bindings", "{layout}");
            if layout == "--config" {
                assert_eq!(used, elsewhere.path().join("mine.toml"));
            }
        }
        // A later start without --config takes the window back to the default file.
        let (exe, app) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let paths = || astromech_store::Paths::new(exe.path().into(), env(app.path()));
        let custom = app.path().join("other.toml");
        choose_config(&Opts { config: Some(custom), ..Opts::default() }, &paths());
        choose_config(&Opts::default(), &paths());
        assert_eq!(paths().config_in_use(), paths().config_file());
    }

    fn call(core: &Arc<Core>, cmd: &str, args: Option<Value>) -> Value {
        core.handle(&Request { token: String::new(), cmd: cmd.into(), route: None, text: None, args })
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
}
