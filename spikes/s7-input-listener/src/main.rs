//! s7-input-listener: listens read-only to controllers, keyboard and mouse in the background
//! for a fixed time, then reports what it saw and whether capture works while Elite has focus.
//! It never sends input and never writes anything except its own report.

#[allow(dead_code)]
mod binds;
mod binds_index;
#[allow(dead_code)]
mod game;
mod model;
mod report;
mod summary;
mod tracker;
#[cfg(windows)]
mod win;

use anyhow::{Context, Result};
use clap::Parser;
use model::{DeviceMeta, Sample};
use serde::Deserialize;
use std::io::IsTerminal;
#[cfg(windows)]
use std::io::Write;
use std::path::PathBuf;

pub const ELITE_TITLE: &str = "Elite - Dangerous (CLIENT)";

#[derive(Parser, Debug)]
#[command(version, about = "Read-only background listener for controller/keyboard/mouse input while Elite Dangerous has focus")]
struct Args {
    /// How long to listen, in seconds
    #[arg(long, default_value_t = 60)]
    seconds: u64,
    /// Countdown before listening starts (time to switch to Elite)
    #[arg(long, default_value_t = 5)]
    countdown: u64,
    /// Load the active ship preset (read-only) and show the Elite actions bound to each control
    #[arg(long)]
    with_binds: bool,
    /// Bindings folder (default: %LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings)
    #[arg(long)]
    bindings_dir: Option<PathBuf>,
    /// Game install folder (for built-in presets and DeviceMappings.xml)
    #[arg(long)]
    game_dir: Option<PathBuf>,
    /// Window title that counts as "Elite focused"
    #[arg(long, default_value = ELITE_TITLE)]
    elite_title: String,
    /// Don't record which keyboard keys were pressed (only that a key was pressed)
    #[arg(long)]
    no_key_names: bool,
    /// Replay recorded/synthetic samples from a JSON file instead of listening (testing)
    #[arg(long)]
    replay: Option<PathBuf>,
    /// Where to write the report (default: next to the exe)
    #[arg(long)]
    out_dir: Option<PathBuf>,
    /// Don't wait for Enter before exiting
    #[arg(long)]
    no_pause: bool,
}

#[derive(Deserialize)]
struct ReplayFile {
    #[serde(default)]
    devices: Vec<DeviceMeta>,
    samples: Vec<Sample>,
    /// Optional focus timeline [[t_ms, title], ...]; otherwise derived from the samples.
    #[serde(default)]
    focus: Vec<(f64, String)>,
    #[serde(default)]
    duration_ms: Option<f64>,
}

fn main() {
    let double_clicked = std::env::args().len() == 1 && std::io::stdin().is_terminal();
    let args = Args::parse();
    let code = match run(&args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("\nError: {e:#}");
            1
        }
    };
    if double_clicked && !args.no_pause {
        eprintln!("\nPress Enter to close.");
        let mut s = String::new();
        let _ = std::io::stdin().read_line(&mut s);
    }
    std::process::exit(code);
}

struct Captured {
    mode: String,
    timeline: Vec<(f64, String)>,
    end_ms: f64,
    registration: Result<(), String>,
    xinput: String,
}

