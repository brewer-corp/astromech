//! Windows: the PTT and kill-switch buttons, read with the S7 Raw Input listener
//! (RIDEV_INPUTSINK, read-only; S7 showed Elite doesn't take exclusive input from it).

use crate::config::Button;
use astromech_input::listen::tracker::Processor;
use astromech_input::listen::win::{self, Msg};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    PttDown,
    PttUp,
    /// Kill-switch pressed.
    KillDown,
    /// Kill-switch released after being held this long.
    KillUp(Duration),
}

/// Start listening. Returns the Raw Input registration status line.
pub fn start(ptt: Option<Button>, kill: Option<Button>, tx: Sender<Control>) -> String {
    let start = Instant::now();
    let (mtx, mrx) = std::sync::mpsc::channel();
    let reg = win::rawinput::start(mtx.clone(), start);
    let xi = win::xinput::start(mtx, start);
    std::thread::Builder::new()
        .name("controls".into())
        .spawn(move || {
            let mut p = Processor::new(astromech_input::ELITE_TITLE, None, true);
            for d in win::rawinput::list_devices() {
                p.add_device(d);
            }
            let mut kill_at: Option<Instant> = None;
            loop {
                let evs = match mrx.recv_timeout(Duration::from_millis(100)) {
                    Ok(Msg::Device(d)) => {
                        p.add_device(d);
                        continue;
                    }
                    Ok(Msg::Sample(s)) => p.process(&s),
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => p.tick(start.elapsed().as_secs_f64() * 1000.0),
                    Err(_) => break,
                };
                // We only react; don't keep a history.
                p.events.clear();
                p.anomalies.clear();
                for e in evs {
                    let Some(down) = e.down else { continue };
                    if ptt.as_ref().map(|b| b.matches(&e)).unwrap_or(false) {
                        let _ = tx.send(if down { Control::PttDown } else { Control::PttUp });
                    }
                    if kill.as_ref().map(|b| b.matches(&e)).unwrap_or(false) {
                        if down {
                            kill_at.get_or_insert_with(Instant::now);
                            let _ = tx.send(Control::KillDown);
                        } else if let Some(t) = kill_at.take() {
                            let _ = tx.send(Control::KillUp(t.elapsed()));
                        }
                    }
                }
            }
        })
        .expect("spawn controls thread");
    format!(
        "Raw Input: {}; XInput: {xi}",
        match reg {
            Ok(()) => "ok".to_string(),
            Err(e) => e,
        }
    )
}
