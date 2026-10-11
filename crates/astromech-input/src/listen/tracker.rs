//! Turns raw samples into logged events: button/hat transitions, debounced axis movements,
//! keyboard (collapsing auto-repeat), mouse buttons and XInput. Platform-neutral, so the replay
//! mode exercises exactly the same logic as live capture.

use crate::listen::binds_index::BindsIndex;
use crate::listen::model::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Movement must exceed this fraction of the range from rest to count as an axis movement.
pub const AXIS_START: f64 = 0.05;
/// Changes smaller than this (fraction) don't extend a movement (noise).
pub const AXIS_NOISE: f64 = 0.01;
/// A movement ends after this long without significant change.
pub const AXIS_SETTLE_MS: f64 = 400.0;
/// Movements longer than this are cut (jittery axes).
pub const AXIS_MAX_MS: f64 = 10_000.0;

#[derive(Debug, Default)]
struct AxisState {
    rest: Option<f64>,
    last: f64,
    mv: Option<(f64, f64, f64, f64, f64, f64, String, bool)>, // start, min, max, end, t0, last_change, fg, injected
}

#[derive(Debug, Default)]
struct DevState {
    buttons: BTreeSet<u16>,
    povs: BTreeSet<(usize, usize)>,
    baseline: bool,
    axes: BTreeMap<String, AxisState>,
    xbuttons: u32,
}

pub struct Processor {
    pub devices: BTreeMap<String, DeviceMeta>,
    state: HashMap<String, DevState>,
    keys_down: BTreeSet<(u16, u16, bool)>,
    mouse_down: [bool; 6],
    pub key_repeats: u64,
    /// Tab/Alt/Win key-ups without a down (Alt+Tab artefacts), suppressed.
    pub switch_artifacts: u64,
    pub elite_title: String,
    pub binds: Option<BindsIndex>,
    pub events: Vec<Event>,
    /// Anomalies seen while processing (e.g. key up without down).
    pub anomalies: Vec<String>,
    pub log_keys: bool,
    /// Report buttons/hats already down in a device's *first* report as presses. Windows
    /// joysticks usually report only on change, so for a device that has been idle since
    /// start-up the first report the listener ever sees IS the press (PTT!). The S7 summary
    /// keeps the conservative default (`false`: first report is only a baseline).
    pub initial_presses: bool,
}

impl Processor {
    pub fn new(elite_title: &str, binds: Option<BindsIndex>, log_keys: bool) -> Self {
        Self {
            devices: BTreeMap::new(),
            state: HashMap::new(),
            keys_down: BTreeSet::new(),
            mouse_down: [false; 6],
            key_repeats: 0,
            switch_artifacts: 0,
            elite_title: elite_title.to_string(),
            binds,
            events: vec![],
            anomalies: vec![],
            log_keys,
            initial_presses: false,
        }
    }

    pub fn add_device(&mut self, d: DeviceMeta) {
        self.devices.entry(d.key.clone()).or_insert(d);
    }

    fn mk(&self, s: &Sample, kind: &str, control: String, elite_name: Option<String>, down: Option<bool>) -> Event {
        let d = self.devices.get(&s.dev);
        let mut e = Event {
            t_ms: s.t_ms,
            dev: s.dev.clone(),
            device: d.map(|d| d.display()).unwrap_or_else(|| s.dev.clone()),
            elite_id: d.and_then(|d| d.elite_id()),
            kind: kind.into(),
            control,
            elite_name,
            down,
            axis: None,
            foreground: s.fg.clone(),
            elite_focused: s.fg == self.elite_title,
            injected: s.injected,
            actions: vec![],
            note: None,
        };
        if let (Some(b), Some(n)) = (&self.binds, &e.elite_name) {
            e.actions = b.lookup(d, n);
        }
        e
    }

