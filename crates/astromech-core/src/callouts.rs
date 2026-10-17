//! Delivering what the reaction engine ([`astromech_announce`]) fires: speech through the
//! [`SpeechQueue`] (one line at a time, after the current one finishes), toasts for the app
//! window (in the core's `status` reply as `toasts`), and Windows notifications.
//!
//! The pipeline owns the [`Engine`] and hands every journal line and Status.json read to it; it
//! marks the engine live once the startup catch-up is applied (see `pipeline`).

use crate::logging::log;
use crate::speech::Speaker;
use astromech_announce::arrival::{ShoppingList, Want};
use astromech_announce::prospector::{self, Thresholds};
use astromech_announce::{Engine, Reaction, SpeechQueue};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// Where the prospector thresholds live in `astromech-store` (JSON of
/// [`Thresholds`]). The config file has no write path yet, so Settings stores them here.
pub const PROSPECTOR_KEY: &str = "callouts.prospector.thresholds";

/// The stored prospector thresholds (defaults if none or unreadable, logged).
pub fn load_thresholds(store: &astromech_store::Store) -> Thresholds {
    match store.get(PROSPECTOR_KEY) {
        Ok(Some(v)) => serde_json::from_str(&v).unwrap_or_else(|e| {
            log(&format!("prospector thresholds in the database are unreadable ({e}); using the defaults"));
            Thresholds::default()
        }),
        Ok(None) => Thresholds::default(),
        Err(e) => {
            log(&format!("couldn't read the prospector thresholds ({e}); using the defaults"));
            Thresholds::default()
        }
    }
}

/// Open the database in `data`, creating the folder first (a fresh machine has none yet).
fn open_store(data: &std::path::Path) -> Result<astromech_store::Store, String> {
    std::fs::create_dir_all(data).map_err(|e| format!("creating {}: {e}", data.display()))?;
    astromech_store::Store::open(&data.join("astromech.sqlite")).map_err(|e| format!("opening the database: {e}"))
}

/// The prospector thresholds for Settings, as [`thresholds_json`]: the defaults on a machine
/// whose data folder does not exist yet.
pub fn thresholds_view(data: &std::path::Path) -> Value {
    match open_store(data) {
        Ok(st) => thresholds_json(&load_thresholds(&st)),
        Err(e) => json!({"ok": false, "error": e}),
    }
}

/// What the callout rules share, with the thresholds and shopping list read from the database in
/// `data`. When it will not open the rules use the defaults, and `log` says so.
pub fn shared_from_store(data: &std::path::Path, log: &dyn Fn(&str)) -> astromech_announce::Shared {
    match open_store(data) {
        Ok(st) => astromech_announce::Shared {
            prospector: std::sync::Arc::new(std::sync::RwLock::new(load_thresholds(&st))),
            shopping: std::sync::Arc::new(std::sync::RwLock::new(load_shopping(&st))),
            ..Default::default()
        },
        Err(e) => {
            log(&format!("callouts: {e}; using the default prospector thresholds and an empty shopping list"));
            astromech_announce::Shared::default()
        }
    }
}

/// Every ore with its default and current threshold (`null`: not called out), for Settings.
pub fn thresholds_json(t: &Thresholds) -> Value {
    let ores: Vec<Value> = prospector::ORES
        .iter()
        .map(|o| json!({"ore": o.symbol, "name": o.name, "default": o.default, "threshold": t.get(o.symbol)}))
        .collect();
    json!({"ok": true, "ores": ores})
}

/// Every callout with what is in effect, for Settings → Callouts: `enabled` and `cooldown_s`
/// are the configured values or the callout's own; `default_*` are the callout's own.
pub fn callouts_json(settings: &astromech_announce::Settings) -> Value {
    let list: Vec<Value> = astromech_announce::catalog()
        .iter()
        .map(|i| {
            let s = settings.0.get(i.id).cloned().unwrap_or_default();
            json!({
                "id": i.id,
                "title": i.title,
                "enabled": s.enabled.unwrap_or(i.default_enabled),
                "cooldown_s": s.cooldown_s.unwrap_or(i.default_cooldown_s),
                "default_enabled": i.default_enabled,
                "default_cooldown_s": i.default_cooldown_s,
            })
        })
        .collect();
    json!({"ok": true, "callouts": list})
}

