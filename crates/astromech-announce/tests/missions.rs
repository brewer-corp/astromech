//! Journal replay tests for the mission reminders: expiry warnings at 1 h and 10 min (time
//! injected through `Engine::tick`), and the target-in-this-system line on a jump.

mod common;

use common::Run;
use std::time::{Duration, Instant};

fn epoch(ts: &str) -> i64 {
    chrono::DateTime::parse_from_rfc3339(ts).unwrap().timestamp()
}

/// The login list at 20:00: mission 1 has 2 h 30 min left, mission 2 has 1 h 10 min.
const LOGIN: &str = r#"{"timestamp":"2026-10-16T20:00:00Z","event":"Missions","Active":[{"MissionID":1,"Name":"Mission_Massacre","PassengerMission":false,"Expires":9000},{"MissionID":2,"Name":"Mission_Delivery","PassengerMission":false,"Expires":4200}],"Failed":[],"Complete":[]}"#;

/// Speech from a tick at `ts`.
fn tick(r: &mut Run, ts: &str) -> Vec<String> {
    let now = Instant::now() + Duration::from_secs(1000);
    r.engine.tick(epoch(ts), &r.state, now).into_iter().filter(|x| x.channels.speech).map(|x| x.text).collect()
}

#[test]
fn a_missions_fixture_warns_at_an_hour_and_ten_minutes_once() {
    let mut r = Run::new();
    assert!(r.spoken(LOGIN).is_empty());
    assert!(tick(&mut r, "2026-10-16T20:00:00Z").is_empty());
    // Mission 2 expires 21:10: an hour before is 20:10.
    assert!(tick(&mut r, "2026-10-16T20:09:59Z").is_empty());
    assert_eq!(tick(&mut r, "2026-10-16T20:10:00Z"), ["Mission expires within an hour."]);
    assert!(tick(&mut r, "2026-10-16T20:10:01Z").is_empty(), "once per threshold");
    assert_eq!(tick(&mut r, "2026-10-16T21:00:00Z"), ["Mission expires within 10 minutes."]);
    assert!(tick(&mut r, "2026-10-16T21:00:05Z").is_empty());
    // Mission 1 expires 22:30.
    assert_eq!(tick(&mut r, "2026-10-16T21:30:00Z"), ["Mission expires within an hour."]);
    assert_eq!(tick(&mut r, "2026-10-16T22:20:00Z"), ["Mission expires within 10 minutes."]);
    // Both expired: nothing.
    assert!(tick(&mut r, "2026-10-16T23:00:00Z").is_empty());
}

#[test]
fn missions_crossing_together_share_one_line() {
    let mut r = Run::new();
    r.spoken(LOGIN);
    r.spoken(r#"{"timestamp":"2026-10-16T20:00:01Z","event":"MissionAccepted","MissionID":3,"Name":"Mission_Courier","Faction":"F","Expiry":"2026-10-16T21:20:00Z"}"#);
    assert_eq!(tick(&mut r, "2026-10-16T20:30:00Z"), ["2 missions expire within an hour."]);
}

#[test]
fn going_live_late_gives_one_line_for_the_tightest_threshold() {
    let mut r = Run::new();
    r.spoken(LOGIN);
    // Mission 2 (21:10 expiry) is 5 minutes away at 21:05: only the 10 minute line, not the hour line too.
    assert_eq!(tick(&mut r, "2026-10-16T21:05:00Z"), ["Mission expires within 10 minutes."]);
}

#[test]
fn a_finished_mission_is_not_warned_about() {
    let mut r = Run::new();
    r.spoken(LOGIN);
    r.spoken(
        r#"{"timestamp":"2026-10-16T20:30:00Z","event":"MissionCompleted","MissionID":2,"Name":"Mission_Delivery"}"#,
    );
    assert!(tick(&mut r, "2026-10-16T21:15:00Z").is_empty());
}

fn accept(dest: Option<&str>, faction: Option<&str>) -> String {
    let d = dest.map(|d| format!(r#","DestinationSystem":"{d}""#)).unwrap_or_default();
    let f = faction.map(|f| format!(r#","TargetFaction":"{f}""#)).unwrap_or_default();
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"MissionAccepted","MissionID":5,"Name":"Mission_Massacre","Faction":"Boss"{d}{f},"KillCount":20,"Expiry":"2026-10-17T20:00:00Z"}}"#
    )
}

fn jump(system: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:10:00Z","event":"FSDJump","StarSystem":"{system}","StarPos":[0,0,0],"JumpDist":10.0,"Factions":[{{"Name":"Pirates Inc","Allegiance":"Independent","FactionState":"None","Influence":0.4}}]}}"#
    )
}

#[test]
fn a_jump_into_a_mission_destination_says_so() {
    let mut r = Run::new();
    r.spoken(&accept(Some("Target Sys"), Some("Pirates Inc")));
    assert!(r.spoken(&jump("Elsewhere")).is_empty());
    assert_eq!(r.spoken(&jump("target sys")), ["Mission target in this system."]);
}

#[test]
fn a_target_faction_in_the_system_counts_when_the_mission_names_no_system() {
    let mut r = Run::new();
    r.spoken(&accept(None, Some("pirates inc")));
    assert_eq!(r.spoken(&jump("Anywhere")), ["Mission target faction here."]);
    let mut r = Run::new();
    r.spoken(&accept(None, Some("Someone Else")));
    assert!(r.spoken(&jump("Anywhere")).is_empty());
}
