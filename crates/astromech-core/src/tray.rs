//! The tray icon and its native menu (docs/V1-DESIGN.md §7.1, §2.2): status line, Open
//! astromech, Keybinds, Settings, Pause voice, Kill switch, Start with Windows, Check for
//! updates…, Save diagnostics…, Quit. Left-click opens the window. Runs a Win32 message loop
//! on the calling (main) thread.

use crate::app::{self, Core, Ui};
use crate::logging::log;
use crate::session::VOICE_PAUSED;
use anyhow::{Context, Result};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

const ICON_PNG: &[u8] = include_bytes!("../../../app/src-tauri/icons/32x32.png");

fn icon() -> Result<tray_icon::Icon> {
    let mut d = png::Decoder::new(std::io::Cursor::new(ICON_PNG));
    d.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA);
    let mut r = d.read_info()?;
    let mut buf = vec![0; r.output_buffer_size().context("icon size")?];
    let info = r.next_frame(&mut buf)?;
    buf.truncate(info.buffer_size());
    Ok(tray_icon::Icon::from_rgba(buf, info.width, info.height)?)
}

pub fn run(core: Arc<Core>, rx: &Receiver<Ui>) -> Result<()> {
    let status = MenuItem::new(core.status_line(), false, None);
    let crash = MenuItem::new("astromech crashed last time · Save diagnostics", true, None);
    let open = MenuItem::new("Open astromech", true, None);
    let keybinds = MenuItem::new("Keybinds", true, None);
    let settings = MenuItem::new("Settings", true, None);
    let pause = CheckMenuItem::new("Pause voice", true, VOICE_PAUSED.load(Ordering::SeqCst), None);
    let kill = MenuItem::new("Kill switch (stop all actions)", false, None);
    let autostart = CheckMenuItem::new("Start with Windows", true, crate::autostart::is_enabled(), None);
    let config = MenuItem::new("Open config file", true, None);
    let models = MenuItem::new("Download speech models (about 800 MB)", true, None);
    let updates = MenuItem::new("Check for updates…", true, None);
    let install = MenuItem::new("Install update", true, None);
    let diag = MenuItem::new("Save diagnostics…", true, None);
    let quit = MenuItem::new("Quit", true, None);
    let sep = PredefinedMenuItem::separator;

    let build = |crash_pending: bool, models_missing: bool, update_ready: bool| -> Result<Menu> {
        let m = Menu::new();
        m.append(&status)?;
        if crash_pending {
            m.append(&crash)?;
        }
        m.append_items(&[&sep(), &open, &keybinds, &settings, &sep(), &pause, &kill, &sep()])?;
        m.append(&autostart)?;
        m.append(&config)?;
        if models_missing {
            m.append(&models)?;
        }
        m.append(&updates)?;
        if update_ready {
            m.append(&install)?;
        }
        m.append_items(&[&diag, &sep(), &quit])?;
        Ok(m)
    };
    let mut shape =
        (core.crash.lock().unwrap().is_some(), app::models_missing(&core.cfg), core.update.lock().unwrap().snap.ready);
    let tray = TrayIconBuilder::new()
        .with_tooltip("astromech")
        .with_icon(icon()?)
        .with_menu(Box::new(build(shape.0, shape.1, shape.2)?))
        .with_menu_on_left_click(false)
        .build()
        .context("creating the tray icon")?;
    log("tray icon ready");

    let refresh = |shape: &mut (bool, bool, bool)| -> Result<()> {
        let line = core.status_line();
        status.set_text(&line);
        let _ = tray.set_tooltip(Some(line.replace("astromech · ", "astromech\n")));
        let active = core.active();
        let killed = core.session.lock().unwrap().as_ref().map(|s| s.runner.policy.is_killed()).unwrap_or(false);
        kill.set_enabled(active);
        kill.set_text(if killed { "Re-arm (kill switch is on)" } else { "Kill switch (stop all actions)" });
        pause.set_checked(VOICE_PAUSED.load(Ordering::SeqCst));
        autostart.set_checked(crate::autostart::is_enabled());
        let downloading = core.models.lock().unwrap().is_some();
        models.set_enabled(!downloading);
        let snap = core.update.lock().unwrap().snap.clone();
        updates.set_text(match snap.status {
            crate::update::Status::Checking => "Checking for updates…",
            crate::update::Status::Downloading => "Downloading update…",
            crate::update::Status::Applying => "Installing update…",
            _ => "Check for updates…",
        });
        updates.set_enabled(!matches!(
            snap.status,
            crate::update::Status::Checking | crate::update::Status::Downloading | crate::update::Status::Applying
        ));
        if let Some(v) = &snap.latest {
            install.set_text(format!("Install update {v}"));
        }
        let now = (core.crash.lock().unwrap().is_some(), app::models_missing(&core.cfg), snap.ready);
        if now != *shape {
            *shape = now;
            tray.set_menu(Some(Box::new(build(now.0, now.1, now.2)?)));
        }
        Ok(())
    };

    loop {
        pump(std::time::Duration::from_millis(100));
        let mut dirty = false;
        while let Ok(e) = TrayIconEvent::receiver().try_recv() {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                let _ = core.open_window(None);
            }
        }
        while let Ok(e) = MenuEvent::receiver().try_recv() {
            dirty = true;
            let id = e.id();
            if id == open.id() {
                let _ = core.open_window(None);
            } else if id == keybinds.id() {
                let _ = core.open_window(Some("keybinds"));
            } else if id == settings.id() {
                let _ = core.open_window(Some("settings"));
            } else if id == pause.id() {
                let on = pause.is_checked();
                VOICE_PAUSED.store(on, Ordering::SeqCst);
                log(if on { "voice paused (tray)" } else { "voice resumed (tray)" });
            } else if id == kill.id() {
                core.kill_switch();
            } else if id == autostart.id() {
                let on = autostart.is_checked();
                let r =
                    std::env::current_exe().map_err(|e| e.to_string()).and_then(|exe| crate::autostart::set(on, &exe));
                match r {
                    Ok(()) => log(if on { "start with Windows: on" } else { "start with Windows: off" }),
                    Err(e) => log(&format!("start with Windows: couldn't change it: {e}")),
                }
            } else if id == config.id() {
                app::open_config(&core.config_path);
            } else if id == models.id() {
                app::download_models(&core);
            } else if id == updates.id() {
                let c = core.clone();
                std::thread::spawn(move || c.check_update(true));
            } else if id == install.id() {
                let _ = core.install_update();
            } else if id == diag.id() || id == crash.id() {
                match core.save_diagnostics() {
                    Ok(p) => app::reveal(&p),
                    Err(e) => log(&format!("couldn't save diagnostics: {e:#}")),
                }
            } else if id == quit.id() {
                return Ok(());
            }
        }
        while let Ok(m) = rx.try_recv() {
            match m {
                Ui::Quit => return Ok(()),
                Ui::Refresh => dirty = true,
            }
        }
        if dirty {
            if let Err(e) = refresh(&mut shape) {
                log(&format!("tray menu update failed: {e:#}"));
            }
        }
    }
}

/// Dispatch pending window messages, then wait up to `max` for more.
fn pump(max: std::time::Duration) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, MsgWaitForMultipleObjects, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, QS_ALLINPUT,
    };
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        MsgWaitForMultipleObjects(0, std::ptr::null(), 0, max.as_millis() as u32, QS_ALLINPUT);
    }
}
