//! XInput poller (for Xbox-compatible pads), for comparison with Raw Input HID.
//! Polls connected slots every ~1 ms; looks for newly connected pads every 2 s.
//! Read-only: never calls XInputSetState (no rumble) or anything else that writes.

use crate::input::{Control, DeviceInfo, InputEvent, Source};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::{Duration, Instant};
use windows_sys::Win32::UI::Input::XboxController::{XInputGetState, XINPUT_STATE};

const BUTTONS: [u32; 14] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200, 0x1000, 0x2000, 0x4000, 0x8000];

fn mask_of(s: &XINPUT_STATE) -> u32 {
    let mut m = s.Gamepad.wButtons as u32;
    if s.Gamepad.bLeftTrigger > 128 {
        m |= 0x10000;
    }
    if s.Gamepad.bRightTrigger > 128 {
        m |= 0x20000;
    }
    m
}

pub fn connected_slots() -> Vec<u32> {
    (0..4u32)
        .filter(|&i| unsafe {
            let mut s: XINPUT_STATE = std::mem::zeroed();
            XInputGetState(i, &mut s) == 0
        })
        .collect()
}

pub fn start(tx: Sender<InputEvent>) {
    std::thread::Builder::new()
        .name("xinput".into())
        .spawn(move || {
            let infos: Vec<Arc<DeviceInfo>> = (0..4)
                .map(|i| Arc::new(DeviceInfo { source: Source::XInput, path: format!("xinput:{i}"), vid: 0, pid: 0, product: format!("XInput controller #{}", i + 1), usage_page: 1, usage: 5 }))
                .collect();
            let mut prev: [Option<u32>; 4] = [None; 4];
            let mut last_scan = Instant::now() - Duration::from_secs(10);
            loop {
                let scan = last_scan.elapsed() >= Duration::from_secs(2);
                if scan {
                    last_scan = Instant::now();
                }
                let mut any = false;
                for i in 0..4usize {
                    if prev[i].is_none() && !scan {
                        continue;
                    }
                    let mut s: XINPUT_STATE = unsafe { std::mem::zeroed() };
                    let r = unsafe { XInputGetState(i as u32, &mut s) };
                    let at = Instant::now();
                    if r != 0 {
                        prev[i] = None;
                        continue;
                    }
                    any = true;
                    let m = mask_of(&s);
                    if let Some(p) = prev[i] {
                        if p != m {
                            let fg = super::rawinput::foreground_title();
                            for b in BUTTONS.iter().copied().chain([0x10000, 0x20000]) {
                                if (p ^ m) & b != 0 {
                                    let c = Control::XButton { mask: b };
                                    let ev = InputEvent { device: infos[i].clone(), control: c, label: c.describe(), down: m & b != 0, at, queue_ms: None, foreground: fg.clone(), injected: false };
                                    if tx.send(ev).is_err() {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    prev[i] = Some(m);
                }
                std::thread::sleep(if any { Duration::from_millis(1) } else { Duration::from_millis(50) });
            }
        })
        .expect("spawn xinput thread");
}
