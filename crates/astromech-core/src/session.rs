//! One "active" session (docs/V1-DESIGN.md §7.6): everything that runs while Elite does. The
//! binds health check and Bindings watch, the journal/Status pipeline, the speech models, the
//! microphone and the push-to-talk / kill-switch path. Dropping the [`Session`] stops it and
//! unloads the models (the voice workers exit); the Raw Input listener is started once and
//! stays (it costs nothing and Windows has no clean way to stop it).

use crate::actions::Runner;
use crate::assistant::Assistant;
use crate::callouts::{self, Callouts, SpeechOut, Toasts};
use crate::config::Config;
use crate::health::{self, BindsState};
use crate::hub::StateHub;
use crate::logging::log;
use crate::speech::{self, ConsoleSpeaker, Speaker, Stt};
use crate::tester::{self, TESTER};
use anyhow::{Context, Result};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{IntegrityReport, Platform};
use astromech_keybinds::devices::Inventory;
use astromech_policy::Policy;
use astromech_proto::Clock;
use astromech_store::Paths;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// "Pause voice" in the tray: push-to-talk presses are ignored while set.
pub static VOICE_PAUSED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Default)]
pub struct Opts {
    pub dry_run: bool,
    pub no_voice: bool,
    pub debug_input: bool,
}

/// The controllers connected now, for the binds check and the keybinds pages.
pub type Devices = Arc<dyn Fn() -> Inventory + Send + Sync>;

/// What a session reaches outside the process through, beside the core's own boundaries
/// ([`crate::app::Boundaries`]).
#[derive(Clone)]
pub struct Io {
    /// Key presses and the foreground window.
    pub platform: Arc<dyn Platform>,
    /// Push-to-talk, the kill switch and the paste button: None where nothing listens for them
    /// (the installed app off Windows).
    pub controls: Option<Arc<dyn crate::controls::Source>>,
    /// The microphone: push-to-talk, calibration and the mic picker.
    pub mic: Arc<dyn crate::mic::Mics>,
    /// The speech worker: speech-to-text, and the speaker for the session and Settings' preview.
    pub speech: Arc<dyn speech::Speech>,
    /// The controllers connected now.
    pub devices: Devices,
}

impl Io {
    /// The installed app's: Windows' key injection and focus, the Raw Input and XInput listener
    /// behind the bus (Windows only), cpal, the voice worker and the live device inventory.
    pub fn real() -> Self {
        #[cfg(windows)]
        let controls: Option<Arc<dyn crate::controls::Source>> = Some(Arc::new(BusSource));
        #[cfg(not(windows))]
        let controls = None;
        Self {
            platform: real_platform(),
            controls,
            mic: Arc::new(crate::mic::DeviceMics),
            speech: Arc::new(speech::WorkerSpeech),
            devices: Arc::new(astromech_keybinds::devices::enumerate_live),
        }
    }
}

pub struct Session {
    pub hub: Arc<StateHub>,
    pub runner: Arc<Runner>,
    pub assistant: Arc<Assistant>,
    pub speaker: Arc<dyn Speaker>,
    /// One line for the tray / status: "voice ready", "voice off: …".
    pub voice: String,
    /// Speech-to-text, until [`Session::start_buttons`] hands it to the PTT thread.
    pub stt: Option<Box<dyn Stt>>,
    /// Callout toasts for the app window.
    pub toasts: Arc<Toasts>,
    /// What the running rules read that the window changes (prospector thresholds, the
    /// shopping list).
    pub shared: astromech_announce::Shared,
    stop: Arc<AtomicBool>,
    io: Io,
    clock: Arc<dyn Clock>,
    _pipe: crate::pipeline::Pipeline,
    _callouts: SpeechOut,
    _hw: Option<health::HealthWatch>,
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(c) = &self.io.controls {
            c.detach();
        }
    }
}

/// Logs presses; forwards focus/integrity to the real platform.
pub struct DryRun(Arc<dyn Platform>);
impl Platform for DryRun {
    fn foreground_title(&self) -> Option<String> {
        self.0.foreground_title()
    }
    fn send_key(&self, k: ScanKey, up: bool) -> Result<(), String> {
        log(&format!(
            "[dry run] would send scan code 0x{:02X}{} {}",
            k.scan,
            if k.extended { " (extended)" } else { "" },
            if up { "up" } else { "down" }
        ));
        Ok(())
    }
    fn integrity(&self) -> IntegrityReport {
        self.0.integrity()
    }
}

pub fn real_platform() -> Arc<dyn Platform> {
    #[cfg(windows)]
    {
        Arc::new(astromech_input::exec::platform::win::WinPlatform)
    }
    #[cfg(not(windows))]
    {
        Arc::new(astromech_input::exec::platform::NoInputPlatform)
    }
}

