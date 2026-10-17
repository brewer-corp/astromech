//! Voice command tester (the page next to Diagnostics): show what a phrase would do without
//! doing it. A typed phrase, or a push-to-talk transcript while the page is open, goes through
//! the same grammar and tool planning as the assistant, and the result says which intent
//! matched, how sure the match is, what astromech would say and which keys it would send.
//!
//! [`dry_run`] is pure: it takes the grammar, a game-state snapshot and the binds check, none
//! of which can reach the policy or the executor, so a dry run cannot press anything. Pressing
//! is [`live`], a separate call the page makes only when the user asks for a live run.

use crate::actions::{Outcome, Runner, Verified};
use crate::assistant::{unavailable_reason, Assistant, Handled};
use crate::config::Config;
use crate::health::BindsState;
use astromech_gamestate::GameState;
use astromech_intent::{Decision, Grammar};
use astromech_keybinds::active::KeyCombo;
use astromech_policy::Wait;
use astromech_proto::Origin;
use astromech_tools::{elite_action, from_command, Plan};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Where the phrase came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Typed,
    Voice,
}

/// The matched intent.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Intent {
    /// Command id, e.g. "landing_gear.down".
    pub command: String,
    /// The grammar phrase that matched.
    pub phrase: String,
    /// Match score, 0..=1.
    pub score: f32,
    /// "execute", "ask" or "no_match" (thresholds in `astromech-intent`).
    pub decision: &'static str,
}

/// What the assistant would do with the phrase right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Would {
    /// Press the keys, then say `message`.
    Press,
    /// Ask before pressing (silent running), saying `message`.
    Confirm,
    /// Nothing to press: already in that state.
    Already,
    /// Not allowed in the current game state.
    Refused,
    /// A spoken answer, nothing pressed.
    Speak,
    /// The action has no keyboard key.
    NoBinding,
    /// Known command the app can't run yet.
    Unavailable,
    /// "Did you mean …?"
    DidYouMean,
    /// "Say again."
    NotUnderstood,
    /// A noise-only transcript: nothing said, nothing done.
    Ignored,
}

/// One tested phrase.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DryRun {
    pub source: Source,
    pub transcript: String,
    pub intent: Option<Intent>,
    pub would: Would,
    /// What astromech would say.
    pub message: String,
    /// The Elite action behind the command, if it presses one.
    pub action: Option<String>,
    /// The keyboard keys that action sends (modifiers first). Shown even when the plan
    /// presses nothing right now (e.g. the gear is already down).
    pub keys: Option<KeyCombo>,
    /// The command a live run would execute (None: nothing to run).
    pub live: Option<String>,
}

fn decision(d: Decision) -> &'static str {
    match d {
        Decision::Execute => "execute",
        Decision::Ask => "ask",
        Decision::NoMatch => "no_match",
    }
}

