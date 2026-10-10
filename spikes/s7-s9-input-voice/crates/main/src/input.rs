//! Platform-neutral input event types and binding matching (S7).

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    RawKeyboard,
    RawMouse,
    RawHid,
    XInput,
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Source::RawKeyboard => "Raw Input keyboard",
            Source::RawMouse => "Raw Input mouse",
            Source::RawHid => "Raw Input HID",
            Source::XInput => "XInput",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Control {
    /// Keyboard key: virtual key, scan code and the E0 prefix flag.
    Key { vkey: u16, scan: u16, e0: bool },
    /// Mouse button 1..=5 (1 left, 2 right, 3 middle, 4/5 side buttons).
    MouseButton { n: u8 },
    /// HID button usage (usage page 9 = generic buttons; `usage` is the button number).
    HidButton { page: u16, usage: u16 },
    /// HID hat switch `index` pointing in direction `dir` (0 = up, 2 = right, 4 = down, 6 = left).
    Hat { index: u8, dir: u8 },
    /// XInput button bit (triggers are reported as 0x10000 = LT, 0x20000 = RT when pressed past half).
    XButton { mask: u32 },
}

pub fn xbutton_name(mask: u32) -> &'static str {
    match mask {
        0x0001 => "D-pad up",
        0x0002 => "D-pad down",
        0x0004 => "D-pad left",
        0x0008 => "D-pad right",
        0x0010 => "Start/Menu",
        0x0020 => "Back/View",
        0x0040 => "Left stick click",
        0x0080 => "Right stick click",
        0x0100 => "LB",
        0x0200 => "RB",
        0x1000 => "A",
        0x2000 => "B",
        0x4000 => "X",
        0x8000 => "Y",
        0x10000 => "LT",
        0x20000 => "RT",
        _ => "unknown",
    }
}

const HAT_DIRS: [&str; 8] = ["up", "up-right", "right", "down-right", "down", "down-left", "left", "up-left"];

