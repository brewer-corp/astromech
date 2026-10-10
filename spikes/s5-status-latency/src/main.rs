use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use s5_status_latency::binds::{self, BindsDoc, Chord};
use s5_status_latency::keys::{self, ScanKey};
use s5_status_latency::platform::{self, Platform};
use s5_status_latency::run::{self, Config, Session, Stop, TrialResult};
use s5_status_latency::stats::{fmt_ms, summarize};
use s5_status_latency::status;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
#[value(rename_all = "verbatim")]
enum Action {
    ShipSpotLightToggle,
    NightVisionToggle,
    LandingGearToggle,
}

impl Action {
    fn name(self) -> &'static str {
        match self {
            Action::ShipSpotLightToggle => "ShipSpotLightToggle",
            Action::NightVisionToggle => "NightVisionToggle",
            Action::LandingGearToggle => "LandingGearToggle",
        }
    }
    fn bit(self) -> (u64, u32, &'static str) {
        match self {
            Action::ShipSpotLightToggle => (status::FLAG_LIGHTS, 8, "Lights On"),
            Action::NightVisionToggle => (status::FLAG_NIGHT_VISION, 28, "Night Vision"),
            Action::LandingGearToggle => (status::FLAG_GEAR, 2, "Landing Gear Down"),
        }
    }
    fn context(self) -> fn(u64) -> std::result::Result<(), String> {
        match self {
            Action::ShipSpotLightToggle => run::context_ship_only,
            Action::NightVisionToggle => run::context_ship_or_srv,
            Action::LandingGearToggle => run::context_gear,
        }
    }
}

/// Spike S5: measure how long Elite Dangerous takes to reflect a simulated key press in Status.json.
///
/// It presses ONE key binding (default: ship lights) a number of times, only while the game window
/// is focused, and times how long the matching Status.json flag takes to flip, measured two ways:
/// a 2 ms file poll and a file-change notification.
#[derive(Parser, Debug)]
#[command(name = "s5-status-latency", version, about, long_about)]
struct Args {
    /// Which binding to press. LandingGearToggle: only in normal space (not docked, landed or in supercruise).
    #[arg(long, value_enum, default_value = "ShipSpotLightToggle")]
    action: Action,
    /// Number of presses (even numbers leave the ship as it started).
    #[arg(long, default_value_t = 20)]
    trials: usize,
    /// How long to hold the key down, in milliseconds.
    #[arg(long, default_value_t = 60, value_parser = clap::value_parser!(u64).range(5..=500))]
    hold_ms: u64,
    /// Run a few trials at each hold time (20, 40, 60, 100 ms) to find the shortest reliable press.
    #[arg(long)]
    hold_sweep: bool,
    /// Trials per hold time with --hold-sweep.
    #[arg(long, default_value_t = 6)]
    sweep_trials: usize,
    /// Parse everything and print the plan without pressing any key.
    #[arg(long)]
    dry_run: bool,
    /// Journal folder (contains Status.json). Default: %USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous
    #[arg(long)]
    journal_dir: Option<PathBuf>,
    /// Bindings folder. Default: %LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings
    #[arg(long)]
    bindings_dir: Option<PathBuf>,
    /// Use this .binds file instead of the one named in StartPreset.
    #[arg(long)]
    binds_file: Option<PathBuf>,
    /// Press this set-1 scancode instead of the binding (e.g. 0x26 for L). Use with care.
    #[arg(long)]
    scancode: Option<String>,
    /// Mark --scancode as an extended (E0) key, e.g. arrows, Insert, right Ctrl.
    #[arg(long, requires = "scancode")]
    extended: bool,
    /// Per-trial timeout in milliseconds (counted as a miss).
    #[arg(long, default_value_t = 3000)]
    timeout_ms: u64,
    /// Poll interval for method (a), in milliseconds.
    #[arg(long, default_value_t = 2, value_parser = clap::value_parser!(u64).range(1..=50))]
    poll_ms: u64,
    /// Countdown before the first press, in seconds (time to alt-tab into the game).
    #[arg(long, default_value_t = 5)]
    countdown: u64,
    /// Where to write the CSV (default: next to the exe, else the current folder).
    #[arg(long)]
    out_dir: Option<PathBuf>,
}

fn env_path(var: &str, rest: &[&str]) -> Option<PathBuf> {
    let mut p = PathBuf::from(std::env::var_os(var)?);
    for r in rest {
        p.push(r);
    }
    Some(p)
}

