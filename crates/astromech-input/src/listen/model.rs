//! Platform-neutral input samples, events and Elite naming.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum DevKind {
    Hid,
    Keyboard,
    Mouse,
    Xinput,
}

/// A device as first seen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMeta {
    /// Unique key: Raw Input interface path, "xinput:N", "keyboard:<path>", ...
    pub key: String,
    pub kind: DevKind,
    #[serde(default)]
    pub vid: u16,
    #[serde(default)]
    pub pid: u16,
    #[serde(default)]
    pub product: String,
    /// HID top-level usage (4 joystick, 5 gamepad, 8 multi-axis; 6 keyboard; 2 mouse).
    #[serde(default)]
    pub usage: u16,
    #[serde(default)]
    pub buttons_declared: u32,
    #[serde(default)]
    pub axes_declared: Vec<String>,
    #[serde(default)]
    pub hats_declared: u32,
}

impl DeviceMeta {
    pub fn elite_id(&self) -> Option<String> {
        if self.kind == DevKind::Hid && (self.vid != 0 || self.pid != 0) {
            Some(elite_id(self.vid, self.pid))
        } else {
            None
        }
    }
    pub fn display(&self) -> String {
        let name = if self.product.is_empty() { "(no product name)" } else { &self.product };
        match self.kind {
            DevKind::Hid => format!("{name} [{:04X}:{:04X}]", self.vid, self.pid),
            DevKind::Xinput => name.to_string(),
            _ if self.vid != 0 => format!("{name} [{:04X}:{:04X}]", self.vid, self.pid),
            _ => name.to_string(),
        }
    }
}

/// Raw sample from the platform layer (or a replay file). HID samples are full state snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SampleData {
    /// Pressed HID buttons (usage page 9 numbers), hat values (direction 0..7 or None), axes as fraction 0..1 of the logical range.
    Hid {
        #[serde(default)]
        buttons: Vec<u16>,
        #[serde(default)]
        hats: Vec<Option<u8>>,
        #[serde(default)]
        axes: Vec<(String, f64)>,
    },
    Key {
        vkey: u16,
        scan: u16,
        #[serde(default)]
        e0: bool,
        down: bool,
        #[serde(default)]
        name: String,
    },
    MouseButton {
        n: u8,
        down: bool,
    },
    /// XInput button mask (+0x10000 LT, 0x20000 RT past half) and axes as 0..1.
    Xinput {
        buttons: u32,
        #[serde(default)]
        axes: Vec<(String, f64)>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub t_ms: f64,
    pub dev: String,
    #[serde(default)]
    pub fg: String,
    #[serde(default)]
    pub injected: bool,
    pub data: SampleData,
}

#[derive(Debug, Clone, Serialize)]
pub struct AxisMove {
    /// Percent of the logical range (0–100).
    pub start: f64,
    pub min: f64,
    pub max: f64,
    pub end: f64,
    pub duration_ms: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub t_ms: f64,
    pub dev: String,
    pub device: String,
    pub elite_id: Option<String>,
    /// "button", "hat", "axis", "key", "mouse", "xinput"
    pub kind: String,
    pub control: String,
    pub elite_name: Option<String>,
    pub down: Option<bool>,
    pub axis: Option<AxisMove>,
    pub foreground: String,
    pub elite_focused: bool,
    pub injected: bool,
    /// Bound Elite actions from the active ship preset (--with-binds).
    pub actions: Vec<String>,
    pub note: Option<String>,
}

pub fn elite_id(vid: u16, pid: u16) -> String {
    format!("{vid:04X}{pid:04X}")
}

pub fn joy_button(n: u16) -> String {
    format!("Joy_{n}")
}

pub const CARDINALS: [&str; 4] = ["Up", "Right", "Down", "Left"];

/// HID hat direction 0..7 (0 = up, clockwise) to the set of Elite cardinal directions it activates.
/// Diagonals activate two (e.g. up-right = Up + Right), which is how DirectInput POVs map to Elite's Joy_POVnUp/Right/Down/Left binds.
pub fn hat_cardinals(dir: Option<u8>) -> Vec<usize> {
    match dir {
        None => vec![],
        Some(d) => match d & 7 {
            0 => vec![0],
            1 => vec![0, 1],
            2 => vec![1],
            3 => vec![1, 2],
            4 => vec![2],
            5 => vec![2, 3],
            6 => vec![3],
            _ => vec![3, 0],
        },
    }
}

pub fn pov_name(hat_index: usize, cardinal: usize) -> String {
    format!("Joy_POV{}{}", hat_index + 1, CARDINALS[cardinal])
}

/// HID generic-desktop value usage to Elite axis name. Sliders/dials take U then V in order.
#[cfg_attr(not(windows), allow(dead_code))]
pub fn axis_names(usages: &[u16]) -> Vec<(u16, String)> {
    let mut out = vec![];
    let mut sliders = 0;
    for &u in usages {
        let n = match u {
            0x30 => "X".to_string(),
            0x31 => "Y".into(),
            0x32 => "Z".into(),
            0x33 => "RX".into(),
            0x34 => "RY".into(),
            0x35 => "RZ".into(),
            0x36 | 0x37 => {
                sliders += 1;
                match sliders {
                    1 => "U".into(),
                    2 => "V".into(),
                    n => format!("Slider{n}"),
                }
            }
            0x38 => "Wheel".into(),
            _ => continue,
        };
        out.push((u, n));
    }
    out
}

