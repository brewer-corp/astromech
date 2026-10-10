//! End-of-run analysis: per-device summary, focus time, anomalies and the verdict.

use crate::binds_index::BindsIndex;
use crate::model::*;
use crate::tracker::Processor;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct Verdict {
    /// "WORKS", "NOT OBSERVED" or "FAILED"
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ButtonSeen {
    pub control: String,
    pub elite_name: Option<String>,
    pub presses: u32,
    pub presses_elite_focused: u32,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AxisSeen {
    pub axis: String,
    pub elite_name: Option<String>,
    pub movements: u32,
    pub min: f64,
    pub max: f64,
    /// Bound actions from the active preset (--with-binds): union over this axis's movements.
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceSummary {
    pub device: String,
    pub kind: DevKind,
    pub elite_id: Option<String>,
    pub buttons_declared: u32,
    pub axes_declared: Vec<String>,
    pub hats_declared: u32,
    pub events: u32,
    pub events_elite_focused: u32,
    pub events_not_focused: u32,
    pub events_injected: u32,
    pub buttons: Vec<ButtonSeen>,
    pub hats: Vec<ButtonSeen>,
    pub axes: Vec<AxisSeen>,
    pub above_32: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct FocusInfo {
    pub total_ms: f64,
    pub elite_focused_ms: f64,
    pub switches: u32,
    /// Foreground titles with time (ms), longest first.
    pub titles: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub tool: String,
    pub generated_at: String,
    pub mode: String,
    pub read_only: String,
    pub seconds: f64,
    pub elite_title: String,
    pub registration: String,
    pub xinput: String,
    pub verdict: Verdict,
    pub verdict_controllers: Verdict,
    pub verdict_keyboard: Verdict,
    pub verdict_mouse: Verdict,
    pub focus: FocusInfo,
    pub devices: Vec<DeviceSummary>,
    pub above_32_warnings: Vec<String>,
    pub anomalies: Vec<String>,
    pub key_repeats_collapsed: u64,
    pub binds: Option<BindsIndex>,
    pub binds_error: Option<String>,
    pub events: Vec<Event>,
    pub notes: Vec<String>,
}

pub fn focus_info(timeline: &[(f64, String)], end_ms: f64, elite: &str) -> FocusInfo {
    let mut f = FocusInfo { total_ms: end_ms, ..Default::default() };
    let mut per: BTreeMap<String, f64> = BTreeMap::new();
    for (i, (t, title)) in timeline.iter().enumerate() {
        let next = timeline.get(i + 1).map(|x| x.0).unwrap_or(end_ms);
        let dur = (next - t).max(0.0);
        *per.entry(title.clone()).or_default() += dur;
        if title == elite {
            f.elite_focused_ms += dur;
        }
        if i > 0 && timeline[i - 1].1 != *title {
            f.switches += 1;
        }
    }
    let mut v: Vec<(String, f64)> = per.into_iter().collect();
    v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    v.truncate(8);
    f.titles = v;
    f
}

fn class_of(k: DevKind) -> &'static str {
    match k {
        DevKind::Hid | DevKind::Xinput => "controllers",
        DevKind::Keyboard => "keyboard",
        DevKind::Mouse => "mouse",
    }
}

fn noun(class: &str) -> &str {
    if class == "controllers" { "controller" } else { class }
}

fn verdict_for(class: &str, p: &Processor, focus: &FocusInfo, registration_ok: bool, reg: &str) -> Verdict {
    if !registration_ok && class != "controllers_xinput" {
        return Verdict { status: "FAILED".into(), detail: format!("Raw Input registration failed: {reg}") };
    }
    let evs: Vec<&Event> = p.events.iter().filter(|e| p.devices.get(&e.dev).map(|d| class_of(d.kind) == class).unwrap_or(class == "keyboard" && e.kind == "key")).collect();
    let real_f = evs.iter().filter(|e| e.elite_focused && !e.injected).count();
    let inj_f = evs.iter().filter(|e| e.elite_focused && e.injected).count();
    let real_nf = evs.iter().filter(|e| !e.elite_focused && !e.injected).count();
    let secs = focus.elite_focused_ms / 1000.0;
    if real_f > 0 {
        Verdict { status: "WORKS".into(), detail: format!("{real_f} event(s) from real {} devices captured while Elite had focus ({secs:.0} s focused).", noun(class)) }
    } else if inj_f > 0 {
        Verdict { status: "WORKS".into(), detail: format!("{inj_f} software-injected {} event(s) captured while the Elite window had focus. Injected input proves the window/focus logic, not real-device capture.", noun(class)) }
    } else if focus.elite_focused_ms <= 0.0 {
        Verdict { status: "NOT OBSERVED".into(), detail: format!("The Elite window (\"{}\") never had focus during the run, so background capture wasn't tested.", p.elite_title) }
    } else if real_nf > 0 && focus.elite_focused_ms >= 10_000.0 {
        Verdict { status: "FAILED".into(), detail: format!("{real_nf} {} event(s) were captured while Elite was NOT focused, but none during {secs:.0} s with Elite focused. Input seems to stop reaching the listener when Elite has focus (suspected; or nothing was pressed then).", noun(class)) }
    } else {
        Verdict { status: "NOT OBSERVED".into(), detail: format!("No {} input arrived while Elite was focused ({secs:.0} s). Press some buttons while in the game.", noun(class)) }
    }
}

pub fn build(p: &Processor, timeline: &[(f64, String)], end_ms: f64, registration: Result<(), String>, notes: &mut Vec<String>) -> (FocusInfo, Vec<DeviceSummary>, Vec<String>, Vec<String>, [Verdict; 4]) {
    let focus = focus_info(timeline, end_ms, &p.elite_title);
    let mut sums = vec![];
    let mut over = vec![];
    for (key, d) in &p.devices {
        let evs: Vec<&Event> = p.events.iter().filter(|e| &e.dev == key).collect();
        let mut buttons: BTreeMap<String, ButtonSeen> = BTreeMap::new();
        let mut hats: BTreeMap<String, ButtonSeen> = BTreeMap::new();
        let mut axes: BTreeMap<String, AxisSeen> = BTreeMap::new();
        let mut above = vec![];
        for e in &evs {
            match e.kind.as_str() {
                "axis" => {
                    let a = e.axis.as_ref().unwrap();
                    let s = axes.entry(e.control.clone()).or_insert(AxisSeen { axis: e.control.clone(), elite_name: e.elite_name.clone(), movements: 0, min: a.min, max: a.max, actions: vec![] });
                    for act in &e.actions {
                        if !s.actions.contains(act) {
                            s.actions.push(act.clone());
                        }
                    }
                    s.movements += 1;
                    s.min = s.min.min(a.min);
                    s.max = s.max.max(a.max);
                }
                k => {
                    if e.down != Some(true) {
                        continue;
                    }
                    let map = if k == "hat" { &mut hats } else { &mut buttons };
                    let s = map.entry(e.control.clone()).or_insert(ButtonSeen { control: e.control.clone(), elite_name: e.elite_name.clone(), presses: 0, presses_elite_focused: 0, actions: e.actions.clone() });
                    s.presses += 1;
                    if e.elite_focused {
                        s.presses_elite_focused += 1;
                    }
                    if k == "button" {
                        if let Some(n) = e.control.strip_prefix("button ").and_then(|n| n.parse::<u16>().ok()) {
                            if n > 32 && !above.contains(&n) {
                                above.push(n);
                            }
                        }
                    }
                }
            }
        }
        above.sort();
        if !above.is_empty() {
            over.push(format!("{}: button(s) {} pressed. Elite reportedly reads only the first 32 buttons per device, so these can't be bound (VKB: check VKBDevCfg button numbering or use a virtual-device split).", d.display(), above.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")));
        }
        if d.buttons_declared > 32 && above.is_empty() {
            notes.push(format!("{} declares {} buttons; Elite reads at most 32. None above 32 were pressed in this run.", d.display(), d.buttons_declared));
        }
        let mut bv: Vec<ButtonSeen> = buttons.into_values().collect();
        bv.sort_by_key(|b| b.control.strip_prefix("button ").and_then(|n| n.parse::<u32>().ok()).unwrap_or(9999));
        sums.push(DeviceSummary {
            device: d.display(),
            kind: d.kind,
            elite_id: d.elite_id(),
            buttons_declared: d.buttons_declared,
            axes_declared: d.axes_declared.clone(),
            hats_declared: d.hats_declared,
            events: evs.len() as u32,
            events_elite_focused: evs.iter().filter(|e| e.elite_focused).count() as u32,
            events_not_focused: evs.iter().filter(|e| !e.elite_focused).count() as u32,
            events_injected: evs.iter().filter(|e| e.injected).count() as u32,
            buttons: bv,
            hats: hats.into_values().collect(),
            axes: axes.into_values().collect(),
            above_32: above,
        });
    }
    sums.sort_by_key(|s| (s.kind, std::cmp::Reverse(s.events)));
    let mut anomalies: Vec<String> = p.held_at_end().into_iter().map(|h| format!("{h} at the end (down without up: still held, or a dropped release)")).collect();
    anomalies.extend(p.anomalies.iter().cloned());
    let (ok, reg) = match &registration {
        Ok(()) => (true, "ok".to_string()),
        Err(e) => (false, e.clone()),
    };
    let c = verdict_for("controllers", p, &focus, ok, &reg);
    let k = verdict_for("keyboard", p, &focus, ok, &reg);
    let m = verdict_for("mouse", p, &focus, ok, &reg);
    let overall = Verdict { status: c.status.clone(), detail: format!("Background input capture while Elite focused (controllers): {}. {}", c.status, c.detail) };
    (focus, sums, over, anomalies, [overall, c, k, m])
}

#[cfg(test)]
mod tests {
    use super::*;
    const ELITE: &str = "Elite - Dangerous (CLIENT)";
    fn proc_with(fg: &str, injected: bool) -> Processor {
        let mut p = Processor::new(ELITE, None, true);
        p.add_device(DeviceMeta { key: "v".into(), kind: DevKind::Hid, vid: 0x231D, pid: 0x200, product: "R".into(), usage: 4, buttons_declared: 128, axes_declared: vec![], hats_declared: 1 });
        let s = |t: f64, b: &[u16]| Sample { t_ms: t, dev: "v".into(), fg: fg.into(), injected, data: SampleData::Hid { buttons: b.to_vec(), hats: vec![], axes: vec![] } };
        p.process(&s(0.0, &[]));
        p.process(&s(10.0, &[40]));
        p.process(&s(20.0, &[]));
        p.process(&s(30.0, &[3]));
        p
    }
    #[test]
    fn verdicts() {
        let p = proc_with(ELITE, false);
        let (f, sums, over, anom, v) = build(&p, &[(0.0, ELITE.into())], 15_000.0, Ok(()), &mut vec![]);
        assert_eq!(v[1].status, "WORKS");
        assert_eq!(f.elite_focused_ms, 15_000.0);
        assert_eq!(sums[0].above_32, vec![40]);
        assert_eq!(over.len(), 1);
        assert_eq!(anom.len(), 1, "{anom:?}"); // button 3 held
        assert_eq!(v[2].status, "NOT OBSERVED");
        let p = proc_with("Desktop", false);
        let (_, _, _, _, v) = build(&p, &[(0.0, "Desktop".into()), (1000.0, ELITE.into())], 20_000.0, Ok(()), &mut vec![]);
        assert_eq!(v[1].status, "FAILED");
        let (_, _, _, _, v) = build(&p, &[(0.0, "Desktop".into())], 20_000.0, Ok(()), &mut vec![]);
        assert_eq!(v[1].status, "NOT OBSERVED");
        let (_, _, _, _, v) = build(&p, &[], 1.0, Err("x".into()), &mut vec![]);
        assert_eq!(v[1].status, "FAILED");
        let p = proc_with(ELITE, true);
        let (_, _, _, _, v) = build(&p, &[(0.0, ELITE.into())], 1.0, Ok(()), &mut vec![]);
        assert!(v[1].detail.contains("injected"));
    }
    #[test]
    fn focus() {
        let f = focus_info(&[(0.0, "A".into()), (100.0, ELITE.into()), (400.0, "A".into())], 500.0, ELITE);
        assert_eq!(f.elite_focused_ms, 300.0);
        assert_eq!(f.switches, 2);
        assert_eq!(f.titles[0], (ELITE.to_string(), 300.0));
    }
}
