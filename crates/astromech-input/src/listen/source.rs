//! The process's one source of input: one Raw Input registration and one XInput poller, shared
//! by every listener in the process (push-to-talk and the kill switch, `--listen`, the window's
//! press listener).
//!
//! Windows keeps only the last `RegisterRawInputDevices` call per device class in a process, so
//! a second registration would take WM_INPUT away from the first (ARCHITECTURE §5.10a). The
//! source starts on its first [`Hub::subscribe`] and then runs for the life of the process.
//! Each subscriber gets its own channel and every Device and Sample message, so one that falls
//! behind or is dropped never blocks the others.

use crate::listen::model::{DeviceMeta, Sample};
use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

/// One message from the source.
#[derive(Debug, Clone)]
pub enum Msg {
    /// A device as first seen (or seen again after it was unplugged).
    Device(DeviceMeta),
    Sample(Sample),
}

/// What produces the messages: Raw Input and XInput on Windows, a fake in tests.
pub trait Backend: Send + Sync + 'static {
    /// Start sending into `tx`, with each sample's `t_ms` counted from `epoch`. Returns the
    /// status line (what registered, what didn't).
    fn start(&self, tx: Sender<Msg>, epoch: Instant) -> String;
    /// Register again, taking the device classes back from a later registration in this process.
    fn restart(&self) -> Result<(), String>;
}

/// A subscriber's end.
pub struct Subscription {
    pub rx: Receiver<Msg>,
    /// What every sample's `t_ms` counts from: compare `epoch.elapsed()` with it.
    pub epoch: Instant,
    /// The backend's status line from when the source started.
    pub status: String,
}

struct Started {
    epoch: Instant,
    status: String,
}

#[derive(Default)]
struct Fanout {
    subscribers: Vec<Sender<Msg>>,
    /// Every device announced so far, by key, so a later subscriber knows them too.
    devices: BTreeMap<String, DeviceMeta>,
    /// The backend stopped sending: new subscribers are disconnected at once.
    closed: bool,
}

/// The fan-out from one backend to any number of subscribers.
pub struct Hub<B: Backend> {
    backend: B,
    started: OnceLock<Started>,
    fanout: Arc<Mutex<Fanout>>,
}

impl<B: Backend> Hub<B> {
    pub fn new(backend: B) -> Self {
        Self { backend, started: OnceLock::new(), fanout: Arc::default() }
    }

    /// A new receiver of every message from now on, after a Device message for each device the
    /// source has already announced. The first call starts the backend.
    pub fn subscribe(&self) -> Subscription {
        let (tx, rx) = channel();
        {
            let mut f = lock(&self.fanout);
            if !f.closed {
                for d in f.devices.values() {
                    let _ = tx.send(Msg::Device(d.clone()));
                }
                f.subscribers.push(tx);
            }
        }
        // Subscribed before the backend starts, so the first subscriber misses nothing.
        let s = self.started.get_or_init(|| {
            let epoch = Instant::now();
            let (btx, brx) = channel();
            let fanout = self.fanout.clone();
            std::thread::Builder::new()
                .name("input-fanout".into())
                .spawn(move || forward(brx, &fanout))
                .expect("spawn input fan-out thread");
            let status = self.backend.start(btx, epoch);
            Started { epoch, status }
        });
        Subscription { rx, epoch: s.epoch, status: s.status.clone() }
    }

    /// Register the backend again. Before the first subscribe this does nothing: that start
    /// registers.
    pub fn restart(&self) -> Result<(), String> {
        match self.started.get() {
            Some(_) => self.backend.restart(),
            None => Ok(()),
        }
    }
}

