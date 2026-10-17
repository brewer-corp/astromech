//! The arrival briefing's "been here before" (astromech-2myu): visit counts from the `visits`
//! ledger of `history.sqlite`.
//!
//! The reaction engine runs on the pipeline thread and must never block, and the history
//! follower is the database's only writer. So this keeps one read-only connection (WAL lets it
//! read while the follower writes), asks one indexed question per jump, and waits at most
//! [`BUSY`] for a lock. Any failure (no database yet, a busy one, a schema it doesn't know)
//! means no line, and the next jump tries again. Counting only arrivals strictly before the
//! one being announced keeps that arrival out of its own count, whether or not the follower
//! has imported it yet.

use astromech_announce::arrival::{Past, VisitHistory};
use astromech_history::History;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

/// The longest a lookup waits on the database.
const BUSY: Duration = Duration::from_millis(50);

pub struct HistoryVisits {
    db: PathBuf,
    conn: Mutex<Option<History>>,
}

impl HistoryVisits {
    pub fn new(db: PathBuf) -> Self {
        Self { db, conn: Mutex::new(None) }
    }
}

impl VisitHistory for HistoryVisits {
    fn before(&self, commander: &str, system_address: i64, before: &str) -> Option<Past> {
        let mut conn = self.conn.try_lock().ok()?;
        if conn.is_none() {
            let h = History::open_read_only(&self.db).ok()?;
            h.conn().busy_timeout(BUSY).ok()?;
            *conn = Some(h);
        }
        match conn.as_ref()?.visits_before(commander, system_address, before) {
            Ok(found) => found.map(|(visits, last)| Past { visits: visits.try_into().unwrap_or(u32::MAX), last }),
            Err(_) => {
                // Reopen next time (the file may have been replaced or migrated).
                *conn = None;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_history::ImportOptions;

    fn jump(ts: &str, addr: i64) -> String {
        format!(
            r#"{{"timestamp":"{ts}","event":"FSDJump","StarSystem":"S{addr}","SystemAddress":{addr},"StarPos":[0,0,0],"JumpDist":9.5}}"#
        )
    }

    /// A journal with three arrivals in system 1 (the last is the one being announced), one in
    /// system 2, and a second commander who was in system 3.
    fn imported() -> (tempfile::TempDir, PathBuf) {
        let d = tempfile::tempdir().unwrap();
        let load = |fid: &str, name: &str| {
            format!(
                r#"{{"timestamp":"2026-10-01T09:00:00Z","event":"LoadGame","FID":"{fid}","Commander":"{name}","Ship":"krait_mkii"}}"#
            )
        };
        let mine = [
            load("F1", "Test"),
            jump("2026-10-01T10:00:00Z", 1),
            jump("2026-10-02T10:00:00Z", 2),
            jump("2026-10-13T20:00:00Z", 1),
            jump("2026-10-16T20:00:00Z", 1),
        ];
        std::fs::write(d.path().join("Journal.2026-10-01T090000.01.log"), mine.join("\n") + "\n").unwrap();
        let theirs = [load("F2", "Other"), jump("2026-10-05T10:00:00Z", 3), jump("2026-10-05T11:00:00Z", 1)];
        std::fs::write(d.path().join("Journal.2026-10-05T090000.01.log"), theirs.join("\n") + "\n").unwrap();
        let db = d.path().join("history.sqlite");
        let mut h = History::open(&db).unwrap();
        h.import(d.path(), &ImportOptions::default()).unwrap();
        (d, db)
    }

    #[test]
    fn counts_earlier_arrivals_of_the_commander_and_not_the_one_announced() {
        let (_dir, db) = imported();
        let v = HistoryVisits::new(db);
        let found = v.before("Test", 1, "2026-10-16T20:00:00Z");
        assert_eq!(found, Some(Past { visits: 2, last: "2026-10-13T20:00:00Z".into() }));
        // A system seen once, a commander who was never there, and a system never visited.
        assert_eq!(v.before("Test", 2, "2026-10-16T20:00:00Z").map(|p| p.visits), Some(1));
        assert_eq!(v.before("Test", 3, "2026-10-16T20:00:00Z"), None);
        assert_eq!(v.before("Nobody", 1, "2026-10-16T20:00:00Z"), None);
        // The first arrival ever has nothing before it.
        assert_eq!(v.before("Test", 1, "2026-10-01T10:00:00Z"), None);
    }

    #[test]
    fn no_database_yet_is_no_answer_and_it_is_found_once_there() {
        let d = tempfile::tempdir().unwrap();
        let v = HistoryVisits::new(d.path().join("history.sqlite"));
        assert_eq!(v.before("Test", 1, "2026-10-16T20:00:00Z"), None);
        let (_dir, db) = imported();
        std::fs::copy(&db, d.path().join("history.sqlite")).unwrap();
        assert!(v.before("Test", 1, "2026-10-16T20:00:00Z").is_some());
    }

    #[test]
    fn a_lookup_does_not_wait_for_the_importing_writer() {
        let (_dir, db) = imported();
        // The follower mid-import: a write transaction open on the same file.
        let writer = History::open(&db).unwrap();
        writer.conn().execute_batch("BEGIN IMMEDIATE").unwrap();
        let v = HistoryVisits::new(db);
        let t = std::time::Instant::now();
        assert!(v.before("Test", 1, "2026-10-16T20:00:00Z").is_some());
        assert!(t.elapsed() < Duration::from_millis(500));
        writer.conn().execute_batch("ROLLBACK").unwrap();
    }
}
