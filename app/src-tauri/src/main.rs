//! # astromech-shell
//!
//! The app window (docs/V1-DESIGN.md §7.1): a thin Tauri v2 shell around the Svelte app in
//! `app/`. It holds no game logic. It talks to `astromech-core` over the core's local control
//! API (`%LOCALAPPDATA%\astromech\data\core.json`: port + token), and:
//! - starts the core if it isn't running (the Start menu shortcut opens this window);
//! - restarts the core if it crashed while the window is open (§7.7 crash restart);
//! - is single-instance: a second start (e.g. from the tray) focuses this window and switches to
//!   the page given with `--route`.
//!
//! Closing the window exits this process and frees WebView2; the core keeps running.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::Duration;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const CORE_EXE: &str = if cfg!(windows) { "astromech-core.exe" } else { "astromech-core" };

#[derive(Deserialize)]
struct Endpoint {
    port: u16,
    token: String,
}

fn app_dir() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share")))
        .unwrap_or_else(|| ".".into())
        .join("astromech")
        .join("data")
}

fn endpoint_file() -> PathBuf {
    app_dir().join("core.json")
}

fn call(cmd: &str) -> Result<Value, String> {
    call_with(cmd, None, None)
}

/// One request to the core; `text` is the voice tester's phrase or command id, `args` a setup
/// command's arguments (e.g. `set_ptt` with `{device, key}`).
fn call_with(cmd: &str, text: Option<&str>, args: Option<Value>) -> Result<Value, String> {
    let t = std::fs::read_to_string(endpoint_file()).map_err(|_| "the core isn't running".to_string())?;
    let ep: Endpoint = serde_json::from_str(&t).map_err(|e| e.to_string())?;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], ep.port));
    let mut s = TcpStream::connect_timeout(&addr, Duration::from_secs(2)).map_err(|e| e.to_string())?;
    s.set_read_timeout(Some(Duration::from_secs(15))).map_err(|e| e.to_string())?;
    let req = serde_json::json!({ "token": ep.token, "cmd": cmd, "text": text, "args": args });
    writeln!(s, "{req}").map_err(|e| e.to_string())?;
    let mut line = String::new();
    BufReader::new(s).read_line(&mut line).map_err(|e| e.to_string())?;
    serde_json::from_str(&line).map_err(|e| e.to_string())
}

fn core_alive() -> bool {
    call("status").map(|v| v["ok"] == true).unwrap_or(false)
}

/// Start `astromech-core --background` from next to this exe, without a console window.
fn start_core() -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?.with_file_name(CORE_EXE);
    if !exe.is_file() {
        return Err(format!("{} not found", exe.display()));
    }
    let mut c = std::process::Command::new(exe);
    c.arg("--background");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        c.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }
    c.spawn().map(|_| ()).map_err(|e| e.to_string())
}

/// While the window is open: if the core's endpoint file is there but the core doesn't answer
/// twice in a row, it crashed: start it again. (A clean Quit removes the file, so it stays off.)
fn watchdog(app: tauri::AppHandle) {
    let mut misses = 0;
    loop {
        std::thread::sleep(Duration::from_secs(5));
        if !endpoint_file().exists() {
            misses = 0;
            continue;
        }
        if core_alive() {
            misses = 0;
            continue;
        }
        misses += 1;
        if misses >= 2 {
            misses = 0;
            let r = start_core();
            let _ = app.emit("core-restarted", r.clone().err());
        }
    }
}

// ---- Keybinds (M1). The window reads and edits presets itself through astromech-keybinds
// (every write backs up first); the core reloads on its own Bindings-folder watch.

struct Binds(std::sync::Mutex<Option<astromech_keybinds::api::Api>>);

#[derive(Deserialize, Default)]
struct GameCfg {
    #[serde(default)]
    bindings_dir: String,
    #[serde(default)]
    game_dir: String,
    #[serde(default)]
    journal_dir: String,
}
#[derive(Deserialize, Default)]
struct Cfg {
    #[serde(default)]
    game: GameCfg,
}

fn make_api() -> Result<astromech_keybinds::api::Api, String> {
    let cfg: Cfg = std::fs::read_to_string(app_dir().join("astromech.toml"))
        .ok()
        .and_then(|t| toml::from_str(&t).ok())
        .unwrap_or_default();
    let opt = |s: &str| (!s.trim().is_empty()).then(|| PathBuf::from(s.trim()));
    astromech_keybinds::api::Api::discover(
        opt(&cfg.game.bindings_dir),
        opt(&cfg.game.game_dir).as_deref(),
        app_dir(),
    )
    .map_err(|e| format!("{e:#}"))
}

