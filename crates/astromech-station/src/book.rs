//! The outfitting book: one row per station (its latest module list, searchable) and every
//! `Outfitting.json` as the game wrote it.

use astromech_engineering::data;
use astromech_journal::companion::OutfittingSnap;
use astromech_store::rusqlite::{self, params, Connection, OptionalExtension, Result};
use serde::Serialize;
use std::path::Path;

/// The book's file name in the data folder.
pub const DB_FILE: &str = "outfitting.sqlite";

const MIGRATIONS: &[&str] = &[
    // v1
    "CREATE TABLE snapshots (
        market_id INTEGER NOT NULL,
        ts TEXT NOT NULL,
        station_name TEXT NOT NULL,
        star_system TEXT NOT NULL,
        raw_json TEXT NOT NULL,
        PRIMARY KEY (market_id, ts)
     );
     CREATE TABLE stations (
        market_id INTEGER PRIMARY KEY,
        station_name TEXT NOT NULL,
        star_system TEXT NOT NULL,
        horizons INTEGER,
        first_seen TEXT NOT NULL,
        seen TEXT NOT NULL
     );
     CREATE INDEX stations_seen ON stations(seen);
     CREATE TABLE station_modules (
        market_id INTEGER NOT NULL,
        item TEXT NOT NULL,
        item_id INTEGER,
        name TEXT NOT NULL,
        search TEXT NOT NULL,
        price INTEGER NOT NULL,
        PRIMARY KEY (market_id, item)
     );
     CREATE INDEX station_modules_item ON station_modules(item);",
];

/// A module as the player knows it: "5A Frame Shift Drive", or the symbol when the catalog
/// doesn't have it.
fn display(item: &str) -> String {
    data::module(item).map(|m| m.display()).unwrap_or_else(|| item.replace('_', " "))
}

/// A station's latest outfitting.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StationOutfitting {
    pub market_id: i64,
    pub station_name: String,
    pub star_system: String,
    /// When the game wrote the file this list is from.
    pub seen: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Item {
    /// Outfitting symbol, lowercase (`int_hyperdrive_size5_class5`).
    pub item: String,
    /// "5A Frame Shift Drive"
    pub name: String,
    pub price: i64,
}

/// One station that sold a module you searched for.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Seen {
    pub market_id: i64,
    pub station_name: String,
    pub star_system: String,
    /// When the outfitting was last seen.
    pub seen: String,
    pub item: String,
    pub name: String,
    pub price: i64,
    /// Light years from where the commander is; filled in by the caller that knows both
    /// positions (the book has no star map).
    pub distance_ly: Option<f64>,
}

pub struct OutfittingBook {
    conn: Connection,
}

/// `%` and `_` in a search term are text, not wildcards.
fn like(term: &str) -> String {
    let mut s = String::from("%");
    for c in term.chars() {
        if matches!(c, '%' | '_' | '\\') {
            s.push('\\');
        }
        s.push(c);
    }
    s.push('%');
    s
}

