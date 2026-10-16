//! # astromech-history
//!
//! The commander's journal history (ARCHITECTURE.md §18.5): every line of every
//! `Journal.*.log` in an `events` table, imported resumably, plus ledgers derived from it
//! (visits, earnings, ships, scans) for the logbook, "been here before" and session stats.
//!
//! - The history lives in its own SQLite file (`history.sqlite`), not in the settings store:
//!   it grows to hundreds of MB, it can always be rebuilt from the journals, and its long
//!   import transactions must not hold the write lock the voice path uses for settings.
//! - `events` is the source of truth. The ledgers are written in the same transaction as the
//!   events they come from, so they always match what has been imported, and
//!   [`History::rebuild_ledgers`] recomputes them from `events` (after a ledger schema change).
//! - Progress is the byte offset of the last complete line imported per file, committed with
//!   that batch of events. A killed import loses at most its open batch and resumes there;
//!   `(file_id, offset)` is unique, so no line is stored twice.
//! - A line without its newline yet (the game is still writing it) waits for the next import.
//! - Every event carries its commander (see `commander`), and every ledger is per commander:
//!   the same journal folder can hold several commanders, and Legacy-mode play.

mod commander;
mod ledgers;
mod logbook;

pub use ledgers::{Commander, Earning, Scan, Ship, Visit};
pub use logbook::{Logbook, Place, Totals};

use commander::FileCommander;

use astromech_journal::parse_line;
use astromech_store::rusqlite::{self, params, Connection, OptionalExtension};
use std::collections::hash_map::{Entry, HashMap};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};

