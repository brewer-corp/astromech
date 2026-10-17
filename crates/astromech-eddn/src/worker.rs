//! Delivery in the background. The worker thread owns its own [`Outbox`] connection and its
//! transport, so the journal side ([`crate::Sender`], [`crate::Feed`]) never shares a lock
//! with a network call: queuing a message is one SQLite insert, whatever the gateway is doing.
//!
//! A retry needs a minute on the worker's own monotonic clock as well as on the wall clock
//! (each message's `next_try`), so a clock jump can't cause an early retry. The worker starts
//! with that minute already running: after a restart, messages left from the last run wait a
//! minute too.
//!
//! Stopping never waits for the network: dropping the [`Worker`] sets a flag the pass checks
//! before each post and returns at once. A post in flight finishes (or times out) on the
//! detached thread; its attempt and retry time were saved before it started.

use crate::outbox::{Delivery, Outbox, MIN_RETRY_SECS};
use crate::transport::Transport;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError, Sender as Tx};
use std::sync::Arc;
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
    /// Set to end the pass in progress before its next post.
    stop: Arc<AtomicBool>,
    /// The user's switch: while off, nothing is posted and the queue is discarded.
    sharing: Arc<AtomicBool>,
}

impl Courier {
    pub fn new(
        outbox: Outbox,
        transport: Box<dyn Transport + Send>,
        started: Instant,
        sharing: Arc<AtomicBool>,
    ) -> Self {
        Self { outbox, transport, hold_until: started + MIN_RETRY, stop: Arc::default(), sharing }
    }

    /// Deliver what's due: `wall` is Unix seconds, `mono` the monotonic time of the same moment.
    /// Switched off, it posts nothing and discards the queue instead.
    pub fn pass(&mut self, wall: i64, mono: Instant) -> rusqlite::Result<Delivery> {
        if !self.sharing.load(Ordering::SeqCst) {
            let n = self.outbox.discard_all(wall)?;
            return Ok(Delivery { gave_up: n, ..Default::default() });
        }
        let stop = || self.stop.load(Ordering::SeqCst) || !self.sharing.load(Ordering::SeqCst);
        let d = self.outbox.deliver_due(self.transport.as_ref(), wall, mono >= self.hold_until, &stop)?;
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

/// What a [`Worker`] runs: one delivery pass at a time (EDDN's [`Courier`], EDSM's
/// [`crate::edsm::Courier`]).
pub trait Pass: Send + 'static {
    /// Deliver what's due: `wall` is Unix seconds, `mono` the monotonic time of the same moment.
    fn pass(&mut self, wall: i64, mono: Instant) -> rusqlite::Result<Delivery>;
    /// How long to sleep after a pass before the next one is worth making.
    fn wait(&self, wall: i64, mono: Instant) -> Duration;
    /// Set to end the pass in progress before its next post.
    fn stop_flag(&self) -> Arc<AtomicBool>;
}

impl Pass for Courier {
    fn pass(&mut self, wall: i64, mono: Instant) -> rusqlite::Result<Delivery> {
        Courier::pass(self, wall, mono)
    }
    fn wait(&self, wall: i64, mono: Instant) -> Duration {
        Courier::wait(self, wall, mono)
    }
    fn stop_flag(&self) -> Arc<AtomicBool> {
        self.stop.clone()
    }
}

enum Signal {
    Wake,
    Stop,
}

/// The delivery thread. Dropping it returns at once; the thread ends before its next post.
pub struct Worker {
    tx: Tx<Signal>,
    stop: Arc<AtomicBool>,
}

impl Worker {
    /// Start delivering to `service` ("EDDN", "EDSM": the thread name and log prefix). `log`
    /// gets each pass that did something, and database errors.
    pub fn spawn(
        mut courier: impl Pass,
        service: &'static str,
        log: impl Fn(&str) + Send + 'static,
    ) -> std::io::Result<Self> {
        let (tx, rx) = channel();
        let stop = courier.stop_flag();
        std::thread::Builder::new().name(service.to_ascii_lowercase()).spawn(move || loop {
            match courier.pass(wall_now(), Instant::now()) {
                Ok(d) if d != Delivery::default() => log(&format!(
                    "{service}: {} sent, {} rejected, {} to retry, {} given up{}",
                    d.sent,
                    d.rejected,
                    d.retrying,
                    d.gave_up,
                    if d.stopped { "; service unreachable, waiting" } else { "" }
                )),
                Ok(_) => {}
                Err(e) => log(&format!("{service} outbox: {e}")),
            }
            if stop_seen(&courier) {
                return;
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
        Ok(Self { tx, stop })
    }

    /// A handle that does what [`Worker::wake`] does, for code that outlives a borrow of the
    /// worker (the EDSM switch wakes the uploader when the credentials change).
    pub fn waker(&self) -> impl Fn() + Send + Sync + 'static {
        let tx = self.tx.clone();
        move || {
            let _ = tx.send(Signal::Wake);
        }
    }

    /// Something was queued: deliver it now. Never blocks.
    pub fn wake(&self) {
        let _ = self.tx.send(Signal::Wake);
    }
}

fn stop_seen(p: &impl Pass) -> bool {
    p.stop_flag().load(Ordering::SeqCst)
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = self.tx.send(Signal::Stop);
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
        let mut c = Courier::new(o, Box::new(script.clone()), t0, Arc::new(AtomicBool::new(true)));
        assert_eq!(c.pass(1000, t0).unwrap().retrying, 1, "a new message goes straight away");
        // The wall clock jumps 20 minutes ahead 5 s later: the message's next_try has passed,
        // but only 5 s went by on the monotonic clock.
        let jumped = 1000 + 20 * 60;
        assert_eq!(c.pass(jumped, t0 + Duration::from_secs(5)).unwrap(), Delivery::default());
        assert_eq!(script.calls(), 1);
        assert!(c.wait(jumped, t0 + Duration::from_secs(5)) >= Duration::from_secs(MIN_RETRY_SECS as u64 - 5));
        assert_eq!(c.pass(jumped, t0 + MIN_RETRY).unwrap().sent, 1);
    }

    #[test]
    fn messages_left_from_the_last_run_wait_a_minute_after_start() {
        let t0 = Instant::now();
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 0).unwrap();
        o.deliver_due(&Script::new([status(413)]), 0, true, &crate::outbox::tests::no_stop).unwrap();
        let script = Script::new([status(200)]);
        let mut c = Courier::new(o, Box::new(script.clone()), t0, Arc::new(AtomicBool::new(true)));
        assert_eq!(c.pass(600, t0 + Duration::from_secs(1)).unwrap(), Delivery::default());
        assert_eq!(c.pass(600, t0 + MIN_RETRY).unwrap().sent, 1);
    }

