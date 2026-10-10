//! The measurement loop: safe key presses + dual Status.json detection.

use crate::binds::Chord;
use crate::keys::ScanKey;
use crate::platform::{Platform, GAME_WINDOW_TITLE};
use crate::status::{self, read_status, read_status_retry};
use anyhow::{bail, Result};
use notify::Watcher;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Src {
    Poll,
    Notify,
}

#[derive(Debug, Clone, Copy)]
pub struct Obs {
    pub src: Src,
    pub t: Instant,
    pub flags: u64,
}

pub struct Config {
    pub action: String,
    pub bit: u64,
    pub chord: Chord,
    pub status_path: PathBuf,
    pub watch_dir: PathBuf,
    pub timeout: Duration,
    pub gap_ms: (u64, u64),
    pub poll_interval: Duration,
    pub focus_giveup: Duration,
    /// Require GuiFocus == 0 before pressing.
    pub require_no_panel: bool,
    /// Allowed context: returns Err(reason) if the current flags make a press unsafe.
    pub context_check: fn(u64) -> std::result::Result<(), String>,
    pub quiet: bool,
}

#[derive(Debug, Clone)]
pub struct TrialResult {
    pub trial: usize,
    pub hold_ms: u64,
    pub actual_hold_ms: f64,
    pub expected_on: bool,
    pub poll_ms: Option<f64>,
    pub notify_ms: Option<f64>,
    pub wallclock: String,
}

impl TrialResult {
    pub fn missed(&self) -> bool {
        self.poll_ms.is_none() && self.notify_ms.is_none()
    }
}

#[derive(Debug)]
pub enum Stop {
    Cancelled,
    FocusLost,
    InputFailed(String),
    Context(String),
}

impl std::fmt::Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stop::Cancelled => write!(f, "stopped by Ctrl+C"),
            Stop::FocusLost => write!(f, "the game was not in focus (or a panel was open) for too long"),
            Stop::InputFailed(e) => write!(f, "key input failed: {e}"),
            Stop::Context(e) => write!(f, "unsafe game state: {e}"),
        }
    }
}

/// Tracks keys we have pressed and releases any still held on drop (Ctrl+C, error, panic).
pub struct KeyGuard<'a> {
    platform: &'a dyn Platform,
    held: Vec<ScanKey>,
}

impl<'a> KeyGuard<'a> {
    pub fn new(platform: &'a dyn Platform) -> Self {
        Self { platform, held: vec![] }
    }
    pub fn down(&mut self, k: ScanKey) -> std::result::Result<(), String> {
        self.held.push(k);
        self.platform.send_key(k, false)
    }
    pub fn up(&mut self, k: ScanKey) -> std::result::Result<(), String> {
        let r = self.platform.send_key(k, true);
        if let Some(i) = self.held.iter().rposition(|h| *h == k) {
            self.held.remove(i);
        }
        r
    }
    pub fn release_all(&mut self) {
        while let Some(k) = self.held.pop() {
            let _ = self.platform.send_key(k, true);
        }
    }
}

impl Drop for KeyGuard<'_> {
    fn drop(&mut self) {
        self.release_all();
    }
}

/// Sleep with ~0.1 ms accuracy: coarse sleep, then spin.
pub fn precise_sleep(d: Duration) {
    let end = Instant::now() + d;
    if d > Duration::from_millis(3) {
        std::thread::sleep(d - Duration::from_millis(2));
    }
    while Instant::now() < end {
        std::hint::spin_loop();
    }
}

/// Both detectors, running for the whole session.
pub struct Detectors {
    rx: Receiver<Obs>,
    stop: Arc<AtomicBool>,
    poll_thread: Option<std::thread::JoinHandle<()>>,
    _watcher: Option<notify::RecommendedWatcher>,
    pub notify_error: Option<String>,
}

