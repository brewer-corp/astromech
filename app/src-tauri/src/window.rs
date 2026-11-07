//! The window itself, built with the `window` feature (see main.rs).
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

use astromech_api::control;
use astromech_store::Paths;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::Duration;
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

const CORE_EXE: &str = if cfg!(windows) { "astromech-core.exe" } else { "astromech-core" };

/// The core's app dir (`astromech_store::Paths`, the core's own rule), where its `core.json` is.
fn app_dir() -> PathBuf {
    Paths::current().app_dir().to_path_buf()
}

fn endpoint_file() -> PathBuf {
    app_dir().join(control::FILE)
}

/// A status check: bounded, so the watchdog notices a core that stopped answering.
fn status() -> Result<Value, String> {
    control::call(&app_dir(), control::Request::new("status"), Some(control::SHORT), &mut |_| {})
}

/// One of the page's requests to the core, stamped `origin: "app_window"`, so the core refuses
/// its `local` commands. `text` is the voice tester's phrase or command id, `args` a command's
/// arguments (e.g. `set_ptt` with `{device, key}`); the command's progress values go to
/// `on_progress`. No read timeout: a plot polls Spansh for minutes; a dropped connection means
/// the core died.
fn call_with(
    cmd: &str,
    text: Option<&str>,
    args: Option<Value>,
    on_progress: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    let req = control::Request {
        text: text.map(String::from),
        args,
        origin: Some(astromech_proto::Origin::AppWindow),
        ..control::Request::new(cmd)
    };
    control::call(&app_dir(), req, None, on_progress)
}

fn core_alive() -> bool {
    status().is_ok()
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

// ---- Every page command is the core's (astromech_api::commands and ::lookups): each forwards
// the page's call unchanged, its arguments under the names the core takes.

/// Run blocking work (a call to the core) off the main thread.
async fn blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T, String> + Send + 'static) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f).await.map_err(|e| e.to_string())?
}

/// The page's call to a command the core serves; the reply is the core's, as before.
async fn forward(cmd: &'static str, args: Option<Value>) -> Result<Value, String> {
    blocking(move || call_with(cmd, None, args, &mut |_| {})).await
}

#[tauri::command]
async fn binds_state() -> Result<Value, String> {
    forward("binds_state", None).await
}

#[tauri::command]
async fn binds_refresh_devices() -> Result<Value, String> {
    forward("binds_refresh_devices", None).await
}

#[tauri::command]
async fn binds_reset() -> Result<Value, String> {
    forward("binds_reset", None).await
}

#[tauri::command]
async fn binds_view(path: PathBuf) -> Result<Value, String> {
    forward("binds_view", Some(json!({ "path": path }))).await
}

#[tauri::command]
async fn binds_set_slot(edit: Value) -> Result<Value, String> {
    forward("binds_set_slot", Some(json!({ "edit": edit }))).await
}

#[tauri::command]
async fn binds_copy(src: PathBuf, name: String) -> Result<Value, String> {
    forward("binds_copy", Some(json!({ "src": src, "name": name }))).await
}

#[tauri::command]
async fn binds_set_active(name: String, contexts: Vec<usize>) -> Result<Value, String> {
    forward("binds_set_active", Some(json!({ "name": name, "contexts": contexts }))).await
}

#[tauri::command]
async fn binds_restore(backup: PathBuf) -> Result<Value, String> {
    forward("binds_restore", Some(json!({ "backup": backup }))).await
}

#[tauri::command]
async fn binds_set_override(device: String, control: String, joy: Option<String>) -> Result<Value, String> {
    forward("binds_set_override", Some(json!({ "device": device, "control": control, "joy": joy }))).await
}

#[tauri::command]
async fn materials_inventory() -> Result<Value, String> {
    forward("materials_inventory", None).await
}

#[tauri::command]
async fn engineering_fleet() -> Result<Value, String> {
    forward("engineering_fleet", None).await
}

#[tauri::command]
async fn engineering_plan(plan: Value) -> Result<Value, String> {
    forward("engineering_plan", Some(json!({ "plan": plan }))).await
}

#[tauri::command]
async fn engineering_stats(plan: Value) -> Result<Value, String> {
    forward("engineering_stats", Some(json!({ "plan": plan }))).await
}

#[tauri::command]
async fn engineering_headlines() -> Result<Value, String> {
    forward("engineering_headlines", None).await
}

#[tauri::command]
async fn engineering_engineers() -> Result<Value, String> {
    forward("engineering_engineers", None).await
}

#[tauri::command]
async fn engineering_unlocks() -> Result<Value, String> {
    forward("engineering_unlocks", None).await
}

#[tauri::command]
async fn odyssey_view() -> Result<Value, String> {
    forward("odyssey_view", None).await
}

#[tauri::command]
async fn odyssey_shopping(picks: Value) -> Result<Value, String> {
    forward("odyssey_shopping", Some(json!({ "picks": picks }))).await
}

