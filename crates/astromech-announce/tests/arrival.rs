//! Journal replay tests for the arrival briefing.

mod common;

use astromech_announce::arrival::{self, ShoppingList};
use astromech_announce::*;
use common::Run;

/// A jump into `system` with the given factions (allegiance, state, influence) and extra keys.
fn jump(system: &str, population: i64, factions: &[(&str, &str, f64)], extra: &str) -> String {
    let f: Vec<String> = factions
        .iter()
        .map(|(al, st, inf)| {
            let active = if *st == "None" { String::new() } else { format!(r#"{{"State":"{st}"}}"#) };
            format!(
                r#"{{"Name":"{al} {st} Party","Allegiance":"{al}","FactionState":"{st}","Influence":{inf},"ActiveStates":[{active}]}}"#
            )
        })
        .collect();
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"FSDJump","StarSystem":"{system}","SystemAddress":1,"StarPos":[1,2,3],"SystemAllegiance":"Independent","SystemEconomy_Localised":"Industrial","SystemSecurity":"$SYSTEM_SECURITY_low;","SystemSecurity_Localised":"Low Security","Population":{population},"JumpDist":12.5,"FuelUsed":2.1,"FuelLevel":20.0,"Factions":[{}]{extra}}}"#,
        f.join(",")
    )
}

/// The arrival star's automatic scan.
fn autoscan(was_discovered: bool) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:01Z","event":"Scan","ScanType":"AutoScan","BodyName":"Sys A","BodyID":1,"StarSystem":"Sys","SystemAddress":1,"DistanceFromArrivalLS":0.0,"StarType":"K","Subclass":3,"Luminosity":"Va","WasDiscovered":{was_discovered},"WasMapped":false}}"#
    )
}

const START_JUMP: &str = r#"{"timestamp":"2026-10-16T19:59:50Z","event":"StartJump","JumpType":"Hyperspace","StarSystem":"Sys","SystemAddress":1,"StarClass":"K"}"#;

fn shopping(ids: &[&str]) -> Run {
    let r = Run::new();
    *r.shared.shopping.write().unwrap() = ShoppingList::wanting(ids);
    r
}

#[test]
fn a_system_whose_state_the_shopping_list_wants_gets_its_state_line() {
    let mut r = shopping(&["fedcorecomposites", "protoheatradiators"]);
    let boom = jump("Sys", 3_000_000, &[("Independent", "Boom", 0.4), ("Federation", "None", 0.6)], "");
    assert_eq!(r.spoken(&boom), ["Boom here. Proto Heat Radiators."]);
    // Famine counts for wake scans.
    let mut r = shopping(&["dataminedwake"]);
    assert_eq!(
        r.spoken(&jump("Sys", 0, &[("Empire", "Famine", 0.3)], "")),
        ["Famine here. Datamined Wake Exceptions."]
    );
    // War and civil war name themselves; the most influential faction's state goes first.
    let mut r = shopping(&["militarysupercapacitors", "improvisedcomponents"]);
    let two = jump("Sys", 5_000_000, &[("Alliance", "CivilUnrest", 0.2), ("Federation", "CivilWar", 0.5)], "");
    assert_eq!(r.spoken(&two), ["Civil war here. Military Supercapacitors."]);
    let one = jump("Sys", 5_000_000, &[("Alliance", "CivilUnrest", 0.2)], "");
    assert_eq!(r.spoken(&one), ["Civil unrest here. Improvised Components."]);
}

#[test]
fn a_state_the_shopping_list_does_not_want_gets_nothing() {
    // Boom, but nothing on the list comes from it.
    let mut r = shopping(&["militarygradealloys"]);
    assert!(r.spoken(&jump("Sys", 3_000_000, &[("Independent", "Boom", 0.9)], "")).is_empty());
    // An empty list: nothing, whatever the states.
    let mut r = Run::new();
    assert!(r.spoken(&jump("Sys", 3_000_000, &[("Independent", "Boom", 0.5), ("Empire", "War", 0.5)], "")).is_empty());
    // A Federation faction in Boom spawns Federation composites, not the Proto pool.
    let mut r = shopping(&["protoheatradiators"]);
    assert!(r.spoken(&jump("Sys", 3_000_000, &[("Federation", "Boom", 0.9)], "")).is_empty());
}

#[test]
fn a_visited_fully_known_system_with_nothing_notable_gets_nothing() {
    let mut r = shopping(&["protoheatradiators", "dataminedwake"]);
    let quiet = [
        START_JUMP.to_string(),
        jump("Sys", 85_000_000, &[("Federation", "None", 0.7), ("Empire", "Election", 0.3)], ""),
        autoscan(true),
    ];
    assert!(r.spoken(&quiet.join("\n")).is_empty());
}