impl Detectors {
    pub fn start(status_path: &Path, watch_dir: &Path, poll_interval: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));

        // (a) tight poll
        let p = status_path.to_path_buf();
        let tx_poll = tx.clone();
        let stop_poll = stop.clone();
        let poll_thread = std::thread::spawn(move || {
            let mut last: Option<u64> = None;
            while !stop_poll.load(Ordering::Relaxed) {
                if let Some(s) = read_status(&p) {
                    let now = Instant::now();
                    if last != Some(s.flags) {
                        last = Some(s.flags);
                        let _ = tx_poll.send(Obs { src: Src::Poll, t: now, flags: s.flags });
                    }
                }
                std::thread::sleep(poll_interval);
            }
        });

        // (b) change notification (ReadDirectoryChangesW on Windows) then read
        let p = status_path.to_path_buf();
        let file_name = p.file_name().map(|f| f.to_os_string());
        let last = std::sync::Mutex::new(None::<u64>);
        let handler = move |res: notify::Result<notify::Event>| {
            let Ok(ev) = res else { return };
            if !ev.paths.iter().any(|q| q.file_name().map(|f| f.to_os_string()) == file_name) {
                return;
            }
            if let Some(s) = read_status_retry(&p, 5) {
                let now = Instant::now();
                let mut l = last.lock().unwrap();
                if *l != Some(s.flags) {
                    *l = Some(s.flags);
                    let _ = tx.send(Obs { src: Src::Notify, t: now, flags: s.flags });
                }
            }
        };
        let mut notify_error = None;
        let watcher = match notify::recommended_watcher(handler) {
            Ok(mut w) => match w.watch(watch_dir, notify::RecursiveMode::NonRecursive) {
                Ok(()) => Some(w),
                Err(e) => {
                    notify_error = Some(format!("watch failed: {e}"));
                    None
                }
            },
            Err(e) => {
                notify_error = Some(format!("watcher failed: {e}"));
                None
            }
        };
        Self { rx, stop, poll_thread: Some(poll_thread), _watcher: watcher, notify_error }
    }

    pub fn drain(&self) {
        while self.rx.try_recv().is_ok() {}
    }

    /// Wait for the bit to reach `expected` after `t0`, per source.
    pub fn wait_flip(&self, t0: Instant, bit: u64, expected: bool, timeout: Duration, want_notify: bool)
        -> (Option<Instant>, Option<Instant>) {
        let deadline = t0 + timeout;
        let (mut poll, mut notif) = (None, None);
        loop {
            if poll.is_some() && (notif.is_some() || !want_notify) {
                break;
            }
            let now = Instant::now();
            if now >= deadline {
                break;
            }
            match self.rx.recv_timeout(deadline - now) {
                Ok(o) => {
                    if o.t < t0 || ((o.flags & bit) != 0) != expected {
                        continue;
                    }
                    match o.src {
                        Src::Poll if poll.is_none() => poll = Some(o.t),
                        Src::Notify if notif.is_none() => notif = Some(o.t),
                        _ => {}
                    }
                }
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
        (poll, notif)
    }
}

impl Drop for Detectors {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.poll_thread.take() {
            let _ = t.join();
        }
    }
}

pub struct Session<'a> {
    pub cfg: &'a Config,
    pub platform: &'a dyn Platform,
    pub cancel: Arc<AtomicBool>,
    pub detectors: Detectors,
    unready_since: Option<Instant>,
    announced_wait: bool,
    pub trial_no: usize,
}

impl<'a> Session<'a> {
    pub fn new(cfg: &'a Config, platform: &'a dyn Platform, cancel: Arc<AtomicBool>) -> Self {
        let detectors = Detectors::start(&cfg.status_path, &cfg.watch_dir, cfg.poll_interval);
        Self { cfg, platform, cancel, detectors, unready_since: None, announced_wait: false, trial_no: 0 }
    }

    fn say(&self, s: &str) {
        if !self.cfg.quiet {
            println!("{s}");
        }
    }