fn with_api<T>(b: &tauri::State<Binds>, f: impl FnOnce(&mut astromech_keybinds::api::Api) -> anyhow::Result<T>) -> Result<T, String> {
    let mut g = b.0.lock().map_err(|_| "busy".to_string())?;
    if g.is_none() {
        *g = Some(make_api()?);
    }
    f(g.as_mut().unwrap()).map_err(|e| format!("{e:#}"))
}

#[tauri::command]
fn binds_state(b: tauri::State<Binds>) -> Result<Value, String> {
    with_api(&b, |a| Ok(serde_json::to_value(a.state())?))
}

#[tauri::command]
fn binds_refresh_devices(b: tauri::State<Binds>) -> Result<Value, String> {
    with_api(&b, |a| {
        a.refresh_devices();
        Ok(serde_json::to_value(a.state())?)
    })
}

/// Forget the discovered folders and devices, e.g. after onboarding set the Bindings folder.
#[tauri::command]
fn binds_reset(b: tauri::State<Binds>) -> Result<Value, String> {
    *b.0.lock().map_err(|_| "busy".to_string())? = None;
    with_api(&b, |a| Ok(serde_json::to_value(a.state())?))
}

#[tauri::command]
fn binds_view(b: tauri::State<Binds>, path: PathBuf) -> Result<Value, String> {
    with_api(&b, |a| Ok(serde_json::to_value(a.view(&path)?)?))
}

#[tauri::command]
fn binds_set_slot(b: tauri::State<Binds>, edit: astromech_keybinds::api::SlotEdit) -> Result<Value, String> {
    with_api(&b, |a| Ok(serde_json::to_value(a.set_slot(&edit)?)?))
}

#[tauri::command]
fn binds_copy(b: tauri::State<Binds>, src: PathBuf, name: String) -> Result<Value, String> {
    with_api(&b, |a| Ok(serde_json::to_value(a.copy_preset(&src, &name)?)?))
}

#[tauri::command]
fn binds_set_active(b: tauri::State<Binds>, name: String, contexts: Vec<usize>) -> Result<Value, String> {
    with_api(&b, |a| Ok(serde_json::to_value(a.set_active(&name, &contexts)?)?))
}

#[tauri::command]
fn binds_restore(b: tauri::State<Binds>, backup: PathBuf) -> Result<Value, String> {
    with_api(&b, |a| Ok(serde_json::to_value(a.restore(&backup)?)?))
}

#[tauri::command]
fn binds_set_override(b: tauri::State<Binds>, device: String, control: String, joy: Option<String>) -> Result<Value, String> {
    with_api(&b, |a| Ok(serde_json::to_value(a.set_override(&device, &control, joy.as_deref())?)?))
}

fn journal_dir() -> PathBuf {
    let cfg: Cfg = std::fs::read_to_string(app_dir().join("astromech.toml"))
        .ok()
        .and_then(|t| toml::from_str(&t).ok())
        .unwrap_or_default();
    let j = cfg.game.journal_dir.trim();
    if !j.is_empty() {
        return PathBuf::from(j);
    }
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Saved Games")
        .join("Frontier Developments")
        .join("Elite Dangerous")
}

struct MaterialsCache(std::sync::Mutex<Option<(astromech_materials::JournalVersion, Value)>>);

/// Directory-listing metadata only. On Windows it can lag appends to the journal the game holds
/// open, which delays the Engineering page (astromech-ztrb).
fn journal_folder_sig(dir: &std::path::Path) -> (u64, u64, u64) {
    let mut n = 0u64;
    let mut newest = 0u64;
    let mut size = 0u64;
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let name = e.file_name();
            let name = name.to_string_lossy();
            if !(name.starts_with("Journal.") && name.ends_with(".log")) {
                continue;
            }
            n += 1;
            if let Ok(m) = e.metadata() {
                let t = m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);
                if t >= newest {
                    newest = t;
                    size = m.len();
                }
            }
        }
    }
    (n, newest, size)
}

