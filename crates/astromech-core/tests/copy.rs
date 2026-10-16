//! The callout copy guide (docs/V1-DESIGN.md §6.5), checked over every line astromech speaks
//! on its own or in reply to a command: word budgets (spoken words, after the reading rules),
//! no contractions, no "I".

use astromech_announce::{builtin, prospector, scan};
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
        ["prospector", "scan"],
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
    v.sort();
    v.dedup();
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
    assert!(replies.iter().any(|l| l == "Landing gear deployed."), "{replies:?}");
    assert!(replies.iter().any(|l| l.contains("already")));
    assert!(replies.iter().any(|l| l.starts_with("Supercruise is not available")), "{replies:?}");
    assert!(answers.iter().any(|l| l.starts_with("Fuel:")), "{answers:?}");
    let mut bad = check("callout", &callouts, CALLOUT_WORDS);
    bad.extend(check("reply", &replies, REPLY_WORDS));
    bad.extend(check("answer", &answers, ANSWER_WORDS));
    assert!(bad.is_empty(), "{} lines break the copy guide:\n{}", bad.len(), bad.join("\n"));
}
