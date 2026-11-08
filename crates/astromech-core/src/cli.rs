//! `astromech-core.exe`'s command line: the app (the default, with `--background` from the Run
//! key), the console preview (`--console`, and the one-shot options such as `--listen` and
//! `--health`), world mode (`--world`, headless against a scripted world), `--quit` and the
//! update helper (`--apply-update`). The binary is built by app/src-tauri, whose `main` is
//! [`main`] with the app it runs: the Tauri host with the tray and the window, or
//! [`crate::app::run`] headless without its `window` feature (decision astromech-nade). Console
//! mode, world mode and `--apply-update` run before any of the app's setup.

use crate as astromech_core;
use anyhow::{bail, Context, Result};
use astromech_api::Recover;
use astromech_core::config::{Config, SAMPLE};
use astromech_core::logging::log;
use astromech_core::session::{self, Session};
use astromech_proto::{Origin, SystemClock};
use astromech_store::Paths;
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
    /// List recording devices and exit
    #[arg(long)]
    list_mics: bool,
    /// Calibrate the chosen mic from WAV files: first is silence, rest are speech phrases
    #[arg(long, value_name = "WAV", num_args = 2..)]
    calibrate_files: Option<Vec<PathBuf>>,
    /// Score Debug-saved clips in this folder (JSON sidecars + optional labels.json)
    #[arg(long, value_name = "DIR")]
    eval: Option<PathBuf>,
    /// Internal: copy a staged update into the install dir (spawned from %TEMP%)
    #[arg(long)]
    apply_update: bool,
    /// Payload directory for --apply-update
    #[arg(long)]
    from: Option<PathBuf>,
    /// Program folder for --apply-update
    #[arg(long)]
    install_dir: Option<PathBuf>,
    /// Wait for this pid to exit before copying
    #[arg(long)]
    wait_pid: Option<u32>,
    /// Start astromech-core again after copying (with --route: open the window at that page)
    #[arg(long)]
    relaunch: bool,
    /// Open WebView2's DevTools port (127.0.0.1) on the window, for CI's onboarding smoke test
    /// (scripts/onboarding_smoke.py). A flag, not an environment variable, so it can't linger
    /// in a user's environment
    #[arg(long, value_name = "PORT")]
    webview_debug_port: Option<u16>,
    /// Run headless against the scripted world in this folder (tests and the dev bridge): a temp
    /// copy, fake processes, opener, clipboard and Start with Windows, no window or tray. Prints
    /// the control API's port and token on stdout, then runs until `quit` or its stdin closes.
    /// It takes no other option but `--record-net`, `--dry-run`, `--no-voice` and `--debug-input`
    #[arg(long, value_name = "DIR", conflicts_with_all = WORLD_CONFLICTS)]
    world: Option<PathBuf>,
    /// With --world: send the lookups and EDSM's reads to the real services, and write their
    /// answers to this file in the world's net.json form, with keys and commander names redacted
    #[arg(long, value_name = "FILE", requires = "world")]
    record_net: Option<PathBuf>,
}

/// Every option `--world` can't be given with: those that act on the user's real core or files
/// (`--quit`, `--apply-update` and its options, `--config`), the installed app's own
/// (`--background`, `--route`, `--webview-debug-port`) and the console's, which `--world` would
/// otherwise ignore.
const WORLD_CONFLICTS: [&str; 19] = [
    "quit",
    "apply_update",
    "from",
    "install_dir",
    "wait_pid",
    "relaunch",
    "config",
    "background",
    "route",
    "webview_debug_port",
    "console",
    "write_config",
    "health",
    "listen",
    "transcribe",
    "download_models",
    "list_mics",
    "calibrate_files",
    "eval",
];

