//! The callout copy guide (docs/V1-DESIGN.md §6.5), checked over every line astromech speaks
//! on its own or in reply to a command: word budgets (spoken words, after the reading rules),
//! no contractions, no "I".

use astromech_announce::{
    arrival, builtin, docking, fuel, hge, missions, powerplay_arrival, project_market, prospector, route, scan, signals,
};
use astromech_core::assistant::unavailable_reason;
use astromech_core::speech::fixed_phrases;
use astromech_gamestate::GameState;
use astromech_tools::{flags, from_command, plan_with, Plan};

/// Unprompted callouts (the reaction engine's rules).
const CALLOUT_WORDS: usize = 8;
/// Replies to a command: confirmations, failures, "already", refusals, the confirm prompt.
const REPLY_WORDS: usize = 10;
/// Answers to a status question.
const ANSWER_WORDS: usize = 16;

fn spoken_words(line: &str) -> usize {
    astromech_voice::speakable::normalise(line).split_whitespace().count()
}

/// Every line the callout rules can produce, including the longest combinations.
fn callout_lines() -> Vec<String> {
    let mut ids: Vec<&str> = builtin(Default::default()).iter().map(|r| r.id()).collect();
    ids.sort();
    assert_eq!(
        ids,
        [
            "arrival",
            "docking",
            "fuel_range",
            "hge",
            "materials_cap",
            "missions",
            "powerplay_arrival",
            "project_market",
            "prospector",
            "route",
            "scan",
            "scoopable",
            "signals"
        ],
        "a callout rule was added or removed: list its lines here (callout_lines) so the copy guide checks them"
    );
    let mut v = vec![];
    let t = prospector::Thresholds::default();
    let ores: Vec<&str> = prospector::ORES.iter().map(|o| o.symbol).collect();
    for a in &ores {
        v.extend(prospector::line(&[(a, 99.9)], None, &t));
        for b in &ores {
            v.extend(prospector::line(&[(a, 99.9), (b, 99.9)], None, &t));
            v.extend(prospector::line(&[(a, 99.9), (b, 99.9)], Some(a), &t));
            // A motherlode plus two other ores above threshold (only one is read).
            v.extend(prospector::line(&[(a, 99.9), (b, 99.9)], Some("opal"), &t));
        }
        v.extend(prospector::line(&[], Some(a), &t));
    }
    for kind in ["Cargo", "Crime", "Cabin", "Data", "Xeno"] {
        for stolen in [false, true] {
            v.push(scan::line(kind, stolen).0);
        }
    }
    v.extend(arrival_lines());
    for pad in 0..=46 {
        v.extend(docking::pad_line(pad, "Coriolis"));
    }
    for jumps in [None, Some(1), Some(2), Some(99)] {
        v.push(fuel::next_line(jumps));
        for here_scoopable in [None, Some(false), Some(true)] {
            let a = fuel::Ahead { here_scoopable, jumps, fuel_needed: Some(10.0), new_stretch: true };
            v.extend(fuel::range_line(&a, 1.0).map(str::to_string));
        }
    }
    for spec in astromech_materials::catalog::catalog() {
        v.push(format!("{} at 300/300.", spec.name));
        v.push(format!("Trader here. Trade down {}.", spec.name));
    }
    for (ty, name) in [
        ("USS", "$Warzone_TG_VeryHigh:#index=1;"),
        ("USS", "$Warzone_Powerplay_Med;"),
        ("Combat", "$Warzone_PointRace_Med:#index=1;"),
        ("$USS_Type_NonHuman;", "$USS_NonHumanSignal;"),
        ("$USS_Type_PowerEmissions;", "$USS_PowerEmissions;"),
        ("Titan", "Titan"),
        ("Megaship", "Ship"),
    ] {
        v.extend(signals::line(ty, Some(name)));
    }
    for n in [1, 2, 99] {
        v.push(project_market::line(n));
    }
    v.push(hge::line(None, &[]));
    for pool in ["fed_composites", "empire_shielding", "proto", "military", "improvised", "pharma"] {
        v.push(hge::line(Some(pool), &[]));
        for spec in astromech_materials::catalog::catalog() {
            v.push(hge::line(Some(pool), &[spec.id.to_string()]));
        }
    }
    for n in [1, 2, 99] {
        for level in [1, 2] {
            v.push(missions::expiry_line(n, level));
        }
    }
    v.extend([missions::target_line(false).to_string(), missions::target_line(true).to_string()]);
    for first in [false, true] {
        for to_end in [false, true] {
            v.push(route::gap_line(250, first, to_end));
        }
    }
    let hop = |c: &str, x: f64| astromech_journal::RouteHop {
        system: "Synuefe BY-F d12-39".into(),
        star_class: Some(c.into()),
        star_pos: Some([x, 0.0, 0.0]),
    };
    let long: Vec<_> = (0..300).map(|i| hop(if i % 2 == 0 { "N" } else { "DA" }, i as f64 * 61.7)).collect();
    v.push(route::summary_line(&long));
    v.push(route::summary_line(&[hop("K", 0.0), astromech_journal::RouteHop { star_pos: None, ..hop("K", 1.0) }]));
    for hops in [&long[..], &long[..3]] {
        v.extend(route::stars_line(hops, route::white_dwarf, "White dwarf", "white dwarfs"));
        v.extend(route::stars_line(hops, route::neutron, "Neutron star", "neutron stars"));
    }
    for reason in [
        "NoSpace",
        "TooLarge",
        "Hostile",
        "Offences",
        "Distance",
        "ActiveFighter",
        "NoReason",
        "RestrictedAccess",
        "DockOffline",
        "JumpImminent",
        "Unknown",
    ] {
        v.push(docking::denied_line(Some(reason)));
    }
    v.sort();
    v.dedup();
    v
}