fn lock(m: &Mutex<Fanout>) -> std::sync::MutexGuard<'_, Fanout> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn forward(rx: Receiver<Msg>, fanout: &Mutex<Fanout>) {
    while let Ok(m) = rx.recv() {
        let mut f = lock(fanout);
        if let Msg::Device(d) = &m {
            f.devices.insert(d.key.clone(), d.clone());
        }
        // Unbounded channels: a send never waits. A dropped subscriber's send fails; forget it.
        f.subscribers.retain(|s| s.send(m.clone()).is_ok());
    }
    // Every backend thread has ended: disconnect the subscribers, as their own listener's
    // channel did before the source was shared.
    let mut f = lock(fanout);
    f.closed = true;
    f.subscribers.clear();
}

#[cfg(windows)]
mod windows {
    use super::{Backend, Hub, Msg, Subscription};
    use crate::listen::win::{rawinput, xinput};
    use std::sync::mpsc::Sender;
    use std::sync::{Mutex, OnceLock};
    use std::time::Instant;

    /// Raw Input (keyboards, mice, sticks, pads) and XInput (Xbox pads).
    #[derive(Default)]
    struct Win {
        sink: Mutex<Option<rawinput::Sink>>,
    }

    impl Backend for Win {
        fn start(&self, tx: Sender<Msg>, epoch: Instant) -> String {
            let (sink, reg) = rawinput::start_sink(tx.clone(), epoch);
            *self.sink.lock().unwrap_or_else(|e| e.into_inner()) = sink;
            let xi = xinput::start(tx, epoch);
            format!(
                "Raw Input: {}; XInput: {xi}",
                match reg {
                    Ok(()) => "ok".to_string(),
                    Err(e) => e,
                }
            )
        }