fn run(args: &Args) -> Result<()> {
    println!("s7-input-listener {} (read-only: listens only, never sends input or changes game files)", env!("CARGO_PKG_VERSION"));
    let mut notes = vec![];
    let (binds, binds_error) = if args.with_binds {
        let dir = args.bindings_dir.clone().or_else(game::default_bindings_dir).context("no Bindings folder; pass --bindings-dir")?;
        match binds_index::load(&dir, args.game_dir.as_deref()) {
            Ok(ix) => {
                println!("Binds (read-only): active {} preset \"{}\" from {} ({} bound slots)", ix.context, ix.preset_name, ix.source, ix.bound_slots);
                (Some(ix), None)
            }
            Err(e) => {
                println!("Binds not loaded: {e}");
                (None, Some(e))
            }
        }
    } else {
        (None, None)
    };
    let red = report::Redactor::from_env();
    let mut p = tracker::Processor::new(&args.elite_title, binds.clone(), !args.no_key_names);
    let cap = match &args.replay {
        Some(path) => replay(path, &mut p, &red)?,
        None => live(args, &mut p, &red)?,
    };
    for e in p.tick(cap.end_ms + 1e9) {
        println!("{}", red.apply(&report::event_line(&e)));
    }
    if p.switch_artifacts > 0 {
        notes.push(format!("{} Tab/Alt/Win key-up(s) without a key-down were ignored: Alt+Tab artefacts (Windows swallows the key-down while switching windows).", p.switch_artifacts));
    }
    if args.no_key_names {
        notes.push("--no-key-names: keyboard key identities were not recorded.".into());
    }
    notes.push("Axis movements are summarised: a movement starts when an axis leaves its rest position by 5% of its range and ends after 400 ms without a change of 1% or more.".into());
    notes.push("Elite axis names for sliders/dials (Joy_UAxis, Joy_VAxis) follow DirectInput slider order and are unverified. XInput controls are shown with XInput names; their Elite names are not mapped.".into());
    let (focus, devices, over, anomalies, v) = summary::build(&p, &cap.timeline, cap.end_ms, cap.registration.clone(), &mut notes);
    let [overall, c, k, m] = v;
    let mut events = p.events.clone();
    events.sort_by(|a, b| a.t_ms.partial_cmp(&b.t_ms).unwrap());
    let rep = summary::Report {
        tool: format!("s7-input-listener {}", env!("CARGO_PKG_VERSION")),
        generated_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string(),
        mode: cap.mode,
        read_only: "Read-only: Raw Input RIDEV_INPUTSINK listener + XInputGetState polling. No input was sent, no game file was read for writing.".into(),
        seconds: cap.end_ms / 1000.0,
        elite_title: args.elite_title.clone(),
        registration: match &cap.registration {
            Ok(()) => "ok (keyboard, mouse, joystick, gamepad, multi-axis; RIDEV_INPUTSINK)".into(),
            Err(e) => format!("FAILED: {e}"),
        },
        xinput: cap.xinput,
        verdict: overall,
        verdict_controllers: c,
        verdict_keyboard: k,
        verdict_mouse: m,
        focus,
        devices,
        above_32_warnings: over,
        anomalies,
        key_repeats_collapsed: p.key_repeats,
        binds,
        binds_error,
        events,
        notes,
    };
    print!("{}", red.apply(&report::console_summary(&rep)));
    let out_dir = match &args.out_dir {
        Some(d) => d.clone(),
        None => std::env::current_exe()?.parent().context("exe has no folder")?.to_path_buf(),
    };
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let base = out_dir.join(format!("s7-input-{stamp}"));
    let mut json = serde_json::to_value(&rep)?;
    red.apply_json(&mut json);
    let jp = base.with_extension("json");
    let hp = base.with_extension("html");
    std::fs::write(&jp, serde_json::to_string_pretty(&json)?).with_context(|| format!("writing {}", jp.display()))?;
    std::fs::write(&hp, red.apply(&report::html(&rep))).with_context(|| format!("writing {}", hp.display()))?;
    println!("\nReport written:\n  {}\n  {}", red.apply(&hp.display().to_string()), red.apply(&jp.display().to_string()));
    Ok(())
}

fn replay(path: &std::path::Path, p: &mut tracker::Processor, red: &report::Redactor) -> Result<Captured> {
    let f: ReplayFile = serde_json::from_str(&std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?).context("parsing replay JSON")?;
    for d in f.devices {
        p.add_device(d);
    }
    let mut samples = f.samples;
    samples.sort_by(|a, b| a.t_ms.partial_cmp(&b.t_ms).unwrap());
    let mut timeline = f.focus;
    if timeline.is_empty() {
        for s in &samples {
            if timeline.last().map(|l: &(f64, String)| l.1 != s.fg).unwrap_or(true) {
                timeline.push((s.t_ms, s.fg.clone()));
            }
        }
    }
    println!("Replaying {} samples from {}\n", samples.len(), red.apply(&path.display().to_string()));
    for s in &samples {
        for e in p.process(s) {
            println!("{}", red.apply(&report::event_line(&e)));
        }
    }
    let end_ms = f.duration_ms.unwrap_or_else(|| samples.last().map(|s| s.t_ms + 1000.0).unwrap_or(0.0));
    Ok(Captured { mode: format!("replay: {}", path.file_name().unwrap_or_default().to_string_lossy()), timeline, end_ms, registration: Ok(()), xinput: "replay".into() })
}

#[cfg(not(windows))]
fn live(_args: &Args, _p: &mut tracker::Processor, _red: &report::Redactor) -> Result<Captured> {
    anyhow::bail!("live listening is Windows-only; use --replay <json>")
}