/// A change from Settings: `{"ore": "platinum", "threshold": 25}` (`null` turns it off) or
/// `{"ore": "platinum", "reset": true}`.
pub fn apply_threshold_change(t: &mut Thresholds, text: &str) -> Result<(), String> {
    #[derive(serde::Deserialize)]
    struct Change {
        ore: String,
        #[serde(default)]
        threshold: Option<f64>,
        #[serde(default)]
        reset: bool,
    }
    let c: Change = serde_json::from_str(text).map_err(|e| format!("bad threshold change: {e}"))?;
    if c.reset {
        prospector::ore(&c.ore).ok_or_else(|| format!("unknown ore {:?}", c.ore))?;
        t.reset(&c.ore);
        Ok(())
    } else {
        t.set(&c.ore, c.threshold)
    }
}

/// Apply a Settings change to the stored thresholds; returns the new set.
pub fn change_thresholds(store: &astromech_store::Store, text: &str) -> Result<Thresholds, String> {
    let mut t = load_thresholds(store);
    apply_threshold_change(&mut t, text)?;
    let v = serde_json::to_string(&t).map_err(|e| e.to_string())?;
    store.set(PROSPECTOR_KEY, &v).map_err(|e| format!("saving the prospector thresholds: {e}"))?;
    Ok(t)
}

/// Where the arrival briefing's shopping list lives in `astromech-store`: per ship id, what its
/// Engineering plan still has to collect, each material with the count to hold when done (JSON
/// object of `[{"id", "until"}]` lists). The app sends a ship's list whenever its plan is worked
/// out; the rules compare it with the materials held, so collecting needs no new list.
pub const SHOPPING_KEY: &str = "callouts.arrival.shopping";

type ShipLists = std::collections::BTreeMap<String, Vec<Want>>;

fn ship_lists(store: &astromech_store::Store) -> ShipLists {
    match store.get(SHOPPING_KEY) {
        Ok(Some(v)) => serde_json::from_str(&v).unwrap_or_else(|e| {
            log(&format!("the shopping list in the database is unreadable ({e}); starting empty"));
            ShipLists::new()
        }),
        Ok(None) => ShipLists::new(),
        Err(e) => {
            log(&format!("couldn't read the shopping list ({e}); starting empty"));
            ShipLists::new()
        }
    }
}

/// Every ship's list in one, each material once with the highest count any ship wants (ships
/// in id order, each list in its order).
fn merged(lists: &ShipLists) -> ShoppingList {
    let mut all: Vec<Want> = vec![];
    for w in lists.values().flatten() {
        match all.iter_mut().find(|a| a.id == w.id) {
            Some(a) => a.until = a.until.max(w.until),
            None => all.push(w.clone()),
        }
    }
    ShoppingList(all)
}

/// The stored shopping list (empty if none or unreadable, logged).
pub fn load_shopping(store: &astromech_store::Store) -> ShoppingList {
    merged(&ship_lists(store))
}

/// A plan worked out in the app: `{"ship_id": 7, "materials": [{"id": "protoheatradiators",
/// "until": 12}, ...]}` (an empty list drops the ship). Returns the new merged list.
pub fn change_shopping(store: &astromech_store::Store, text: &str) -> Result<ShoppingList, String> {
    #[derive(serde::Deserialize)]
    struct Change {
        ship_id: i64,
        materials: Vec<Want>,
    }
    let c: Change = serde_json::from_str(text).map_err(|e| format!("bad shopping list: {e}"))?;
    let mut lists = ship_lists(store);
    let materials: Vec<Want> =
        c.materials.into_iter().map(|w| Want { id: w.id.to_ascii_lowercase(), until: w.until }).collect();
    if materials.is_empty() {
        lists.remove(&c.ship_id.to_string());
    } else {
        lists.insert(c.ship_id.to_string(), materials);
    }
    let v = serde_json::to_string(&lists).map_err(|e| e.to_string())?;
    store.set(SHOPPING_KEY, &v).map_err(|e| format!("saving the shopping list: {e}"))?;
    Ok(merged(&lists))
}