impl Args {
    fn console_mode(&self) -> bool {
        self.console
            || self.write_config
            || self.health
            || self.listen.is_some()
            || self.transcribe.is_some()
            || self.download_models
            || self.list_mics
            || self.calibrate_files.is_some()
            || self.eval.is_some()
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

/// `--quit` (used by the installer): 0 if no core runs any more. Real time: a console-only
/// path, which world mode doesn't run.
#[allow(clippy::disallowed_methods)]
fn quit_running() -> i32 {
    let dir = Paths::current().app_dir().to_path_buf();
    let file = dir.join(astromech_api::control::FILE);
    if astromech_api::control::call_short(&dir, "quit", None).is_err() {
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

/// Parse the command line and do what it asks; `app` runs the app itself. Never returns.
pub fn main(app: fn(astromech_core::app::Opts) -> i32) -> ! {
    let args = Args::parse();
    if args.apply_update {
        let from = args.from.expect("--apply-update needs --from");
        let install = args.install_dir.expect("--apply-update needs --install-dir");
        use astromech_core::update::After;
        let after = match (args.relaunch, args.route.is_some()) {
            (false, _) => After::Exit,
            (true, false) => After::Relaunch,
            (true, true) => After::RelaunchWithWindow,
        };
        let paths = Paths::current();
        let app_dir = paths.app_dir().to_path_buf();
        astromech_core::logging::init(&paths.log_dir(), false);
        std::process::exit(astromech_core::update::apply_cli(&from, &install, args.wait_pid, after, &app_dir));
    }
    if args.quit {
        std::process::exit(quit_running());
    }
    if let Some(world) = &args.world {
        std::process::exit(astromech_core::world::run(world, args.session_opts(), args.record_net.as_deref()));
    }
    if !args.console_mode() {
        let code = app(astromech_core::app::Opts {
            background: args.background,
            route: args.route.clone(),
            config: args.config.clone(),
            session: args.session_opts(),
            webview_debug_port: args.webview_debug_port,
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
    std::process::exit(0);
}

fn download_models(cfg: &Config, paths: &Paths) -> Result<()> {
    use astromech_voice::{catalog, models};
    let base = cfg.models_dir(paths);
    let assets = models::missing(&base);
    if assets.is_empty() {
        println!("The speech models are already in {}.", base.display());
    } else {
        for a in &assets {
            println!("  {:>6.0} MB  {}", catalog::mb(a.size), a.label);
        }
        println!("Into {}. Download now? [y/N]", base.display());
        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;
        if !s.trim().eq_ignore_ascii_case("y") {
            bail!("cancelled");
        }
    }
    for a in &assets {
        let mut last = 0u64;
        models::download(&base, a, &mut |p| {
            let pct = (p.done * 100).checked_div(p.total).unwrap_or(0);
            if pct != last {
                last = pct;
                print!("\r  {} {:?} {pct}%   ", a.label, p.phase);
                let _ = std::io::Write::flush(&mut std::io::stdout());
            }
        })?;
        println!();
    }
    for d in astromech_core::app::remove_retired_models(cfg, paths) {
        println!("Removed the replaced model {}", d.display());
    }
    println!("Done.");
    Ok(())
}

/// Real time: a console-only path, which world mode doesn't run.
#[allow(clippy::disallowed_methods)]
fn run_console(args: Args) -> Result<()> {
    let paths = Paths::current();
    let cfg_path = args.config.clone().unwrap_or_else(|| paths.config_file());
    if args.write_config {
        if cfg_path.exists() {
            bail!("{} already exists; not overwriting it", cfg_path.display());
        }
        std::fs::write(&cfg_path, SAMPLE)?;
        println!("Wrote {}. Edit the [controls] section, then start astromech-core.", cfg_path.display());
        return Ok(());
    }
    // --listen, --health, --download-models and --transcribe work without a PTT button.
    let exempt = args.listen.is_some()
        || args.health
        || args.download_models
        || args.transcribe.is_some()
        || args.list_mics
        || args.calibrate_files.is_some()
        || args.eval.is_some();
    let mut cfg = if cfg_path.exists() {
        log(&format!("config: {}", cfg_path.display()));
        let (c, problems) = Config::load_report(&cfg_path)?;
        for p in &problems {
            log(&format!("config: ignoring {p}; using the default for it"));
        }
        c
    } else {
        let mut alike = astromech_core::config::look_alikes(cfg_path.parent().unwrap_or(Path::new(".")));
        let exe_dir = paths.exe_dir().to_path_buf();
        if cfg_path.parent() != Some(exe_dir.as_path()) {
            alike.extend(astromech_core::config::look_alikes(&exe_dir));
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
        return download_models(&cfg, &paths);
    }
    if args.list_mics {
        return list_mics();
    }
    if let Some(files) = args.calibrate_files {
        return calibrate_files(&cfg, &cfg.data_dir(&paths), &files);
    }
    if let Some(dir) = args.eval {
        let run = astromech_core::eval::run(&dir)?;
        println!("{}\n", run.report.summary());
        println!("{}", run.table("Family", |c| c.family.clone()));
        for row in &run.report.rows {
            println!("  {}  {:?}  label={} got={}", row.id, row.kind, row.label, row.got);
        }
        return Ok(());
    }
    if let Some(secs) = args.listen {
        return listen(secs);
    }
    println!(
        "astromech-core {} (console). Type a command and press Enter at any time; !help for more.",
        env!("CARGO_PKG_VERSION")
    );
    if args.health {
        let data = cfg.data_dir(&paths);
        let bindings = cfg.bindings_dir().context("can't find the Bindings folder; set game.bindings_dir")?;
        let st = astromech_core::health::run_check(&bindings, cfg.game_dir().as_deref(), cfg!(windows), &SystemClock);
        print!("{}", st.summary());
        session::write_health(&data, &st);
        println!("Full report: {}", data.join("health.html").display());
        return Ok(());
    }
    let opts = args.session_opts();
    astromech_core::pipeline::EDDN_SHARING.store(cfg.eddn.enabled, std::sync::atomic::Ordering::SeqCst);
    let data = cfg.data_dir(&paths);
    astromech_core::app::apply_edsm_switch(&cfg, &data, &astromech_store::secrets::OsKeyring::new(), &SystemClock);
    let route =
        std::sync::Arc::new(astromech_core::route::Desk::in_dir(&data, Box::new(astromech_core::clipboard::set_text)));
    // The command layer's state, for its one remote-data client and star map; the folders are
    // this run's config, fixed.
    let preload = std::env::current_exe().ok().and_then(|exe| astromech_api::preload_beside(&exe));
    let (journal, bindings, game) = (cfg.journal_dir(&paths).unwrap_or_default(), cfg.bindings_dir(), cfg.game_dir());
    let folders =
        move || astromech_api::Folders { journal: journal.clone(), bindings: bindings.clone(), game: game.clone() };
    let remote = astromech_core::app::remote_client(&data);
    let api = astromech_api::State::new(data, remote, preload, folders);
    let mut s = Session::start(
        &cfg,
        &paths,
        &opts,
        route,
        &api,
        astromech_core::callouts::platform_notifier(),
        std::sync::Arc::new(SystemClock),
        None,
        &astromech_core::app::Http::real(),
        &session::Io::real(),
    )?;
    log(&s.voice);

    if let Some(wav) = &args.transcribe {
        let mut stt = s.stt.take().context("--transcribe needs the speech models and the voice worker")?;
        let (rate, samples) = astromech_voice::wav::read_mono_f32(wav)?;
        let n = astromech_voice::gain::normalise(&samples, rate);
        log(&format!("clip gain {:+.0} dB (speech peak {:.0} dBFS)", n.gain_db, n.speech_peak_dbfs));
        let t0 = std::time::Instant::now();
        let text = stt.transcribe(&n.samples, rate)?;
        log(&format!("heard {text:?} ({} ms)", t0.elapsed().as_millis()));
        let h = s.assistant.handle(&text, Origin::Voice);
        log(&format!("  -> {}", session::short(&h)));
        if let astromech_core::assistant::Handled::Ran { outcome: astromech_core::actions::Outcome::Sent(j), .. } = h {
            log(&format!("verification: {:?}", j.join()));
        }
        std::thread::sleep(Duration::from_secs(3)); // let the reply play
        return Ok(());
    }

    s.start_buttons(&cfg, &paths, &opts)?;
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
            "!health" => print!("{}", runner.binds.read().recover().summary()),
            _ => {
                let h = assistant.handle(t, Origin::AppWindow);
                log(&format!("typed {t:?} -> {}", session::short(&h)));
            }
        }
    }
    drop(s);
    Ok(())
}

fn list_mics() -> Result<()> {
    #[cfg(windows)]
    {
        let names = astromech_voice::mic::Recorder::list_names();
        if names.is_empty() {
            println!("No recording devices found.");
        }
        for n in names {
            println!("  {n}");
        }
        if let Some(d) = astromech_voice::mic::Recorder::default_device_name() {
            println!("default: {d}");
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        bail!("--list-mics needs Windows")
    }
}

fn calibrate_files(cfg: &Config, data: &Path, files: &[PathBuf]) -> Result<()> {
    use astromech_core::calibration;
    let (rate, noise, speech) = calibration::read_wavs(files)?;
    let device = calibration::configured_device(&cfg.voice.mic);
    let c = calibration::calibrate_and_store(&calibration::open_store(data)?, &device, rate, &noise, &speech)?;
    println!(
        "calibration: noise {:.0} dBFS, speech peak {:.0} dBFS, min_level_dbfs {:.0}, suggested gain {:+.0} dB",
        c.noise_dbfs, c.speech_peak_dbfs, c.min_level_dbfs, c.gain_db
    );
    println!("stored under {} for mic {device}", data.join("astromech.sqlite").display());
    println!("paste into astromech.toml: min_level_dbfs = {:.1}", c.min_level_dbfs);
    Ok(())
}

/// Real time: a console-only path, which world mode doesn't run.
#[cfg(windows)]
#[allow(clippy::disallowed_methods)]
fn listen(secs: u64) -> Result<()> {
    use astromech_input::listen::{source, tracker::Processor, win};
    println!("Press buttons for {secs} s; each line shows the device and key to put in astromech.toml.");
    let started = std::time::Instant::now();
    let rx = source::subscribe().rx;
    let mut p = Processor::new(astromech_input::ELITE_TITLE, None, true);
    for d in win::rawinput::list_devices() {
        p.add_device(d);
    }
    while started.elapsed() < Duration::from_secs(secs) {
        let evs = match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(source::Msg::Device(d)) => {
                p.add_device(d);
                continue;
            }
            Ok(source::Msg::Sample(s)) => p.process(&s),
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

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;

    /// `--world` runs only on its own (with the session's test options): with an option that
    /// acts on the user's real core (`--quit` reads the real app folder) or one it would ignore,
    /// parsing fails before anything runs.
    #[test]
    fn world_mode_refuses_every_other_option() {
        let with = |extra: &[&str]| Args::try_parse_from([&["astromech-core", "--world", "w"], extra].concat());
        for extra in [
            &["--quit"][..],
            &["--apply-update"],
            &["--from", "p"],
            &["--install-dir", "p"],
            &["--wait-pid", "1"],
            &["--relaunch"],
            &["--config", "c.toml"],
            &["--background"],
            &["--route", "settings"],
            &["--webview-debug-port", "9222"],
            &["--console"],
            &["--write-config"],
            &["--health"],
            &["--listen", "5"],
            &["--transcribe", "a.wav"],
            &["--download-models"],
            &["--list-mics"],
            &["--calibrate-files", "a.wav", "b.wav"],
            &["--eval", "d"],
        ] {
            let e = with(extra).map(|_| ()).unwrap_err();
            assert_eq!(e.kind(), clap::error::ErrorKind::ArgumentConflict, "--world {extra:?}: {e}");
        }
        for extra in [&[][..], &["--dry-run", "--no-voice", "--debug-input"], &["--record-net", "net.json"]] {
            let a = with(extra).unwrap();
            assert_eq!(a.world.as_deref(), Some(Path::new("w")), "{extra:?}");
        }
        let e = Args::try_parse_from(["astromech-core", "--record-net", "net.json"]).map(|_| ()).unwrap_err();
        assert_eq!(e.kind(), clap::error::ErrorKind::MissingRequiredArgument, "--record-net needs --world: {e}");
    }
}