#[tauri::command]
async fn powerplay_overview() -> Result<Value, String> {
    forward("powerplay_overview", None).await
}

#[tauri::command]
async fn colonisation_projects() -> Result<Value, String> {
    forward("colonisation_projects", None).await
}

#[tauri::command]
async fn exploration_system() -> Result<Value, String> {
    forward("exploration_system", None).await
}

#[tauri::command]
async fn exploration_unsold() -> Result<Value, String> {
    forward("exploration_unsold", None).await
}

#[tauri::command]
async fn logbook(query: String, cmdr: Option<String>) -> Result<Value, String> {
    forward("logbook", Some(json!({ "query": query, "cmdr": cmdr }))).await
}

#[tauri::command]
async fn session_summary() -> Result<Value, String> {
    forward("session_summary", None).await
}

#[tauri::command]
async fn docked_here() -> Result<Value, String> {
    forward("docked_here", None).await
}

#[tauri::command]
async fn trade_route_defaults() -> Result<Value, String> {
    forward("trade_route_defaults", None).await
}

// ---- The star map, lookups, route plotting and the neutron star file are the core's too
// (astromech_api::lookups).

#[tauri::command]
async fn powerplay_targets(
    target: Value,
    origin: Option<String>,
    filters: Option<Value>,
    within_ly: f64,
) -> Result<Value, String> {
    let args = json!({ "target": target, "origin": origin, "filters": filters, "within_ly": within_ly });
    forward("powerplay_targets", Some(args)).await
}

#[tauri::command]
async fn outfitting_search(query: String) -> Result<Value, String> {
    forward("outfitting_search", Some(json!({ "query": query }))).await
}

#[tauri::command]
async fn engineering_trip(plan: Value) -> Result<Value, String> {
    forward("engineering_trip", Some(json!({ "plan": plan }))).await
}

