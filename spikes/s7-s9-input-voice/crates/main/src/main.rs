//! s7-s9-input-voice: spikes S7 (controller input with Elite focused) and S9 (local voice speed).
//!
//! Safety: this tool only LISTENS to input (Raw Input + XInput). It never sends key presses,
//! mouse input or controller output of any kind, never touches game files, and only reads
//! Status.json. Downloads come from the official sherpa-onnx GitHub releases and are
//! SHA-256 checked. Results stay on this PC.

#![cfg_attr(not(windows), allow(dead_code))]

mod input;
mod s7;
mod sysinfo;
mod voice;
mod worker;
#[cfg(windows)]
mod win;

use anyhow::Result;
use clap::Parser;
use s79_common::catalog;
use serde::Serialize;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "s7-s9-input-voice",
    version,
    about = "Spike S7 (do PTT/kill-switch buttons reach us while Elite is focused?) and S9 (local Parakeet STT / Kokoro TTS speed).",
    long_about = "Spike S7: learn a push-to-talk and a kill-switch button, then check with Elite in front that every press \
reaches this tool (Raw Input + XInput) and that the game still gets the buttons.\n\
Spike S9: download Parakeet (speech-to-text) and Kokoro (text-to-speech) for sherpa-onnx, then time PTT-release -> transcript \
and text -> first audio on this PC.\n\n\
The tool only listens to input; it never sends key presses. Results go to a JSON file next to the exe.",
    after_help = "MODES:\n  all    (default) S7, then S9\n  s7     input test only (needs Elite running)\n  s9     voice test only (uses the PTT learned in S7, or Space); same as --s9-only\n  bench  voice speed only, no microphone/PTT/playback (also works on Linux)\n\nExamples:\n  s7-s9-input-voice.exe --dry-run\n  s7-s9-input-voice.exe\n  s7-s9-input-voice.exe --s9-only --gpu"
)]
pub struct Args {
    /// all | s7 | s9 | bench
    #[arg(default_value = "all")]
    pub mode: String,
    /// Show what would happen (devices, downloads, paths) without testing, downloading or writing anything.
    #[arg(long)]
    pub dry_run: bool,
    /// Accept the model downloads without asking.
    #[arg(long, short = 'y')]
    pub yes: bool,
    /// Skip S7 and run only S9 (same as the `s9` mode; uses the PTT saved by an earlier S7 run, or Space).
    #[arg(long)]
    pub s9_only: bool,
    /// Also run the optional DirectML GPU test (any DirectX 12 GPU: AMD/Intel/NVIDIA; downloads ~15 MB from Microsoft's NuGet).
    #[arg(long, conflicts_with = "no_gpu")]
    pub gpu: bool,
    /// Never offer the GPU test.
    #[arg(long)]
    pub no_gpu: bool,
    /// Use the int8 Kokoro model (103 MB instead of 320 MB).
    #[arg(long)]
    pub small_tts: bool,
    /// Number of PTT presses asked for in S7.
    #[arg(long, default_value_t = 20)]
    pub ptt_count: usize,
    /// Number of kill-switch presses asked for in S7.
    #[arg(long, default_value_t = 5)]
    pub kill_count: usize,
    /// Seconds to switch to the game before the S7 test starts.
    #[arg(long, default_value_t = 10)]
    pub countdown: u64,
    /// S7: do not require Elite to be the foreground window (for trying the tool without the game).
    #[arg(long)]
    pub any_window: bool,
    /// Exact game window title to require.
    #[arg(long, default_value = "Elite - Dangerous (CLIENT)")]
    pub window_title: String,
    /// Listen with a message-only window instead of a hidden top-level window.
    #[arg(long)]
    pub message_only: bool,
    /// No cue beeps (S7 relies on them, so only for testing).
    #[arg(long)]
    pub no_sound: bool,
    /// Threads for STT/TTS on the CPU (default: 4 on 8+ logical CPUs, else 2).
    #[arg(long)]
    pub threads: Option<i32>,
    /// Kokoro voice (speaker id).
    #[arg(long, default_value_t = 0)]
    pub voice: i32,
    /// How many of the 15 sample commands to use in S9.
    #[arg(long, default_value_t = 15)]
    pub phrases: usize,
    /// Skip the CPU thread-count sweep.
    #[arg(long)]
    pub no_sweep: bool,
    /// S9 without the microphone test.
    #[arg(long)]
    pub no_mic: bool,
    /// Folder with Status.json (default: %USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous).
    #[arg(long)]
    pub journal_dir: Option<String>,
    /// Where models, runtimes, recordings and results go (default: the exe's folder).
    #[arg(long)]
    pub base: Option<PathBuf>,
    /// Path of s9-voice-worker (default: next to this exe).
    #[arg(long)]
    pub worker: Option<PathBuf>,
    /// Path of s9-gpu-probe (default: next to this exe).
    #[arg(long, hide = true)]
    pub probe: Option<PathBuf>,
    /// Do not wait for Enter before exiting.
    #[arg(long)]
    pub no_pause: bool,
    /// Also print each S7 cue as a text line (for automated testing).
    #[arg(long, hide = true)]
    pub print_cues: bool,
}

