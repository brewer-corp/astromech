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
#[cfg(windows)]
use crate::tester::{self, TESTER};
use anyhow::{Context, Result};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{IntegrityReport, Platform};
use astromech_policy::Policy;
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
    stop: Arc<AtomicBool>,
    _pipe: crate::pipeline::Pipeline,
    _callouts: SpeechOut,
    _hw: Option<health::HealthWatch>,
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        #[cfg(windows)]
        bus::detach();
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
                o => format!("{o:?}"),
            }
        ),
        h => format!("{h:?}"),
    }
}

impl Session {
    /// Start everything except the buttons and microphone ([`Session::start_buttons`]).
    pub fn start(cfg: &Config, opts: &Opts) -> Result<Session> {
        let args = opts;
        let data = cfg.data_dir();
        std::fs::create_dir_all(&data).with_context(|| format!("creating {}", data.display()))?;
        // 1. Binds health check.
        let bindings = cfg.bindings_dir().context("can't find the Bindings folder; set game.bindings_dir")?;
        let live = cfg!(windows);
        let st = health::run_check(&bindings, cfg.game_dir().as_deref(), live);
        for l in st.summary().lines() {
            log(l);
        }
        write_health(&data, &st);
        let binds: health::Shared = Arc::new(RwLock::new(st));
        let data2 = data.clone();
        let hw = health::watch(bindings.clone(), cfg.game_dir(), live, binds.clone(), move |s| {
            log("Bindings folder changed; health check re-run:");
            for l in s.summary().lines() {
                log(l);
            }
            write_health(&data2, s);
        })
        .map_err(|e| log(&format!("not watching the Bindings folder: {e:#}")))
        .ok();

        // 2. Game state pipeline, feeding the callouts (spoken once the speaker exists, step 3).
        let hub = Arc::new(StateHub::new());
        let jdir = cfg.journal_dir().context("can't find the journal folder; set game.journal_dir")?;
        let engine = astromech_announce::Engine::new(astromech_announce::builtin(), &cfg.callouts);
        for p in engine.setting_problems() {
            log(&format!("config: {p}"));
        }
        let (pending, toasts) = (Arc::new(callouts::Pending::default()), Arc::new(Toasts::default()));
        let deliver = Box::new(callouts::router(pending.clone(), toasts.clone(), callouts::default_notifier()));
        let pipe = crate::pipeline::start(
            &jdir,
            hub.clone(),
            Some(&data.join("astromech.sqlite")),
            log,
            Some(Callouts { engine, deliver }),
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
        let worker = cfg.worker_exe();
        let models = cfg.models_dir();
        let voice_ok = !args.no_voice && worker.is_file();
        if !args.no_voice {
            use astromech_voice::models;
            let stt_ok = models::stt_paths(&models::model_root(&models, &models::STT)).is_ok();
            let tts_ok = models::find_tts(&models).is_ok();
            log(&format!(
                "voice: worker {} ({}); models {}: speech-to-text {}, text-to-speech {}",
                worker.display(),
                if worker.is_file() { "found" } else { "MISSING" },
                models.display(),
                if stt_ok { "found" } else { "MISSING" },
                if tts_ok { "found" } else { "MISSING" }
            ));
        }
        if !args.no_voice && !worker.is_file() {
            log(&format!("voice off: {} not found", worker.display()));
        }
        let speaker: Arc<dyn Speaker> = if voice_ok && cfg.voice.tts {
            match speech::TtsSpeaker::start(
                &worker,
                &models,
                cfg.voice.threads,
                &data,
                &cfg.voice.speaker,
                cfg.voice.speed,
                cfg.voice.colour,
                speech::fixed_phrases(),
            ) {
                Ok(s) => {
                    log(&format!(
                        "Kokoro text-to-speech loaded ({}, {})",
                        cfg.voice.speaker,
                        cfg.voice.colour.as_str()
                    ));
                    Arc::new(s)
                }
                Err(e) => {
                    log(&format!("speech output off ({e:#}); replies are printed"));
                    Arc::new(ConsoleSpeaker)
                }
            }
        } else {
            Arc::new(ConsoleSpeaker)
        };
        let stt: Option<Box<dyn Stt>> = if voice_ok {
            match speech::WorkerStt::start(&worker, &models, cfg.voice.threads, &data) {
                Ok(s) => {
                    log("Parakeet speech-to-text loaded");
                    Some(Box::new(s))
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
            if cfg.actions.dry_run { Arc::new(DryRun(real_platform())) } else { real_platform() };
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
            policy: Policy::new(platform, pcfg),
            hub: hub.clone(),
            binds,
            speaker: speaker.clone(),
            verify_timeout: Duration::from_millis(cfg.actions.verify_timeout_ms),
            log: Arc::new(|s: &str| log(s)),
        });
        let alias_store = astromech_store::Store::open(&data.join("astromech.sqlite"))
            .ok()
            .map(|s| Arc::new(std::sync::Mutex::new(s)));
        let assistant = Arc::new(match alias_store {
            Some(st) => Assistant::new(runner.clone()).with_store(st),
            None => Assistant::new(runner.clone()),
        });

        let voice = if args.no_voice {
            "voice off (--no-voice)".to_string()
        } else if !worker.is_file() {
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
            stop: Arc::new(AtomicBool::new(false)),
            _pipe: pipe,
            _callouts: speech_out,
            _hw: hw,
        })
    }

    /// Start push-to-talk, the kill switch and the microphone (Windows only).
    pub fn start_buttons(&mut self, cfg: &Config, opts: &Opts) -> Result<()> {
        let stt = self.stt.take();
        #[cfg(windows)]
        start_voice(cfg, self.assistant.clone(), self.runner.clone(), stt, opts.debug_input, self.stop.clone())?;
        #[cfg(not(windows))]
        {
            let _ = (cfg, opts);
            drop(stt);
            log("push-to-talk and the kill switch need Windows; type commands instead");
        }
        Ok(())
    }
}

#[cfg(windows)]
struct ClipJob {
    samples: Vec<f32>,
    rate: u32,
    sidecar: Option<std::path::PathBuf>,
    meta: serde_json::Value,
}

#[cfg(windows)]
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

#[cfg(windows)]
fn start_voice(
    cfg: &Config,
    assistant: Arc<Assistant>,
    runner: Arc<Runner>,
    stt: Option<Box<dyn Stt>>,
    debug_input: bool,
    stop: Arc<AtomicBool>,
) -> Result<()> {
    use crate::controls::{self, Control};
    let (rx, status, devices) = bus::attach(cfg, debug_input);
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
    let have_stt = stt.is_some();
    let want_mic = cfg.voice.mic.trim();
    let mic = if stt.is_some() {
        match astromech_voice::mic::Recorder::start_named((!want_mic.is_empty()).then_some(want_mic)) {
            Ok(m) => {
                log(&format!("microphone: {} ({} Hz)", m.device_name(), m.sample_rate()));
                if let Some(d) = &m.default_name {
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
        let a = assistant.clone();
        std::thread::Builder::new().name("stt".into()).spawn(move || {
            while let Ok(job) = jrx.recv() {
                let t0 = std::time::Instant::now();
                match stt.transcribe(&job.samples, job.rate) {
                    Ok(t) if t.is_empty() && !TESTER.armed(std::time::Instant::now()) => {
                        a.runner.speaker.say("Didn't catch that.")
                    }
                    Ok(t) => match tester::on_heard(&TESTER, &a, &t, std::time::Instant::now()) {
                        // The voice tester page is open: it shows what this would do. Nothing pressed.
                        tester::Heard::Tested(r) => {
                            log(&format!(
                                "voice tester heard {t:?} ({} ms): would {:?}, nothing pressed",
                                t0.elapsed().as_millis(),
                                r.would
                            ));
                            if let Some(p) = job.sidecar {
                                write_sidecar(&p, &job.meta, &t, r.intent.as_ref().map(|i| i.command.as_str()));
                            }
                        }
                        tester::Heard::Handled(h) => {
                            if matches!(h, crate::assistant::Handled::Ignored) {
                                log(&format!("heard {t:?} ({} ms): noise only, ignored", t0.elapsed().as_millis()));
                            } else {
                                log(&format!("heard {t:?} ({} ms)", t0.elapsed().as_millis()));
                                log(&format!("  -> {}", short(&h)));
                            }
                            if let Some(p) = job.sidecar {
                                let cmd = match &h {
                                    crate::assistant::Handled::Ran { command, .. }
                                    | crate::assistant::Handled::Asked { command, .. }
                                    | crate::assistant::Handled::NotAvailable(command) => Some(*command),
                                    _ => None,
                                };
                                write_sidecar(&p, &job.meta, &t, cmd);
                            }
                        }
                    },
                    Err(e) => log(&format!("speech-to-text failed: {e:#}")),
                }
                // After the reply (or the voice tester's result): callouts may speak again.
                callouts::VOICE_TURN.job_done();
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
        let key = format!("mic.{}.min_level_dbfs", m.device_name());
        if let Ok(st) = astromech_store::Store::open(&cfg.data_dir().join("astromech.sqlite")) {
            if let Ok(Some(v)) = st.get(&key) {
                if let Ok(n) = v.parse::<f32>() {
                    log(&format!("microphone: using calibrated min_level_dbfs {n:.0} for {}", m.device_name()));
                    min_level = n;
                }
            }
        }
    }
    let save_clips = cfg.voice.save_clips;
    let clips_dir = cfg.data_dir().join("clips");
    if save_clips {
        let _ = std::fs::create_dir_all(&clips_dir);
        log(&format!("debug: saving PTT clips under {}", clips_dir.display()));
    }
    let t_start = std::time::Instant::now();
    let secs = move |t: std::time::Instant| t.duration_since(t_start).as_secs_f64();
    std::thread::Builder::new().name("ptt".into()).spawn(move || {
        let mut from: Option<u64> = None;
        let mut down_at = std::time::Instant::now();
        let mut press_killed = false;
        let mut mic = mic;
        let report_mic = |mic: &mut Option<astromech_voice::mic::Recorder>| {
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
                    callouts::VOICE_TURN.ptt_down(std::time::Instant::now());
                    runner.speaker.interrupt();
                    report_mic(&mut mic);
                    down_at = std::time::Instant::now();
                    log(&format!("PTT down (t={:.3} s) — listening", secs(down_at)));
                    if let Some(m) = &mic {
                        let back = m.sample_rate() as u64 * pre / 1000;
                        from = Some(m.position().saturating_sub(back));
                    } else {
                        runner.speaker.say(if have_stt {
                            "No microphone found. Check the Windows sound input settings."
                        } else {
                            "Voice isn't loaded. Type commands in the console."
                        });
                    }
                }
                Control::PttUp => {
                    if let (Some(m), Some(f)) = (&mic, from.take()) {
                        // The release reaches us after the debounce window; time it from the
                        // real release and keep the post-roll measured from there too.
                        let up_at = std::time::Instant::now().checked_sub(debounce).unwrap_or(down_at).max(down_at);
                        let held = up_at.duration_since(down_at).as_millis() as u64;
                        std::thread::sleep(post.saturating_sub(debounce));
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
                                    let stamp = chrono::Utc::now().format("%Y%m%dT%H%M%S%.3fZ");
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
                                callouts::VOICE_TURN.job_queued(std::time::Instant::now());
                                if jtx.send(ClipJob { samples: n.samples, rate: m.sample_rate(), sidecar, meta }).is_err() {
                                    callouts::VOICE_TURN.job_done();
                                }
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
}