const MIGRATIONS: &[&str] = &[
    // v1
    "CREATE TABLE journal_files (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        offset INTEGER NOT NULL DEFAULT 0,
        lines INTEGER NOT NULL DEFAULT 0
     );
     CREATE TABLE events (
        id INTEGER PRIMARY KEY,
        file_id INTEGER NOT NULL,
        offset INTEGER NOT NULL,
        ts TEXT NOT NULL,
        event TEXT NOT NULL,
        json TEXT NOT NULL,
        UNIQUE (file_id, offset)
     );
     CREATE INDEX events_event_ts ON events(event, ts);
     CREATE INDEX events_ts ON events(ts);
     CREATE TABLE visits (
        id INTEGER PRIMARY KEY,
        event_id INTEGER NOT NULL,
        ts TEXT NOT NULL,
        kind TEXT NOT NULL CHECK (kind IN ('system', 'station')),
        star_system TEXT NOT NULL,
        system_address INTEGER,
        station_name TEXT,
        market_id INTEGER,
        jump_dist REAL
     );
     CREATE INDEX visits_system ON visits(star_system);
     CREATE INDEX visits_address ON visits(system_address);
     CREATE INDEX visits_market ON visits(market_id);
     CREATE INDEX visits_ts ON visits(ts);
     CREATE TABLE earnings (
        event_id INTEGER PRIMARY KEY,
        ts TEXT NOT NULL,
        kind TEXT NOT NULL,
        amount INTEGER NOT NULL,
        cost INTEGER,
        detail TEXT
     );
     CREATE INDEX earnings_ts ON earnings(ts);
     CREATE TABLE ships (
        ship_id INTEGER PRIMARY KEY,
        ship_type TEXT NOT NULL,
        name TEXT,
        ident TEXT,
        first_seen TEXT NOT NULL,
        last_flown TEXT,
        acquired TEXT,
        sold TEXT
     );
     CREATE TABLE scans (
        event_id INTEGER PRIMARY KEY,
        ts TEXT NOT NULL,
        kind TEXT NOT NULL CHECK (kind IN ('body', 'mapped', 'organic')),
        system_address INTEGER,
        star_system TEXT,
        body_name TEXT,
        body_id INTEGER,
        class TEXT,
        landable INTEGER,
        terraformable INTEGER,
        was_discovered INTEGER,
        was_mapped INTEGER
     );
     CREATE INDEX scans_body ON scans(system_address, body_id);
     CREATE INDEX scans_ts ON scans(ts);",
    // v2 — commanders (astromech-i4gx). Events get their commander back from the journal lines
    // and the ledgers are rebuilt per commander: see `PENDING_COMMANDERS`.
    "ALTER TABLE journal_files ADD COLUMN cmdr TEXT;
     ALTER TABLE journal_files ADD COLUMN legacy INTEGER NOT NULL DEFAULT 0;
     ALTER TABLE events ADD COLUMN cmdr TEXT;
     DROP INDEX events_ts;
     CREATE INDEX events_cmdr_ts ON events(cmdr, ts);
     DROP TABLE visits;
     DROP TABLE earnings;
     DROP TABLE ships;
     DROP TABLE scans;
     CREATE TABLE commanders (
        cmdr TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        legacy INTEGER NOT NULL,
        first_seen TEXT NOT NULL,
        last_seen TEXT NOT NULL
     );
     CREATE TABLE visits (
        id INTEGER PRIMARY KEY,
        cmdr TEXT NOT NULL,
        event_id INTEGER NOT NULL,
        ts TEXT NOT NULL,
        kind TEXT NOT NULL CHECK (kind IN ('system', 'station')),
        star_system TEXT NOT NULL,
        system_address INTEGER,
        station_name TEXT,
        market_id INTEGER,
        jump_dist REAL
     );
     CREATE INDEX visits_cmdr_ts ON visits(cmdr, ts);
     CREATE INDEX visits_system ON visits(star_system);
     CREATE INDEX visits_address ON visits(system_address);
     CREATE INDEX visits_market ON visits(market_id);
     CREATE TABLE earnings (
        event_id INTEGER PRIMARY KEY,
        cmdr TEXT NOT NULL,
        ts TEXT NOT NULL,
        kind TEXT NOT NULL,
        amount INTEGER NOT NULL,
        cost INTEGER,
        detail TEXT
     );
     CREATE INDEX earnings_cmdr_ts ON earnings(cmdr, ts);
     CREATE TABLE ships (
        cmdr TEXT NOT NULL,
        ship_id INTEGER NOT NULL,
        ship_type TEXT NOT NULL,
        name TEXT,
        ident TEXT,
        first_seen TEXT NOT NULL,
        last_flown TEXT,
        acquired TEXT,
        sold TEXT,
        PRIMARY KEY (cmdr, ship_id)
     );
     CREATE TABLE scans (
        event_id INTEGER PRIMARY KEY,
        cmdr TEXT NOT NULL,
        ts TEXT NOT NULL,
        kind TEXT NOT NULL CHECK (kind IN ('body', 'mapped', 'organic')),
        system_address INTEGER,
        star_system TEXT,
        body_name TEXT,
        body_id INTEGER,
        class TEXT,
        landable INTEGER,
        terraformable INTEGER,
        was_discovered INTEGER,
        was_mapped INTEGER
     );
     CREATE INDEX scans_body ON scans(system_address, body_id);
     CREATE INDEX scans_cmdr_ts ON scans(cmdr, ts);
     CREATE TABLE pending (task TEXT PRIMARY KEY);
     INSERT INTO pending VALUES ('commanders');",
    // v3 — ledgers follow game time (astromech-u3f4): ships remember the time of the event that
    // last set their type and name, and every ledger is rebuilt in game-time order.
    "ALTER TABLE ships ADD COLUMN updated TEXT NOT NULL DEFAULT '';
     INSERT OR IGNORE INTO pending VALUES ('rebuild');",
];

/// Work a migration leaves for Rust code, in the `pending` table so a crash before it is done
/// redoes it on the next open: fill `events.cmdr` and `journal_files.cmdr` from the stored
/// lines, then rebuild the ledgers.
const PENDING_COMMANDERS: &str = "commanders";
/// Rebuild every ledger (migration v3).
const PENDING_REBUILD: &str = "rebuild";
/// Prefix of a task to rebuild one commander's ledgers, left by an import that stored events
/// older than ones that commander's ledgers already had.
const PENDING_COMMANDER_REBUILD: &str = "rebuild:";

#[derive(Debug, thiserror::Error)]
pub enum HistoryError {
    #[error("history database: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("reading {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("the history database is at schema {found}, this build reads {want}; the core upgrades it")]
    Schema { found: i64, want: i64 },
    /// Raised by [`ImportOptions::crash_after_lines`].
    #[error("import stopped by the crash test hook")]
    Crashed,
}