pub fn write_health(dir: &Path, st: &BindsState) {
    let red = astromech_keybinds::report::Redactor::from_env();
    let _ = std::fs::create_dir_all(dir);
    let mut v = serde_json::json!({ "summary": st, "doctor": st.doctor });
    red.apply_json(&mut v);
    let _ = std::fs::write(dir.join("health.json"), serde_json::to_string_pretty(&v).unwrap_or_default());
    if let Some(d) = &st.doctor {
        let _ = std::fs::write(dir.join("health.html"), red.apply(&astromech_keybinds::report::html(d)));
    }
}

pub fn short(h: &crate::assistant::Handled) -> String {
    use crate::actions::Outcome;
    use crate::assistant::Handled;
    match h {
        Handled::Ran { command, outcome } => format!(
            "{command}: {}",
            match outcome {
                Outcome::Sent(_) => "pressed, verifying".to_string(),
                Outcome::Spoken(s) => s.clone(),
                Outcome::LookingUp(_) => "looking it up".to_string(),
                o => format!("{o:?}"),
            }
        ),
        h => format!("{h:?}"),
    }
}

impl Session {
    /// Start everything except the buttons and microphone ([`Session::start_buttons`]).
    /// `route` is the core's route follower; the session's callout rule moves it on as the
    /// commander jumps. `api` is the command layer's state: its one remote-data client and its
    /// star map serve the voice lookups too, so they share one rate limit and cache with the
    /// window's. `paths` places the folders the config doesn't name, and `notifier` shows the
    /// callouts' notifications (one at a time, behind [`callouts::Background`]). Its timers
    /// read and wait on `clock`, and its threads are registered with it. `files`: a world's file
    /// generations, which the pipeline publishes (None in the installed app). EDDN's upload and
    /// EDSM's API come from `http`; keys, buttons, the mic, speech and the devices from `io`.
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        cfg: &Config,
        paths: &Paths,
        opts: &Opts,
        route: Arc<crate::route::Desk>,
        api: &astromech_api::State,
        notifier: Arc<dyn callouts::Notifier>,
        clock: Arc<dyn astromech_proto::Clock>,
        files: Option<Arc<crate::world::settle::FileGens>>,
        http: &crate::app::Http,
        io: &Io,
    ) -> Result<Session> {
        let args = opts;
        let data = cfg.data_dir(paths);
        std::fs::create_dir_all(&data).with_context(|| format!("creating {}", data.display()))?;
        // Open the star map now, off the speech thread (it can seed from the preload), so the
        // voice lookups find it open (`Runner::stars_now`).
        let stars = api.galaxy_holder().clone();
        std::thread::Builder::new()
            .name("star-map".into())
            .spawn(move || {
                let _ = stars.get();
            })
            .context("starting the star map's thread")?;
        // 1. Binds health check.
        let bindings = cfg.bindings_dir().context("can't find the Bindings folder; set game.bindings_dir")?;
        let st = health::run_check_with(&bindings, cfg.game_dir().as_deref(), (io.devices)(), &*clock);
        for l in st.summary().lines() {
            log(l);
        }
        write_health(&data, &st);
        let binds: health::Shared = Arc::new(RwLock::new(st));
        let data2 = data.clone();
        let hw = health::watch(
            bindings.clone(),
            cfg.game_dir(),
            io.devices.clone(),
            binds.clone(),
            move |s| {
                log("Bindings folder changed; health check re-run:");
                for l in s.summary().lines() {
                    log(l);
                }
                write_health(&data2, s);
            },
            clock.clone(),
        )
        .map_err(|e| log(&format!("not watching the Bindings folder: {e:#}")))
        .ok();

        // 2. Game state pipeline, feeding the callouts (spoken once the speaker exists, step 3).
        let hub = Arc::new(StateHub::with_clock(clock.clone()));
        let jdir = cfg.journal_dir(paths).context("can't find the journal folder; set game.journal_dir")?;
        let shared = callouts::shared_from_store(&data, &|m: &str| log(m));
        shared.visits.set(Arc::new(crate::visits::HistoryVisits::new(data.join(crate::history::DB_FILE))));
        // The process's one remote-data client: the known-biology and bodies-known callouts look
        // up through it, as the voice lookups below do.
        let remote = api.remote().clone();
        shared.known_bio.set(Arc::new(crate::biology::SpanshBiology::new(remote.clone())));
        shared
            .known_bodies
            .set(Arc::new(crate::known_bodies::EdsmBodies::new(remote.clone(), Arc::new(|s: &str| log(s)))));
        shared.sessions.set(Arc::new(crate::summary::HistorySessions::new(data.join(crate::history::DB_FILE))));
        // The route follower is the `route_follow` callout rule: live arrivals only.
        shared.follower.set(route.clone());
        let engine = astromech_announce::Engine::new(astromech_announce::builtin(shared.clone()), &cfg.callouts);
        for p in engine.setting_problems() {
            log(&format!("config: {p}"));
        }
        let (pending, toasts) = (Arc::new(callouts::Pending::new(clock.clone())), Arc::new(Toasts::new(clock.clone())));
        let deliver = Box::new(callouts::router(
            pending.clone(),
            toasts.clone(),
            Arc::new(
                callouts::Background::start(notifier, callouts::NOTIFY_GAP, clock.clone())
                    .context("starting the notification thread")?,
            ),
        ));
        let eddn = crate::pipeline::eddn_feed(cfg, &data, clock.clone(), (http.eddn)());
        let pipe = crate::pipeline::start_with_eddn(
            &jdir,
            hub.clone(),
            Some(&data.join("astromech.sqlite")),
            log,
            eddn,
            crate::pipeline::edsm_uploader(
                &data,
                clock.clone(),
                (http.edsm)(astromech_eddn::edsm::api::REQUEST_TIMEOUT),
            ),
            Some(Callouts { engine, deliver }),
            files,
        )
        .with_context(|| format!("journal folder {}", jdir.display()))?;
        let g = hub.snapshot();
        log(&format!(
            "game state: {} | system {} | status flags {}",
            g.commander.clone().unwrap_or_else(|| "no commander yet".into()),
            g.system.clone().unwrap_or_else(|| "?".into()),
            if g.status_seen { format!("0x{:08X}", g.status_flags) } else { "not seen yet (start the game)".into() }
        ));

        // 3. Voice.
        let worker = cfg.worker_exe(paths);
        let models = cfg.models_dir(paths);
        let speech = io.speech.clone();
        let voice_ok = !args.no_voice && speech.worker_found(&worker);
        if !args.no_voice {
            let stt_ok = speech.stt_found(&models);
            let tts_ok = speech.tts_found(&models);
            log(&format!(
                "voice: worker {} ({}); models {}: speech-to-text {}, text-to-speech {}",
                worker.display(),
                if speech.worker_found(&worker) { "found" } else { "MISSING" },
                models.display(),
                if stt_ok { "found" } else { "MISSING" },
                if tts_ok { "found" } else { "MISSING" }
            ));
        }
        if !args.no_voice && !speech.worker_found(&worker) {
            log(&format!("voice off: {} not found", worker.display()));
        }
        let speaker: Arc<dyn Speaker> = if voice_ok && cfg.voice.tts {
            let (exe, dir, data, v) = (worker.clone(), models.clone(), data.clone(), cfg.voice.clone());
            let sp = speech.clone();
            let start = move || -> Result<Arc<dyn Speaker>> {
                let r = sp.speaker(&exe, &dir, &data, &v, speech::fixed_phrases());
                match r {
                    Ok(s) => {
                        log(&format!("Kokoro text-to-speech loaded ({}, {})", v.speaker, v.colour.as_str()));
                        Ok(s)
                    }
                    Err(e) => {
                        log(&format!("speech output off ({e:#}); replies are printed"));
                        Err(e)
                    }
                }
            };
            if speech.tts_found(&models) {
                start().unwrap_or_else(|_| Arc::new(ConsoleSpeaker))
            } else {
                // Still downloading: speak from the first line said once the download completes.
                log("text-to-speech model not downloaded yet; replies are printed until it is");
                let (m, sp) = (models.clone(), speech.clone());
                Arc::new(speech::LazySpeaker::new(move || sp.tts_found(&m), start))
            }
        } else {
            Arc::new(ConsoleSpeaker)
        };
        let stt: Option<Box<dyn Stt>> = if voice_ok {
            match speech.stt(&worker, &models, cfg.voice.threads, &data) {
                Ok(s) => {
                    log("Parakeet speech-to-text loaded");
                    Some(s)
                }
                Err(e) => {
                    log(&format!(
                        "speech input off ({e:#}); type commands instead (models folder: {})",
                        models.display()
                    ));
                    None
                }
            }
        } else {
            None
        };

        let speech_out = SpeechOut::start(pending, speaker.clone(), &callouts::VOICE_TURN)
            .context("starting the callout speaker")?;

        // 4. Policy + executor.
        let platform: Arc<dyn Platform> =
            if cfg.actions.dry_run { Arc::new(DryRun(io.platform.clone())) } else { io.platform.clone() };
        if cfg.actions.dry_run {
            log("DRY RUN: key presses are logged, not sent");
        }
        match platform.integrity().verdict() {
            Ok(m) => log(&format!("integrity: {m}")),
            Err(m) => log(&format!("WARNING integrity: {m}")),
        }
        let pcfg = astromech_policy::Config {
            hold: Duration::from_millis(cfg.actions.hold_ms),
            skip_window_checks: cfg.actions.dry_run && !cfg!(windows),
            ..Default::default()
        };
        let runner = Arc::new(Runner {
            policy: Policy::with_clock(platform, pcfg, clock.clone()),
            hub: hub.clone(),
            binds,
            speaker: speaker.clone(),
            verify_timeout: Duration::from_millis(cfg.actions.verify_timeout_ms),
            log: Arc::new(|s: &str| log(s)),
            remote,
            stars: api.galaxy_holder().clone(),
        });
        let alias_store = astromech_store::Store::open(&data.join("astromech.sqlite"))
            .ok()
            .map(|s| Arc::new(std::sync::Mutex::new(s)));
        let assistant = Assistant::new(runner.clone())
            .with_sessions(shared.sessions.clone())
            .with_journals(jdir.clone())
            .with_history(data.join(crate::history::DB_FILE))
            .with_route(route);
        let assistant = Arc::new(match alias_store {
            Some(st) => assistant.with_store(st),
            None => assistant,
        });

        let voice = if args.no_voice {
            "voice off (--no-voice)".to_string()
        } else if !speech.worker_found(&worker) {
            format!("voice off: {} missing", astromech_voice::client::WORKER_EXE)
        } else if stt.is_none() {
            "voice off: speech models missing or failed to load".to_string()
        } else {
            "voice ready".to_string()
        };
        Ok(Session {
            hub,
            runner,
            assistant,
            speaker,
            voice,
            stt,
            toasts,
            shared,
            stop: Arc::new(AtomicBool::new(false)),
            io: io.clone(),
            clock,
            _pipe: pipe,
            _callouts: speech_out,
            _hw: hw,
        })
    }

    /// Start push-to-talk, the kill switch and the microphone (where something listens for the
    /// buttons: on Windows, or in a world).
    pub fn start_buttons(&mut self, cfg: &Config, paths: &Paths, opts: &Opts) -> Result<()> {
        let stt = self.stt.take();
        match &self.io.controls {
            Some(controls) => start_voice(
                cfg,
                &cfg.data_dir(paths),
                self.assistant.clone(),
                self.runner.clone(),
                stt,
                opts.debug_input,
                self.stop.clone(),
                controls.as_ref(),
                self.io.mic.as_ref(),
                self.clock.clone(),
            )?,
            None => {
                drop(stt);
                log("push-to-talk and the kill switch need Windows; type commands instead");
            }
        }
        Ok(())
    }
}

