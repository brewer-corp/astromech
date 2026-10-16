//! Delivering what the reaction engine ([`astromech_announce`]) fires: speech through the
//! [`SpeechQueue`] (one line at a time, after the current one finishes), toasts for the app
//! window (in the core's `status` reply as `toasts`), and Windows notifications.
//!
//! The pipeline owns the [`Engine`] and hands every journal line and Status.json read to it; it
//! marks the engine live once the startup catch-up is applied (see `pipeline`).

use crate::logging::log;
use crate::speech::Speaker;
use astromech_announce::{Engine, Reaction, SpeechQueue};
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

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
/// text goes in an environment variable, never into the script). ⚠ Not yet run on Windows.
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
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
        if let Err(e) = r {
            log(&format!("notification failed ({e}): {text}"));
        }
    }
}

pub fn default_notifier() -> Arc<dyn Notifier> {
    #[cfg(windows)]
    {
        Arc::new(WindowsNotifier)
    }
    #[cfg(not(windows))]
    {
        Arc::new(LogNotifier)
    }
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
/// and [`ptt_up`](Self::ptt_up); a clip sent to speech-to-text is a job, finished by the STT
/// thread after the reply (or the voice tester's result). A lost release or a stuck job stops
/// holding after [`MAX_TURN`].
pub struct VoiceTurn {
    inner: Mutex<Turn>,
}

struct Turn {
    held_since: Option<Instant>,
    jobs: usize,
    job_since: Option<Instant>,
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

    pub fn job_queued(&self, now: Instant) {
        let mut t = self.inner.lock().unwrap();
        t.jobs += 1;
        t.job_since = Some(now);
    }

    pub fn job_done(&self) {
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
        turn.job_queued(Instant::now());
        turn.ptt_up();
        assert!(said_within(&speaker, 1, Duration::from_millis(300)).is_empty());
        // The reply is handed to the speaker (busy), then the job ends: still waiting.
        speaker.busy.store(true, Ordering::SeqCst);
        turn.job_done();
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
        t.job_queued(t0);
        t.job_queued(t0);
        t.job_done();
        assert!(t.active(t0));
        t.job_done();
        assert!(!t.active(t0));
        t.job_done(); // an extra done doesn't go below zero
                      // A lost release or a stuck job stops holding after MAX_TURN.
        t.ptt_down(t0);
        t.job_queued(t0);
        assert!(!t.active(t0 + MAX_TURN));
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
