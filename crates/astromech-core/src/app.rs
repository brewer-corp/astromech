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
    pub config_note: Option<String>,
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

    /// The tray's one status line (also its tooltip, under "astromech").
    pub fn status_line(&self) -> String {
        let game = *self.game.lock().unwrap();
        let mut s = match (game, self.active(), self.starting.load(Ordering::SeqCst)) {
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
        if self.config_note.is_some() {
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
        let (killed, toasts) = match self.session.lock().unwrap().as_ref() {
            Some(s) => (s.runner.policy.is_killed(), s.toasts.recent()),
            None => (false, vec![]),
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
            "log_dir": self.app_dir.join("logs").display().to_string(),
            "config": self.config_path.display().to_string(),
            "config_note": self.config_note,
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
        })
    }

    pub fn save_diagnostics(&self) -> Result<PathBuf> {
        let log_dir = crate::logging::dir().unwrap_or_else(|| self.app_dir.join("logs"));
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
        let r = Session::start(&cfg, &self.opts.session).and_then(|mut s| {
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
        if let Some(s) = self.session.lock().unwrap().take() {
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

    /// Fetch `latest.json`, and download the zip if it is newer. Safe while Elite is running.
    pub fn check_update(self: &Arc<Self>, manual: bool) {
        {
            let mut h = self.update.lock().unwrap();
            if matches!(h.snap.status, UpStatus::Checking | UpStatus::Downloading | UpStatus::Applying) {
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
        let install_dir = Config::exe_dir();
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
        match update::spawn_apply(&payload, &Config::exe_dir(), update::After::Exit) {
            Ok(()) => log("update: applying on quit"),
            Err(e) => log(&format!("update: apply on quit failed: {e:#}")),
        }
    }

    /// Launch (or focus) the app window at `route`.
    pub fn open_window(&self, route: Option<&str>) -> Result<(), String> {
        let exe =
            std::env::var_os("ASTROMECH_SHELL").map(PathBuf::from).unwrap_or_else(|| Config::exe_dir().join(SHELL_EXE));
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
            crate::setup::edit(&self.config_lock, &self.config_path, &edits, |mut c| {
                c.actions.dry_run |= self.opts.session.dry_run;
                *self.cfg.write().unwrap() = c;
            })
        });
        match res {
            Ok(()) => {
                log(&format!("config: {} saved from the window", r.cmd.trim_start_matches("set_")));
                if r.cmd == "set_voice" {
                    self.apply_voice();
                }
                self.notify();
                json!({"ok": true, "applies_next_session": r.cmd != "set_voice" && self.active()})
            }
            Err(e) => json!({"ok": false, "error": format!("{e:#}")}),
        }
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

    pub fn handle(self: &Arc<Self>, r: &Request) -> Value {
        match r.cmd.as_str() {
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
                let c = self.clone();
                std::thread::spawn(move || c.check_update(true));
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
            "prospector_thresholds" => {
                match astromech_store::Store::open(&self.cfg().data_dir().join("astromech.sqlite")) {
                    Ok(st) => crate::callouts::thresholds_json(&crate::callouts::load_thresholds(&st)),
                    Err(e) => json!({"ok": false, "error": format!("opening the database: {e}")}),
                }
            }
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
                            *s.prospector.write().unwrap() = t.clone();
                        }
                        log(&format!("prospector thresholds changed: {text}"));
                        crate::callouts::thresholds_json(&t)
                    }
                    Err(e) => json!({"ok": false, "error": e}),
                }
            }
            "find_elite" => json!({"ok": true, "found": crate::setup::find_elite(&self.cfg())}),
            "set_ptt" | "set_kill_switch" | "set_mic" | "set_folders" | "set_voice" => self.edit_config(r),
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

/// Create the config from the sample if it doesn't exist, then open it in the editor.
pub fn open_config(path: &Path) {
    if !path.exists() {
        if let Some(d) = path.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let _ = std::fs::write(path, crate::config::SAMPLE);
        log(&format!("wrote a sample config to {}; edit [controls], then quit and restart astromech", path.display()));
    }
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
    let app_dir = Config::app_dir();
    let _ = std::fs::create_dir_all(&app_dir);
    let log_dir =
        if Config::portable() { Config::exe_dir().join("astromech-data").join("logs") } else { app_dir.join("logs") };
    crate::logging::init(&log_dir, false);
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

    let config_path = opts.config.clone().unwrap_or_else(Config::default_path);
    let config_exists = config_path.is_file();
    let (mut cfg, config_note) = if config_exists {
        match Config::load(&config_path) {
            Ok(c) => {
                log(&format!("config: {}", config_path.display()));
                (c, None)
            }
            Err(e) => {
                log(&format!("config error, using defaults: {e:#}"));
                (Config::default(), Some("config error (see logs)".to_string()))
            }
        }
    } else {
        log(&format!("config: {} not found; using defaults (Settings → Open config file)", config_path.display()));
        (Config::default(), None)
    };
    if opts.session.dry_run {
        cfg.actions.dry_run = true;
    }

    if !Config::portable() {
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
        config_note,
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

    if !opts.background && !Config::portable() {
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
}