/// What `transcript` would do, without doing it. Mirrors [`Assistant::handle`] minus the
/// pending "Did you mean …?" state: each phrase is judged on its own.
pub fn dry_run(grammar: &Grammar, transcript: &str, game: &GameState, binds: &BindsState, source: Source) -> DryRun {
    let mut r = DryRun {
        source,
        transcript: transcript.to_string(),
        intent: None,
        would: Would::NotUnderstood,
        message: "Say again.".into(),
        action: None,
        keys: None,
        live: None,
    };
    if astromech_intent::is_noise_only(transcript) {
        r.would = Would::Ignored;
        r.message.clear();
        return r;
    }
    let m = grammar.best(transcript);
    if !m.command.is_empty() {
        r.intent = Some(Intent {
            command: m.command.to_string(),
            phrase: m.phrase.clone(),
            score: m.score,
            decision: decision(m.decision),
        });
    }
    if m.decision == Decision::NoMatch || m.command == "confirm" || m.command == "cancel" {
        return r;
    }
    if astromech_tools::lookup::is_lookup(m.command) {
        (r.would, r.message) =
            match (m.decision, astromech_tools::lookup::want(m.command, m.slot.as_deref(), m.slot_text.as_deref())) {
                (_, None) => (Would::NotUnderstood, "Didn't catch what to look for.".into()),
                (Decision::Ask, Some(_)) => (Would::DidYouMean, format!("Did you mean {}?", m.phrase)),
                (_, Some(w)) => {
                    (Would::Speak, format!("Would look up the nearest {} on Spansh and say it.", w.label()))
                }
            };
        return r;
    }
    if astromech_tools::session::is_summary(m.command) {
        (r.would, r.message) = match m.decision {
            Decision::Ask => (Would::DidYouMean, format!("Did you mean {}?", m.phrase)),
            _ => (Would::Speak, astromech_tools::session::describe()),
        };
        return r;
    }
    if let Some(target) = astromech_tools::targets::from_command(m.command) {
        (r.would, r.message) = match m.decision {
            Decision::Ask => (Would::DidYouMean, format!("Did you mean {}?", m.phrase)),
            _ => (Would::Speak, astromech_tools::targets::describe(target)),
        };
        return r;
    }
    if let Some(ask) = astromech_tools::trade::from_command(m.command) {
        // A lookup, not a press: say what it would look up, without asking Ardent.
        (r.would, r.message) = match m.decision {
            Decision::Ask => (Would::DidYouMean, format!("Did you mean {}?", m.phrase)),
            _ => (Would::Speak, astromech_tools::trade::describe(ask, m.slot.as_deref())),
        };
        return r;
    }
    let Some(call) = from_command(m.command) else {
        r.would = Would::Unavailable;
        r.message = unavailable_reason(m.command).into();
        return r;
    };
    r.live = Some(m.command.to_string());
    if let Some(a) = elite_action(call) {
        r.action = Some(a.to_string());
        r.keys = binds.keyboard(a);
    }
    if m.decision == Decision::Ask {
        r.would = Would::DidYouMean;
        r.message = format!("Did you mean {}?", m.phrase);
        return r;
    }
    (r.would, r.message) = match astromech_tools::plan_with(call, game) {
        Plan::Press { action, confirm, .. } => match &r.keys {
            Some(_) => (Would::Press, confirm),
            None => (Would::NoBinding, binds.missing_reason(action, astromech_tools::action_noun(call))),
        },
        Plan::Confirm { prompt, .. } => (Would::Confirm, prompt),
        Plan::Already(s) => (Would::Already, s),
        Plan::Refused(s) => (Would::Refused, s),
        Plan::Speak(s) => (Would::Speak, s),
        Plan::Lookup(kind) => {
            let noun = astromech_galaxy::site_kind(kind).map_or(kind, |k| k.noun);
            (Would::Speak, format!("Would look up the nearest {noun} and say where it is."))
        }
    };
    r
}

/// A typed phrase while Elite isn't running (no session): the binds check is run on demand
/// and there is no game state, so ship actions read as "not available" but still show their keys.
pub fn dry_run_idle(cfg: &Config, transcript: &str) -> DryRun {
    let mut grammar = crate::assistant::grammar();
    if let Ok(st) = astromech_store::Store::open(&cfg.data_dir().join("astromech.sqlite")) {
        crate::assistant::add_aliases(&mut grammar, &st);
    }
    let binds = match cfg.bindings_dir() {
        Some(b) => crate::health::run_check(&b, cfg.game_dir().as_deref(), false),
        None => BindsState::default(),
    };
    dry_run(&grammar, transcript, &GameState::default(), &binds, Source::Typed)
}

/// How long a live run waits for the user to switch from astromech to Elite.
pub const SWITCH_WAIT: Duration = Duration::from_secs(5);

