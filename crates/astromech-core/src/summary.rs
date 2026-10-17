//! The session summary's reads (astromech-oo8y): the totals of one session from the `earnings`
//! and `visits` ledgers of `history.sqlite`, for the logout summary (on the pipeline thread)
//! and the voice question.
//!
//! Like the visit counts (`visits.rs`) this keeps one read-only connection, waits at most
//! [`BUSY`] for a lock, and treats any failure as no answer; the next ask tries again.

use astromech_announce::session_summary::{SessionHistory, Totals};
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
    fn session(&self, commander: Option<&str>, at: Option<&str>) -> Option<Totals> {
        let mut conn = self.conn.try_lock().ok()?;
        if conn.is_none() {
            let h = History::open_read_only(&self.db).ok()?;
            h.conn().busy_timeout(BUSY).ok()?;
            *conn = Some(h);
        }
        let h = conn.as_ref()?;
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
            Ok(found) => found,
            Err(_) => {
                // Reopen next time (the file may have been replaced or migrated).
                *conn = None;
                None
            }
        }
    }
}
