//! Replays synthetic journals (`fixtures/standing`: three sessions across the Thursday cycle tick)
//! and checks the standing the app and voice show.

use astromech_journal::parse_line;
use astromech_powerplay::{standing_from_journal_dir, standing_from_lines, Standing};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/standing")
}

fn at(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}

#[test]
fn replaying_the_journals_gives_the_standing() {
    let s = standing_from_journal_dir(&dir());
    assert_eq!(s.power.as_deref(), Some("Li Yong-Rui"));
    assert_eq!(s.rank, Some(9), "PowerplayRank after the login snapshot");
    assert_eq!(s.merits, Some(47_129));
    assert_eq!(s.time_pledged_secs, Some(22_680_000), "from the last login");
    assert_eq!(s.session_merits, 1_329, "since the last LoadGame");

    // Friday evening: this cycle began Thursday 07:00 UTC, so 500 + 1300 + 29; Wednesday's 300
    // belong to last week.
    let v = s.view(at("2026-10-16T20:00:00Z"));
    assert_eq!(v.week_merits, 1_829);
    assert_eq!(v.week_ends, "2026-10-22T07:00:00+00:00");
    let next = v.next.unwrap();
    assert_eq!((next.rank, next.merits, next.to_go), (10, 55_000, 7_871));
    assert_eq!(next.reward, "Care package");
    assert!(next.note.is_none());

    // After the next tick nothing has been earned yet.
    assert_eq!(s.merits_this_week(at("2026-10-22T07:00:00Z")), 0);

    assert_eq!(
        s.spoken(),
        "Pledged to Li Yong-Rui, rank 9, 47129 merits. 7871 merits to rank 10: Care package. 1329 merits this session."
    );
}

fn fold(lines: &[&str]) -> Standing {
    let parsed: Vec<_> = lines.iter().map(|l| parse_line(l).unwrap()).collect();
    standing_from_lines(&parsed)
}

#[test]
fn joining_defecting_and_leaving() {
    let s = fold(&[
        r#"{"timestamp":"2026-10-16T10:00:00Z","event":"PowerplayJoin","Power":"Nakato Kaine"}"#,
        r#"{"timestamp":"2026-10-16T10:10:00Z","event":"PowerplayMerits","Power":"Nakato Kaine","MeritsGained":40,"TotalMerits":40}"#,
    ]);
    assert_eq!((s.power.as_deref(), s.merits, s.rank), (Some("Nakato Kaine"), Some(40), None));
    let s = fold(&[
        r#"{"timestamp":"2026-10-16T10:00:00Z","event":"Powerplay","Power":"Nakato Kaine","Rank":0,"Merits":900,"TimePledged":3600}"#,
    ]);
    let next = s.next_rank().unwrap();
    assert_eq!((next.rank, next.to_go), (1, 0));
    assert_eq!(next.note.as_deref(), Some("Complete the five initial assignments."));
    assert_eq!(s.spoken(), "Pledged to Nakato Kaine, rank 0, 900 merits. Complete the five initial assignments.");

    let s = fold(&[
        r#"{"timestamp":"2026-10-16T10:00:00Z","event":"Powerplay","Power":"Nakato Kaine","Rank":3,"Merits":6000,"TimePledged":3600}"#,
        r#"{"timestamp":"2026-10-16T10:10:00Z","event":"PowerplayMerits","Power":"Nakato Kaine","MeritsGained":40,"TotalMerits":6040}"#,
        r#"{"timestamp":"2026-10-16T10:20:00Z","event":"PowerplayDefect","FromPower":"Nakato Kaine","ToPower":"Edmund Mahon"}"#,
    ]);
    assert_eq!((s.power.as_deref(), s.merits, s.rank), (Some("Edmund Mahon"), Some(0), None));
    assert_eq!(s.merits_this_week(at("2026-10-16T11:00:00Z")), 0, "a defection starts over");
    assert_eq!(s.session_merits, 40, "the session still earned them");

    let s = fold(&[
        r#"{"timestamp":"2026-10-16T10:00:00Z","event":"Powerplay","Power":"Nakato Kaine","Rank":3,"Merits":6000,"TimePledged":3600}"#,
        r#"{"timestamp":"2026-10-16T10:20:00Z","event":"PowerplayLeave","Power":"Nakato Kaine"}"#,
    ]);
    assert_eq!(s.power, None);
    assert_eq!(s.spoken(), "Not pledged to a power.");
}

#[test]
fn merits_for_another_power_are_ignored() {
    let s = fold(&[
        r#"{"timestamp":"2026-10-16T10:00:00Z","event":"Powerplay","Power":"Li Yong-Rui","Rank":8,"Merits":45000,"TimePledged":1}"#,
        r#"{"timestamp":"2026-10-16T10:10:00Z","event":"PowerplayMerits","Power":"Yuri Grom","MeritsGained":40,"TotalMerits":40}"#,
    ]);
    assert_eq!((s.merits, s.session_merits), (Some(45_000), 0));
}

