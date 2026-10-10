//! XInput poller for Xbox-compatible pads. Read-only: only XInputGetState (no rumble).
//! The DLL is loaded at runtime, so the exe has no hard dependency on it.

use super::{foreground_title, wide, Msg};
use crate::listen::model::{DevKind, DeviceMeta, Sample, SampleData};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows_sys::Win32::UI::Input::XboxController::XINPUT_STATE;

type GetState = unsafe extern "system" fn(u32, *mut XINPUT_STATE) -> u32;

fn load() -> Option<GetState> {
    unsafe {
        for name in ["xinput1_4.dll", "xinput9_1_0.dll"] {
            let lib = LoadLibraryW(wide(name).as_ptr());
            if !lib.is_null() {
                if let Some(f) = GetProcAddress(lib, c"XInputGetState".as_ptr() as *const u8) {
                    return Some(std::mem::transmute::<_, GetState>(f));
                }
            }
        }
    }
    None
}

fn sample(s: &XINPUT_STATE) -> SampleData {
    let g = s.Gamepad;
    let mut m = g.wButtons as u32;
    if g.bLeftTrigger > 128 {
        m |= 0x10000;
    }
    if g.bRightTrigger > 128 {
        m |= 0x20000;
    }
    let st = |v: i16| (v as f64 + 32768.0) / 65535.0;
    SampleData::Xinput {
        buttons: m,
        axes: vec![
            ("LeftX".into(), st(g.sThumbLX)),
            ("LeftY".into(), st(g.sThumbLY)),
            ("RightX".into(), st(g.sThumbRX)),
            ("RightY".into(), st(g.sThumbRY)),
            ("LT".into(), g.bLeftTrigger as f64 / 255.0),
            ("RT".into(), g.bRightTrigger as f64 / 255.0),
        ],
    }
}

/// Start polling. Returns a status string.
pub fn start(tx: Sender<Msg>, start: Instant) -> String {
    let Some(get) = load() else { return "XInput DLL not available; Xbox pads not polled".into() };
    std::thread::Builder::new()
        .name("xinput".into())
        .spawn(move || {
            let mut prev: [Option<u32>; 4] = [None; 4];
            let mut announced = [false; 4];
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
                    if unsafe { get(i as u32, &mut s) } != 0 {
                        prev[i] = None;
                        continue;
                    }
                    any = true;
                    if !announced[i] {
                        announced[i] = true;
                        let _ = tx.send(Msg::Device(DeviceMeta {
                            key: format!("xinput:{i}"),
                            kind: DevKind::Xinput,
                            vid: 0,
                            pid: 0,
                            product: format!("XInput controller #{}", i + 1),
                            usage: 5,
                            buttons_declared: 16,
                            axes_declared: vec![],
                            hats_declared: 0,
                        }));
                    }
                    if prev[i] != Some(s.dwPacketNumber) {
                        prev[i] = Some(s.dwPacketNumber);
                        let t_ms = start.elapsed().as_secs_f64() * 1000.0;
                        if tx
                            .send(Msg::Sample(Sample {
                                t_ms,
                                dev: format!("xinput:{i}"),
                                fg: foreground_title(),
                                injected: false,
                                data: sample(&s),
                            }))
                            .is_err()
                        {
                            return;
                        }
                    }
                }
                std::thread::sleep(if any { Duration::from_millis(2) } else { Duration::from_millis(50) });
            }
        })
        .map(|_| "polling slots 0-3 (XInputGetState only)".to_string())
        .unwrap_or_else(|e| format!("XInput thread failed: {e}"))
}
