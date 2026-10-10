//! S7: does controller/keyboard input reach us while Elite is focused, without taking it
//! away from the game? Learn PTT + kill switch, then run cued trials with the game in front.
//!
//! This module only *listens*. It never sends key presses or any other input.

use crate::input::{ignored_for_learning, is_escape, Binding, BoundControl, InputEvent, Source};
use s79_common::stats::{summarize, Summary};
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cue {
    Tick,
    Start,
    Ptt,
    Kill,
    Phase,
    Done,
    Error,
}

/// Status.json fields that change when the player does something.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusSnap {
    pub key: String,
}

pub trait Env {
    fn recv(&self, timeout: Duration) -> Option<InputEvent>;
    fn cue(&self, c: Cue);
    fn foreground(&self) -> String;
    fn status(&self) -> Option<StatusSnap>;
    fn out(&self, s: &str);
    /// Ask a question on the console; returns the trimmed lowercase answer.
    fn ask(&self, q: &str) -> String;
}

pub fn ask_yn(env: &dyn Env, q: &str, default: bool) -> bool {
    loop {
        let a = env.ask(&format!("{q} [{}] ", if default { "Y/n" } else { "y/N" }));
        match a.as_str() {
            "" => return default,
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => env.out("  please answer y or n"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct S7Config {
    pub ptt_count: usize,
    pub kill_count: usize,
    pub press_timeout: Duration,
    pub ptt_release_timeout: Duration,
    pub kill_release_timeout: Duration,
    pub gap_min: Duration,
    pub gap_max: Duration,
    pub countdown_secs: u64,
    pub focus_wait: Duration,
    pub require_game: bool,
    pub game_title: String,
    pub receive_check: Duration,
    pub receive_check_presses: usize,
    pub learn_timeout: Duration,
}

impl Default for S7Config {
    fn default() -> Self {
        S7Config {
            ptt_count: 20,
            kill_count: 5,
            press_timeout: Duration::from_secs(5),
            ptt_release_timeout: Duration::from_secs(10),
            kill_release_timeout: Duration::from_secs(3),
            gap_min: Duration::from_millis(1500),
            gap_max: Duration::from_millis(3000),
            countdown_secs: 10,
            focus_wait: Duration::from_secs(60),
            require_game: true,
            game_title: "Elite - Dangerous (CLIENT)".into(),
            receive_check: Duration::from_secs(25),
            receive_check_presses: 3,
            learn_timeout: Duration::from_secs(60),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LearnRecord {
    pub what: String,
    pub binding: Option<Binding>,
    /// Sources that reported the same press (e.g. ["raw_hid", "x_input"] for an Xbox pad).
    pub seen_by: Vec<Source>,
    pub released_seen: bool,
    pub injected: bool,
    pub note: Option<String>,
}

/// Wait for one new button press and turn it into a binding.
pub fn learn(env: &dyn Env, cfg: &S7Config, what: &str, exclude: Option<&Binding>) -> LearnRecord {
    let mut rec = LearnRecord { what: what.into(), binding: None, seen_by: vec![], released_seen: false, injected: false, note: None };
    // Discard anything already queued (e.g. the Enter release from the last prompt).
    let settle = Instant::now() + Duration::from_millis(300);
    while Instant::now() < settle {
        env.recv(Duration::from_millis(20));
    }
    env.out(&format!(
        "  Press and release the button you want as {what} (keyboard, mouse buttons 3-5, joystick, HOTAS or gamepad).\n  Esc = skip. Waiting up to {} s...",
        cfg.learn_timeout.as_secs()
    ));
    let deadline = Instant::now() + cfg.learn_timeout;
    let first = loop {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            rec.note = Some("no button press seen before the timeout".into());
            return rec;
        }
        let Some(ev) = env.recv(left) else { continue };
        if !ev.down {
            continue;
        }
        if is_escape(&ev.control) {
            rec.note = Some("skipped with Esc".into());
            return rec;
        }
        if ignored_for_learning(&ev.control) {
            continue;
        }
        let candidate = Binding { primary: bound(&ev), alternate: None };
        if exclude.map(|x| x.same_as(&candidate)).unwrap_or(false) {
            env.out("  That button is already the PTT button - please pick a different one.");
            continue;
        }
        break ev;
    };
    rec.injected = first.injected;
    rec.seen_by.push(first.device.source);
    let mut binding = Binding { primary: bound(&first), alternate: None };
    // The same physical press may show up through the other API shortly after.
    let other_window = Instant::now() + Duration::from_millis(400);
    let release_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let now = Instant::now();
        let until = if rec.released_seen { other_window } else { release_deadline.max(other_window) };
        if now >= until {
            break;
        }
        let Some(ev) = env.recv(until - now) else { continue };
        if binding.primary.matches(&ev) {
            if !ev.down {
                rec.released_seen = true;
            }
            continue;
        }
        if ev.down && now < other_window && binding.alternate.is_none() && is_cross_api(first.device.source, ev.device.source) {
            binding.alternate = Some(bound(&ev));
            rec.seen_by.push(ev.device.source);
        }
    }
    rec.binding = Some(binding);
    rec
}

fn is_cross_api(a: Source, b: Source) -> bool {
    matches!((a, b), (Source::RawHid, Source::XInput) | (Source::XInput, Source::RawHid))
}

fn bound(ev: &InputEvent) -> BoundControl {
    BoundControl { device: (*ev.device).clone(), control: ev.control, label: ev.label.clone() }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SourceHit {
    /// ms after the cue.
    pub down_after_cue_ms: Option<f64>,
    pub up: bool,
    pub hold_ms: Option<f64>,
    pub fg_is_game_down: Option<bool>,
    pub fg_is_game_up: Option<bool>,
    /// GetTickCount() at receipt - GetMessageTime() (Raw Input only, ~16 ms granularity).
    pub queue_ms: Option<u32>,
    /// Input thread receipt -> test loop "callback" (µs, QueryPerformanceCounter-based Instant).
    pub transit_us: Option<f64>,
    pub injected: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Trial {
    pub kind: &'static str,
    pub index: usize,
    pub cue_at_s: f64,
    pub focused_at_cue: bool,
    pub foreground_at_cue: Option<String>,
    pub primary: SourceHit,
    pub alternate: Option<SourceHit>,
    /// Best-available press->callback estimate (queue_ms + transit), ms.
    pub press_to_callback_ms: Option<f64>,
}

impl Trial {
    pub fn pressed(&self) -> bool {
        self.primary.down_after_cue_ms.is_some() || self.alternate.as_ref().map(|a| a.down_after_cue_ms.is_some()).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PhaseSummary {
    pub kind: &'static str,
    pub asked: usize,
    pub pressed: usize,
    pub missed: usize,
    pub primary_seen: usize,
    pub alternate_seen: Option<usize>,
    pub released: usize,
    pub not_game_foreground: usize,
    pub extra_presses: usize,
    pub press_to_callback_ms: Option<Summary>,
    pub transit_us: Option<Summary>,
    pub reaction_ms: Option<Summary>,
    pub hold_ms: Option<Summary>,
    /// Kill switch only: presses whose measurable press->callback was <= 50 ms.
    pub within_50ms: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReceiveCheck {
    pub presses_heard: usize,
    pub press_times_s: Vec<f64>,
    pub status_json_readable: bool,
    pub status_changes_s: Vec<f64>,
    /// Presses followed by a Status.json change within 2.5 s.
    pub presses_with_status_change: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct S7Run {
    pub bindings: Option<crate::input::Bindings>,
    pub learn: Vec<LearnRecord>,
    pub trials: Vec<Trial>,
    pub ptt: Option<PhaseSummary>,
    pub kill: Option<PhaseSummary>,
    pub receive_check: Option<ReceiveCheck>,
    pub aborted: Option<String>,
    pub answers: Vec<(String, String)>,
}

fn ms_between(a: Instant, b: Instant) -> f64 {
    if b >= a {
        (b - a).as_secs_f64() * 1000.0
    } else {
        -((a - b).as_secs_f64() * 1000.0)
    }
}

/// Wait until the game window is in front (or `focus_wait` passes).
pub fn wait_for_focus(env: &dyn Env, cfg: &S7Config) -> bool {
    if !cfg.require_game {
        return true;
    }
    let deadline = Instant::now() + cfg.focus_wait;
    loop {
        if env.foreground() == cfg.game_title {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        drain_for(env, Duration::from_millis(100), &mut |_| {});
    }
}

/// Receive events for `d`, handing each to `f`.
fn drain_for(env: &dyn Env, d: Duration, f: &mut dyn FnMut(InputEvent)) {
    let end = Instant::now() + d;
    loop {
        let now = Instant::now();
        if now >= end {
            return;
        }
        if let Some(ev) = env.recv(end - now) {
            f(ev);
        }
    }
}

/// One cued trial. Returns None if the game lost focus for too long (abort).
pub fn run_trial(env: &dyn Env, cfg: &S7Config, b: &Binding, kind: &'static str, index: usize, t0: Instant, extras: &mut usize) -> Option<Trial> {
    let gap = cfg.gap_min + (cfg.gap_max.saturating_sub(cfg.gap_min)).mul_f64(fastrand::f64());
    drain_for(env, gap, &mut |ev| {
        if ev.down && b.matches_any(&ev) {
            *extras += 1;
        }
    });
    let fg = env.foreground();
    let mut focused = !cfg.require_game || fg == cfg.game_title;
    if !focused {
        env.cue(Cue::Error);
        if !wait_for_focus(env, cfg) {
            return None;
        }
        focused = false; // record that it was not focused at the original cue time
    }
    let cue_at = Instant::now();
    env.cue(if kind == "kill" { Cue::Kill } else { Cue::Ptt });
    let release_timeout = if kind == "kill" { cfg.kill_release_timeout } else { cfg.ptt_release_timeout };
    let mut prim = SourceHit::default();
    let mut alt = b.alternate.as_ref().map(|_| SourceHit::default());
    let mut prim_down_at: Option<Instant> = None;
    let mut alt_down_at: Option<Instant> = None;
    let mut first_down: Option<Instant> = None;
    let mut all_up_at: Option<Instant> = None;
    loop {
        let now = Instant::now();
        let deadline = match (first_down, all_up_at) {
            (None, _) => cue_at + cfg.press_timeout,
            (Some(_), Some(u)) => u + Duration::from_millis(250),
            (Some(d), None) => d + release_timeout,
        };
        if now >= deadline {
            break;
        }
        let Some(ev) = env.recv(deadline - now) else { continue };
        let got = Instant::now();
        let is_prim = b.matches_primary(&ev);
        let is_alt = !is_prim && b.matches_alternate(&ev);
        if !is_prim && !is_alt {
            continue;
        }
        let (hit, down_at) = if is_prim { (&mut prim, &mut prim_down_at) } else { (alt.as_mut().unwrap(), &mut alt_down_at) };
        let fg_game = ev.foreground == cfg.game_title || !cfg.require_game;
        if ev.down {
            if down_at.is_some() {
                continue; // key auto-repeat
            }
            *down_at = Some(ev.at);
            hit.down_after_cue_ms = Some(ms_between(cue_at, ev.at));
            hit.fg_is_game_down = Some(fg_game);
            hit.queue_ms = ev.queue_ms;
            hit.transit_us = Some(got.saturating_duration_since(ev.at).as_secs_f64() * 1e6);
            hit.injected = ev.injected;
            first_down.get_or_insert(ev.at);
            all_up_at = None;
        } else if let Some(d) = *down_at {
            if hit.up {
                continue;
            }
            hit.up = true;
            hit.hold_ms = Some(ms_between(d, ev.at));
            hit.fg_is_game_up = Some(fg_game);
            let prim_done = prim_down_at.is_none() || prim.up;
            let alt_done = alt_down_at.is_none() || alt.as_ref().map(|a| a.up).unwrap_or(true);
            if prim_done && alt_done {
                all_up_at = Some(ev.at);
            }
        }
    }
    let best = [Some(&prim), alt.as_ref()]
        .into_iter()
        .flatten()
        .filter(|h| h.down_after_cue_ms.is_some())
        .min_by(|a, b| a.down_after_cue_ms.partial_cmp(&b.down_after_cue_ms).unwrap());
    let press_to_callback_ms = best.and_then(|h| h.transit_us.map(|t| h.queue_ms.unwrap_or(0) as f64 + t / 1000.0));
    Some(Trial {
        kind,
        index,
        cue_at_s: (cue_at - t0).as_secs_f64(),
        focused_at_cue: focused,
        foreground_at_cue: if focused { None } else { Some(fg) },
        primary: prim,
        alternate: alt,
        press_to_callback_ms,
    })
}

pub fn summarize_phase(kind: &'static str, trials: &[Trial], extras: usize) -> PhaseSummary {
    let t: Vec<&Trial> = trials.iter().filter(|t| t.kind == kind).collect();
    let hits = |f: &dyn Fn(&Trial) -> Option<f64>| -> Option<Summary> { summarize(&t.iter().filter_map(|x| f(x)).collect::<Vec<_>>()) };
    let first_hit = |x: &Trial| -> Option<SourceHit> {
        [Some(&x.primary), x.alternate.as_ref()]
            .into_iter()
            .flatten()
            .filter(|h| h.down_after_cue_ms.is_some())
            .min_by(|a, b| a.down_after_cue_ms.partial_cmp(&b.down_after_cue_ms).unwrap())
            .cloned()
    };
    let has_alt = t.iter().any(|x| x.alternate.is_some());
    PhaseSummary {
        kind,
        asked: t.len(),
        pressed: t.iter().filter(|x| x.pressed()).count(),
        missed: t.iter().filter(|x| !x.pressed()).count(),
        primary_seen: t.iter().filter(|x| x.primary.down_after_cue_ms.is_some()).count(),
        alternate_seen: if has_alt { Some(t.iter().filter(|x| x.alternate.as_ref().map(|a| a.down_after_cue_ms.is_some()).unwrap_or(false)).count()) } else { None },
        released: t.iter().filter(|x| first_hit(x).map(|h| h.up).unwrap_or(false)).count(),
        not_game_foreground: t.iter().filter(|x| first_hit(x).and_then(|h| h.fg_is_game_down) == Some(false) || !x.focused_at_cue).count(),
        extra_presses: extras,
        press_to_callback_ms: hits(&|x| x.press_to_callback_ms),
        transit_us: hits(&|x| first_hit(x).and_then(|h| h.transit_us)),
        reaction_ms: hits(&|x| first_hit(x).and_then(|h| h.down_after_cue_ms)),
        hold_ms: hits(&|x| first_hit(x).and_then(|h| h.hold_ms)),
        within_50ms: if kind == "kill" { Some(t.iter().filter(|x| x.press_to_callback_ms.map(|v| v <= 50.0).unwrap_or(false)).count()) } else { None },
    }
}

/// Phase C: the user presses in-game-bound buttons on the same device; we count what we
/// hear and watch Status.json for changes (the game rewrites it ~once a second).
pub fn receive_check(env: &dyn Env, cfg: &S7Config, b: &Binding, t0: Instant) -> ReceiveCheck {
    env.cue(Cue::Phase);
    let start = Instant::now();
    let mut presses: Vec<Instant> = vec![];
    let mut changes: Vec<Instant> = vec![];
    let mut last = env.status();
    let readable = last.is_some();
    let mut held = std::collections::HashSet::new();
    let mut next_poll = Instant::now();
    let mut done_at: Option<Instant> = None;
    while start.elapsed() < cfg.receive_check {
        if let Some(d) = done_at {
            // keep watching Status.json a bit after the last press
            if d.elapsed() > Duration::from_millis(2600) {
                break;
            }
        }
        if Instant::now() >= next_poll {
            next_poll = Instant::now() + Duration::from_millis(100);
            let s = env.status();
            if s.is_some() && s != last {
                if last.is_some() {
                    changes.push(Instant::now());
                }
                last = s;
            }
        }
        if let Some(ev) = env.recv(Duration::from_millis(20)) {
            let same_dev = b.primary.device.same_device(&ev.device) || b.alternate.as_ref().map(|a| a.device.same_device(&ev.device)).unwrap_or(false);
            // Count each physical press once (primary device's API only).
            if same_dev && ev.device.source == b.primary.device.source {
                if ev.down {
                    if held.insert(ev.control) && done_at.is_none() {
                        presses.push(ev.at);
                        if presses.len() >= cfg.receive_check_presses {
                            done_at = Some(Instant::now());
                        }
                    }
                } else {
                    held.remove(&ev.control);
                }
            }
        }
    }
    let with_change = presses
        .iter()
        .filter(|p| changes.iter().any(|c| *c >= **p && c.duration_since(**p) <= Duration::from_millis(2500)))
        .count();
    ReceiveCheck {
        presses_heard: presses.len(),
        press_times_s: presses.iter().map(|p| (*p - t0).as_secs_f64()).collect(),
        status_json_readable: readable,
        status_changes_s: changes.iter().map(|c| (*c - t0).as_secs_f64()).collect(),
        presses_with_status_change: with_change,
    }
}

/// The in-game part: countdown, focus wait, PTT trials, kill trials, receive check.
pub fn run_test(env: &dyn Env, cfg: &S7Config, bindings: &crate::input::Bindings, run: &mut S7Run) {
    env.out(&format!(
        "\n  When you press Enter, a {} s countdown starts (ticks on the last 5 s). Switch to Elite and stay there.\n\
         \n  What you will hear (in the game):\n\
         \x20   rising 3 notes ......... test started\n\
         \x20   one high beep .......... press AND release PTT ({}), hold it about half a second\n\
         \x20   two low beeps .......... press the kill switch ({})\n\
         \x20   long buzz .............. Elite is not the active window - click back into the game\n\
         \x20   two-note chime ......... last part: press a button on the same device that does something\n\
         \x20                            visible in Elite (lights, landing gear, cargo scoop...) {} times, ~3 s apart\n\
         \x20   falling 3 notes ........ finished - come back here\n\
         \n  {} PTT presses and {} kill-switch presses; about {} minutes in total.",
        cfg.countdown_secs,
        bindings.ptt.primary.label,
        bindings.kill.primary.label,
        cfg.receive_check_presses,
        cfg.ptt_count,
        cfg.kill_count,
        ((cfg.ptt_count + cfg.kill_count) as f64 * 4.5 / 60.0 + 0.8).ceil()
    ));
    env.ask("  Press Enter to start the countdown...");
    for s in (1..=cfg.countdown_secs).rev() {
        env.out(&format!("  {s}..."));
        if s <= 5 {
            env.cue(Cue::Tick);
        }
        drain_for(env, Duration::from_secs(1), &mut |_| {});
    }
    if !wait_for_focus(env, cfg) {
        env.cue(Cue::Error);
        run.aborted = Some(format!("'{}' never became the active window", cfg.game_title));
        return;
    }
    let t0 = Instant::now();
    env.cue(Cue::Start);
    drain_for(env, Duration::from_millis(1500), &mut |_| {});
    let mut extras_ptt = 0;
    for i in 1..=cfg.ptt_count {
        match run_trial(env, cfg, &bindings.ptt, "ptt", i, t0, &mut extras_ptt) {
            Some(t) => run.trials.push(t),
            None => {
                run.aborted = Some(format!("game lost focus for more than {} s during PTT trial {i}", cfg.focus_wait.as_secs()));
                break;
            }
        }
    }
    let mut extras_kill = 0;
    if run.aborted.is_none() {
        drain_for(env, Duration::from_millis(1500), &mut |_| {});
        for i in 1..=cfg.kill_count {
            match run_trial(env, cfg, &bindings.kill, "kill", i, t0, &mut extras_kill) {
                Some(t) => run.trials.push(t),
                None => {
                    run.aborted = Some(format!("game lost focus for more than {} s during kill trial {i}", cfg.focus_wait.as_secs()));
                    break;
                }
            }
        }
    }
    run.ptt = Some(summarize_phase("ptt", &run.trials, extras_ptt));
    run.kill = Some(summarize_phase("kill", &run.trials, extras_kill));
    if run.aborted.is_none() {
        drain_for(env, Duration::from_millis(1500), &mut |_| {});
        run.receive_check = Some(receive_check(env, cfg, &bindings.ptt, t0));
    }
    env.cue(if run.aborted.is_some() { Cue::Error } else { Cue::Done });
}

pub fn print_summary(env: &dyn Env, run: &S7Run) {
    if let Some(a) = &run.aborted {
        env.out(&format!("  ABORTED: {a}"));
    }
    for p in [&run.ptt, &run.kill].into_iter().flatten() {
        let name = if p.kind == "ptt" { "PTT" } else { "Kill switch" };
        env.out(&format!(
            "  {name}: {}/{} presses received, {} missed, {} released, {} not with Elite in front, {} extra presses",
            p.pressed, p.asked, p.missed, p.released, p.not_game_foreground, p.extra_presses
        ));
        if let Some(a) = p.alternate_seen {
            env.out(&format!("    primary API saw {}/{}, other API saw {a}/{}", p.primary_seen, p.asked, p.asked));
        }
        if let Some(s) = &p.press_to_callback_ms {
            env.out(&format!("    press->callback ms (OS queue + our pipeline): {s}"));
        }
        if let Some(s) = &p.transit_us {
            env.out(&format!("    our pipeline only, µs: {s}"));
        }
        if let Some(w) = p.within_50ms {
            env.out(&format!("    within the 50 ms kill-switch target: {w}/{}", p.pressed));
        }
    }
    if let Some(r) = &run.receive_check {
        env.out(&format!(
            "  Game-receives check: {} presses heard; Status.json {}; {} of them followed by a Status.json change within 2.5 s",
            r.presses_heard,
            if r.status_json_readable { "readable" } else { "NOT readable (game not running or different folder)" },
            r.presses_with_status_change
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::tests::{dev, ev};
    use crate::input::{Bindings, Control};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::sync::mpsc;

    type OnCue = Box<dyn FnMut(Cue, &mpsc::Sender<InputEvent>)>;

    /// Fake environment: a script reacts to cues by queueing input events on a channel.
    struct Fake {
        rx: mpsc::Receiver<InputEvent>,
        tx: mpsc::Sender<InputEvent>,
        on_cue: RefCell<OnCue>,
        cues: RefCell<Vec<Cue>>,
        fg: RefCell<String>,
        answers: RefCell<VecDeque<String>>,
        status_calls: RefCell<u32>,
    }

    impl Env for Fake {
        fn recv(&self, t: Duration) -> Option<InputEvent> {
            self.rx.recv_timeout(t).ok()
        }
        fn cue(&self, c: Cue) {
            self.cues.borrow_mut().push(c);
            (self.on_cue.borrow_mut())(c, &self.tx);
        }
        fn foreground(&self) -> String {
            self.fg.borrow().clone()
        }
        fn status(&self) -> Option<StatusSnap> {
            let mut n = self.status_calls.borrow_mut();
            *n += 1;
            Some(StatusSnap { key: format!("{}", *n / 8) })
        }
        fn out(&self, _s: &str) {}
        fn ask(&self, _q: &str) -> String {
            self.answers.borrow_mut().pop_front().unwrap_or_default()
        }
    }

    fn fake(on_cue: OnCue) -> Fake {
        let (tx, rx) = mpsc::channel();
        Fake { rx, tx, on_cue: RefCell::new(on_cue), cues: RefCell::new(vec![]), fg: RefCell::new("Elite - Dangerous (CLIENT)".into()), answers: RefCell::new(VecDeque::new()), status_calls: RefCell::new(0) }
    }

    fn fast_cfg() -> S7Config {
        S7Config {
            ptt_count: 4,
            kill_count: 2,
            press_timeout: Duration::from_millis(150),
            ptt_release_timeout: Duration::from_millis(200),
            kill_release_timeout: Duration::from_millis(100),
            gap_min: Duration::from_millis(5),
            gap_max: Duration::from_millis(10),
            countdown_secs: 0,
            focus_wait: Duration::from_millis(50),
            require_game: true,
            receive_check: Duration::from_millis(1500),
            receive_check_presses: 2,
            learn_timeout: Duration::from_millis(300),
            ..Default::default()
        }
    }

    fn bindings() -> (Bindings, Control, Control) {
        let stick = dev(Source::RawHid, "stick", 1, 2);
        let x = dev(Source::XInput, "xinput:0", 0, 0);
        let b1 = Control::HidButton { page: 9, usage: 1 };
        let b2 = Control::HidButton { page: 9, usage: 2 };
        let ptt = Binding {
            primary: BoundControl { device: (*stick).clone(), control: b1, label: "button 1".into() },
            alternate: Some(BoundControl { device: (*x).clone(), control: Control::XButton { mask: 0x1000 }, label: "A".into() }),
        };
        let kill = Binding { primary: BoundControl { device: (*stick).clone(), control: b2, label: "button 2".into() }, alternate: None };
        (Bindings { ptt, kill }, b1, b2)
    }

    #[test]
    fn trials_count_hits_misses_and_alternate() {
        let (b, b1, b2) = bindings();
        let stick = dev(Source::RawHid, "stick", 1, 2);
        let x = dev(Source::XInput, "xinput:0", 0, 0);
        let mut n_ptt = 0;
        let f = fake(Box::new(move |c, tx| match c {
            Cue::Ptt => {
                n_ptt += 1;
                if n_ptt == 3 {
                    return; // user misses trial 3
                }
                tx.send(ev(&stick, b1, true)).unwrap();
                if n_ptt != 2 {
                    tx.send(ev(&x, Control::XButton { mask: 0x1000 }, true)).unwrap();
                    tx.send(ev(&x, Control::XButton { mask: 0x1000 }, false)).unwrap();
                }
                tx.send(ev(&stick, b1, true)).unwrap(); // auto-repeat, ignored
                tx.send(ev(&stick, b1, false)).unwrap();
            }
            Cue::Kill => {
                tx.send(ev(&stick, b2, true)).unwrap();
                tx.send(ev(&stick, b2, false)).unwrap();
            }
            Cue::Phase => {
                for c in [b1, b2] {
                    tx.send(ev(&stick, c, true)).unwrap();
                    tx.send(ev(&stick, c, false)).unwrap();
                }
            }
            _ => {}
        }));
        let mut run = S7Run { bindings: Some(b.clone()), learn: vec![], trials: vec![], ptt: None, kill: None, receive_check: None, aborted: None, answers: vec![] };
        run_test(&f, &fast_cfg(), &b, &mut run);
        assert!(run.aborted.is_none());
        let p = run.ptt.unwrap();
        assert_eq!((p.asked, p.pressed, p.missed, p.primary_seen, p.alternate_seen, p.released), (4, 3, 1, 3, Some(2), 3));
        let k = run.kill.unwrap();
        assert_eq!((k.asked, k.pressed, k.missed, k.within_50ms), (2, 2, 0, Some(2)));
        assert!(k.press_to_callback_ms.unwrap().max < 50.0);
        let rc = run.receive_check.unwrap();
        assert_eq!(rc.presses_heard, 2);
        assert!(rc.status_json_readable);
        assert!(!rc.status_changes_s.is_empty());
        assert_eq!(*f.cues.borrow().last().unwrap(), Cue::Done);
    }

    #[test]
    fn aborts_when_game_not_focused() {
        let (b, _, _) = bindings();
        let f = fake(Box::new(|_, _| {}));
        *f.fg.borrow_mut() = "Command Prompt".into();
        let mut run = S7Run { bindings: None, learn: vec![], trials: vec![], ptt: None, kill: None, receive_check: None, aborted: None, answers: vec![] };
        run_test(&f, &fast_cfg(), &b, &mut run);
        assert!(run.aborted.unwrap().contains("never became the active window"));
        assert!(run.trials.is_empty());
    }

    #[test]
    fn learn_picks_first_press_and_cross_api_alternate() {
        let stick = dev(Source::RawHid, "pad", 0x045E, 0x0B12);
        let x = dev(Source::XInput, "xinput:0", 0, 0);
        let mouse = dev(Source::RawMouse, "m", 0, 0);
        let f = fake(Box::new(|_, _| {}));
        // Arrive after the 300 ms settle period.
        let tx = f.tx.clone();
        let (s2, x2, m2) = (stick.clone(), x.clone(), mouse.clone());
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(350));
            tx.send(ev(&m2, Control::MouseButton { n: 1 }, true)).unwrap(); // ignored
            tx.send(ev(&s2, Control::HidButton { page: 9, usage: 1 }, true)).unwrap();
            tx.send(ev(&x2, Control::XButton { mask: 0x1000 }, true)).unwrap();
            tx.send(ev(&s2, Control::HidButton { page: 9, usage: 1 }, false)).unwrap();
        });
        let cfg = S7Config { learn_timeout: Duration::from_secs(2), ..fast_cfg() };
        let r = learn(&f, &cfg, "PTT", None);
        let b = r.binding.unwrap();
        assert_eq!(b.primary.control, Control::HidButton { page: 9, usage: 1 });
        assert_eq!(b.alternate.unwrap().control, Control::XButton { mask: 0x1000 });
        assert_eq!(r.seen_by, vec![Source::RawHid, Source::XInput]);
        assert!(r.released_seen);
    }

    #[test]
    fn learn_rejects_same_button_and_supports_escape() {
        let (b, b1, _) = bindings();
        let stick = dev(Source::RawHid, "stick", 1, 2);
        let kb = dev(Source::RawKeyboard, "kb", 0, 0);
        let f = fake(Box::new(|_, _| {}));
        let tx = f.tx.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(350));
            tx.send(ev(&stick, b1, true)).unwrap(); // same as PTT -> rejected
            tx.send(ev(&kb, Control::Key { vkey: 0x1B, scan: 1, e0: false }, true)).unwrap();
        });
        let cfg = S7Config { learn_timeout: Duration::from_secs(2), ..fast_cfg() };
        let r = learn(&f, &cfg, "kill switch", Some(&b.ptt));
        assert!(r.binding.is_none());
        assert_eq!(r.note.as_deref(), Some("skipped with Esc"));
    }
}