/// Engineering materials from the journal, laid out like the in-game traders.
/// Off the main thread: the page polls every second and a recompute re-reads the session journal.
#[tauri::command]
async fn materials_inventory(app: tauri::AppHandle) -> Result<Value, String> {
    blocking(move || materials_inventory_now(&app.state::<MaterialsCache>())).await
}

fn materials_inventory_now(cache: &MaterialsCache) -> Result<Value, String> {
    let dir = journal_dir();
    if !dir.is_dir() {
        return Err(format!("journal folder not found: {}", dir.display()));
    }
    // Not journal_folder_sig: on Windows that misses appends to the open journal (astromech-txzq).
    let version = astromech_materials::journal_version(&dir);
    {
        let g = cache.0.lock().map_err(|_| "busy".to_string())?;
        if let Some((s, v)) = &*g {
            if *s == version {
                return Ok(v.clone());
            }
        }
    }
    let v = serde_json::to_value(astromech_materials::from_journal_dir(&dir)).map_err(|e| e.to_string())?;
    *cache.0.lock().map_err(|_| "busy".to_string())? = Some((version, v.clone()));
    Ok(v)
}

/// Everything the Engineering screen reads from the journals, re-read when the folder changes.
struct JournalState {
    fleet: astromech_engineering::Fleet,
    ledger: astromech_materials::Ledger,
    engineers: Vec<astromech_engineering::EngineerView>,
    here: Option<astromech_galaxy::Here>,
}

#[allow(clippy::type_complexity)]
struct EngineeringCache(std::sync::Mutex<Option<((u64, u64, u64), std::sync::Arc<JournalState>)>>);

fn engineering_state(cache: &EngineeringCache) -> Result<std::sync::Arc<JournalState>, String> {
    let dir = journal_dir();
    if !dir.is_dir() {
        return Err(format!("journal folder not found: {}", dir.display()));
    }
    let sig = journal_folder_sig(&dir);
    if let Some((s, v)) = &*cache.0.lock().map_err(|_| "busy".to_string())? {
        if *s == sig {
            return Ok(v.clone());
        }
    }
    let v = std::sync::Arc::new(JournalState {
        fleet: astromech_engineering::from_journal_dir(&dir),
        ledger: astromech_materials::replay_journal_dir(&dir),
        engineers: astromech_engineering::engineers::from_journal_dir(&dir),
        here: astromech_galaxy::here_from_journal_dir(&dir),
    });
    *cache.0.lock().map_err(|_| "busy".to_string())? = Some((sig, v.clone()));
    Ok(v)
}

/// The local star map (app dir `galaxy.db`), seeded from the bundled preload. A failed open is
/// retried on the next call instead of being remembered.
struct GalaxyState(std::sync::Mutex<Option<std::sync::Arc<astromech_galaxy::Galaxy>>>);

fn galaxy(app: &tauri::AppHandle) -> Result<std::sync::Arc<astromech_galaxy::Galaxy>, String> {
    let st = app.state::<GalaxyState>();
    let mut g = st.0.lock().map_err(|_| "busy".to_string())?;
    if let Some(x) = g.as_ref() {
        return Ok(x.clone());
    }
    let preload = app.path().resolve("starmap.db", tauri::path::BaseDirectory::Resource).ok();
    let opened = astromech_galaxy::Galaxy::open(&app_dir().join("galaxy.db"), preload.as_deref())
        .map(std::sync::Arc::new)
        .map_err(|e| e.to_string())?;
    *g = Some(opened.clone());
    Ok(opened)
}

/// Jump range for counting: the ship's unladen maximum, else a typical 20 ly.
fn range_of(ship: Option<&astromech_engineering::Ship>) -> f64 {
    ship.and_then(|s| s.max_jump_range).filter(|r| *r > 1.0).unwrap_or(20.0)
}

/// Run blocking work (journal reads, network, SQLite) off the main thread.
async fn blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T, String> + Send + 'static) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f).await.map_err(|e| e.to_string())?
}

/// Every owned ship with its modules and current engineering.
#[tauri::command]
async fn engineering_fleet(app: tauri::AppHandle) -> Result<Value, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        serde_json::to_value(&st.fleet).map_err(|e| e.to_string())
    })
    .await
}