fn main() {
    let args = Args::parse();
    match real_main(args) {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("\nERROR: {e:#}");
            std::process::exit(2);
        }
    }
}

fn resolve_chord(args: &Args, bindings_dir: &std::path::Path) -> Result<Chord> {
    if let Some(sc) = &args.scancode {
        let scan = keys::parse_scancode(sc).with_context(|| format!("--scancode '{sc}' is not a scancode between 0x01 and 0x7F"))?;
        return Ok(Chord {
            source: "--scancode".into(),
            key_name: format!("scancode 0x{scan:02X}"),
            key: ScanKey { scan, extended: args.extended },
            modifiers: vec![],
            notes: vec!["--scancode override: the .binds file was not used".into()],
        });
    }
    let binds_path = match &args.binds_file {
        Some(p) => {
            println!("Binds file:     {} (from --binds-file)", p.display());
            p.clone()
        }
        None => {
            let sp = binds::find_start_preset(bindings_dir)?;
            println!("StartPreset:    {}", sp.file.display());
            for (ctx, name) in sp.context_names() {
                println!("                {ctx:<8} {name}");
            }
            if !sp.all_same() {
                println!("                note: presets differ per context; using the Ship one ('{}')", sp.ship_preset());
            }
            if sp.lines.len() != 1 && sp.lines.len() != 4 {
                println!("                warning: expected 1 or 4 lines, found {}; assuming line 2 is Ship", sp.lines.len());
            }
            let p = binds::find_binds_file(bindings_dir, sp.ship_preset())?;
            println!("Binds file:     {}", p.display());
            p
        }
    };
    let xml = std::fs::read_to_string(&binds_path).with_context(|| format!("reading {}", binds_path.display()))?;
    let doc = BindsDoc::parse(&xml)?;
    println!(
        "Preset:         {} (version {}, keyboard layout {})",
        doc.preset_name.as_deref().unwrap_or("?"),
        doc.version.as_deref().unwrap_or("?"),
        doc.keyboard_layout.as_deref().unwrap_or("?")
    );
    if let Some(l) = &doc.keyboard_layout {
        if !l.eq_ignore_ascii_case("en-US") {
            println!("                note: key names are treated as physical (US-position) keys; not yet verified on {l}");
        }
    }
    let ab = doc
        .action(args.action.name())
        .with_context(|| format!("{} not found in {}", args.action.name(), binds_path.display()))?;
    println!(
        "Bindings:       Primary {} | Secondary {}",
        ab.primary.as_ref().map(|b| b.describe()).unwrap_or_else(|| "(missing)".into()),
        ab.secondary.as_ref().map(|b| b.describe()).unwrap_or_else(|| "(missing)".into())
    );
    binds::select_keyboard_chord(args.action.name(), &ab)
}

fn print_summary(label: &str, rows: &[TrialResult]) {
    let p = summarize(&rows.iter().map(|r| r.poll_ms).collect::<Vec<_>>());
    let n = summarize(&rows.iter().map(|r| r.notify_ms).collect::<Vec<_>>());
    println!("\n{label}");
    println!("  method        n  miss     min  median     p95     max   (ms from key-down)");
    for (name, s) in [("(a) poll 2ms", &p), ("(b) notify", &n)] {
        println!(
            "  {name:<12} {:>2}  {:>4} {:>7} {:>7} {:>7} {:>7}",
            s.n, s.misses, fmt_ms(s.min), fmt_ms(s.median), fmt_ms(s.p95), fmt_ms(s.max)
        );
    }
    let missed_both = rows.iter().filter(|r| r.missed()).count();
    println!("  trials {} | missed by both methods: {}", rows.len(), missed_both);
}

