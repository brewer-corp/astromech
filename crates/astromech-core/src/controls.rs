//! The PTT and kill-switch buttons, read with the S7 Raw Input listener (RIDEV_INPUTSINK,
//! read-only; S7 showed Elite doesn't take exclusive input from it).
//!
//! [`Mapper`] is the platform-independent part (listener samples → [`Control`]s, with the
//! diagnostics); [`start`] wires it to Raw Input + XInput on Windows.

use crate::config::Button;
use astromech_input::listen::model::{DeviceMeta, Event, Sample};
use astromech_input::listen::tracker::Processor;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    PttDown,
    PttUp,
    /// Kill-switch pressed.
    KillDown,
    /// Kill-switch released after being held this long.
    KillUp(Duration),
    /// Paste button pressed (paste into the galaxy map's search box).
    PasteDown,
}

/// How many unmatched presses on the PTT device are reported (without --debug-input).
pub const UNMATCHED_REPORTS: u32 = 5;
/// Default PTT debounce: a release followed by a press within this window is contact bounce
/// (or a double report) and is ignored, so one push gives one clip.
pub const DEBOUNCE_MS: f64 = 150.0;
/// The controls thread's idle tick (ms): axis movements are debounced on it.
const IDLE_TICK_MS: f64 = 100.0;
/// The shortest sleep when a deadline is due (ms), so an early wake-up can't spin.
const MIN_WAIT_MS: f64 = 1.0;

pub struct Mapper {
    p: Processor,
    ptt: Option<Button>,
    kill: Option<Button>,
    kill_at: Option<Instant>,
    /// Optional paste button (`controls.paste`).
    pub paste: Option<Button>,
    /// Log every button event (`--debug-input`).
    pub debug: bool,
    unmatched: u32,
    log: Box<dyn Fn(&str) + Send>,
    /// PTT debounce window (ms).
    pub debounce_ms: f64,
    ptt_down: bool,
    /// Time of a release that hasn't been reported yet (waiting out the debounce window).
    pending_up: Option<f64>,
    /// Bounces ignored so far (for the log).
    pub bounces: u64,
}

fn device_of(e: &Event) -> String {
    match e.kind.as_str() {
        "key" => "Keyboard".into(),
        "mouse" => "Mouse".into(),
        "xinput" => "GamePad".into(),
        _ => e.elite_id.clone().unwrap_or_else(|| format!("unknown device {}", e.dev)),
    }
}

impl Mapper {
    pub fn new(ptt: Option<Button>, kill: Option<Button>, debug: bool, log: impl Fn(&str) + Send + 'static) -> Self {
        let mut p = Processor::new(astromech_input::ELITE_TITLE, None, true);
        // The first report from an idle joystick is the press itself (see Processor docs).
        p.initial_presses = true;
        Self {
            p,
            ptt,
            kill,
            kill_at: None,
            paste: None,
            debug,
            unmatched: 0,
            log: Box::new(log),
            debounce_ms: DEBOUNCE_MS,
            ptt_down: false,
            pending_up: None,
            bounces: 0,
        }
    }

    pub fn add_device(&mut self, d: DeviceMeta) {
        self.p.add_device(d);
    }

    pub fn sample(&mut self, s: &Sample) -> Vec<Control> {
        let mut out = self.flush(s.t_ms);
        let evs = self.p.process(s);
        out.extend(self.handle(evs));
        out
    }

    /// Call periodically (and it is called on every sample): reports a release once the
    /// debounce window has passed with no new press.
    pub fn tick(&mut self, now_ms: f64) -> Vec<Control> {
        let mut out = self.flush(now_ms);
        let evs = self.p.tick(now_ms);
        out.extend(self.handle(evs));
        out
    }

    /// How long the controls thread may sleep before it must call [`Mapper::tick`] (ms): the idle
    /// tick the axis debounce needs, but never past the end of a pending release's debounce
    /// window, so the release is reported when the window ends and not at the tick after it.
    pub fn wait_ms(&self, now_ms: f64) -> f64 {
        match self.pending_up {
            Some(t) => (t + self.debounce_ms - now_ms).clamp(MIN_WAIT_MS, IDLE_TICK_MS),
            None => IDLE_TICK_MS,
        }
    }

    fn flush(&mut self, now_ms: f64) -> Vec<Control> {
        match self.pending_up {
            Some(t) if now_ms - t >= self.debounce_ms => {
                self.pending_up = None;
                self.ptt_down = false;
                vec![Control::PttUp]
            }
            _ => vec![],
        }
    }