/// The game sometimes writes one award twice (same second and gain, `TotalMerits` counting both)
/// but counts it once, as the next award's `TotalMerits - MeritsGained` or the next login's
/// `Merits` show (seen in a real journal; the values here are made up).
#[test]
fn an_award_the_game_wrote_twice_counts_once() {
    const LOGIN: &str = r#"{"timestamp":"2026-10-15T09:00:00Z","event":"Powerplay","Power":"Nakato Kaine","Rank":0,"Merits":1200,"TimePledged":360000}"#;
    const LOAD: &str = r#"{"timestamp":"2026-10-16T08:00:00Z","event":"LoadGame","FID":"F0000001","Commander":"Jameson","Horizons":true,"Odyssey":true,"Ship":"Mandalay","ShipID":9,"GameMode":"Solo","Credits":1000,"Loan":0}"#;
    let award = |ts: &str, gained: i64, total: i64| {
        format!(
            r#"{{"timestamp":"2026-10-16T{ts}Z","event":"PowerplayMerits","Power":"Nakato Kaine","MeritsGained":{gained},"TotalMerits":{total}}}"#
        )
    };
    let (a, b, dup) = (award("06:00:00", 31, 1231), award("06:00:07", 41, 1272), award("06:00:07", 41, 1313));
    let now = at("2026-10-16T12:00:00Z");

    // Corrected by the next login's total.
    let next_login = r#"{"timestamp":"2026-10-16T08:00:30Z","event":"Powerplay","Power":"Nakato Kaine","Rank":0,"Merits":1272,"TimePledged":441000}"#;
    let s = fold(&[LOGIN, &a, &b, &dup, LOAD, next_login]);
    assert_eq!((s.merits, s.merits_this_week(now), s.session_merits), (Some(1272), 72, 0));

    // Corrected by the next award in the same session.
    let next = award("07:00:00", 17, 1289);
    let s = fold(&[LOGIN, &a, &b, &dup, &next]);
    assert_eq!((s.merits, s.merits_this_week(now), s.session_merits), (Some(1289), 89, 89));

    // Two real awards of the same size in one second both count.
    let s = fold(&[LOGIN, &a, &b, &dup, &award("07:00:00", 17, 1330)]);
    assert_eq!((s.merits, s.merits_this_week(now)), (Some(1330), 130));
}

/// The game writes `Powerplay` at the first login of a game run when pledged, before
/// `Missions`; a relog from the main menu in the same run doesn't write it (seen in real
/// journals). So a run's first login with no `Powerplay` before `Missions` means not pledged,
/// even when the `PowerplayLeave` is in a journal since deleted (astromech-xvov).
#[test]
fn first_login_without_powerplay_clears_the_pledge() {
    let header = r#"{"timestamp":"2026-10-17T10:00:00Z","event":"Fileheader","part":1,"gameversion":"4.4.1.1"}"#;
    let load = r#"{"timestamp":"2026-10-17T10:00:05Z","event":"LoadGame","FID":"F1","Commander":"T"}"#;
    let pp = r#"{"timestamp":"2026-10-17T10:00:06Z","event":"Powerplay","Power":"Li Yong-Rui","Rank":8,"Merits":45000,"TimePledged":100}"#;
    let missions = r#"{"timestamp":"2026-10-17T10:00:07Z","event":"Missions","Active":[],"Failed":[],"Complete":[]}"#;
    let relog = r#"{"timestamp":"2026-10-17T11:00:05Z","event":"LoadGame","FID":"F1","Commander":"T"}"#;
    let relog_missions =
        r#"{"timestamp":"2026-10-17T11:00:07Z","event":"Missions","Active":[],"Failed":[],"Complete":[]}"#;
    let next_run = r#"{"timestamp":"2026-10-18T10:00:00Z","event":"Fileheader","part":1,"gameversion":"4.4.1.1"}"#;
    let next_load = r#"{"timestamp":"2026-10-18T10:00:05Z","event":"LoadGame","FID":"F1","Commander":"T"}"#;
    let next_missions =
        r#"{"timestamp":"2026-10-18T10:00:07Z","event":"Missions","Active":[],"Failed":[],"Complete":[]}"#;

    let pledged = [header, load, pp, missions];
    assert_eq!(fold(&pledged).power.as_deref(), Some("Li Yong-Rui"));
    // A relog in the same run: still pledged.
    assert_eq!(fold(&[header, load, pp, missions, relog, relog_missions]).power.as_deref(), Some("Li Yong-Rui"));
    // The next run's login has no Powerplay: not pledged.
    let s = fold(&[header, load, pp, missions, next_run, next_load, next_missions]);
    assert_eq!((s.power, s.rank, s.merits), (None, None, None));
    // A run cut short before Missions decides nothing.
    assert_eq!(fold(&[header, load, pp, missions, next_run, next_load]).power.as_deref(), Some("Li Yong-Rui"));
}

