//! Raw response bodies in SQLite, keyed by request. Only bodies that passed the schema check
//! are stored.

use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

const SCHEMA: &str =
    "CREATE TABLE IF NOT EXISTS remote_cache(key TEXT PRIMARY KEY, fetched INTEGER NOT NULL, body TEXT NOT NULL);";

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
        db.execute_batch("PRAGMA journal_mode=WAL;")?;
        db.execute_batch(SCHEMA)?;
        Ok(Cache { db })
    }

    pub fn in_memory() -> rusqlite::Result<Self> {
        let db = Connection::open_in_memory()?;
        db.execute_batch(SCHEMA)?;
        Ok(Cache { db })
    }

    /// `(fetched, body)` for `key` if it was fetched at most `ttl_s` seconds before `now_s`.
    pub fn get(&self, key: &str, now_s: i64, ttl_s: i64) -> rusqlite::Result<Option<(i64, String)>> {
        Ok(self
            .db
            .query_row("SELECT fetched, body FROM remote_cache WHERE key = ?1", params![key], |r| {
                Ok((r.get(0)?, r.get(1)?))
            })
            .optional()?
            .filter(|(t, _): &(i64, String)| now_s - *t < ttl_s))
    }

    pub fn put(&self, key: &str, now_s: i64, body: &str) -> rusqlite::Result<()> {
        self.db.execute("INSERT OR REPLACE INTO remote_cache VALUES (?1, ?2, ?3)", params![key, now_s, body])?;
        Ok(())
    }

    pub fn len(&self) -> rusqlite::Result<usize> {
        self.db.query_row("SELECT count(*) FROM remote_cache", [], |r| r.get::<_, i64>(0)).map(|n| n as usize)
    }

    pub fn is_empty(&self) -> rusqlite::Result<bool> {
        self.len().map(|n| n == 0)
    }
}