/// The engine plus where its reactions go. Lives on the pipeline thread.
pub struct Callouts {
    pub engine: Engine,
    pub deliver: Box<dyn Fn(Vec<Reaction>) + Send>,
}

/// One toast for the app window.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Toast {
    /// Increases by one per toast; the window shows the ones newer than it has seen.
    pub id: u64,
    pub rule: String,
    pub text: String,
    /// Local time, RFC 3339.
    pub time: String,
}

/// The last few toasts.
#[derive(Default)]
pub struct Toasts {
    inner: Mutex<(u64, VecDeque<Toast>)>,
}

const TOASTS_KEPT: usize = 20;

impl Toasts {
    pub fn push(&self, rule: &str, text: &str) {
        let mut g = self.inner.lock().unwrap();
        g.0 += 1;
        let t = Toast {
            id: g.0,
            rule: rule.into(),
            text: text.into(),
            time: chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        };
        g.1.push_back(t);
        while g.1.len() > TOASTS_KEPT {
            g.1.pop_front();
        }
    }

    pub fn recent(&self) -> Vec<Toast> {
        self.inner.lock().unwrap().1.iter().cloned().collect()
    }
}

/// Shows a desktop notification.
pub trait Notifier: Send + Sync {
    fn notify(&self, text: &str);
}

/// Logs the notification (no desktop notifications on this platform).
pub struct LogNotifier;
impl Notifier for LogNotifier {
    fn notify(&self, text: &str) {
        log(&format!("notification: {text}"));
    }
}

