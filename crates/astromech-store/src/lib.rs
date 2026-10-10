//! # astromech-store
//!
//! Local persistence (ARCHITECTURE.md §5.19): one SQLite file with versioned migrations
//! (`PRAGMA user_version`). Secrets go to the OS keyring later, never into SQLite.

use rusqlite::{Connection, Result};

const MIGRATIONS: &[&str] = &[
    // v1
    "CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
     CREATE TABLE journal_files (path TEXT PRIMARY KEY, size INTEGER NOT NULL, offset INTEGER NOT NULL);",
];

pub struct Store {
    conn: Connection,
}

impl Store {
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
    pub fn schema_version(&self) -> Result<i64> {
        self.conn.query_row("PRAGMA user_version", [], |r| r.get(0))
    }
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings(key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
            (key, value),
        )?;
        Ok(())
    }
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let mut st = self.conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = st.query([key])?;
        Ok(match rows.next()? {
            Some(r) => Some(r.get(0)?),
            None => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn migrates_and_stores() {
        let s = Store::in_memory().unwrap();
        assert_eq!(s.schema_version().unwrap(), MIGRATIONS.len() as i64);
        s.set("voice.threads", "4").unwrap();
        s.set("voice.threads", "6").unwrap();
        assert_eq!(s.get("voice.threads").unwrap().as_deref(), Some("6"));
        assert_eq!(s.get("missing").unwrap(), None);
    }
}
