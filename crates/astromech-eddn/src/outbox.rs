//! The persistent outbox and send log (SQLite, `eddn.sqlite` in the data folder; the EDSM
//! upload keeps its own, `edsm.sqlite`, in the same shape).
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
use std::cell::Cell;

/// The first retry waits this long; each later one doubles it, up to [`MAX_BACKOFF_SECS`].
pub const MIN_RETRY_SECS: i64 = 60;
pub const MAX_BACKOFF_SECS: i64 = 3600;
/// Tries per message (the first send plus retries) before it's dropped as undeliverable.
pub const MAX_ATTEMPTS: i64 = 8;
/// A message queued longer ago than this is given up: it's no longer live data (for example,
/// queued before astromech last quit).
pub const MAX_AGE_SECS: i64 = 30 * 60;
/// Send-log rows kept for messages that reached the gateway (sent, rejected, retry, gave up).
pub const LOG_KEEP: i64 = 500;
/// Send-log rows kept for messages never queued (`dropped`), counted apart so a burst of them
/// can't push the gateway's answers out of the log.
pub const DROP_KEEP: i64 = 200;
/// Messages kept queued at most; past this the oldest are given up (a long outage, or delivery
/// not running, can't grow the file without limit).
pub const MAX_QUEUED: i64 = 2000;
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
    // v2: ids are never reused, so a delivery pass that read rows before `discard_all` can't
    // claim, post or remove a newer message that got a purged row's id.
    "BEGIN;
     CREATE TABLE outbox_new (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        schema_ref TEXT NOT NULL,
        body BLOB NOT NULL,
        attempts INTEGER NOT NULL DEFAULT 0,
        next_try INTEGER NOT NULL,
        created INTEGER NOT NULL
     );
     INSERT INTO outbox_new SELECT id, schema_ref, body, attempts, next_try, created FROM outbox;
     DROP TABLE outbox;
     ALTER TABLE outbox_new RENAME TO outbox;
     COMMIT;",
    // v3: small values a sender keeps between runs (EDSM's discard list).
    "CREATE TABLE meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
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
    /// Failed [`MAX_ATTEMPTS`] times, or queued too long; dropped.
    GaveUp,
    /// Queued but dropped here without a send failure: sharing was switched off, or the outbox
    /// was full. Listed with the gateway's answers, but not a send error.
    Discarded,
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
            Outcome::Discarded => "discarded",
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

/// One queued message, for a sender that doesn't post one message at a time (EDSM batches).
#[derive(Debug, Clone, PartialEq)]
pub struct Queued {
    pub id: i64,
    pub schema_ref: String,
    pub body: Vec<u8>,
    /// Tries so far.
    pub attempts: i64,
}