#[cfg(windows)]
fn live(args: &Args, p: &mut tracker::Processor, red: &report::Redactor) -> Result<Captured> {
    use std::time::{Duration, Instant};
    for d in win::rawinput::list_devices() {
        println!("  device: {}{}", d.display(), d.elite_id().map(|i| format!("  Elite id {i}, declares {} buttons, {} hat(s), axes {}", d.buttons_declared, d.hats_declared, d.axes_declared.join("/"))).unwrap_or_default());
        p.add_device(d);
    }
    for i in (1..=args.countdown).rev() {
        print!("\rSwitch to Elite now. Listening starts in {i} s... ");
        let _ = std::io::stdout().flush();
        std::thread::sleep(Duration::from_secs(1));
    }
    let start = Instant::now();
    let (tx, rx) = std::sync::mpsc::channel();
    let registration = win::rawinput::start(tx.clone(), start);
    let xinput = win::xinput::start(tx, start);
    println!("\rListening for {} s (Raw Input: {}; XInput: {}). Press buttons, move axes, use hats.\n", args.seconds, match &registration { Ok(()) => "ok".to_string(), Err(e) => e.clone() }, xinput);
    let mut timeline: Vec<(f64, String)> = vec![];
    let mut last_fg = Instant::now() - Duration::from_secs(1);
    let total = Duration::from_secs(args.seconds);
    let mut last_progress = 0u64;
    while start.elapsed() < total {
        if last_fg.elapsed() >= Duration::from_millis(200) {
            last_fg = Instant::now();
            let t = win::foreground_title();
            if timeline.last().map(|l| l.1 != t).unwrap_or(true) {
                let now = start.elapsed().as_secs_f64() * 1000.0;
                println!("{:>8.3}s focus -> {}", now / 1000.0, if t == args.elite_title { format!("ELITE ({t})") } else { red.apply(&t) });
                timeline.push((now, t));
            }
        }
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(win::Msg::Device(d)) => p.add_device(d),
            Ok(win::Msg::Sample(s)) => {
                for e in p.process(&s) {
                    println!("{}", red.apply(&report::event_line(&e)));
                }
            }
            Err(_) => {}
        }
        for e in p.tick(start.elapsed().as_secs_f64() * 1000.0) {
            println!("{}", red.apply(&report::event_line(&e)));
        }
        let left = total.saturating_sub(start.elapsed()).as_secs();
        if left % 15 == 0 && left != last_progress && left > 0 {
            last_progress = left;
            println!("          ... {left} s left");
        }
    }
    // Drain what's queued.
    while let Ok(m) = rx.try_recv() {
        match m {
            win::Msg::Device(d) => p.add_device(d),
            win::Msg::Sample(s) => {
                for e in p.process(&s) {
                    println!("{}", red.apply(&report::event_line(&e)));
                }
            }
        }
    }
    Ok(Captured { mode: "live".into(), timeline, end_ms: start.elapsed().as_secs_f64() * 1000.0, registration, xinput })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_fixture_end_to_end() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/replay-vkb.json");
        let red = report::Redactor::new(vec![], None);
        let mut p = tracker::Processor::new(ELITE_TITLE, None, true);
        let cap = replay(&path, &mut p, &red).unwrap();
        p.tick(1e12);
        let (focus, devs, over, anomalies, v) = summary::build(&p, &cap.timeline, cap.end_ms, Ok(()), &mut vec![]);
        assert_eq!(v[0].status, "WORKS");
        assert_eq!(focus.elite_focused_ms, 9300.0);
        let l = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D3201")).unwrap();
        assert_eq!(l.buttons.iter().map(|b| b.elite_name.clone().unwrap()).collect::<Vec<_>>(), vec!["Joy_4", "Joy_7", "Joy_12"]);
        assert_eq!(l.hats.len(), 2);
        assert_eq!(l.axes[0].movements, 2);
        let r = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D0200")).unwrap();
        assert_eq!(r.above_32, vec![33]);
        assert_eq!((r.events_elite_focused, r.events_not_focused), (5, 2));
        assert_eq!(over.len(), 1);
        assert_eq!(anomalies.len(), 1);
        assert!(anomalies[0].contains("Joy_7"));
    }
    /// Shape of Sumeet's DualVKB.4.2.binds: axes live under axis elements with a <Binding> plus
    /// Inverted/Deadzone siblings; buttons use Primary/Secondary.
    #[test]
    fn axis_actions_from_his_preset_shape() {
        let d = tempfile::tempdir().unwrap();
        let x = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="DualVKB" MajorVersion="4" MinorVersion="2">
	<YawAxisRaw>
		<Binding Device="231D0200" Key="Joy_RZAxis" />
		<Inverted Value="0" />
		<Deadzone Value="0.00000000" />
	</YawAxisRaw>
	<PitchAxisRaw>
		<Binding Device="231D0200" Key="Joy_YAxis" />
		<Inverted Value="1" />
		<Deadzone Value="0.00000000" />
	</PitchAxisRaw>
	<ThrottleAxis>
		<Binding Device="231D3201" Key="Joy_YAxis" />
		<Inverted Value="0" />
		<Deadzone Value="0.00000000" />
	</ThrottleAxis>
	<ForwardKey>
		<Primary Device="{NoDevice}" Key="" />
		<Secondary Device="231D3201" Key="Pos_Joy_RZAxis" />
	</ForwardKey>
</Root>"#;
        std::fs::write(d.path().join("DualVKB.4.2.binds"), x).unwrap();
        std::fs::write(d.path().join("StartPreset.4.start"), "DualVKB\nDualVKB\nDualVKB\nDualVKB\n").unwrap();
        let ix = binds_index::load(d.path(), Some(&d.path().join("nogame"))).unwrap();
        let mut p = tracker::Processor::new(ELITE_TITLE, Some(ix), true);
        for (k, pid) in [("l", 0x3201u16), ("r", 0x0200)] {
            p.add_device(DeviceMeta { key: k.into(), kind: model::DevKind::Hid, vid: 0x231D, pid, product: k.into(), usage: 4, buttons_declared: 128, axes_declared: vec!["Y".into(), "RZ".into()], hats_declared: 1 });
        }
        let s = |t: f64, dev: &str, y: f64, rz: f64| Sample { t_ms: t, dev: dev.into(), fg: ELITE_TITLE.into(), injected: false, data: model::SampleData::Hid { buttons: vec![], hats: vec![None], axes: vec![("Y".into(), y), ("RZ".into(), rz)] } };
        p.process(&s(0.0, "l", 0.5, 0.5));
        p.process(&s(0.0, "r", 0.5, 0.5));
        p.process(&s(100.0, "l", 0.9, 0.5));
        p.process(&s(100.0, "r", 0.5, 0.1));
        p.process(&s(200.0, "l", 0.5, 0.8));
        p.tick(5000.0);
        let (_, devs, _, _, _) = summary::build(&p, &[(0.0, ELITE_TITLE.into())], 5000.0, Ok(()), &mut vec![]);
        let l = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D3201")).unwrap();
        let y = l.axes.iter().find(|a| a.axis == "Y axis").unwrap();
        assert_eq!(y.actions, vec!["ThrottleAxis (Binding)"]);
        let rz = l.axes.iter().find(|a| a.axis == "RZ axis").unwrap();
        assert_eq!(rz.actions, vec!["ForwardKey (Secondary)"], "Pos_/Neg_ axis keys");
        let r = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D0200")).unwrap();
        assert_eq!(r.axes.iter().find(|a| a.axis == "RZ axis").unwrap().actions, vec!["YawAxisRaw (Binding)"]);
        let html = report::html(&summary::Report {
            tool: String::new(), generated_at: String::new(), mode: String::new(), read_only: String::new(), seconds: 0.0, elite_title: String::new(), registration: String::new(), xinput: String::new(),
            verdict: summary::Verdict { status: "WORKS".into(), detail: String::new() },
            verdict_controllers: summary::Verdict { status: "WORKS".into(), detail: String::new() },
            verdict_keyboard: summary::Verdict { status: "WORKS".into(), detail: String::new() },
            verdict_mouse: summary::Verdict { status: "WORKS".into(), detail: String::new() },
            focus: Default::default(), devices: devs, above_32_warnings: vec![], anomalies: vec![], key_repeats_collapsed: 0, binds: None, binds_error: None, events: vec![], notes: vec![],
        });
        assert!(html.contains("ThrottleAxis (Binding)"));
    }
    #[test]
    fn redaction() {
        let r = report::Redactor::new(vec![("C:\\Users\\Sumeet\\AppData\\Local".into(), "%LOCALAPPDATA%".into())], Some("Sumeet".into()));
        assert_eq!(r.apply("c:\\users\\sumeet\\AppData\\Local\\x"), "%LOCALAPPDATA%\\x");
        assert_eq!(r.apply("C:\\Users\\SUMEET\\Desktop"), "C:\\Users\\<user>\\Desktop");
    }
}
