//! Published game-state snapshots. Writers: the pipeline. Readers: tools (planning),
//! verification (waiting for a flag), subscribers (the future API).

use astromech_gamestate::GameState;
use astromech_journal::JournalLine;
use astromech_status::StatusSnap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct StateHub {
    state: Mutex<(GameState, u64)>,
    cv: Condvar,
    subs: Mutex<Vec<Sender<GameState>>>,
}

impl StateHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn snapshot(&self) -> GameState {
        self.state.lock().unwrap().0.clone()
    }

    /// Version counter: increases on every change.
    pub fn version(&self) -> u64 {
        self.state.lock().unwrap().1
    }

    pub fn subscribe(&self) -> Receiver<GameState> {
        let (tx, rx) = channel();
        let _ = tx.send(self.snapshot());
        self.subs.lock().unwrap().push(tx);
        rx
    }

    pub fn update(&self, f: impl FnOnce(&mut GameState)) {
        let snap = {
            let mut g = self.state.lock().unwrap();
            let before = g.0.clone();
            f(&mut g.0);
            if g.0 == before {
                return;
            }
            g.1 += 1;
            g.0.clone()
        };
        self.cv.notify_all();
        self.subs.lock().unwrap().retain(|s| s.send(snap.clone()).is_ok());
    }

    pub fn apply_status(&self, s: &StatusSnap) {
        self.update(|g| g.apply_status(s));
    }

    pub fn apply_journal(&self, l: &JournalLine) {
        self.update(|g| g.apply_line(l));
    }

    /// Wait until `flag` is set (`expect`) or clear, up to `timeout`. Returns early (false)
    /// when `cancelled()` turns true.
    pub fn wait_flag(&self, flag: u64, expect: bool, timeout: Duration, cancelled: &dyn Fn() -> bool) -> bool {
        let end = Instant::now() + timeout;
        let mut g = self.state.lock().unwrap();
        loop {
            if g.0.status_seen && (g.0.status_flags & flag != 0) == expect {
                return true;
            }
            if cancelled() {
                return false;
            }
            let now = Instant::now();
            if now >= end {
                return false;
            }
            g = self.cv.wait_timeout(g, (end - now).min(Duration::from_millis(25))).unwrap().0;
        }
    }

    /// Wait until `GuiFocus` equals `want`.
    pub fn wait_gui(&self, want: u64, timeout: Duration, cancelled: &dyn Fn() -> bool) -> bool {
        let end = Instant::now() + timeout;
        let mut g = self.state.lock().unwrap();
        loop {
            if g.0.status_seen && g.0.gui_focus.unwrap_or(0) == want {
                return true;
            }
            if cancelled() {
                return false;
            }
            let now = Instant::now();
            if now >= end {
                return false;
            }
            g = self.cv.wait_timeout(g, (end - now).min(Duration::from_millis(25))).unwrap().0;
        }
    }
}
