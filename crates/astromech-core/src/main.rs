//! astromech-core (gear-down preview): console app. See README "Try the gear-down slice".

use anyhow::{bail, Context, Result};
use astromech_core::actions::Runner;
use astromech_core::assistant::Assistant;
use astromech_core::config::{Config, SAMPLE};
use astromech_core::health::{self, BindsState};
use astromech_core::hub::StateHub;
use astromech_core::speech::{self, ConsoleSpeaker, Speaker, Stt};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{IntegrityReport, Platform};
use astromech_policy::Policy;
use astromech_proto::Origin;
use clap::Parser;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about = "astromech core (gear-down preview): push-to-talk ship commands for Elite Dangerous")]
struct Args {
    /// Config file (default: astromech.toml next to the exe)
    #[arg(long)]
    config: Option<PathBuf>,
    /// Write a sample astromech.toml next to the exe and exit
    #[arg(long)]
    write_config: bool,
    /// Log key presses instead of sending them
    #[arg(long)]
    dry_run: bool,
    /// Don't load the speech models (typed commands only, printed replies)
    #[arg(long)]
    no_voice: bool,
    /// Run the binds health check once, print it and exit
    #[arg(long)]
    health: bool,
    /// Print button names as you press them (to fill in the config), for this many seconds
    #[arg(long, value_name = "SECONDS")]
    listen: Option<u64>,
    /// Test the voice path without a microphone: transcribe this WAV file, act on it, exit
    #[arg(long, value_name = "WAV")]
    transcribe: Option<PathBuf>,
    /// Log every button/key event with its device ID and Elite name, and whether it matched
    /// PTT or the kill switch (keyboard keys included, so don't share it if you typed passwords)
    #[arg(long)]
    debug_input: bool,
    /// Download the speech models (about 800 MB) into the models folder, then exit
    #[arg(long)]
    download_models: bool,
}

fn log(s: &str) {
    println!("{}  {s}", chrono::Local::now().format("%H:%M:%S"));
}

/// Logs presses; forwards focus/integrity to the real platform.
struct DryRun(Arc<dyn Platform>);
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

fn real_platform() -> Arc<dyn Platform> {
    #[cfg(windows)]
    {
        Arc::new(astromech_input::exec::platform::win::WinPlatform)
    }
    #[cfg(not(windows))]
    {
        Arc::new(astromech_input::exec::platform::NoInputPlatform)
    }
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("\nError: {e:#}");
        if std::env::args().len() == 1 {
            eprintln!("Press Enter to close.");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
        std::process::exit(1);
    }
}

fn write_health(dir: &Path, st: &BindsState) {
    let red = astromech_keybinds::report::Redactor::from_env();
    let _ = std::fs::create_dir_all(dir);
    let mut v = serde_json::json!({ "summary": st, "doctor": st.doctor });
    red.apply_json(&mut v);
    let _ = std::fs::write(dir.join("health.json"), serde_json::to_string_pretty(&v).unwrap_or_default());
    if let Some(d) = &st.doctor {
        let _ = std::fs::write(dir.join("health.html"), red.apply(&astromech_keybinds::report::html(d)));
    }
}

fn download_models(cfg: &Config) -> Result<()> {
    use astromech_voice::{catalog, download, models};
    let base = cfg.models_dir();
    let assets = [models::STT, models::TTS];
    for a in &assets {
        let have = download::is_installed(base.parent().unwrap_or(&base), a) || models::model_root(&base, a).is_dir();
        println!("  {:>6.0} MB  {}  {}", catalog::mb(a.size), a.label, if have { "(already there)" } else { "" });
    }
    println!("Into {}. Download now? [y/N]", base.display());
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    if !s.trim().eq_ignore_ascii_case("y") {
        bail!("cancelled");
    }
    // download::ensure extracts into <base>/<asset.dest> = <exe dir>/models when base is the exe dir.
    let root = base.parent().map(Path::to_path_buf).unwrap_or_else(|| ".".into());
    for a in &assets {
        if models::model_root(&base, a).is_dir() {
            continue;
        }
        let mut last = 0u64;
        download::ensure(&root, a, &mut |p| {
            let pct = (p.done * 100).checked_div(p.total).unwrap_or(0);
            if pct != last {
                last = pct;
                print!("\r  {} {:?} {pct}%   ", a.label, p.phase);
                let _ = std::io::Write::flush(&mut std::io::stdout());
            }
        })?;
        println!();
    }
    println!("Done.");
    Ok(())
}