/// The arrival briefing: every state with every material it can name, alone and followed by
/// each Powerplay line; the undiscovered line.
fn arrival_lines() -> Vec<String> {
    use astromech_journal::FactionSnap;
    use astromech_powerplay::{PowerShare, SystemPower};
    let faction = |allegiance: &str, state: &str| FactionSnap {
        name: "F".into(),
        allegiance: allegiance.into(),
        state: state.into(),
        influence: 0.5,
        active_states: vec![state.into()],
    };
    let states = [
        faction("Independent", "Boom"),
        faction("Empire", "War"),
        faction("Empire", "CivilWar"),
        faction("Alliance", "CivilUnrest"),
        faction("Alliance", "Outbreak"),
        faction("Federation", "Famine"),
        faction("Federation", "None"),
        faction("Empire", "None"),
    ];
    let enemy = SystemPower {
        system: "S".into(),
        controlling_power: Some("Zachary Hudson".into()),
        state: Some("Fortified".into()),
        control_progress: Some(0.5),
        reinforcement: Some(10),
        undermining: Some(20),
        contesting: vec![],
        acquisition: vec![],
    };
    let unoccupied = SystemPower {
        controlling_power: None,
        acquisition: vec![PowerShare { power: "Aisling Duval".into(), progress: 0.3 }],
        ..enemy.clone()
    };
    let mut v = vec![arrival::UNDISCOVERED.to_string()];
    // Been here before: every unit of "ago", with the most digits each count and gap can have.
    for visits in [1, 2, 99_999] {
        for gap in [0, 90, 7_200, 200_000, 5_000_000, 90_000_000] {
            let past = arrival::Past { visits, last: "2026-01-01T00:00:00Z".into() };
            let now = chrono::DateTime::from_timestamp(1_767_225_600 + gap, 0).unwrap().to_rfc3339();
            v.extend(arrival::visits_line(&past, &now));
        }
    }
    for spec in astromech_materials::catalog::catalog() {
        let wanted = vec![spec.id.to_string()];
        for f in &states {
            for system in [None, Some(&enemy), Some(&unoccupied)] {
                v.extend(arrival::line(std::slice::from_ref(f), 2_000_000_000, &wanted, system, Some("Aisling Duval")));
            }
        }
    }
    v.extend(powerplay_arrival_lines());
    v
}

