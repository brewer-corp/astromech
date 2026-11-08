//! The Tauri mode: `"windows": []`, a tray shaped like crates/astromech-core/src/tray.rs, the
//! window built on demand, and the control commands the probe drives.

use crate::control::{self, Handler};
use crate::win::{self, log, Opts, Sink, STARTED};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::webview::PageLoadEvent;
use tauri::{AppHandle, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder, WindowEvent, Wry};

/// The probe finds the window by this title, as a click on X would.
const TITLE: &str = "astromech spike";

static BEATS: AtomicU64 = AtomicU64::new(0);
static LAST_BEAT: Mutex<Option<Instant>> = Mutex::new(None);

/// The page's heartbeat (dist/index.html), once a second while it is alive.
#[tauri::command]
fn beat() {
    BEATS.fetch_add(1, Ordering::SeqCst);
    *LAST_BEAT.lock().unwrap() = Some(Instant::now());
}

/// tray.rs's shape: a status line, Open, a Pause check item, an optional line, and Quit.
struct Tray {
    icon: TrayIcon<Wry>,
    status: MenuItem<Wry>,
    open: MenuItem<Wry>,
    pause: CheckMenuItem<Wry>,
    line: MenuItem<Wry>,
    quit: MenuItem<Wry>,
}

fn menu(
    app: &AppHandle,
    status: &MenuItem<Wry>,
    open: &MenuItem<Wry>,
    pause: &CheckMenuItem<Wry>,
    line: Option<&MenuItem<Wry>>,
    quit: &MenuItem<Wry>,
) -> tauri::Result<Menu<Wry>> {
    let m = Menu::new(app)?;
    m.append(status)?;
    if let Some(l) = line {
        m.append(l)?;
    }
    m.append(&PredefinedMenuItem::separator(app)?)?;
    m.append(open)?;
    m.append(&PredefinedMenuItem::separator(app)?)?;
    m.append(pause)?;
    m.append(&PredefinedMenuItem::separator(app)?)?;
    m.append(quit)?;
    Ok(m)
}

fn build_tray(app: &AppHandle) -> tauri::Result<Tray> {
    let status = MenuItem::with_id(app, "status", "astromech: ready", false, None::<&str>)?;
    let open = MenuItem::with_id(app, "open", "Open astromech", true, None::<&str>)?;
    let pause = CheckMenuItem::with_id(app, "pause", "Pause voice", true, false, None::<&str>)?;
    let line = MenuItem::with_id(app, "line", "Finish setup…", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit astromech", true, None::<&str>)?;
    let m = menu(app, &status, &open, &pause, None, &quit)?;
    let mut b = TrayIconBuilder::with_id("main")
        .tooltip("astromech spike")
        .menu(&m)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, e| {
            let app = app.clone();
            match e.id().as_ref() {
                // Off the main thread: building a webview from a main-thread handler can deadlock on Windows.
                "open" | "line" => {
                    std::thread::spawn(move || {
                        let _ = open_window(&app, None);
                    });
                }
                "pause" => log("tray: pause toggled"),
                "quit" => app.exit(0),
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, e| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                let app = tray.app_handle().clone();
                std::thread::spawn(move || {
                    let _ = open_window(&app, None);
                });
            }
        });
    if let Some(i) = app.default_window_icon() {
        b = b.icon(i.clone());
    }
    let icon = b.build(app)?;
    log("tray icon ready");
    Ok(Tray { icon, status, open, pause, line, quit })
}

/// Log WebView2's ProcessFailed events without acting on them: what the spike observes, not a
/// recovery.
fn observe_process_failed(w: &tauri::WebviewWindow) {
    let r = w.with_webview(|pw| unsafe {
        use webview2_com::Microsoft::Web::WebView2::Win32::COREWEBVIEW2_PROCESS_FAILED_KIND;
        use webview2_com::ProcessFailedEventHandler;
        let Ok(core) = pw.controller().CoreWebView2() else {
            log("window: no CoreWebView2 to observe");
            return;
        };
        let h = ProcessFailedEventHandler::create(Box::new(|_, args| {
            let mut kind = COREWEBVIEW2_PROCESS_FAILED_KIND(-1);
            if let Some(a) = args {
                let _ = a.ProcessFailedKind(&mut kind);
            }
            let name = match kind.0 {
                0 => "browser process exited",
                1 => "render process exited",
                2 => "render process unresponsive",
                3 => "frame render process exited",
                4 => "utility process exited",
                5 => "sandbox helper process exited",
                6 => "GPU process exited",
                7 => "PPAPI plugin process exited",
                8 => "PPAPI broker process exited",
                _ => "unknown",
            };
            log(&format!("window: WebView2 process failed: kind {} ({name})", kind.0));
            Ok(())
        }));
        let mut token = 0i64;
        if let Err(e) = core.add_ProcessFailed(&h, &mut token) {
            log(&format!("window: couldn't observe ProcessFailed: {e}"));
        }
    });
    if let Err(e) = r {
        log(&format!("window: with_webview failed: {e}"));
    }
}