    /// Is it OK to press right now? Ok(None) = ready, Ok(Some(reason)) = wait, Err = stop.
    fn readiness(&self) -> std::result::Result<Option<String>, Stop> {
        let title = self.platform.foreground_title().unwrap_or_default();
        if title != GAME_WINDOW_TITLE {
            return Ok(Some(format!("foreground window is '{title}', not '{GAME_WINDOW_TITLE}'")));
        }
        match read_status(&self.cfg.status_path) {
            None => Ok(Some("Status.json unreadable or has no Flags (main menu?)".into())),
            Some(s) => {
                if let Err(e) = (self.cfg.context_check)(s.flags) {
                    return Err(Stop::Context(e));
                }
                if self.cfg.require_no_panel && s.gui_focus.unwrap_or(0) != 0 {
                    return Ok(Some(format!("a panel or map is open (GuiFocus={})", s.gui_focus.unwrap_or(0))));
                }
                Ok(None)
            }
        }
    }

    /// Sleep `d` while watching for Ctrl+C and long focus loss.
    pub fn idle(&mut self, d: Duration) -> std::result::Result<(), Stop> {
        let end = Instant::now() + d;
        loop {
            if self.cancel.load(Ordering::SeqCst) {
                return Err(Stop::Cancelled);
            }
            let now = Instant::now();
            if now >= end {
                return Ok(());
            }
            self.track_focus()?;
            std::thread::sleep((end - now).min(Duration::from_millis(100)));
        }
    }

    fn track_focus(&mut self) -> std::result::Result<bool, Stop> {
        match self.readiness()? {
            None => {
                self.unready_since = None;
                self.announced_wait = false;
                Ok(true)
            }
            Some(reason) => {
                let since = *self.unready_since.get_or_insert_with(Instant::now);
                if since.elapsed() > self.cfg.focus_giveup {
                    return Err(Stop::FocusLost);
                }
                if !self.announced_wait {
                    self.say(&format!("  ...paused: {reason}. Waiting (gives up after {} s)", self.cfg.focus_giveup.as_secs()));
                    self.announced_wait = true;
                }
                Ok(false)
            }
        }
    }

