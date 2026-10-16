//! # astromech-policy
//!
//! Every state-changing command passes through here (ARCHITECTURE.md §5.14), and this is the
//! **only** crate that drives `astromech_input::exec`:
//! - tiers (T3 never automated; cloud origin limited to T0/T1; T2 needs confirmation),
//! - a rate limit per origin,
//! - the **in-flight duplicate block**: the same tool can't start again while it is in flight
//!   or within 3 s of its start,
//! - the **kill switch**: cancels pending tickets, releases held keys and refuses all presses
//!   until re-armed,
//! - pre-press checks: game window focused, integrity levels compatible (UIPI).

use astromech_input::exec::keys::{self, KeyLookup, ScanKey};
use astromech_input::exec::platform::Platform;
use astromech_proto::{Origin, Tier};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The game window title the executor targets.
pub const TARGET_WINDOW: &str = astromech_input::ELITE_TITLE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Denial {
    KillSwitch,
    NeverAutomated,
    OriginNotAllowed,
    Duplicate,
    RateLimited,
    NeedsConfirmation(String),
}

impl Denial {
    pub fn spoken(&self) -> String {
        match self {
            Denial::KillSwitch => "Kill switch is on. Re-arm it to allow actions.".into(),
            Denial::NeverAutomated => "That's never automated.".into(),
            Denial::OriginNotAllowed => "Not allowed from there.".into(),
            Denial::Duplicate => "Already on it.".into(),
            Denial::RateLimited => "Too many actions, slow down.".into(),
            Denial::NeedsConfirmation(p) => p.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PressError {
    Cancelled,
    NotForeground(String),
    Privilege(String),
    UnknownKey(String),
    Send(String),
}

impl PressError {
    pub fn spoken(&self) -> String {
        match self {
            PressError::Cancelled => "Cancelled.".into(),
            PressError::NotForeground(_) => "Elite isn't the active window, so I didn't press anything.".into(),
            PressError::Privilege(_) => {
                "Windows is blocking key presses to the game. Run both normally, not as administrator.".into()
            }
            PressError::UnknownKey(k) => format!("I can't press the key {k}. Please bind a different key."),
            PressError::Send(_) => "The key press failed.".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    /// Key hold time (S5: 50 ms is reliable).
    pub hold: Duration,
    /// Gap between modifier and key events.
    pub gap: Duration,
    pub duplicate_window: Duration,
    /// At most this many T1+ actions per `rate_window`.
    pub rate_max: usize,
    pub rate_window: Duration,
    /// Skip the foreground and integrity checks (tests and `--dry-run` only).
    pub skip_window_checks: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hold: Duration::from_millis(50),
            gap: Duration::from_millis(15),
            duplicate_window: Duration::from_secs(3),
            rate_max: 8,
            rate_window: Duration::from_secs(10),
            skip_window_checks: false,
        }
    }
}

struct State {
    in_flight: HashMap<String, Instant>,
    recent: HashMap<String, Instant>,
    starts: VecDeque<Instant>,
}

struct Inner {
    cfg: Config,
    platform: Arc<dyn Platform>,
    killed: AtomicBool,
    generation: AtomicU64,
    state: Mutex<State>,
    held: Mutex<Vec<ScanKey>>,
}

/// The policy gate. Cheap to clone; share one per process.
#[derive(Clone)]
pub struct Policy {
    inner: Arc<Inner>,
}

/// Permission to act for one tool call. Presses through it are refused once the kill switch
/// fires. Dropping it ends the in-flight period (the 3 s window still applies).
pub struct Ticket {
    inner: Arc<Inner>,
    name: String,
    generation: u64,
}

impl Policy {
    pub fn new(platform: Arc<dyn Platform>, cfg: Config) -> Self {
        Self {
            inner: Arc::new(Inner {
                cfg,
                platform,
                killed: AtomicBool::new(false),
                generation: AtomicU64::new(0),
                state: Mutex::new(State { in_flight: HashMap::new(), recent: HashMap::new(), starts: VecDeque::new() }),
                held: Mutex::new(vec![]),
            }),
        }
    }

    pub fn config(&self) -> &Config {
        &self.inner.cfg
    }

    pub fn is_killed(&self) -> bool {
        self.inner.killed.load(Ordering::SeqCst)
    }

    /// Kill switch: cancel every ticket, release held keys, refuse until [`Policy::rearm`].
    pub fn kill(&self) {
        self.inner.killed.store(true, Ordering::SeqCst);
        self.inner.generation.fetch_add(1, Ordering::SeqCst);
        self.inner.release_all();
        self.inner.state.lock().unwrap().in_flight.clear();
    }

    pub fn rearm(&self) {
        self.inner.killed.store(false, Ordering::SeqCst);
    }

    /// Ask to start a command.
    pub fn authorize(&self, name: &str, tier: Tier, origin: Origin, now: Instant) -> Result<Ticket, Denial> {
        let i = &self.inner;
        if tier > Tier::T0 && self.is_killed() {
            return Err(Denial::KillSwitch);
        }
        if tier == Tier::T3 {
            return Err(Denial::NeverAutomated);
        }
        if origin == Origin::Cloud && tier > Tier::T1 {
            return Err(Denial::OriginNotAllowed);
        }
        let mut st = i.state.lock().unwrap();
        if tier > Tier::T0 {
            if st.in_flight.contains_key(name) {
                return Err(Denial::Duplicate);
            }
            if let Some(t) = st.recent.get(name) {
                if now.saturating_duration_since(*t) < i.cfg.duplicate_window {
                    return Err(Denial::Duplicate);
                }
            }
            while st.starts.front().map(|t| now.saturating_duration_since(*t) >= i.cfg.rate_window).unwrap_or(false) {
                st.starts.pop_front();
            }
            if st.starts.len() >= i.cfg.rate_max {
                return Err(Denial::RateLimited);
            }
        }
        if tier == Tier::T2 {
            return Err(Denial::NeedsConfirmation(format!("Confirm {name}?")));
        }
        if tier > Tier::T0 {
            st.in_flight.insert(name.to_string(), now);
            st.recent.insert(name.to_string(), now);
            st.starts.push_back(now);
        }
        Ok(Ticket { inner: i.clone(), name: name.to_string(), generation: i.generation.load(Ordering::SeqCst) })
    }

    /// Wait up to `timeout` for the game window to come to the front (true at once when window
    /// checks are off). For a live run started from the app window: the user clicked in
    /// astromech and needs a moment to switch to Elite. Presses still check the window.
    pub fn wait_foreground(&self, timeout: Duration) -> bool {
        let i = &self.inner;
        if i.cfg.skip_window_checks {
            return true;
        }
        let end = Instant::now() + timeout;
        loop {
            if i.platform.foreground_title().as_deref() == Some(TARGET_WINDOW) {
                return true;
            }
            if Instant::now() >= end || self.is_killed() {
                return false;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    /// Back-compat helper: decide without keeping a ticket.
    pub fn check(&self, name: &str, tier: Tier, origin: Origin) -> Result<(), Denial> {
        self.authorize(name, tier, origin, Instant::now()).map(|_| ())
    }
}

impl Inner {
    fn release_all(&self) {
        let keys: Vec<ScanKey> = std::mem::take(&mut *self.held.lock().unwrap());
        for k in keys.into_iter().rev() {
            let _ = self.platform.send_key(k, true);
        }
    }
}

impl Ticket {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_cancelled(&self) -> bool {
        self.inner.killed.load(Ordering::SeqCst) || self.inner.generation.load(Ordering::SeqCst) != self.generation
    }

    /// Press `key` (an Elite `Key_*` name) with `modifiers` held, for the configured hold.
    pub fn press(&self, key: &str, modifiers: &[String]) -> Result<(), PressError> {
        let i = &self.inner;
        let resolve = |n: &str| match keys::lookup(n) {
            KeyLookup::Ok(k) | KeyLookup::Unverified(k) => Ok(k),
            _ => Err(PressError::UnknownKey(n.to_string())),
        };
        let mods: Vec<ScanKey> = modifiers.iter().map(|m| resolve(m)).collect::<Result<_, _>>()?;
        let k = resolve(key)?;
        if self.is_cancelled() {
            return Err(PressError::Cancelled);
        }
        if !i.cfg.skip_window_checks {
            let fg = i.platform.foreground_title().unwrap_or_default();
            if fg != TARGET_WINDOW {
                return Err(PressError::NotForeground(fg));
            }
            if let Err(e) = i.platform.integrity().verdict() {
                if e.contains("UIPI") || e.contains("cannot read") {
                    return Err(PressError::Privilege(e));
                }
            }
        }
        let down = |k: ScanKey| -> Result<(), PressError> {
            i.platform.send_key(k, false).map_err(PressError::Send)?;
            i.held.lock().unwrap().push(k);
            Ok(())
        };
        let up = |k: ScanKey| {
            let mut h = i.held.lock().unwrap();
            if let Some(p) = h.iter().rposition(|x| *x == k) {
                h.remove(p);
                drop(h);
                let _ = i.platform.send_key(k, true);
            }
        };
        i.platform.begin_precise_timing();
        let r = (|| {
            for m in &mods {
                down(*m)?;
                std::thread::sleep(i.cfg.gap);
            }
            down(k)?;
            // Sleep in small slices so the kill switch can cut a hold short.
            let end = Instant::now() + i.cfg.hold;
            while Instant::now() < end {
                if self.is_cancelled() {
                    break;
                }
                std::thread::sleep(Duration::from_millis(5).min(end.saturating_duration_since(Instant::now())));
            }
            up(k);
            for m in mods.iter().rev() {
                std::thread::sleep(i.cfg.gap);
                up(*m);
            }
            Ok(())
        })();
        i.platform.end_precise_timing();
        if r.is_err() {
            i.release_all();
        }
        r
    }
}

impl Drop for Ticket {
    fn drop(&mut self) {
        self.inner.state.lock().unwrap().in_flight.remove(&self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_input::exec::platform::{GameIntegrity, IntegrityReport};

    #[derive(Default)]
    struct Rec {
        log: Mutex<Vec<(u16, bool)>>,
        title: Mutex<String>,
    }
    impl Platform for Rec {
        fn foreground_title(&self) -> Option<String> {
            Some(self.title.lock().unwrap().clone())
        }
        fn send_key(&self, k: ScanKey, up: bool) -> Result<(), String> {
            self.log.lock().unwrap().push((k.scan, up));
            Ok(())
        }
        fn integrity(&self) -> IntegrityReport {
            IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
        }
    }

    fn setup() -> (Arc<Rec>, Policy) {
        let r = Arc::new(Rec::default());
        *r.title.lock().unwrap() = TARGET_WINDOW.into();
        (r.clone(), Policy::new(r, Config::default()))
    }

    #[test]
    fn tiers_dedupe_and_rate_limit() {
        let (_, p) = setup();
        let t = Instant::now();
        let a = p.authorize("set_landing_gear", Tier::T1, Origin::Voice, t).unwrap();
        assert_eq!(p.authorize("set_landing_gear", Tier::T1, Origin::Voice, t).err(), Some(Denial::Duplicate));
        drop(a);
        assert_eq!(
            p.authorize("set_landing_gear", Tier::T1, Origin::Voice, t + Duration::from_secs(2)).err(),
            Some(Denial::Duplicate),
            "within 3 s"
        );
        assert!(p.authorize("set_landing_gear", Tier::T1, Origin::Voice, t + Duration::from_secs(4)).is_ok());
        assert!(matches!(
            p.authorize("nav.jump", Tier::T2, Origin::Voice, t).err(),
            Some(Denial::NeedsConfirmation(_))
        ));
        assert_eq!(p.authorize("x", Tier::T2, Origin::Cloud, t).err(), Some(Denial::OriginNotAllowed));
        assert_eq!(p.authorize("x", Tier::T3, Origin::Voice, t).err(), Some(Denial::NeverAutomated));
        assert!(p.authorize("state.get", Tier::T0, Origin::Voice, t).is_ok());
        for n in 0..8 {
            let _ = p.authorize(&format!("t{n}"), Tier::T1, Origin::Voice, t + Duration::from_secs(5));
        }
        assert_eq!(
            p.authorize("t9", Tier::T1, Origin::Voice, t + Duration::from_secs(6)).err(),
            Some(Denial::RateLimited)
        );
        assert!(p.authorize("t9", Tier::T1, Origin::Voice, t + Duration::from_secs(30)).is_ok());
    }

    #[test]
    fn press_with_modifier_and_focus_check() {
        let (r, p) = setup();
        let t = p.authorize("set_lights", Tier::T1, Origin::Voice, Instant::now()).unwrap();
        t.press("Key_L", &["Key_LeftShift".into()]).unwrap();
        assert_eq!(*r.log.lock().unwrap(), vec![(0x2A, false), (0x26, false), (0x26, true), (0x2A, true)]);
        *r.title.lock().unwrap() = "Discord".into();
        assert!(matches!(t.press("Key_L", &[]), Err(PressError::NotForeground(_))));
        assert!(matches!(t.press("Key_Pause", &[]), Err(PressError::UnknownKey(_))));
    }

    #[test]
    fn wait_foreground_until_the_game_is_in_front() {
        let (r, p) = setup();
        *r.title.lock().unwrap() = "astromech".into();
        let start = Instant::now();
        assert!(!p.wait_foreground(Duration::from_millis(120)));
        assert!(start.elapsed() >= Duration::from_millis(120));
        let r2 = r.clone();
        let h = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            *r2.title.lock().unwrap() = TARGET_WINDOW.into();
        });
        assert!(p.wait_foreground(Duration::from_secs(2)));
        h.join().unwrap();
        assert!(r.log.lock().unwrap().is_empty(), "waiting presses nothing");
    }

    #[test]
    fn kill_switch_cancels_releases_and_blocks() {
        let (r, p) = setup();
        let t = p.authorize("set_landing_gear", Tier::T1, Origin::Voice, Instant::now()).unwrap();
        let p2 = p.clone();
        let h = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            p2.kill();
        });
        // Long hold: the kill switch fires mid-hold and must release the key.
        let long = Policy::new(r.clone(), Config { hold: Duration::from_millis(500), ..Config::default() });
        let lt = long.authorize("a", Tier::T1, Origin::Voice, Instant::now()).unwrap();
        let l2 = long.clone();
        let h2 = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(50));
            l2.kill();
        });
        let start = Instant::now();
        lt.press("Key_L", &[]).unwrap();
        assert!(start.elapsed() < Duration::from_millis(300), "hold was cut short");
        h.join().unwrap();
        h2.join().unwrap();
        let log = r.log.lock().unwrap().clone();
        assert_eq!(log.iter().filter(|e| !e.1).count(), log.iter().filter(|e| e.1).count(), "every down has an up");
        assert!(t.is_cancelled());
        assert_eq!(t.press("Key_L", &[]), Err(PressError::Cancelled));
        assert_eq!(p.authorize("set_lights", Tier::T1, Origin::Voice, Instant::now()).err(), Some(Denial::KillSwitch));
        p.rearm();
        let t2 = p.authorize("set_lights", Tier::T1, Origin::Voice, Instant::now()).unwrap();
        assert!(!t2.is_cancelled());
    }
}