fn run(args: Args) -> Result<()> {
    let cfg_path = args.config.clone().unwrap_or_else(|| Config::exe_dir().join("astromech.toml"));
    if args.write_config {
        if cfg_path.exists() {
            bail!("{} already exists; not overwriting it", cfg_path.display());
        }
        std::fs::write(&cfg_path, SAMPLE)?;
        println!("Wrote {}. Edit the [controls] section, then start astromech-core.", cfg_path.display());
        return Ok(());
    }
    // --listen, --health, --download-models and --transcribe work without a PTT button.
    let exempt = args.listen.is_some() || args.health || args.download_models || args.transcribe.is_some();
    let mut cfg = if cfg_path.exists() {
        log(&format!("config: {}", cfg_path.display()));
        Config::load(&cfg_path)?
    } else {
        let alike = astromech_core::config::look_alikes(cfg_path.parent().unwrap_or(Path::new(".")));
        let mut msg = format!("config: {} not found.", cfg_path.display());
        if !alike.is_empty() {
            msg += &format!(
                " Found {} instead: Windows probably added an extension. Rename it to exactly \"astromech.toml\" \
                 (turn on View > File name extensions in Explorer to see the real name).",
                alike
                    .iter()
                    .map(|p| format!("\"{}\"", p.file_name().unwrap_or_default().to_string_lossy()))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        } else {
            msg += " Create one with --write-config.";
        }
        if !exempt {
            bail!("{msg}");
        }
        log(&format!("{msg} Using defaults for this mode."));
        Config::default()
    };
    if cfg.actions.retries.is_some() {
        log("config: [actions] retries is deprecated and ignored; astromech never presses a command twice");
    }
    if !exempt && cfg.controls.ptt.is_none() {
        bail!(
            "no push-to-talk button in {}. Add e.g. `ptt = {{ device = \"231D3201\", key = \"Joy_15\" }}` under [controls] \
             (run `astromech-core --listen 20` to see your buttons' names).",
            cfg_path.display()
        );
    }
    if args.dry_run {
        cfg.actions.dry_run = true;
    }
    if args.download_models {
        return download_models(&cfg);
    }
    if let Some(secs) = args.listen {
        return listen(secs);
    }
    println!(
        "astromech-core {} (gear-down preview). Type a command and press Enter at any time; !help for more.",
        env!("CARGO_PKG_VERSION")
    );
    let data = cfg.data_dir();
    std::fs::create_dir_all(&data).with_context(|| format!("creating {}", data.display()))?;

    // 1. Binds health check.
    let bindings = cfg.bindings_dir().context("can't find the Bindings folder; set game.bindings_dir")?;
    let live = cfg!(windows);
    let st = health::run_check(&bindings, cfg.game_dir().as_deref(), live);
    print!("{}", st.summary());
    write_health(&data, &st);
    if args.health {
        println!("Full report: {}", data.join("health.html").display());
        return Ok(());
    }
    let binds: health::Shared = Arc::new(RwLock::new(st));
    let data2 = data.clone();
    let _hw = health::watch(bindings.clone(), cfg.game_dir(), live, binds.clone(), move |s| {
        log("Bindings folder changed; health check re-run:");
        print!("{}", s.summary());
        write_health(&data2, s);
    })
    .map_err(|e| log(&format!("not watching the Bindings folder: {e:#}")))
    .ok();

    // 2. Game state pipeline.
    let hub = Arc::new(StateHub::new());
    let jdir = cfg.journal_dir().context("can't find the journal folder; set game.journal_dir")?;
    let _pipe = astromech_core::pipeline::start(&jdir, hub.clone(), Some(&data.join("astromech.sqlite")), log)
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
            cfg.voice.speaker_id,
            cfg.voice.speed,
            speech::fixed_phrases(),
        ) {
            Ok(s) => {
                log("Kokoro text-to-speech loaded");
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
                log(&format!("speech input off ({e:#}); type commands instead (models folder: {})", models.display()));
                None
            }
        }
    } else {
        None
    };

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
    let assistant = Arc::new(Assistant::new(runner.clone()));

    if let Some(wav) = &args.transcribe {
        let mut stt = stt.context("--transcribe needs the speech models and the voice worker")?;
        let (rate, samples) = astromech_voice::wav::read_mono_f32(wav)?;
        let t0 = std::time::Instant::now();
        let text = stt.transcribe(&samples, rate)?;
        log(&format!("heard {text:?} ({} ms)", t0.elapsed().as_millis()));
        let h = assistant.handle(&text, Origin::Voice);
        log(&format!("  -> {}", short(&h)));
        if let astromech_core::assistant::Handled::Ran { outcome: astromech_core::actions::Outcome::Sent(j), .. } = h {
            log(&format!("verification: {:?}", j.join()));
        }
        std::thread::sleep(Duration::from_secs(3)); // let the reply play
        return Ok(());
    }

    // 5. Buttons and microphone (Windows).
    #[cfg(windows)]
    let _voice = start_voice(&cfg, assistant.clone(), runner.clone(), stt, args.debug_input)?;
    #[cfg(not(windows))]
    {
        drop(stt);
        log("push-to-talk and the kill switch need Windows; type commands instead");
    }

    // 6. Console commands (also the way to test without a microphone).
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let t = line.trim();
        match t {
            "" => {}
            "quit" | "exit" | "!quit" => break,
            "!help" => println!("  type a voice command (e.g. 'gear down'), or: !kill, !rearm, !state, !health, quit"),
            "!kill" => {
                runner.policy.kill();
                speaker.say("Kill switch on. All actions stopped.");
            }
            "!rearm" => {
                runner.policy.rearm();
                speaker.say("Re-armed.");
            }
            "!state" => {
                let g = hub.snapshot();
                println!("  {g:?}");
            }
            "!health" => print!("{}", runner.binds.read().unwrap().summary()),
            _ => {
                let h = assistant.handle(t, Origin::AppWindow);
                log(&format!("typed {t:?} -> {}", short(&h)));
            }
        }
    }
    Ok(())
}