pub struct Outbox {
    conn: Connection,
    /// The service, for send-log rows about the whole queue ("switched off: ...").
    label: &'static str,
    /// While the queue stays full: the send-log row for this overflow and how many it counts,
    /// so a long outage writes one row, not one per message queued.
    overflow: Cell<Option<(i64, i64)>>,
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
        Ok(Self { conn, label: "eddn", overflow: Cell::new(None) })
    }

    /// Name the service in send-log rows about the whole queue (default `eddn`).
    pub fn with_label(self, label: &'static str) -> Self {
        Self { label, ..self }
    }

    /// Queue a gzipped upload; it's due immediately. A full queue drops its oldest, counted in
    /// one send-log row until an upload fits again.
    pub fn enqueue(&self, schema_ref: &str, gzipped: &[u8], now: i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO outbox(schema_ref, body, next_try, created) VALUES (?1, ?2, ?3, ?3)",
            (schema_ref, gzipped, now),
        )?;
        let over = self.conn.execute(
            "DELETE FROM outbox WHERE id <= (SELECT id FROM outbox ORDER BY id DESC LIMIT 1 OFFSET ?1)",
            [MAX_QUEUED],
        )? as i64;
        match (over, self.overflow.get()) {
            (0, _) => self.overflow.set(None),
            (_, Some((row, total))) => {
                let total = total + over;
                self.conn.execute(
                    "UPDATE send_log SET detail = ?2 WHERE id = ?1",
                    (row, format!("outbox full: {total} oldest dropped")),
                )?;
                self.overflow.set(Some((row, total)));
            }
            (_, None) => {
                self.log(now, schema_ref, Outcome::Discarded, &format!("outbox full: {over} oldest dropped"))?;
                self.overflow.set(Some((self.conn.last_insert_rowid(), over)));
            }
        }
        Ok(())
    }

    /// Drop everything queued, because sharing was switched off, and say so in the send log.
    /// Returns how many.
    pub fn discard_all(&self, now: i64) -> Result<usize> {
        let n = self.conn.execute("DELETE FROM outbox", [])?;
        if n > 0 {
            self.log(now, self.label, Outcome::Discarded, &format!("switched off: {n} queued discarded"))?;
        }
        Ok(n)
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
        self.query("1", limit)
    }

    /// The newest entries with one of `outcomes`, newest first.
    pub fn recent_of(&self, outcomes: &[Outcome], limit: usize) -> Result<Vec<LogEntry>> {
        let list: Vec<String> = outcomes.iter().map(|o| format!("'{}'", o.as_str())).collect();
        self.query(&format!("outcome IN ({})", list.join(",")), limit)
    }

    /// `filter` is built from [`Outcome`] names only, never from input.
    fn query(&self, filter: &str, limit: usize) -> Result<Vec<LogEntry>> {
        let mut st = self.conn.prepare(&format!(
            "SELECT at, schema_ref, outcome, detail FROM send_log WHERE {filter} ORDER BY id DESC LIMIT ?1"
        ))?;
        let rows = st.query_map([limit as i64], |r| {
            Ok(LogEntry { at: r.get(0)?, schema_ref: r.get(1)?, outcome: r.get(2)?, detail: r.get(3)? })
        })?;
        rows.collect()
    }

    /// Try the messages due at `now`, oldest first. With `retries` false only messages never
    /// tried yet are sent (the caller's own clock says a minute hasn't passed since a failure).
    ///
    /// A connection error, `408` or `5xx` means the gateway is down or overloaded, so the pass
    /// stops there and every other due message waits as long as the one that failed. `stop` is
    /// checked before each post. Messages older than [`MAX_AGE_SECS`] are given up first.
    pub fn deliver_due(
        &self,
        transport: &dyn Transport,
        now: i64,
        retries: bool,
        stop: &dyn Fn() -> bool,
    ) -> Result<Delivery> {
        let mut d = Delivery::default();
        let old: Vec<(i64, String, i64)> = {
            let mut st =
                self.conn.prepare("SELECT id, schema_ref, created FROM outbox WHERE created < ?1 ORDER BY id")?;
            let rows = st.query_map([now - MAX_AGE_SECS], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
            rows.collect::<Result<_>>()?
        };
        for (id, schema_ref, created) in old {
            self.remove(id)?;
            let mins = (now - created) / 60;
            self.log(now, &schema_ref, Outcome::GaveUp, &format!("too old: queued {mins} min ago"))?;
            d.gave_up += 1;
        }
        let due: Vec<(i64, String, Vec<u8>, i64)> = {
            let mut st = self.conn.prepare(
                "SELECT id, schema_ref, body, attempts FROM outbox
                 WHERE next_try <= ?1 AND (?2 OR attempts = 0) ORDER BY id",
            )?;
            let rows = st.query_map((now, retries), |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
            rows.collect::<Result<_>>()?
        };
        for (id, schema_ref, body, attempts) in due {
            if stop() {
                break;
            }
            let attempts = attempts + 1;
            let wait = backoff(attempts);
            // Count the attempt and schedule the retry before posting, so a failure anywhere
            // after this point (a panic, a database error) can't leave the message due again.
            // No row: it was discarded during this pass (sharing switched off, then back on).
            let claimed = self
                .conn
                .execute("UPDATE outbox SET attempts = ?2, next_try = ?3 WHERE id = ?1", (id, attempts, now + wait))?;
            if claimed == 0 {
                continue;
            }
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

    /// Up to `limit` messages due at `now`, oldest first; with `retries` false only those never
    /// tried. For a sender with its own delivery rules (EDSM); [`Outbox::deliver_due`] is EDDN's.
    pub fn due(&self, now: i64, retries: bool, limit: usize) -> Result<Vec<Queued>> {
        let mut st = self.conn.prepare(
            "SELECT id, schema_ref, body, attempts FROM outbox
             WHERE next_try <= ?1 AND (?2 OR attempts = 0) ORDER BY id LIMIT ?3",
        )?;
        let rows = st.query_map((now, retries, limit as i64), |r| {
            Ok(Queued { id: r.get(0)?, schema_ref: r.get(1)?, body: r.get(2)?, attempts: r.get(3)? })
        })?;
        rows.collect()
    }

    /// Take a message off the queue (sent, or never to be sent). False if it was already gone
    /// (discarded while it was being posted).
    pub fn take(&self, id: i64) -> Result<bool> {
        Ok(self.conn.execute("DELETE FROM outbox WHERE id = ?1", [id])? > 0)
    }

    /// Count a failed try and set when the message is due again. False if it's gone.
    pub fn retry_at(&self, id: i64, attempts: i64, next_try: i64) -> Result<bool> {
        Ok(self
            .conn
            .execute("UPDATE outbox SET attempts = ?2, next_try = ?3 WHERE id = ?1", (id, attempts, next_try))?
            > 0)
    }

    /// Every message due at `now` waits until `until` (the service looks down).
    pub fn hold_due(&self, now: i64, until: i64) -> Result<()> {
        self.conn.execute("UPDATE outbox SET next_try = ?2 WHERE next_try <= ?1", (now, until))?;
        Ok(())
    }

    /// Every queued message is due at `now` with its tries forgotten: whatever held them back
    /// (a wrong key, an outage) is over.
    pub fn make_due(&self, now: i64) -> Result<()> {
        self.conn
            .execute("UPDATE outbox SET next_try = ?1, attempts = 0 WHERE next_try > ?1 OR attempts > 0", [now])?;
        Ok(())
    }

    /// A value the sender kept from an earlier run.
    pub fn meta(&self, key: &str) -> Result<Option<String>> {
        self.conn.query_row("SELECT value FROM meta WHERE key = ?1", [key], |r| r.get(0)).optional()
    }

    pub fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute("INSERT OR REPLACE INTO meta(key, value) VALUES (?1, ?2)", (key, value))?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::transport::Response;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    /// A pass that isn't being stopped.
    pub fn no_stop() -> bool {
        false
    }

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
        assert_eq!(o.deliver_due(&t, 1000, true, &no_stop).unwrap(), Delivery { sent: 1, ..Default::default() });
        assert_eq!(o.pending().unwrap(), 0);
        assert_eq!(o.recent(10).unwrap()[0].outcome, "sent");
    }

    #[test]
    fn http_400_is_not_retried() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::new([status(400)]);
        assert_eq!(o.deliver_due(&t, 1000, true, &no_stop).unwrap().rejected, 1);
        assert_eq!(o.pending().unwrap(), 0, "a 400 must leave the outbox");
        assert_eq!(o.deliver_due(&t, 1000 + 10 * MAX_BACKOFF_SECS, true, &no_stop).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 1, "never posted again");
        let log = o.recent(1).unwrap();
        assert_eq!((log[0].outcome.as_str(), log[0].detail.as_str()), ("rejected", "HTTP 400: FAIL: 400"));
    }

    #[test]
    fn http_426_is_not_retried() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::new([status(426)]);
        assert_eq!(o.deliver_due(&t, 1000, true, &no_stop).unwrap().rejected, 1);
        assert_eq!(o.deliver_due(&t, 1000 + 10 * MAX_BACKOFF_SECS, true, &no_stop).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 1);
    }

    #[test]
    fn other_failures_retry_no_sooner_than_one_minute() {
        for failure in [status(503), status(413), status(408), Err("connection refused".to_string())] {
            let o = Outbox::in_memory().unwrap();
            o.enqueue(REF, b"m", 1000).unwrap();
            let t = FakeTransport::new([failure.clone(), status(200)]);
            assert_eq!(o.deliver_due(&t, 1000, true, &no_stop).unwrap().retrying, 1, "{failure:?}");
            assert_eq!(o.pending().unwrap(), 1);
            assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS));
            // Every second up to 59 s later: nothing is posted.
            for s in 1..MIN_RETRY_SECS {
                assert_eq!(o.deliver_due(&t, 1000 + s, true, &no_stop).unwrap(), Delivery::default());
            }
            assert_eq!(t.calls(), 1, "{failure:?} retried too soon");
            assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS, true, &no_stop).unwrap().sent, 1);
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
            // Keep it young: this test is about the number of tries, not the age limit.
            o.conn.execute("UPDATE outbox SET created = ?1", [now]).unwrap();
            o.deliver_due(&t, now, true, &no_stop).unwrap();
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
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            o.deliver_due(&PanickingTransport, 1000, true, &no_stop)
        }));
        assert!(r.is_err());
        assert_eq!(o.pending().unwrap(), 1, "kept for a retry");
        assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS), "not due again straight away");
        let t = FakeTransport::default();
        assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS - 1, true, &no_stop).unwrap(), Delivery::default());
        assert_eq!(t.calls(), 0);
    }

    #[test]
    fn one_bad_message_does_not_block_the_others() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"bad", 0).unwrap();
        o.enqueue(REF, b"good", 0).unwrap();
        let t = FakeTransport::new([status(413), status(200)]);
        let d = o.deliver_due(&t, 0, true, &no_stop).unwrap();
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
            let d = o.deliver_due(&t, 1000, true, &no_stop).unwrap();
            assert_eq!(d, Delivery { retrying: 1, stopped: true, ..Default::default() }, "{failure:?}");
            assert_eq!(t.calls(), 1, "{failure:?}: one request, not one per queued message");
            // The untried messages wait for the retry time too, and keep their tries.
            assert_eq!(o.next_due().unwrap(), Some(1000 + MIN_RETRY_SECS));
            assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS - 1, true, &no_stop).unwrap(), Delivery::default());
            let tries: Vec<i64> = {
                let mut st = o.conn.prepare("SELECT attempts FROM outbox ORDER BY id").unwrap();
                st.query_map([], |r| r.get(0)).unwrap().collect::<Result<_>>().unwrap()
            };
            assert_eq!(tries, [1, 0, 0]);
            let t = FakeTransport::new([status(200), status(200), status(200)]);
            assert_eq!(o.deliver_due(&t, 1000 + MIN_RETRY_SECS, true, &no_stop).unwrap().sent, 3);
        }
    }

    #[test]
    fn without_retries_only_untried_messages_are_sent() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"old", 0).unwrap();
        let t = FakeTransport::new([status(413)]);
        assert_eq!(o.deliver_due(&t, 0, true, &no_stop).unwrap().retrying, 1);
        o.enqueue(REF, b"new", 0).unwrap();
        let t = FakeTransport::new([status(200)]);
        // The wall clock says the retry is due; the caller says it isn't allowed yet.
        assert_eq!(o.deliver_due(&t, 10 * MIN_RETRY_SECS, false, &no_stop).unwrap().sent, 1);
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

    #[test]
    fn messages_older_than_the_age_limit_are_given_up() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"old", 1000).unwrap();
        o.enqueue(REF, b"young", 1000 + MAX_AGE_SECS).unwrap();
        let t = FakeTransport::new([status(200)]);
        let d = o.deliver_due(&t, 1000 + MAX_AGE_SECS + 1, true, &no_stop).unwrap();
        assert_eq!((d.gave_up, d.sent), (1, 1));
        assert_eq!(*t.posted.borrow(), [b"young".to_vec()], "the old one is never posted");
        let log = o.recent(10).unwrap();
        let gave_up = log.iter().find(|e| e.outcome == "gave_up").unwrap();
        assert_eq!(gave_up.detail, "too old: queued 30 min ago");
    }

    #[test]
    fn a_stopped_pass_posts_nothing_more() {
        let o = Outbox::in_memory().unwrap();
        o.enqueue(REF, b"m", 1000).unwrap();
        let t = FakeTransport::default();
        let d = o.deliver_due(&t, 1000, true, &|| true).unwrap();
        assert_eq!((d, t.calls()), (Delivery::default(), 0));
        assert_eq!(o.next_due().unwrap(), Some(1000), "untouched");
    }

    #[test]
    fn the_queue_is_capped_by_row_count() {
        let o = Outbox::in_memory().unwrap();
        for i in 0..MAX_QUEUED + 3 {
            o.enqueue(REF, &i.to_le_bytes(), 1000).unwrap();
        }
        assert_eq!(o.pending().unwrap() as i64, MAX_QUEUED);
        let rows = o.recent(10).unwrap();
        assert_eq!(rows.len(), 1, "one row for the overflow, not one per message queued");
        assert_eq!((rows[0].outcome.as_str(), rows[0].detail.as_str()), ("discarded", "outbox full: 3 oldest dropped"));
        assert_eq!(rows[0].at, 1000);

        // Delivery makes room: the next message fits and ends the overflow. Filling up again
        // starts a new row.
        o.conn.execute("DELETE FROM outbox WHERE id = (SELECT MIN(id) FROM outbox)", []).unwrap();
        o.enqueue(REF, b"fits", 2000).unwrap();
        assert_eq!(o.recent(10).unwrap().len(), 1);
        o.enqueue(REF, b"over", 2001).unwrap();
        let rows = o.recent(10).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!((rows[0].at, rows[0].detail.as_str()), (2001, "outbox full: 1 oldest dropped"));
        assert_eq!(rows[1].detail, "outbox full: 3 oldest dropped");
    }

    /// Discards the queue (through its own connection) during its first post, as the core does
    /// when sharing is switched off and on again while a pass is posting.
    struct DiscardDuringPost(std::path::PathBuf, RefCell<usize>);
    impl Transport for DiscardDuringPost {
        fn post(&self, _: &[u8]) -> std::result::Result<Response, String> {
            *self.1.borrow_mut() += 1;
            Outbox::open(&self.0).unwrap().discard_all(1000).unwrap();
            Ok(Response { status: 200, body: "OK".into() })
        }
    }

    #[test]
    fn messages_discarded_during_a_pass_are_not_posted() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("eddn.sqlite");
        let o = Outbox::open(&p).unwrap();
        for b in [b"a", b"b", b"c"] {
            o.enqueue(REF, b, 1000).unwrap();
        }
        let t = DiscardDuringPost(p, RefCell::new(0));
        let d = o.deliver_due(&t, 1000, true, &no_stop).unwrap();
        assert_eq!(*t.1.borrow(), 1, "the discarded messages read before the post are not sent");
        assert_eq!(d, Delivery { sent: 1, ..Default::default() });
        let outcomes: Vec<String> = o.recent(10).unwrap().into_iter().map(|e| e.outcome).collect();
        assert_eq!(outcomes, ["sent", "discarded"]);
    }

    /// Purges the queue and queues fresh messages during its first post, as the core does when
    /// sharing is switched off and on again and new events arrive mid-pass.
    struct PurgeAndRequeue(std::path::PathBuf);
    impl Transport for PurgeAndRequeue {
        fn post(&self, b: &[u8]) -> std::result::Result<Response, String> {
            if b == b"a" {
                let o = Outbox::open(&self.0).unwrap();
                o.discard_all(1000).unwrap();
                for n in [b"x", b"y", b"z"] {
                    o.enqueue(REF, n, 1000).unwrap();
                }
            }
            Ok(Response { status: 200, body: "OK".into() })
        }
    }

    #[test]
    fn new_messages_never_take_a_purged_messages_id() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("eddn.sqlite");
        let o = Outbox::open(&p).unwrap();
        for b in [b"a", b"b", b"c"] {
            o.enqueue(REF, b, 1000).unwrap();
        }
        let posted = RefCell::new(Vec::new());
        struct Rec<'a>(PurgeAndRequeue, &'a RefCell<Vec<Vec<u8>>>);
        impl Transport for Rec<'_> {
            fn post(&self, b: &[u8]) -> std::result::Result<Response, String> {
                self.1.borrow_mut().push(b.to_vec());
                self.0.post(b)
            }
        }
        let d = o.deliver_due(&Rec(PurgeAndRequeue(p), &posted), 1000, true, &no_stop).unwrap();
        assert_eq!(*posted.borrow(), [b"a".to_vec()], "purged b and c must not be posted");
        assert_eq!(d.sent, 1);
        assert_eq!(o.pending().unwrap(), 3, "the new messages are untouched by the old pass");
    }

    #[test]
    fn migrating_a_v1_outbox_keeps_its_rows() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("eddn.sqlite");
        {
            let c = Connection::open(&p).unwrap();
            c.execute_batch(MIGRATIONS[0]).unwrap();
            c.execute_batch("INSERT INTO outbox(id, schema_ref, body, next_try, created) VALUES (7, 'r', x'01', 5, 5)")
                .unwrap();
            c.pragma_update(None, "user_version", 1).unwrap();
        }
        let o = Outbox::open(&p).unwrap();
        assert_eq!(o.pending().unwrap(), 1);
        o.discard_all(10).unwrap();
        o.enqueue(REF, b"n", 10).unwrap();
        let id: i64 = o.conn.query_row("SELECT id FROM outbox", [], |r| r.get(0)).unwrap();
        assert!(id > 7, "ids continue past the purged one, got {id}");
    }

    #[test]
    fn discarding_the_queue_empties_it_and_logs_once() {
        let o = Outbox::in_memory().unwrap();
        assert_eq!(o.discard_all(1000).unwrap(), 0);
        assert!(o.recent(10).unwrap().is_empty(), "nothing queued, nothing logged");
        o.enqueue(REF, b"a", 1000).unwrap();
        o.enqueue(REF, b"b", 1000).unwrap();
        assert_eq!(o.discard_all(1001).unwrap(), 2);
        assert_eq!(o.pending().unwrap(), 0);
        let rows = o.recent(10).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(
            (rows[0].outcome.as_str(), rows[0].detail.as_str()),
            ("discarded", "switched off: 2 queued discarded")
        );
    }

    #[test]
    fn recent_of_filters_by_outcome() {
        let o = Outbox::in_memory().unwrap();
        o.log(1, REF, Outcome::Sent, "").unwrap();
        o.log(2, REF, Outcome::Dropped, "d").unwrap();
        o.log(3, REF, Outcome::Retry, "r").unwrap();
        let answers = o.recent_of(&[Outcome::Sent, Outcome::Retry], 10).unwrap();
        assert_eq!(answers.iter().map(|e| e.at).collect::<Vec<_>>(), [3, 1]);
        assert_eq!(o.recent_of(&[Outcome::Dropped], 10).unwrap()[0].detail, "d");
    }
}
