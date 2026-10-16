//! Journal replay tests for the reaction engine: cooldown, per-rule toggle, silent catch-up,
//! quiet during docking and combat. Synthetic journal lines; the engine's clock is the
//! journal's own timestamps.

use astromech_announce::*;
use astromech_gamestate::GameState;
use astromech_journal::parse_line;
use astromech_status::{parse_status, FLAG_GEAR};
use std::time::{Duration, Instant};

/// Says "Cargo scan." on every `Scanned`.
struct ScanRule {
    priority: Priority,
    channels: Channels,
}
impl Rule for ScanRule {
    fn id(&self) -> &'static str {
        "scan"
    }
    fn cooldown(&self) -> Duration {
        Duration::from_secs(30)
    }
    fn react(&self, input: &Input, _: &GameState) -> Option<Callout> {
        match input {
            Input::Journal(l) if l.name == "Scanned" => {
                Some(Callout { text: "Cargo scan.".into(), priority: self.priority, channels: self.channels })
            }
            _ => None,
        }
    }
}

/// Urgent, on `Scanned` too: speaks through quiet.
struct UrgentRule;
impl Rule for UrgentRule {
    fn id(&self) -> &'static str {
        "urgent"
    }
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, _: &GameState) -> Option<Callout> {
        matches!(input, Input::Journal(l) if l.name == "Scanned").then(|| Callout {
            text: "Stolen goods aboard.".into(),
            priority: Priority::Urgent,
            channels: Channels::SPEECH,
        })
    }
}

/// Speaks the pad on `DockingGranted`.
struct PadRule;
impl Rule for PadRule {
    fn id(&self) -> &'static str {
        "pad"
    }
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, _: &GameState) -> Option<Callout> {
        matches!(input, Input::Journal(l) if l.name == "DockingGranted").then(|| Callout {
            text: "Pad seven.".into(),
            priority: Priority::Normal,
            channels: Channels::SPEECH,
        })
    }
}

/// "Gear down." when the gear flag turns on in Status.json.
struct GearRule;
impl Rule for GearRule {
    fn id(&self) -> &'static str {
        "gear"
    }
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, _: &GameState) -> Option<Callout> {
        match input {
            Input::Status { now, before } => {
                let was = before.is_some_and(|b| b.flags & FLAG_GEAR != 0);
                (now.flags & FLAG_GEAR != 0 && !was).then(|| Callout {
                    text: "Gear down.".into(),
                    priority: Priority::Normal,
                    channels: Channels::SPEECH,
                })
            }
            _ => None,
        }
    }
}

fn scan() -> Box<dyn Rule> {
    Box::new(ScanRule { priority: Priority::Normal, channels: Channels::SPEECH })
}

struct Replay {
    engine: Engine,
    state: GameState,
    t0: Instant,
    /// (rule, text, spoken?, toast?)
    out: Vec<(&'static str, String, bool, bool)>,
}

/// Seconds since midnight of a journal timestamp ("2026-10-16T20:00:05Z").
fn secs(ts: &str) -> u64 {
    let t = &ts[11..19];
    let p: Vec<u64> = t.split(':').map(|x| x.parse().unwrap()).collect();
    p[0] * 3600 + p[1] * 60 + p[2]
}

impl Replay {
    fn new(rules: Vec<Box<dyn Rule>>, settings: &Settings) -> Self {
        Self { engine: Engine::new(rules, settings), state: GameState::default(), t0: Instant::now(), out: vec![] }
    }

    fn at(&self, ts: &str) -> Instant {
        self.t0 + Duration::from_secs(secs(ts))
    }

    fn record(&mut self, rs: Vec<Reaction>) {
        self.out.extend(rs.into_iter().map(|r| (r.rule, r.text, r.channels.speech, r.channels.toast)));
    }

    fn journal(&mut self, lines: &str) {
        for l in lines.lines().map(str::trim).filter(|l| !l.is_empty()) {
            let j = parse_line(l).unwrap();
            self.state.apply_line(&j);
            let now = self.at(&j.timestamp);
            let rs = self.engine.journal(&j, &self.state, now);
            self.record(rs);
        }
    }