/// Materials a build plan needs against the current inventory, with trades and farming.
#[tauri::command]
async fn engineering_plan(app: tauri::AppHandle, plan: astromech_engineering::Plan) -> Result<Value, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        let ship = st.fleet.ships.iter().find(|s| s.id == plan.ship_id).ok_or("ship not in the fleet")?;
        serde_json::to_value(astromech_engineering::evaluate(ship, &plan, &st.ledger)).map_err(|e| e.to_string())
    })
    .await
}

/// Ship engineers: where they are, access and rank.
#[tauri::command]
async fn engineering_engineers(app: tauri::AppHandle) -> Result<Value, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        serde_json::to_value(serde_json::json!({ "engineers": st.engineers, "here": st.here })).map_err(|e| e.to_string())
    })
    .await
}

/// Which engineers to visit for a plan, in order, from the commander's current system.
#[tauri::command]
async fn engineering_trip(
    app: tauri::AppHandle,
    plan: astromech_engineering::Plan,
) -> Result<Value, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        let ship = st.fleet.ships.iter().find(|s| s.id == plan.ship_id).ok_or("ship not in the fleet")?;
        let result = astromech_engineering::evaluate(ship, &plan, &st.ledger);
        let range = range_of(Some(ship));
        let g = galaxy(&app).ok();
        if let (Some(g), Some(h)) = (&g, &st.here) {
            let _ = g.ensure_covered(h.pos);
        }
        let map = g.as_ref().and_then(|g| g.map().ok());
        let hop = |a: [f64; 3], b: [f64; 3]| {
            let h = match &map {
                Some(m) => m.hop(a, b, range),
                None => {
                    let ly = astromech_galaxy::dist(a, b);
                    astromech_galaxy::Hop { ly, jumps: astromech_galaxy::estimate(ly, range), exact: false }
                }
            };
            astromech_engineering::Hop { ly: h.ly, jumps: h.jumps, exact: h.exact }
        };
        let start = st.here.as_ref().map(|h| (h.system.as_str(), h.pos));
        let trip = astromech_engineering::plan_trip(ship, &result.targets, &st.engineers, start, range, &hop);
        serde_json::to_value(trip).map_err(|e| e.to_string())
    })
    .await
}

/// Systems near the commander matching a lookup (`famine`, or an HGE pool id), with jumps.
#[tauri::command]
async fn galaxy_nearby(app: tauri::AppHandle, query: String) -> Result<Value, String> {
    blocking(move || {
        let st = engineering_state(&app.state::<EngineeringCache>())?;
        let here = st.here.clone().ok_or("no position in the journal yet")?;
        let current = st.fleet.ships.iter().find(|s| s.current);
        let n = galaxy(&app)?.nearby(&query, &here, range_of(current), 8).map_err(|e| e.to_string())?;
        serde_json::to_value(n).map_err(|e| e.to_string())
    })
    .await
}

/// Presses from every stick, pad and key, for "press a control to find it", the key tester
/// and press-to-bind. Read-only (Raw Input with INPUTSINK + XInput polling, like the core).
#[cfg(windows)]
fn start_input(app: tauri::AppHandle) {
    use astromech_input::listen::{tracker::Processor, win};
    std::thread::spawn(move || {
        let start = std::time::Instant::now();
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = win::rawinput::start(tx.clone(), start);
        let _ = win::xinput::start(tx, start);
        let mut p = Processor::new(astromech_input::ELITE_TITLE, None, true);
        for d in win::rawinput::list_devices() {
            p.add_device(d);
        }
        loop {
            let evs = match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(win::Msg::Device(d)) => {
                    p.add_device(d);
                    continue;
                }
                Ok(win::Msg::Sample(s)) => p.process(&s),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => p.tick(start.elapsed().as_secs_f64() * 1000.0),
                Err(_) => break,
            };
            p.events.clear();
            for e in evs {
                let device = match e.kind.as_str() {
                    "key" => "Keyboard".to_string(),
                    "mouse" => "Mouse".to_string(),
                    "xinput" => "GamePad".to_string(),
                    _ => e.elite_id.clone().unwrap_or_default(),
                };
                let Some(key) = e.elite_name.clone() else { continue };
                if e.kind != "axis" && e.down != Some(true) && e.down != Some(false) {
                    continue;
                }
                let _ = app.emit(
                    "input",
                    serde_json::json!({
                        "device": device,
                        "key": key,
                        "kind": e.kind,
                        "down": e.down,
                        "device_name": e.device,
                        "t_ms": e.t_ms,
                    }),
                );
            }
        }
    });
}