pub type Result<T> = std::result::Result<T, HistoryError>;

#[derive(Debug, Clone)]
pub struct ImportOptions {
    /// Lines per transaction. Progress is committed after each batch.
    pub batch_lines: usize,
    /// Test hook: after this many lines, return [`HistoryError::Crashed`] without committing the
    /// open batch, like a process killed mid-import.
    #[doc(hidden)]
    pub crash_after_lines: Option<u64>,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self { batch_lines: 20_000, crash_after_lines: None }
    }
}

/// What one [`History::import`] run did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportReport {
    /// Files with new complete lines.
    pub files: usize,
    pub lines: u64,
    pub bytes: u64,
    /// Bytes not imported yet when the run started (the most it will read; an unfinished last
    /// line stays behind), for a progress bar.
    pub bytes_total: u64,
    /// New rows in `events`.
    pub events: u64,
    /// Lines that are not a journal event (bad JSON, no `event` or `timestamp`); skipped.
    pub malformed: u64,
    /// Files shorter than the offset already imported (never expected); left alone.
    pub shrunk: Vec<String>,
}

pub struct History {
    conn: Connection,
}

impl History {
    pub fn open(path: &Path) -> Result<Self> {
        Self::init(Connection::open(path)?)
    }

    /// Open for reading only, alongside the core that writes it (the app window). Doesn't
    /// create, migrate or rebuild anything.
    pub fn open_read_only(path: &Path) -> Result<Self> {
        let conn = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        let found: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        let want = MIGRATIONS.len() as i64;
        if found != want {
            return Err(HistoryError::Schema { found, want });
        }
        Ok(Self { conn })
    }

    pub fn in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Self> {
        let conn = astromech_store::migrate(conn, MIGRATIONS)?;
        // WAL with NORMAL sync: a power cut can lose the last commits, never corrupt; the
        // journals are still there to re-import from.
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        let mut h = Self { conn };
        h.run_pending()?;
        Ok(h)
    }