pub fn elite_axis(name: &str) -> Option<String> {
    match name {
        "X" | "Y" | "Z" | "RX" | "RY" | "RZ" | "U" | "V" => Some(format!("Joy_{name}Axis")),
        _ => None,
    }
}

/// Map a HID hat value to a direction 0..=7, or None when centred/out of range. (From the S7/S9 tool.)
#[cfg_attr(not(windows), allow(dead_code))]
pub fn hat_direction(value: i64, logical_min: i64, logical_max: i64) -> Option<u8> {
    if value < logical_min || value > logical_max || logical_max <= logical_min {
        return None;
    }
    match logical_max - logical_min + 1 {
        8 => Some((value - logical_min) as u8),
        4 => Some(((value - logical_min) * 2) as u8),
        p => Some((((value - logical_min) * 8) / p) as u8),
    }
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
pub const XBUTTONS: [u32; 16] =
    [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000, 0x20000];

/// Elite keyboard key name from a Windows virtual key (US layout naming, as in .binds files). Approximate for non-US layouts.
pub fn elite_key(vkey: u16, scan: u16, e0: bool) -> Option<String> {
    let s = match vkey {
        0x41..=0x5A => return Some(format!("Key_{}", vkey as u8 as char)),
        0x30..=0x39 => return Some(format!("Key_{}", vkey as u8 as char)),
        0x70..=0x87 => return Some(format!("Key_F{}", vkey - 0x6F)),
        0x60..=0x69 => return Some(format!("Key_Numpad_{}", vkey - 0x60)),
        0x08 => "Backspace",
        0x09 => "Tab",
        0x0D => {
            if e0 {
                "Numpad_Enter"
            } else {
                "Enter"
            }
        }
        0x10 => {
            if scan == 0x36 {
                "RightShift"
            } else {
                "LeftShift"
            }
        }
        0xA0 => "LeftShift",
        0xA1 => "RightShift",
        0x11 => {
            if e0 {
                "RightControl"
            } else {
                "LeftControl"
            }
        }
        0xA2 => "LeftControl",
        0xA3 => "RightControl",
        0x12 => {
            if e0 {
                "RightAlt"
            } else {
                "LeftAlt"
            }
        }
        0xA4 => "LeftAlt",
        0xA5 => "RightAlt",
        0x14 => "CapsLock",
        0x1B => "Escape",
        0x20 => "Space",
        0x21 => "PageUp",
        0x22 => "PageDown",
        0x23 => "End",
        0x24 => "Home",
        0x25 => "LeftArrow",
        0x26 => "UpArrow",
        0x27 => "RightArrow",
        0x28 => "DownArrow",
        0x2D => "Insert",
        0x2E => "Delete",
        0x6A => "Numpad_Multiply",
        0x6B => "Numpad_Add",
        0x6D => "Numpad_Subtract",
        0x6E => "Numpad_Decimal",
        0x6F => "Numpad_Divide",
        0xBA => "SemiColon",
        0xBB => "Equals",
        0xBC => "Comma",
        0xBD => "Minus",
        0xBE => "Period",
        0xBF => "Slash",
        0xC0 => "Grave",
        0xDB => "LeftBracket",
        0xDC => "BackSlash",
        0xDD => "RightBracket",
        0xDE => "Apostrophe",
        _ => return None,
    };
    Some(format!("Key_{s}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn naming() {
        assert_eq!(elite_id(0x231d, 0x3201), "231D3201");
        assert_eq!(joy_button(33), "Joy_33");
        assert_eq!(hat_cardinals(Some(1)), vec![0, 1]);
        assert_eq!(hat_cardinals(Some(7)), vec![3, 0]);
        assert!(hat_cardinals(None).is_empty());
        assert_eq!(pov_name(0, 2), "Joy_POV1Down");
        let a = axis_names(&[0x30, 0x31, 0x36, 0x37, 0x35, 0x38]);
        let n: Vec<&str> = a.iter().map(|x| x.1.as_str()).collect();
        assert_eq!(n, vec!["X", "Y", "U", "V", "RZ", "Wheel"]);
        assert_eq!(elite_axis("RX").as_deref(), Some("Joy_RXAxis"));
        assert_eq!(elite_axis("Wheel"), None);
        assert_eq!(hat_direction(0, 0, 7), Some(0));
        assert_eq!(hat_direction(8, 0, 7), None);
        assert_eq!(hat_direction(3, 0, 3), Some(6));
        assert_eq!(hat_direction(315, 0, 315), Some(7));
    }
    #[test]
    fn keys() {
        assert_eq!(elite_key(0x41, 0x1E, false).as_deref(), Some("Key_A"));
        assert_eq!(elite_key(0x10, 0x36, false).as_deref(), Some("Key_RightShift"));
        assert_eq!(elite_key(0x11, 0x1D, true).as_deref(), Some("Key_RightControl"));
        assert_eq!(elite_key(0x0D, 0x1C, true).as_deref(), Some("Key_Numpad_Enter"));
        assert_eq!(elite_key(0x70, 0x3B, false).as_deref(), Some("Key_F1"));
        assert_eq!(elite_key(0x63, 0x51, false).as_deref(), Some("Key_Numpad_3"));
        assert_eq!(elite_key(0xBA, 0x27, false).as_deref(), Some("Key_SemiColon"));
    }
}
