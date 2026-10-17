//! The commander's Powerplay standing from the journal: power, rank, merits, time pledged,
//! merits this session and this weekly cycle, and the next rank with its reward.
//!
//! Events (field names checked against kayahr/ed-journal's typings and journal lines quoted in
//! community trackers, Oct 2026; Frontier's manual predates Powerplay 2.0):
//! - `Powerplay` at login: `Power`, `Rank`, `Merits` (lifetime total), `TimePledged` (seconds);
//! - `PowerplayMerits`: `Power`, `MeritsGained`, `TotalMerits`;
//! - `PowerplayRank`: `Power`, `Rank`;
//! - `PowerplayJoin` / `PowerplayLeave`: `Power`; `PowerplayDefect`: `FromPower`, `ToPower`.
//!
//! Week merits are the sum of `MeritsGained` since the cycle tick, not a difference of totals:
//! the commander is rarely logged in at the tick. Weekly assignment rewards are not in the
//! journal, so the week total can be short of the game's.
//!
//! The game sometimes writes one `PowerplayMerits` twice (same second and gain, `TotalMerits`
//! counting both) but counts it once. The total before an award (`TotalMerits - MeritsGained`)
//! and the login `Merits` give the game's count, so when either is below the running total the
//! difference is taken back from the session and the week.

use crate::rules::rules;
use astromech_journal::{ExtraEvent, JournalEvent, JournalLine, PowerplayEvent};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Standing {
    pub power: Option<String>,
    /// 0 until the five initial assignments are done.
    pub rank: Option<i64>,
    /// Lifetime merits with the power.
    pub merits: Option<i64>,
    /// Time pledged at the last login, in seconds.
    pub time_pledged_secs: Option<i64>,
    /// Merits gained since the last `LoadGame`.
    pub session_merits: i64,
    /// The weekly cycle `cycle_merits` belong to (its start, Unix seconds).
    pub cycle_start: Option<i64>,
    pub cycle_merits: i64,
}

/// The rank after the current one and what it takes.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NextRank {
    pub rank: i64,
    /// Lifetime merits the rank needs.
    pub merits: i64,
    /// Merits still to earn (0 when only the initial assignments are missing).
    pub to_go: i64,
    pub reward: String,
    /// Something the merits don't cover, e.g. the initial assignments for rank 1.
    pub note: Option<String>,
}

/// What the app shows: the standing as of `now`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StandingView {
    pub power: Option<String>,
    pub rank: Option<i64>,
    pub merits: Option<i64>,
    pub time_pledged_secs: Option<i64>,
    pub session_merits: i64,
    pub week_merits: i64,
    /// When this weekly cycle ends (RFC 3339).
    pub week_ends: String,
    pub next: Option<NextRank>,
}

fn parse_ts(ts: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(ts).ok().map(|t| t.with_timezone(&Utc))
}

impl Standing {
    /// Fold one journal event. `ts` is the line's timestamp; without a readable one the merits
    /// still count for the session and the total, but not for the week.
    pub fn apply(&mut self, ts: &str, ev: &JournalEvent) {
        match ev {
            JournalEvent::LoadGame { .. } => self.session_merits = 0,
            JournalEvent::Extra(ExtraEvent::Powerplay(p)) => self.apply_powerplay(ts, p),
            _ => {}
        }
    }

    fn reset_progress(&mut self, power: Option<String>) {
        *self = Standing { power, merits: Some(0), session_merits: self.session_merits, ..Standing::default() };
    }

    /// The game's total is `total`, below the running one: awards it wrote but didn't count.
    fn take_back(&mut self, total: Option<i64>) {
        let excess = self.merits.zip(total).map_or(0, |(ours, game)| ours - game);
        if excess > 0 {
            self.session_merits = (self.session_merits - excess).max(0);
            self.cycle_merits = (self.cycle_merits - excess).max(0);
        }
    }