/// A Windows toast notification through PowerShell's WinRT bridge (no extra dependencies; the
/// text goes in an environment variable, never into the script). Blocks until PowerShell exits
/// and logs a failure, so run it behind [`Background`]. ⚠ Not yet run on Windows.
#[cfg(windows)]
pub struct WindowsNotifier;
#[cfg(windows)]
impl Notifier for WindowsNotifier {
    fn notify(&self, text: &str) {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        const SCRIPT: &str = "$ErrorActionPreference='Stop';\
            [Windows.UI.Notifications.ToastNotificationManager,Windows.UI.Notifications,ContentType=WindowsRuntime]>$null;\
            $x=[Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02);\
            $t=$x.GetElementsByTagName('text');\
            $t.Item(0).AppendChild($x.CreateTextNode('astromech'))>$null;\
            $t.Item(1).AppendChild($x.CreateTextNode($env:ASTROMECH_NOTIFY))>$null;\
            [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe').Show([Windows.UI.Notifications.ToastNotification]::new($x))";
        let r = std::process::Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", SCRIPT])
            .env("ASTROMECH_NOTIFY", text)
            .stdin(std::process::Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        match r {
            Ok(o) if o.status.success() => {}
            Ok(o) => {
                let err = String::from_utf8_lossy(&o.stderr);
                let first = err.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("no error text");
                log(&format!("notification failed (PowerShell {}): {first}; text: {text}", o.status));
            }
            Err(e) => log(&format!("notification failed (couldn't start PowerShell: {e}); text: {text}")),
        }
    }
}

/// At most one notification per this long; the rest are logged and skipped.
pub const NOTIFY_GAP: Duration = Duration::from_secs(15);

/// Runs a slow notifier on its own thread, one at a time, at most one per `gap`. `notify`
/// never blocks: with a few already waiting, a new one is logged and dropped. The thread ends
/// when this is dropped.
pub struct Background {
    tx: Mutex<std::sync::mpsc::SyncSender<String>>,
}

impl Background {
    pub fn start(inner: Arc<dyn Notifier>, gap: Duration) -> std::io::Result<Self> {
        let (tx, rx) = std::sync::mpsc::sync_channel::<String>(4);
        std::thread::Builder::new().name("notify".into()).spawn(move || {
            let mut last: Option<Instant> = None;
            while let Ok(text) = rx.recv() {
                let now = Instant::now();
                if last.is_some_and(|l| now.saturating_duration_since(l) < gap) {
                    log(&format!("notification skipped (one per {} s): {text}", gap.as_secs()));
                    continue;
                }
                last = Some(now);
                inner.notify(&text);
            }
        })?;
        Ok(Self { tx: Mutex::new(tx) })
    }
}

impl Notifier for Background {
    fn notify(&self, text: &str) {
        if let Err(e) = self.tx.lock().unwrap().try_send(text.into()) {
            log(&format!(
                "notification dropped ({}): {text}",
                if matches!(e, std::sync::mpsc::TrySendError::Full(_)) {
                    "too many waiting"
                } else {
                    "notifier stopped"
                }
            ));
        }
    }
}

/// The platform's notifier behind [`Background`].
pub fn default_notifier() -> std::io::Result<Arc<dyn Notifier>> {
    #[cfg(windows)]
    let inner: Arc<dyn Notifier> = Arc::new(WindowsNotifier);
    #[cfg(not(windows))]
    let inner: Arc<dyn Notifier> = Arc::new(LogNotifier);
    Ok(Arc::new(Background::start(inner, NOTIFY_GAP)?))
}

/// Spoken callouts waiting for the speaker, shared by the pipeline (push) and [`SpeechOut`].
#[derive(Default)]
pub struct Pending {
    q: Mutex<SpeechQueue>,
    cv: Condvar,
}

/// Where reactions go: speech to `pending`, toasts to `toasts`, notifications to `notifier`.
pub fn router(
    pending: Arc<Pending>,
    toasts: Arc<Toasts>,
    notifier: Arc<dyn Notifier>,
) -> impl Fn(Vec<Reaction>) + Send + 'static {
    move |rs: Vec<Reaction>| {
        for r in rs {
            log(&format!("callout {}: {}", r.rule, r.text));
            if r.channels.toast {
                toasts.push(r.rule, &r.text);
            }
            if r.channels.notification {
                notifier.notify(&r.text);
            }
            if r.channels.speech {
                pending.q.lock().unwrap().push(r);
                pending.cv.notify_all();
            }
        }
    }
}

/// The commander talking to astromech: from push-to-talk down until each clip's reply has been
/// handed to the speaker (which is then [`Speaker::busy`]). Callouts wait for it, so none is
/// recorded into a clip or spoken over the reply. The PTT thread calls [`ptt_down`](Self::ptt_down)
/// and [`ptt_up`](Self::ptt_up); a clip sent to speech-to-text is a job, finished when the STT
/// thread drops it after the reply (or the voice tester's result). A lost release or a stuck job stops
/// holding after [`MAX_TURN`].
pub struct VoiceTurn {
    inner: Mutex<Turn>,
}

struct Turn {
    held_since: Option<Instant>,
    jobs: usize,
    job_since: Option<Instant>,
}

/// One queued clip; see [`VoiceTurn::job_queued`].
pub struct TurnJob<'a>(&'a VoiceTurn);

impl Drop for TurnJob<'_> {
    fn drop(&mut self) {
        self.0.job_done();
    }
}

/// The longest one voice turn holds callouts back.
pub const MAX_TURN: Duration = Duration::from_secs(60);

/// The session's voice turn (the PTT and STT threads are Windows-only).
pub static VOICE_TURN: VoiceTurn = VoiceTurn::new();

impl Default for VoiceTurn {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceTurn {
    pub const fn new() -> Self {
        Self { inner: Mutex::new(Turn { held_since: None, jobs: 0, job_since: None }) }
    }

