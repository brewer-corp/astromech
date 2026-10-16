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

/// The thread that speaks queued callouts, one at a time: it waits while the speaker is still
/// talking so a callout doesn't cut off the one before it. Stops when dropped.
pub struct SpeechOut {
    stop: Arc<AtomicBool>,
    pending: Arc<Pending>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl SpeechOut {
    pub fn start(pending: Arc<Pending>, speaker: Arc<dyn Speaker>) -> std::io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let (stop2, p) = (stop.clone(), pending.clone());
        let thread = std::thread::Builder::new().name("callouts".into()).spawn(move || loop {
            let next = {
                let mut q = p.q.lock().unwrap();
                loop {
                    if stop2.load(Ordering::SeqCst) {
                        return;
                    }
                    if !q.is_empty() && !speaker.busy() {
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
        let out = SpeechOut::start(pending, speaker.clone()).unwrap();
        let end = Instant::now() + Duration::from_secs(5);
        while speaker.said.lock().unwrap().len() < 2 && Instant::now() < end {
            std::thread::sleep(Duration::from_millis(10));
        }
        drop(out);
        assert_eq!(speaker.take(), ["High line.", "Low line."]);
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