    /// Do the work migrations and interrupted imports left in `pending`.
    fn run_pending(&mut self) -> Result<()> {
        let tasks: Vec<String> = {
            let mut st = self.conn.prepare("SELECT task FROM pending ORDER BY task")?;
            let rows = st.query_map([], |r| r.get(0))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        if tasks.iter().any(|t| t == PENDING_COMMANDERS) {
            self.rederive_commanders()?;
        } else if tasks.iter().any(|t| t == PENDING_REBUILD) {
            let tx = self.conn.transaction()?;
            rebuild(&tx)?;
            tx.execute("DELETE FROM pending WHERE task = ?1", [PENDING_REBUILD])?;
            tx.commit()?;
        }
        for cmdr in tasks.iter().filter_map(|t| t.strip_prefix(PENDING_COMMANDER_REBUILD)) {
            let tx = self.conn.transaction()?;
            rebuild_commander(&tx, cmdr)?;
            tx.execute("DELETE FROM pending WHERE task = ?1", [format!("{PENDING_COMMANDER_REBUILD}{cmdr}")])?;
            tx.commit()?;
        }
        Ok(())
    }

    /// Recompute each event's commander from the stored lines, then the ledgers (migration v2).
    fn rederive_commanders(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;
        {
            let mut files = tx.prepare("SELECT id FROM journal_files")?;
            let ids: Vec<i64> = files.query_map([], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?;
            let mut events = tx.prepare("SELECT id, json FROM events WHERE file_id = ?1 ORDER BY offset")?;
            let mut set = tx.prepare("UPDATE events SET cmdr = ?2 WHERE id = ?1")?;
            for file_id in ids {
                let mut state = FileCommander::default();
                let mut rows = events.query([file_id])?;
                while let Some(r) = rows.next()? {
                    let json: String = r.get(1)?;
                    if let Ok(line) = parse_line(&json) {
                        state.observe(&line);
                    }
                    set.execute(params![r.get::<_, i64>(0)?, state.cmdr])?;
                }
                tx.execute(
                    "UPDATE journal_files SET cmdr = ?2, legacy = ?3 WHERE id = ?1",
                    params![file_id, state.cmdr, state.legacy],
                )?;
            }
        }
        rebuild(&tx)?;
        tx.execute("DELETE FROM pending WHERE task IN (?1, ?2)", [PENDING_COMMANDERS, PENDING_REBUILD])?;
        tx.commit()?;
        Ok(())
    }

    /// Read-only access for queries over the history.
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Import every complete line not imported yet from the `Journal.*.log` files in `dir`,
    /// oldest file first.
    pub fn import(&mut self, dir: &Path, opts: &ImportOptions) -> Result<ImportReport> {
        self.import_with(dir, opts, |_| {})
    }

    /// [`History::import`], calling `progress` with the report so far after every batch.
    pub fn import_with(
        &mut self,
        dir: &Path,
        opts: &ImportOptions,
        mut progress: impl FnMut(&ImportReport),
    ) -> Result<ImportReport> {
        self.run_pending()?;
        let mut report = ImportReport::default();
        let mut todo = vec![];
        for path in astromech_journal::journal_logs(dir) {
            let len = std::fs::metadata(&path).map_err(|source| HistoryError::Io { path: path.clone(), source })?.len();
            let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            let (file_id, start, state) = self.file_state(&name)?;
            if len < start {
                report.shrunk.push(name);
            } else if len > start {
                report.bytes_total += len - start;
                todo.push((path, file_id, start, state));
            }
        }
        progress(&report);
        let mut seen = 0u64;
        // Per commander, the newest event time its ledgers hold.
        let mut newest: HashMap<String, String> = HashMap::new();
        for (path, file_id, start, mut state) in todo {
            let io = |source| HistoryError::Io { path: path.clone(), source };
            let mut file = std::fs::File::open(&path).map_err(io)?;
            file.seek(SeekFrom::Start(start)).map_err(io)?;
            let mut reader = BufReader::with_capacity(1 << 16, file);
            let mut buf = Vec::with_capacity(4096);
            let mut pos = start;
            let mut committed = start;
            let mut batch = 0usize;
            let mut file_lines = 0u64;
            let mut tx = self.conn.transaction()?;
            loop {
                buf.clear();
                let n = reader.read_until(b'\n', &mut buf).map_err(io)?;
                if n == 0 || buf.last() != Some(&b'\n') {
                    break; // end of file, or a line the game hasn't finished writing
                }
                if opts.crash_after_lines.is_some_and(|limit| seen >= limit) {
                    return Err(HistoryError::Crashed); // `tx` drops: the open batch rolls back
                }
                seen += 1;
                report.lines += 1;
                let offset = pos;
                pos += n as u64;
                file_lines += 1;
                let text = String::from_utf8_lossy(&buf);
                let text = text.trim().trim_start_matches('\u{feff}');
                if !text.is_empty() {
                    match parse_line(text) {
                        Ok(line) => {
                            state.observe(&line);
                            if let Some(id) = insert_event(&tx, file_id, offset, state.cmdr.as_deref(), &line)? {
                                if let Some(cmdr) = &state.cmdr {
                                    ledgers::derive(&tx, id, cmdr, &line)?;
                                    let top = match newest.entry(cmdr.clone()) {
                                        Entry::Occupied(o) => o.into_mut(),
                                        Entry::Vacant(v) => v.insert(newest_event(&tx, cmdr, id)?),
                                    };
                                    if line.timestamp < *top {
                                        tx.prepare_cached("INSERT OR IGNORE INTO pending VALUES (?1)")?
                                            .execute([format!("{PENDING_COMMANDER_REBUILD}{cmdr}")])?;
                                    } else {
                                        top.clone_from(&line.timestamp);
                                    }
                                }
                                report.events += 1;
                            }
                        }
                        Err(_) => report.malformed += 1,
                    }
                }
                batch += 1;
                if batch >= opts.batch_lines.max(1) {
                    set_progress(&tx, file_id, pos, file_lines, &state)?;
                    tx.commit()?;
                    report.bytes += pos - committed;
                    committed = pos;
                    progress(&report);
                    tx = self.conn.transaction()?;
                    file_lines = 0;
                    batch = 0;
                }
            }
            set_progress(&tx, file_id, pos, file_lines, &state)?;
            tx.commit()?;
            report.bytes += pos - committed;
            if pos > start {
                report.files += 1;
            }
            progress(&report);
        }
        self.run_pending()?;
        Ok(report)
    }

    /// Recompute every ledger from `events`, in import order.
    pub fn rebuild_ledgers(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;
        rebuild(&tx)?;
        tx.commit()?;
        Ok(())
    }

    /// Number of rows in `events`.
    pub fn event_count(&self) -> Result<u64> {
        Ok(self.conn.query_row("SELECT COUNT(*) FROM events", [], |r| r.get::<_, i64>(0))? as u64)
    }

    /// `(file_id, imported byte offset, commander state there)`, adding the file if it is new.
    fn file_state(&self, name: &str) -> Result<(i64, u64, FileCommander)> {
        let found = self
            .conn
            .query_row("SELECT id, offset, cmdr, legacy FROM journal_files WHERE name = ?1", [name], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, i64>(1)? as u64,
                    FileCommander { cmdr: r.get(2)?, legacy: r.get(3)? },
                ))
            })
            .optional()?;
        if let Some(f) = found {
            return Ok(f);
        }
        self.conn.execute("INSERT INTO journal_files(name) VALUES (?1)", [name])?;
        Ok((self.conn.last_insert_rowid(), 0, FileCommander::default()))
    }
}