#[test]
fn an_undiscovered_system_is_said_after_the_jump() {
    let mut r = Run::new();
    let lines = [START_JUMP.to_string(), jump("Sys", 0, &[], ""), autoscan(false)];
    assert_eq!(r.spoken(&lines.join("\n")), ["Undiscovered system."]);
    // Only the arrival star's automatic scan: a body scanned later in the FSS is not news.
    let later = autoscan(false).replace("\"AutoScan\"", "\"Detailed\"");
    assert!(r.spoken(&later).is_empty());
    let companion = autoscan(false).replace("\"DistanceFromArrivalLS\":0.0", "\"DistanceFromArrivalLS\":1520.4");
    assert!(r.spoken(&companion).is_empty());
}

#[test]
fn the_pledged_power_hears_undermining_and_acquisition_targets() {
    let mut r = Run::new();
    r.spoken(r#"{"timestamp":"2026-10-16T19:00:00Z","event":"Powerplay","Power":"Aisling Duval","Rank":10,"Merits":5000,"TimePledged":1000}"#);
    let enemy = jump(
        "Sys",
        1000,
        &[],
        r#","ControllingPower":"Zachary Hudson","Powers":["Zachary Hudson"],"PowerplayState":"Fortified","PowerplayStateControlProgress":0.4"#,
    );
    assert_eq!(r.spoken(&enemy), ["Undermining target."]);
    let unoccupied = jump(
        "Sys",
        1000,
        &[],
        r#","Powers":["Aisling Duval","Zachary Hudson"],"PowerplayState":"Unoccupied","PowerplayConflictProgress":[{"Power":"Aisling Duval","ConflictProgress":0.3}]"#,
    );
    assert_eq!(r.spoken(&unoccupied), ["Acquisition target."]);
    // Her own system: the normal case, quiet.
    let own = jump(
        "Sys",
        1000,
        &[],
        r#","ControllingPower":"Aisling Duval","Powers":["Aisling Duval"],"PowerplayState":"Stronghold""#,
    );
    assert!(r.spoken(&own).is_empty());
    // With a state line, the Powerplay line follows it.
    *r.shared.shopping.write().unwrap() = ShoppingList::wanting(&["protoheatradiators"]);
    let both = jump(
        "Sys",
        3_000_000,
        &[("Independent", "Boom", 0.5)],
        r#","ControllingPower":"Zachary Hudson","Powers":["Zachary Hudson"],"PowerplayState":"Exploited""#,
    );
    assert_eq!(r.spoken(&both), ["Boom here. Proto Heat Radiators. Undermining target."]);
    // Not pledged: no Powerplay line.
    let mut r = Run::new();
    assert!(r.spoken(&enemy).is_empty());
}

#[test]
fn star_class_security_and_allegiance_are_never_spoken() {
    let mut r = shopping(&["protoheatradiators", "militarygradealloys", "dataminedwake"]);
    r.spoken(r#"{"timestamp":"2026-10-16T19:00:00Z","event":"Powerplay","Power":"Aisling Duval","Rank":10}"#);
    let lines = [
        START_JUMP.to_string(),
        jump(
            "Sys",
            3_000_000,
            &[("Independent", "Boom", 0.5), ("Empire", "War", 0.3), ("Alliance", "Famine", 0.2)],
            r#","ControllingPower":"Zachary Hudson","Powers":["Zachary Hudson"],"PowerplayState":"Exploited""#,
        ),
        autoscan(false),
    ];
    let said = r.spoken(&lines.join("\n"));
    assert_eq!(said.len(), 2, "{said:?}");
    for l in &said {
        let low = l.to_lowercase();
        for banned in ["security", "class", "independent", "empire", "alliance", "federation", "industrial", "sys"] {
            assert!(!low.split(|c: char| !c.is_alphanumeric()).any(|w| w == banned), "{banned:?} in {l:?}");
        }
        assert!(!l.contains(" K ") && !l.starts_with("K "), "{l:?}");
    }
}

#[test]
fn the_briefing_is_silent_during_the_catch_up() {
    let shared = Shared::default();
    *shared.shopping.write().unwrap() = ShoppingList::wanting(&["protoheatradiators"]);
    let mut r = Run::with(shared, &Settings::default());
    r.engine.set_live(false);
    let lines = [jump("Sys", 3_000_000, &[("Independent", "Boom", 0.5)], ""), autoscan(false)];
    assert!(r.spoken(&lines.join("\n")).is_empty());
    r.engine.set_live(true);
    assert_eq!(r.spoken(&lines.join("\n")), ["Boom here. Proto Heat Radiators.", "Undiscovered system."]);
}

#[test]
fn a_material_collected_up_to_its_count_drops_off_the_list() {
    use astromech_announce::arrival::Want;
    let mut r = Run::new();
    *r.shared.shopping.write().unwrap() = ShoppingList(vec![Want { id: "protoheatradiators".into(), until: 5 }]);
    let held = |n: i64| {
        format!(
            r#"{{"timestamp":"2026-10-16T19:00:00Z","event":"Materials","Raw":[],"Manufactured":[{{"Name":"protoheatradiators","Count":{n}}}],"Encoded":[]}}"#
        )
    };
    let boom = jump("Sys", 3_000_000, &[("Independent", "Boom", 0.5)], "");
    r.spoken(&held(4));
    assert_eq!(r.spoken(&boom), ["Boom here. Proto Heat Radiators."]);
    // Collected in game, with the app closed: the list needs no update.
    r.spoken(&held(5));
    assert!(r.spoken(&boom).is_empty());
}

/// Arrivals recorded in a fixture history: (commander, system address, journal timestamp). It
/// answers as the ledger does: only arrivals strictly before the one being announced.
struct Ledger(Vec<(&'static str, i64, &'static str)>);

impl arrival::VisitHistory for Ledger {
    fn before(&self, commander: &str, address: i64, before: &str) -> Option<arrival::Past> {
        let mine: Vec<&str> =
            self.0.iter().filter(|(c, a, t)| *c == commander && *a == address && *t < before).map(|v| v.2).collect();
        Some(arrival::Past { visits: mine.len() as u32, last: mine.iter().max()?.to_string() })
    }
}

const LOAD_GAME: &str =
    r#"{"timestamp":"2026-10-16T19:00:00Z","event":"LoadGame","Commander":"Test","Ship":"krait_mkii","Credits":1}"#;

fn with_ledger(rows: Vec<(&'static str, i64, &'static str)>) -> Run {
    let r = Run::new();
    r.shared.visits.set(std::sync::Arc::new(Ledger(rows)));
    r
}

#[test]
fn a_system_visited_twice_before_says_so_and_when_last() {
    // The history already holds the arrival being announced (the follower imports it at once).
    let mut r = with_ledger(vec![
        ("Test", 1, "2026-10-01T10:00:00Z"),
        ("Test", 1, "2026-10-13T20:00:00Z"),
        ("Test", 1, "2026-10-16T20:00:00Z"),
    ]);
    r.spoken(LOAD_GAME);
    assert_eq!(r.spoken(&jump("Sys", 0, &[], "")), ["Visited 2 times. Last here 3 days ago."]);
}

#[test]
fn a_new_system_says_nothing_about_visits() {
    // Only this arrival is in the history; another commander's and another system's don't count.
    let mut r = with_ledger(vec![
        ("Test", 1, "2026-10-16T20:00:00Z"),
        ("Other", 1, "2026-10-01T10:00:00Z"),
        ("Test", 2, "2026-10-01T10:00:00Z"),
    ]);
    r.spoken(LOAD_GAME);
    assert!(r.spoken(&jump("Sys", 0, &[], "")).is_empty());
}

#[test]
fn the_visits_line_follows_the_state_line_within_the_word_budget() {
    let mut r = shopping(&["protoheatradiators"]);
    r.shared.visits.set(std::sync::Arc::new(Ledger(vec![("Test", 1, "2026-10-16T19:30:00Z")])));
    r.spoken(LOAD_GAME);
    let boom = jump("Sys", 3_000_000, &[("Independent", "Boom", 0.5)], "");
    assert_eq!(r.spoken(&boom), ["Boom here. Proto Heat Radiators.", "Visited once. Last here 30 minutes ago."]);
}

#[test]
fn the_visits_line_is_silent_in_the_catch_up_and_without_a_history() {
    let mut r = with_ledger(vec![("Test", 1, "2026-10-01T10:00:00Z")]);
    r.spoken(LOAD_GAME);
    r.engine.set_live(false);
    assert!(r.spoken(&jump("Sys", 0, &[], "")).is_empty());
    // No history plugged in (no journal folder, database not there yet): nothing, no error.
    let mut r = Run::new();
    r.spoken(LOAD_GAME);
    assert!(r.spoken(&jump("Sys", 0, &[], "")).is_empty());
}

#[test]
fn how_long_ago_reads_in_the_nearest_unit() {
    let past = |last: &str| arrival::Past { visits: 3, last: last.into() };
    let now = "2026-10-16T20:00:00Z";
    for (last, said) in [
        ("2026-10-16T19:59:40Z", "Visited 3 times. Last here moments ago."),
        ("2026-10-16T19:59:00Z", "Visited 3 times. Last here 1 minute ago."),
        ("2026-10-16T17:00:00Z", "Visited 3 times. Last here 3 hours ago."),
        ("2026-10-15T20:00:00Z", "Visited 3 times. Last here 1 day ago."),
        ("2026-06-16T20:00:00Z", "Visited 3 times. Last here 4 months ago."),
        ("2024-10-16T20:00:00Z", "Visited 3 times. Last here 2 years ago."),
    ] {
        assert_eq!(arrival::visits_line(&past(last), now).as_deref(), Some(said));
    }
    assert_eq!(arrival::visits_line(&arrival::Past { visits: 0, last: now.into() }, now), None);
}

/// Landings in a fixture history: (commander, system address, body, timestamp).
struct Landings(Vec<(&'static str, i64, &'static str, &'static str)>);

impl arrival::VisitHistory for Landings {
    fn before(&self, _: &str, _: i64, _: &str) -> Option<arrival::Past> {
        None
    }
    fn landings(&self, commander: &str, address: i64, before: &str) -> Vec<arrival::Landed> {
        let mut out: Vec<arrival::Landed> = vec![];
        for (c, a, body, t) in &self.0 {
            if *c != commander || *a != address || *t >= before {
                continue;
            }
            match out.iter_mut().find(|l| l.body == *body) {
                Some(l) => l.count += 1,
                None => out.push(arrival::Landed { body: body.to_string(), count: 1 }),
            }
        }
        out
    }
}

fn with_landings(rows: Vec<(&'static str, i64, &'static str, &'static str)>) -> Run {
    let r = Run::new();
    r.shared.visits.set(std::sync::Arc::new(Landings(rows)));
    r
}

#[test]
fn a_body_landed_on_before_is_named_with_how_often() {
    let mut r = with_landings(vec![
        ("Test", 1, "Sys A 2", "2026-10-01T10:00:00Z"),
        ("Test", 1, "Sys A 2", "2026-10-13T10:00:00Z"),
        // Another commander's landing, another system's, and one after this arrival.
        ("Other", 1, "Sys A 3", "2026-10-01T10:00:00Z"),
        ("Test", 2, "Else A 1", "2026-10-01T10:00:00Z"),
        ("Test", 1, "Sys A 4", "2026-10-16T21:00:00Z"),
    ]);
    r.spoken(LOAD_GAME);
    assert_eq!(r.spoken(&jump("Sys", 0, &[], "")), ["Landed on A 2 twice before."]);
    // Arriving by carrier is not news.
    let carrier = jump("Sys", 0, &[], "").replace(r#""event":"FSDJump""#, r#""event":"CarrierJump""#);
    assert!(r.spoken(&carrier).is_empty());
}

#[test]
fn several_bodies_are_counted_and_none_says_nothing() {
    let mut r = with_landings(vec![
        ("Test", 1, "Sys A 2", "2026-10-01T10:00:00Z"),
        ("Test", 1, "Sys B 1", "2026-10-02T10:00:00Z"),
        ("Test", 1, "Sys B 1 a", "2026-10-03T10:00:00Z"),
    ]);
    r.spoken(LOAD_GAME);
    assert_eq!(r.spoken(&jump("Sys", 0, &[], "")), ["Landed on 3 bodies here before."]);
    let mut r = with_landings(vec![]);
    r.spoken(LOAD_GAME);
    assert!(r.spoken(&jump("Sys", 0, &[], "")).is_empty());
}

#[test]
fn landings_lines() {
    let one = |body: &str, count| [arrival::Landed { body: body.into(), count }];
    assert_eq!(arrival::landings_line("Sys", &one("Sys A 2", 1)).as_deref(), Some("Landed on A 2 before."));
    assert_eq!(arrival::landings_line("Sys", &one("Sys A 2 a", 7)).as_deref(), Some("Landed on A 2 a 7 times before."));
    // A body not named after the system, and one whose name is too long to read out.
    assert_eq!(arrival::landings_line("Sol", &one("Earth", 2)).as_deref(), Some("Landed on Earth twice before."));
    assert_eq!(
        arrival::landings_line("Alpha Centauri", &one("Proxima Centauri b c d", 2)).as_deref(),
        Some("Landed on a body here twice before.")
    );
    assert_eq!(arrival::landings_line("Sys", &[]), None);
}
