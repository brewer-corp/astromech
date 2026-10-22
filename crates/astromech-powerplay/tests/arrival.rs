//! Replays a synthetic session (`fixtures/arrival`, Powerplay states as Spansh showed them on
//! 16 Oct 2026) and checks the arrival line at each jump and the summary the page shows.

use astromech_journal::{parse_line, JournalEvent, JournalLine};
use astromech_powerplay::{arrival_line, from_journal_dir, Journal, Work};
use std::path::PathBuf;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/arrival")
}

fn lines() -> Vec<JournalLine> {
    let p = dir().join("Journal.2026-10-16T190000.01.log");
    std::fs::read_to_string(p).unwrap().lines().map(|l| parse_line(l).unwrap()).collect()
}

#[test]
fn each_jump_gives_the_expected_arrival_line() {
    let mut j = Journal::default();
    let mut said = vec![];
    for l in lines() {
        j.apply(&l);
        if matches!(l.event, JournalEvent::FsdJump { .. } | JournalEvent::Location { .. }) {
            said.push(arrival_line(&l.event, j.standing.power.as_deref()));
        }
    }
    assert_eq!(
        said,
        [
            None, // Location at login is not an arrival
            Some(
                "Li Yong-Rui fortified, 28 percent. Reinforcement 82, undermining 3707. Your power: reinforce here."
                    .into()
            ),
            Some(
                "Pranav Antal exploited, 8 percent. Reinforcement 0, undermining 9. Also active: Li Yong-Rui. \
                 Undermining target."
                    .into()
            ),
            Some("Unoccupied. Acquisition: Li Yong-Rui 9 percent, Pranav Antal 0 percent. Acquisition target.".into()),
            None, // no Powerplay activity: nothing to say
            Some(
                "Pranav Antal exploited, 8 percent. Reinforcement 0, undermining 9. Also active: Li Yong-Rui. \
                 Undermining target."
                    .into()
            ),
        ]
    );
}

#[test]
fn not_pledged_hears_the_state_without_a_target() {
    let jump = lines().into_iter().find(|l| l.name == "FSDJump").unwrap();
    assert_eq!(
        arrival_line(&jump.event, None).as_deref(),
        Some("Li Yong-Rui fortified, 28 percent. Reinforcement 82, undermining 3707.")
    );
}

#[test]
fn the_page_shows_the_current_system() {
    let j = from_journal_dir(&dir());
    let o = j.overview();
    assert_eq!(o.here.as_deref(), Some("Piscium Sector EB-X b1-3"));
    let s = o.system.unwrap();
    assert_eq!(s.power.controlling_power.as_deref(), Some("Pranav Antal"));
    assert_eq!(s.power.state.as_deref(), Some("Exploited"));
    assert_eq!(s.work, Some(Work::Undermine));
    assert_eq!(s.summary, "Pranav Antal exploited, 8% · reinforcement 0, undermining 9 · also Li Yong-Rui");
    assert_eq!(o.standing.power.as_deref(), Some("Li Yong-Rui"));

    // A system without Powerplay clears the state but keeps the name.
    let mut j = Journal::default();
    for l in lines().iter().take(8) {
        j.apply(l);
    }
    assert_eq!((j.here.as_deref(), j.system.is_none()), (Some("Test Empty"), true));
}

/// A Powerplay state with no controller (a contested system) is said and shown as that state,
/// not as Unoccupied (astromech-xvov).
#[test]
fn a_state_without_a_controller_is_named() {
    let jump = parse_line(
        r#"{"timestamp":"2026-10-16T20:00:00Z","event":"FSDJump","StarSystem":"Contest","SystemAddress":5,"StarPos":[0,0,0],"JumpDist":5.0,"Powers":["Li Yong-Rui","Pranav Antal"],"PowerplayState":"Contested","PowerplayConflictProgress":[{"Power":"Li Yong-Rui","ConflictProgress":0.6},{"Power":"Pranav Antal","ConflictProgress":0.5}]}"#,
    )
    .unwrap();
    assert_eq!(
        arrival_line(&jump.event, Some("Li Yong-Rui")).as_deref(),
        Some("Contested. Acquisition: Li Yong-Rui 60 percent, Pranav Antal 50 percent. Acquisition target.")
    );
    let mut j = Journal::default();
    j.apply(&jump);
    assert_eq!(j.overview().system.unwrap().summary, "Contested · acquisition: Li Yong-Rui 60%, Pranav Antal 50%");
}