    /// Block until the game is focused and ready (never presses while waiting).
    pub fn wait_ready(&mut self) -> std::result::Result<(), Stop> {
        loop {
            if self.cancel.load(Ordering::SeqCst) {
                return Err(Stop::Cancelled);
            }
            if self.track_focus()? {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    pub fn current_bit(&self) -> Option<bool> {
        read_status_retry(&self.cfg.status_path, 20).map(|s| s.flags & self.cfg.bit != 0)
    }

    /// One press: modifiers down, key down (t0), hold, key up, modifiers up.
    fn press(&mut self, hold: Duration) -> std::result::Result<(Instant, f64), Stop> {
        let platform = self.platform;
        let cfg = self.cfg;
        let mut guard = KeyGuard::new(platform);
        let chord = &cfg.chord;
        for (_, m) in &chord.modifiers {
            guard.down(*m).map_err(Stop::InputFailed)?;
            precise_sleep(Duration::from_millis(15));
        }
        let t0 = Instant::now();
        guard.down(chord.key).map_err(Stop::InputFailed)?;
        precise_sleep(hold);
        let r = guard.up(chord.key);
        let actual = t0.elapsed().as_secs_f64() * 1000.0;
        r.map_err(Stop::InputFailed)?;
        for (_, m) in chord.modifiers.iter().rev() {
            precise_sleep(Duration::from_millis(10));
            guard.up(*m).map_err(Stop::InputFailed)?;
        }
        Ok((t0, actual))
    }

    pub fn trial(&mut self, hold_ms: u64) -> std::result::Result<TrialResult, Stop> {
        self.wait_ready()?;
        let Some(before) = self.current_bit() else {
            return Err(Stop::Context("cannot read Status.json".into()));
        };
        let expected = !before;
        self.detectors.drain();
        // Re-check focus at the last moment, then press.
        if self.platform.foreground_title().as_deref() != Some(GAME_WINDOW_TITLE) {
            self.wait_ready()?;
        }
        let (t0, actual) = self.press(Duration::from_millis(hold_ms))?;
        let (p, n) = self.detectors.wait_flip(
            t0, self.cfg.bit, expected, self.cfg.timeout, self.detectors.notify_error.is_none());
        self.trial_no += 1;
        let ms = |t: Option<Instant>| t.map(|t| (t - t0).as_secs_f64() * 1000.0);
        Ok(TrialResult {
            trial: self.trial_no,
            hold_ms,
            actual_hold_ms: actual,
            expected_on: expected,
            poll_ms: ms(p),
            notify_ms: ms(n),
            wallclock: chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string(),
        })
    }

    pub fn gap(&mut self) -> std::result::Result<(), Stop> {
        let (lo, hi) = self.cfg.gap_ms;
        let ms = if hi > lo { fastrand::u64(lo..=hi) } else { lo };
        self.idle(Duration::from_millis(ms))
    }
}

pub fn line_for(r: &TrialResult, action: &str) -> String {
    let f = |x: Option<f64>| x.map(|v| format!("{v:7.1} ms")).unwrap_or_else(|| "   MISS   ".into());
    format!(
        "  #{:>3}  {:<20} -> {:<3}  poll {}  notify {}  (hold {:.1} ms){}",
        r.trial,
        action,
        if r.expected_on { "ON" } else { "OFF" },
        f(r.poll_ms),
        f(r.notify_ms),
        r.actual_hold_ms,
        if r.missed() { "  <- no change within timeout" } else { "" }
    )
}

pub fn write_csv(path: &Path, action: &str, rows: &[TrialResult]) -> Result<()> {
    let mut s = String::from("trial,action,expected_state,poll_ms,notify_ms,missed,hold_ms,actual_hold_ms,wallclock\n");
    let f = |x: Option<f64>| x.map(|v| format!("{v:.3}")).unwrap_or_default();
    for r in rows {
        s += &format!(
            "{},{},{},{},{},{},{},{:.3},{}\n",
            r.trial,
            action,
            if r.expected_on { "on" } else { "off" },
            f(r.poll_ms),
            f(r.notify_ms),
            r.missed(),
            r.hold_ms,
            r.actual_hold_ms,
            r.wallclock
        );
    }
    std::fs::write(path, s)?;
    Ok(())
}

/// Context rules per action (refuse to press when unsafe).
pub fn context_ship_only(flags: u64) -> std::result::Result<(), String> {
    if flags & status::FLAG_IN_MAIN_SHIP == 0 {
        return Err("you are not in your ship's cockpit (Status.json Flags bit 24 is off)".into());
    }
    Ok(())
}

pub fn context_ship_or_srv(flags: u64) -> std::result::Result<(), String> {
    if flags & (status::FLAG_IN_MAIN_SHIP | status::FLAG_IN_SRV) == 0 {
        return Err("you are not in your ship or SRV".into());
    }
    Ok(())
}

pub fn context_gear(flags: u64) -> std::result::Result<(), String> {
    context_ship_only(flags)?;
    if flags & status::FLAG_DOCKED != 0 {
        return Err("docked: gear must only be tested in normal space".into());
    }
    if flags & status::FLAG_LANDED != 0 {
        return Err("landed: gear must only be tested in normal space".into());
    }
    if flags & status::FLAG_SUPERCRUISE != 0 {
        return Err("in supercruise: gear must only be tested in normal space".into());
    }
    Ok(())
}

pub fn ensure_parent_writable(dir: &Path) -> Result<()> {
    let probe = dir.join(".s5-write-test");
    match std::fs::write(&probe, b"x") {
        Ok(()) => {
            let _ = std::fs::remove_file(probe);
            Ok(())
        }
        Err(e) => bail!("{} is not writable: {e}", dir.display()),
    }
}