    /// Process one sample; returns the new events (also appended to `self.events`).
    pub fn process(&mut self, s: &Sample) -> Vec<Event> {
        let mut out = self.tick(s.t_ms);
        let n0 = out.len(); // tick events are already recorded
        match &s.data {
            SampleData::Hid { buttons, hats, axes } => {
                let mut st = self.state.remove(&s.dev).unwrap_or_default();
                let cur_b: BTreeSet<u16> = buttons.iter().copied().collect();
                let cur_p: BTreeSet<(usize, usize)> = hats
                    .iter()
                    .enumerate()
                    .flat_map(|(i, d)| hat_cardinals(*d).into_iter().map(move |c| (i, c)))
                    .collect();
                if st.baseline {
                    for b in cur_b.difference(&st.buttons) {
                        out.push(self.button_event(s, *b, true));
                    }
                    for b in st.buttons.difference(&cur_b) {
                        out.push(self.button_event(s, *b, false));
                    }
                    for &(i, c) in cur_p.difference(&st.povs) {
                        out.push(self.mk(
                            s,
                            "hat",
                            format!("hat {} {}", i + 1, CARDINALS[c].to_lowercase()),
                            Some(pov_name(i, c)),
                            Some(true),
                        ));
                    }
                    for &(i, c) in st.povs.difference(&cur_p) {
                        out.push(self.mk(
                            s,
                            "hat",
                            format!("hat {} {}", i + 1, CARDINALS[c].to_lowercase()),
                            Some(pov_name(i, c)),
                            Some(false),
                        ));
                    }
                } else if self.initial_presses {
                    for b in &cur_b {
                        out.push(self.button_event(s, *b, true));
                    }
                    for &(i, c) in &cur_p {
                        out.push(self.mk(
                            s,
                            "hat",
                            format!("hat {} {}", i + 1, CARDINALS[c].to_lowercase()),
                            Some(pov_name(i, c)),
                            Some(true),
                        ));
                    }
                } else if !cur_b.is_empty() || !cur_p.is_empty() {
                    self.anomalies.push(format!(
                        "{}: {} control(s) already held when first seen (baseline, not logged as presses)",
                        self.devices.get(&s.dev).map(|d| d.display()).unwrap_or(s.dev.clone()),
                        cur_b.len() + cur_p.len()
                    ));
                }
                st.baseline = true;
                st.buttons = cur_b;
                st.povs = cur_p;
                for (name, v) in axes {
                    self.axis_sample(&mut st, s, name, *v, &mut out);
                }
                self.state.insert(s.dev.clone(), st);
            }
            SampleData::Key { vkey, scan, e0, down, name } => {
                let k = (*vkey, *scan, *e0);
                let label = if name.is_empty() { format!("vk 0x{vkey:02X}") } else { name.clone() };
                let en = elite_key(*vkey, *scan, *e0);
                if *down {
                    if !self.keys_down.insert(k) {
                        self.key_repeats += 1; // auto-repeat: not logged
                        return out;
                    }
                } else if !self.keys_down.remove(&k) {
                    if matches!(*vkey, 0x09 | 0x12 | 0xA4 | 0xA5 | 0x5B | 0x5C) {
                        // Alt+Tab / Win-key window switching: Windows swallows the key-down, so only the up arrives. Benign; not logged.
                        self.switch_artifacts += 1;
                        return out;
                    }
                    let mut e = self.mk(s, "key", label.clone(), en.clone(), Some(false));
                    e.note = Some("up without a down (pressed before the listener started, or a dropped down)".into());
                    if !self.log_keys {
                        e.control = "key".into();
                        e.elite_name = None;
                    }
                    out.push(e);
                    self.anomalies.push(format!("key {label}: up without down"));
                    self.events.extend(out[n0..].iter().cloned());
                    return out;
                }
                let mut e = self.mk(s, "key", label, en, Some(*down));
                if !self.log_keys {
                    e.control = "key".into();
                    e.elite_name = None;
                    e.actions.clear();
                }
                out.push(e);
            }
            SampleData::MouseButton { n, down } => {
                let n = (*n).min(5);
                if self.mouse_down[n as usize] == *down {
                    if !*down {
                        self.anomalies.push(format!("mouse button {n}: up without down"));
                    }
                } else {
                    self.mouse_down[n as usize] = *down;
                    out.push(self.mk(s, "mouse", format!("mouse button {n}"), Some(format!("Mouse_{n}")), Some(*down)));
                }
            }
            SampleData::Xinput { buttons, axes } => {
                let mut st = self.state.remove(&s.dev).unwrap_or_default();
                if st.baseline {
                    for b in XBUTTONS {
                        if (st.xbuttons ^ buttons) & b != 0 {
                            out.push(self.mk(
                                s,
                                "xinput",
                                format!("XInput {}", xbutton_name(b)),
                                None,
                                Some(buttons & b != 0),
                            ));
                        }
                    }
                }
                st.baseline = true;
                st.xbuttons = *buttons;
                for (name, v) in axes {
                    self.axis_sample(&mut st, s, name, *v, &mut out);
                }
                self.state.insert(s.dev.clone(), st);
            }
        }
        self.events.extend(out[n0..].iter().cloned());
        out
    }

