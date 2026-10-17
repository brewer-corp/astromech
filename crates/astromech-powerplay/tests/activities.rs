//! What earns merits here: replays a synthetic session (`fixtures/activities`, pledged to Li
//! Yong-Rui) with one system per state in the rules table and checks the suggestions the page
//! shows at each stop, in order (the power's ethos and the controller's weak spots first).

use astromech_journal::{parse_line, JournalEvent};
use astromech_powerplay::{Journal, Suggestion};
use std::path::PathBuf;

/// (system, suggestions) at each jump or location.
fn stops() -> Vec<(String, Vec<Suggestion>)> {
    let p =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/activities/Journal.2026-10-16T200000.01.log");
    let mut j = Journal::default();
    let mut out = vec![];
    for l in std::fs::read_to_string(p).unwrap().lines().map(|l| parse_line(l).unwrap()) {
        j.apply(&l);
        if matches!(l.event, JournalEvent::FsdJump { .. } | JournalEvent::Location { .. }) {
            let o = j.overview();
            out.push((o.here.unwrap(), o.system.map(|s| s.suggestions).unwrap_or_default()));
        }
    }
    out
}

fn ids(s: &[Suggestion]) -> Vec<&str> {
    s.iter().map(|s| s.activity.as_str()).collect()
}

fn find<'a>(s: &'a [Suggestion], id: &str) -> Vec<&'a Suggestion> {
    s.iter().filter(|s| s.activity == id).collect()
}

/// Li Yong-Rui's ethos activities when reinforcing come first.
const REINFORCE: [&str; 16] = [
    "exobiology",
    "cartography",
    "sell-for-profit",
    "sell-rares",
    "political-data",
    "bounty-hunting",
    "support-missions",
    "salvage",
    "holoscreen-hacking",
    "power-kills",
    "reboot-missions",
    "scan-datalinks",
    "scan-ships",
    "sell-mined",
    "classified-data",
    "power-commodities",
];

const ACQUIRE: [&str; 14] = [
    "holoscreen-hacking",
    "sell-for-profit",
    "sell-rares",
    "political-data",
    "bounty-hunting",
    "power-kills",
    "retrieve-goods",
    "scan-datalinks",
    "sell-mined",
    "classified-data",
    "research-data",
    "power-commodities",
    "upload-malware",
    "reboot-missions",
];

#[test]
fn each_state_in_the_rules_table_gets_its_suggestions() {
    let stops = stops();
    let names: Vec<&str> = stops.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        names,
        [
            "Test Own Stronghold",
            "Test Own Fortified",
            "Test Own Exploited",
            "Test Acquisition",
            "Test Conflict",
            "Test Enemy Exploited",
            "Test Enemy Fortified",
            "Test Enemy Stronghold",
            "Test Other Acquisition",
            "Test Empty",
        ]
    );
    let at = |i: usize| ids(&stops[i].1);

    // Reinforce, Stronghold: the general rules, then picking up all three commodities here.
    assert_eq!(at(0), [&REINFORCE[..], &["pick-up-commodities"]].concat());
    // Reinforce, Fortified: picking up acquisition goods here too.
    assert_eq!(at(1), [&REINFORCE[..], &["pick-up-commodities"]].concat());
    // Reinforce, Exploited: nothing to pick up here.
    assert_eq!(at(2), REINFORCE);

    // Acquire, below the conflict threshold.
    assert_eq!(at(3), ACQUIRE);
    // Acquire in conflict: the conflict activities are added.
    assert_eq!(at(4), [&ACQUIRE[..], &["support-missions", "flood-markets", "scan-ships"]].concat());

    // Undermine Pranav Antal (weak to classified data and flooding markets).
    assert_eq!(
        at(5),
        [
            "commit-crimes",
            "flood-markets",
            "holoscreen-hacking",
            "classified-data",
            "political-data",
            "research-data",
            "support-missions",
            "salvage",
            "power-kills",
            "retrieve-goods",
            "scan-datalinks",
            "sell-mined",
            "power-commodities",
            "upload-malware",
        ]
    );
    // Undermine Zemina Torval, fortified (weak to retrieving goods and mining).
    assert_eq!(
        at(6),
        [
            "commit-crimes",
            "flood-markets",
            "holoscreen-hacking",
            "retrieve-goods",
            "sell-mined",
            "political-data",
            "research-data",
            "support-missions",
            "salvage",
            "power-kills",
            "scan-datalinks",
            "classified-data",
            "power-commodities",
            "upload-malware",
        ]
    );
    // Undermine Archon Delaine's stronghold: the stronghold carrier warning first (while a
    // carrier is here most undermining earns nothing), and bounty hunting counts against him.
    assert_eq!(
        at(7),
        [
            "stronghold-carrier",
            "commit-crimes",
            "flood-markets",
            "holoscreen-hacking",
            "power-kills",
            "political-data",
            "research-data",
            "bounty-hunting",
            "support-missions",
            "salvage",
            "retrieve-goods",
            "scan-datalinks",
            "sell-mined",
            "classified-data",
            "power-commodities",
            "upload-malware",
        ]
    );

    // Another power's acquisition, and no Powerplay at all: nothing.
    assert!(stops[8].1.is_empty());
    assert!(stops[9].1.is_empty());
}