pub struct ConsoleUi;

impl ConsoleUi {
    pub fn out_line(&self, s: &str) {
        println!("{s}");
    }
    pub fn ask_line(&self, q: &str) -> String {
        #[cfg(windows)]
        win::platform::flush_console_input();
        print!("{q}");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        match std::io::stdin().lock().read_line(&mut line) {
            Ok(0) | Err(_) => {
                println!();
                String::new()
            }
            Ok(_) => line.trim().to_lowercase(),
        }
    }
}

impl voice::Ui for ConsoleUi {
    fn out(&self, s: &str) {
        self.out_line(s)
    }
    fn ask(&self, q: &str) -> String {
        self.ask_line(q)
    }
}

#[derive(Serialize, Default)]
struct Results {
    tool: String,
    version: String,
    started: String,
    finished: Option<String>,
    mode: String,
    args: Vec<String>,
    system: sysinfo::SystemInfo,
    raw_input_window: Option<String>,
    raw_input_registration: Option<String>,
    xinput_slots_connected: Vec<u32>,
    integrity: Option<String>,
    devices: Vec<input::DeviceInfo>,
    s7: Vec<s7::S7Run>,
    s9: Option<voice::S9Results>,
    notes: Vec<String>,
}

fn now_local() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

fn save_json(path: &Path, r: &Results) {
    match serde_json::to_string_pretty(r) {
        Ok(s) => {
            if let Err(e) = std::fs::write(path, s) {
                eprintln!("  (could not write {}: {e})", path.display());
            }
        }
        Err(e) => eprintln!("  (could not serialise results: {e})"),
    }
}

fn base_dir(args: &Args) -> PathBuf {
    if let Some(b) = &args.base {
        return std::path::absolute(b).unwrap_or_else(|_| b.clone());
    }
    std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())).unwrap_or_else(|| PathBuf::from("."))
}

fn tts_asset(args: &Args) -> catalog::Asset {
    if args.small_tts {
        catalog::TTS_KOKORO_EN_INT8
    } else {
        catalog::TTS_KOKORO_EN
    }
}

/// A helper exe: the given path, else next to this exe (never the current directory).
fn helper_path(given: &Option<PathBuf>, name: &str, base: &Path) -> PathBuf {
    given.clone().map(|w| std::path::absolute(&w).unwrap_or(w)).unwrap_or_else(|| {
        std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.join(name))).unwrap_or_else(|| base.join(name))
    })
}

fn worker_src(args: &Args, base: &Path) -> PathBuf {
    helper_path(&args.worker, worker::WORKER_EXE, base)
}

fn probe_src(args: &Args, base: &Path) -> PathBuf {
    helper_path(&args.probe, worker::PROBE_EXE, base)
}

fn banner() {
    println!("s7-s9-input-voice {} - Elite voice assistant spikes S7 + S9", env!("CARGO_PKG_VERSION"));
    println!("This tool only listens to input. It never presses keys or buttons for you.\n");
}

