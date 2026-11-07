//! The window itself, built with the `window` feature (see main.rs).
//!
//! The app window (docs/V1-DESIGN.md §7.1): a thin Tauri v2 shell around the Svelte app in
//! `app/`. It holds no game logic. Every call the page makes goes through one Tauri command,
//! [`call`], to `astromech-core` over the core's local control API
//! (`%LOCALAPPDATA%\astromech\data\core.json`: port + token), and the shell:
//! - starts the core if it isn't running (the Start menu shortcut opens this window);
//! - restarts the core if it crashed while the window is open (§7.7 crash restart);
//! - is single-instance: a second start (e.g. from the tray) focuses this window and switches to
//!   the page given with `--route`.
//!
//! Closing the window exits this process and frees WebView2; the core keeps running.

use astromech_api::control;
use astromech_store::Paths;
use serde_json::Value;
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
fn core_alive() -> bool {
    control::call_short(&app_dir(), "status", None).is_ok()
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

/// The page's one way to the core: any command, forwarded with `origin: "app_window"` and no
/// read timeout (`control::call_window`). It resolves with the command's value and rejects with
/// its error, or why the core couldn't be reached; the command's progress values go to
/// `progress`. Off the main thread: a voice tester live run waits for Elite to come to the
/// front, calibration and the mic check record for a few seconds, a plot polls Spansh.
#[tauri::command]
async fn call(cmd: String, args: Option<Value>, progress: tauri::ipc::Channel<Value>) -> Result<Value, String> {
    tauri::async_runtime::spawn_blocking(move || {
        control::call_window(&app_dir(), &cmd, args, &mut |p| {
            let _ = progress.send(p);
        })
    })
    .await
    .map_err(|e| e.to_string())?
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
        .invoke_handler(tauri::generate_handler![call])
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
