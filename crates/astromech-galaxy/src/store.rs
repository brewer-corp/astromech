//! Local SQLite: known star systems (preload + fill-in), which spheres are fully covered, and a
//! small response cache. Schema shared with `scripts/gen-starmap.py`.

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS systems(id64 INTEGER PRIMARY KEY, name TEXT NOT NULL,
  x INTEGER NOT NULL, y INTEGER NOT NULL, z INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS coverage(id INTEGER PRIMARY KEY, x REAL NOT NULL, y REAL NOT NULL,
  z REAL NOT NULL, radius REAL NOT NULL, source TEXT NOT NULL, fetched TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS meta(key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS cache(key TEXT PRIMARY KEY, fetched INTEGER NOT NULL, body TEXT NOT NULL);
";

/// A star system: id64 and position in light years.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Star {
    pub id64: i64,
    pub pos: [f32; 3],
}

/// A sphere in which every known system is in the store.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: [f64; 3],
    pub radius: f64,
}

impl Sphere {
    pub fn contains(&self, p: [f64; 3]) -> bool {
        dist(self.center, p) <= self.radius
    }
}

pub fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

fn grid(v: f64) -> i64 {
    (v * 32.0).round() as i64
}

pub struct Store {
    db: Connection,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d)?;
        }
        let db = Connection::open(path)?;
        db.execute_batch("PRAGMA journal_mode=WAL;")?;
        db.execute_batch(SCHEMA)?;
        Ok(Store { db })
    }

    pub fn in_memory() -> Result<Self> {
        let db = Connection::open_in_memory()?;
        db.execute_batch(SCHEMA)?;
        Ok(Store { db })
    }

    /// Copy the bundled preload in once (marked in `meta`). Missing or empty preloads are fine.
    pub fn seed_from(&mut self, preload: &Path) -> Result<bool> {
        let done: Option<String> =
            self.db.query_row("SELECT value FROM meta WHERE key='preload'", [], |r| r.get(0)).optional()?;
        if done.is_some() || !preload.is_file() {
            return Ok(false);
        }
        self.db.execute("ATTACH DATABASE ?1 AS pre", params![preload.to_string_lossy()])?;
        // The CI placeholder has the schema but no coverage: skip it without marking.
        let n: i64 = self.db.query_row("SELECT count(*) FROM pre.coverage", [], |r| r.get(0))?;
        if n == 0 {
            self.db.execute("DETACH DATABASE pre", [])?;
            return Ok(false);
        }
        let tx = self.db.transaction()?;
        tx.execute("INSERT OR IGNORE INTO systems SELECT * FROM pre.systems", [])?;
        tx.execute(
            "INSERT INTO coverage(x, y, z, radius, source, fetched) SELECT x, y, z, radius, source, fetched FROM pre.coverage",
            [],
        )?;
        let src: String = tx
            .query_row("SELECT value FROM pre.meta WHERE key='source'", [], |r| r.get(0))
            .optional()?
            .unwrap_or_else(|| "preload".into());
        tx.execute("INSERT OR REPLACE INTO meta VALUES ('preload', ?1)", params![src])?;
        tx.commit()?;
        self.db.execute("DETACH DATABASE pre", [])?;
        Ok(true)
    }

    pub fn insert_systems<'a>(&mut self, rows: impl IntoIterator<Item = (i64, &'a str, [f64; 3])>) -> Result<usize> {
        let tx = self.db.transaction()?;
        let mut n = 0;
        {
            let mut st = tx.prepare("INSERT OR REPLACE INTO systems VALUES (?1, ?2, ?3, ?4, ?5)")?;
            for (id64, name, p) in rows {
                st.execute(params![id64, name, grid(p[0]), grid(p[1]), grid(p[2])])?;
                n += 1;
            }
        }
        tx.commit()?;
        Ok(n)
    }

    pub fn add_coverage(&self, s: Sphere, source: &str, fetched: &str) -> Result<()> {
        self.db.execute(
            "INSERT INTO coverage(x, y, z, radius, source, fetched) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![s.center[0], s.center[1], s.center[2], s.radius, source, fetched],
        )?;
        Ok(())
    }

    pub fn coverage(&self) -> Result<Vec<Sphere>> {
        let mut st = self.db.prepare("SELECT x, y, z, radius FROM coverage")?;
        let rows = st.query_map([], |r| Ok(Sphere { center: [r.get(0)?, r.get(1)?, r.get(2)?], radius: r.get(3)? }))?;
        Ok(rows.collect::<Result<_, _>>()?)
    }

    pub fn stars(&self) -> Result<Vec<Star>> {
        let mut st = self.db.prepare("SELECT id64, x, y, z FROM systems")?;
        let rows = st.query_map([], |r| {
            let c = |i: usize| -> rusqlite::Result<f32> { Ok(r.get::<_, i64>(i)? as f32 / 32.0) };
            Ok(Star { id64: r.get(0)?, pos: [c(1)?, c(2)?, c(3)?] })
        })?;
        Ok(rows.collect::<Result<_, _>>()?)
    }

    pub fn system(&self, name: &str) -> Result<Option<(i64, [f64; 3])>> {
        Ok(self
            .db
            .query_row("SELECT id64, x, y, z FROM systems WHERE name = ?1 COLLATE NOCASE LIMIT 1", params![name], |r| {
                let c = |i: usize| -> rusqlite::Result<f64> { Ok(r.get::<_, i64>(i)? as f64 / 32.0) };
                Ok((r.get(0)?, [c(1)?, c(2)?, c(3)?]))
            })
            .optional()?)
    }

    pub fn count(&self) -> Result<i64> {
        Ok(self.db.query_row("SELECT count(*) FROM systems", [], |r| r.get(0))?)
    }

    pub fn meta(&self, key: &str) -> Result<Option<String>> {
        Ok(self.db.query_row("SELECT value FROM meta WHERE key = ?1", params![key], |r| r.get(0)).optional()?)
    }

    /// Cached body for `key` if younger than `max_age_s`.
    pub fn cache_get(&self, key: &str, now_s: i64, max_age_s: i64) -> Result<Option<(i64, String)>> {
        Ok(self
            .db
            .query_row("SELECT fetched, body FROM cache WHERE key = ?1", params![key], |r| Ok((r.get(0)?, r.get(1)?)))
            .optional()?
            .filter(|(t, _): &(i64, String)| now_s - *t <= max_age_s))
    }

    pub fn cache_put(&self, key: &str, now_s: i64, body: &str) -> Result<()> {
        self.db.execute("INSERT OR REPLACE INTO cache VALUES (?1, ?2, ?3)", params![key, now_s, body])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_insert_cover_cache() {
        let d = tempfile::tempdir().unwrap();
        let pre = d.path().join("pre.db");
        {
            let mut p = Store::open(&pre).unwrap();
            p.insert_systems([
                (10477373803, "Sol", [0.0, 0.0, 0.0]),
                (1, "Alpha Centauri", [3.03125, -0.09375, 3.15625]),
            ])
            .unwrap();
            p.add_coverage(Sphere { center: [0.0; 3], radius: 500.0 }, "preload", "t").unwrap();
            p.db.execute("INSERT INTO meta VALUES ('source', 'test preload')", []).unwrap();
        }
        let empty = d.path().join("empty.db");
        Store::open(&empty).unwrap();
        let mut s = Store::open(&d.path().join("galaxy.db")).unwrap();
        assert!(!s.seed_from(&empty).unwrap(), "placeholder is skipped");
        assert!(s.seed_from(&pre).unwrap());
        assert!(!s.seed_from(&pre).unwrap(), "seeds once");
        assert_eq!(s.count().unwrap(), 2);
        assert_eq!(s.meta("preload").unwrap().as_deref(), Some("test preload"));
        assert!(s.coverage().unwrap()[0].contains([100.0, 0.0, 0.0]));
        let (_, p) = s.system("alpha centauri").unwrap().unwrap();
        assert_eq!(p, [3.03125, -0.09375, 3.15625]);
        assert_eq!(s.stars().unwrap().iter().find(|x| x.id64 == 1).unwrap().pos[0], 3.03125);
        s.cache_put("k", 100, "body").unwrap();
        assert_eq!(s.cache_get("k", 150, 60).unwrap().unwrap().1, "body");
        assert!(s.cache_get("k", 200, 60).unwrap().is_none());
    }
}
