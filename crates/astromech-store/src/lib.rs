//! # astromech-store
//!
//! Local persistence (ARCHITECTURE.md §5.19): one SQLite file with versioned migrations
//! (`PRAGMA user_version`). Secrets go to the OS keyring later, never into SQLite.

use rusqlite::{Connection, Result};

pub const MAX_ALIASES: usize = 200;

const MIGRATIONS: &[&str] = &[
    // v1
    "CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
     CREATE TABLE journal_files (path TEXT PRIMARY KEY, size INTEGER NOT NULL, offset INTEGER NOT NULL);",
    // v2 — personal phrase learning (V1-DESIGN.md §1.5 #4)
    "CREATE TABLE aliases (
        transcript TEXT PRIMARY KEY,
        command TEXT NOT NULL,
        count INTEGER NOT NULL DEFAULT 1
     );",
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
    /// Saved read position of a journal file: (size when saved, byte offset).
    pub fn journal_offset(&self, path: &str) -> Result<Option<(u64, u64)>> {
        let mut st = self.conn.prepare("SELECT size, offset FROM journal_files WHERE path = ?1")?;
        let mut rows = st.query([path])?;
        Ok(match rows.next()? {
            Some(r) => Some((r.get::<_, i64>(0)? as u64, r.get::<_, i64>(1)? as u64)),
            None => None,
        })
    }
    pub fn set_journal_offset(&self, path: &str, size: u64, offset: u64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO journal_files(path, size, offset) VALUES (?1, ?2, ?3)
             ON CONFLICT(path) DO UPDATE SET size = ?2, offset = ?3",
            (path, size as i64, offset as i64),
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

    pub fn aliases(&self) -> Result<Vec<(String, String)>> {
        let mut st = self.conn.prepare("SELECT transcript, command FROM aliases ORDER BY count DESC")?;
        let rows = st.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect()
    }

    /// Remember `transcript` → `command`. Capped at [`MAX_ALIASES`]; oldest (lowest count) drop first.
    pub fn learn_alias(&self, transcript: &str, command: &str) -> Result<bool> {
        let t = transcript.trim();
        if t.is_empty() {
            return Ok(false);
        }
        let n: i64 = self.conn.query_row("SELECT COUNT(*) FROM aliases", [], |r| r.get(0))?;
        if n >= MAX_ALIASES as i64 {
            let exists: i64 =
                self.conn.query_row("SELECT COUNT(*) FROM aliases WHERE transcript = ?1", [t], |r| r.get(0))?;
            if exists == 0 {
                self.conn.execute(
                    "DELETE FROM aliases WHERE transcript = (
                        SELECT transcript FROM aliases ORDER BY count ASC, transcript ASC LIMIT 1
                    )",
                    [],
                )?;
            }
        }
        self.conn.execute(
            "INSERT INTO aliases(transcript, command, count) VALUES (?1, ?2, 1)
             ON CONFLICT(transcript) DO UPDATE SET command = ?2, count = count + 1",
            (t, command),
        )?;
        Ok(true)
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
        assert_eq!(s.journal_offset("J.log").unwrap(), None);
        s.set_journal_offset("J.log", 10, 5).unwrap();
        s.set_journal_offset("J.log", 20, 20).unwrap();
        assert_eq!(s.journal_offset("J.log").unwrap(), Some((20, 20)));
    }

    #[test]
    fn learns_aliases_and_caps() {
        let s = Store::in_memory().unwrap();
        assert!(s.learn_alias("you're down", "landing_gear.down").unwrap());
        assert!(s.learn_alias("you're down", "landing_gear.down").unwrap());
        let a = s.aliases().unwrap();
        assert_eq!(a, vec![("you're down".into(), "landing_gear.down".into())]);
        for i in 0..MAX_ALIASES {
            s.learn_alias(&format!("t{i}"), "lights.on").unwrap();
        }
        assert_eq!(s.aliases().unwrap().len(), MAX_ALIASES);
    }
}
