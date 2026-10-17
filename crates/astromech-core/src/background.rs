//! Lookups for the reaction engine's remote callouts (known biology, bodies known to EDSM). The
//! engine asks on the pipeline thread, which must never wait on the network: the first ask for
//! a key starts the lookup on its own thread and answers None, later asks answer from what it
//! found. A failed lookup is not an answer: the key stays unknown (None) and an ask after the
//! back-off tries again.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// At most this many keys' answers are kept.
const KEEP: usize = 64;

/// How long after a failed lookup the next ask tries again.
pub const RETRY_AFTER: Duration = Duration::from_secs(15);

enum Lookup<V> {
    Pending,
    Done(V),
    /// Failed at this time: unknown, not "nothing there".
    Failed(Instant),
}

/// Background lookups by key, each on its own named thread.
pub struct Background<V> {
    thread: &'static str,
    entries: Arc<Mutex<HashMap<String, Lookup<V>>>>,
    pub(crate) retry_after: Duration,
}

impl<V: Clone + Send + 'static> Background<V> {
    /// `thread` names the lookup threads.
    pub fn new(thread: &'static str) -> Self {
        Self { thread, entries: Arc::default(), retry_after: RETRY_AFTER }
    }

    /// `key`'s answer if it is in; otherwise starts `look_up` (unless one is out, or the last
    /// failed within the back-off) and returns None. Never blocks on the lookup. An `Err` from
    /// `look_up` is the failure, already logged by it.
    pub fn get<E: Send + 'static>(
        &self,
        key: &str,
        look_up: impl FnOnce() -> Result<V, E> + Send + 'static,
    ) -> Option<V> {
        let mut entries = self.entries.lock().unwrap();
        match entries.get(key) {
            Some(Lookup::Done(v)) => return Some(v.clone()),
            Some(Lookup::Pending) => return None,
            Some(Lookup::Failed(at)) if at.elapsed() < self.retry_after => return None,
            Some(Lookup::Failed(_)) | None => {}
        }
        if entries.len() >= KEEP {
            entries.retain(|_, l| matches!(l, Lookup::Pending));
        }
        entries.insert(key.to_string(), Lookup::Pending);
        let (all, key2) = (self.entries.clone(), key.to_string());
        let spawned = std::thread::Builder::new().name(self.thread.into()).spawn(move || {
            let found = match look_up() {
                Ok(v) => Lookup::Done(v),
                Err(_) => Lookup::Failed(Instant::now()),
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
}
