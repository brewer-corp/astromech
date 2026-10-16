//! Delivery in the background. The worker thread owns its own [`Outbox`] connection and its
//! transport, so the journal side ([`crate::Sender`], [`crate::Feed`]) never shares a lock
//! with a network call: queuing a message is one SQLite insert, whatever the gateway is doing.
//!
//! A retry needs a minute on the worker's own monotonic clock as well as on the wall clock
//! (each message's `next_try`), so a clock jump can't cause an early retry. The worker starts
//! with that minute already running: after a restart, messages left from the last run wait a
//! minute too.

use crate::outbox::{Delivery, Outbox, MIN_RETRY_SECS};
use crate::transport::Transport;
use std::sync::mpsc::{channel, RecvTimeoutError, Sender as Tx};
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// How long the worker sleeps when nothing is queued (a submit wakes it sooner).
const IDLE: Duration = Duration::from_secs(300);
const MIN_RETRY: Duration = Duration::from_secs(MIN_RETRY_SECS as u64);

/// Unix seconds now.
pub fn wall_now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// One delivery pass at a time, with the monotonic retry hold.
pub struct Courier {
    outbox: Outbox,
    transport: Box<dyn Transport + Send>,
    /// No retry before this instant: a minute after the last failure (or the start).
    hold_until: Instant,
}

impl Courier {
    pub fn new(outbox: Outbox, transport: Box<dyn Transport + Send>, started: Instant) -> Self {
        Self { outbox, transport, hold_until: started + MIN_RETRY }
    }

    /// Deliver what's due: `wall` is Unix seconds, `mono` the monotonic time of the same moment.
    pub fn pass(&mut self, wall: i64, mono: Instant) -> rusqlite::Result<Delivery> {
        let d = self.outbox.deliver_due(self.transport.as_ref(), wall, mono >= self.hold_until)?;
        if d.retrying > 0 || d.stopped {
            self.hold_until = mono + MIN_RETRY;
        }
        Ok(d)
    }

    /// How long to sleep after a pass before the next one is worth making.
    pub fn wait(&self, wall: i64, mono: Instant) -> Duration {
        match self.outbox.next_due() {
            Ok(Some(due)) => {
                let by_wall = Duration::from_secs(due.saturating_sub(wall).max(0) as u64);
                by_wall.max(self.hold_until.saturating_duration_since(mono)).clamp(Duration::from_secs(1), IDLE)
            }
            Ok(None) => IDLE,
            // The database is unreadable right now; try again in a while.
            Err(_) => MIN_RETRY,
        }
    }

    pub fn outbox(&self) -> &Outbox {
        &self.outbox
    }
}

enum Signal {
    Wake,
    Stop,
}

/// The delivery thread. Dropping it stops the thread after the post in progress, if any.
pub struct Worker {
    tx: Tx<Signal>,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Start delivering. `log` gets each pass that did something, and database errors.
    pub fn spawn(mut courier: Courier, log: impl Fn(&str) + Send + 'static) -> std::io::Result<Self> {
        let (tx, rx) = channel();
        let thread = std::thread::Builder::new().name("eddn".into()).spawn(move || loop {
            match courier.pass(wall_now(), Instant::now()) {
                Ok(d) if d != Delivery::default() => log(&format!(
                    "EDDN: {} sent, {} rejected, {} to retry, {} given up{}",
                    d.sent,
                    d.rejected,
                    d.retrying,
                    d.gave_up,
                    if d.stopped { "; gateway unreachable, waiting" } else { "" }
                )),
                Ok(_) => {}
                Err(e) => log(&format!("EDDN outbox: {e}")),
            }
            let wait = courier.wait(wall_now(), Instant::now());
            match rx.recv_timeout(wait) {
                Ok(Signal::Wake) | Err(RecvTimeoutError::Timeout) => {
                    // Several submits in a row wake it once.
                    while let Ok(s) = rx.try_recv() {
                        if matches!(s, Signal::Stop) {
                            return;
                        }
                    }
                }
                Ok(Signal::Stop) | Err(RecvTimeoutError::Disconnected) => return,
            }
        })?;
        Ok(Self { tx, thread: Some(thread) })
    }

    /// Something was queued: deliver it now. Never blocks.
    pub fn wake(&self) {
        let _ = self.tx.send(Signal::Wake);
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.tx.send(Signal::Stop);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outbox::tests::status;
    use crate::transport::Response;
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    const REF: &str = "https://eddn.edcd.io/schemas/journal/1/test";

    /// Answers from a script; shared so the test can look while the courier owns it.
    #[derive(Clone, Default)]
    struct Script(Arc<Mutex<(VecDeque<Result<Response, String>>, usize)>>);
    impl Script {
        fn new(a: impl IntoIterator<Item = Result<Response, String>>) -> Self {
            Self(Arc::new(Mutex::new((a.into_iter().collect(), 0))))
        }
        fn calls(&self) -> usize {
            self.0.lock().unwrap().1
        }
    }
    impl Transport for Script {
        fn post(&self, _: &[u8]) -> Result<Response, String> {
            let mut g = self.0.lock().unwrap();
            g.1 += 1;
            g.0.pop_front().expect("unexpected post")
        }
    }

    #[test]
    fn a_wall_clock_jump_does_not_cause_an_early_retry() {
        let t0 = Instant::now();
        let script = Script::new([status(503), status(200)]);
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let mut c = Courier::new(o, Box::new(script.clone()), t0);
        assert_eq!(c.pass(1000, t0).unwrap().retrying, 1, "a new message goes straight away");
        // The wall clock jumps an hour ahead 5 s later: the message's next_try has passed,
        // but only 5 s went by on the monotonic clock.
        assert_eq!(c.pass(1000 + 3600, t0 + Duration::from_secs(5)).unwrap(), Delivery::default());
        assert_eq!(script.calls(), 1);
        assert!(c.wait(1000 + 3600, t0 + Duration::from_secs(5)) >= Duration::from_secs(MIN_RETRY_SECS as u64 - 5));
        assert_eq!(c.pass(1000 + 3600, t0 + MIN_RETRY).unwrap().sent, 1);
    }

    #[test]
    fn messages_left_from_the_last_run_wait_a_minute_after_start() {
        let t0 = Instant::now();
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 0).unwrap();
        o.deliver_due(&Script::new([status(413)]), 0, true).unwrap();
        let script = Script::new([status(200)]);
        let mut c = Courier::new(o, Box::new(script.clone()), t0);
        assert_eq!(c.pass(10_000, t0 + Duration::from_secs(1)).unwrap(), Delivery::default());
        assert_eq!(c.pass(10_000, t0 + MIN_RETRY).unwrap().sent, 1);
    }

    #[test]
    fn a_new_message_is_not_held_by_an_earlier_failure() {
        let t0 = Instant::now();
        let script = Script::new([status(413), status(200)]);
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"old", 0).unwrap();
        let mut c = Courier::new(o, Box::new(script.clone()), t0);
        c.pass(0, t0).unwrap();
        c.outbox().enqueue(REF, b"new", 1).unwrap();
        assert_eq!(c.pass(1, t0 + Duration::from_secs(1)).unwrap().sent, 1);
        assert_eq!(c.outbox().pending().unwrap(), 1, "the failed one still waits");
    }
}
