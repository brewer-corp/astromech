//! The process's one input source on a real desktop: a key sent with SendInput reaches two
//! subscribers, is lost to both once another registration in the process takes the keyboard
//! (as tao's does when Tauri builds its event loop), and reaches both again after `restart()`.
//!
//! SendInput needs an interactive desktop session, so this is `#[ignore]`d:
//! `cargo test -p astromech-input --test source_sendinput -- --ignored`. Windows CI runs it
//! best effort.
#![cfg(windows)]
#![allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)

use astromech_input::listen::model::SampleData;
use astromech_input::listen::source::{self, Msg};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyW, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC,
};
use windows_sys::Win32::UI::Input::{RegisterRawInputDevices, RAWINPUTDEVICE};

/// F24: on no keyboard, bound by nothing.
const VK_F24: u16 = 0x87;

fn send(up: bool) {
    let i = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_F24,
                wScan: unsafe { MapVirtualKeyW(VK_F24 as u32, MAPVK_VK_TO_VSC) } as u16,
                dwFlags: if up { KEYEVENTF_KEYUP } else { 0 },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    assert_eq!(unsafe { SendInput(1, &i, std::mem::size_of::<INPUT>() as i32) }, 1, "SendInput");
}

fn press() {
    send(false);
    std::thread::sleep(Duration::from_millis(30));
    send(true);
}

/// Whether `rx` gets F24 down within `wait`.
fn saw(rx: &Receiver<Msg>, wait: Duration) -> bool {
    let end = Instant::now() + wait;
    while let Some(left) = end.checked_duration_since(Instant::now()) {
        match rx.recv_timeout(left) {
            Ok(Msg::Sample(s)) => {
                if let SampleData::Key { vkey: VK_F24, down: true, .. } = s.data {
                    return true;
                }
            }
            Ok(Msg::Device(_)) => {}
            Err(_) => return false,
        }
    }
    false
}

fn drain(rx: &Receiver<Msg>) {
    while rx.recv_timeout(Duration::from_millis(200)).is_ok() {}
}

#[test]
#[ignore = "needs an interactive desktop session for SendInput"]
fn two_subscribers_see_a_sent_key_before_and_after_restart() {
    let a = source::subscribe();
    let b = source::subscribe();
    assert!(a.status.starts_with("Raw Input: ok"), "status: {}", a.status);
    let wait = Duration::from_secs(2);

    press();
    assert!(saw(&a.rx, wait), "first subscriber didn't see the key");
    assert!(saw(&b.rx, wait), "second subscriber didn't see the key");

    // Another registration in this process, as tao's: the keyboard follows focus, away from
    // the source's window.
    let rid = RAWINPUTDEVICE { usUsagePage: 1, usUsage: 6, dwFlags: 0, hwndTarget: std::ptr::null_mut() };
    assert_ne!(unsafe { RegisterRawInputDevices(&rid, 1, std::mem::size_of::<RAWINPUTDEVICE>() as u32) }, 0);
    drain(&a.rx);
    drain(&b.rx);
    press();
    assert!(!saw(&a.rx, Duration::from_millis(700)), "the later registration didn't take the keyboard");
    assert!(!saw(&b.rx, Duration::from_millis(100)));

    source::restart().expect("register again");
    press();
    assert!(saw(&a.rx, wait), "first subscriber didn't see the key after restart");
    assert!(saw(&b.rx, wait), "second subscriber didn't see the key after restart");
}