fn real_main(args: Args) -> Result<i32> {
    println!("s5-status-latency {} (spike S5, MIT licence)\n", env!("CARGO_PKG_VERSION"));
    let journal_dir = args
        .journal_dir
        .clone()
        .or_else(|| env_path("USERPROFILE", &["Saved Games", "Frontier Developments", "Elite Dangerous"]))
        .context("cannot find the journal folder: pass --journal-dir")?;
    let bindings_dir = args
        .bindings_dir
        .clone()
        .or_else(|| env_path("LOCALAPPDATA", &["Frontier Developments", "Elite Dangerous", "Options", "Bindings"]))
        .context("cannot find the bindings folder: pass --bindings-dir")?;
    println!("Journal folder: {}", journal_dir.display());
    println!("Bindings folder:{}{}", if bindings_dir.as_os_str().is_empty() { "" } else { " " }, bindings_dir.display());

    let (bit, bitno, bitname) = args.action.bit();
    let chord = resolve_chord(&args, &bindings_dir)?;
    println!("Action:         {} -> Status.json Flags bit {bitno} (0x{bit:X}, {bitname})", args.action.name());
    println!("Will press:     {} [{}]", chord.describe(), chord.source);
    for n in &chord.notes {
        println!("                note: {n}");
    }
    if args.action == Action::LandingGearToggle {
        println!("\nWARNING: landing gear test. Only run this in normal space, away from stations and planets'");
        println!("         surfaces: NOT docked, NOT landed, NOT in supercruise. The tool refuses if Status.json");
        println!("         shows docked/landed/supercruise, but you are responsible for being somewhere safe.\n");
    }

    match status::latest_fileheader(&journal_dir) {
        Some(b) => println!(
            "Game build:     {} {} (from {})",
            b.gameversion.as_deref().unwrap_or("?"),
            b.build.as_deref().unwrap_or("?"),
            b.journal.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default()
        ),
        None => println!("Game build:     no Journal.*.log found in the journal folder"),
    }

    let status_path = journal_dir.join("Status.json");
    let snap = status::read_status_retry(&status_path, 20);
    match snap {
        Some(s) => println!(
            "Status.json:    Flags=0x{:08X}, bit {bitno} currently {}, GuiFocus={}",
            s.flags,
            if s.flags & bit != 0 { "ON" } else { "OFF" },
            s.gui_focus.unwrap_or(0)
        ),
        None => println!("Status.json:    not readable or no Flags yet ({})", status_path.display()),
    }

    #[cfg(windows)]
    let platform: Box<dyn Platform> = Box::new(platform::win::WinPlatform);
    #[cfg(not(windows))]
    let platform: Box<dyn Platform> = Box::new(platform::NoInputPlatform);
    let integ = platform.integrity();
    let integrity_problem = match integ.verdict() {
        Ok(s) => {
            println!("Integrity:      {s}");
            false
        }
        Err(s) => {
            println!("Integrity:      WARNING: {s}");
            true
        }
    };

    let out_dir = match &args.out_dir {
        Some(d) => d.clone(),
        None => {
            let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf()));
            match exe_dir {
                Some(d) if run::ensure_parent_writable(&d).is_ok() => d,
                _ => std::env::current_dir()?,
            }
        }
    };
    let csv_path = out_dir.join(format!(
        "s5-status-latency-{}-{}.csv",
        args.action.name(),
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    ));

    let holds: Vec<u64> = if args.hold_sweep { vec![20, 40, 60, 100] } else { vec![args.hold_ms] };
    let per_hold = if args.hold_sweep { args.sweep_trials } else { args.trials };
    println!(
        "Plan:           {} press(es) per hold, hold {:?} ms, timeout {} ms, gap 1.5-2.5 s random, poll every {} ms",
        per_hold, holds, args.timeout_ms, args.poll_ms
    );
    if per_hold % 2 == 1 {
        println!("                note: odd trial count; the tool presses once more at the end to restore the original state");
    }
    println!("CSV:            {}", csv_path.display());

    if args.dry_run {
        if let Some(s) = snap {
            let mut on = s.flags & bit != 0;
            let seq: Vec<&str> = (0..per_hold.min(8))
                .map(|_| {
                    on = !on;
                    if on { "ON" } else { "OFF" }
                })
                .collect();
            println!("Expected flips: {}{}", seq.join(", "), if per_hold > 8 { ", ..." } else { "" });
            if let Err(e) = (args.action.context())(s.flags) {
                println!("Context:        would REFUSE right now: {e}");
            } else {
                println!("Context:        OK to press");
            }
        }
        println!("\nDry run: nothing was pressed. Remove --dry-run to measure.");
        return Ok(0);
    }

    if !cfg!(windows) {
        bail!("live measurement needs Windows (this build can only do --dry-run)");
    }
    let s = snap.context("Status.json has no Flags. Start Elite, load into your ship, then run again")?;
    (args.action.context())(s.flags).map_err(|e| anyhow::anyhow!("refusing to start: {e}"))?;
    if integrity_problem {
        println!("\n>>> Continuing despite the integrity warning above; expect misses if Windows blocks the presses.");
    }

    let cancel = Arc::new(AtomicBool::new(false));
    {
        let c = cancel.clone();
        ctrlc::set_handler(move || {
            c.store(true, Ordering::SeqCst);
        })
        .context("installing the Ctrl+C handler")?;
    }

    let cfg = Config {
        action: args.action.name().into(),
        bit,
        chord,
        status_path: status_path.clone(),
        watch_dir: journal_dir.clone(),
        timeout: Duration::from_millis(args.timeout_ms),
        gap_ms: (1500, 2500),
        poll_interval: Duration::from_millis(args.poll_ms),
        focus_giveup: Duration::from_secs(30),
        require_no_panel: true,
        context_check: args.action.context(),
        quiet: false,
    };

    println!("\nSwitch to Elite Dangerous now (alt-tab or click the game). Press Ctrl+C here to stop at any time.");
    for i in (1..=args.countdown).rev() {
        if cancel.load(Ordering::SeqCst) {
            println!("Cancelled before any key was pressed.");
            return Ok(130);
        }
        println!("  starting in {i}...");
        std::thread::sleep(Duration::from_secs(1));
    }

    platform.begin_precise_timing();
    let mut session = Session::new(&cfg, platform.as_ref(), cancel.clone());
    if let Some(e) = &session.detectors.notify_error {
        println!("note: change notification unavailable ({e}); only the poll method will report");
    }
    let initial = session.current_bit();
    let mut rows: Vec<TrialResult> = vec![];
    let mut stop: Option<Stop> = None;
    println!();
    'outer: for &hold in &holds {
        if args.hold_sweep {
            println!("-- hold {hold} ms --");
        }
        for i in 0..per_hold {
            match session.trial(hold) {
                Ok(r) => {
                    println!("{}", run::line_for(&r, &cfg.action));
                    rows.push(r);
                }
                Err(e) => {
                    stop = Some(e);
                    break 'outer;
                }
            }
            let last = i + 1 == per_hold && hold == *holds.last().unwrap();
            if !last {
                if let Err(e) = session.gap() {
                    stop = Some(e);
                    break 'outer;
                }
            }
        }
    }

    // Restore the original state after a normal finish (same binding, one press).
    if stop.is_none() {
        if let (Some(init), Some(now)) = (initial, session.current_bit()) {
            if init != now {
                println!("\nRestoring the original state with one more press...");
                let r = session.gap().and_then(|_| session.trial(*holds.last().unwrap()));
                match r {
                    Ok(r) if !r.missed() => println!("  restored."),
                    Ok(_) => println!("  the restore press was not observed; please check the {} state in game.", cfg.action),
                    Err(e) => println!("  could not restore ({e}); please toggle it back by hand."),
                }
            }
        }
    }
    drop(session);
    platform.end_precise_timing();

    if let Some(e) = &stop {
        println!("\nRun stopped early: {e}. No keys are held.");
    }
    if rows.is_empty() {
        println!("No trials completed; no CSV written.");
        return Ok(if matches!(stop, Some(Stop::Cancelled)) { 130 } else { 1 });
    }

    if args.hold_sweep {
        println!("\nHold sweep (both methods; a press counts as reliable when no trial was missed)");
        println!("  hold   trials  missed  poll median  poll p95");
        let mut best = None;
        for &h in &holds {
            let rs: Vec<_> = rows.iter().filter(|r| r.hold_ms == h).cloned().collect();
            if rs.is_empty() {
                continue;
            }
            let p = summarize(&rs.iter().map(|r| r.poll_ms).collect::<Vec<_>>());
            let missed = rs.iter().filter(|r| r.missed()).count();
            println!("  {h:>4}   {:>6}  {:>6}  {:>11}  {:>8}", rs.len(), missed, fmt_ms(p.median), fmt_ms(p.p95));
            if missed == 0 && best.is_none() {
                best = Some(h);
            }
        }
        match best {
            Some(h) => println!("  shortest hold with no misses: {h} ms"),
            None => println!("  every hold time had misses"),
        }
        print_summary("All sweep trials combined", &rows);
    } else {
        print_summary(&format!("Summary: {} x {}, hold {} ms", rows.len(), cfg.action, args.hold_ms), &rows);
    }

    run::write_csv(&csv_path, &cfg.action, &rows)?;
    println!("\nCSV written: {}", csv_path.display());
    println!("Please send that CSV file back (plus this window's text if anything looked odd).");
    Ok(if stop.is_some() { 1 } else { 0 })
}
