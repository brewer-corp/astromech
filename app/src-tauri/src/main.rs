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
    let t = std::fs::read_to_string(endpoint_file()).map_err(|_| "the core isn't running".to_string())?;
    let ep: Endpoint = serde_json::from_str(&t).map_err(|e| e.to_string())?;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], ep.port));
    let mut s = TcpStream::connect_timeout(&addr, Duration::from_secs(2)).map_err(|e| e.to_string())?;
    s.set_read_timeout(Some(Duration::from_secs(15))).map_err(|e| e.to_string())?;
    let req = serde_json::json!({ "token": ep.token, "cmd": cmd });
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

#[tauri::command]
fn core_status() -> Option<Value> {
    call("status").ok()
}

#[tauri::command]
fn core_call(cmd: String) -> Value {
    // The window may only ask for these.
    if !matches!(cmd.as_str(), "status" | "save_diagnostics" | "ack_crash") {
        return serde_json::json!({ "ok": false, "error": "not allowed" });
    }
    call(&cmd).unwrap_or_else(|e| serde_json::json!({ "ok": false, "error": e }))
}

#[tauri::command]
fn shell_info() -> Value {
    serde_json::json!({ "version": env!("CARGO_PKG_VERSION") })
}

/// Open the project's GitHub pages in the browser, or a folder under the app folder.
#[tauri::command]
fn open_external(target: String) -> Result<(), String> {
    let ok_url = target.starts_with("https://github.com/spadaval/astromech");
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
        .invoke_handler(tauri::generate_handler![core_status, core_call, shell_info, open_external])
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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the astromech app window");
}
