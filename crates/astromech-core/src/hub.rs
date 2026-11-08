//! Published game-state snapshots. Writers: the pipeline. Readers: tools (planning),
//! verification (waiting for a flag), subscribers (the future API).

use astromech_api::Recover;
use astromech_gamestate::mining::MiningRun;
use astromech_gamestate::GameState;
use astromech_journal::JournalLine;
use astromech_proto::{Clock, SystemClock};
use astromech_status::StatusSnap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

/// The mining run in the status reply: when it started (RFC 3339), tons refined by commodity,
/// their total and the rate per hour.
#[derive(Debug, Clone, serde::Serialize, ts_rs::TS)]
pub struct Mining {
    pub started: Option<String>,
    pub refined: std::collections::BTreeMap<String, u32>,
    pub total: u32,
    pub per_hour: Option<f64>,
}

pub struct StateHub {
    state: Arc<Mutex<(GameState, u64)>>,
    cv: Arc<Condvar>,
    subs: Mutex<Vec<Sender<GameState>>>,
    mining: Mutex<MiningRun>,
    /// The verify waits' time, and the pipeline's ([`crate::pipeline::start`]).
    clock: Arc<dyn Clock>,
}

impl Default for StateHub {
    fn default() -> Self {
        Self::new()
    }
}

impl StateHub {
    /// On the system clock.
    pub fn new() -> Self {
        Self::with_clock(Arc::new(SystemClock))
    }

    pub fn with_clock(clock: Arc<dyn Clock>) -> Self {
        Self {
            state: Default::default(),
            cv: Default::default(),
            subs: Default::default(),
            mining: Default::default(),
            clock,
        }
    }

    pub fn clock(&self) -> &Arc<dyn Clock> {
        &self.clock
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
        self.clock.notify_all(&self.state, &self.cv);
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
    pub fn mining(&self) -> Mining {
        let m = self.mining.lock().recover();
        Mining {
            started: m.started.map(|t| t.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
            refined: m.refined.clone(),
            total: m.total(),
            per_hour: m.per_hour(),
        }
    }

    /// [`StateHub::mining`] as JSON.
    pub fn mining_json(&self) -> serde_json::Value {
        serde_json::to_value(self.mining()).unwrap_or_default()
    }

    /// Wait until `flag` is set (`expect`) or clear, up to `timeout`. Returns early (false)
    /// when `cancelled()` turns true.
    pub fn wait_flag(&self, flag: u64, expect: bool, timeout: Duration, cancelled: &dyn Fn() -> bool) -> bool {
        let end = self.clock.now() + timeout;
        let mut g = self.state.lock().recover();
        loop {
            if g.0.status_seen && (g.0.status_flags & flag != 0) == expect {
                return true;
            }
            if cancelled() {
                return false;
            }
            let now = self.clock.now();
            if now >= end {
                return false;
            }
            // In 25 ms slices, so `cancelled` (the kill switch) is looked at while waiting.
            g = self
                .clock
                .wait_timeout_from(&self.state, &self.cv, g, now, (end - now).min(Duration::from_millis(25)))
                .0;
        }
    }

    /// Wait until `GuiFocus` equals `want`.
    pub fn wait_gui(&self, want: u64, timeout: Duration, cancelled: &dyn Fn() -> bool) -> bool {
        let end = self.clock.now() + timeout;
        let mut g = self.state.lock().recover();
        loop {
            if g.0.status_seen && g.0.gui_focus.unwrap_or(0) == want {
                return true;
            }
            if cancelled() {
                return false;
            }
            let now = self.clock.now();
            if now >= end {
                return false;
            }
            // In 25 ms slices, so `cancelled` (the kill switch) is looked at while waiting.
            g = self
                .clock
                .wait_timeout_from(&self.state, &self.cv, g, now, (end - now).min(Duration::from_millis(25)))
                .0;
        }
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;
    use std::sync::Arc;

    /// `advance` on a helper thread, failing the test (not hanging it) if it never settles:
    /// a registered thread waiting off the clock would hold it up for good.
    fn settle(m: &Arc<astromech_proto::ManualClock>, d: Duration) {
        let (m, (tx, rx)) = (m.clone(), std::sync::mpsc::channel());
        std::thread::spawn(move || {
            m.advance(d);
            let _ = tx.send(());
        });
        // A guard against hanging, never how the test decides.
        #[allow(clippy::disallowed_methods)]
        let settled = rx.recv_timeout(Duration::from_secs(30));
        assert!(settled.is_ok(), "advance never settled: the wait isn't on the clock");
    }

    /// A verify wait on the clock: it returns at once when the flag is set, with no `advance`,
    /// and times out only once `advance` has passed the timeout.
    #[test]
    fn a_verify_wait_returns_on_the_flag_and_times_out_on_the_clock() {
        const FLAG: u64 = astromech_status::FLAG_IN_MAIN_SHIP;
        const OTHER: u64 = astromech_status::FLAG_IN_SRV;
        let m = Arc::new(astromech_proto::ManualClock::at_unix(1_800_000_000));
        let clock: Arc<dyn Clock> = m.clone();
        let hub = Arc::new(StateHub::with_clock(clock.clone()));
        let (tx, out) = std::sync::mpsc::channel();
        let wait = |flag: u64| {
            let (h, tx) = (hub.clone(), tx.clone());
            clock
                .spawn(std::thread::Builder::new(), move || {
                    let _ = tx.send(h.wait_flag(flag, true, Duration::from_secs(2), &|| false));
                })
                .unwrap()
        };
        let set = wait(FLAG);
        settle(&m, Duration::ZERO);
        hub.update(|g| {
            g.status_seen = true;
            g.status_flags = FLAG;
        });
        set.join().unwrap();
        assert_eq!(out.try_recv(), Ok(true), "seen at once, with no advance");

        let unset = wait(OTHER);
        settle(&m, Duration::ZERO);
        settle(&m, Duration::from_millis(1999));
        assert!(out.try_recv().is_err(), "timed out before the timeout passed on the clock");
        settle(&m, Duration::from_millis(1));
        unset.join().unwrap();
        assert_eq!(out.try_recv(), Ok(false), "timed out once it had");
    }

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