    fn ptt_edge(&mut self, down: bool, t: f64, out: &mut Vec<Control>) {
        if down {
            if let Some(up) = self.pending_up.take() {
                self.bounces += 1;
                let msg = format!("PTT bounce ignored (released and pressed again within {:.0} ms)", t - up);
                if self.debug || self.bounces <= UNMATCHED_REPORTS as u64 {
                    (self.log)(&msg);
                }
            } else if !self.ptt_down {
                self.ptt_down = true;
                out.push(Control::PttDown);
            }
        } else if self.ptt_down && self.pending_up.is_none() {
            self.pending_up = Some(t);
        }
    }

    fn handle(&mut self, evs: Vec<Event>) -> Vec<Control> {
        // We only react; don't keep a history.
        self.p.events.clear();
        self.p.anomalies.clear();
        let mut out = vec![];
        for e in evs {
            let Some(down) = e.down else { continue };
            let is_ptt = self.ptt.as_ref().map(|b| b.matches(&e)).unwrap_or(false);
            let is_kill = self.kill.as_ref().map(|b| b.matches(&e)).unwrap_or(false);
            let is_paste = !is_ptt && !is_kill && self.paste.as_ref().map(|b| b.matches(&e)).unwrap_or(false);
            let name = e.elite_name.clone().unwrap_or_else(|| e.control.clone());
            if self.debug {
                (self.log)(&format!(
                    "input: {} {name} on {} ({}) {}{}",
                    e.kind,
                    device_of(&e),
                    e.device,
                    if down { "down" } else { "up" },
                    if is_ptt {
                        " -> PTT"
                    } else if is_kill {
                        " -> kill switch"
                    } else if is_paste {
                        " -> paste"
                    } else {
                        ""
                    }
                ));
            }
            if is_ptt {
                self.ptt_edge(down, e.t_ms, &mut out);
            } else if is_paste {
                if down {
                    out.push(Control::PasteDown);
                }
            } else if down && !is_kill {
                if let Some(p) = &self.ptt {
                    let same_device = device_of(&e).eq_ignore_ascii_case(&p.device);
                    if same_device && self.unmatched < UNMATCHED_REPORTS && !self.debug {
                        self.unmatched += 1;
                        (self.log)(&format!("{name} on {} pressed; PTT is {}", device_of(&e), p.key));
                    }
                }
            }
            if is_kill {
                if down {
                    self.kill_at.get_or_insert_with(Instant::now);
                    out.push(Control::KillDown);
                } else if let Some(t) = self.kill_at.take() {
                    out.push(Control::KillUp(t.elapsed()));
                }
            }
        }
        out
    }
}

/// Startup line: is the configured button's device connected, and is the button plausible?
pub fn describe_button(what: &str, b: &Button, devices: &[DeviceMeta]) -> String {
    match b.device.to_ascii_lowercase().as_str() {
        "keyboard" | "mouse" | "gamepad" => format!("{what}: {b}"),
        id => match devices.iter().find(|d| d.elite_id().map(|x| x.eq_ignore_ascii_case(id)).unwrap_or(false)) {
            Some(d) => {
                let n: Option<u32> = b.key.strip_prefix("Joy_").and_then(|n| n.parse().ok());
                let warn = match n {
                    Some(n) if d.buttons_declared > 0 && n > d.buttons_declared => {
                        format!(" WARNING: the device declares only {} buttons", d.buttons_declared)
                    }
                    _ => String::new(),
                };
                format!("{what}: {b} — device connected: {}{warn}", d.display())
            }
            None => {
                let ids: Vec<String> =
                    devices.iter().filter_map(|d| d.elite_id().map(|i| format!("{i} ({})", d.product))).collect();
                format!(
                    "{what}: {b} — WARNING: no connected device has ID {}. Connected controllers: {}",
                    b.device,
                    if ids.is_empty() { "none".to_string() } else { ids.join(", ") }
                )
            }
        },
    }
}