/// Run a tested command for real, through the policy like any other command. The user clicked
/// Run live in astromech, so this first waits up to `wait` for Elite to be the active window
/// (presses are only ever sent to Elite in front). The user asked for it on the tester page, so
/// a spoken confirmation (silent running) or "Did you mean …?" counts as answered.
pub fn live(runner: &Runner, command: &str, wait: Duration) -> Result<String, String> {
    let call = from_command(command).ok_or_else(|| format!("{command} can't be run"))?;
    match runner.policy.wait_foreground(wait) {
        Wait::Front => {}
        Wait::TimedOut => return Ok("Elite didn't come to the front in time, so nothing was pressed.".into()),
        Wait::Killed => return Ok("The kill switch was used, so nothing was sent to the game.".into()),
    }
    Ok(match runner.run_confirmed(call, Origin::AppWindow) {
        Outcome::Sent(h) => match h.join() {
            Ok(Verified::Confirmed { ms }) => format!("Pressed; the game confirmed it in {ms} ms."),
            Ok(Verified::NotConfirmed) => "Pressed, but the game didn't show the change.".into(),
            Ok(Verified::Cancelled) => "Pressed, then the kill switch cancelled it.".into(),
            Ok(Verified::Unverified) => "Pressed (the game gives no signal to check it).".into(),
            Err(_) => "Pressed.".into(),
        },
        Outcome::Already(s) | Outcome::Refused(s) | Outcome::NoBinding(s) | Outcome::Spoken(s) => s,
        Outcome::NeedsConfirm(s) => s,
        Outcome::LookingUp(h) => h.join().unwrap_or_else(|_| "The lookup failed.".into()),
        Outcome::Denied(d) => d.spoken(),
        Outcome::PressFailed(e) => e.spoken(),
    })
}

/// How long test mode lasts after the page last checked in. The page polls every second, so
/// closing it (or the window) hands push-to-talk back to the assistant within this time.
pub const LEASE: Duration = Duration::from_secs(3);

/// Test mode: while the tester page is open, push-to-talk is dry-run and the result kept here
/// for the page to show.
pub struct Tester {
    armed_until: Mutex<Option<Instant>>,
    last: Mutex<Option<(u64, DryRun)>>,
    seq: AtomicU64,
}

impl Default for Tester {
    fn default() -> Self {
        Self::new()
    }
}

impl Tester {
    pub const fn new() -> Self {
        Self { armed_until: Mutex::new(None), last: Mutex::new(None), seq: AtomicU64::new(0) }
    }

    /// Extend test mode by [`LEASE`]. True if it was off.
    pub fn arm(&self, now: Instant) -> bool {
        let mut a = self.armed_until.lock().unwrap();
        let was = a.is_some_and(|t| now < t);
        *a = Some(now + LEASE);
        !was
    }

    pub fn disarm(&self) {
        *self.armed_until.lock().unwrap() = None;
    }

    pub fn armed(&self, now: Instant) -> bool {
        self.armed_until.lock().unwrap().is_some_and(|t| now < t)
    }

    /// Keep a push-to-talk result for the page; returns its sequence number (from 1).
    pub fn record(&self, r: DryRun) -> u64 {
        let n = self.seq.fetch_add(1, Ordering::SeqCst) + 1;
        *self.last.lock().unwrap() = Some((n, r));
        n
    }

    pub fn last(&self) -> Option<(u64, DryRun)> {
        self.last.lock().unwrap().clone()
    }
}

/// The process's tester (the control API arms it, the push-to-talk thread reads it).
pub static TESTER: Tester = Tester::new();

/// What a push-to-talk transcript did.
#[derive(Debug)]
pub enum Heard {
    /// Test mode: dry-run and kept for the tester page. Nothing pressed or said.
    Tested(DryRun),
    /// Normal mode: the assistant acted on it.
    Handled(Handled),
}

