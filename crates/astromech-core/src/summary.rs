//! The session summary's reads (astromech-oo8y): the totals of one session from the `earnings`
//! and `visits` ledgers of `history.sqlite`, for the logout summary (on the pipeline thread)
//! and the voice question.
//!
//! Like the visit counts (`visits.rs`) this keeps one read-only connection, waits at most
//! [`BUSY`] for a lock, and treats any failure as [`Unreadable`]; the next ask tries again.

use astromech_announce::session_summary::{SessionHistory, Totals, Unreadable};
use astromech_history::History;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

/// The longest a read waits on the database.
const BUSY: Duration = Duration::from_millis(50);

pub struct HistorySessions {
    db: PathBuf,
    conn: Mutex<Option<History>>,
}

impl HistorySessions {
    pub fn new(db: PathBuf) -> Self {
        Self { db, conn: Mutex::new(None) }
    }
}

impl SessionHistory for HistorySessions {
    fn session(&self, commander: Option<&str>, at: Option<&str>) -> Result<Option<Totals>, Unreadable> {
        let mut conn = self.conn.try_lock().map_err(|_| Unreadable)?;
        if conn.is_none() {
            // No file yet: no journals imported, nothing to read.
            if !self.db.exists() {
                return Ok(None);
            }
            let h = History::open_read_only(&self.db).map_err(|_| Unreadable)?;
            h.conn().busy_timeout(BUSY).map_err(|_| Unreadable)?;
            *conn = Some(h);
        }
        let h = conn.as_ref().ok_or(Unreadable)?;
        let read = || -> astromech_history::Result<Option<Totals>> {
            // The history keys commanders by FID; the game state knows the name.
            let key = match commander {
                Some(name) => match h.commanders()?.into_iter().find(|c| c.name == name && !c.legacy) {
                    Some(c) => Some(c.cmdr),
                    None => return Ok(None),
                },
                None => None,
            };
            Ok(h.session(key.as_deref(), at)?.map(|s| Totals {
                end: s.end,
                total: s.total,
                per_hour: s.per_hour,
                jumps: s.jumps,
                distance_ly: s.distance_ly,
                bounties: s.bounties_awarded,
            }))
        };
        match read() {
            Ok(found) => Ok(found),
            Err(_) => {
                // Reopen next time (the file may have been replaced or migrated).
                *conn = None;
                Err(Unreadable)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_file_is_no_history_but_a_lost_lock_or_bad_file_is_unreadable() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("history.sqlite");
        let s = HistorySessions::new(db.clone());
        assert_eq!(s.session(None, None), Ok(None));
        // The connection is in use (the pipeline tick holds it): the voice query must not
        // mistake that for an empty history.
        {
            let _busy = s.conn.lock().unwrap();
            assert_eq!(s.session(None, None), Err(Unreadable));
        }
        std::fs::write(&db, b"not a database, not even close to one").unwrap();
        assert_eq!(s.session(None, None), Err(Unreadable));
    }
}
