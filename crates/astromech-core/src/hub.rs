//! Published game-state snapshots. Writers: the pipeline. Readers: tools (planning),
//! verification (waiting for a flag), subscribers (the future API).

use astromech_api::Recover;
use astromech_gamestate::mining::MiningRun;
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
    mining: Mutex<MiningRun>,
}

impl StateHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn snapshot(&self) -> GameState {
        self.state.lock().recover().0.clone()
    }

    /// Look at the current state without copying it. Keep `f` short: writers wait for it.
    pub fn read<R>(&self, f: impl FnOnce(&GameState) -> R) -> R {
        f(&self.state.lock().recover().0)
    }

    /// Version counter: increases on every change.
    pub fn version(&self) -> u64 {
        self.state.lock().recover().1
    }

    pub fn subscribe(&self) -> Receiver<GameState> {
        let (tx, rx) = channel();
        let _ = tx.send(self.snapshot());
        self.subs.lock().recover().push(tx);
        rx
    }

    pub fn update(&self, f: impl FnOnce(&mut GameState)) {
        let snap = {
            let mut g = self.state.lock().recover();
            let before = g.0.clone();
            f(&mut g.0);
            if g.0 == before {
                return;
            }
            g.1 += 1;
            g.0.clone()
        };
        self.cv.notify_all();
        self.subs.lock().recover().retain(|s| s.send(snap.clone()).is_ok());
    }

    pub fn apply_status(&self, s: &StatusSnap) {
        self.update(|g| g.apply_status(s));
    }

    pub fn apply_journal(&self, l: &JournalLine) {
        self.mining.lock().recover().apply(l);
        self.update(|g| g.apply_line(l));
    }

    /// The current mining run, as the status reply shows it.
    pub fn mining_json(&self) -> serde_json::Value {
        let m = self.mining.lock().recover();
        serde_json::json!({
            "started": m.started.map(|t| t.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
            "refined": m.refined,
            "total": m.total(),
            "per_hour": m.per_hour(),
        })
    }

    /// Wait until `flag` is set (`expect`) or clear, up to `timeout`. Returns early (false)
    /// when `cancelled()` turns true.
    pub fn wait_flag(&self, flag: u64, expect: bool, timeout: Duration, cancelled: &dyn Fn() -> bool) -> bool {
        let end = Instant::now() + timeout;
        let mut g = self.state.lock().recover();
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
            g = self.cv.wait_timeout(g, (end - now).min(Duration::from_millis(25))).recover().0;
        }
    }

    /// Wait until `GuiFocus` equals `want`.
    pub fn wait_gui(&self, want: u64, timeout: Duration, cancelled: &dyn Fn() -> bool) -> bool {
        let end = Instant::now() + timeout;
        let mut g = self.state.lock().recover();
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
            g = self.cv.wait_timeout(g, (end - now).min(Duration::from_millis(25))).recover().0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// A panic while the state was locked (a handler's, say) doesn't make the next press's
    /// verification panic: the wait recovers the lock and still sees the flag the pipeline sets.
    #[test]
    fn waiting_for_a_flag_works_after_a_panic_poisoned_the_state() {
        const FLAG: u64 = astromech_status::FLAG_IN_MAIN_SHIP;
        let hub = Arc::new(StateHub::new());
        let h = hub.clone();
        let poisoned = std::thread::spawn(move || h.update(|_| panic!("a bug while the state is locked"))).join();
        assert!(poisoned.is_err() && hub.state.is_poisoned());
        // Set the flag from another thread once the wait holds the lock (`cancelled` runs under
        // it), so the update lands while the wait waits.
        let setter = Mutex::new(None);
        let cancelled = || {
            let mut s = setter.lock().recover();
            if s.is_none() {
                let h = hub.clone();
                *s = Some(std::thread::spawn(move || {
                    h.update(|g| {
                        g.status_seen = true;
                        g.status_flags = FLAG;
                    })
                }));
            }
            false
        };
        assert!(hub.wait_flag(FLAG, true, Duration::from_secs(60), &cancelled));
        setter.lock().recover().take().unwrap().join().unwrap();
    }
}