/// Start listening on Windows. Returns the Raw Input/XInput status and the devices found.
#[cfg(windows)]
pub fn start(
    ptt: Option<Button>,
    kill: Option<Button>,
    paste: Option<Button>,
    debug: bool,
    debounce_ms: u64,
    tx: std::sync::mpsc::Sender<Control>,
    log: impl Fn(&str) + Send + Clone + 'static,
) -> (String, Vec<DeviceMeta>) {
    use astromech_input::listen::win::{self, Msg};
    // Enumerate first, then register (same order as s7-input-listener and --listen).
    let devices = win::rawinput::list_devices();
    let mut m = Mapper::new(ptt, kill, debug, log.clone());
    m.paste = paste;
    m.debounce_ms = debounce_ms as f64;
    for d in &devices {
        m.add_device(d.clone());
    }
    let start = Instant::now();
    let (mtx, mrx) = std::sync::mpsc::channel();
    let reg = win::rawinput::start(mtx.clone(), start);
    let xi = win::xinput::start(mtx, start);
    std::thread::Builder::new()
        .name("controls".into())
        .spawn(move || {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| loop {
                let wait = Duration::from_secs_f64(m.wait_ms(start.elapsed().as_secs_f64() * 1000.0) / 1000.0);
                let cs = match mrx.recv_timeout(wait) {
                    Ok(Msg::Device(d)) => {
                        m.add_device(d);
                        continue;
                    }
                    Ok(Msg::Sample(s)) => m.sample(&s),
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => m.tick(start.elapsed().as_secs_f64() * 1000.0),
                    Err(_) => {
                        log("ERROR: the input listener stopped; PTT and the kill switch no longer work");
                        break;
                    }
                };
                for c in cs {
                    let _ = tx.send(c);
                }
            }));
            if r.is_err() {
                log("ERROR: the input thread crashed; PTT and the kill switch no longer work. Please send this log.");
            }
        })
        .expect("spawn controls thread");
    let status = format!(
        "Raw Input: {}; XInput: {xi}",
        match reg {
            Ok(()) => "ok".to_string(),
            Err(e) => e,
        }
    );
    (status, devices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_input::listen::model::{DevKind, SampleData};
    use std::sync::{Arc, Mutex};

    /// Sumeet's Omni Throttle left stick as Raw Input reports it (S7 run 2: 128 buttons,
    /// X/Y/Z/RX/RY/RZ, one hat, device path HID#VID_231D&PID_3201).
    fn vkb_l() -> DeviceMeta {
        DeviceMeta {
            key: r"\\?\HID#VID_231D&PID_3201#a&f2155fd&0&0000#{4d1e55b2-f16f-11cf-88cb-001111000030}".into(),
            kind: DevKind::Hid,
            vid: 0x231D,
            pid: 0x3201,
            product: "VKBsim Gladiator EVO OT L".into(),
            usage: 4,
            buttons_declared: 128,
            axes_declared: ["X", "Y", "Z", "RX", "RY", "RZ"].map(String::from).to_vec(),
            hats_declared: 1,
        }
    }

    fn hid(t: f64, buttons: &[u16], y: f64) -> Sample {
        Sample {
            t_ms: t,
            dev: vkb_l().key,
            fg: "Elite - Dangerous (CLIENT)".into(),
            injected: false,
            data: SampleData::Hid {
                buttons: buttons.to_vec(),
                hats: vec![None],
                axes: vec![
                    ("X".into(), 0.5),
                    ("Y".into(), y),
                    ("Z".into(), 0.0),
                    ("RX".into(), 0.5),
                    ("RY".into(), 0.5),
                    ("RZ".into(), 0.5),
                ],
            },
        }
    }

    fn mapper(debug: bool) -> (Mapper, Arc<Mutex<Vec<String>>>) {
        let logs = Arc::new(Mutex::new(vec![]));
        let l2 = logs.clone();
        let ptt = Button { device: "231D3201".into(), key: "Joy_15".into() };
        let kill = Button { device: "231D3201".into(), key: "Joy_16".into() };
        let mut m = Mapper::new(Some(ptt), Some(kill), debug, move |s: &str| l2.lock().unwrap().push(s.to_string()));
        m.add_device(vkb_l());
        (m, logs)
    }

    /// Sleep as the controls thread does (`wait_ms`), tick on waking, and return when `PttUp`
    /// comes out, or None by `until_ms`. A fake clock: `now_ms` is the only time there is.
    fn run_idle(m: &mut Mapper, mut now_ms: f64, until_ms: f64) -> Option<f64> {
        while now_ms < until_ms {
            now_ms += m.wait_ms(now_ms);
            if m.tick(now_ms).contains(&Control::PttUp) {
                return Some(now_ms);
            }
        }
        None
    }

    /// astromech-e0zz: with no further input, a released PTT is reported when its debounce window
    /// ends, not at the first idle tick after it (up to 100 ms late: about 200 ms after the release).
    #[test]
    fn release_is_reported_when_the_debounce_window_ends() {
        let (mut m, _) = mapper(false);
        assert_eq!(m.sample(&hid(5000.0, &[15], 0.3)), vec![Control::PttDown]);
        assert!(m.sample(&hid(5600.0, &[], 0.3)).is_empty());
        let at = run_idle(&mut m, 5600.0, 6000.0).expect("the release is reported");
        assert!(at >= 5600.0 + DEBOUNCE_MS, "never before the window ends: {at}");
        assert!(at <= 5600.0 + DEBOUNCE_MS + 2.0, "within a few ms of the deadline: {at}");

        // For contrast, the old loop's fixed 100 ms tick (the idle tick landed 50 ms past the
        // deadline, and the thread's own wake-up and queueing added the rest).
        let (mut old, _) = mapper(false);
        old.sample(&hid(5000.0, &[15], 0.3));
        old.sample(&hid(5600.0, &[], 0.3));
        let (mut now, mut found) = (5600.0, None);
        while found.is_none() && now < 6000.0 {
            now += 100.0;
            found = old.tick(now).contains(&Control::PttUp).then_some(now);
        }
        assert_eq!(found, Some(5800.0), "a 100 ms tick reports it 50 ms late at best");
    }

    /// The debounce still ignores chatter: a press inside the window cancels the release, and the
    /// wait never runs past the idle tick.
    #[test]
    fn the_wait_is_the_idle_tick_except_to_end_a_debounce_window() {
        let (mut m, _) = mapper(false);
        assert_eq!(m.wait_ms(1000.0), IDLE_TICK_MS);
        m.sample(&hid(5000.0, &[15], 0.3));
        assert_eq!(m.wait_ms(5001.0), IDLE_TICK_MS, "pressed, nothing pending");
        m.sample(&hid(5600.0, &[], 0.3));
        assert_eq!(m.wait_ms(5600.0), IDLE_TICK_MS, "150 ms to go: the idle tick comes first");
        assert!((m.wait_ms(5700.0) - 50.0).abs() < 1e-9, "50 ms to go");
        assert_eq!(m.wait_ms(5900.0), MIN_WAIT_MS, "overdue: wake at once, but never spin");
        // A press inside the window is bounce: no release, and nothing pending.
        m.sample(&hid(5700.0, &[15], 0.3));
        assert_eq!(m.wait_ms(5701.0), IDLE_TICK_MS);
        assert_eq!(run_idle(&mut m, 5701.0, 7000.0), None);
    }

    /// The bug from Sumeet's PC: the stick is idle (Windows joysticks report only on change),
    /// so the very first report the listener sees is the PTT press. It used to become the
    /// baseline, the release then came as an "up" with no down, and nothing happened.
    #[test]
    fn first_report_is_the_ptt_press() {
        let (mut m, _) = mapper(false);
        assert_eq!(m.sample(&hid(5000.0, &[15], 0.3)), vec![Control::PttDown]);
        assert!(m.sample(&hid(5600.0, &[], 0.3)).is_empty(), "release waits out the debounce");
        assert_eq!(m.tick(5760.0), vec![Control::PttUp]);
    }

    #[test]
    fn ptt_after_axis_traffic_and_kill_switch() {
        let (mut m, logs) = mapper(false);
        assert!(m.sample(&hid(0.0, &[], 0.5)).is_empty());
        assert!(m.sample(&hid(100.0, &[], 0.7)).is_empty());
        assert_eq!(m.sample(&hid(1000.0, &[15], 0.7)), vec![Control::PttDown]);
        assert!(m.sample(&hid(1200.0, &[15], 0.71)).is_empty(), "held: no repeat");
        assert!(m.sample(&hid(1500.0, &[], 0.71)).is_empty());
        assert_eq!(m.sample(&hid(2000.0, &[16], 0.71)), vec![Control::PttUp, Control::KillDown]);
        assert!(matches!(m.sample(&hid(2100.0, &[], 0.71))[..], [Control::KillUp(_)]));
        // A neighbouring button is reported (a few times), not treated as PTT.
        assert!(m.sample(&hid(3000.0, &[14], 0.71)).is_empty());
        assert_eq!(logs.lock().unwrap().last().unwrap(), "Joy_14 on 231D3201 pressed; PTT is Joy_15");
        for i in 0..10 {
            m.sample(&hid(3100.0 + i as f64 * 10.0, &[], 0.71));
            m.sample(&hid(3105.0 + i as f64 * 10.0, &[14], 0.71));
        }
        assert_eq!(logs.lock().unwrap().len() as u32, UNMATCHED_REPORTS);
    }

    #[test]
    fn paste_button_fires_once_per_press() {
        let (mut m, logs) = mapper(false);
        m.paste = Some(Button { device: "231D3201".into(), key: "Joy_12".into() });
        m.sample(&hid(0.0, &[], 0.5));
        assert_eq!(m.sample(&hid(100.0, &[12], 0.5)), vec![Control::PasteDown]);
        assert!(m.sample(&hid(200.0, &[12], 0.5)).is_empty(), "held: no repeat");
        assert!(m.sample(&hid(300.0, &[], 0.5)).is_empty());
        assert!(logs.lock().unwrap().is_empty(), "not reported as an unmatched press");
    }

    /// One push with contact bounce (down, up 8 ms later, down again) must give exactly one
    /// PttDown/PttUp pair, not a short spurious clip followed by the real one.
    #[test]
    fn bounce_gives_one_clip() {
        let (mut m, logs) = mapper(false);
        m.sample(&hid(0.0, &[], 0.5));
        let mut got = vec![];
        for (t, b) in [(1000.0, vec![15]), (1008.0, vec![]), (1016.0, vec![15]), (1030.0, vec![]), (1040.0, vec![15])] {
            got.extend(m.sample(&hid(t, &b, 0.5)));
        }
        got.extend(m.tick(2000.0));
        got.extend(m.sample(&hid(3000.0, &[], 0.5)));
        got.extend(m.tick(3100.0));
        assert_eq!(got, vec![Control::PttDown]);
        assert_eq!(m.tick(3200.0), vec![Control::PttUp]);
        assert_eq!(m.bounces, 2);
        assert!(logs.lock().unwrap()[0].starts_with("PTT bounce ignored"));
    }

    #[test]
    fn debug_input_logs_every_button() {
        let (mut m, logs) = mapper(true);
        m.sample(&hid(0.0, &[3], 0.5));
        m.sample(&hid(10.0, &[15], 0.5));
        let l = logs.lock().unwrap().clone();
        assert_eq!(l[0], "input: button Joy_3 on 231D3201 (VKBsim Gladiator EVO OT L [231D:3201]) down");
        assert!(l.iter().any(|x| x == "input: button Joy_3 on 231D3201 (VKBsim Gladiator EVO OT L [231D:3201]) up"));
        assert!(l
            .iter()
            .any(|x| x.ends_with("Joy_15 on 231D3201 (VKBsim Gladiator EVO OT L [231D:3201]) down -> PTT")));
    }

    /// The S7 replay fixture (VKB pair): button 4 on the left stick as PTT.
    #[test]
    fn replay_fixture_reaches_ptt() {
        let f = concat!(env!("CARGO_MANIFEST_DIR"), "/../astromech-input/tests/fixtures/replay-vkb.json");
        let text = std::fs::read_to_string(f).unwrap();
        let rf: astromech_input::listen::replay::ReplayFile = serde_json::from_str(&text).unwrap();
        let mut m =
            Mapper::new(Some(Button { device: "231D3201".into(), key: "Joy_4".into() }), None, false, |_: &str| {});
        for d in rf.devices {
            m.add_device(d);
        }
        let mut got = vec![];
        for s in &rf.samples {
            got.extend(m.sample(s));
        }
        got.extend(m.tick(1e9));
        assert_eq!(&got[..2], &[Control::PttDown, Control::PttUp]);
    }

    #[test]
    fn describes_missing_and_implausible_buttons() {
        let ds = vec![vkb_l()];
        let ok = describe_button("PTT", &Button { device: "231D3201".into(), key: "Joy_15".into() }, &ds);
        assert!(ok.contains("device connected: VKBsim Gladiator EVO OT L"), "{ok}");
        let gone = describe_button("PTT", &Button { device: "231D0201".into(), key: "Joy_15".into() }, &ds);
        assert!(gone.contains("WARNING: no connected device has ID 231D0201") && gone.contains("231D3201"));
        let big = describe_button("PTT", &Button { device: "231D3201".into(), key: "Joy_200".into() }, &ds);
        assert!(big.contains("declares only 128"));
    }
}