#[tauri::command]
async fn galaxy_nearby(query: String, origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("galaxy_nearby", Some(json!({ "query": query, "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn stations_nearby(want: Value, origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("stations_nearby", Some(json!({ "want": want, "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn galaxy_sites(kind: String, origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("galaxy_sites", Some(json!({ "kind": kind, "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn colony_candidates(interest: Value, origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("colony_candidates", Some(json!({ "interest": interest, "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn trade_search(
    side: Value,
    commodities: Vec<String>,
    origin: Option<String>,
    filters: Option<Value>,
) -> Result<Value, String> {
    let args = json!({ "side": side, "commodities": commodities, "origin": origin, "filters": filters });
    forward("trade_search", Some(args)).await
}

#[tauri::command]
async fn trade_anywhere(
    side: Value,
    commodity: String,
    origin: Option<String>,
    filters: Option<Value>,
) -> Result<Value, String> {
    let args = json!({ "side": side, "commodity": commodity, "origin": origin, "filters": filters });
    forward("trade_anywhere", Some(args)).await
}

#[tauri::command]
async fn trade_cargo(origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("trade_cargo", Some(json!({ "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn mining_hotspots(want: Value, origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("mining_hotspots", Some(json!({ "want": want, "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn market_compare() -> Result<Value, String> {
    forward("market_compare", None).await
}

#[tauri::command]
async fn trade_route(ask: Value, origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("trade_route", Some(json!({ "ask": ask, "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn route_plot_ship() -> Result<Value, String> {
    forward("route_plot_ship", None).await
}

#[tauri::command]
async fn route_plot(ask: Value) -> Result<Value, String> {
    forward("route_plot", Some(json!({ "ask": ask }))).await
}

#[tauri::command]
async fn neutron_file() -> Result<Value, String> {
    forward("neutron_file", None).await
}

/// The core sends the download's progress as progress lines; each goes to the page's channel.
#[tauri::command]
async fn neutron_download(on_progress: tauri::ipc::Channel<Value>) -> Result<Value, String> {
    blocking(move || {
        call_with("neutron_download", None, None, &mut |p| {
            let _ = on_progress.send(p);
        })
    })
    .await
}

#[tauri::command]
async fn neutron_check() -> Result<Value, String> {
    forward("neutron_check", None).await
}

#[tauri::command]
async fn colonisation_buy(needs: Value, origin: Option<String>, filters: Option<Value>) -> Result<Value, String> {
    forward("colonisation_buy", Some(json!({ "needs": needs, "origin": origin, "filters": filters }))).await
}

#[tauri::command]
async fn search_filters(lookup: String) -> Result<Value, String> {
    forward("search_filters", Some(json!({ "lookup": lookup }))).await
}

#[tauri::command]
async fn set_search_filters(lookup: String, filters: Value) -> Result<Value, String> {
    forward("set_search_filters", Some(json!({ "lookup": lookup, "filters": filters }))).await
}

#[tauri::command]
async fn galaxy_systems(prefix: String) -> Result<Value, String> {
    forward("galaxy_systems", Some(json!({ "prefix": prefix }))).await
}

#[tauri::command]
async fn shell_info() -> Result<Value, String> {
    forward("shell_info", None).await
}

#[tauri::command]
async fn open_external(target: String) -> Result<Value, String> {
    forward("open_external", Some(json!({ "target": target }))).await
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
    blocking(|| Ok(status().ok())).await.ok().flatten()
}

/// Blocking: a voice tester live run waits for Elite to come to the front; calibration and the
/// mic check record for a few seconds.
#[tauri::command]
async fn core_call(cmd: String, text: Option<String>, args: Option<Value>) -> Value {
    blocking(move || {
        Ok(call_with(&cmd, text.as_deref(), args, &mut |_| {})
            .unwrap_or_else(|e| serde_json::json!({ "ok": false, "error": e })))
    })
    .await
    .unwrap_or_else(|e| serde_json::json!({ "ok": false, "error": e }))
}

fn route_arg(args: &[String]) -> Option<String> {
    let i = args.iter().position(|a| a == "--route")?;
    args.get(i + 1).filter(|r| r.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')).cloned()
}

/// `--webview-debug-port N`: open WebView2's DevTools port (127.0.0.1) for CI's onboarding
/// smoke test (scripts/onboarding_smoke.py). A flag, not an environment variable, so it can't
/// linger in a user's environment.
fn debug_port_arg(args: &[String]) -> Option<u16> {
    let i = args.iter().position(|a| a == "--webview-debug-port")?;
    args.get(i + 1)?.parse().ok()
}

/// When this process started, for the cold-start line (the WebView2 cost harness reads it).
static STARTED: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

pub fn main() {
    STARTED.get_or_init(std::time::Instant::now);
    // Release builds have no console, so the window's problems go to its own log in the core's
    // log folder (`astromech-shell.log`).
    let red = astromech_keybinds::report::Redactor::from_env();
    astromech_log::init(
        &Paths::current().log_dir(),
        astromech_log::SHELL,
        cfg!(debug_assertions),
        Box::new(move |l: &str| red.apply(l)),
    );
    let args: Vec<String> = std::env::args().collect();
    let route = route_arg(&args);
    let debug_port = debug_port_arg(&args);
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
            if let Some(r) = route_arg(&argv) {
                match app.emit("route", &r) {
                    Ok(()) => astromech_log::log(&format!("window: page {r} asked for")),
                    Err(e) => astromech_log::log(&format!("window: couldn't switch to page {r}: {e}")),
                }
            }
        }))
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
            engineering_stats,
            engineering_headlines,
            engineering_engineers,
            engineering_unlocks,
            engineering_trip,
            odyssey_view,
            odyssey_shopping,
            galaxy_nearby,
            galaxy_sites,
            colony_candidates,
            stations_nearby,
            galaxy_systems,
            trade_search,
            trade_anywhere,
            trade_cargo,
            mining_hotspots,
            market_compare,
            trade_route_defaults,
            trade_route,
            route_plot_ship,
            route_plot,
            neutron_file,
            neutron_download,
            neutron_check,
            colonisation_projects,
            colonisation_buy,
            search_filters,
            set_search_filters,
            powerplay_overview,
            powerplay_targets,
            logbook,
            docked_here,
            outfitting_search,
            session_summary,
            exploration_system,
            exploration_unsold
        ])
        .setup(move |app| {
            let hash = route.as_deref().map(|r| format!("index.html#/{r}")).unwrap_or_else(|| "index.html".into());
            let mut window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App(hash.into()))
                .title("astromech")
                .inner_size(1200.0, 800.0)
                .min_inner_size(800.0, 500.0)
                .on_page_load(|_, p| {
                    // The first page load is the end of a cold start (scripts/webview_cost.py).
                    static LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                    if p.event() == tauri::webview::PageLoadEvent::Finished
                        && !LOGGED.swap(true, std::sync::atomic::Ordering::SeqCst)
                    {
                        let ms = STARTED.get().map_or(0, |t| t.elapsed().as_millis());
                        astromech_log::log(&format!("window: page loaded {ms} ms after start"));
                    }
                });
            // CI's onboarding smoke test drives the page over the DevTools protocol. WebView2
            // ignores WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS when the app passes its own
            // arguments, so the port is asked for here; wry's defaults are kept.
            if let Some(port) = debug_port {
                window = window.additional_browser_args(&format!(
                    "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection \
                     --autoplay-policy=no-user-gesture-required --remote-debugging-port={port}"
                ));
            }
            window.build()?;
            // Off the setup path, which holds up the first page: a crashed core leaves core.json
            // behind, and connecting to its dead port takes about 2 s on Windows (astromech-k8yk).
            let h = app.handle().clone();
            std::thread::spawn(move || {
                if !core_alive() {
                    if let Err(e) = start_core() {
                        astromech_log::log(&format!("couldn't start the core: {e}"));
                    }
                }
                watchdog(h)
            });
            #[cfg(windows)]
            start_input(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the astromech app window");
}
