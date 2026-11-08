//! The Tauri host: the one app's startup, its window and its [`Ui`] for the core.
//!
//! [`run`] is the installed app's run on Tauri's event loop, built on `astromech_core::app`:
//! [`app::begin`] (logging, the crash hook, the single-instance mutex), then Tauri's
//! `Builder::build`, then [`app::boot`] (the core), the tray, the window unless `--background`,
//! and the event loop until Quit, after which [`app::shutdown`] runs in `RunEvent::Exit`.
//!
//! The window (docs/V1-DESIGN.md §7.1) is built when asked for and destroyed when closed, which
//! ends its WebView2 processes; the app stays in the tray. Every call the page makes goes
//! through one Tauri command, [`call`], into the core's command registry in this process.

use crate::tray;
use astromech_api::{control, Registry};
use astromech_core::app::{self, Core, Opts, Ui, UiState, WindowState};
use astromech_core::logging::log;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};
#[cfg(windows)]
use std::time::Duration;
use std::time::Instant;
#[cfg(windows)]
use tauri::WebviewWindow;
use tauri::{AppHandle, Emitter, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder, WindowEvent};

/// The window's label and title; scripts/webview_cost.py and the onboarding smoke find
/// the window by the title, as a click on X would close it.
const LABEL: &str = "main";
const TITLE: &str = "astromech";

/// When this process started, for the page-load line (scripts/webview_cost.py reads it).
static STARTED: OnceLock<Instant> = OnceLock::new();

/// The core's commands, for [`call`].
struct Commands(Arc<Registry>);