/// The same, read from the journal folder.
#[test]
fn first_login_without_powerplay_clears_the_pledge_from_the_folder() {
    let d = tempfile::tempdir().unwrap();
    for f in std::fs::read_dir(dir()).unwrap() {
        let f = f.unwrap().path();
        std::fs::copy(&f, d.path().join(f.file_name().unwrap())).unwrap();
    }
    std::fs::write(
        d.path().join("Journal.2026-10-18T100000.01.log"),
        r#"{"timestamp":"2026-10-18T10:00:00Z","event":"Fileheader","part":1,"gameversion":"4.4.1.1"}
{"timestamp":"2026-10-18T10:00:05Z","event":"LoadGame","FID":"F0000000","Commander":"Test"}
{"timestamp":"2026-10-18T10:00:07Z","event":"Missions","Active":[],"Failed":[],"Complete":[]}
"#,
    )
    .unwrap();
    assert_eq!(standing_from_journal_dir(d.path()).power, None);
}

/// The page's follower reads only what the game appended (astromech-xvov), and gives what a
/// full replay gives.
#[test]
fn follower_reads_only_new_lines() {
    use astromech_powerplay::{from_journal_dir, Follower};
    use std::io::Write;
    let d = tempfile::tempdir().unwrap();
    let mut logs: Vec<PathBuf> = vec![];
    for f in std::fs::read_dir(dir()).unwrap() {
        let f = f.unwrap().path();
        let to = d.path().join(f.file_name().unwrap());
        std::fs::copy(&f, &to).unwrap();
        logs.push(to);
    }
    logs.sort();
    let append = |p: &PathBuf, text: &str| {
        std::fs::OpenOptions::new().append(true).open(p).unwrap().write_all(text.as_bytes()).unwrap();
    };
    let merits = |ts: &str, total: i64| {
        format!(
            r#"{{"timestamp":"{ts}","event":"PowerplayMerits","Power":"Li Yong-Rui","MeritsGained":10,"TotalMerits":{total}}}"#
        )
    };
    let mut f = Follower::new(d.path());
    assert_eq!(*f.refresh(), from_journal_dir(d.path()));
    let session = f.refresh().standing.session_merits;

    // The newest journal grows, a line at a time and half a line at a time.
    let newest = logs.last().unwrap();
    append(newest, &format!("{}\n", merits("2026-10-16T21:00:00Z", 47_139)));
    assert_eq!(f.refresh().standing.session_merits, session + 10);
    let line = merits("2026-10-16T21:01:00Z", 47_149);
    let (a, b) = line.split_at(30);
    append(newest, a);
    assert_eq!(f.refresh().standing.session_merits, session + 10, "half a line waits");
    append(newest, &format!("{b}\n"));
    assert_eq!(f.refresh().standing.session_merits, session + 20);
    assert_eq!(*f.refresh(), from_journal_dir(d.path()));

    // A new journal starts.
    std::fs::write(
        d.path().join("Journal.2026-10-17T100000.01.log"),
        format!(
            "{}\n{}\n{}\n",
            r#"{"timestamp":"2026-10-17T10:00:00Z","event":"Fileheader","part":1,"gameversion":"4.4.1.1"}"#,
            r#"{"timestamp":"2026-10-17T10:00:05Z","event":"LoadGame","FID":"F0000000","Commander":"Test"}"#,
            r#"{"timestamp":"2026-10-17T10:00:06Z","event":"Powerplay","Power":"Li Yong-Rui","Rank":9,"Merits":47149,"TimePledged":22700000}"#,
        ),
    )
    .unwrap();
    assert_eq!(f.refresh().standing.time_pledged_secs, Some(22_700_000));
    assert_eq!(*f.refresh(), from_journal_dir(d.path()));

    // Finished journals aren't read again: a jump added to the oldest one isn't seen...
    append(
        &logs[0],
        &format!(
            "{}\n",
            r#"{"timestamp":"2026-10-14T23:00:00Z","event":"FSDJump","StarSystem":"Sol","SystemAddress":1,"StarPos":[0,0,0],"JumpDist":1.0}"#
        ),
    );
    assert_eq!(f.refresh().here, None);
    assert_eq!(from_journal_dir(d.path()).here.as_deref(), Some("Sol"));
    // ...until the folder no longer starts with the files read: then it replays them all.
    std::fs::remove_file(&logs[1]).unwrap();
    assert_eq!(f.refresh().here.as_deref(), Some("Sol"));
    assert_eq!(*f.refresh(), from_journal_dir(d.path()));
}