struct ClipJob {
    samples: Vec<f32>,
    rate: u32,
    sidecar: Option<std::path::PathBuf>,
    meta: serde_json::Value,
    /// Holds callouts until this clip is handled (dropped).
    _turn: callouts::TurnJob<'static>,
    /// Whether the voice tester's test mode was on when push-to-talk was released: the phrase is
    /// tested or run by this, whatever the page does while the clip is transcribed.
    testing: bool,
}

fn write_sidecar(path: &std::path::Path, meta: &serde_json::Value, transcript: &str, command: Option<&str>) {
    let mut v = meta.clone();
    if let serde_json::Value::Object(m) = &mut v {
        m.insert("transcript".into(), transcript.into());
        if let Some(c) = command {
            m.insert("command".into(), c.into());
        }
    }
    let _ = std::fs::write(path, serde_json::to_string_pretty(&v).unwrap_or_default());
}

/// The push-to-talk loop and its speech-to-text thread. Its presses come from `controls`, its
/// clips from `mics`; it reads the time and waits on `clock`, and its thread is registered with
/// it (the speech-to-text thread is not).
#[allow(clippy::too_many_arguments)]
fn start_voice(
    cfg: &Config,
    data: &Path,
    assistant: Arc<Assistant>,
    runner: Arc<Runner>,
    stt: Option<Box<dyn Stt>>,
    debug_input: bool,
    stop: Arc<AtomicBool>,
    controls: &dyn crate::controls::Source,
    mics: &dyn crate::mic::Mics,
    clock: Arc<dyn Clock>,
) -> Result<()> {
    use crate::controls::{self, Control};
    let (rx, status, devices) = controls.attach(cfg, debug_input);
    log(&format!(
        "input listener: {status}{}",
        if debug_input { " (--debug-input: logging every button)" } else { "" }
    ));
    match &cfg.controls.ptt {
        Some(b) => log(&controls::describe_button("push-to-talk", b, &devices)),
        None => log("push-to-talk: NOT SET"),
    }
    match &cfg.controls.kill_switch {
        Some(b) => log(&controls::describe_button("kill switch", b, &devices)),
        None => log("kill switch: NOT SET (use !kill in the console)"),
    }
    if let Some(b) = &cfg.controls.paste {
        log(&controls::describe_button("paste button", b, &devices));
    }
    let have_stt = stt.is_some();
    let want_mic = cfg.voice.mic.trim();
    let mic = if stt.is_some() {
        match mics.open((!want_mic.is_empty()).then_some(want_mic)) {
            Ok(m) => {
                log(&format!("microphone: {} ({} Hz)", m.device_name(), m.sample_rate()));
                if let Some(d) = &m.default_name() {
                    if !d.eq_ignore_ascii_case(&m.device_name()) {
                        log(&format!("microphone: Windows default is {d}; recording the chosen device instead"));
                    }
                }
                Some(m)
            }
            Err(e) => {
                log(&format!("microphone unavailable: {e:#}"));
                None
            }
        }
    } else {
        None
    };
    let (jtx, jrx) = std::sync::mpsc::channel::<ClipJob>();
    if let Some(mut stt) = stt {
        let (a, c) = (assistant.clone(), clock.clone());
        std::thread::Builder::new().name("stt".into()).spawn(move || {
            while let Ok(job) = jrx.recv() {
                let t0 = c.now();
                let ms = || c.now().saturating_duration_since(t0).as_millis();
                match stt.transcribe(&job.samples, job.rate) {
                    Ok(t) if t.is_empty() && !job.testing => a.runner.speaker.say("Say again."),
                    Ok(t) => match tester::on_heard(&TESTER, &a, &t, job.testing) {
                        // The voice tester page is open: it shows what this would do. Nothing pressed.
                        tester::Heard::Tested(r) => {
                            log(&format!(
                                "voice tester heard {t:?} ({} ms): would {:?}, nothing pressed",
                                ms(),
                                r.would
                            ));
                            a.runner.speaker.say(tester::TEST_ONLY);
                            if let Some(p) = job.sidecar {
                                write_sidecar(&p, &job.meta, &t, r.intent.as_ref().map(|i| i.command.as_str()));
                            }
                        }
                        tester::Heard::Handled(h) => {
                            if matches!(h, crate::assistant::Handled::Ignored) {
                                log(&format!("heard {t:?} ({} ms): noise only, ignored", ms()));
                            } else {
                                log(&format!("heard {t:?} ({} ms)", ms()));
                                log(&format!("  -> {}", short(&h)));
                            }
                            if let Some(p) = job.sidecar {
                                let cmd = match &h {
                                    crate::assistant::Handled::Ran { command, .. }
                                    | crate::assistant::Handled::Asked { command, .. }
                                    | crate::assistant::Handled::Lookup { command, .. }
                                    | crate::assistant::Handled::NotAvailable(command) => Some(*command),
                                    _ => None,
                                };
                                write_sidecar(&p, &job.meta, &t, cmd);
                            }
                        }
                    },
                    Err(e) => log(&format!("speech-to-text failed: {e:#}")),
                }
                // `job` (and its voice turn) ends here, after the reply or the voice tester's
                // result; a panic above ends it too.
            }
        })?;
    }
    let pre = cfg.voice.pre_roll_ms;
    let post = Duration::from_millis(cfg.voice.post_roll_ms);
    let rearm_hold = Duration::from_millis(cfg.controls.rearm_hold_ms);
    let debounce = Duration::from_millis(cfg.controls.debounce_ms);
    let min_hold = cfg.voice.min_hold_ms;
    let mut min_level = cfg.voice.min_level_dbfs;
    if let Some(m) = &mic {
        if let Ok(st) = astromech_store::Store::open(&data.join("astromech.sqlite")) {
            if let Some(n) = crate::calibration::stored_gate(&st, &m.device_name()) {
                log(&format!("microphone: using calibrated min_level_dbfs {n:.0} for {}", m.device_name()));
                min_level = n;
            }
        }
    }
    let save_clips = cfg.voice.save_clips;
    let clips_dir = data.join("clips");
    if save_clips {
        let _ = std::fs::create_dir_all(&clips_dir);
        log(&format!("debug: saving PTT clips under {}", clips_dir.display()));
    }
    let t_start = clock.now();
    let secs = move |t: std::time::Instant| t.duration_since(t_start).as_secs_f64();
    let clk = clock.clone();
    clock.spawn(std::thread::Builder::new().name("ptt".into()), move || {
        let mut from: Option<u64> = None;
        let mut down_at = clk.now();
        let mut press_killed = false;
        let mut mic = mic;
        let report_mic = |mic: &mut Option<Box<dyn crate::mic::Mic>>| {
            let Some(m) = mic.as_mut() else { return };
            if let Some(e) = m.take_errors() {
                log(&format!(
                    "microphone: {} stream error(s) ({} under/overruns so far; last: {}). Capture continues; \
                     at most a few ms of audio were lost",
                    e.new,
                    e.xruns,
                    e.last.unwrap_or_default()
                ));
            }
            match m.recover() {
                Ok(true) => log(&format!("microphone: stream restarted ({}, {} Hz)", m.device_name(), m.sample_rate())),
                Ok(false) => {}
                Err(e) => log(&format!("microphone: restart failed: {e:#}")),
            }
        };
        loop {
            let c = match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(c) => c,
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    if stop.load(Ordering::SeqCst) {
                        break;
                    }
                    if from.is_none() {
                        report_mic(&mut mic);
                    }
                    continue;
                }
                Err(_) => break,
            };
            match c {
                Control::PttDown if VOICE_PAUSED.load(Ordering::SeqCst) => {
                    log("PTT down — voice is paused (tray menu), ignored");
                }
                Control::PttUp if from.is_none() && VOICE_PAUSED.load(Ordering::SeqCst) => callouts::VOICE_TURN.ptt_up(),
                Control::PttDown => {
                    callouts::VOICE_TURN.ptt_down(clk.now());
                    runner.speaker.interrupt();
                    report_mic(&mut mic);
                    down_at = clk.now();
                    log(&format!("PTT down (t={:.3} s) — listening", secs(down_at)));
                    if let Some(m) = &mic {
                        let back = m.sample_rate() as u64 * pre / 1000;
                        from = Some(m.position().saturating_sub(back));
                    } else {
                        runner.speaker.say(if have_stt {
                            "No microphone found. Check the Windows sound input settings."
                        } else {
                            "Voice is not loaded. Type commands in the console."
                        });
                    }
                }
                Control::PttUp => {
                    if let (Some(m), Some(f)) = (&mic, from.take()) {
                        // The release reaches us after the debounce window; time it from the
                        // real release and keep the post-roll measured from there too.
                        let up_at = clk.now().checked_sub(debounce).unwrap_or(down_at).max(down_at);
                        let held = up_at.duration_since(down_at).as_millis() as u64;
                        // Test mode as it is at release; the clip may take seconds to transcribe.
                        let testing = TESTER.armed(clk.now());
                        clk.sleep(post.saturating_sub(debounce));
                        // Real time: the device's next callback is device timing.
                        #[allow(clippy::disallowed_methods)]
                        m.wait_after(std::time::Instant::now(), Duration::from_millis(100));
                        let samples = m.take(f, m.position());
                        let st = crate::speech::clip_stats(&samples, m.sample_rate());
                        let head = format!(
                            "PTT up (t={:.3} s) — {} held {held} ms; clip {} ms, RMS {:.0} dBFS, loudest frame {:.0} dBFS",
                            secs(up_at),
                            m.device_name(),
                            st.audio_ms,
                            st.rms_dbfs,
                            st.peak_frame_dbfs
                        );
                        use crate::speech::{gate, Gate};
                        match gate(held, &st, min_hold, min_level) {
                            Gate::Pass => {
                                let n = astromech_voice::gain::normalise(&samples, m.sample_rate());
                                log(&format!("{head}; gain {:+.0} dB; transcribing", n.gain_db));
                                let mut sidecar = None;
                                let mut meta = serde_json::json!({});
                                if save_clips {
                                    let stamp = chrono::DateTime::<chrono::Utc>::from(clk.utc()).format("%Y%m%dT%H%M%S%.3fZ");
                                    let wav = clips_dir.join(format!("{stamp}.wav"));
                                    if astromech_voice::wav::write_mono_i16(&wav, m.sample_rate(), &n.samples).is_ok() {
                                        sidecar = Some(wav.with_extension("json"));
                                        meta = serde_json::json!({
                                            "device": m.device_name(),
                                            "rate": m.sample_rate(),
                                            "held_ms": held,
                                            "rms_dbfs": st.rms_dbfs,
                                            "peak_frame_dbfs": st.peak_frame_dbfs,
                                            "gain_db": n.gain_db,
                                            "wav": wav.file_name().map(|x| x.to_string_lossy().to_string()),
                                        });
                                    }
                                }
                                let _ = jtx.send(ClipJob {
                                    samples: n.samples,
                                    rate: m.sample_rate(),
                                    sidecar,
                                    meta,
                                    _turn: callouts::VOICE_TURN.job_queued(clk.now()),
                                    testing,
                                });
                            }
                            Gate::TooShort => log(&format!("{head}; shorter than {min_hold} ms, ignored")),
                            Gate::TooQuiet => log(&format!("{head}; quieter than {min_level:.0} dBFS, ignored")),
                        }
                    } else if mic.is_none() {
                        log("PTT up — no speech-to-text or microphone, nothing to transcribe");
                    } else {
                        log("PTT up without a PTT down (ignored)");
                    }
                    // After any clip was queued as a job, so the hold doesn't lapse in between.
                    callouts::VOICE_TURN.ptt_up();
                }
                Control::PasteDown => {
                    log("paste button pressed");
                    let r = runner.clone();
                    // A local button, like push-to-talk: the voice origin's limits apply.
                    std::thread::spawn(move || r.paste(astromech_proto::Origin::Voice));
                }
                Control::KillDown => {
                    if !runner.policy.is_killed() {
                        runner.speaker.interrupt();
                        runner.policy.kill();
                        press_killed = true;
                        runner.speaker.say("Kill switch on. All actions stopped.");
                    }
                }
                Control::KillUp(held) => {
                    if press_killed {
                        press_killed = false;
                    } else if runner.policy.is_killed() && held >= rearm_hold {
                        runner.policy.rearm();
                        runner.speaker.say("Re-armed.");
                    } else if runner.policy.is_killed() {
                        runner.speaker.say("Still killed. Hold the kill switch to re-arm.");
                    }
                }
            }
        }
    })?;
    Ok(())
}

