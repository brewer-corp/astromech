//! Lookups for the reaction engine's remote callouts (known biology, bodies known to EDSM). The
//! engine asks on the pipeline thread, which must never wait on the network: the first ask for
//! a key starts the lookup on its own thread and answers None, later asks answer from what it
//! found. A failed lookup is not an answer: the key stays unknown (None) and an ask after the
//! back-off tries again.
//!
//! Retry cadence: the arrival rules ask on every clock tick (once a second) while a callout waits
//! for a late answer, for up to a minute (`astromech_announce::known_bodies::LATE_S`,
//! `astromech_announce::biology::LATE_KNOWN_S`). A lookup that keeps failing is started again at
//! most once per [`RETRY_AFTER`], counted from its failure, so an arrival costs at most
//! 1 + 60 s / 15 s = 5 lookups (and 5 logged failures), fewer when each takes a while to fail.
//! Each goes through the remote-data client, which rate limits them with everything else.
//!
//! An answer is kept for [`KEEP_FOR`]; the next ask after that looks the key up again (and
//! answers None until it is in), so a long session doesn't keep an answer from its start. The
//! lookup goes through the client's cache, whose lifetime for these requests is no longer
//! (`astromech_data::ttl::SCANNED` for EDSM's bodies, `ttl::SEARCH` for Spansh's), so the new
//! answer is the service's.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// At most this many keys' answers are kept.
const KEEP: usize = 64;

/// How long after a failed lookup the next ask tries again.
pub const RETRY_AFTER: Duration = Duration::from_secs(15);

/// How long a found answer is used before the next ask looks it up again.
pub const KEEP_FOR: Duration = Duration::from_secs(3600);

enum Lookup<V> {
    Pending,
    /// Found at this time.
    Done(V, Instant),
    /// Failed at this time: unknown, not "nothing there".
    Failed(Instant),
}

pub(crate) type Clock = Arc<dyn Fn() -> Instant + Send + Sync>;

/// Background lookups by key, each on its own named thread.
pub struct Background<V> {
    thread: &'static str,
    entries: Arc<Mutex<HashMap<String, Lookup<V>>>>,
    pub(crate) retry_after: Duration,
    pub(crate) keep_for: Duration,
    /// Now (tests set their own).
    clock: Clock,
}

impl<V: Clone + Send + 'static> Background<V> {
    /// `thread` names the lookup threads.
    pub fn new(thread: &'static str) -> Self {
        Self {
            thread,
            entries: Arc::default(),
            retry_after: RETRY_AFTER,
            keep_for: KEEP_FOR,
            clock: Arc::new(Instant::now),
        }
    }

    /// `key`'s answer if it is in and not older than `keep_for`; otherwise starts `look_up`
    /// (unless one is out, or the last failed within the back-off) and returns None. Never blocks
    /// on the lookup. An `Err` from `look_up` is the failure, already logged by it.
    pub fn get<E: Send + 'static>(
        &self,
        key: &str,
        look_up: impl FnOnce() -> Result<V, E> + Send + 'static,
    ) -> Option<V> {
        let now = (self.clock)();
        let mut entries = self.entries.lock().unwrap();
        match entries.get(key) {
            Some(Lookup::Done(v, at)) if now.saturating_duration_since(*at) < self.keep_for => return Some(v.clone()),
            Some(Lookup::Pending) => return None,
            Some(Lookup::Failed(at)) if now.saturating_duration_since(*at) < self.retry_after => return None,
            Some(Lookup::Done(..) | Lookup::Failed(_)) | None => {}
        }
        if entries.len() >= KEEP {
            entries.retain(|_, l| matches!(l, Lookup::Pending));
        }
        entries.insert(key.to_string(), Lookup::Pending);
        let (all, key2, clock) = (self.entries.clone(), key.to_string(), self.clock.clone());
        let spawned = std::thread::Builder::new().name(self.thread.into()).spawn(move || {
            let found = match look_up() {
                Ok(v) => Lookup::Done(v, clock()),
                Err(_) => Lookup::Failed(clock()),
            };
            all.lock().unwrap().insert(key2, found);
        });
        if spawned.is_err() {
            entries.remove(key);
        }
        None
    }

    /// The last lookup of `key` failed (tests).
    #[cfg(test)]
    pub(crate) fn failed(&self, key: &str) -> bool {
        matches!(self.entries.lock().unwrap().get(key), Some(Lookup::Failed(_)))
    }

    /// Use `clock` for now (tests).
    #[cfg(test)]
    pub(crate) fn set_clock(&mut self, clock: Clock) {
        self.clock = clock;
    }

    /// Wait for the lookup that is out for `key` to finish (tests).
    #[cfg(test)]
    pub(crate) fn settle(&self, key: &str) {
        let deadline = Instant::now() + Duration::from_secs(10);
        while matches!(self.entries.lock().unwrap().get(key), Some(Lookup::Pending)) {
            assert!(Instant::now() < deadline, "the lookup never finished");
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A background whose clock the test moves, with the lookups it started counted.
    fn fake(start: Instant) -> (Background<u32>, Arc<Mutex<Instant>>) {
        let now = Arc::new(Mutex::new(start));
        let mut b = Background::new("test-lookup");
        let n2 = now.clone();
        b.set_clock(Arc::new(move || *n2.lock().unwrap()));
        (b, now)
    }

    /// Asked once a second for as long as an arrival callout waits for a late answer, with the
    /// service down: one lookup at the arrival and one per back-off after a failure.
    #[test]
    fn a_failing_lookup_is_retried_once_per_back_off_while_a_callout_waits() {
        let window = astromech_announce::known_bodies::LATE_S;
        assert_eq!(window, astromech_announce::biology::LATE_KNOWN_S, "the same window for both rules");
        let start = Instant::now();
        let (b, now) = fake(start);
        let tries = Arc::new(AtomicUsize::new(0));
        for s in 0..=window as u64 {
            *now.lock().unwrap() = start + Duration::from_secs(s);
            let t = tries.clone();
            assert_eq!(
                b.get("Sol", move || {
                    t.fetch_add(1, Ordering::SeqCst);
                    Err::<u32, ()>(())
                }),
                None
            );
            b.settle("Sol");
        }
        // At 0, 15, 30, 45 and 60 s.
        let expected = 1 + window as usize / RETRY_AFTER.as_secs() as usize;
        assert_eq!((tries.load(Ordering::SeqCst), expected), (5, 5));
    }

    /// An answer is used until it is `KEEP_FOR` old; the next ask then looks it up again and
    /// answers from the new lookup once it is in.
    #[test]
    fn an_answer_is_looked_up_again_once_it_is_kept_for_long_enough() {
        let start = Instant::now();
        let (b, now) = fake(start);
        let tries = Arc::new(AtomicUsize::new(0));
        let ask = |b: &Background<u32>| {
            let t = tries.clone();
            b.get("Sol", move || Ok::<u32, ()>(t.fetch_add(1, Ordering::SeqCst) as u32 + 7))
        };
        assert_eq!(ask(&b), None);
        b.settle("Sol");
        assert_eq!(ask(&b), Some(7));
        *now.lock().unwrap() = start + KEEP_FOR - Duration::from_secs(1);
        assert_eq!(ask(&b), Some(7), "still fresh");
        assert_eq!(tries.load(Ordering::SeqCst), 1);

        *now.lock().unwrap() = start + KEEP_FOR;
        assert_eq!(ask(&b), None, "expired: looked up again");
        b.settle("Sol");
        assert_eq!(ask(&b), Some(8), "the new answer");
        assert_eq!(tries.load(Ordering::SeqCst), 2);
    }
}
