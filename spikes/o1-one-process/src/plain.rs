//! `--plain-tray`: the same tray through the `tray-icon` crate and a Win32 pump that wakes every
//! 100 ms, without Tauri, as crates/astromech-core/src/tray.rs runs it today. The cost baseline.
//! Its control port answers `ui_state` and `quit`, handled on the main thread.

use crate::control;
use crate::win::{self, log, Opts, Sink};
use serde_json::{json, Value};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::TrayIconBuilder;

type Request = (String, Sender<Result<Value, String>>);

/// A plain 32×32 square: the baseline measures the tray, not the PNG decoder.
fn icon() -> tray_icon::Icon {
    let px: Vec<u8> = std::iter::repeat_n([0xFF, 0x8A, 0x00, 0xFF], 32 * 32).flatten().collect();
    tray_icon::Icon::from_rgba(px, 32, 32).expect("icon")
}

pub fn run(opts: &Opts) {
    if opts.sink != Sink::None {
        win::start_sink("in the plain tray");
    }
    win::dark_menus();
    let status = MenuItem::new("astromech: ready", false, None);
    let open = MenuItem::new("Open astromech", true, None);
    let pause = CheckMenuItem::new("Pause voice", true, false, None);
    let quit = MenuItem::new("Quit astromech", true, None);
    let sep = PredefinedMenuItem::separator;
    let m = Menu::new();
    m.append_items(&[&status, &sep(), &open, &sep(), &pause, &sep(), &quit]).expect("menu");
    let tray = TrayIconBuilder::new()
        .with_tooltip("astromech spike (plain tray)")
        .with_icon(icon())
        .with_menu(Box::new(m))
        .with_menu_on_left_click(false)
        .build()
        .expect("tray icon");
    log("tray icon ready (plain)");

    let (tx, rx) = channel::<Request>();
    let tx = Mutex::new(tx);
    let h: control::Handler = Arc::new(move |cmd, _args| {
        let (rtx, rrx) = channel();
        tx.lock().unwrap().send((cmd.to_string(), rtx)).map_err(|e| e.to_string())?;
        rrx.recv_timeout(Duration::from_secs(5)).map_err(|e| format!("main thread: {e}"))?
    });
    if let Err(e) = control::serve(h) {
        log(&format!("control: couldn't listen: {e}"));
    }
    loop {
        pump(Duration::from_millis(100));
        while let Ok(e) = MenuEvent::receiver().try_recv() {
            if e.id() == quit.id() {
                return;
            }
        }
        while let Ok((cmd, reply)) = rx.try_recv() {
            let r = match cmd.as_str() {
                "ui_state" => {
                    let mut v = win::base_state();
                    let rect = tray.rect();
                    v["window"] = json!("closed");
                    v["tray"] = json!(rect.is_some());
                    v["tray_rect"] = json!(format!("{rect:?}"));
                    v["tray_built"] = json!(true);
                    Ok(v)
                }
                "quit" => {
                    let _ = reply.send(Ok(json!(null)));
                    std::thread::sleep(Duration::from_millis(200));
                    log("quit (plain)");
                    return;
                }
                _ => Err(format!("{cmd} is not in the plain tray")),
            };
            let _ = reply.send(r);
        }
    }
}

/// tray.rs's pump: dispatch pending messages, then wait up to `max` for more.
fn pump(max: Duration) {
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
