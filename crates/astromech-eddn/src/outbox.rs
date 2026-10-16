//! The persistent outbox and send log (SQLite, `eddn.sqlite` in the data folder).
//!
//! Retry rules from EDCD `docs/Developers.md`: wait at least a minute before retrying, never
//! retry a `400` or `426`, don't retry forever, and don't let one bad message hold up the rest
//! (each due message is tried on its own).

use crate::transport::Transport;
use rusqlite::{Connection, OptionalExtension, Result};
use serde::Serialize;

/// The first retry waits this long; each later one doubles it, up to [`MAX_BACKOFF_SECS`].
pub const MIN_RETRY_SECS: i64 = 60;
pub const MAX_BACKOFF_SECS: i64 = 3600;
/// Tries per message (the first send plus retries) before it's dropped as undeliverable.
pub const MAX_ATTEMPTS: i64 = 8;
/// Send-log rows kept.
pub const LOG_KEEP: i64 = 500;

const MIGRATIONS: &[&str] = &[
    // v1
    "CREATE TABLE outbox (
        id INTEGER PRIMARY KEY,
        schema_ref TEXT NOT NULL,
        body BLOB NOT NULL,
        attempts INTEGER NOT NULL DEFAULT 0,
        next_try INTEGER NOT NULL,
        created INTEGER NOT NULL
     );
     CREATE TABLE send_log (
        id INTEGER PRIMARY KEY,
        at INTEGER NOT NULL,
        schema_ref TEXT NOT NULL,
        outcome TEXT NOT NULL,
        detail TEXT NOT NULL
     );",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    /// Accepted by the gateway (`200`).
    Sent,
    /// `400` or `426`: dropped, never retried.
    Rejected,
    /// Failed; queued for another try.
    Retry,
    /// Failed [`MAX_ATTEMPTS`] times; dropped.
    GaveUp,
    /// Not queued at all (beta, too large, location mismatch, …).
    Dropped,
}

impl Outcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Outcome::Sent => "sent",
            Outcome::Rejected => "rejected",
            Outcome::Retry => "retry",
            Outcome::GaveUp => "gave_up",
            Outcome::Dropped => "dropped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LogEntry {
    /// Unix seconds.
    pub at: i64,
    pub schema_ref: String,
    pub outcome: String,
    pub detail: String,
}

/// What one delivery pass did.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Delivery {
    pub sent: usize,
    pub rejected: usize,
    pub retrying: usize,
    pub gave_up: usize,
}

/// Seconds to wait after the `attempts`-th failure.
pub fn backoff(attempts: i64) -> i64 {
    let doublings = (attempts - 1).clamp(0, 20) as u32;
    (MIN_RETRY_SECS << doublings).min(MAX_BACKOFF_SECS)
}

pub struct Outbox {
    conn: Connection,
}