fn short(h: &astromech_core::assistant::Handled) -> String {
    use astromech_core::actions::Outcome;
    use astromech_core::assistant::Handled;
    match h {
        Handled::Ran { command, outcome } => format!(
            "{command}: {}",
            match outcome {
                Outcome::Sent(_) => "pressed, verifying".to_string(),
                o => format!("{o:?}"),
            }
        ),
        h => format!("{h:?}"),
    }
}

#[cfg(windows)]
fn listen(secs: u64) -> Result<()> {
    use astromech_input::listen::{tracker::Processor, win};
    println!("Press buttons for {secs} s; each line shows the device and key to put in astromech.toml.");
    let start = std::time::Instant::now();
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = win::rawinput::start(tx.clone(), start);
    let _ = win::xinput::start(tx, start);
    let mut p = Processor::new(astromech_input::ELITE_TITLE, None, true);
    for d in win::rawinput::list_devices() {
        p.add_device(d);
    }
    while start.elapsed() < Duration::from_secs(secs) {
        let evs = match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(win::Msg::Device(d)) => {
                p.add_device(d);
                continue;
            }
            Ok(win::Msg::Sample(s)) => p.process(&s),
            Err(_) => continue,
        };
        for e in evs {
            if e.down != Some(true) {
                continue;
            }
            let device = match e.kind.as_str() {
                "key" => "Keyboard".to_string(),
                "mouse" => "Mouse".to_string(),
                "xinput" => "GamePad".to_string(),
                _ => e.elite_id.clone().unwrap_or_else(|| "?".into()),
            };
            if let Some(k) = &e.elite_name {
                println!("  {{ device = \"{device}\", key = \"{k}\" }}   ({})", e.device);
            }
        }
        p.events.clear();
    }
    Ok(())
}