#[tauri::command]
async fn core_status() -> Option<Value> {
    blocking(|| Ok(call("status").ok())).await.ok().flatten()
}

/// Blocking: a voice tester live run waits for Elite to come to the front; calibration and the
/// mic check record for a few seconds.
#[tauri::command]
async fn core_call(cmd: String, text: Option<String>, args: Option<Value>) -> Value {
    // The window may only ask for these ("quit" and "open_window" stay with the tray).
    const ALLOWED: &[&str] = &[
        "status",
        "save_diagnostics",
        "ack_crash",
        "check_update",
        "install_update",
        "autostart_on",
        "autostart_off",
        "pause_voice",
        "resume_voice",
        "kill_switch",
        "download_models",
        "open_config",
        "voice_test",
        "voice_test_poll",
        "voice_test_stop",
        "voice_test_live",
        "find_elite",
        "set_ptt",
        "set_kill_switch",
        "set_mic",
        "set_folders",
        "mic_list",
        "mic_check",
        "calibrate_start",
        "calibrate_record",
        "calibrate_finish",
        "calibrate_cancel",
        "onboarding_done",
        "set_voice",
        "voice_settings",
        "voice_preview",
    ];
    if !ALLOWED.contains(&cmd.as_str()) {
        return serde_json::json!({ "ok": false, "error": "not allowed" });
    }
    blocking(move || Ok(call_with(&cmd, text.as_deref(), args).unwrap_or_else(|e| serde_json::json!({ "ok": false, "error": e }))))
        .await
        .unwrap_or_else(|e| serde_json::json!({ "ok": false, "error": e }))
}

#[tauri::command]
fn shell_info() -> Value {
    serde_json::json!({ "version": env!("CARGO_PKG_VERSION") })
}

/// Open the project's GitHub pages or a Coriolis / EDSY / Spansh / Inara link in the browser, or a folder under the app folder.
#[tauri::command]
fn open_external(target: String) -> Result<(), String> {
    let ok_url = target.starts_with("https://github.com/spadaval/astromech")
        || astromech_engineering::links::is_openable_url(&target);
    let ok_dir = {
        let p = PathBuf::from(&target);
        p.is_dir() && p.canonicalize().ok().zip(app_dir().canonicalize().ok()).is_some_and(|(p, a)| p.starts_with(a))
    };
    if !ok_url && !ok_dir {
        return Err("not allowed".into());
    }
    #[cfg(windows)]
    let r = std::process::Command::new("explorer.exe").arg(&target).spawn();
    #[cfg(not(windows))]
    let r = std::process::Command::new("xdg-open").arg(&target).spawn();
    r.map(|_| ()).map_err(|e| e.to_string())
}

fn route_arg(args: &[String]) -> Option<String> {
    let i = args.iter().position(|a| a == "--route")?;
    args.get(i + 1).filter(|r| r.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')).cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let route = route_arg(&args);
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
            if let Some(r) = route_arg(&argv) {
                let _ = app.emit("route", r);
            }
        }))
        .manage(Binds(std::sync::Mutex::new(None)))
        .manage(MaterialsCache(std::sync::Mutex::new(None)))
        .manage(EngineeringCache(std::sync::Mutex::new(None)))
        .manage(GalaxyState(std::sync::Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            core_status,
            core_call,
            shell_info,
            open_external,
            binds_state,
            binds_refresh_devices,
            binds_reset,
            binds_view,
            binds_set_slot,
            binds_copy,
            binds_set_active,
            binds_restore,
            binds_set_override,
            materials_inventory,
            engineering_fleet,
            engineering_plan,
            engineering_engineers,
            engineering_trip,
            galaxy_nearby
        ])
        .setup(move |app| {
            let hash = route.as_deref().map(|r| format!("index.html#/{r}")).unwrap_or_else(|| "index.html".into());
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App(hash.into()))
                .title("astromech")
                .inner_size(1200.0, 800.0)
                .min_inner_size(800.0, 500.0)
                .build()?;
            if !core_alive() {
                if let Err(e) = start_core() {
                    eprintln!("couldn't start the core: {e}");
                }
            }
            let h = app.handle().clone();
            std::thread::spawn(move || watchdog(h));
            #[cfg(windows)]
            start_input(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the astromech app window");
}