    fn apply_powerplay(&mut self, ts: &str, p: &PowerplayEvent) {
        match p.kind.as_str() {
            "Powerplay" => {
                if self.power != p.power {
                    self.cycle_start = None;
                    self.cycle_merits = 0;
                } else {
                    self.take_back(p.total_merits);
                }
                self.power = p.power.clone();
                self.rank = p.rank;
                self.merits = p.total_merits;
                self.time_pledged_secs = p.time_pledged_secs;
            }
            "PowerplayMerits" => {
                if self.power.is_some() && p.power.is_some() && self.power != p.power {
                    return;
                }
                if self.power.is_none() {
                    self.power = p.power.clone();
                }
                let gained = p.merits_gained.unwrap_or(0);
                self.take_back(p.total_merits.map(|t| t - gained));
                self.merits = p.total_merits.or(self.merits.map(|m| m + gained));
                self.session_merits += gained;
                if let Some(t) = parse_ts(ts) {
                    let start = rules().cycle_start(t).timestamp();
                    if self.cycle_start != Some(start) {
                        self.cycle_start = Some(start);
                        self.cycle_merits = 0;
                    }
                    self.cycle_merits += gained;
                }
            }
            "PowerplayRank" => {
                if p.power.is_some() {
                    self.power = p.power.clone();
                }
                self.rank = p.rank;
            }
            "PowerplayJoin" => self.reset_progress(p.power.clone()),
            "PowerplayDefect" => self.reset_progress(p.to_power.clone()),
            "PowerplayLeave" => *self = Standing { session_merits: self.session_merits, ..Standing::default() },
            _ => {}
        }
    }

    /// Merits gained in the weekly cycle `now` is in.
    pub fn merits_this_week(&self, now: DateTime<Utc>) -> i64 {
        if self.cycle_start == Some(rules().cycle_start(now).timestamp()) {
            self.cycle_merits
        } else {
            0
        }
    }

    pub fn next_rank(&self) -> Option<NextRank> {
        let rank = self.rank?;
        self.power.as_ref()?;
        let r = rules();
        let next = rank.max(0) + 1;
        let merits = r.merits_for(next)?;
        let to_go = (merits - self.merits.unwrap_or(0)).max(0);
        let note = (next == 1).then(|| "Complete the five initial assignments.".to_string());
        Some(NextRank { rank: next, merits, to_go, reward: r.reward(next).to_string(), note })
    }

    pub fn view(&self, now: DateTime<Utc>) -> StandingView {
        StandingView {
            power: self.power.clone(),
            rank: self.rank,
            merits: self.merits,
            time_pledged_secs: self.time_pledged_secs,
            session_merits: self.session_merits,
            week_merits: self.merits_this_week(now),
            week_ends: (rules().cycle_start(now) + Duration::days(7)).to_rfc3339(),
            next: self.next_rank(),
        }
    }

    /// The spoken answer to "what's my Powerplay rank".
    pub fn spoken(&self) -> String {
        let Some(power) = &self.power else {
            return "Not pledged to a power.".into();
        };
        let mut s = format!("Pledged to {power}");
        match self.rank {
            Some(r) => s += &format!(", rank {r}"),
            None => s += ", rank unknown",
        }
        if let Some(m) = self.merits {
            s += &format!(", {m} merits");
        }
        s.push('.');
        if let Some(n) = self.next_rank() {
            match &n.note {
                Some(note) => s += &format!(" {note}"),
                None => s += &format!(" {} merits to rank {}: {}.", n.to_go, n.rank, n.reward),
            }
        }
        if self.session_merits > 0 {
            s += &format!(" {} merits this session.", self.session_merits);
        }
        s
    }
}

/// Fold journal lines in order.
pub fn standing_from_lines<'a>(lines: impl IntoIterator<Item = &'a JournalLine>) -> Standing {
    let mut s = Standing::default();
    for l in lines {
        s.apply(&l.timestamp, &l.event);
    }
    s
}

/// The standing after every journal in `dir`, oldest first.
pub fn standing_from_journal_dir(dir: &Path) -> Standing {
    crate::from_journal_dir(dir).standing
}
