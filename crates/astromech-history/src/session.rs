//! Session summary (astromech-bg0x): what one game session earned and covered, like EDMC's
//! hourly-income plugin.
//!
//! A session is a commander's events from a `LoadGame` up to their next `LoadGame` (or the
//! newest event: the session being played). Its length runs from that `LoadGame` to the
//! session's last event.
//!
//! - Earnings are the `earnings` ledger in the session, by kind (the list is in `ledgers`);
//!   `per_hour` is their total over the session's length.
//! - Jumps and distance are `FSDJump`s only; carrier jumps are counted on their own.
//! - Bounties awarded are the `Bounty` events (vouchers not yet cashed in); cashed-in bounty
//!   vouchers are the `bounty` earnings.
//! - Data sold is `exploration` plus `organic` earnings.

use crate::ledgers::Commander;
use astromech_journal::{ExtraEvent, JournalEvent};
use astromech_store::rusqlite::{params, Result};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EarningTotal {
    pub kind: String,
    pub amount: i64,
    /// Events paid.
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Session {
    pub commander: Commander,
    /// The `LoadGame` that started it, and the session's last event.
    pub start: String,
    pub end: String,
    pub duration_secs: i64,
    /// True when no later session has started (it may still be going).
    pub latest: bool,
    /// By kind, largest first.
    pub earnings: Vec<EarningTotal>,
    pub total: i64,
    /// Trade income minus what the goods cost (where the game reported it).
    pub trade_profit: i64,
    /// `total` per hour of session, once it has lasted a minute.
    pub per_hour: Option<i64>,
    pub jumps: i64,
    pub distance_ly: f64,
    pub carrier_jumps: i64,
    pub bounties_awarded: i64,
    pub bounty_value: i64,
    pub data_sold: i64,
}

impl crate::History {
    /// The session of `cmdr` (the most recently played commander if `None`) that was going on
    /// at `at` (the latest session if `None`). `None` if there is no such session.
    pub fn session(&self, cmdr: Option<&str>, at: Option<&str>) -> crate::Result<Option<Session>> {
        let commanders = self.commanders()?;
        let Some(commander) = (match cmdr {
            Some(k) => commanders.into_iter().find(|c| c.cmdr == k),
            None => commanders.into_iter().next(),
        }) else {
            return Ok(None);
        };
        let c = &self.conn;
        let k = commander.cmdr.as_str();
        let start: Option<String> = c.query_row(
            "SELECT max(ts) FROM events WHERE event = 'LoadGame' AND cmdr = ?1 AND (?2 IS NULL OR ts <= ?2)",
            params![k, at],
            |r| r.get(0),
        )?;
        let Some(start) = start else { return Ok(None) };
        let next: Option<String> = c.query_row(
            "SELECT min(ts) FROM events WHERE event = 'LoadGame' AND cmdr = ?1 AND ts > ?2",
            params![k, start],
            |r| r.get(0),
        )?;
        // The window is [start, next); with no next session, open-ended.
        let upto = next.clone().unwrap_or_else(|| "~".into());
        let (end, duration_secs): (String, i64) = c.query_row(
            "SELECT max(ts), CAST(strftime('%s', max(ts)) AS INTEGER) - CAST(strftime('%s', ?2) AS INTEGER)
             FROM events WHERE cmdr = ?1 AND ts >= ?2 AND ts < ?3",
            params![k, start, upto],
            |r| Ok((r.get(0)?, r.get::<_, Option<i64>>(1)?.unwrap_or(0))),
        )?;
        let mut st = c.prepare(
            "SELECT kind, sum(amount), count(*), sum(CASE WHEN kind = 'trade' THEN amount - coalesce(cost, 0) END)
             FROM earnings WHERE cmdr = ?1 AND ts >= ?2 AND ts < ?3 GROUP BY kind ORDER BY sum(amount) DESC, kind",
        )?;
        let mut trade_profit = 0;
        let earnings = st
            .query_map(params![k, start, upto], |r| {
                Ok((EarningTotal { kind: r.get(0)?, amount: r.get(1)?, count: r.get(2)? }, r.get::<_, Option<i64>>(3)?))
            })?
            .map(|row| {
                row.map(|(e, profit)| {
                    trade_profit += profit.unwrap_or(0);
                    e
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let total = earnings.iter().map(|e| e.amount).sum();
        let data_sold =
            earnings.iter().filter(|e| e.kind == "exploration" || e.kind == "organic").map(|e| e.amount).sum();
        let (jumps, distance_ly, carrier_jumps): (i64, f64, i64) = c.query_row(
            "SELECT count(CASE WHEN via = 'FSDJump' THEN 1 END), coalesce(sum(jump_dist), 0),
                    count(CASE WHEN via = 'CarrierJump' THEN 1 END)
             FROM visits WHERE cmdr = ?1 AND kind = 'system' AND ts >= ?2 AND ts < ?3",
            params![k, start, upto],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )?;
        let mut bounties_awarded = 0;
        let mut bounty_value = 0;
        let mut st =
            c.prepare("SELECT json FROM events WHERE event = 'Bounty' AND cmdr = ?1 AND ts >= ?2 AND ts < ?3")?;
        let mut rows = st.query(params![k, start, upto])?;
        while let Some(r) = rows.next()? {
            if let Ok(line) = astromech_journal::parse_line(&r.get::<_, String>(0)?) {
                if let JournalEvent::Extra(ExtraEvent::Bounty { rewards, total_reward, .. }) = line.event {
                    bounties_awarded += 1;
                    bounty_value += total_reward.unwrap_or_else(|| rewards.iter().map(|r| r.reward).sum());
                }
            }
        }
        let per_hour = (duration_secs >= 60).then(|| (total as f64 * 3600.0 / duration_secs as f64).round() as i64);
        Ok(Some(Session {
            commander,
            start,
            end,
            duration_secs,
            latest: next.is_none(),
            earnings,
            total,
            trade_profit,
            per_hour,
            jumps,
            distance_ly,
            carrier_jumps,
            bounties_awarded,
            bounty_value,
            data_sold,
        }))
    }
}
