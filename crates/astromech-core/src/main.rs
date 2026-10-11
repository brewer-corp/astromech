//! astromech-core: the always-running core with the tray icon (default), or the console
//! preview (`--console`, and the one-shot options such as `--listen` and `--health`).

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use anyhow::{bail, Context, Result};
use astromech_core::config::{Config, SAMPLE};
use astromech_core::logging::log;
use astromech_core::session::{self, Session};
use astromech_proto::Origin;
use clap::Parser;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "astromech core: push-to-talk ship commands for Elite Dangerous (tray app; --console for the console)"
)]
struct Args {
    /// Start in the background (tray only, no window); what "Start with Windows" uses
    #[arg(long)]
    background: bool,
    /// Open the app window at this page (e.g. keybinds, settings)
    #[arg(long, value_name = "PAGE")]
    route: Option<String>,
    /// Ask the running core to quit (cleanly: cancels actions, releases keys), wait for it, exit
    #[arg(long)]
    quit: bool,
    /// Run in this console instead of the tray: type commands, see the log
    #[arg(long)]
    console: bool,
    /// Config file (default: astromech.toml next to the exe if there is one, else
    /// %LOCALAPPDATA%\astromech\astromech.toml)
    #[arg(long)]
    config: Option<PathBuf>,
    /// Write a sample astromech.toml and exit
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

impl Args {
    fn console_mode(&self) -> bool {
        self.console
            || self.write_config
            || self.health
            || self.listen.is_some()
            || self.transcribe.is_some()
            || self.download_models
    }
    fn session_opts(&self) -> session::Opts {
        session::Opts { dry_run: self.dry_run, no_voice: self.no_voice, debug_input: self.debug_input }
    }
}

/// Release builds are GUI-subsystem (no console window for the tray app); console mode attaches
/// to the parent console (cmd, PowerShell) or opens one. Pipes passed by the parent are kept.
/// Returns true if a new console window was opened.
#[cfg(windows)]
fn attach_console() -> bool {
    use windows_sys::Win32::System::Console::{
        AllocConsole, AttachConsole, GetStdHandle, ATTACH_PARENT_PROCESS, STD_OUTPUT_HANDLE,
    };
    unsafe {
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        if !h.is_null() && h != windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            return false;
        }
        if AttachConsole(ATTACH_PARENT_PROCESS) != 0 {
            return false;
        }
        AllocConsole() != 0
    }
}

/// `--quit` (used by the installer): 0 if no core runs any more.
fn quit_running() -> i32 {
    let dir = Config::app_dir();
    let file = dir.join(astromech_core::control::FILE);
    if astromech_core::control::call(&dir, "quit", None).is_err() {
        return 0;
    }
    for _ in 0..50 {
        if !file.exists() {
            return 0;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    1
}

fn main() {
    let args = Args::parse();
    if args.quit {
        std::process::exit(quit_running());
    }
    if !args.console_mode() {
        let code = astromech_core::app::run(astromech_core::app::Opts {
            background: args.background,
            route: args.route.clone(),
            config: args.config.clone(),
            session: args.session_opts(),
        });
        std::process::exit(code);
    }
    #[cfg(windows)]
    let own_console = attach_console();
    #[cfg(not(windows))]
    let own_console = false;
    if let Err(e) = run_console(args) {
        eprintln!("\nError: {e:#}");
        if own_console {
            eprintln!("Press Enter to close.");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
        std::process::exit(1);
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

fn run_console(args: Args) -> Result<()> {
    let cfg_path = args.config.clone().unwrap_or_else(Config::default_path);
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
        let mut alike = astromech_core::config::look_alikes(cfg_path.parent().unwrap_or(Path::new(".")));
        if cfg_path.parent() != Some(Config::exe_dir().as_path()) {
            alike.extend(astromech_core::config::look_alikes(&Config::exe_dir()));
        }
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
            msg += " Create one with --write-config (or from the tray: Open config file).";
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
        "astromech-core {} (console). Type a command and press Enter at any time; !help for more.",
        env!("CARGO_PKG_VERSION")
    );
    if args.health {
        let data = cfg.data_dir();
        let bindings = cfg.bindings_dir().context("can't find the Bindings folder; set game.bindings_dir")?;
        let st = astromech_core::health::run_check(&bindings, cfg.game_dir().as_deref(), cfg!(windows));
        print!("{}", st.summary());
        session::write_health(&data, &st);
        println!("Full report: {}", data.join("health.html").display());
        return Ok(());
    }
    let opts = args.session_opts();
    let mut s = Session::start(&cfg, &opts)?;
    log(&s.voice);

    if let Some(wav) = &args.transcribe {
        let mut stt = s.stt.take().context("--transcribe needs the speech models and the voice worker")?;
        let (rate, samples) = astromech_voice::wav::read_mono_f32(wav)?;
        let t0 = std::time::Instant::now();
        let text = stt.transcribe(&samples, rate)?;
        log(&format!("heard {text:?} ({} ms)", t0.elapsed().as_millis()));
        let h = s.assistant.handle(&text, Origin::Voice);
        log(&format!("  -> {}", session::short(&h)));
        if let astromech_core::assistant::Handled::Ran { outcome: astromech_core::actions::Outcome::Sent(j), .. } = h {
            log(&format!("verification: {:?}", j.join()));
        }
        std::thread::sleep(Duration::from_secs(3)); // let the reply play
        return Ok(());
    }

    s.start_buttons(&cfg, &opts)?;
    let (runner, speaker, hub, assistant) = (s.runner.clone(), s.speaker.clone(), s.hub.clone(), s.assistant.clone());

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let t = line.trim();
        match t {
            "" => {}
            "quit" | "exit" | "!quit" => break,
            "!help" => println!("  type a voice command (e.g. 'gear down'), or: !kill, !rearm, !state, !health, quit"),
            "!kill" => {
                speaker.interrupt();
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
                log(&format!("typed {t:?} -> {}", session::short(&h)));
            }
        }
    }
    drop(s);
    Ok(())
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