    pub fn ptt_down(&self, now: Instant) {
        self.inner.lock().unwrap().held_since = Some(now);
    }

    /// Call after the clip (if any) was queued with [`job_queued`](Self::job_queued).
    pub fn ptt_up(&self) {
        self.inner.lock().unwrap().held_since = None;
    }

    /// A clip went to speech-to-text. The job lasts until the returned guard is dropped: keep it
    /// with the clip, so a reply, a failed send, a discarded clip or a panic all end it.
    #[must_use]
    pub fn job_queued(&self, now: Instant) -> TurnJob<'_> {
        let mut t = self.inner.lock().unwrap();
        t.jobs += 1;
        t.job_since = Some(now);
        TurnJob(self)
    }

    fn job_done(&self) {
        let mut t = self.inner.lock().unwrap();
        t.jobs = t.jobs.saturating_sub(1);
    }

    pub fn active(&self, now: Instant) -> bool {
        let t = self.inner.lock().unwrap();
        let recent = |since: Option<Instant>| since.is_some_and(|s| now.saturating_duration_since(s) < MAX_TURN);
        recent(t.held_since) || (t.jobs > 0 && recent(t.job_since))
    }
}

/// The thread that speaks queued callouts, one at a time: it waits while the speaker is still
/// talking so a callout doesn't cut off the one before it, and during a [`VoiceTurn`]. Stops
/// when dropped.
pub struct SpeechOut {
    stop: Arc<AtomicBool>,
    pending: Arc<Pending>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl SpeechOut {
    pub fn start(pending: Arc<Pending>, speaker: Arc<dyn Speaker>, turn: &'static VoiceTurn) -> std::io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let (stop2, p) = (stop.clone(), pending.clone());
        let thread = std::thread::Builder::new().name("callouts".into()).spawn(move || loop {
            let next = {
                let mut q = p.q.lock().unwrap();
                loop {
                    if stop2.load(Ordering::SeqCst) {
                        return;
                    }
                    if !q.is_empty() && !speaker.busy() && !turn.active(Instant::now()) {
                        if let Some(r) = q.pop(Instant::now()) {
                            break r;
                        }
                    }
                    let wait = if q.is_empty() { Duration::from_millis(250) } else { Duration::from_millis(50) };
                    q = p.cv.wait_timeout(q, wait).unwrap().0;
                }
            };
            speaker.say(&next.text);
        })?;
        Ok(Self { stop, pending, thread: Some(thread) })
    }
}

impl Drop for SpeechOut {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        self.pending.cv.notify_all();
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::speech::RecordingSpeaker;
    use astromech_announce::{Channels, Priority};

    fn reaction(rule: &'static str, text: &str, priority: Priority, channels: Channels) -> Reaction {
        Reaction { rule, text: text.into(), priority, channels, at: Instant::now() }
    }

    struct Recorded(Mutex<Vec<String>>);
    impl Notifier for Recorded {
        fn notify(&self, text: &str) {
            self.0.lock().unwrap().push(text.into());
        }
    }

    #[test]
    fn routes_each_channel_and_speaks_highest_priority_first() {
        let pending = Arc::new(Pending::default());
        let toasts = Arc::new(Toasts::default());
        let notes = Arc::new(Recorded(Mutex::new(vec![])));
        let route = router(pending.clone(), toasts.clone(), notes.clone());
        // Queued before the speaker thread starts, so the order is the queue's.
        route(vec![
            reaction("a", "Low line.", Priority::Low, Channels::SPEECH),
            reaction("b", "Toast only.", Priority::Normal, Channels::TOAST),
            reaction("c", "High line.", Priority::High, Channels::SPEECH | Channels::NOTIFICATION),
        ]);
        assert_eq!(toasts.recent().iter().map(|t| (t.id, t.text.as_str())).collect::<Vec<_>>(), [(1, "Toast only.")]);
        assert_eq!(*notes.0.lock().unwrap(), ["High line."]);
        let speaker = Arc::new(RecordingSpeaker::default());
        let out = SpeechOut::start(pending, speaker.clone(), Box::leak(Box::default())).unwrap();
        let end = Instant::now() + Duration::from_secs(5);
        while speaker.said.lock().unwrap().len() < 2 && Instant::now() < end {
            std::thread::sleep(Duration::from_millis(10));
        }
        drop(out);
        assert_eq!(speaker.take(), ["High line.", "Low line."]);
    }