        fn restart(&self) -> Result<(), String> {
            match self.sink.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
                Some(s) => s.register(),
                None => Err("the Raw Input window was never created".into()),
            }
        }
    }

    static SOURCE: OnceLock<Hub<Win>> = OnceLock::new();

    fn source() -> &'static Hub<Win> {
        SOURCE.get_or_init(|| Hub::new(Win::default()))
    }

    /// Subscribe to the process's input. The first call starts Raw Input and XInput.
    pub fn subscribe() -> Subscription {
        source().subscribe()
    }

    /// Register Raw Input again, after something else in the process registered (tao does when
    /// Tauri builds its event loop). Does nothing before the first [`subscribe`].
    pub fn restart() -> Result<(), String> {
        source().restart()
    }
}
#[cfg(windows)]
pub use windows::{restart, subscribe};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listen::model::{DevKind, SampleData};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    /// Hands its sender to the test, which then plays the backend.
    #[derive(Default)]
    struct Fake {
        starts: AtomicUsize,
        restarts: AtomicUsize,
        tx: Mutex<Option<Sender<Msg>>>,
    }

    impl Backend for Arc<Fake> {
        fn start(&self, tx: Sender<Msg>, _epoch: Instant) -> String {
            self.starts.fetch_add(1, Ordering::SeqCst);
            *self.tx.lock().unwrap() = Some(tx);
            "fake: ok".into()
        }
        fn restart(&self) -> Result<(), String> {
            self.restarts.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    impl Fake {
        fn send(&self, m: Msg) {
            self.tx.lock().unwrap().as_ref().expect("started").send(m).unwrap();
        }
        fn stop(&self) {
            *self.tx.lock().unwrap() = None;
        }
    }

    fn hub() -> (Hub<Arc<Fake>>, Arc<Fake>) {
        let f = Arc::new(Fake::default());
        (Hub::new(f.clone()), f)
    }

    fn key(n: u16) -> Msg {
        Msg::Sample(Sample {
            t_ms: n as f64,
            dev: "Keyboard:injected".into(),
            fg: String::new(),
            injected: true,
            data: SampleData::Key { vkey: n, scan: n, e0: false, down: true, name: format!("key {n}") },
        })
    }

    fn device(k: &str) -> Msg {
        Msg::Device(DeviceMeta {
            key: k.into(),
            kind: DevKind::Hid,
            vid: 1,
            pid: 2,
            product: k.into(),
            usage: 4,
            buttons_declared: 0,
            axes_declared: vec![],
            hats_declared: 0,
        })
    }

    fn label(m: &Msg) -> String {
        match m {
            Msg::Device(d) => format!("device {}", d.key),
            Msg::Sample(s) => match &s.data {
                SampleData::Key { vkey, .. } => format!("key {vkey}"),
                _ => "other".into(),
            },
        }
    }

    fn take(rx: &Receiver<Msg>, n: usize) -> Vec<String> {
        (0..n).map(|_| label(&rx.recv_timeout(Duration::from_secs(5)).expect("a message"))).collect()
    }

    #[test]
    fn two_subscribers_each_see_every_message_in_order() {
        let (h, f) = hub();
        let a = h.subscribe();
        let b = h.subscribe();
        f.send(device("stick"));
        for n in 1..=50 {
            f.send(key(n));
        }
        let want: Vec<String> =
            std::iter::once("device stick".to_string()).chain((1..=50).map(|n| format!("key {n}"))).collect();
        assert_eq!(take(&a.rx, 51), want);
        assert_eq!(take(&b.rx, 51), want);
        assert_eq!(a.epoch, b.epoch, "one time base for every subscriber");
        assert_eq!(a.status, "fake: ok");
    }

    #[test]
    fn dropping_one_subscriber_leaves_the_other_receiving() {
        let (h, f) = hub();
        let a = h.subscribe();
        let b = h.subscribe();
        f.send(key(1));
        drop(a);
        f.send(key(2));
        f.send(key(3));
        assert_eq!(take(&b.rx, 3), ["key 1", "key 2", "key 3"]);
        assert_eq!(lock(&h.fanout).subscribers.len(), 1, "the dropped one is forgotten");
    }

    #[test]
    fn one_that_falls_behind_never_holds_up_the_others() {
        let (h, f) = hub();
        let slow = h.subscribe();
        let b = h.subscribe();
        for n in 1..=1000 {
            f.send(key(n));
        }
        assert_eq!(take(&b.rx, 1000).last().unwrap(), "key 1000");
        // The slow one, not read until now, still gets everything.
        assert_eq!(take(&slow.rx, 1000).last().unwrap(), "key 1000");
    }

    #[test]
    fn the_source_starts_once_on_the_first_subscribe() {
        let (h, f) = hub();
        assert_eq!(f.starts.load(Ordering::SeqCst), 0, "not started by being created");
        h.restart().unwrap();
        assert_eq!(f.restarts.load(Ordering::SeqCst), 0, "nothing to register again before the start");
        let a = h.subscribe();
        assert_eq!(f.starts.load(Ordering::SeqCst), 1);
        let b = h.subscribe();
        drop(a);
        drop(b);
        let _c = h.subscribe();
        assert_eq!(f.starts.load(Ordering::SeqCst), 1, "started once, for the life of the hub");
        h.restart().unwrap();
        assert_eq!(f.restarts.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn a_later_subscriber_is_told_the_devices_already_seen() {
        let (h, f) = hub();
        let a = h.subscribe();
        f.send(device("stick"));
        f.send(key(1));
        f.send(device("pad"));
        assert_eq!(take(&a.rx, 3), ["device stick", "key 1", "device pad"]);
        let b = h.subscribe();
        f.send(key(2));
        assert_eq!(take(&b.rx, 3), ["device pad", "device stick", "key 2"]);
    }

    #[test]
    fn subscribers_are_disconnected_when_the_backend_stops() {
        let (h, f) = hub();
        let a = h.subscribe();
        f.send(key(1));
        f.stop();
        assert_eq!(take(&a.rx, 1), ["key 1"]);
        assert!(a.rx.recv_timeout(Duration::from_secs(5)).is_err());
        let late = h.subscribe();
        assert!(late.rx.recv_timeout(Duration::from_secs(5)).is_err());
    }
}