/// Focus the window, or build it at `route`. Called off the main thread.
fn open_window(app: &AppHandle, route: Option<&str>) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
        if let Some(r) = route {
            let _ = w.eval(format!("location.hash = {}", json!(format!("#/{r}"))));
            log(&format!("window: page {r} asked for"));
        }
        return Ok(());
    }
    let page = route.map(|r| format!("index.html#/{r}")).unwrap_or_else(|| "index.html".into());
    let w = WebviewWindowBuilder::new(app, "main", WebviewUrl::App(page.into()))
        .title(TITLE)
        .inner_size(900.0, 600.0)
        // WebView2 makes its EBWebView folder in here, which the probe matches command lines on.
        .data_directory(win::dir())
        .on_page_load(|_, p| {
            if p.event() == PageLoadEvent::Finished {
                let ms = STARTED.get().map_or(0, |t| t.elapsed().as_millis());
                log(&format!("window: page loaded {ms} ms after start"));
            }
        })
        .build()
        .map_err(|e| format!("building the window: {e}"))?;
    log(&format!("window: built at {route:?}"));
    observe_process_failed(&w);
    Ok(())
}

fn churn(app: &AppHandle, t: &Tray) -> Result<Value, String> {
    let t0 = Instant::now();
    let e = |e: tauri::Error| e.to_string();
    for i in 0..50 {
        t.status.set_text(format!("astromech: churn {i}")).map_err(e)?;
        t.line.set_text(format!("Finish setup: step {i}…")).map_err(e)?;
        t.open.set_enabled(true).map_err(e)?;
        t.quit.set_enabled(i % 3 != 0).map_err(e)?;
        t.pause.set_checked(i % 2 == 0).map_err(e)?;
        let line = (i % 2 == 0).then_some(&t.line);
        let m = menu(app, &t.status, &t.open, &t.pause, line, &t.quit).map_err(e)?;
        t.icon.set_menu(Some(m)).map_err(e)?;
        t.icon.set_tooltip(Some(format!("astromech spike\nchurn {i}"))).map_err(e)?;
    }
    t.quit.set_enabled(true).map_err(e)?;
    Ok(json!({ "iterations": 50, "ms": t0.elapsed().as_millis() as u64 }))
}

fn handler(app: AppHandle, tray: Arc<Tray>, crash_hook: bool) -> Handler {
    Arc::new(move |cmd, args| match cmd {
        "ui_state" => {
            let mut v = win::base_state();
            v["window"] = json!(if app.get_webview_window("main").is_some() { "open" } else { "closed" });
            // A main-thread round trip: Tauri runs rect() on the main thread and waits for it.
            let rect = tray.icon.rect();
            v["tray"] = json!(matches!(rect, Ok(Some(_))));
            v["tray_rect"] = json!(format!("{rect:?}"));
            v["tray_built"] = json!(true);
            v["page_beats"] = json!(BEATS.load(Ordering::SeqCst));
            v["last_beat_ms_ago"] = json!(LAST_BEAT.lock().unwrap().map(|t| t.elapsed().as_millis() as u64));
            Ok(v)
        }
        "open_window" => {
            let route = args.as_ref().and_then(|a| a["route"].as_str());
            open_window(&app, route).map(|()| json!(null))
        }
        "tray_churn" => churn(&app, &tray),
        "quit" => {
            let app = app.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(200));
                log("quit: app.exit(0) from a worker thread");
                app.exit(0);
            });
            Ok(json!(null))
        }
        "crash" if crash_hook => {
            let kind = args.as_ref().and_then(|a| a["kind"].as_str()).unwrap_or("").to_string();
            if kind == "main_thread_panic" {
                let app = app.clone();
                log("crash: main_thread_panic in 300 ms");
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(300));
                    let _ = app.run_on_main_thread(|| panic!("spike: a panic on the main thread"));
                });
                Ok(json!(null))
            } else {
                win::crash_soon(&kind).map(|()| json!(null))
            }
        }
        "crash" => Err("crash needs --crash-hook".into()),
        _ => Err(format!("unknown command {cmd}")),
    })
}

pub fn run(opts: Opts) {
    if opts.sink == Sink::BeforeBuild {
        win::start_sink("before Builder::build");
    }
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![beat])
        .build(tauri::generate_context!())
        .expect("building the Tauri app");
    // tao turns dark mode on for the app when it builds its event loop
    // (tao-0.37.1 platform_impl/windows/event_loop.rs:190), so the menus go dark after that.
    win::dark_menus();
    if opts.sink == Sink::AfterBuild {
        win::start_sink("after Builder::build");
    }
    let handle = app.handle().clone();
    let tray = Arc::new(build_tray(&handle).expect("building the tray"));
    if let Err(e) = control::serve(handler(handle.clone(), tray.clone(), opts.crash_hook)) {
        log(&format!("control: couldn't listen: {e}"));
    }
    if !opts.background {
        let h = handle.clone();
        let route = opts.route.clone();
        std::thread::spawn(move || {
            if let Err(e) = open_window(&h, route.as_deref()) {
                log(&format!("window: {e}"));
            }
        });
    }
    log(&format!("ready: background={} restarted={} sink={:?}", opts.background, opts.restarted, opts.sink));
    app.run(move |_, e| match e {
        // Closing the last window asks to exit with no code; the tray keeps the process.
        RunEvent::ExitRequested { code: None, api, .. } => {
            log("exit requested with no code (last window closed): prevented");
            api.prevent_exit();
        }
        RunEvent::ExitRequested { code: Some(c), .. } => log(&format!("exit requested with code {c}")),
        RunEvent::Exit => log("exit"),
        RunEvent::WindowEvent { label, event: WindowEvent::Destroyed, .. } => {
            log(&format!("window: {label} destroyed"))
        }
        _ => {}
    });
}