impl OutfittingBook {
    pub fn open(path: &Path) -> Result<Self> {
        Self::init(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Self> {
        Ok(Self { conn: astromech_store::migrate(conn, MIGRATIONS)? })
    }

    /// Open for reading only, alongside the core that writes it (the app window). `Ok(None)`
    /// when no outfitting was ever seen (no file yet).
    pub fn open_read_only(path: &Path) -> Result<Option<Self>> {
        if !path.is_file() {
            return Ok(None);
        }
        let conn = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        Ok(Some(Self { conn }))
    }

    /// Keep `snap`. Its raw file is stored once (the same `MarketID` and `timestamp` again is
    /// the game rewriting the file, or a journal replay at startup); the station's module list
    /// becomes `snap`'s unless a newer snapshot of the station is already there. Returns
    /// whether the snapshot was new. A snapshot without a `MarketID` can't be told apart from
    /// another station's, so it is skipped.
    pub fn record(&mut self, snap: &OutfittingSnap) -> Result<bool> {
        let Some(market_id) = snap.market_id else { return Ok(false) };
        let tx = self.conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let added = tx.execute(
            "INSERT OR IGNORE INTO snapshots (market_id, ts, station_name, star_system, raw_json) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![market_id, snap.timestamp, snap.station_name, snap.star_system, snap.raw_json],
        )? > 0;
        let newest: Option<String> =
            tx.query_row("SELECT seen FROM stations WHERE market_id = ?1", [market_id], |r| r.get(0)).optional()?;
        // Timestamps are the game's `YYYY-MM-DDThh:mm:ssZ`, so text order is time order.
        if added && newest.as_deref().is_none_or(|n| n < snap.timestamp.as_str()) {
            tx.execute(
                "INSERT INTO stations (market_id, station_name, star_system, horizons, first_seen, seen)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?5)
                 ON CONFLICT(market_id) DO UPDATE SET station_name = ?2, star_system = ?3, horizons = ?4, seen = ?5",
                params![market_id, snap.station_name, snap.star_system, snap.horizons, snap.timestamp],
            )?;
            tx.execute("DELETE FROM station_modules WHERE market_id = ?1", [market_id])?;
            for i in &snap.items {
                let name = display(&i.name);
                let search = format!("{} {}", i.name, name).to_lowercase();
                tx.execute(
                    "INSERT OR REPLACE INTO station_modules (market_id, item, item_id, name, search, price) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![market_id, i.name.to_lowercase(), i.id, name, search, i.buy_price],
                )?;
            }
        } else if added {
            // An older file than the one the list is from: keep it, leave the list alone.
            tx.execute(
                "UPDATE stations SET first_seen = ?2 WHERE market_id = ?1 AND first_seen > ?2",
                params![market_id, snap.timestamp],
            )?;
        }
        tx.commit()?;
        Ok(added)
    }

    /// The latest list for a station.
    pub fn station(&self, market_id: i64) -> Result<Option<StationOutfitting>> {
        let head = self
            .conn
            .query_row("SELECT station_name, star_system, seen FROM stations WHERE market_id = ?1", [market_id], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?))
            })
            .optional()?;
        let Some((station_name, star_system, seen)) = head else { return Ok(None) };
        let mut q = self
            .conn
            .prepare("SELECT item, name, price FROM station_modules WHERE market_id = ?1 ORDER BY name, item")?;
        let items = q
            .query_map([market_id], |r| Ok(Item { item: r.get(0)?, name: r.get(1)?, price: r.get(2)? }))?
            .collect::<Result<Vec<_>>>()?;
        Ok(Some(StationOutfitting { market_id, station_name, star_system, seen, items }))
    }

    /// Stations whose latest outfitting sells a module matching every word of `query`, most
    /// recently seen first. A word matches the module's symbol or its name with class and
    /// rating (`5a frame shift`, `hyperdrive_size5`, `pulse laser`); `fsd` stands for
    /// `hyperdrive`.
    pub fn find(&self, query: &str, limit: usize) -> Result<Vec<Seen>> {
        let words: Vec<String> = query
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .map(|w| if w == "fsd" { "hyperdrive".to_string() } else { w })
            .map(|w| like(&w))
            .collect();
        if words.is_empty() {
            return Ok(vec![]);
        }
        let cond =
            (1..=words.len()).map(|i| format!("m.search LIKE ?{i} ESCAPE '\\'")).collect::<Vec<_>>().join(" AND ");
        let sql = format!(
            "SELECT s.market_id, s.station_name, s.star_system, s.seen, m.item, m.name, m.price
             FROM station_modules m JOIN stations s USING (market_id)
             WHERE {cond}
             ORDER BY s.seen DESC, s.market_id, m.name, m.item LIMIT {limit}"
        );
        let mut q = self.conn.prepare(&sql)?;
        let rows = q.query_map(rusqlite::params_from_iter(words.iter()), |r| {
            Ok(Seen {
                market_id: r.get(0)?,
                station_name: r.get(1)?,
                star_system: r.get(2)?,
                seen: r.get(3)?,
                item: r.get(4)?,
                name: r.get(5)?,
                price: r.get(6)?,
                distance_ly: None,
            })
        })?;
        rows.collect()
    }

    /// (stations, snapshots) kept.
    pub fn counts(&self) -> Result<(i64, i64)> {
        Ok((
            self.conn.query_row("SELECT COUNT(*) FROM stations", [], |r| r.get(0))?,
            self.conn.query_row("SELECT COUNT(*) FROM snapshots", [], |r| r.get(0))?,
        ))
    }

    /// Every file kept for a station, oldest first, as the game wrote it.
    pub fn raw_snapshots(&self, market_id: i64) -> Result<Vec<(String, String)>> {
        let mut q = self.conn.prepare("SELECT ts, raw_json FROM snapshots WHERE market_id = ?1 ORDER BY ts")?;
        let rows = q.query_map([market_id], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect()
    }
}
