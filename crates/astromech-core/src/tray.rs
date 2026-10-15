//! The tray icon and its native menu (docs/V1-DESIGN.md §7.1, §2.2): a remote for the core
//! while you play. Status line, Open astromech, Pause voice, Kill switch (while active), Quit,
//! plus one line each when something needs attention (a crash, unfinished setup, an update
//! ready to install). Everything else lives in the window. Left-click opens the window. Runs a
//! Win32 message loop on the calling (main) thread.

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

/// Dark context menus, to match the window. Windows has no public API for this; uxtheme's
/// `SetPreferredAppMode` (ordinal 135, Windows 10 1903+) and `FlushMenuThemes` (136) are what
/// Explorer and most apps use. On older builds 135 is `AllowDarkModeForApp`, which also takes a
/// non-zero argument; without either the menu stays light.
fn dark_menus() {
    use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
    type Proc = unsafe extern "system" fn() -> isize;
    const FORCE_DARK: i32 = 2;
    let name: Vec<u16> = "uxtheme.dll".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let h = LoadLibraryW(name.as_ptr());
        if h.is_null() {
            return;
        }
        if let Some(f) = GetProcAddress(h, 135usize as *const u8) {
            let set_mode = std::mem::transmute::<Proc, unsafe extern "system" fn(i32) -> i32>(f);
            set_mode(FORCE_DARK);
        }
        if let Some(f) = GetProcAddress(h, 136usize as *const u8) {
            let flush = std::mem::transmute::<Proc, unsafe extern "system" fn()>(f);
            flush();
        }
    }
}

/// Which optional lines the menu has; it is rebuilt when this changes.
#[derive(Clone, PartialEq)]
struct Shape {
    crash: bool,
    setup: bool,
    active: bool,
    update: bool,
}

fn shape(core: &Core) -> Shape {
    Shape {
        crash: core.crash.lock().unwrap().is_some(),
        setup: !core.setup_needed().is_empty(),
        active: core.active(),
        update: core.update.lock().unwrap().snap.ready,
    }
}

fn setup_text(core: &Core) -> String {
    match core.setup_needed().first() {
        Some(what) => format!("Finish setup: {what}…"),
        None => "Finish setup…".into(),
    }
}

fn update_text(core: &Core) -> String {
    match &core.update.lock().unwrap().snap.latest {
        Some(v) => format!("Restart to update to {v}"),
        None => "Restart to update".into(),
    }
}

pub fn run(core: Arc<Core>, rx: &Receiver<Ui>) -> Result<()> {
    dark_menus();
    let status = MenuItem::new(core.status_line(), false, None);
    let crash = MenuItem::new("astromech crashed last time: save diagnostics…", true, None);
    let setup = MenuItem::new(setup_text(&core), true, None);
    let open = MenuItem::new("Open astromech", true, None);
    let pause = CheckMenuItem::new("Pause voice", true, VOICE_PAUSED.load(Ordering::SeqCst), None);
    let kill = MenuItem::new("Kill switch (stop all actions)", true, None);
    let install = MenuItem::new(update_text(&core), true, None);
    let quit = MenuItem::new("Quit astromech", true, None);
    let sep = PredefinedMenuItem::separator;

    let build = |s: &Shape| -> Result<Menu> {
        let m = Menu::new();
        m.append(&status)?;
        if s.crash {
            m.append(&crash)?;
        }
        if s.setup {
            m.append(&setup)?;
        }
        m.append_items(&[&sep(), &open, &sep(), &pause])?;
        if s.active {
            m.append(&kill)?;
        }
        m.append(&sep())?;
        if s.update {
            m.append(&install)?;
        }
        m.append(&quit)?;
        Ok(m)
    };
    let mut current = shape(&core);
    let tray = TrayIconBuilder::new()
        .with_tooltip(format!("astromech\n{}", core.status_line()))
        .with_icon(icon()?)
        .with_menu(Box::new(build(&current)?))
        .with_menu_on_left_click(false)
        .build()
        .context("creating the tray icon")?;
    log("tray icon ready");

    let refresh = |current: &mut Shape| -> Result<()> {
        let line = core.status_line();
        status.set_text(&line);
        let _ = tray.set_tooltip(Some(format!("astromech\n{line}")));
        let killed = core.session.lock().unwrap().as_ref().map(|s| s.runner.policy.is_killed()).unwrap_or(false);
        kill.set_text(if killed { "Re-arm (kill switch is on)" } else { "Kill switch (stop all actions)" });
        pause.set_checked(VOICE_PAUSED.load(Ordering::SeqCst));
        setup.set_text(setup_text(&core));
        install.set_text(update_text(&core));
        let now = shape(&core);
        if now != *current {
            tray.set_menu(Some(Box::new(build(&now)?)));
            *current = now;
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
            } else if id == setup.id() {
                let _ = core.open_window(Some("settings"));
            } else if id == pause.id() {
                let on = pause.is_checked();
                VOICE_PAUSED.store(on, Ordering::SeqCst);
                log(if on { "voice paused (tray)" } else { "voice resumed (tray)" });
            } else if id == kill.id() {
                core.kill_switch();
            } else if id == install.id() {
                let _ = core.install_update();
            } else if id == crash.id() {
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
            if let Err(e) = refresh(&mut current) {
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