fn dry_run(args: &Args, base: &Path) -> Result<i32> {
    println!("DRY RUN: nothing is downloaded, tested or written.\n");
    let si = sysinfo::collect();
    for l in si.lines() {
        println!("  {l}");
    }
    println!("\n  Folder for models/results: {}", base.display());
    warn_temp(base);
    let w = worker_src(args, base);
    let rt = catalog::cpu_runtime();
    match (&rt, voice::runtime_lib_dir(base, rt.as_ref())) {
        (Some(_), Err(_)) => println!("  Voice worker: {} ({})", w.display(), if w.is_file() { "found; runtime not downloaded yet" } else { "MISSING - keep it next to the main exe" }),
        (_, lib) => match worker::preflight(&w, lib.ok().flatten().as_deref()) {
            Ok(v) => println!("  Voice worker: {} - starts OK ({v})", w.display()),
            Err(e) => println!("  Voice worker: PROBLEM - {e:#}"),
        },
    }
    let p = probe_src(args, base);
    println!("  GPU test helper: {} ({})", p.display(), if p.is_file() { "found" } else { "MISSING - only needed for --gpu" });
    println!("\n  Downloads needed for S9:");
    let mut total = 0;
    let mut assets: Vec<catalog::Asset> = rt.into_iter().collect();
    assets.extend([catalog::STT_PARAKEET_V2_INT8, tts_asset(args)]);
    let gpu_mb: f64 = catalog::gpu_runtime().iter().filter(|a| !s79_common::download::is_installed(base, a)).map(|a| catalog::mb(a.size)).sum();
    if args.gpu {
        assets.extend(catalog::gpu_runtime());
    }
    for a in &assets {
        let have = s79_common::download::is_installed(base, a);
        if !have {
            total += a.size;
        }
        println!("    {:>7.1} MB  {:<60} {}", catalog::mb(a.size), a.label, if have { "already installed" } else { "to download" });
        println!("              {}", a.url());
    }
    println!("    {:>7.1} MB  still to download{}", catalog::mb(total), if args.gpu || gpu_mb == 0.0 { String::new() } else { format!(" (+{gpu_mb:.0} MB if you opt into the DirectML GPU test)") });
    platform_dry_run(args);
    println!("\nDry run finished.");
    Ok(0)
}

#[cfg(windows)]
fn platform_dry_run(args: &Args) {
    println!("\n  Input devices (Raw Input):");
    for d in win::rawinput::list_devices() {
        println!("    - {}", d.describe());
    }
    let slots = win::xinput::connected_slots();
    println!("  XInput controllers connected: {}", if slots.is_empty() { "none".to_string() } else { format!("{slots:?}") });
    let (tx, _rx) = std::sync::mpsc::channel();
    let l = win::rawinput::start(tx, args.message_only);
    println!(
        "  Raw Input registration ({} window, RIDEV_INPUTSINK): {}",
        if l.message_only { "message-only" } else { "hidden top-level" },
        match &l.registered {
            Ok(()) => "OK".to_string(),
            Err(e) => format!("FAILED: {e}"),
        }
    );
    println!("  Microphone: {}", win::audio::Recorder::default_device_name().unwrap_or_else(|| "none found".into()));
    println!("  Elite: {}", win::platform::integrity_note(&args.window_title));
    let sp = win::platform::status_json_path(args.journal_dir.as_deref());
    println!("  Status.json: {} ({})", sp.display(), if sp.is_file() { "found" } else { "not found" });
}

#[cfg(not(windows))]
fn platform_dry_run(_args: &Args) {
    println!("\n  (Not Windows: S7 and the microphone/playback parts of S9 are unavailable; 'bench' mode works.)");
}

fn warn_temp(base: &Path) {
    let s = base.display().to_string().to_lowercase();
    if s.contains("\\appdata\\local\\temp") || s.contains("\\temp\\") && s.contains(".zip") {
        println!("\n  WARNING: this looks like a temporary folder (running from inside the zip?).");
        println!("  Please extract the zip to a normal folder first, e.g. C:\\EliteSpikes\\s7-s9, then run it from there.\n");
    }
}

fn main() {
    let args = Args::parse();
    let code = match real_main(args.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\nERROR: {e:#}");
            1
        }
    };
    if !args.no_pause && !args.dry_run && cfg!(windows) {
        ConsoleUi.ask_line("\nPress Enter to close this window...");
    }
    std::process::exit(code);
}

fn real_main(args: Args) -> Result<i32> {
    banner();
    let base = base_dir(&args);
    let mode = if args.s9_only { "s9".to_string() } else { args.mode.to_lowercase() };
    if !["all", "s7", "s9", "bench"].contains(&mode.as_str()) {
        anyhow::bail!("unknown mode '{}': use all, s7, s9 or bench (see --help)", args.mode);
    }
    if args.dry_run {
        return dry_run(&args, &base);
    }
    warn_temp(&base);
    std::fs::create_dir_all(&base)?;
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    let results_path = base.join(format!("s7-s9-results-{stamp}.json"));
    let mut res = Results {
        tool: "s7-s9-input-voice".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        started: now_local(),
        mode: mode.clone(),
        args: std::env::args().skip(1).collect(),
        system: sysinfo::collect(),
        ..Default::default()
    };
    println!("System:");
    for l in res.system.lines() {
        println!("  {l}");
    }
    println!("Results file: {}\n", results_path.display());
    save_json(&results_path, &res);
    let r = run_modes(&args, &base, &mode, &stamp, &mut res, &results_path);
    if let Err(e) = &r {
        res.notes.push(format!("stopped with error: {e:#}"));
    }
    res.finished = Some(now_local());
    save_json(&results_path, &res);
    print_final(&res, &results_path);
    r.map(|_| 0)
}