impl Control {
    /// Fallback label when the platform did not supply a nicer one.
    pub fn describe(&self) -> String {
        match *self {
            Control::Key { vkey, scan, e0 } => format!("key vk=0x{vkey:02X} scan=0x{scan:02X}{}", if e0 { " (E0)" } else { "" }),
            Control::MouseButton { n } => format!("mouse button {n}"),
            Control::HidButton { page: 9, usage } => format!("button {usage}"),
            Control::HidButton { page, usage } => format!("usage page 0x{page:X} usage 0x{usage:X}"),
            Control::Hat { index, dir } => format!("hat {} {}", index + 1, HAT_DIRS[(dir & 7) as usize]),
            Control::XButton { mask } => format!("XInput {}", xbutton_name(mask)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceInfo {
    pub source: Source,
    /// Raw Input device interface path, or "xinput:<slot>".
    pub path: String,
    pub vid: u16,
    pub pid: u16,
    pub product: String,
    /// Top-level collection usage page/usage (1/4 joystick, 1/5 gamepad, 1/8 multi-axis...).
    pub usage_page: u16,
    pub usage: u16,
}

impl DeviceInfo {
    pub fn describe(&self) -> String {
        let kind = match (self.source, self.usage_page, self.usage) {
            (Source::RawKeyboard, ..) => "keyboard",
            (Source::RawMouse, ..) => "mouse",
            (Source::XInput, ..) => "XInput controller",
            (_, 1, 4) => "joystick",
            (_, 1, 5) => "gamepad",
            (_, 1, 8) => "multi-axis controller",
            _ => "HID device",
        };
        let name = if self.product.is_empty() { "(no product name)" } else { &self.product };
        if self.vid != 0 || self.pid != 0 {
            format!("{name} [{kind}, VID {:04X} PID {:04X}]", self.vid, self.pid)
        } else {
            format!("{name} [{kind}]")
        }
    }

    /// Same physical device? Keyboards/mice: any keyboard/any mouse (Windows merges them for
    /// games too). HID: same interface path, or same non-zero VID/PID (path can change after
    /// replugging). XInput: same slot.
    pub fn same_device(&self, other: &DeviceInfo) -> bool {
        if self.source != other.source {
            return false;
        }
        match self.source {
            Source::RawKeyboard | Source::RawMouse => true,
            Source::XInput => self.path == other.path,
            Source::RawHid => {
                self.path.eq_ignore_ascii_case(&other.path)
                    || ((self.vid != 0 || self.pid != 0) && self.vid == other.vid && self.pid == other.pid)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputEvent {
    pub device: Arc<DeviceInfo>,
    pub control: Control,
    pub label: String,
    pub down: bool,
    /// When our input thread received the event.
    pub at: Instant,
    /// Raw Input only: GetTickCount() at receipt minus GetMessageTime() (ms; the OS tick
    /// is ~15.6 ms coarse, so this is 0 or ~16 most of the time).
    pub queue_ms: Option<u32>,
    /// Foreground window title when the event was received.
    pub foreground: String,
    /// Keyboard/mouse events with no device handle (generated by software, e.g. SendInput).
    pub injected: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BoundControl {
    pub device: DeviceInfo,
    pub control: Control,
    pub label: String,
}

impl BoundControl {
    pub fn matches(&self, ev: &InputEvent) -> bool {
        self.control == ev.control && self.device.same_device(&ev.device)
    }
    pub fn describe(&self) -> String {
        format!("{} on {} (via {})", self.label, self.device.describe(), self.device.source.label())
    }
}

/// A learned binding: the first source that reported the press, plus the same press seen
/// through the other API (an Xbox pad shows up in both Raw Input HID and XInput).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Binding {
    pub primary: BoundControl,
    #[serde(default)]
    pub alternate: Option<BoundControl>,
}

impl Binding {
    pub fn matches_primary(&self, ev: &InputEvent) -> bool {
        self.primary.matches(ev)
    }
    pub fn matches_alternate(&self, ev: &InputEvent) -> bool {
        self.alternate.as_ref().map(|a| a.matches(ev)).unwrap_or(false)
    }
    pub fn matches_any(&self, ev: &InputEvent) -> bool {
        self.matches_primary(ev) || self.matches_alternate(ev)
    }
    pub fn same_as(&self, other: &Binding) -> bool {
        let mine = std::iter::once(&self.primary).chain(self.alternate.iter());
        let theirs: Vec<&BoundControl> = std::iter::once(&other.primary).chain(other.alternate.iter()).collect();
        mine.into_iter().any(|m| theirs.iter().any(|t| m.control == t.control && m.device.same_device(&t.device)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Bindings {
    pub ptt: Binding,
    pub kill: Binding,
}

/// Events we ignore while learning: left/right mouse (used to click the console) and Enter/Esc
/// (used to answer prompts; Esc cancels learning).
pub fn ignored_for_learning(c: &Control) -> bool {
    matches!(c, Control::MouseButton { n: 1 | 2 } | Control::Key { vkey: 0x0D | 0x1B, .. })
}

pub fn is_escape(c: &Control) -> bool {
    matches!(c, Control::Key { vkey: 0x1B, .. })
}

/// Parse "VID_044F&PID_B10A" (any case) out of a device interface path.
pub fn parse_vid_pid(path: &str) -> (u16, u16) {
    let up = path.to_ascii_uppercase();
    let grab = |tag: &str| -> u16 {
        up.find(tag)
            .and_then(|i| up.get(i + tag.len()..i + tag.len() + 4))
            .and_then(|h| u16::from_str_radix(h, 16).ok())
            .unwrap_or(0)
    };
    (grab("VID_"), grab("PID_"))
}

/// Map a HID hat value to a direction 0..=7, or None when centred/out of range.
pub fn hat_direction(value: i64, logical_min: i64, logical_max: i64) -> Option<u8> {
    if value < logical_min || value > logical_max || logical_max <= logical_min {
        return None;
    }
    let positions = logical_max - logical_min + 1;
    match positions {
        8 => Some((value - logical_min) as u8),
        4 => Some(((value - logical_min) * 2) as u8),
        p => Some((((value - logical_min) * 8) / p) as u8),
    }
}

/// Turn a previous/current set of pressed controls into (control, down) transitions.
pub fn diff_sets<T: Copy + Eq + std::hash::Hash + Ord>(prev: &std::collections::BTreeSet<T>, cur: &std::collections::BTreeSet<T>) -> Vec<(T, bool)> {
    let mut v: Vec<(T, bool)> = cur.difference(prev).map(|c| (*c, true)).collect();
    v.extend(prev.difference(cur).map(|c| (*c, false)));
    v
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::BTreeSet;

    pub fn dev(source: Source, path: &str, vid: u16, pid: u16) -> Arc<DeviceInfo> {
        Arc::new(DeviceInfo { source, path: path.into(), vid, pid, product: "Test".into(), usage_page: 1, usage: 4 })
    }

    pub fn ev(d: &Arc<DeviceInfo>, control: Control, down: bool) -> InputEvent {
        InputEvent {
            device: d.clone(),
            control,
            label: control.describe(),
            down,
            at: Instant::now(),
            queue_ms: Some(0),
            foreground: "Elite - Dangerous (CLIENT)".into(),
            injected: false,
        }
    }

    #[test]
    fn vid_pid() {
        assert_eq!(parse_vid_pid(r"\\?\HID#VID_044F&PID_B10A&MI_00#7&1&0&0000#{4d1e55b2}"), (0x044F, 0xB10A));
        assert_eq!(parse_vid_pid(r"\\?\hid#vid_06a3&pid_075c#x"), (0x06A3, 0x075C));
        assert_eq!(parse_vid_pid("xinput:0"), (0, 0));
    }

    #[test]
    fn hats() {
        assert_eq!(hat_direction(0, 0, 7), Some(0));
        assert_eq!(hat_direction(8, 0, 7), None);
        assert_eq!(hat_direction(15, 0, 7), None);
        assert_eq!(hat_direction(3, 0, 3), Some(6));
        assert_eq!(hat_direction(5, 1, 8), Some(4));
        assert_eq!(hat_direction(0, 1, 8), None);
    }

    #[test]
    fn diffs() {
        let a: BTreeSet<u16> = [1, 2].into();
        let b: BTreeSet<u16> = [2, 3].into();
        let mut d = diff_sets(&a, &b);
        d.sort();
        assert_eq!(d, vec![(1, false), (3, true)]);
    }

    #[test]
    fn matching() {
        let stick = dev(Source::RawHid, r"\\?\HID#VID_044F&PID_B10A#1", 0x044F, 0xB10A);
        let stick_replugged = dev(Source::RawHid, r"\\?\HID#VID_044F&PID_B10A#2", 0x044F, 0xB10A);
        let other = dev(Source::RawHid, r"\\?\HID#VID_1234&PID_0001#1", 0x1234, 1);
        let kb1 = dev(Source::RawKeyboard, "kb1", 0, 0);
        let kb2 = dev(Source::RawKeyboard, "kb2", 0, 0);
        let b3 = Control::HidButton { page: 9, usage: 3 };
        let bind = BoundControl { device: (*stick).clone(), control: b3, label: "button 3".into() };
        assert!(bind.matches(&ev(&stick, b3, true)));
        assert!(bind.matches(&ev(&stick_replugged, b3, false)));
        assert!(!bind.matches(&ev(&other, b3, true)));
        assert!(!bind.matches(&ev(&stick, Control::HidButton { page: 9, usage: 4 }, true)));
        let space = Control::Key { vkey: 0x20, scan: 0x39, e0: false };
        let kbind = BoundControl { device: (*kb1).clone(), control: space, label: "Space".into() };
        assert!(kbind.matches(&ev(&kb2, space, true)), "any keyboard");
        let x = dev(Source::XInput, "xinput:0", 0, 0);
        let xa = Control::XButton { mask: 0x1000 };
        let b = Binding { primary: bind.clone(), alternate: Some(BoundControl { device: (*x).clone(), control: xa, label: "A".into() }) };
        assert!(b.matches_alternate(&ev(&x, xa, true)));
        assert!(b.matches_any(&ev(&stick, b3, true)));
        let b2 = Binding { primary: BoundControl { device: (*x).clone(), control: xa, label: "A".into() }, alternate: None };
        assert!(b.same_as(&b2));
        assert!(!b2.same_as(&Binding { primary: kbind, alternate: None }));
    }

    #[test]
    fn learning_filter() {
        assert!(ignored_for_learning(&Control::MouseButton { n: 1 }));
        assert!(!ignored_for_learning(&Control::MouseButton { n: 4 }));
        assert!(ignored_for_learning(&Control::Key { vkey: 0x0D, scan: 0x1C, e0: false }));
        assert!(!ignored_for_learning(&Control::Key { vkey: 0x20, scan: 0x39, e0: false }));
    }

    #[test]
    fn serde_binding() {
        let stick = dev(Source::RawHid, "p", 1, 2);
        let b = Binding { primary: BoundControl { device: (*stick).clone(), control: Control::Hat { index: 0, dir: 2 }, label: "hat 1 right".into() }, alternate: None };
        let s = serde_json::to_string(&b).unwrap();
        assert_eq!(serde_json::from_str::<Binding>(&s).unwrap(), b);
    }
}