/// The Powerplay arrival callout: `arrival_line` a sentence at a time, for every kind of system
/// (controlled with the most powers contesting, unoccupied with every power acquiring) and
/// every standing of the commander.
fn powerplay_arrival_lines() -> Vec<String> {
    use astromech_powerplay::{PowerShare, SystemPower};
    const POWERS: [&str; 3] = ["Zachary Hudson", "Aisling Duval", "Archon Delaine"];
    let controlled = SystemPower {
        system: "S".into(),
        controlling_power: Some("Felicia Winters".into()),
        state: Some("Fortified".into()),
        control_progress: Some(0.5),
        reinforcement: Some(1234),
        undermining: Some(5678),
        contesting: POWERS.iter().map(|p| p.to_string()).collect(),
        acquisition: vec![],
    };
    let unoccupied = SystemPower {
        controlling_power: None,
        state: None,
        contesting: vec![],
        acquisition: POWERS.iter().map(|p| PowerShare { power: p.to_string(), progress: 0.3 }).collect(),
        ..controlled.clone()
    };
    let mut v = vec![];
    for system in [&controlled, &unoccupied] {
        for pledged in [None, Some("Felicia Winters"), Some("Aisling Duval"), Some("Pranav Antal")] {
            v.extend(powerplay_arrival::sentences(&system.spoken(pledged)));
        }
    }
    v
}

/// Game states that reach every branch of the replies and answers.
fn states() -> Vec<GameState> {
    let seen = |f: u64| GameState { status_flags: f, status_seen: true, ..Default::default() };
    let all_on = flags::IN_MAIN_SHIP
        | flags::FLAG_GEAR
        | flags::FLAG_LIGHTS
        | flags::HARDPOINTS
        | flags::CARGO_SCOOP
        | flags::FLAG_NIGHT_VISION
        | flags::FLAG_SILENT_RUNNING
        | flags::FLAG_HUD_ANALYSIS
        | flags::FLAG_SHIELDS_UP;
    let mut rich = seen(flags::IN_MAIN_SHIP | flags::FLAG_SUPERCRUISE | flags::FLAG_SHIELDS_UP);
    rich.body = Some("Synuefe BY-F d12-39 A 1".into());
    rich.system = Some("Synuefe BY-F d12-39".into());
    rich.station = Some("Jameson Memorial".into());
    rich.fuel_main = Some(12.34);
    rich.fuel_reservoir = Some(0.63);
    rich.fuel_capacity_main = Some(32.0);
    rich.cargo_tonnes = Some(120.0);
    rich.cargo_capacity = Some(256);
    rich.cargo_items =
        vec![("Low Temperature Diamonds".into(), 64), ("Void Opals".into(), 40), ("Platinum".into(), 16)];
    rich.max_jump_range = Some(48.3);
    rich.route = vec!["Sol".into(), "Col 285 Sector AB-C d14-123".into(), "HIP 12345".into()];
    rich.hull_health = Some(0.8);
    rich.credits = Some(1_234_567_890);
    rich.rebuy = Some(98_765_432);
    rich.ship = Some("Imperial Cutter".into());
    rich.ship_name = Some("Long Name Of The Ship".into());
    rich.ship_ident = Some("AB-12C".into());
    rich.legal_state = Some("Clean".into());
    vec![
        GameState::default(),
        seen(0),
        seen(flags::IN_MAIN_SHIP),
        seen(all_on),
        seen(flags::IN_MAIN_SHIP | flags::FLAG_FLIGHT_ASSIST_OFF),
        seen(flags::IN_SRV | flags::FLAG_SRV_HANDBRAKE | flags::FLAG_SRV_DRIVE_ASSIST),
        seen(flags::IN_SRV),
        GameState { gui_focus: Some(flags::GUI_GALAXY), ..seen(flags::IN_MAIN_SHIP) },
        rich,
    ]
}