fn print_final(res: &Results, path: &Path) {
    println!("\n================ SUMMARY ================");
    for (i, run) in res.s7.iter().enumerate() {
        println!("S7 run {} ({}):", i + 1, run.bindings.as_ref().map(|b| b.ptt.primary.device.describe()).unwrap_or_else(|| "no bindings".into()));
        for p in [&run.ptt, &run.kill].into_iter().flatten() {
            println!(
                "  {:<4} {}/{} received, {} missed{}{}",
                if p.kind == "ptt" { "PTT" } else { "KILL" },
                p.pressed,
                p.asked,
                p.missed,
                p.press_to_callback_ms.as_ref().map(|s| format!(", press->callback median {:.1} ms (max {:.1})", s.median, s.max)).unwrap_or_default(),
                p.within_50ms.map(|w| format!(", {w} within 50 ms")).unwrap_or_default()
            );
        }
        if let Some(a) = &run.aborted {
            println!("  aborted: {a}");
        }
        for (q, a) in &run.answers {
            println!("  {q} -> {a}");
        }
    }
    if let Some(s9) = &res.s9 {
        println!("S9: {} / {} (sherpa-onnx {})", s9.stt_model, s9.tts_model, s9.sherpa_onnx_version.clone().unwrap_or_default());
        if let Some(e) = &s9.synthetic_exact {
            println!("  synthetic TTS->STT: {e}");
        }
        if let (Some(e), Some(s)) = (&s9.mic_exact, &s9.release_to_transcript_ms) {
            println!("  mic: {e}; PTT release -> transcript median {:.0} ms, p95 {:.0} ms", s.median, s.p95);
        }
        if let (Some(f), Some(r)) = (&s9.tts_first_audio_ms, &s9.tts_rtf) {
            println!("  TTS: first audio median {:.0} ms (max {:.0}), RTF median {:.3}", f.median, f.max, r.median);
        }
        for p in &s9.providers {
            println!("  {}", voice::provider_line(p));
        }
        for l in &s9.gpu_summary {
            println!("  GPU: {l}");
        }
        if let Some(e) = &s9.error {
            println!("  error: {e}");
        }
    }
    println!("\nAll details: {}", path.display());
    println!("Please send that JSON file back (it contains no audio; recordings stay in the s9-audio folder).");
}

#[cfg(not(windows))]
fn run_modes(args: &Args, base: &Path, mode: &str, stamp: &str, res: &mut Results, path: &Path) -> Result<()> {
    if mode == "s7" {
        anyhow::bail!("S7 needs Windows (Raw Input/XInput).");
    }
    if mode != "bench" {
        println!("Not Windows: running 'bench' (no S7, no microphone, no playback).");
        res.notes.push("non-Windows: bench mode only".into());
    }
    run_s9(args, base, stamp, res, path, None, None, None)
}

#[allow(clippy::too_many_arguments)]
fn run_s9(
    args: &Args,
    base: &Path,
    stamp: &str,
    res: &mut Results,
    path: &Path,
    mic: Option<&dyn voice::Mic>,
    ptt: Option<&dyn voice::PttSource>,
    player: Option<&dyn voice::Player>,
) -> Result<()> {
    println!("\n================ S9: local voice speed ================");
    let si = &res.system;
    let cfg = voice::S9Config {
        base: base.to_path_buf(),
        worker_src: worker_src(args, base),
        probe_src: probe_src(args, base),
        threads: args.threads.unwrap_or_else(voice::default_threads),
        sid: args.voice,
        speed: 1.0,
        phrases: args.phrases.clamp(1, s79_common::phrases::COMMANDS.len()),
        tts_asset: tts_asset(args),
        gpu: if args.gpu { Some(true) } else if args.no_gpu || args.yes { Some(false) } else { None },
        gpu_present: si.has_gpu(),
        sweep: !args.no_sweep,
        yes: args.yes,
        audio_dir: base.join("s9-audio").join(stamp),
        log_dir: base.join("logs"),
    };
    let ui = ConsoleUi;
    let io = voice::S9Io { ui: &ui, mic, ptt, player };
    let mut s9 = voice::S9Results::default();
    let snapshot = |r: &Results, s: &voice::S9Results| {
        let mut tmp = Results { s9: Some(s.clone()), ..clone_results(r) };
        tmp.finished = None;
        save_json(path, &tmp);
    };
    let out = voice::run_s9(&cfg, &io, &mut s9, &mut |s| snapshot(res, s));
    if let Err(e) = &out {
        s9.error = Some(format!("{e:#}"));
    }
    res.s9 = Some(s9);
    save_json(path, res);
    out
}

