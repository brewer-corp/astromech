//! The tray icon on Tauri's tray (the `tray-icon` feature): the menu `astromech_core::tray`
//! describes, with its actions. Left-click opens the window. Every menu call runs on the main
//! thread: the menu is refreshed there ([`Tray::show`]) and Tauri delivers its events there.

use crate::host::Host;
use astromech_core::logging::log;
use astromech_core::session::VOICE_PAUSED;
use astromech_core::tray::{self as menu, Id, Item, Menu, Shape};
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use tauri::image::Image;
use tauri::menu::{CheckMenuItem, Menu as TauriMenu, MenuEvent, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::AppHandle;

const ICON_PNG: &[u8] = include_bytes!("../icons/32x32.png");

/// The menu's items, made once; the menu is rebuilt from them when its [`Shape`] changes.
struct Items {
    text: HashMap<Id, MenuItem<tauri::Wry>>,
    pause: CheckMenuItem<tauri::Wry>,
}

impl Items {
    fn new(app: &AppHandle, m: &Menu) -> tauri::Result<Self> {
        let mut text = HashMap::new();
        for id in Id::ALL.into_iter().filter(|i| *i != Id::Pause) {
            text.insert(id, MenuItem::with_id(app, id.as_str(), "", true, None::<&str>)?);
        }
        let pause = CheckMenuItem::with_id(app, Id::Pause.as_str(), menu::PAUSE, true, m.paused, None::<&str>)?;
        let items = Self { text, pause };
        items.set(m)?;
        Ok(items)
    }

    /// Give each item of `m` its text and state.
    fn set(&self, m: &Menu) -> tauri::Result<()> {
        for item in menu::items(m) {
            match item {
                Item::Separator => {}
                Item::Text { id, text, enabled } => {
                    let h = &self.text[&id];
                    h.set_text(text)?;
                    h.set_enabled(enabled)?;
                }
                Item::Check { checked, .. } => self.pause.set_checked(checked)?,
            }
        }
        Ok(())
    }

    /// The menu for `m`'s shape.
    fn menu(&self, app: &AppHandle, m: &Menu) -> tauri::Result<TauriMenu<tauri::Wry>> {
        let built = TauriMenu::new(app)?;
        for item in menu::items(m) {
            match item {
                Item::Separator => built.append(&PredefinedMenuItem::separator(app)?)?,
                Item::Text { id, .. } => built.append(&self.text[&id])?,
                Item::Check { .. } => built.append(&self.pause)?,
            }
        }
        Ok(built)
    }
}

/// The tray icon and its menu.
pub struct Tray {
    pub icon: TrayIcon,
    items: Items,
    shape: Mutex<Shape>,
}

impl Tray {
    /// Show `m`: the texts, states and tooltip, and a new menu if its shape changed.
    pub fn show(&self, app: &AppHandle, m: &Menu) -> tauri::Result<()> {
        self.items.set(m)?;
        self.icon.set_tooltip(Some(menu::tooltip(&m.status)))?;
        let mut shape = self.shape.lock().unwrap_or_else(|e| e.into_inner());
        if *shape != m.shape {
            self.icon.set_menu(Some(self.items.menu(app, m)?))?;
            *shape = m.shape.clone();
        }
        Ok(())
    }
}

/// Build the tray icon showing `m`. Its clicks act on the core in `host`.
pub fn build(app: &AppHandle, host: Arc<Host>, m: &Menu) -> tauri::Result<Tray> {
    let items = Items::new(app, m)?;
    let on_icon = host.clone();
    let icon = TrayIconBuilder::new()
        .tooltip(menu::tooltip(&m.status))
        .icon(Image::from_bytes(ICON_PNG)?)
        .menu(&items.menu(app, m)?)
        .show_menu_on_left_click(false)
        .on_menu_event(move |_, e| clicked(&host, &e))
        .on_tray_icon_event(move |_, e| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                if let Some(core) = on_icon.core() {
                    let _ = core.open_window(None);
                }
            }
        })
        .build(app)?;
    Ok(Tray { icon, items, shape: Mutex::new(m.shape.clone()) })
}

/// A menu item was clicked (on the main thread). What may take a while runs on its own thread,
/// so the window and the menu stay responsive.
fn clicked(host: &Host, e: &MenuEvent) {
    let (Some(core), Some(id)) = (host.core(), Id::parse(e.id().as_ref())) else { return };
    match id {
        Id::Open => {
            let _ = core.open_window(None);
        }
        Id::Setup => {
            let _ = core.open_window(Some("settings"));
        }
        Id::Pause => {
            let on = host.tray().and_then(|t| t.items.pause.is_checked().ok()).unwrap_or(false);
            VOICE_PAUSED.store(on, Ordering::SeqCst);
            log(if on { "voice paused (tray)" } else { "voice resumed (tray)" });
        }
        Id::Kill => core.kill_switch(),
        Id::Check => core.check_update_in_background(),
        Id::Install => {
            let c = core.clone();
            std::thread::spawn(move || {
                let _ = c.install_update();
            });
        }
        Id::Crash => {
            let c = core.clone();
            std::thread::spawn(move || match c.save_diagnostics() {
                Ok(p) => c.opener.reveal(&p),
                Err(e) => log(&format!("couldn't save diagnostics: {e:#}")),
            });
        }
        Id::Quit => {
            log("quit (tray)");
            core.ui.quit();
            return;
        }
        Id::Status => {}
    }
    core.notify();
}

/// Dark context menus, to match the window. Windows has no public API for this; uxtheme's
/// `SetPreferredAppMode` (ordinal 135, Windows 10 1903+) and `FlushMenuThemes` (136) are what
/// Explorer and most apps use. On older builds 135 is `AllowDarkModeForApp`, which also takes a
/// non-zero argument; without either the menu stays light. Called after Tauri's
/// `Builder::build`, whose event loop turns dark mode on for the app.
#[cfg(windows)]
pub fn dark_menus() {
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