/// (replies, answers) for every command the grammar can run, in every state above.
fn reply_and_answer_lines() -> (Vec<String>, Vec<String>) {
    let (mut replies, mut answers) = (fixed_phrases(), vec![]);
    for (id, _) in astromech_intent::MVP {
        if astromech_intent::UNAVAILABLE.contains(id) {
            replies.push(unavailable_reason(id).to_string());
        }
        let Some(call) = from_command(id) else { continue };
        for g in states() {
            match plan_with(call, &g) {
                Plan::Press { confirm, failure, .. } => replies.extend([confirm, failure]),
                Plan::Confirm { prompt, confirm, failure, .. } => replies.extend([prompt, confirm, failure]),
                Plan::Already(s) | Plan::Refused(s) => replies.push(s),
                Plan::Speak(s) => answers.push(s),
                // The lookup's answer is built from live data; what the plan says up front is fixed.
                Plan::Lookup(_) => replies.push("Looking that up.".to_string()),
            }
        }
    }
    for v in [&mut replies, &mut answers] {
        v.sort();
        v.dedup();
    }
    (replies, answers)
}

fn check(kind: &str, lines: &[String], budget: usize) -> Vec<String> {
    let mut bad = vec![];
    for l in lines {
        let n = spoken_words(l);
        if n > budget {
            bad.push(format!("{kind} over {budget} words ({n}): {l:?}"));
        }
        if l.contains('\'') {
            bad.push(format!("{kind} has a contraction or apostrophe: {l:?}"));
        }
        if l.split(|c: char| !c.is_alphanumeric()).any(|w| w == "I") {
            bad.push(format!("{kind} says \"I\": {l:?}"));
        }
        if !l.ends_with(['.', '?']) {
            bad.push(format!("{kind} is not a sentence: {l:?}"));
        }
    }
    bad
}

#[test]
fn every_line_keeps_to_the_copy_guide() {
    let callouts = callout_lines();
    let (replies, answers) = reply_and_answer_lines();
    // The enumeration really reaches the rules and the tools.
    assert!(callouts.iter().any(|l| l == "Motherlode. Void Opals."), "{callouts:?}");
    assert!(callouts.iter().any(|l| l == "Cargo scan. Stolen goods aboard."));
    assert!(callouts.iter().any(|l| l == "Boom here. Proto Heat Radiators. Undermining target."), "{callouts:?}");
    assert!(callouts.iter().any(|l| l == "Pad at eleven, middle."));
    assert!(callouts.iter().any(|l| l == "Scoop here. Next scoopable star out of reach."));
    assert!(callouts.iter().any(|l| l == "Next scoopable star in 99 jumps."));
    assert!(callouts.iter().any(|l| l == "150 white dwarfs, first at jump 1."), "{callouts:?}");
    assert!(callouts.iter().any(|l| l == "Fuel gap: 250 jumps between scoopable stars."));
    assert!(callouts.iter().any(|l| l == "High Grade Emissions. Federation composites."));
    assert!(callouts.iter().any(|l| l == "Good for Core Dynamics Composites. Undermining target."), "{callouts:?}");
    assert!(callouts.iter().any(|l| l == "Docking denied. Carrier about to jump."));
    assert!(callouts.iter().any(|l| l == "Reinforcement 1234, undermining 5678."), "{callouts:?}");
    assert!(callouts.iter().any(|l| l == "Undermining target."));
    assert!(callouts.iter().any(|l| l == "Famine here. Datamined Wake Exceptions. Acquisition target."));
    assert!(replies.iter().any(|l| l == "Landing gear deployed."), "{replies:?}");
    assert!(replies.iter().any(|l| l.contains("already")));
    assert!(replies.iter().any(|l| l.starts_with("Supercruise is not available")), "{replies:?}");
    assert!(answers.iter().any(|l| l.starts_with("Fuel:")), "{answers:?}");
    let mut bad = check("callout", &callouts, CALLOUT_WORDS);
    bad.extend(check("reply", &replies, REPLY_WORDS));
    bad.extend(check("answer", &answers, ANSWER_WORDS));
    assert!(bad.is_empty(), "{} lines break the copy guide:\n{}", bad.len(), bad.join("\n"));
}