#[cfg(not(windows))]
fn listen(_secs: u64) -> Result<()> {
    bail!("--listen needs Windows")
}

#[cfg(windows)]
struct VoiceHandles;

#[cfg(windows)]
fn start_voice(
    cfg: &Config,
    assistant: Arc<Assistant>,
    runner: Arc<Runner>,
    stt: Option<Box<dyn Stt>>,
    debug_input: bool,
) -> Result<VoiceHandles> {
    use astromech_core::controls::{self, Control};
    let (tx, rx) = std::sync::mpsc::channel();
    let (status, devices) = controls::start(
        cfg.controls.ptt.clone(),
        cfg.controls.kill_switch.clone(),
        debug_input,
        cfg.controls.debounce_ms,
        tx,
        log,
    );
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
    let mic = if stt.is_some() {
        match astromech_voice::mic::Recorder::start() {
            Ok(m) => {
                log(&format!("microphone: {} ({} Hz)", m.device_name(), m.sample_rate()));
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
    let (jtx, jrx) = std::sync::mpsc::channel::<(Vec<f32>, u32)>();
    if let Some(mut stt) = stt {
        let a = assistant.clone();
        std::thread::Builder::new().name("stt".into()).spawn(move || {
            while let Ok((samples, rate)) = jrx.recv() {
                let t0 = std::time::Instant::now();
                match stt.transcribe(&samples, rate) {
                    Ok(t) if t.is_empty() => a.runner.speaker.say("Didn't catch that."),
                    Ok(t) => {
                        let h = a.handle(&t, Origin::Voice);
                        if matches!(h, astromech_core::assistant::Handled::Ignored) {
                            log(&format!("heard {t:?} ({} ms): noise only, ignored", t0.elapsed().as_millis()));
                        } else {
                            log(&format!("heard {t:?} ({} ms)", t0.elapsed().as_millis()));
                            log(&format!("  -> {}", short(&h)));
                        }
                    }
                    Err(e) => log(&format!("speech-to-text failed: {e:#}")),
                }
            }
        })?;
    }
    let pre = cfg.voice.pre_roll_ms;
    let post = Duration::from_millis(cfg.voice.post_roll_ms);
    let rearm_hold = Duration::from_millis(cfg.controls.rearm_hold_ms);
    let debounce = Duration::from_millis(cfg.controls.debounce_ms);
    let (min_hold, min_level) = (cfg.voice.min_hold_ms, cfg.voice.min_level_dbfs);
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
                    if from.is_none() {
                        report_mic(&mut mic);
                    }
                    continue;
                }
                Err(_) => break,
            };
            match c {
                Control::PttDown => {
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
                        let st = astromech_core::speech::clip_stats(&samples, m.sample_rate());
                        let head = format!(
                            "PTT up (t={:.3} s) — held {held} ms; clip {} ms, RMS {:.0} dBFS, loudest frame {:.0} dBFS",
                            secs(up_at),
                            st.audio_ms,
                            st.rms_dbfs,
                            st.peak_frame_dbfs
                        );
                        use astromech_core::speech::{gate, Gate};
                        match gate(held, &st, min_hold, min_level) {
                            Gate::Pass => {
                                log(&format!("{head}; transcribing"));
                                let _ = jtx.send((samples, m.sample_rate()));
                            }
                            Gate::TooShort => log(&format!("{head}; shorter than {min_hold} ms, ignored")),
                            Gate::TooQuiet => log(&format!("{head}; quieter than {min_level:.0} dBFS, ignored")),
                        }
                    } else if mic.is_none() {
                        log("PTT up — no speech-to-text or microphone, nothing to transcribe");
                    } else {
                        log("PTT up without a PTT down (ignored)");
                    }
                }
                Control::KillDown => {
                    if !runner.policy.is_killed() {
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
    Ok(VoiceHandles)
}
