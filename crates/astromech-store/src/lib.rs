//! # astromech-store
//!
//! Local persistence (ARCHITECTURE.md §5.19): one SQLite file with versioned migrations
//! (`PRAGMA user_version`). Secrets go to the OS keyring ([`secrets`]), never into SQLite.

pub mod paths;
pub mod secrets;
pub mod window;

pub use paths::Paths;

pub use rusqlite;
pub use rusqlite::Error as StoreError;
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
    // v3 — domain records kept as one JSON body per (kind, key), e.g. colonisation projects
    "CREATE TABLE records (
        kind TEXT NOT NULL,
        key TEXT NOT NULL,
        body TEXT NOT NULL,
        PRIMARY KEY (kind, key)
     );",
];

/// Switching a file to WAL while another connection is opening it fails at once with "database
/// is locked" (SQLite doesn't call the busy handler for it), so retry for a few seconds.
fn enable_wal(conn: &Connection) -> Result<()> {
    let start = std::time::Instant::now();
    loop {
        match conn.pragma_update(None, "journal_mode", "WAL") {
            Err(rusqlite::Error::SqliteFailure(e, _))
                if e.code == rusqlite::ErrorCode::DatabaseBusy
                    && start.elapsed() < std::time::Duration::from_secs(5) =>
            {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            r => return r,
        }
    }
}

/// Put `conn` in WAL mode and apply the `migrations` it hasn't seen yet (`PRAGMA user_version`
/// counts the applied ones). Other SQLite files (the journal history) use the same pattern.
///
/// Each migration and its `user_version` bump run in one `BEGIN IMMEDIATE` transaction (SQLite
/// DDL is transactional), so a crash or a failing statement leaves the file at the previous
/// version, never half-migrated; the version is re-read inside the transaction, so two
/// processes opening a new file at once don't both apply it. Migrations must not use PRAGMAs.
pub fn migrate(mut conn: Connection, migrations: &[&str]) -> Result<Connection> {
    enable_wal(&conn)?;
    let v: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    for (i, m) in migrations.iter().enumerate().skip(v as usize) {
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let v: i64 = tx.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if v > i as i64 {
            continue; // another connection applied it meanwhile
        }
        tx.execute_batch(m)?;
        tx.pragma_update(None, "user_version", (i + 1) as i64)?;
        tx.commit()?;
    }
    Ok(conn)
}

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
        Ok(Self { conn: migrate(conn, MIGRATIONS)? })
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

    /// Insert or replace the JSON `body` stored under (`kind`, `key`).
    pub fn put_record(&self, kind: &str, key: &str, body: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO records(kind, key, body) VALUES (?1, ?2, ?3)
             ON CONFLICT(kind, key) DO UPDATE SET body = ?3",
            (kind, key, body),
        )?;
        Ok(())
    }

    /// Every (key, body) stored under `kind`, ordered by key.
    pub fn records(&self, kind: &str) -> Result<Vec<(String, String)>> {
        let mut st = self.conn.prepare("SELECT key, body FROM records WHERE kind = ?1 ORDER BY key")?;
        let rows = st.query_map([kind], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect()
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

    /// A migration that fails part-way (a crash mid-migration) leaves the file at the previous
    /// version with none of that migration applied, and the next open succeeds.
    #[test]
    fn failed_migration_rolls_back_whole() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("t.sqlite");
        let v1 = "CREATE TABLE a (x INTEGER);";
        let broken = "ALTER TABLE a ADD COLUMN y INTEGER; CREATE TABLE b (z INTEGER); SELECT * FROM missing;";
        assert!(migrate(Connection::open(&db).unwrap(), &[v1, broken]).is_err());
        let c = Connection::open(&db).unwrap();
        assert_eq!(c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0)).unwrap(), 1);
        assert!(c.prepare("SELECT y FROM a").is_err(), "the ALTER was rolled back");
        assert!(c.prepare("SELECT z FROM b").is_err(), "the CREATE was rolled back");
        drop(c);
        let fixed = "ALTER TABLE a ADD COLUMN y INTEGER; CREATE TABLE b (z INTEGER);";
        let c = migrate(Connection::open(&db).unwrap(), &[v1, fixed]).unwrap();
        assert_eq!(c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0)).unwrap(), 2);
        c.prepare("SELECT y FROM a").unwrap();
    }

    /// Two processes opening a brand-new file at once: both succeed, the schema is applied once.
    #[test]
    fn concurrent_first_open_succeeds() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("new.sqlite");
        let gate = std::sync::Arc::new(std::sync::Barrier::new(8));
        let hs: Vec<_> = (0..8)
            .map(|_| {
                let (db, gate) = (db.clone(), gate.clone());
                std::thread::spawn(move || {
                    let c = Connection::open(&db).unwrap();
                    c.busy_timeout(std::time::Duration::from_secs(10)).unwrap();
                    gate.wait();
                    migrate(c, MIGRATIONS).map(|_| ()).map_err(|e| e.to_string())
                })
            })
            .collect();
        for h in hs {
            h.join().unwrap().unwrap();
        }
        assert_eq!(Store::open(&db).unwrap().schema_version().unwrap(), MIGRATIONS.len() as i64);
    }

    #[test]
    fn records_upsert_and_list_by_kind() {
        let s = Store::in_memory().unwrap();
        s.put_record("a", "2", "{\"n\":1}").unwrap();
        s.put_record("a", "1", "x").unwrap();
        s.put_record("a", "2", "y").unwrap();
        s.put_record("b", "1", "z").unwrap();
        assert_eq!(s.records("a").unwrap(), vec![("1".into(), "x".into()), ("2".into(), "y".into())]);
        assert_eq!(s.records("none").unwrap(), vec![]);
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