fn clone_results(r: &Results) -> Results {
    // Results holds only plain data; round-trip through JSON-free manual clone.
    Results {
        tool: r.tool.clone(),
        version: r.version.clone(),
        started: r.started.clone(),
        finished: r.finished.clone(),
        mode: r.mode.clone(),
        args: r.args.clone(),
        system: r.system.clone(),
        raw_input_window: r.raw_input_window.clone(),
        raw_input_registration: r.raw_input_registration.clone(),
        xinput_slots_connected: r.xinput_slots_connected.clone(),
        integrity: r.integrity.clone(),
        devices: r.devices.clone(),
        s7: r.s7.clone(),
        s9: r.s9.clone(),
        notes: r.notes.clone(),
    }
}

#[cfg(windows)]
fn run_modes(args: &Args, base: &Path, mode: &str, stamp: &str, res: &mut Results, path: &Path) -> Result<()> {
    use std::sync::mpsc;
    unsafe { windows_sys::Win32::Media::timeBeginPeriod(1) };
    let (tx, rx) = mpsc::channel();
    let listener = win::rawinput::start(tx.clone(), args.message_only);
    res.raw_input_window = Some(if listener.message_only { "message-only (HWND_MESSAGE)".into() } else { "hidden top-level window".into() });
    res.raw_input_registration = Some(match &listener.registered {
        Ok(()) => "OK".into(),
        Err(e) => e.clone(),
    });
    if let Err(e) = &listener.registered {
        anyhow::bail!("could not register for Raw Input: {e}");
    }
    win::xinput::start(tx);
    res.xinput_slots_connected = win::xinput::connected_slots();
    res.devices = win::rawinput::list_devices();
    let env = win::platform::WinEnv { rx, ui: ConsoleUi, status_path: win::platform::status_json_path(args.journal_dir.as_deref()), sound: !args.no_sound, print_cues: args.print_cues };
    let bindings_path = base.join("s7-bindings.json");
    let mut ptt_binding: Option<input::Binding> = std::fs::read_to_string(&bindings_path).ok().and_then(|t| serde_json::from_str::<input::Bindings>(&t).ok()).map(|b| b.ptt);

    if mode == "all" || mode == "s7" {
        let do_s7 = mode == "s7" || s7::ask_yn(&env, "Run S7 (input test; Elite should be running, in your ship) now?", true);
        if do_s7 {
            if let Some(b) = s7_flow(args, &env, res, path, &bindings_path)? {
                ptt_binding = Some(b.ptt);
            }
        } else {
            res.notes.push("S7 skipped by user".into());
        }
    }
    if mode == "all" || mode == "s9" || mode == "bench" {
        if mode == "all" && !s7::ask_yn(&env, "\nRun S9 (voice speed; does not need the game) now?", true) {
            res.notes.push("S9 skipped by user".into());
            return Ok(());
        }
        let bench = mode == "bench";
        let mic = if bench || args.no_mic {
            None
        } else {
            match win::audio::Recorder::start() {
                Ok(r) => {
                    res.notes.push(format!("microphone: {} ({} ch, {} Hz, {})", voice::Mic::device_name(&r), r.channels, voice::Mic::sample_rate(&r), r.format));
                    Some(r)
                }
                Err(e) => {
                    println!("  Microphone unavailable ({e:#}); the mic test will be skipped.");
                    res.notes.push(format!("microphone unavailable: {e:#}"));
                    None
                }
            }
        };
        let ptt = win::platform::WinPtt { env: &env, binding: ptt_binding };
        let player = win::sound::WinPlayer;
        run_s9(
            args,
            base,
            stamp,
            res,
            path,
            mic.as_ref().map(|m| m as &dyn voice::Mic),
            if mic.is_some() { Some(&ptt as &dyn voice::PttSource) } else { None },
            if bench { None } else { Some(&player as &dyn voice::Player) },
        )?;
    }
    Ok(())
}