/// The installed app's control source on Windows: the [`bus`].
#[cfg(windows)]
struct BusSource;

#[cfg(windows)]
impl crate::controls::Source for BusSource {
    fn attach(
        &self,
        cfg: &Config,
        debug: bool,
    ) -> (Box<dyn crate::controls::ControlRx>, String, Vec<astromech_input::listen::model::DeviceMeta>) {
        let (rx, status, devices) = bus::attach(cfg, debug);
        (Box::new(rx), status, devices)
    }
    fn detach(&self) {
        bus::detach();
    }
}

/// The Raw Input/XInput listener outlives sessions: it's started once, and each session
/// attaches to receive its push-to-talk and kill-switch events.
#[cfg(windows)]
mod bus {
    use crate::config::Config;
    use crate::controls::Control;
    use astromech_input::listen::model::DeviceMeta;
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::sync::{Mutex, OnceLock};

    struct Bus {
        current: Mutex<Option<Sender<Control>>>,
        status: String,
        devices: Vec<DeviceMeta>,
    }
    static BUS: OnceLock<Bus> = OnceLock::new();

    pub fn attach(cfg: &Config, debug: bool) -> (Receiver<Control>, String, Vec<DeviceMeta>) {
        let b = BUS.get_or_init(|| {
            let (tx, rx) = channel::<Control>();
            let (status, devices) = crate::controls::start(
                cfg.controls.ptt.clone(),
                cfg.controls.kill_switch.clone(),
                cfg.controls.paste.clone(),
                debug,
                cfg.controls.debounce_ms,
                tx,
                crate::logging::log,
            );
            std::thread::Builder::new()
                .name("controls-bus".into())
                .spawn(move || {
                    while let Ok(c) = rx.recv() {
                        if let Some(b) = BUS.get() {
                            if let Some(s) = b.current.lock().unwrap().as_ref() {
                                let _ = s.send(c);
                            }
                        }
                    }
                })
                .expect("spawn controls bus");
            Bus { current: Mutex::new(None), status, devices }
        });
        let (tx, rx) = channel();
        *b.current.lock().unwrap() = Some(tx);
        (rx, b.status.clone(), b.devices.clone())
    }

