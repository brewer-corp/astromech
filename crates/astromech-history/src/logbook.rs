//! The logbook (astromech-fxp0): the places a commander has been, with how often and when,
//! the timeline of arrivals and dockings, and a search over both.
//!
//! Search is a case-insensitive substring match on the system name and the station name
//! (`col 285` finds `Col 285 Sector ...`; `lincoln` finds Abraham Lincoln). Names and query are
//! folded in Rust with Unicode rules (SQLite's `lower` folds ASCII only). A place matches on
//! its latest name, and its counts are over all its visits. Systems are one place per system
//! address, stations one per market id, so a renamed or moved fleet carrier stays one place.

use crate::ledgers::{Commander, Visit};
use astromech_store::rusqlite::{params, Result};
use serde::Serialize;

/// A visited system or station.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Place {
    /// `system` or `station`.
    pub kind: String,
    /// The system; for a station, the system it was in on the latest visit.
    pub star_system: String,
    pub system_address: Option<i64>,
    pub station_name: Option<String>,
    pub market_id: Option<i64>,
    pub visits: i64,
    pub first: String,
    pub last: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Totals {
    /// Different systems and stations visited, and every arrival and docking.
    pub systems: i64,
    pub stations: i64,
    pub system_visits: i64,
    pub station_visits: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Logbook {
    pub commander: Option<Commander>,
    pub commanders: Vec<Commander>,
    pub totals: Totals,
    /// The search.
    pub query: String,
    /// Places matching the search, latest visit first, at most `limit`.
    pub places: Vec<Place>,
    /// How many places match in all.
    pub matches: usize,
    /// Arrivals and dockings matching the search, newest first, at most `limit`.
    pub timeline: Vec<Visit>,
}

/// Whether a place or visit named `star_system` / `station_name` matches the folded query `q`.
fn is_match(q: &str, star_system: &str, station_name: Option<&str>) -> bool {
    q.is_empty() || star_system.to_lowercase().contains(q) || station_name.is_some_and(|s| s.to_lowercase().contains(q))
}

impl crate::History {
    /// The logbook of `cmdr` (the most recently played commander if `None`), searched for
    /// `query`, with at most `limit` places and timeline entries.
    pub fn logbook(&self, cmdr: Option<&str>, query: &str, limit: usize) -> crate::Result<Logbook> {
        let commanders = self.commanders()?;
        let commander = match cmdr {
            Some(k) => commanders.iter().find(|c| c.cmdr == k).cloned(),
            None => commanders.first().cloned(),
        };
        let key = commander.as_ref().map(|c| c.cmdr.clone()).unwrap_or_default();
        let q = query.trim().to_lowercase();
        let c = &self.conn;
        let totals = c.query_row(
            "SELECT count(DISTINCT CASE WHEN kind = 'system' THEN coalesce(system_address, star_system) END),
                    count(DISTINCT CASE WHEN kind = 'station' THEN coalesce(market_id, star_system || '|' || station_name) END),
                    count(CASE WHEN kind = 'system' THEN 1 END),
                    count(CASE WHEN kind = 'station' THEN 1 END)
             FROM visits WHERE cmdr = ?1",
            [&key],
            |r| Ok(Totals { systems: r.get(0)?, stations: r.get(1)?, system_visits: r.get(2)?, station_visits: r.get(3)? }),
        )?;
        let sql = "WITH keyed AS (
                SELECT *, CASE WHEN kind = 'system' THEN coalesce(system_address, star_system)
                               ELSE coalesce(market_id, star_system || '|' || station_name) END AS place
                FROM visits WHERE cmdr = ?1),
             latest AS (
                SELECT kind, place, max(ts) AS last, star_system, system_address, station_name, market_id
                FROM keyed GROUP BY kind, place),
             counts AS (SELECT kind, place, count(*) AS n, min(ts) AS first FROM keyed GROUP BY kind, place)
             SELECT l.kind, l.star_system, l.system_address, l.station_name, l.market_id, n, first, l.last
             FROM latest l JOIN counts USING (kind, place)
             ORDER BY l.last DESC, l.kind";
        let mut st = c.prepare(sql)?;
        let mut places = st
            .query_map(params![key], |r| {
                Ok(Place {
                    kind: r.get(0)?,
                    star_system: r.get(1)?,
                    system_address: r.get(2)?,
                    station_name: r.get(3)?,
                    market_id: r.get(4)?,
                    visits: r.get(5)?,
                    first: r.get(6)?,
                    last: r.get(7)?,
                })
            })?
            .filter(|p| p.as_ref().map_or(true, |p| is_match(&q, &p.star_system, p.station_name.as_deref())))
            .collect::<Result<Vec<_>>>()?;
        let matches = places.len();
        places.truncate(limit);
        let mut st = c.prepare(
            "SELECT ts, kind, star_system, system_address, station_name, market_id, jump_dist, via FROM visits
             WHERE cmdr = ?1 ORDER BY ts DESC, id DESC",
        )?;
        let timeline = st
            .query_map(params![key], |r| {
                Ok(Visit {
                    ts: r.get(0)?,
                    kind: r.get(1)?,
                    star_system: r.get(2)?,
                    system_address: r.get(3)?,
                    station_name: r.get(4)?,
                    market_id: r.get(5)?,
                    jump_dist: r.get(6)?,
                    via: r.get(7)?,
                })
            })?
            .filter(|v| v.as_ref().map_or(true, |v| is_match(&q, &v.star_system, v.station_name.as_deref())))
            .take(limit)
            .collect::<Result<Vec<_>>>()?;
        Ok(Logbook { commander, commanders, totals, query: query.trim().to_string(), places, matches, timeline })
    }
}