impl Outbox {
    pub fn open(path: &std::path::Path) -> Result<Self> {
        Self::init(Connection::open(path)?)
    }
    pub fn in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }
    fn init(conn: Connection) -> Result<Self> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        let v: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        for (i, m) in MIGRATIONS.iter().enumerate().skip(v as usize) {
            conn.execute_batch(m)?;
            conn.pragma_update(None, "user_version", (i + 1) as i64)?;
        }
        Ok(Self { conn })
    }

    /// Queue a gzipped upload; it's due immediately.
    pub fn enqueue(&self, schema_ref: &str, gzipped: &[u8], now: i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO outbox(schema_ref, body, next_try, created) VALUES (?1, ?2, ?3, ?3)",
            (schema_ref, gzipped, now),
        )?;
        Ok(())
    }

    pub fn pending(&self) -> Result<usize> {
        self.conn.query_row("SELECT COUNT(*) FROM outbox", [], |r| r.get::<_, i64>(0)).map(|n| n as usize)
    }

    /// When the next queued message is due, if any.
    pub fn next_due(&self) -> Result<Option<i64>> {
        self.conn.query_row("SELECT MIN(next_try) FROM outbox", [], |r| r.get(0)).optional().map(Option::flatten)
    }

    pub fn log(&self, at: i64, schema_ref: &str, outcome: Outcome, detail: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO send_log(at, schema_ref, outcome, detail) VALUES (?1, ?2, ?3, ?4)",
            (at, schema_ref, outcome.as_str(), detail),
        )?;
        self.conn.execute("DELETE FROM send_log WHERE id <= (SELECT MAX(id) FROM send_log) - ?1", [LOG_KEEP])?;
        Ok(())
    }

    /// Newest first.
    pub fn recent(&self, limit: usize) -> Result<Vec<LogEntry>> {
        let mut st =
            self.conn.prepare("SELECT at, schema_ref, outcome, detail FROM send_log ORDER BY id DESC LIMIT ?1")?;
        let rows = st.query_map([limit as i64], |r| {
            Ok(LogEntry { at: r.get(0)?, schema_ref: r.get(1)?, outcome: r.get(2)?, detail: r.get(3)? })
        })?;
        rows.collect()
    }

    /// Try every message due at `now`, oldest first.
    pub fn deliver_due(&self, transport: &dyn Transport, now: i64) -> Result<Delivery> {
        let due: Vec<(i64, String, Vec<u8>, i64)> = {
            let mut st = self
                .conn
                .prepare("SELECT id, schema_ref, body, attempts FROM outbox WHERE next_try <= ?1 ORDER BY id")?;
            let rows = st.query_map([now], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
            rows.collect::<Result<_>>()?
        };
        let mut d = Delivery::default();
        for (id, schema_ref, body, attempts) in due {
            let attempts = attempts + 1;
            let wait = backoff(attempts);
            // Count the attempt and schedule the retry before posting, so a failure anywhere
            // after this point (a panic, a database error) can't leave the message due again.
            self.conn
                .execute("UPDATE outbox SET attempts = ?2, next_try = ?3 WHERE id = ?1", (id, attempts, now + wait))?;
            let failure = match transport.post(&body) {
                Ok(r) if r.status == 200 => {
                    self.remove(id)?;
                    self.log(now, &schema_ref, Outcome::Sent, "")?;
                    d.sent += 1;
                    continue;
                }
                Ok(r) if r.status == 400 || r.status == 426 => {
                    self.remove(id)?;
                    self.log(now, &schema_ref, Outcome::Rejected, &format!("HTTP {}: {}", r.status, r.body.trim()))?;
                    d.rejected += 1;
                    continue;
                }
                Ok(r) => format!("HTTP {}: {}", r.status, r.body.trim()),
                Err(e) => e,
            };
            if attempts >= MAX_ATTEMPTS {
                self.remove(id)?;
                self.log(now, &schema_ref, Outcome::GaveUp, &format!("after {attempts} tries; last: {failure}"))?;
                d.gave_up += 1;
            } else {
                self.log(now, &schema_ref, Outcome::Retry, &format!("{failure}; retry in {wait} s"))?;
                d.retrying += 1;
            }
        }
        Ok(d)
    }

    fn remove(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM outbox WHERE id = ?1", [id])?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::transport::Response;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    /// Answers from a script and records every body it was given.
    #[derive(Default)]
    pub struct FakeTransport {
        pub answers: RefCell<VecDeque<Result<Response, String>>>,
        pub posted: RefCell<Vec<Vec<u8>>>,
    }

    impl FakeTransport {
        pub fn new(answers: impl IntoIterator<Item = Result<Response, String>>) -> Self {
            Self { answers: RefCell::new(answers.into_iter().collect()), posted: Default::default() }
        }
        pub fn calls(&self) -> usize {
            self.posted.borrow().len()
        }
    }

    impl Transport for FakeTransport {
        fn post(&self, b: &[u8]) -> std::result::Result<Response, String> {
            self.posted.borrow_mut().push(b.to_vec());
            self.answers.borrow_mut().pop_front().expect("unexpected post")
        }
    }

    pub fn status(code: u16) -> std::result::Result<Response, String> {
        Ok(Response { status: code, body: if code == 200 { "OK".into() } else { format!("FAIL: {code}") } })
    }

    const REF: &str = "https://eddn.edcd.io/schemas/journal/1/test";

    #[test]
    fn sent_messages_leave_the_outbox_and_are_logged() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::new([status(200)]);
        assert_eq!(o.deliver_due(&t, 1000).unwrap(), Delivery { sent: 1, ..Default::default() });
        assert_eq!(o.pending().unwrap(), 0);
        assert_eq!(o.recent(10).unwrap()[0].outcome, "sent");
    }

    #[test]
    fn http_400_is_not_retried() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::new([status(400)]);
        assert_eq!(o.deliver_due(&t, 1000).unwrap().rejected, 1);
        assert_eq!(o.pending().unwrap(), 0, "a 400 must leave the outbox");
        assert_eq!(o.deliver_due(&t, 1000 + 10 * MAX_BACKOFF_SECS).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 1, "never posted again");
        let log = o.recent(1).unwrap();
        assert_eq!((log[0].outcome.as_str(), log[0].detail.as_str()), ("rejected", "HTTP 400: FAIL: 400"));
    }

    #[test]
    fn http_426_is_not_retried() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::new([status(426)]);
        assert_eq!(o.deliver_due(&t, 1000).unwrap().rejected, 1);
        assert_eq!(o.deliver_due(&t, 1000 + 10 * MAX_BACKOFF_SECS).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 1);
    }

    #[test]
    fn other_failures_retry_no_sooner_than_one_minute() {
        for failure in [status(503), status(413), status(408), Err("connection refused".to_string())] {
            let o = Outbox::in_memory().unwrap();
            o.enqueue(REF, b"m", 1000).unwrap();
            let t = FakeTransport::new([failure.clone(), status(200)]);
            assert_eq!(o.deliver_due(&t, 1000).unwrap().retrying, 1, "{failure:?}");
            assert_eq!(o.pending().unwrap(), 1);
            assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS));
            // Every second up to 59 s later: nothing is posted.
            for s in 1..MIN_RETRY_SECS {
                assert_eq!(o.deliver_due(&t, 1000 + s).unwrap(), Delivery::default());
            }
            assert_eq!(t.calls(), 1, "{failure:?} retried too soon");
            assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS).unwrap().sent, 1);
            assert_eq!(t.calls(), 2);
        }
    }

    #[test]
    fn backoff_doubles_from_a_minute_and_gives_up() {
        assert_eq!((1..=8).map(backoff).collect::<Vec<_>>(), [60, 120, 240, 480, 960, 1920, 3600, 3600]);
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 0).unwrap();
        let t = FakeTransport::new((0..MAX_ATTEMPTS).map(|_| status(503)));
        let mut now = 0;
        for _ in 0..MAX_ATTEMPTS {
            now = o.next_due().unwrap().unwrap();
            o.deliver_due(&t, now).unwrap();
        }
        assert_eq!(o.pending().unwrap(), 0);
        assert_eq!(t.calls() as i64, MAX_ATTEMPTS);
        assert_eq!(o.recent(1).unwrap()[0].outcome, "gave_up");
        assert!(now >= (1..MAX_ATTEMPTS).map(backoff).sum::<i64>());
    }

    struct PanickingTransport;
    impl Transport for PanickingTransport {
        fn post(&self, _: &[u8]) -> std::result::Result<Response, String> {
            panic!("transport bug")
        }
    }

    #[test]
    fn a_failure_mid_send_does_not_leave_the_message_due() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| o.deliver_due(&PanickingTransport, 1000)));
        assert!(r.is_err());
        assert_eq!(o.pending().unwrap(), 1, "kept for a retry");
        assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS), "not due again straight away");
        let t = FakeTransport::default();
        assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS - 1).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 0);
    }

    #[test]
    fn one_bad_message_does_not_block_the_others() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"bad", 0).unwrap();
        o.enqueue(REF, b"good", 0).unwrap();
        let t = FakeTransport::new([status(503), status(200)]);
        let d = o.deliver_due(&t, 0).unwrap();
        assert_eq!((d.retrying, d.sent), (1, 1));
        assert_eq!(*t.posted.borrow(), [b"bad".to_vec(), b"good".to_vec()]);
    }

    #[test]
    fn outbox_survives_a_restart() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("eddn.sqlite");
        Outbox::open(&p).unwrap().enqueue(REF, b"m", 5).unwrap();
        assert_eq!(Outbox::open(&p).unwrap().pending().unwrap(), 1);
    }

    #[test]
    fn send_log_is_capped() {
        let o = Outbox::in_memory().unwrap();
        for i in 0..LOG_KEEP + 20 {
            o.log(i, REF, Outcome::Dropped, &i.to_string()).unwrap();
        }
        let all = o.recent(10_000).unwrap();
        assert_eq!(all.len() as i64, LOG_KEEP);
        assert_eq!(all[0].detail, (LOG_KEEP + 19).to_string());
    }
}