    pub fn detach() {
        if let Some(b) = BUS.get() {
            *b.current.lock().unwrap() = None;
        }
    }

    /// Send `c` to the attached session, as the controls-bus thread does with the listener's.
    #[cfg(test)]
    pub fn forward(c: Control) {
        if let Some(b) = BUS.get() {
            if let Some(s) = b.current.lock().unwrap().as_ref() {
                let _ = s.send(c);
            }
        }
    }
}

/// The stand-in for the in-game push-to-talk path on Windows: `start_voice` on the real
/// `bus::attach` (the real Raw Input and XInput listener and the bus), with a fake microphone,
/// speech-to-text and speaker. The S7 replay's presses go through `controls::Mapper` into the
/// bus as its thread forwards the listener's.
#[cfg(all(test, windows))]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod windows_tests {
    use super::*;
    use crate::config::Button;
    use crate::controls::{Control, Mapper};
    use crate::mic::{Mic, Mics, StreamErrors};
    use astromech_input::exec::platform::mock::MockGame;
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::sync::Mutex;
    use std::time::Instant;

    /// One loud second of tone, there from the start.
    struct ToneMic;
    impl Mic for ToneMic {
        fn device_name(&self) -> String {
            "Test microphone".into()
        }
        fn sample_rate(&self) -> u32 {
            16_000
        }
        fn default_name(&self) -> Option<String> {
            None
        }
        fn position(&self) -> u64 {
            16_000
        }
        fn take(&self, from: u64, to: u64) -> Vec<f32> {
            (from..to.min(16_000)).map(|i| 0.5 * (i as f32 * 0.2).sin()).collect()
        }
        fn wait_after(&self, _t: Instant, _max: Duration) {}
        fn take_errors(&self) -> Option<StreamErrors> {
            None
        }
        fn recover(&mut self) -> Result<bool> {
            Ok(false)
        }
    }
    struct ToneMics;
    impl Mics for ToneMics {
        fn open(&self, _want: Option<&str>) -> Result<Box<dyn Mic>> {
            Ok(Box::new(ToneMic))
        }
        fn record(&self, _want: &str, _ms: u64) -> Result<crate::calibration::Clip> {
            anyhow::bail!("not in this test")
        }
        fn names(&self) -> (Vec<String>, Option<String>) {
            (vec![], None)
        }
    }

    /// Each line said, to the test.
    struct Said(Mutex<Sender<String>>);
    impl Speaker for Said {
        fn say(&self, text: &str) {
            let _ = self.0.lock().unwrap().send(text.to_string());
        }
    }

    fn heard(rx: &Receiver<String>, line: &str) {
        loop {
            match rx.recv_timeout(Duration::from_secs(30)) {
                Ok(l) if l == line => return,
                Ok(_) => continue,
                Err(e) => panic!("{line:?} was never said: {e}"),
            }
        }
    }

    const BINDS: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="Keys" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="Keyboard" Key="Key_L" /><Secondary Device="{NoDevice}" Key="" /></LandingGearToggle>
