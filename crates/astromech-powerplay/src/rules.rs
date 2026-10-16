//! Powerplay 2.0 rules as data (`data/catalog/powerplay.json`): merits per rank, the reward at
//! each rank, the weekly cycle and the powers. Frontier changes Powerplay often, so these live
//! in the data file, not in code.

use chrono::{DateTime, Datelike, Duration, TimeZone, Utc, Weekday};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct RankMerits {
    rank: i64,
    merits: i64,
}

#[derive(Deserialize)]
struct RankReward {
    rank: i64,
    reward: String,
}

#[derive(Deserialize)]
struct RulesFile {
    rank_merits: Vec<RankMerits>,
    merits_per_rank_after: i64,
    rank_rewards: Vec<RankReward>,
    other_rank_reward: String,
    cycle_weekday: String,
    cycle_hour_utc: u32,
    powers: Vec<String>,
}

pub struct Rules {
    file: RulesFile,
    weekday: Weekday,
}

/// The bundled rules.
pub fn rules() -> &'static Rules {
    static R: OnceLock<Rules> = OnceLock::new();
    R.get_or_init(|| {
        let file: RulesFile =
            serde_json::from_str(include_str!("../../../data/catalog/powerplay.json")).expect("powerplay.json");
        let weekday = file.cycle_weekday.parse().expect("powerplay.json cycle_weekday");
        Rules { file, weekday }
    })
}

impl Rules {
    /// Lifetime merits needed for `rank` (1 and up). Rank 1 also needs the five initial
    /// assignments, which the journal doesn't record.
    pub fn merits_for(&self, rank: i64) -> Option<i64> {
        if rank < 1 {
            return None;
        }
        let table = &self.file.rank_merits;
        if let Some(r) = table.iter().find(|r| r.rank == rank) {
            return Some(r.merits);
        }
        let last = table.last()?;
        (rank > last.rank).then(|| last.merits + (rank - last.rank) * self.file.merits_per_rank_after)
    }

    /// What reaching `rank` gives.
    pub fn reward(&self, rank: i64) -> &str {
        self.file.rank_rewards.iter().find(|r| r.rank == rank).map_or(&self.file.other_rank_reward, |r| &r.reward)
    }

    /// The twelve powers, as the journal and Spansh spell them.
    pub fn powers(&self) -> impl Iterator<Item = &str> {
        self.file.powers.iter().map(String::as_str)
    }

    /// Start of the weekly cycle `t` falls in (the most recent cycle tick at or before `t`).
    pub fn cycle_start(&self, t: DateTime<Utc>) -> DateTime<Utc> {
        let back = (7 + t.weekday().num_days_from_monday() - self.weekday.num_days_from_monday()) % 7;
        let day = (t - Duration::days(back as i64)).date_naive();
        let tick = Utc.from_utc_datetime(&day.and_hms_opt(self.file.cycle_hour_utc, 0, 0).expect("cycle hour"));
        if tick > t {
            tick - Duration::days(7)
        } else {
            tick
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    #[test]
    fn rank_thresholds_follow_the_wiki_table() {
        let r = rules();
        assert_eq!(r.merits_for(0), None);
        assert_eq!(r.merits_for(1), Some(0));
        assert_eq!(r.merits_for(5), Some(15_000));
        // "+8,000 merits per Rank (ex: Rank 8 would require 39,000 merits)"
        assert_eq!(r.merits_for(8), Some(39_000));
        assert_eq!(r.merits_for(10), Some(55_000));
        assert_eq!(r.merits_for(100), Some(775_000));
        assert_eq!(r.merits_for(130), Some(1_015_000));
        assert_eq!(r.reward(34), "First power module");
        assert_eq!(r.reward(9), "Care package");
        assert_eq!(r.powers().count(), 12);
    }

    #[test]
    fn the_cycle_turns_on_thursday_at_seven_utc() {
        let r = rules();
        // Thursday 15 Oct 2026, just before and after the tick.
        assert_eq!(r.cycle_start(at("2026-10-15T06:59:59Z")), at("2026-10-08T07:00:00Z"));
        assert_eq!(r.cycle_start(at("2026-10-15T07:00:00Z")), at("2026-10-15T07:00:00Z"));
        assert_eq!(r.cycle_start(at("2026-10-21T23:00:00Z")), at("2026-10-15T07:00:00Z"));
    }
}