#[cfg(windows)]
fn s7_flow(args: &Args, env: &win::platform::WinEnv, res: &mut Results, path: &Path, bindings_path: &Path) -> Result<Option<input::Bindings>> {
    use s7::Env;
    let cfg = s7::S7Config {
        ptt_count: args.ptt_count,
        kill_count: args.kill_count,
        countdown_secs: args.countdown,
        require_game: !args.any_window,
        game_title: args.window_title.clone(),
        ..Default::default()
    };
    println!("================ S7: controller input with Elite focused ================");
    println!("Devices Windows reports:");
    for d in &res.devices {
        println!("  - {}", d.describe());
    }
    println!("XInput (Xbox-compatible) pads connected: {}", if res.xinput_slots_connected.is_empty() { "none".to_string() } else { format!("{:?}", res.xinput_slots_connected) });
    println!(
        "Note for Xbox pads: Windows reports them through XInput AND as a HID gamepad. This tool listens to both and\n\
         records which one(s) see your presses (HID merges the triggers and has no Guide button).\n"
    );
    let integ = win::platform::integrity_note(&cfg.game_title);
    println!("Integrity levels: {integ}");
    res.integrity = Some(integ);
    if cfg.require_game && env.foreground() != cfg.game_title && win::platform::integrity_note(&cfg.game_title).contains("not found") {
        println!("Elite does not seem to be running (no window titled '{}'). Start it and get into your ship first.", cfg.game_title);
    }
    let mut last: Option<input::Bindings> = None;
    loop {
        let mut run = s7::S7Run { bindings: None, learn: vec![], trials: vec![], ptt: None, kill: None, receive_check: None, aborted: None, answers: vec![] };
        println!("\n--- Learn mode ---");
        let ptt = learn_confirm(env, &cfg, "PUSH-TO-TALK (PTT)", None, &mut run);
        let kill = match &ptt {
            Some(p) => learn_confirm(env, &cfg, "KILL SWITCH", Some(p), &mut run),
            None => None,
        };
        if let (Some(ptt), Some(kill)) = (ptt, kill) {
            let b = input::Bindings { ptt, kill };
            let _ = std::fs::write(bindings_path, serde_json::to_string_pretty(&b)?);
            run.bindings = Some(b.clone());
            println!("\n--- Test mode ---");
            s7::run_test(env, &cfg, &b, &mut run);
            println!("\nBack in the console. Results:");
            s7::print_summary(env, &run);
            let qs = [
                ("pressed_every_cue", "Did you press for every beep (so any 'missed' are real misses)? [y/n] "),
                ("game_reacted_in_last_part", "In the last part, did Elite react to each of your presses? [y/n/s=skipped] "),
                ("bound_in_game_still_worked", "Are your PTT/kill buttons also bound in Elite? If yes, did they still work in-game during the test? [y/n/na] "),
                ("comment", "Anything odd you noticed? (Enter to skip) "),
            ];
            for (k, q) in qs {
                let a = env.ask(q);
                run.answers.push((k.into(), a));
            }
            last = Some(b);
        } else {
            run.aborted = Some("bindings not learned".into());
        }
        res.s7.push(run);
        save_json(path, res);
        if !s7::ask_yn(env, "\nTest another device?", false) {
            break;
        }
    }
    Ok(last)
}

#[cfg(windows)]
fn learn_confirm(env: &win::platform::WinEnv, cfg: &s7::S7Config, what: &str, exclude: Option<&input::Binding>, run: &mut s7::S7Run) -> Option<input::Binding> {
    use s7::Env;
    for _ in 0..5 {
        println!("\n{what}:");
        let rec = s7::learn(env, cfg, what, exclude);
        let b = rec.binding.clone();
        match &b {
            Some(b) => {
                println!("  Detected: {}", b.primary.describe());
                if let Some(a) = &b.alternate {
                    println!("  Also seen through: {}", a.describe());
                }
                if rec.injected {
                    println!("  WARNING: this press came from software (no device handle), e.g. a macro tool or remapper.");
                }
                if !rec.released_seen {
                    println!("  (no release seen within 5 s)");
                }
            }
            None => println!("  {}", rec.note.clone().unwrap_or_default()),
        }
        run.learn.push(rec);
        b.as_ref()?;
        let a = env.ask("  Use this button? [Y = yes / n = try again / s = skip] ");
        match a.as_str() {
            "" | "y" | "yes" => return b,
            "s" | "skip" => return None,
            _ => continue,
        }
    }
    None
}