/// Route a push-to-talk transcript: to the tester while its page is open, else to the assistant.
pub fn on_heard(tester: &Tester, assistant: &Assistant, transcript: &str, now: Instant) -> Heard {
    if tester.armed(now) {
        let r = assistant.dry_run(transcript, Source::Voice);
        tester.record(r.clone());
        Heard::Tested(r)
    } else {
        Heard::Handled(assistant.handle(transcript, Origin::Voice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_status::{FLAG_GEAR, FLAG_IN_MAIN_SHIP};

    fn flying(flags: u64) -> GameState {
        GameState { status_flags: flags, status_seen: true, ..Default::default() }
    }

    #[test]
    fn noise_and_nonsense_have_no_keys() {
        let g = Grammar::mvp(&["Sol"]);
        let b = BindsState::default();
        let r = dry_run(&g, "Mm-hmm.", &flying(FLAG_IN_MAIN_SHIP), &b, Source::Voice);
        assert_eq!((r.would, r.intent, r.live), (Would::Ignored, None, None));
        let r = dry_run(&g, "banana bread", &flying(FLAG_IN_MAIN_SHIP), &b, Source::Typed);
        assert_eq!(r.would, Would::NotUnderstood);
        assert_eq!(r.live, None);
        assert_eq!(r.intent.unwrap().decision, "no_match");
    }

    #[test]
    fn unavailable_and_did_you_mean() {
        let g = Grammar::mvp(&["Sol"]);
        let b = BindsState::default();
        let r = dry_run(&g, "engage supercruise", &flying(FLAG_IN_MAIN_SHIP), &b, Source::Typed);
        assert_eq!((r.would, r.live.as_deref()), (Would::Unavailable, None));
        assert!(r.message.starts_with("Supercruise is not available yet"));
        let r = dry_run(&g, "You're down.", &flying(FLAG_IN_MAIN_SHIP), &b, Source::Voice);
        assert_eq!(r.would, Would::DidYouMean);
        assert_eq!(r.message, "Did you mean gear down?");
        assert_eq!(r.live.as_deref(), Some("landing_gear.down"));
        assert_eq!(r.action.as_deref(), Some("LandingGearToggle"));
    }

    #[test]
    fn lookups_say_what_they_would_look_up() {
        let g = crate::assistant::grammar();
        let b = BindsState::default();
        let r = dry_run(&g, "nearest interstellar factors", &GameState::default(), &b, Source::Typed);
        assert_eq!((r.would, r.live, r.keys), (Would::Speak, None, None));
        assert_eq!(r.message, "Would look up the nearest Interstellar Factors on Spansh and say it.");
        assert_eq!(r.intent.unwrap().command, "find.service");
        let r = dry_run(&g, "nearest interstellar factor", &GameState::default(), &b, Source::Typed);
        assert_eq!(r.would, Would::DidYouMean);
    }

    #[test]
    fn plan_reflects_game_state() {
        let g = Grammar::mvp(&["Sol"]);
        let b = BindsState::default();
        let r = dry_run(&g, "gear down", &flying(FLAG_IN_MAIN_SHIP | FLAG_GEAR), &b, Source::Typed);
        assert_eq!(r.would, Would::Already);
        let r = dry_run(&g, "gear down", &GameState::default(), &b, Source::Typed);
        assert_eq!(r.would, Would::Refused);
        let r = dry_run(&g, "silent running on", &flying(FLAG_IN_MAIN_SHIP), &b, Source::Typed);
        assert_eq!(r.would, Would::Confirm);
        // No binds read: a press plan becomes "no binding", with the reason it would say.
        let r = dry_run(&g, "gear down", &flying(FLAG_IN_MAIN_SHIP), &b, Source::Typed);
        assert_eq!(r.would, Would::NoBinding);
        assert!(r.message.contains("couldn't read your Elite keybindings"), "{}", r.message);
    }

    #[test]
    fn lease_expires() {
        let t = Tester::new();
        let now = Instant::now();
        assert!(!t.armed(now));
        assert!(t.arm(now));
        assert!(!t.arm(now + Duration::from_secs(1)), "already on");
        assert!(t.armed(now + Duration::from_secs(3)));
        assert!(!t.armed(now + Duration::from_secs(5)));
        t.arm(now);
        t.disarm();
        assert!(!t.armed(now));
    }
}