/// The new event's id, or `None` if this line is already stored.
fn insert_event(
    c: &Connection,
    file_id: i64,
    offset: u64,
    cmdr: Option<&str>,
    line: &astromech_journal::JournalLine,
) -> Result<Option<i64>> {
    let mut st = c.prepare_cached(
        "INSERT OR IGNORE INTO events(file_id, offset, cmdr, ts, event, json) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;
    let n = st.execute(params![file_id, offset as i64, cmdr, line.timestamp, line.name, line.raw_json])?;
    Ok((n == 1).then(|| c.last_insert_rowid()))
}

fn set_progress(c: &Connection, file_id: i64, offset: u64, new_lines: u64, state: &FileCommander) -> Result<()> {
    c.prepare_cached("UPDATE journal_files SET offset = ?2, lines = lines + ?3, cmdr = ?4, legacy = ?5 WHERE id = ?1")?
        .execute(params![file_id, offset as i64, new_lines as i64, state.cmdr, state.legacy])?;
    Ok(())
}

/// Newest event time stored for `cmdr` before event `id`.
fn newest_event(c: &Connection, cmdr: &str, id: i64) -> Result<String> {
    Ok(c.prepare_cached("SELECT coalesce(max(ts), '') FROM events WHERE cmdr = ?1 AND id < ?2")?
        .query_row(params![cmdr, id], |r| r.get(0))?)
}

/// Clear one commander's ledgers and derive them again from their events, in game-time order.
fn rebuild_commander(c: &Connection, cmdr: &str) -> Result<()> {
    for table in ["commanders", "visits", "earnings", "ships", "scans"] {
        c.execute(&format!("DELETE FROM {table} WHERE cmdr = ?1"), [cmdr])?;
    }
    let mut st = c.prepare("SELECT id, json FROM events WHERE cmdr = ?1 ORDER BY ts, id")?;
    let mut rows = st.query([cmdr])?;
    while let Some(r) = rows.next()? {
        let (id, json): (i64, String) = (r.get(0)?, r.get(1)?);
        if let Ok(line) = parse_line(&json) {
            ledgers::derive(c, id, cmdr, &line)?;
        }
    }
    Ok(())
}

/// Clear the ledgers and derive them again from every stored event, in game-time order.
fn rebuild(c: &Connection) -> Result<()> {
    c.execute_batch(
        "DELETE FROM commanders; DELETE FROM visits; DELETE FROM earnings; DELETE FROM ships; DELETE FROM scans;",
    )?;
    let mut st = c.prepare("SELECT id, cmdr, json FROM events WHERE cmdr IS NOT NULL ORDER BY ts, id")?;
    let mut rows = st.query([])?;
    while let Some(r) = rows.next()? {
        let (id, cmdr, json): (i64, String, String) = (r.get(0)?, r.get(1)?, r.get(2)?);
        if let Ok(line) = parse_line(&json) {
            ledgers::derive(c, id, &cmdr, &line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