    fn button_event(&self, s: &Sample, n: u16, down: bool) -> Event {
        let mut e = self.mk(s, "button", format!("button {n}"), Some(joy_button(n)), Some(down));
        if n > 32 && down {
            e.note = Some(
                "above button 32: Elite reportedly reads only Joy_1..Joy_32 per device, so this can't be bound".into(),
            );
        }
        e
    }

    fn axis_sample(&self, st: &mut DevState, s: &Sample, name: &str, v: f64, out: &mut Vec<Event>) {
        let a = st.axes.entry(name.to_string()).or_default();
        a.last = v;
        let Some(rest) = a.rest else {
            a.rest = Some(v);
            return;
        };
        match &mut a.mv {
            None => {
                if (v - rest).abs() >= AXIS_START {
                    a.mv = Some((rest, rest.min(v), rest.max(v), v, s.t_ms, s.t_ms, s.fg.clone(), s.injected));
                }
            }
            Some(m) => {
                m.1 = m.1.min(v);
                m.2 = m.2.max(v);
                if (v - m.3).abs() >= AXIS_NOISE {
                    m.5 = s.t_ms;
                    m.3 = v;
                }
            }
        }
        let _ = out;
    }

    /// Finish axis movements that have settled. Call periodically with the current time.
    pub fn tick(&mut self, now_ms: f64) -> Vec<Event> {
        let mut out = vec![];
        let mut done: Vec<(String, String, (f64, f64, f64, f64, f64, f64, String, bool), f64)> = vec![];
        for (dev, st) in self.state.iter_mut() {
            for (name, a) in st.axes.iter_mut() {
                if let Some(m) = &a.mv {
                    if now_ms - m.5 >= AXIS_SETTLE_MS || now_ms - m.4 >= AXIS_MAX_MS {
                        let m = a.mv.take().unwrap();
                        a.rest = Some(a.last);
                        done.push((dev.clone(), name.clone(), m, a.last));
                    }
                }
            }
        }
        done.sort_by(|a, b| a.2 .4.partial_cmp(&b.2 .4).unwrap());
        for (dev, name, m, last) in done {
            let s = Sample {
                t_ms: m.4,
                dev: dev.clone(),
                fg: m.6.clone(),
                injected: m.7,
                data: SampleData::MouseButton { n: 0, down: false },
            };
            let kind = if self.devices.get(&dev).map(|d| d.kind == DevKind::Xinput).unwrap_or(false) {
                "xinput"
            } else {
                "axis"
            };
            let en = if kind == "axis" { elite_axis(&name) } else { None };
            let mut e = self.mk(&s, "axis", format!("{name} axis"), en.clone(), None);
            e.kind = "axis".into();
            let pct = |x: f64| (x * 1000.0).round() / 10.0;
            e.axis = Some(AxisMove {
                start: pct(m.0),
                min: pct(m.1),
                max: pct(m.2),
                end: pct(last),
                duration_ms: (m.5 - m.4).max(0.0),
            });
            if en.is_none() && kind == "axis" {
                e.note = Some(format!("{name} has no Elite axis slot"));
            }
            if let (Some(b), Some(n)) = (&self.binds, &en) {
                let d = self.devices.get(&dev);
                let mut acts = b.lookup(d, n);
                for p in ["Pos_", "Neg_"] {
                    acts.extend(b.lookup(d, &format!("{p}{n}")));
                }
                e.actions = acts;
            }
            out.push(e);
        }
        self.events.extend(out.iter().cloned());
        out
    }