</Root>"#;

    #[test]
    fn push_to_talk_and_the_kill_switch_through_the_real_bus() {
        use astromech_input::exec::keys::{lookup, KeyLookup};
        let dir = tempfile::tempdir().unwrap();
        let bindings = dir.path().join("Bindings");
        std::fs::create_dir_all(&bindings).unwrap();
        std::fs::write(bindings.join("Keys.4.2.binds"), BINDS).unwrap();
        std::fs::write(bindings.join("StartPreset.4.start"), "Keys\nKeys\nKeys\nKeys\n").unwrap();
        let KeyLookup::Ok(l) = lookup("Key_L") else { panic!("Key_L") };
        let game = Arc::new(MockGame::new(
            dir.path().join("Status.json"),
            l,
            astromech_status::FLAG_GEAR,
            astromech_status::FLAG_IN_MAIN_SHIP,
            Duration::ZERO,
        ));
        let hub = Arc::new(StateHub::new());
        hub.update(|g| {
            g.status_flags = astromech_status::FLAG_IN_MAIN_SHIP;
            g.status_seen = true;
        });
        let (tx, said) = channel();
        let speaker: Arc<dyn Speaker> = Arc::new(Said(Mutex::new(tx)));
        let runner = Arc::new(Runner {
            policy: Policy::new(game.clone(), Default::default()),
            hub,
            binds: Arc::new(RwLock::new(health::run_check(
                &bindings,
                Some(&dir.path().join("nogame")),
                false,
                &astromech_proto::SystemClock,
            ))),
            speaker,
            verify_timeout: Duration::from_millis(200),
            log: Arc::new(|_: &str| {}),
            remote: Arc::new(astromech_data::Client::new(astromech_data::Cache::in_memory().unwrap())),
            stars: Arc::new(astromech_api::GalaxyHolder::new(|| anyhow::bail!("no star map in this test"))),
        });
        let assistant = Arc::new(Assistant::new(runner.clone()));
        let (ptt, kill) = (
            Button { device: "231D3201".into(), key: "Joy_4".into() },
            Button { device: "231D0200".into(), key: "Joy_1".into() },
        );
        let mut cfg = Config::default();
        cfg.controls.ptt = Some(ptt.clone());
        cfg.controls.kill_switch = Some(kill.clone());
        // The replay's presses are forwarded at once, so the hold is measured as nothing.
        cfg.voice.min_hold_ms = 0;
        let stt: Box<dyn Stt> = Box::new(speech::FakeStt::new(&["gear down"]));
        let stop = Arc::new(AtomicBool::new(false));
        start_voice(
            &cfg,
            dir.path(),
            assistant,
            runner.clone(),
            Some(stt),
            false,
            stop.clone(),
            &BusSource,
            &ToneMics,
            Arc::new(astromech_proto::SystemClock),
        )
        .unwrap();

        let f = concat!(env!("CARGO_MANIFEST_DIR"), "/../astromech-input/tests/fixtures/replay-vkb.json");
        let rf: astromech_input::listen::replay::ReplayFile =
            serde_json::from_str(&std::fs::read_to_string(f).unwrap()).unwrap();
        let mut m = Mapper::new(Some(ptt), Some(kill), false, |_: &str| {});
        for d in rf.devices {
            m.add_device(d);
        }
        let mut got = vec![];
        for s in &rf.samples {
            got.extend(m.sample(s));
        }
        got.extend(m.tick(1e9));
        assert_eq!(&got[..3], &[Control::PttDown, Control::PttUp, Control::KillDown], "{got:?}");

        bus::forward(got[0]);
        bus::forward(got[1]);
        heard(&said, "Landing gear deployed.");
        assert!(game.log.lock().unwrap().contains(&(l, false)), "Key_L went to the game");
        bus::forward(got[2]);
        heard(&said, "Kill switch on. All actions stopped.");
        assert!(runner.policy.is_killed());
        stop.store(true, Ordering::SeqCst);
        bus::detach();
    }
}