#[test]
fn suggestions_carry_the_details_the_page_shows() {
    let stops = stops();
    // The pledged power's commodities are named.
    assert!(find(&stops[1].1, "power-commodities")[0].how.contains("Sirius Industrial Equipment"));
    let pickup = find(&stops[1].1, "pick-up-commodities")[0];
    assert!(pickup.how.contains("Sirius Franchise Package") && pickup.how.contains("20 ly"), "{}", pickup.how);
    assert_eq!(pickup.name, "Pick up power commodities");
    let stronghold = find(&stops[0].1, "pick-up-commodities")[0];
    assert!(stronghold.how.contains("Sirius Corporate Contracts") && stronghold.how.contains("30 ly"));
    assert!(find(&stops[5].1, "power-commodities")[0].how.contains("Sirius Corporate Contracts"));
    assert!(find(&stops[3].1, "power-commodities")[0].how.contains("Sirius Franchise Package"));
    assert!(stops.iter().flat_map(|s| &s.1).all(|s| !s.how.contains('{')));

    // Flags: ethos, weak spot (undermining only), on foot, unverified.
    let crimes = find(&stops[5].1, "commit-crimes")[0];
    assert!(crimes.favoured && !crimes.weak_spot && !crimes.odyssey);
    let classified = find(&stops[5].1, "classified-data")[0];
    assert!(classified.weak_spot && !classified.favoured && classified.odyssey);
    assert!(stops[..5].iter().flat_map(|s| &s.1).all(|s| !s.weak_spot));
    let carrier = find(&stops[7].1, "stronghold-carrier")[0];
    assert!(carrier.uncertain.as_deref().is_some_and(|u| u.contains("carrier")));
    assert!(find(&stops[0].1, "cartography")[0].uncertain.is_none());
    // Conflict activities carry the assumed conflict threshold.
    assert!(find(&stops[4].1, "scan-ships")[0].uncertain.as_deref().is_some_and(|u| u.contains("0.3")));
    // Bounty hunting against Delaine is a weak spot, not an ethos activity.
    let bounty = find(&stops[7].1, "bounty-hunting")[0];
    assert!(bounty.weak_spot && !bounty.favoured);

    // Suspended activities (escape pods) are never suggested.
    assert!(stops.iter().flat_map(|s| &s.1).all(|s| s.activity != "escape-pods"));
}

#[test]
fn nothing_is_suggested_before_pledging() {
    let mut j = Journal::default();
    let line = r#"{ "timestamp":"2026-10-16T20:10:00Z", "event":"FSDJump", "StarSystem":"Test Own Fortified", "JumpDist":5.0, "ControllingPower":"Li Yong-Rui", "Powers":["Li Yong-Rui"], "PowerplayState":"Fortified", "PowerplayStateControlProgress":0.28 }"#;
    j.apply(&parse_line(line).unwrap());
    let s = j.overview().system.unwrap();
    assert!(s.suggestions.is_empty());
    assert_eq!(s.activity_rules.fetched, "2026-10-16");
}
