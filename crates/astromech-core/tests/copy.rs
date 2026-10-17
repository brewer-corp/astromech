//! The callout copy guide (docs/V1-DESIGN.md §6.5), checked over every line astromech speaks
//! on its own or in reply to a command: word budgets (spoken words, after the reading rules),
//! no contractions, no "I".

use astromech_announce::{
    arrival, biology, builtin, discovery, docking, fuel, hge, missions, powerplay_arrival, project_market, prospector,
    route, sampling, scan, session_summary, signals,
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
            "biology",
            "discovery",
            "docking",
            "fuel_range",
            "hge",
            "materials_cap",
            "missions",
            "powerplay_arrival",
            "project_market",
            "prospector",
            "route",
            "sampling",
            "scan",
            "scoopable",
            "session_summary",
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
    for next in [2, 3] {
        v.push(sampling::line(next));
    }
    for n in [1, 2, 99] {
        v.push(biology::count_line(n, false));
        v.push(biology::count_line(n, true));
    }
    // Every genus and species, on short and long body names, exact and not.
    let labels = astromech_exobiology::catalog::genera()
        .iter()
        .flat_map(|g| std::iter::once(g.name.clone()).chain(g.species.iter().map(|s| s.name.clone())));
    for label in labels {
        for body in ["A 2", "12 d", "AB 1 c", "AB 1 c a"] {
            for exact in [false, true] {
                v.push(biology::body_line(body, &label, 16_777_215, exact));
                v.push(biology::body_line(body, &label, 1_000_000, exact));
            }
        }
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
    v.extend(discovery_lines());
    v.extend(session_summary_lines());
    v.sort();
    v.dedup();
    v
}

/// The discovery callout for every planet class, terraformable or not, from the smallest value
/// it can have to the largest (a heavy first-discovered body).
fn discovery_lines() -> Vec<String> {
    let classes = [
        "Metal rich body",
        "High metal content body",
        "Rocky body",
        "Icy body",
        "Rocky ice body",
        "Earthlike body",
        "Water world",
        "Ammonia world",
        "Water giant",
        "Water giant with life",
        "Gas giant with water based life",
        "Gas giant with ammonia based life",
        "Sudarsky class I gas giant",
        "Sudarsky class II gas giant",
        "Sudarsky class III gas giant",
        "Sudarsky class IV gas giant",
        "Sudarsky class V gas giant",
        "Helium rich gas giant",
        "Helium gas giant",
    ];
    let mut v = vec![];
    for class in classes {
        for tf in ["", "Terraformable"] {
            for mass in [0.001, 1.0, 5000.0] {
                for new in [false, true] {
                    let b = astromech_journal::ScanBody {
                        scan_type: "Detailed".into(),
                        planet_class: Some(class.into()),
                        terraform_state: Some(tf.into()),
                        mass_em: Some(mass),
                        was_discovered: Some(!new),
                        was_mapped: Some(!new),
                        ..Default::default()
                    };
                    v.extend(discovery::line(&b));
                }
            }
        }
    }
    v
}

/// The session summary, over the magnitudes credits, jumps and bounties are said in.
fn session_summary_lines() -> Vec<String> {
    let mut v = vec![];
    for total in [1, 9_999, 52_000, 999_999, 4_200_000, 1_234_567_890] {
        for per_hour in [None, Some(1), Some(1_900_000), Some(987_654_321)] {
            for jumps in [0, 1, 38, 9_999] {
                for bounties in [0, 1, 6, 1_234] {
                    let t = session_summary::Totals {
                        end: String::new(),
                        total,
                        per_hour,
                        jumps,
                        distance_ly: 123_456.7,
                        bounties,
                    };
                    v.extend(session_summary::lines(&t));
                }
            }
        }
    }
    v.extend(astromech_tools::session::answer(Ok(None)));
    v.extend(astromech_tools::session::answer(Err(session_summary::Unreadable)));
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

/// The colonisation answers by voice: progress, what is left and where to buy, for a project with
/// a long name, the largest amounts, cargo aboard and many loads, a sentence at a time within the
/// answer budget; and the lines said when there is no answer.
#[test]
fn the_colonisation_answers_keep_to_the_copy_guide() {
    use astromech_colonisation::shopping::{shopping_list, Hold};
    use astromech_colonisation::{Project, ProjectView, Resource};
    use astromech_market::list::{ListPlan, Need, StockLine, Stockist};
    use astromech_tools::colonisation as c;
    let now = 1_792_152_000; // 2026-10-16T12:00:00Z
    let resource = |name: &str, local: &str| Resource {
        name: name.into(),
        name_localised: Some(local.into()),
        required: 1_234_567,
        provided: 1,
        payment: 1,
    };
    let project = Project {
        market_id: 1,
        station_name: Some("Planetary Construction Site: Synuefe BY-F d12-39 Landing".into()),
        system_name: Some("Synuefe BY-F d12-39".into()),
        progress: 0.999,
        complete: false,
        failed: false,
        resources: vec![
            resource("$cmmcomposite_name;", "CMM Composite"),
            resource("$ceramiccomposites_name;", "Ceramic Composites"),
            resource("$insulatingmembrane_name;", "Insulating Membrane"),
        ],
        first_seen: String::new(),
        depot_updated: String::new(),
        contributions: vec![],
    };
    let mut lines = vec![c::NO_PROJECT.to_string(), c::NO_POSITION.to_string(), c::NO_STAR_MAP.to_string()];
    let mut split = |s: String| lines.extend(s.split_inclusive(". ").map(|s| s.trim().to_string()));
    for (capacity, aboard) in [(None, 0), (Some(1), 1), (Some(794), 999_999)] {
        let mut hold = Hold { capacity, ..Hold::default() };
        hold.cargo.insert("insulatingmembrane".into(), aboard);
        let v = ProjectView { shopping: shopping_list(&project, &hold), project: project.clone() };
        split(c::progress(&v));
        split(c::remaining(&v));
        for commodity in ["Insulating Membrane", "Low Temperature Diamonds", "nonsense"] {
            if let Err(why) = c::need(&v, commodity) {
                split(why);
            }
        }
    }
    let mut built = ProjectView { shopping: shopping_list(&project, &Hold::default()), project: project.clone() };
    built.project.resources.iter_mut().for_each(|r| r.provided = r.required);
    split(c::remaining(&built));
    let need = Need { symbol: "insulatingmembrane".into(), name: "Insulating Membrane".into(), tonnes: 1_234_566 };
    let stockist = Stockist {
        market_id: 1,
        station: "Carvalho Engineering Foundry".into(),
        system: "Synuefe BY-F d12-39".into(),
        system_pos: [0.0; 3],
        hop: astromech_galaxy::Hop { ly: 499.6, jumps: 99, exact: false },
        pad: None,
        arrival_ls: None,
        fleet_carrier: false,
        odyssey: false,
        updated_at: "2024-01-01T00:00:00Z".into(),
        stocked: 1,
        lines: vec![StockLine {
            commodity: "insulatingmembrane".into(),
            name: "Insulating Membrane".into(),
            needed: 1_234_566,
            stock: 9_999_999,
            price: 123_456,
        }],
        missing: vec![],
    };
    for stations in [vec![], vec![stockist]] {
        let plan = ListPlan { stations, unavailable: vec![], truncated: vec![], fetched_at: 0 };
        split(c::buy(&need, &plan, 500, now));
    }
    assert!(lines.iter().any(|l| l.starts_with("Most needed: ")), "{lines:?}");
    let bad = check("colonisation answer", &lines, ANSWER_WORDS);
    assert!(bad.is_empty(), "{} lines break the copy guide:\n{}", bad.len(), bad.join("\n"));
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
    // The session summary by voice says the same lines, one at a time.
    assert!(callouts.iter().any(|l| l == "Earned 4.2 million credits, 1.9 million an hour."), "{callouts:?}");
    let mut answers = answers;
    answers.push(astromech_tools::session::NOTHING_YET.to_string());
    let mut bad = check("callout", &callouts, CALLOUT_WORDS);
    bad.extend(check("reply", &replies, REPLY_WORDS));
    bad.extend(check("answer", &answers, ANSWER_WORDS));
    assert!(bad.is_empty(), "{} lines break the copy guide:\n{}", bad.len(), bad.join("\n"));
}

/// The Powerplay target answers by voice ("where can I undermine near me"): each kind, on a long
/// procedural name far away, controlled by a long-named power, a sentence at a time within the
/// answer budget; and the lines said when there is no answer.
#[test]
fn the_powerplay_target_answers_keep_to_the_copy_guide() {
    use astromech_galaxy::{Here, Hop, Target, TargetHit, Targets};
    use astromech_tools::targets;
    let mut lines: Vec<String> =
        [targets::NOT_PLEDGED, targets::NO_POSITION, targets::NO_STAR_MAP].iter().map(|s| s.to_string()).collect();
    for target in [Target::Acquire, Target::Reinforce, Target::Undermine] {
        let found = |hits| Targets {
            target,
            power: "Archon Delaine".into(),
            here: Here { system: "Synuefe BY-F d12-39".into(), id64: None, pos: [0.0; 3] },
            range: 20.0,
            within_ly: targets::WITHIN_LY,
            fetched: String::new(),
            hits,
            warning: None,
        };
        lines.push(targets::spoken(&found(vec![])));
        for (ly, exact) in [(0.0, true), (4.0, true), (199.6, false)] {
            for controller in [None, Some("Jerome Archer"), Some("Archon Delaine")] {
                for state in ["Unoccupied", "Exploited", "Fortified", "Stronghold"] {
                    let hit = TargetHit {
                        name: "Synuefe BY-F d12-39".into(),
                        id64: 1,
                        hop: Hop { ly, jumps: 99, exact },
                        controlling_power: controller.map(str::to_string),
                        state: Some(state.into()),
                        control_progress: controller.map(|_| 1.234),
                        reinforcement: Some(123_456),
                        undermining: Some(654_321),
                        acquisition: controller.is_none().then_some(0.999),
                        others: vec!["Aisling Duval".into(), "Zemina Torval".into()],
                        updated_at: None,
                    };
                    let said = targets::spoken(&found(vec![hit]));
                    lines.extend(said.split_inclusive(". ").map(|s| s.trim().to_string()));
                }
            }
        }
    }
    assert!(lines.iter().any(|l| l == "Jerome Archer stronghold, 123 percent."), "{lines:?}");
    let bad = check("Powerplay target answer", &lines, ANSWER_WORDS);
    assert!(bad.is_empty(), "{} lines break the copy guide:\n{}", bad.len(), bad.join("\n"));
}

/// A hold of five commodities no market takes whole: the cargo answer adds at most two
/// partial-buyer sentences, each within the answer budget, naming a station only when it is not
/// the one the answer already gave.
#[test]
fn the_cargo_answer_limits_its_partial_buyer_sentences() {
    use astromech_galaxy::Hop;
    use astromech_market::cargo::{CargoPlan, CommoditySale, Load, SaleLine, StationSale};
    use astromech_market::Offer;

    let hop = Hop { ly: 12.0, jumps: 1, exact: false };
    let offer = |market_id: i64, station: &str, price: i64, volume: i64| Offer {
        commodity: "x".into(),
        market_id,
        station: station.into(),
        station_type: None,
        system: "Sol".into(),
        system_pos: [0.0; 3],
        hop,
        price,
        volume,
        pad: None,
        arrival_ls: None,
        fleet_carrier: false,
        odyssey: false,
        updated_at: "2026-10-17T00:00:00Z".into(),
    };
    let names = ["Platinum", "Painite", "Osmium", "Gold", "Silver"];
    let load: Vec<Load> =
        names.iter().map(|n| Load { symbol: n.to_lowercase(), name: (*n).into(), tonnes: 100 }).collect();
    // The best station takes part of each; the other commodities' best partial buyers differ.
    let per_commodity: Vec<CommoditySale> = names
        .iter()
        .enumerate()
        .map(|(i, n)| CommoditySale {
            commodity: n.to_lowercase(),
            name: (*n).into(),
            carried: 100,
            best: None,
            partial: Some(if i % 2 == 0 {
                offer(1, "Good Landing Orbital Starport", 1000 * (i as i64 + 1), 70)
            } else {
                offer(2 + i as i64, "Carvalho Engineering Foundry", 1000 * (i as i64 + 1), 60)
            }),
        })
        .collect();
    let lines = names
        .iter()
        .map(|n| SaleLine {
            commodity: n.to_lowercase(),
            name: (*n).into(),
            carried: 100,
            sold: 70,
            price: 1000,
            proceeds: 70_000,
        })
        .collect();
    let best = StationSale {
        market_id: 1,
        station: "Good Landing Orbital Starport".into(),
        system: "Sol".into(),
        system_pos: [0.0; 3],
        hop,
        pad: None,
        arrival_ls: None,
        fleet_carrier: false,
        odyssey: false,
        updated_at: "2026-10-17T00:00:00Z".into(),
        total: 350_000,
        sold: 350,
        lines,
    };
    let plan = CargoPlan { load, kept: vec![], stations: vec![best], per_commodity, fetched_at: 0 };
    let now = chrono::DateTime::parse_from_rfc3339("2026-10-17T00:00:00Z").unwrap().timestamp();
    let said = astromech_tools::trade::cargo_answer(&plan, 40, now);

    let sentences: Vec<String> = said.split_inclusive(". ").map(|s| s.trim().to_string()).collect();
    let partial: Vec<&String> = sentences.iter().filter(|s| s.contains(": only ")).collect();
    assert_eq!(partial.len(), 2, "{said}");
    let bad = check("partial buyer", &partial.iter().map(|s| s.to_string()).collect::<Vec<_>>(), ANSWER_WORDS);
    assert!(bad.is_empty(), "{}\n{said}", bad.join("\n"));
    // The two worth the most are said (Silver and Gold); Silver's buyer is the answer's station.
    assert!(partial.iter().any(|s| s.starts_with("Silver") && s.contains("the same station")), "{said}");
    assert!(partial.iter().any(|s| s.starts_with("Gold") && s.contains("Carvalho Engineering Foundry")), "{said}");
    assert_eq!(said.matches("Good Landing Orbital Starport").count(), 1, "the station is not repeated: {said}");
}

/// The faction-state lookups by voice ("nearest famine system"): every lookup's answer, on a
/// long procedural name far away, with each kind of data age, a sentence at a time within the
/// answer budget; and the lines said when there is no answer.
#[test]
fn the_faction_lookup_answers_keep_to_the_copy_guide() {
    use astromech_galaxy::{Here, Hop, Nearby, NearbyHit};
    let now = 1_792_152_000; // 2026-10-16T12:00:00Z
    let here = Here { system: "Synuefe BY-F d12-39".into(), id64: None, pos: [0.0; 3] };
    let mut lines =
        vec![astromech_core::lookups::NO_POSITION.to_string(), astromech_core::lookups::NO_STAR_MAP.to_string()];
    for q in astromech_galaxy::QUERIES {
        let nearby = |hits| Nearby {
            query: q.clone(),
            here: here.clone(),
            range: 20.0,
            fetched: String::new(),
            hits,
            warning: None,
        };
        lines.push(astromech_tools::faction::spoken(&nearby(vec![]), now));
        for (ly, exact) in [(0.0, true), (4.0, true), (1999.6, false)] {
            for population in [0, 9_000_000_000] {
                for (updated_at, seen_on_visit) in
                    [("2026-10-16T11:00:00Z", false), ("2026-10-15T11:00:00Z", true), ("2024-01-01T00:00:00Z", false)]
                {
                    let hit = NearbyHit {
                        name: "Synuefe BY-F d12-39".into(),
                        id64: 1,
                        hop: Hop { ly, jumps: 99, exact },
                        population,
                        updated_at: Some(updated_at.into()),
                        factions: vec![],
                        seen_on_visit,
                        score: None,
                    };
                    let said = astromech_tools::faction::spoken(&nearby(vec![hit]), now);
                    lines.extend(said.split_inclusive(". ").map(|s| s.trim().to_string()));
                }
            }
        }
    }
    assert!(lines.iter().any(|l| l == "Seen on your visit yesterday."), "{lines:?}");
    assert!(lines.iter().any(|l| l == "Over a billion people."), "{lines:?}");
    let bad = check("faction answer", &lines, ANSWER_WORDS);
    assert!(bad.is_empty(), "{} lines break the copy guide:\n{}", bad.len(), bad.join("\n"));
}
