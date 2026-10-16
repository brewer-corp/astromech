//! The persistent outbox and send log (SQLite, `eddn.sqlite` in the data folder).
//!
//! Retry rules from EDCD `docs/Developers.md`: wait at least a minute before retrying, never
//! retry a `400` or `426`, don't retry forever, and don't let one bad message hold up the rest
//! (each due message is tried on its own). When the gateway itself is down (no connection, a
//! `408` or a `5xx`) a pass stops at that message instead of trying every queued one.
//!
//! Two connections use the file at once: the [`crate::Sender`] queues and the
//! [`crate::worker`] delivers. Each statement commits on its own, so no transaction is open
//! while a message is being posted.

use crate::transport::Transport;
use rusqlite::{Connection, OptionalExtension, Result};
use serde::Serialize;

/// The first retry waits this long; each later one doubles it, up to [`MAX_BACKOFF_SECS`].
pub const MIN_RETRY_SECS: i64 = 60;
pub const MAX_BACKOFF_SECS: i64 = 3600;
/// Tries per message (the first send plus retries) before it's dropped as undeliverable.
pub const MAX_ATTEMPTS: i64 = 8;
/// Send-log rows kept for messages that reached the gateway (sent, rejected, retry, gave up).
pub const LOG_KEEP: i64 = 500;
/// Send-log rows kept for messages never queued (`dropped`), counted apart so a burst of them
/// can't push the gateway's answers out of the log.
pub const DROP_KEEP: i64 = 200;
/// How long a statement waits for the other connection's write to finish.
const BUSY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

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
    /// The pass stopped early: the gateway looks down, so the rest wait for the next retry time.
    pub stopped: bool,
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
        conn.busy_timeout(BUSY_TIMEOUT)?;
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
        // Trim only the class this row belongs to (dropped, or the gateway's answers).
        let (dropped, keep) = if outcome == Outcome::Dropped { (true, DROP_KEEP) } else { (false, LOG_KEEP) };
        self.conn.execute(
            "DELETE FROM send_log WHERE (outcome = 'dropped') = ?1 AND id <= (
                SELECT id FROM send_log WHERE (outcome = 'dropped') = ?1 ORDER BY id DESC LIMIT 1 OFFSET ?2)",
            (dropped, keep),
        )?;
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

    /// Try the messages due at `now`, oldest first. With `retries` false only messages never
    /// tried yet are sent (the caller's own clock says a minute hasn't passed since a failure).
    ///
    /// A connection error, `408` or `5xx` means the gateway is down or overloaded, so the pass
    /// stops there and every other due message waits as long as the one that failed.
    pub fn deliver_due(&self, transport: &dyn Transport, now: i64, retries: bool) -> Result<Delivery> {
        let due: Vec<(i64, String, Vec<u8>, i64)> = {
            let mut st = self.conn.prepare(
                "SELECT id, schema_ref, body, attempts FROM outbox
                 WHERE next_try <= ?1 AND (?2 OR attempts = 0) ORDER BY id",
            )?;
            let rows = st.query_map((now, retries), |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
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
            let (failure, outage) = match transport.post(&body) {
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
                Ok(r) => (format!("HTTP {}: {}", r.status, r.body.trim()), r.status == 408 || r.status >= 500),
                Err(e) => (e, true),
            };
            if attempts >= MAX_ATTEMPTS {
                self.remove(id)?;
                self.log(now, &schema_ref, Outcome::GaveUp, &format!("after {attempts} tries; last: {failure}"))?;
                d.gave_up += 1;
            } else {
                self.log(now, &schema_ref, Outcome::Retry, &format!("{failure}; retry in {wait} s"))?;
                d.retrying += 1;
            }
            if outage {
                // Not attempts: the others weren't tried, so they keep their tries.
                self.conn.execute(
                    "UPDATE outbox SET next_try = ?2 WHERE next_try <= ?1 AND id != ?3",
                    (now, now + wait, id),
                )?;
                d.stopped = true;
                break;
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
        assert_eq!(o.deliver_due(&t, 1000, true).unwrap(), Delivery { sent: 1, ..Default::default() });
        assert_eq!(o.pending().unwrap(), 0);
        assert_eq!(o.recent(10).unwrap()[0].outcome, "sent");
    }

    #[test]
    fn http_400_is_not_retried() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::new([status(400)]);
        assert_eq!(o.deliver_due(&t, 1000, true).unwrap().rejected, 1);
        assert_eq!(o.pending().unwrap(), 0, "a 400 must leave the outbox");
        assert_eq!(o.deliver_due(&t, 1000 + 10 * MAX_BACKOFF_SECS, true).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 1, "never posted again");
        let log = o.recent(1).unwrap();
        assert_eq!((log[0].outcome.as_str(), log[0].detail.as_str()), ("rejected", "HTTP 400: FAIL: 400"));
    }

    #[test]
    fn http_426_is_not_retried() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::new([status(426)]);
        assert_eq!(o.deliver_due(&t, 1000, true).unwrap().rejected, 1);
        assert_eq!(o.deliver_due(&t, 1000 + 10 * MAX_BACKOFF_SECS, true).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 1);
    }

    #[test]
    fn other_failures_retry_no_sooner_than_one_minute() {
        for failure in [status(503), status(413), status(408), Err("connection refused".to_string())] {
            let o = Outbox::in_memory().unwrap();
            o.enqueue(REF, b"m", 1000).unwrap();
            let t = FakeTransport::new([failure.clone(), status(200)]);
            assert_eq!(o.deliver_due(&t, 1000, true).unwrap().retrying, 1, "{failure:?}");
            assert_eq!(o.pending().unwrap(), 1);
            assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS));
            // Every second up to 59 s later: nothing is posted.
            for s in 1..MIN_RETRY_SECS {
                assert_eq!(o.deliver_due(&t, 1000 + s, true).unwrap(), Delivery::default());
            }
            assert_eq!(t.calls(), 1, "{failure:?} retried too soon");
            assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS, true).unwrap().sent, 1);
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
            o.deliver_due(&t, now, true).unwrap();
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
        let r =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| o.deliver_due(&PanickingTransport, 1000, true)));
        assert!(r.is_err());
        assert_eq!(o.pending().unwrap(), 1, "kept for a retry");
        assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS), "not due again straight away");
        let t = FakeTransport::default();
        assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS - 1, true).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 0);
    }

    #[test]
    fn one_bad_message_does_not_block_the_others() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"bad", 0).unwrap();
        o.enqueue(REF, b"good", 0).unwrap();
        let t = FakeTransport::new([status(413), status(200)]);
        let d = o.deliver_due(&t, 0, true).unwrap();
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
    fn a_gateway_outage_stops_the_pass_at_the_first_failure() {
        for failure in [Err("connection refused".to_string()), status(408), status(500), status(503)] {
            let o = Outbox::in_memory().unwrap();
            for b in [b"a", b"b", b"c"] {
                o.enqueue(REF, b, 1000).unwrap();
            }
            let t = FakeTransport::new([failure.clone()]);
            let d = o.deliver_due(&t, 1000, true).unwrap();
            assert_eq!(d, Delivery { retrying: 1, stopped: true, ..Default::default() }, "{failure:?}");
            assert_eq!(t.calls(), 1, "{failure:?}: one request, not one per queued message");
            // The untried messages wait for the retry time too, and keep their tries.
            assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS));
            assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS - 1, true).unwrap(), Delivery::default());
            let tries: Vec<i64> = {
                let mut st = o.conn.prepare("SELECT attempts FROM outbox ORDER BY id").unwrap();
                st.query_map([], |r| r.get(0)).unwrap().collect::<Result<_>>().unwrap()
            };
            assert_eq!(tries, [1, 0, 0]);
            let t = FakeTransport::new([status(200), status(200), status(200)]);
            assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS, true).unwrap().sent, 3);
        }
    }

    #[test]
    fn without_retries_only_untried_messages_are_sent() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"old", 0).unwrap();
        let t = FakeTransport::new([status(413)]);
        assert_eq!(o.deliver_due(&t, 0, true).unwrap().retrying, 1);
        o.enqueue(REF, b"new", 0).unwrap();
        let t = FakeTransport::new([status(200)]);
        // The wall clock says the retry is long due; the caller says it isn't allowed yet.
        assert_eq!(o.deliver_due(&t, 10 * MAX_BACKOFF_SECS, false).unwrap().sent, 1);
        assert_eq!(*t.posted.borrow(), [b"new".to_vec()]);
        assert_eq!(o.pending().unwrap(), 1);
    }

    #[test]
    fn send_log_is_capped() {
        let o = Outbox::in_memory().unwrap();
        for i in 0..LOG_KEEP + 20 {
            o.log(i, REF, Outcome::Sent, &i.to_string()).unwrap();
        }
        let all = o.recent(10_000).unwrap();
        assert_eq!(all.len() as i64, LOG_KEEP);
        assert_eq!(all[0].detail, (LOG_KEEP + 19).to_string());
    }

    #[test]
    fn dropped_rows_do_not_push_out_the_gateway_answers() {
        let o = Outbox::in_memory().unwrap();
        o.log(1, REF, Outcome::Sent, "s").unwrap();
        o.log(2, REF, Outcome::Rejected, "r").unwrap();
        o.log(3, REF, Outcome::Retry, "t").unwrap();
        for i in 0..(LOG_KEEP + DROP_KEEP) * 2 {
            o.log(10 + i, REF, Outcome::Dropped, "d").unwrap();
        }
        let all = o.recent(10_000).unwrap();
        assert_eq!(all.len() as i64, DROP_KEEP + 3);
        let kept: Vec<&str> = all.iter().rev().take(3).map(|e| e.outcome.as_str()).collect();
        assert_eq!(kept, ["sent", "rejected", "retry"]);
    }
}