    /// Controls still held at the end (down without up).
    pub fn held_at_end(&self) -> Vec<String> {
        let mut v = vec![];
        for (dev, st) in &self.state {
            let name = self.devices.get(dev).map(|d| d.display()).unwrap_or(dev.clone());
            for b in &st.buttons {
                v.push(format!("{name}: button {b} (Joy_{b}) still down"));
            }
            for (i, c) in &st.povs {
                v.push(format!("{name}: {} still held", pov_name(*i, *c)));
            }
            for b in XBUTTONS {
                if st.xbuttons & b != 0 {
                    v.push(format!("{name}: XInput {} still down", xbutton_name(b)));
                }
            }
        }
        for k in &self.keys_down {
            v.push(format!("keyboard: vk 0x{:02X} still down", k.0));
        }
        for (n, d) in self.mouse_down.iter().enumerate() {
            if *d {
                v.push(format!("mouse button {n} still down"));
            }
        }
        v.sort();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ELITE: &str = "Elite - Dangerous (CLIENT)";
    fn hid(t: f64, b: &[u16], h: Option<u8>, x: f64) -> Sample {
        Sample {
            t_ms: t,
            dev: "vkb".into(),
            fg: ELITE.into(),
            injected: false,
            data: SampleData::Hid { buttons: b.to_vec(), hats: vec![h], axes: vec![("X".into(), x)] },
        }
    }
    fn p() -> Processor {
        let mut p = Processor::new(ELITE, None, true);
        p.add_device(DeviceMeta {
            key: "vkb".into(),
            kind: DevKind::Hid,
            vid: 0x231D,
            pid: 0x3201,
            product: "VKB L".into(),
            usage: 4,
            buttons_declared: 128,
            axes_declared: vec!["X".into()],
            hats_declared: 1,
        });
        p
    }
    #[test]
    fn buttons_and_hats() {
        let mut p = p();
        assert!(p.process(&hid(0.0, &[], None, 0.5)).is_empty());
        let e = p.process(&hid(10.0, &[4], Some(1), 0.5));
        let names: Vec<_> = e.iter().map(|e| (e.elite_name.clone().unwrap(), e.down.unwrap())).collect();
        assert_eq!(names, vec![("Joy_4".into(), true), ("Joy_POV1Up".into(), true), ("Joy_POV1Right".into(), true)]);
        assert!(e[0].elite_focused);
        assert_eq!(e[0].elite_id.as_deref(), Some("231D3201"));
        let e = p.process(&hid(20.0, &[33], Some(2), 0.5));
        let names: Vec<_> = e.iter().map(|e| (e.elite_name.clone().unwrap(), e.down.unwrap())).collect();
        assert_eq!(names, vec![("Joy_33".into(), true), ("Joy_4".into(), false), ("Joy_POV1Up".into(), false)]);
        assert!(e[0].note.as_ref().unwrap().contains("32"));
        assert_eq!(p.held_at_end().len(), 2);
    }
    #[test]
    fn axis_debounce() {
        let mut p = p();
        p.process(&hid(0.0, &[], None, 0.50));
        assert!(p.process(&hid(10.0, &[], None, 0.52)).is_empty(), "below start threshold");
        p.process(&hid(20.0, &[], None, 0.70));
        p.process(&hid(30.0, &[], None, 0.95));
        p.process(&hid(40.0, &[], None, 0.951)); // noise
        p.process(&hid(100.0, &[], None, 0.60));
        assert!(p.tick(300.0).is_empty());
        let e = p.tick(600.0);
        assert_eq!(e.len(), 1);
        let a = e[0].axis.as_ref().unwrap();
        assert_eq!((a.start, a.min, a.max, a.end), (50.0, 50.0, 95.1, 60.0));
        assert_eq!(e[0].elite_name.as_deref(), Some("Joy_XAxis"));
        assert!(p.tick(5000.0).is_empty());
        // A later sample triggers the tick inside process(): recorded exactly once.
        p.process(&hid(6000.0, &[], None, 0.2));
        p.process(&hid(7000.0, &[1], None, 0.2));
        assert_eq!(p.events.iter().filter(|e| e.kind == "axis").count(), 2);
        assert_eq!(p.events.len(), 3);
    }
    #[test]
    fn keyboard_repeat_and_orphans() {
        let mut p = Processor::new(ELITE, None, true);
        let k = |t: f64, down: bool| Sample {
            t_ms: t,
            dev: "kb".into(),
            fg: "Notepad".into(),
            injected: true,
            data: SampleData::Key { vkey: 0x41, scan: 0x1E, e0: false, down, name: "A".into() },
        };
        assert_eq!(p.process(&k(0.0, true)).len(), 1);
        assert_eq!(p.process(&k(30.0, true)).len(), 0);
        assert_eq!(p.key_repeats, 1);
        let e = p.process(&k(60.0, false));
        assert_eq!(e[0].elite_name.as_deref(), Some("Key_A"));
        assert!(!e[0].elite_focused);
        let e = p.process(&k(90.0, false));
        assert!(e[0].note.is_some());
        assert_eq!(p.anomalies.len(), 1);
        // Alt+Tab artefact: Tab up without down is suppressed, not an anomaly.
        let tab = Sample {
            t_ms: 100.0,
            dev: "kb".into(),
            fg: ELITE.into(),
            injected: false,
            data: SampleData::Key { vkey: 0x09, scan: 0x0F, e0: false, down: false, name: "key Tab".into() },
        };
        assert!(p.process(&tab).is_empty());
        assert_eq!((p.anomalies.len(), p.switch_artifacts), (1, 1));
    }
}