    /// Records lines; reports busy while `busy` is set.
    #[derive(Default)]
    struct BusySpeaker {
        busy: AtomicBool,
        said: Mutex<Vec<String>>,
    }
    impl Speaker for BusySpeaker {
        fn say(&self, text: &str) {
            self.said.lock().unwrap().push(text.into());
        }
        fn busy(&self) -> bool {
            self.busy.load(Ordering::SeqCst)
        }
    }

    fn said_within(s: &BusySpeaker, n: usize, wait: Duration) -> Vec<String> {
        let end = Instant::now() + wait;
        while s.said.lock().unwrap().len() < n && Instant::now() < end {
            std::thread::sleep(Duration::from_millis(10));
        }
        s.said.lock().unwrap().clone()
    }

    fn queue_one(pending: &Arc<Pending>) {
        let route = router(pending.clone(), Arc::new(Toasts::default()), Arc::new(LogNotifier));
        route(vec![reaction("a", "Callout.", Priority::Normal, Channels::SPEECH)]);
    }

    #[test]
    fn waits_while_the_speaker_is_busy() {
        let pending = Arc::new(Pending::default());
        let speaker = Arc::new(BusySpeaker::default());
        speaker.busy.store(true, Ordering::SeqCst);
        let out = SpeechOut::start(pending.clone(), speaker.clone(), Box::leak(Box::default())).unwrap();
        queue_one(&pending);
        assert!(said_within(&speaker, 1, Duration::from_millis(300)).is_empty());
        speaker.busy.store(false, Ordering::SeqCst);
        assert_eq!(said_within(&speaker, 1, Duration::from_secs(5)), ["Callout."]);
        drop(out);
    }

    #[test]
    fn waits_while_push_to_talk_is_held_and_until_the_reply_is_queued() {
        let turn: &'static VoiceTurn = Box::leak(Box::default());
        let pending = Arc::new(Pending::default());
        let speaker = Arc::new(BusySpeaker::default());
        let out = SpeechOut::start(pending.clone(), speaker.clone(), turn).unwrap();
        // PTT down: the barge-in empties the speaker, so it is not busy, but the turn holds.
        turn.ptt_down(Instant::now());
        queue_one(&pending);
        assert!(said_within(&speaker, 1, Duration::from_millis(300)).is_empty());
        // Released; the clip is with speech-to-text.
        let job = turn.job_queued(Instant::now());
        turn.ptt_up();
        assert!(said_within(&speaker, 1, Duration::from_millis(300)).is_empty());
        // The reply is handed to the speaker (busy), then the job ends: still waiting.
        speaker.busy.store(true, Ordering::SeqCst);
        drop(job);
        assert!(said_within(&speaker, 1, Duration::from_millis(300)).is_empty());
        speaker.busy.store(false, Ordering::SeqCst);
        assert_eq!(said_within(&speaker, 1, Duration::from_secs(5)), ["Callout."]);
        drop(out);
    }

