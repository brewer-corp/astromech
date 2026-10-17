//! Delivering what the reaction engine ([`astromech_announce`]) fires: speech through the
//! [`SpeechQueue`] (one line at a time, after the current one finishes), toasts for the app
//! window (in the core's `status` reply as `toasts`), and Windows notifications.
//!
//! The pipeline owns the [`Engine`] and hands every journal line and Status.json read to it; it
//! marks the engine live once the startup catch-up is applied (see `pipeline`).

use crate::logging::log;
use crate::speech::Speaker;
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

/// Every ore with its default and current threshold (`null`: not called out), for Settings.
pub fn thresholds_json(t: &Thresholds) -> Value {
    let ores: Vec<Value> = prospector::ORES
        .iter()
        .map(|o| json!({"ore": o.symbol, "name": o.name, "default": o.default, "threshold": t.get(o.symbol)}))
        .collect();
    json!({"ok": true, "ores": ores})
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

    /// A notifier that takes a while, like PowerShell.
    struct Slow(Mutex<Vec<String>>);
    impl Notifier for Slow {
        fn notify(&self, text: &str) {
            std::thread::sleep(Duration::from_millis(200));
            self.0.lock().unwrap().push(text.into());
        }
    }

    #[test]
    fn notifications_run_in_the_background_at_most_one_per_gap() {
        let slow = Arc::new(Slow(Mutex::new(vec![])));
        let n = Background::start(slow.clone(), Duration::from_secs(60)).unwrap();
        let t0 = Instant::now();
        n.notify("First.");
        n.notify("Second.");
        n.notify("Third.");
        assert!(t0.elapsed() < Duration::from_millis(100), "notify blocked for {:?}", t0.elapsed());
        std::thread::sleep(Duration::from_millis(700));
        assert_eq!(*slow.0.lock().unwrap(), ["First."]);
        // After the gap the next one goes.
        let fast = Arc::new(Slow(Mutex::new(vec![])));
        let n = Background::start(fast.clone(), Duration::from_millis(300)).unwrap();
        n.notify("A.");
        std::thread::sleep(Duration::from_millis(600));
        n.notify("B.");
        std::thread::sleep(Duration::from_millis(400));
        assert_eq!(*fast.0.lock().unwrap(), ["A.", "B."]);
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