    #[test]
    fn a_new_message_is_not_held_by_an_earlier_failure() {
        let t0 = Instant::now();
        let script = Script::new([status(413), status(200)]);
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"old", 0).unwrap();
        let mut c = Courier::new(o, Box::new(script.clone()), t0, Arc::new(AtomicBool::new(true)));
        c.pass(0, t0).unwrap();
        c.outbox().enqueue(REF, b"new", 1).unwrap();
        assert_eq!(c.pass(1, t0 + Duration::from_secs(1)).unwrap().sent, 1);
        assert_eq!(c.outbox().pending().unwrap(), 1, "the failed one still waits");
    }

    #[test]
    fn switched_off_it_posts_nothing_and_discards_the_queue() {
        let t0 = Instant::now();
        let script = Script::new([]);
        let sharing = Arc::new(AtomicBool::new(true));
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"a", 1000).unwrap();
        o.enqueue(REF, b"b", 1000).unwrap();
        let mut c = Courier::new(o, Box::new(script.clone()), t0, sharing.clone());
        sharing.store(false, Ordering::SeqCst);
        assert_eq!(c.pass(1000, t0).unwrap().gave_up, 2);
        assert_eq!(script.calls(), 0);
        assert_eq!(c.outbox().pending().unwrap(), 0);
        let row = &c.outbox().recent(1).unwrap()[0];
        assert_eq!((row.outcome.as_str(), row.detail.as_str()), ("discarded", "switched off: 2 queued discarded"));
        // Nothing left: no more rows.
        c.pass(1001, t0).unwrap();
        assert_eq!(c.outbox().recent(10).unwrap().len(), 1);
    }

    #[test]
    fn switching_off_mid_pass_stops_before_the_next_post() {
        let t0 = Instant::now();
        let sharing = Arc::new(AtomicBool::new(true));
        /// Switches sharing off during its first post.
        struct OffAfterOne(Arc<AtomicBool>, Arc<Mutex<usize>>);
        impl Transport for OffAfterOne {
            fn post(&self, _: &[u8]) -> Result<Response, String> {
                *self.1.lock().unwrap() += 1;
                self.0.store(false, Ordering::SeqCst);
                Ok(Response { status: 200, body: "OK".into() })
            }
        }
        let calls = Arc::new(Mutex::new(0));
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"a", 1000).unwrap();
        o.enqueue(REF, b"b", 1000).unwrap();
        let mut c = Courier::new(o, Box::new(OffAfterOne(sharing.clone(), calls.clone())), t0, sharing);
        assert_eq!(c.pass(1000, t0).unwrap().sent, 1);
        assert_eq!(*calls.lock().unwrap(), 1);
    }
}