    #[test]
    fn a_voice_turn_ends_with_its_jobs_or_after_the_limit() {
        let t = VoiceTurn::new();
        let t0 = Instant::now();
        assert!(!t.active(t0));
        // A clip gated out (too short, too quiet): down then up, no job.
        t.ptt_down(t0);
        assert!(t.active(t0));
        t.ptt_up();
        assert!(!t.active(t0));
        // Two clips in flight: held until both are done.
        let a = t.job_queued(t0);
        let b = t.job_queued(t0);
        drop(a);
        assert!(t.active(t0));
        drop(b);
        assert!(!t.active(t0));
        // A panic while handling a clip, or a clip still queued when the STT thread dies, ends
        // its job.
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send(t.job_queued(t0)).unwrap();
        tx.send(t.job_queued(t0)).unwrap();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _job = rx.recv().unwrap();
            panic!("speech-to-text crashed");
        }));
        assert!(r.is_err());
        assert!(!t.active(t0));
        // A lost release or a stuck job stops holding after MAX_TURN.
        t.ptt_down(t0);
        let stuck = t.job_queued(t0);
        assert!(!t.active(t0 + MAX_TURN));
        drop(stuck);
    }

    /// A notifier that takes a while, like PowerShell. Records what was delivered and when.
    struct Slow(Mutex<Vec<(String, Instant)>>);
    impl Slow {
        fn new() -> Arc<Self> {
            Arc::new(Self(Mutex::new(vec![])))
        }
        fn texts(&self) -> Vec<String> {
            self.0.lock().unwrap().iter().map(|(t, _)| t.clone()).collect()
        }
        /// Polls until `want` is delivered, or fails after a generous deadline so a loaded
        /// runner can't flake the test.
        fn wait_for(&self, want: &[&str]) {
            let deadline = Instant::now() + Duration::from_secs(10);
            while self.texts() != want {
                assert!(Instant::now() < deadline, "timed out; delivered {:?}, wanted {want:?}", self.texts());
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
    impl Notifier for Slow {
        fn notify(&self, text: &str) {
            std::thread::sleep(Duration::from_millis(200));
            self.0.lock().unwrap().push((text.into(), Instant::now()));
        }
    }

    #[test]
    fn notifications_run_in_the_background_at_most_one_per_gap() {
        // notify never blocks the caller, and the rest inside the gap are dropped.
        let slow = Slow::new();
        let n = Background::start(slow.clone(), Duration::from_secs(60)).unwrap();
        let t0 = Instant::now();
        n.notify("First.");
        n.notify("Second.");
        n.notify("Third.");
        assert!(t0.elapsed() < Duration::from_millis(100), "notify blocked for {:?}", t0.elapsed());
        slow.wait_for(&["First."]);
        // A negative check, so it has to wait: give the others time to (wrongly) go.
        std::thread::sleep(Duration::from_millis(600));
        assert_eq!(slow.texts(), ["First."]);

        // After the gap the next one goes. Measure the gap from A's delivery, which is never
        // earlier than the start of A that the notifier measures from.
        let gap = Duration::from_millis(300);
        let fast = Slow::new();
        let n = Background::start(fast.clone(), gap).unwrap();
        n.notify("A.");
        fast.wait_for(&["A."]);
        let a_at = fast.0.lock().unwrap()[0].1;
        while a_at.elapsed() < gap {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(a_at.elapsed() >= gap);
        n.notify("B.");
        fast.wait_for(&["A.", "B."]);
    }

    #[test]
    fn prospector_thresholds_are_stored_and_changed_from_settings() {
        let store = astromech_store::Store::in_memory().unwrap();
        assert_eq!(load_thresholds(&store), Thresholds::default());
        let t = change_thresholds(&store, r#"{"ore":"platinum","threshold":35}"#).unwrap();
        assert_eq!(t.get("platinum"), Some(35.0));
        change_thresholds(&store, r#"{"ore":"painite","threshold":null}"#).unwrap();
        let t = load_thresholds(&store);
        assert_eq!((t.get("platinum"), t.get("painite")), (Some(35.0), None));
        let v = thresholds_json(&t);
        let plat = v["ores"].as_array().unwrap().iter().find(|o| o["ore"] == "platinum").unwrap().clone();
        assert_eq!((plat["default"].as_f64(), plat["threshold"].as_f64()), (Some(20.0), Some(35.0)));
        change_thresholds(&store, r#"{"ore":"platinum","reset":true}"#).unwrap();
        assert_eq!(load_thresholds(&store).get("platinum"), Some(20.0));
        // Bad input is refused and nothing is stored.
        assert!(change_thresholds(&store, r#"{"ore":"platinum","threshold":250}"#).is_err());
        assert!(change_thresholds(&store, r#"{"ore":"iron","threshold":5}"#).is_err());
        assert!(change_thresholds(&store, "not json").is_err());
        store.set(PROSPECTOR_KEY, "{broken").unwrap();
        assert_eq!(load_thresholds(&store), Thresholds::default());
    }

    #[test]
    fn thresholds_open_on_a_fresh_machine_and_a_broken_store_is_logged() {
        let d = tempfile::tempdir().unwrap();
        // No data folder yet: Settings gets the defaults, not "opening the database".
        let data = d.path().join("not").join("yet");
        let v = thresholds_view(&data);
        assert_eq!(v["ok"], true, "{v}");
        let plat = v["ores"].as_array().unwrap().iter().find(|o| o["ore"] == "platinum").unwrap().clone();
        assert_eq!(plat["threshold"].as_f64(), Some(20.0));
        assert!(data.join("astromech.sqlite").exists());
        // A store that will not open: the defaults, and a line in the log saying so.
        let blocked = d.path().join("file");
        std::fs::write(&blocked, "x").unwrap();
        let logged = std::sync::Mutex::new(vec![]);
        let shared = shared_from_store(&blocked.join("data"), &|m: &str| logged.lock().unwrap().push(m.to_string()));
        assert_eq!(*shared.prospector.read().unwrap(), Thresholds::default());
        let logged = logged.into_inner().unwrap();
        assert!(logged.len() == 1 && logged[0].contains("default prospector thresholds"), "{logged:?}");
        // One that opens logs nothing and reads what Settings stored.
        change_thresholds(&open_store(&data).unwrap(), r#"{"ore":"platinum","threshold":35}"#).unwrap();
        let shared = shared_from_store(&data, &|m: &str| panic!("{m}"));
        assert_eq!(shared.prospector.read().unwrap().get("platinum"), Some(35.0));
    }

    #[test]
    fn the_shopping_list_is_kept_per_ship_and_merged() {
        let store = astromech_store::Store::in_memory().unwrap();
        assert_eq!(load_shopping(&store), ShoppingList::default());
        let ids = |l: &ShoppingList| l.0.iter().map(|w| (w.id.clone(), w.until)).collect::<Vec<_>>();
        change_shopping(
            &store,
            r#"{"ship_id":7,"materials":[{"id":"ProtoHeatRadiators","until":12},{"id":"dataminedwake","until":30}]}"#,
        )
        .unwrap();
        let l = change_shopping(
            &store,
            r#"{"ship_id":3,"materials":[{"id":"dataminedwake","until":45},{"id":"militarygradealloys","until":6}]}"#,
        )
        .unwrap();
        // Each material once, at the highest count wanted.
        assert_eq!(
            ids(&l),
            [("dataminedwake".into(), 45), ("militarygradealloys".into(), 6), ("protoheatradiators".into(), 12)]
        );
        // A ship's new plan replaces its old list; an empty one drops it.
        change_shopping(&store, r#"{"ship_id":3,"materials":[]}"#).unwrap();
        assert_eq!(ids(&load_shopping(&store)), [("protoheatradiators".into(), 12), ("dataminedwake".into(), 30)]);
        assert!(change_shopping(&store, "not json").is_err());
        store.set(SHOPPING_KEY, "{broken").unwrap();
        assert_eq!(load_shopping(&store), ShoppingList::default());
    }

    #[test]
    fn keeps_the_last_toasts() {
        let t = Toasts::default();
        for i in 0..25 {
            t.push("r", &format!("{i}"));
        }
        let r = t.recent();
        assert_eq!(r.len(), TOASTS_KEPT);
        assert_eq!((r[0].id, r.last().unwrap().id), (6, 25));
    }
}