    fn status(&mut self, ts: &str, flags: u64) {
        let s = parse_status(format!(r#"{{"timestamp":"{ts}","event":"Status","Flags":{flags}}}"#).as_bytes()).unwrap();
        self.state.apply_status(&s);
        let now = self.at(ts);
        let rs = self.engine.status(&s, &self.state, now);
        self.record(rs);
    }

    fn spoken(&self) -> Vec<String> {
        self.out.iter().filter(|o| o.2).map(|o| o.1.clone()).collect()
    }
}

fn ev(ts: &str, event: &str) -> String {
    format!(r#"{{"timestamp":"2026-10-16T{ts}Z","event":"{event}"}}"#)
}

fn scanned(ts: &str) -> String {
    format!(r#"{{"timestamp":"2026-10-16T{ts}Z","event":"Scanned","ScanType":"Cargo"}}"#)
}

#[test]
fn a_rule_fires_once_per_event_within_its_cooldown() {
    let mut r = Replay::new(vec![scan()], &Settings::default());
    r.engine.set_live(true);
    // Three scans inside the 30 s cooldown, an unrelated event, then one after it.
    let lines =
        [scanned("20:00:00"), scanned("20:00:05"), ev("20:00:10", "Music"), scanned("20:00:29"), scanned("20:00:31")];
    r.journal(&lines.join("\n"));
    assert_eq!(r.spoken(), ["Cargo scan.", "Cargo scan."]);

    // A cooldown from the settings replaces the rule's own.
    let settings =
        Settings([("scan".to_string(), RuleSettings { cooldown_s: Some(2.0), ..Default::default() })].into());
    let mut r = Replay::new(vec![scan()], &settings);
    r.engine.set_live(true);
    r.journal(&[scanned("20:00:00"), scanned("20:00:01"), scanned("20:00:02")].join("\n"));
    assert_eq!(r.spoken().len(), 2);
}

#[test]
fn a_disabled_rule_never_fires() {
    let toml = "[scan]\nenabled = false\n";
    let settings: Settings = toml::from_str(toml).unwrap();
    let mut r = Replay::new(vec![scan(), Box::new(UrgentRule)], &settings);
    r.engine.set_live(true);
    let lines: Vec<String> = (0..10).map(|i| scanned(&format!("20:0{i}:00"))).collect();
    r.journal(&lines.join("\n"));
    assert!(r.out.iter().all(|o| o.0 != "scan"), "{:?}", r.out);
    // The other rule on the same events still fires.
    assert_eq!(r.out.iter().filter(|o| o.0 == "urgent").count(), 10);
    assert!(r.engine.setting_problems().is_empty());
}

#[test]
fn bad_settings_are_reported_not_fatal() {
    let toml = "[sacn]\nenabled = false\n[scan]\ncooldown_s = 1e20\n";
    let settings: Settings = toml::from_str(toml).unwrap();
    let mut r = Replay::new(vec![scan()], &settings);
    let problems = r.engine.setting_problems().to_vec();
    assert_eq!(problems.len(), 2, "{problems:?}");
    assert!(problems[0].contains("[callouts.scan] cooldown_s = 100000000000000000000") && problems[0].contains("30 s"));
    assert!(problems[1].contains("[callouts.sacn] names no callout"));
    // The rule keeps its own 30 s cooldown.
    r.engine.set_live(true);
    r.journal(&[scanned("20:00:00"), scanned("20:00:10"), scanned("20:00:31")].join("\n"));
    assert_eq!(r.spoken().len(), 2);
    // A negative cooldown is reported too.
    let neg = Settings([("scan".to_string(), RuleSettings { cooldown_s: Some(-5.0), ..Default::default() })].into());
    let e = Engine::new(vec![scan()], &neg);
    assert!(e.setting_problems()[0].contains("cooldown_s = -5"), "{:?}", e.setting_problems());
}

#[test]
fn nothing_is_spoken_during_the_journal_catch_up() {
    let mut r = Replay::new(vec![scan(), Box::new(GearRule)], &Settings::default());
    // Startup: the current journal from the top, then the first Status.json read.
    r.journal(
        &[ev("19:59:00", "Fileheader"), scanned("19:59:10"), ev("19:59:55", "Music"), scanned("19:59:57")].join("\n"),
    );
    r.status("2026-10-16T19:59:58Z", FLAG_GEAR);
    assert!(r.out.is_empty(), "{:?}", r.out);
    // The live point.
    r.engine.set_live(true);
    // The same Status.json again is not an event; a scan one second after the last replayed
    // one still fires (replay did not start the cooldown).
    r.status("2026-10-16T19:59:58Z", FLAG_GEAR);
    r.journal(&scanned("19:59:59"));
    r.status("2026-10-16T20:00:00Z", 0);
    r.status("2026-10-16T20:00:01Z", FLAG_GEAR);
    assert_eq!(r.spoken(), ["Cargo scan.", "Gear down."]);
}

#[test]
fn quiet_during_docking() {
    let both = Box::new(ScanRule { priority: Priority::High, channels: Channels::SPEECH | Channels::TOAST });
    let mut r = Replay::new(vec![both, Box::new(UrgentRule), Box::new(PadRule)], &Settings::default());
    r.engine.set_live(true);
    r.journal(&ev("20:00:00", "DockingRequested"));
    assert_eq!(r.engine.quiet(r.at("2026-10-16T20:00:00Z")), None);
    // The grant starts the approach; the grant itself is not "during docking", so its pad
    // line speaks.
    r.journal(&ev("20:00:01", "DockingGranted"));
    assert_eq!(r.engine.quiet(r.at("2026-10-16T20:00:01Z")), Some(Quiet::Docking));
    r.journal(&scanned("20:00:10"));
    assert_eq!(r.spoken(), ["Pad seven.", "Stolen goods aboard."]);
    // The non-urgent scan was shown, not spoken.
    assert!(r.out.iter().any(|o| o.0 == "scan" && !o.2 && o.3), "{:?}", r.out);
    r.out.clear();
    r.journal(&[ev("20:01:00", "Docked"), scanned("20:01:10")].join("\n"));
    assert_eq!(r.spoken(), ["Cargo scan.", "Stolen goods aboard."]);
    // A denial or timeout ends it too.
    r.journal(&[ev("20:02:00", "DockingGranted"), ev("20:02:01", "DockingTimeout")].join("\n"));
    assert_eq!(r.engine.quiet(r.at("2026-10-16T20:02:02Z")), None);
}

#[test]
fn quiet_during_combat() {
    use astromech_status::{FLAG_BEING_INTERDICTED, FLAG_IN_DANGER};
    let no_cooldown =
        Settings([("scan".to_string(), RuleSettings { cooldown_s: Some(0.0), ..Default::default() })].into());
    let mut r = Replay::new(vec![scan(), Box::new(UrgentRule)], &no_cooldown);
    r.engine.set_live(true);
    r.status("2026-10-16T20:00:00Z", FLAG_IN_DANGER);
    r.journal(&scanned("20:00:01"));
    assert_eq!(r.spoken(), ["Stolen goods aboard."]);
    r.out.clear();
    r.status("2026-10-16T20:00:05Z", FLAG_BEING_INTERDICTED);
    r.journal(&scanned("20:00:06"));
    assert_eq!(r.spoken(), ["Stolen goods aboard."]);
    r.out.clear();
    r.status("2026-10-16T20:00:10Z", 0);
    r.journal(&scanned("20:00:11"));
    assert_eq!(r.spoken(), ["Cargo scan.", "Stolen goods aboard."]);
    r.out.clear();
    // UnderAttack keeps it quiet for 30 s.
    r.journal(&[ev("20:01:00", "UnderAttack"), scanned("20:01:29")].join("\n"));
    assert_eq!(r.spoken(), ["Stolen goods aboard."]);
    r.out.clear();
    r.journal(&scanned("20:01:31"));
    assert_eq!(r.spoken(), ["Cargo scan.", "Stolen goods aboard."]);
}

#[test]
fn catch_up_leaves_docking_quiet_in_place() {
    let mut r = Replay::new(vec![scan()], &Settings::default());
    // Astromech started while the commander was on approach.
    r.journal(&[ev("19:59:00", "DockingRequested"), ev("19:59:01", "DockingGranted")].join("\n"));
    r.engine.set_live(true);
    r.journal(&scanned("20:00:00"));
    assert!(r.spoken().is_empty());
    // An UnderAttack in the catch-up is history: no combat quiet after it.
    let mut r = Replay::new(vec![scan()], &Settings::default());
    r.journal(&ev("19:59:59", "UnderAttack"));
    r.engine.set_live(true);
    r.journal(&scanned("20:00:00"));
    assert_eq!(r.spoken(), ["Cargo scan."]);
}

#[test]
fn every_builtin_rule_is_silent_during_the_catch_up() {
    let rules = builtin(Default::default());
    assert_eq!(rules.len(), 2, "a callout rule was added or removed: give it a line in this catch-up test");
    let mut r = Replay::new(rules, &Settings::default());
    let rock = r#"{"timestamp":"2026-10-16T19:59:00Z","event":"ProspectedAsteroid","Materials":[{"Name":"Platinum","Proportion":40.0}],"MotherlodeMaterial":"Opal","Remaining":100.0}"#;
    r.journal(&[rock.to_string(), scanned("19:59:10")].join("\n"));
    assert!(r.out.is_empty(), "{:?}", r.out);
    r.engine.set_live(true);
    r.journal(&[rock.replace("19:59:00", "20:00:00"), scanned("20:00:10")].join("\n"));
    assert_eq!(r.spoken(), ["Motherlode. Void Opals. Platinum 40%.", "Cargo scan."]);
}
