//! Raw response bodies in SQLite, keyed by request. Only bodies that passed the schema check
//! are stored, each with the time it expires; expired rows are deleted on every write, so the
//! table holds only live answers.

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::path::Path;

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS remote_cache(
        key TEXT PRIMARY KEY, fetched INTEGER NOT NULL, expires INTEGER NOT NULL, body TEXT NOT NULL);
    CREATE INDEX IF NOT EXISTS remote_cache_expires ON remote_cache(expires);";

/// How long a write waits for the other process's write to the shared file.
pub const BUSY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

pub struct Cache {
    db: Connection,
}

impl Cache {
    /// Open (or create) the cache at `path`. The table can share a file with other stores.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        if let Some(d) = path.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let db = Connection::open(path)?;
        // The core and the app window share this file: wait for the other's write, not fail.
        db.busy_timeout(BUSY_TIMEOUT)?;
        db.execute_batch("PRAGMA journal_mode=WAL;")?;
        Self::init(db)
    }

    pub fn in_memory() -> rusqlite::Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(mut db: Connection) -> rusqlite::Result<Self> {
        // The core and the app window can open an old file at the same moment, so the
        // replacement runs under the write lock and looks at the layout again once it has it.
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        // A table from before expiry times were stored (v0.1.5) is only a cache: start afresh.
        let cols = tx
            .prepare("SELECT name FROM pragma_table_info('remote_cache')")?
            .query_map([], |r| r.get::<_, String>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        if !cols.is_empty() && !cols.iter().any(|c| c == "expires") {
            tx.execute_batch("DROP TABLE remote_cache;")?;
        }
        tx.execute_batch(SCHEMA)?;
        tx.commit()?;
        Ok(Cache { db })
    }

    /// `(fetched, body)` for `key` if it hasn't expired by `now_s`.
    pub fn get(&self, key: &str, now_s: i64) -> rusqlite::Result<Option<(i64, String)>> {
        self.db
            .query_row(
                "SELECT fetched, body FROM remote_cache WHERE key = ?1 AND expires > ?2",
                params![key, now_s],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
    }

    /// Store `body` for `key`, fetched at `now_s` and good for `ttl_s` seconds, and delete the
    /// rows that have expired.
    pub fn put(&self, key: &str, now_s: i64, ttl_s: i64, body: &str) -> rusqlite::Result<()> {
        self.db.execute("DELETE FROM remote_cache WHERE expires <= ?1", params![now_s])?;
        self.db.execute(
            "INSERT OR REPLACE INTO remote_cache(key, fetched, expires, body) VALUES (?1, ?2, ?3, ?4)",
            params![key, now_s, now_s.saturating_add(ttl_s), body],
        )?;
        Ok(())
    }

    pub fn len(&self) -> rusqlite::Result<usize> {
        self.db.query_row("SELECT count(*) FROM remote_cache", [], |r| r.get::<_, i64>(0)).map(|n| n as usize)
    }

    pub fn is_empty(&self) -> rusqlite::Result<bool> {
        self.len().map(|n| n == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_v015_table_without_expiry_times_is_replaced() {
        let dir = std::env::temp_dir().join(format!("astromech-data-oldcache-{}", std::process::id()));
        let path = dir.join("remote.db");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Connection::open(&path)
            .unwrap()
            .execute_batch(
                "CREATE TABLE remote_cache(key TEXT PRIMARY KEY, fetched INTEGER NOT NULL, body TEXT NOT NULL);
                 INSERT INTO remote_cache VALUES ('k', 1, 'old');",
            )
            .unwrap();
        let c = Cache::open(&path).unwrap();
        assert!(c.is_empty().unwrap());
        c.put("k", 10, 5, "new").unwrap();
        assert_eq!(c.get("k", 14).unwrap(), Some((10, "new".into())));
        assert_eq!(c.get("k", 15).unwrap(), None, "expired at fetched + ttl");
        // Opening it again keeps what is there.
        assert_eq!(Cache::open(&path).unwrap().len().unwrap(), 1);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn two_openers_of_a_v015_file_both_get_the_file_backed_cache() {
        let dir = std::env::temp_dir().join(format!("astromech-data-race-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for round in 0..50 {
            let path = dir.join(format!("remote{round}.db"));
            Connection::open(&path)
                .unwrap()
                .execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE remote_cache(key TEXT PRIMARY KEY, fetched INTEGER NOT NULL, body TEXT NOT NULL);")
                .unwrap();
            let gate = std::sync::Arc::new(std::sync::Barrier::new(2));
            let openers: Vec<_> = (0..2)
                .map(|_| {
                    let (path, gate) = (path.clone(), gate.clone());
                    std::thread::spawn(move || {
                        gate.wait();
                        Cache::open(&path).map(|_| ())
                    })
                })
                .collect();
            for o in openers {
                o.join().unwrap().expect("both openers keep the file-backed cache");
            }
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_shared_file_waits_for_the_other_process() {
        let dir = std::env::temp_dir().join(format!("astromech-data-busy-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let c = Cache::open(&dir.join("remote.db")).unwrap();
        let ms: i64 = c.db.query_row("PRAGMA busy_timeout", [], |r| r.get(0)).unwrap();
        assert_eq!(ms, BUSY_TIMEOUT.as_millis() as i64);
        let _ = std::fs::remove_dir_all(dir);
    }
}