/// The page's one way to the core: any `window` command, dispatched in process with `origin:
/// "app_window"` (`control::window_request`: a string `args` is the request's text). It resolves
/// with the command's value and rejects with its error; the command's progress values go to
/// `progress`. Off the main thread: a voice tester live run waits for Elite to come to the
/// front, calibration and the mic check record for a few seconds, a plot polls Spansh.
#[tauri::command]
async fn call(
    commands: tauri::State<'_, Commands>,
    cmd: String,
    args: Option<Value>,
    progress: tauri::ipc::Channel<Value>,
) -> Result<Value, String> {
    let reg = commands.0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        reg.dispatch(&control::window_request(&cmd, args), &|p| {
            let _ = progress.send(p);
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

/// What the host keeps: the app, the core once it is built, the tray once it is, and the
/// window's press listener.
pub struct Host {
    app: AppHandle,
    core: OnceLock<Weak<Core>>,
    tray: OnceLock<tray::Tray>,
    /// A tray refresh is already queued on the main thread.
    refresh_queued: AtomicBool,
    debug_port: Option<u16>,
    /// One window at a time: building it, or finding it to focus.
    opening: Mutex<()>,
    /// The window's press listener runs until this is set (when the window is destroyed).
    input_stop: Mutex<Option<Arc<AtomicBool>>>,
}

impl Host {
    pub fn core(&self) -> Option<Arc<Core>> {
        self.core.get().and_then(Weak::upgrade)
    }

    pub fn tray(&self) -> Option<&tray::Tray> {
        self.tray.get()
    }

    /// Show the core's state in the tray, on the main thread. The menu is read first, so no
    /// menu call runs while one of the core's locks is held.
    fn refresh_tray(&self) {
        self.refresh_queued.store(false, Ordering::SeqCst);
        let (Some(core), Some(t)) = (self.core(), self.tray.get()) else { return };
        let m = astromech_core::tray::read(&core);
        if let Err(e) = t.show(&self.app, &m) {
            log(&format!("tray menu update failed: {e:#}"));
        }
    }

    /// Focus the window and switch it to `route`, or build it at `route`. Off the main thread:
    /// building a webview from a main-thread handler can deadlock on Windows.
    fn open_window(self: &Arc<Self>, route: Option<&str>) -> Result<(), String> {
        let _one = self.opening.lock().unwrap_or_else(|e| e.into_inner());
        let route = route.filter(|r| !r.is_empty() && r.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
        if let Some(w) = self.app.get_webview_window(LABEL) {
            let _ = w.unminimize();
            let _ = w.show();
            let _ = w.set_focus();
            if let Some(r) = route {
                match self.app.emit("route", r) {
                    Ok(()) => log(&format!("window: page {r} asked for")),
                    Err(e) => log(&format!("window: couldn't switch to page {r}: {e}")),
                }
            }
            return Ok(());
        }
        let page = route.map(|r| format!("index.html#/{r}")).unwrap_or_else(|| "index.html".into());
        // The first page load of each window ends a cold start (scripts/webview_cost.py).
        let loaded = AtomicBool::new(false);
        let mut b = WebviewWindowBuilder::new(&self.app, LABEL, WebviewUrl::App(page.into()))
            .title(TITLE)
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 500.0)
            .on_page_load(move |_, p| {
                if p.event() == tauri::webview::PageLoadEvent::Finished && !loaded.swap(true, Ordering::SeqCst) {
                    // Real time: the cold-start line measures this process's own start.
                    #[allow(clippy::disallowed_methods)]
                    let ms = STARTED.get().map_or(0, |t| t.elapsed().as_millis());
                    log(&format!("window: page loaded {ms} ms after start"));
                }
            });
        // CI's onboarding smoke test drives the page over the DevTools protocol. WebView2
        // ignores WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS when the app passes its own
        // arguments, so the port is asked for here; wry's defaults are kept.
        if let Some(port) = self.debug_port {
            b = b.additional_browser_args(&format!(
                "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection \
                 --autoplay-policy=no-user-gesture-required --remote-debugging-port={port}"
            ));
        }
        let w = b.build().map_err(|e| format!("building the window: {e}"))?;
        #[cfg(windows)]
        self.watch_process_failed(&w);
        #[cfg(not(windows))]
        let _ = w;
        self.start_input();
        Ok(())
    }

    /// The window was destroyed (closed, or rebuilt after a crash): its WebView2 processes are
    /// ending, and its press listener stops.
    fn window_destroyed(&self) {
        log("window: closed");
        if let Some(stop) = self.input_stop.lock().unwrap_or_else(|e| e.into_inner()).take() {
            stop.store(true, Ordering::SeqCst);
        }
    }

    /// While the window exists, presses from every stick, pad and key go to the page, for
    /// "press a control to find it", the key tester and press-to-bind: a subscriber to the
    /// process's one input source (`astromech_input::listen::source`), which push-to-talk and
    /// the kill switch share. Read-only.
    fn start_input(&self) {
        let stop = Arc::new(AtomicBool::new(false));
        if let Some(old) = self.input_stop.lock().unwrap_or_else(|e| e.into_inner()).replace(stop.clone()) {
            old.store(true, Ordering::SeqCst);
        }
        #[cfg(windows)]
        {
            let app = self.app.clone();
            std::thread::Builder::new()
                .name("window-input".into())
                .spawn(move || press_listener(app, &stop))
                .expect("spawn the window's press listener");
        }
    }

    /// Recover the window in place when WebView2's processes die (spike astromech-wgtr): after
    /// the page's renderer exits, WebView2 shows its "This page is having a problem" page with
    /// the browser process still up, so reload; after the browser process exits, the window is
    /// blank with no WebView2 left, so rebuild it.
    #[cfg(windows)]
    fn watch_process_failed(self: &Arc<Self>, w: &WebviewWindow) {
        let host = Arc::downgrade(self);
        let r = w.with_webview(move |pw| unsafe {
            use webview2_com::Microsoft::Web::WebView2::Win32::COREWEBVIEW2_PROCESS_FAILED_KIND;
            use webview2_com::ProcessFailedEventHandler;
            let Ok(core) = pw.controller().CoreWebView2() else {
                log("window: no CoreWebView2 to watch for process failures");
                return;
            };
            let h = ProcessFailedEventHandler::create(Box::new(move |_, args| {
                let mut kind = COREWEBVIEW2_PROCESS_FAILED_KIND(-1);
                if let Some(a) = args {
                    let _ = a.ProcessFailedKind(&mut kind);
                }
                if let Some(host) = host.upgrade() {
                    host.process_failed(kind.0);
                }
                Ok(())
            }));
            let mut token = 0i64;
            if let Err(e) = core.add_ProcessFailed(&h, &mut token) {
                log(&format!("window: couldn't watch for WebView2 process failures: {e}"));
            }
        });
        if let Err(e) = r {
            log(&format!("window: couldn't reach its webview: {e}"));
        }
    }

    /// WebView2's ProcessFailed, on the main thread.
    #[cfg(windows)]
    fn process_failed(self: &Arc<Self>, kind: i32) {
        const BROWSER_EXITED: i32 = 0;
        const RENDER_EXITED: i32 = 1;
        let what = match kind {
            BROWSER_EXITED => "browser process exited",
            RENDER_EXITED => "render process exited",
            2 => "render process unresponsive",
            3 => "frame render process exited",
            4 => "utility process exited",
            6 => "GPU process exited",
            _ => "other",
        };
        log(&format!("window: WebView2 process failed: kind {kind} ({what})"));
        match kind {
            // Off this WebView2 callback, so Tauri runs the reload from its own loop.
            RENDER_EXITED => {
                let app = self.app.clone();
                std::thread::spawn(move || match app.get_webview_window(LABEL).map(|w| w.reload()) {
                    Some(Ok(())) => log("window: reloading the page"),
                    Some(Err(e)) => log(&format!("window: couldn't reload the page: {e}")),
                    None => {}
                });
            }
            BROWSER_EXITED => {
                let host = self.clone();
                std::thread::spawn(move || host.rebuild_window());
            }
            _ => {}
        }
    }

    /// Destroy the window whose webview died and build it again at the page it showed.
    #[cfg(windows)]
    // Real time: a short wait for Tauri to finish destroying the window, which ends on its own;
    // the installed app's window only, which world mode never builds.
    #[allow(clippy::disallowed_methods)]
    fn rebuild_window(self: &Arc<Self>) {
        let Some(w) = self.app.get_webview_window(LABEL) else { return };
        let route = w
            .url()
            .ok()
            .and_then(|u| u.fragment().map(|f| f.trim_start_matches('/').to_string()))
            .filter(|r| !r.is_empty());
        if let Err(e) = w.destroy() {
            log(&format!("window: couldn't destroy the dead window: {e}"));
        }
        let t0 = Instant::now();
        while self.app.get_webview_window(LABEL).is_some() && t0.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(50));
        }
        match self.open_window(route.as_deref()) {
            Ok(()) => log(&format!("window: rebuilt{}", route.map(|r| format!(" at {r}")).unwrap_or_default())),
            Err(e) => log(&format!("window: couldn't rebuild it: {e}")),
        }
    }
}

/// The page's presses, until `stop` is set: the window's subscription to the input source.
#[cfg(windows)]
#[allow(clippy::disallowed_methods, reason = "raw-input and XInput sample timing, against the source's epoch")]
fn press_listener(app: AppHandle, stop: &AtomicBool) {
    use astromech_input::listen::{source, tracker::Processor, win};
    let mut p = Processor::new(astromech_input::ELITE_TITLE, None, true);
    for d in win::rawinput::list_devices() {
        p.add_device(d);
    }
    let source::Subscription { rx, epoch: start, .. } = source::subscribe();
    while !stop.load(Ordering::SeqCst) {
        let evs = match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(source::Msg::Device(d)) => {
                p.add_device(d);
                continue;
            }
            Ok(source::Msg::Sample(s)) => p.process(&s),
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
    // Dropping the receiver unsubscribes: the source forgets a subscriber whose send fails.
}

/// The core's [`Ui`]: the tray and the window on Tauri.
struct TauriUi(Arc<Host>);

impl Ui for TauriUi {
    fn refresh(&self) {
        if self.0.refresh_queued.swap(true, Ordering::SeqCst) {
            return;
        }
        let h = self.0.clone();
        if self.0.app.run_on_main_thread(move || h.refresh_tray()).is_err() {
            self.0.refresh_queued.store(false, Ordering::SeqCst);
        }
    }

    /// Answers once the open is asked for, without waiting for the window. Building it waits
    /// for WebView2, whose first start on a machine can take many seconds (13 s on a CI runner),
    /// longer than a second start waits for the control API's reply. The build runs on its own
    /// thread, which hands it to the main thread (building from a main-thread handler can
    /// deadlock on Windows); a failed build is logged there. Callers that need the window wait
    /// for `ui_state` or the page-load line.
    fn open_window(&self, route: Option<&str>) -> Result<(), String> {
        let (h, route) = (self.0.clone(), route.map(String::from));
        std::thread::Builder::new()
            .name("window-open".into())
            .spawn(move || {
                if let Err(e) = h.open_window(route.as_deref()) {
                    log(&format!("couldn't open the app window: {e}"));
                }
            })
            .map(|_| ())
            .map_err(|e| format!("couldn't start opening the window: {e}"))
    }

    fn window_open(&self) -> bool {
        self.0.app.get_webview_window(LABEL).is_some()
    }

    fn state(&self) -> UiState {
        // TrayIcon::rect runs on the main thread, so this is also a main-thread round trip.
        let tray = self.0.tray.get().is_some_and(|t| matches!(t.icon.rect(), Ok(Some(_))));
        let w = self.0.app.get_webview_window(LABEL);
        let page = w.as_ref().and_then(|w| w.url().ok()).and_then(|u| app::page_of(u.as_str()));
        UiState { window: if w.is_some() { WindowState::Open } else { WindowState::Closed }, tray, page }
    }

    fn quit(&self) {
        log("quit: ending the event loop");
        self.0.app.exit(0);
    }
}

/// Run the installed app on Tauri's event loop until Quit. Returns the exit code when it
/// doesn't start; after Quit the process exits from the event loop with 0.
// Real time: the cold-start line measures this process's own start (scripts/webview_cost.py
// reads it).
#[allow(clippy::disallowed_methods)]
pub fn run(opts: Opts) -> i32 {
    STARTED.get_or_init(Instant::now);
    let Some(begun) = app::begin(&opts) else { return 0 };
    let tauri_app = match tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![call])
        .build(tauri::generate_context!())
    {
        Ok(a) => a,
        Err(e) => {
            log(&format!("couldn't start the app: {e}"));
            return 1;
        }
    };
    // tao turns dark mode on for the app when it builds its event loop (tao-0.37.1
    // platform_impl/windows/event_loop.rs:190), so the menus go dark after it.
    #[cfg(windows)]
    tray::dark_menus();
    let host = Arc::new(Host {
        app: tauri_app.handle().clone(),
        core: OnceLock::new(),
        tray: OnceLock::new(),
        refresh_queued: AtomicBool::new(false),
        debug_port: opts.webview_debug_port,
        opening: Mutex::new(()),
        input_stop: Mutex::new(None),
    });
    // The core starts only now, after Builder::build. tao registers keyboard and mouse Raw
    // Input to its own hidden window while it builds the loop, and a process keeps only the
    // last registration per device class, so the process's one input source must register
    // after it (spike astromech-wgtr). Nothing subscribes to the source before this line: the
    // session's listener starts with the core's game watch, and the window's with the window.
    let booted = app::boot(&opts, &begun, Arc::new(TauriUi(host.clone())));
    let _ = host.core.set(Arc::downgrade(&booted.core));
    tauri_app.manage(Commands(booted.started.commands.clone()));
    match tray::build(tauri_app.handle(), host.clone(), &astromech_core::tray::read(&booted.core)) {
        Ok(t) => {
            let _ = host.tray.set(t);
            log("tray icon ready");
        }
        Err(e) => log(&format!("tray icon failed: {e:#}; running without it until Quit (via the app window)")),
    }
    app::open_at_start(&opts, &booted.core);

    let mut held = Some((booted, begun));
    tauri_app.run(move |_, e| match e {
        // Closing the window asks to exit with no code; the app stays in the tray.
        RunEvent::ExitRequested { code: None, api, .. } => api.prevent_exit(),
        RunEvent::WindowEvent { label, event: WindowEvent::Destroyed, .. } if label == LABEL => host.window_destroyed(),
        // Quit (the tray, the `quit` command, an update's restart): the session ends and held
        // keys are released, then the control API's endpoint file goes and the mutex with it.
        RunEvent::Exit => {
            if let Some((booted, begun)) = held.take() {
                app::shutdown(&booted.core);
                drop(booted);
                drop(begun);
            }
        }
        _ => {}
    });
    0
}
