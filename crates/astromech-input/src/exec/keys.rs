//! Elite Dangerous `Key_*` names -> PS/2 set-1 scan codes for `SendInput(KEYEVENTF_SCANCODE)`.
//!
//! Elite's keyboard key names are DirectInput (`DIK_*`) key names, which are *positional*
//! (they name the physical key on a US layout, whatever character the active layout produces).
//! Evidence: names such as Key_AX, Key_Unlabeled, Key_ABNT_C1, Key_Yen are DIK names, and
//! bindED's Neo2 map sends VK 'U' for Key_A, i.e. Key_A is the physical "A position" key.
//! A DIK value is the set-1 make code, with bit 0x80 meaning "E0-prefixed" (extended).
//! The table below is written from the DIK constants and cross-checked in tests against
//! COVAS:NEXT's keymap.json (MIT), which stores DIK values.

/// A key ready for SendInput.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanKey {
    pub scan: u16,
    pub extended: bool,
}

impl ScanKey {
    /// DIK-style value: scan | 0x80 when extended.
    pub fn dik(&self) -> u16 {
        self.scan | if self.extended { 0x80 } else { 0 }
    }
}

impl std::fmt::Display for ScanKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.extended {
            write!(f, "E0 {:02X}", self.scan)
        } else {
            write!(f, "{:02X}", self.scan)
        }
    }
}

/// Keys we map. (name, DIK value). Extended = DIK & 0x80.
const TABLE: &[(&str, u16)] = &[
    ("Key_Escape", 0x01),
    ("Key_1", 0x02),
    ("Key_2", 0x03),
    ("Key_3", 0x04),
    ("Key_4", 0x05),
    ("Key_5", 0x06),
    ("Key_6", 0x07),
    ("Key_7", 0x08),
    ("Key_8", 0x09),
    ("Key_9", 0x0A),
    ("Key_0", 0x0B),
    ("Key_Minus", 0x0C),
    ("Key_Equals", 0x0D),
    ("Key_Backspace", 0x0E),
    ("Key_Tab", 0x0F),
    ("Key_Q", 0x10),
    ("Key_W", 0x11),
    ("Key_E", 0x12),
    ("Key_R", 0x13),
    ("Key_T", 0x14),
    ("Key_Y", 0x15),
    ("Key_U", 0x16),
    ("Key_I", 0x17),
    ("Key_O", 0x18),
    ("Key_P", 0x19),
    ("Key_LeftBracket", 0x1A),
    ("Key_RightBracket", 0x1B),
    ("Key_Enter", 0x1C),
    ("Key_LeftControl", 0x1D),
    ("Key_A", 0x1E),
    ("Key_S", 0x1F),
    ("Key_D", 0x20),
    ("Key_F", 0x21),
    ("Key_G", 0x22),
    ("Key_H", 0x23),
    ("Key_J", 0x24),
    ("Key_K", 0x25),
    ("Key_L", 0x26),
    ("Key_SemiColon", 0x27),
    ("Key_Apostrophe", 0x28),
    ("Key_Grave", 0x29),
    ("Key_LeftShift", 0x2A),
    ("Key_BackSlash", 0x2B),
    ("Key_Z", 0x2C),
    ("Key_X", 0x2D),
    ("Key_C", 0x2E),
    ("Key_V", 0x2F),
    ("Key_B", 0x30),
    ("Key_N", 0x31),
    ("Key_M", 0x32),
    ("Key_Comma", 0x33),
    ("Key_Period", 0x34),
    ("Key_Slash", 0x35),
    ("Key_RightShift", 0x36),
    ("Key_Numpad_Multiply", 0x37),
    ("Key_LeftAlt", 0x38),
    ("Key_Space", 0x39),
    ("Key_CapsLock", 0x3A),
    ("Key_F1", 0x3B),
    ("Key_F2", 0x3C),
    ("Key_F3", 0x3D),
    ("Key_F4", 0x3E),
    ("Key_F5", 0x3F),
    ("Key_F6", 0x40),
    ("Key_F7", 0x41),
    ("Key_F8", 0x42),
    ("Key_F9", 0x43),
    ("Key_F10", 0x44),
    ("Key_NumLock", 0x45),
    ("Key_ScrollLock", 0x46),
    ("Key_Numpad_7", 0x47),
    ("Key_Numpad_8", 0x48),
    ("Key_Numpad_9", 0x49),
    ("Key_Numpad_Subtract", 0x4A),
    ("Key_Numpad_4", 0x4B),
    ("Key_Numpad_5", 0x4C),
    ("Key_Numpad_6", 0x4D),
    ("Key_Numpad_Add", 0x4E),
    ("Key_Numpad_1", 0x4F),
    ("Key_Numpad_2", 0x50),
    ("Key_Numpad_3", 0x51),
    ("Key_Numpad_0", 0x52),
    ("Key_Numpad_Decimal", 0x53),
    ("Key_OEM_102", 0x56), // ISO key between left Shift and Z
    ("Key_F11", 0x57),
    ("Key_F12", 0x58),
    ("Key_F13", 0x64),
    ("Key_F14", 0x65),
    ("Key_F15", 0x66),
    ("Key_Kana", 0x70),
    ("Key_ABNT_C1", 0x73),
    ("Key_Convert", 0x79),
    ("Key_NoConvert", 0x7B),
    ("Key_Yen", 0x7D),
    ("Key_ABNT_C2", 0x7E),
    // Extended (E0-prefixed)
    ("Key_Numpad_Enter", 0x9C),
    ("Key_RightControl", 0x9D),
    ("Key_Numpad_Divide", 0xB5),
    ("Key_SYSRQ", 0xB7),
    ("Key_RightAlt", 0xB8),
    ("Key_Home", 0xC7),
    ("Key_UpArrow", 0xC8),
    ("Key_PageUp", 0xC9),
    ("Key_LeftArrow", 0xCB),
    ("Key_RightArrow", 0xCD),
    ("Key_End", 0xCF),
    ("Key_DownArrow", 0xD0),
    ("Key_PageDown", 0xD1),
    ("Key_Insert", 0xD2),
    ("Key_Delete", 0xD3),
    ("Key_LeftWin", 0xDB),
    ("Key_RightWin", 0xDC),
    ("Key_Apps", 0xDD),
];

/// Names that appear in Elite/community key lists but are NOT Elite DIK names we can trust;
/// mapped by best guess and flagged as unverified.
const UNVERIFIED: &[(&str, u16)] = &[
    ("Key_Hash", 0x2B),     // UK '#' key: same physical position as US backslash
    ("Key_LessThan", 0x56), // ISO '<>' key = OEM_102 position
];

/// Keys we deliberately refuse: they can't be sent as a single set-1 scan code
/// (Pause is E1 1D 45), are NEC PC-98-only DIK codes, or are media/power keys that act on
/// Windows itself. The user should rebind or pass --scancode.
const REFUSED: &[&str] = &[
    "Key_Pause",
    "Key_Power",
    "Key_Sleep",
    "Key_Wake",
    "Key_Mute",
    "Key_VolumeUp",
    "Key_VolumeDown",
    "Key_PlayPause",
    "Key_MediaStop",
    "Key_Stop",
    "Key_PrevTrack",
    "Key_NextTrack",
    "Key_Calculator",
    "Key_Mail",
    "Key_MediaSelect",
    "Key_MyComputer",
    "Key_WebBack",
    "Key_WebForward",
    "Key_WebHome",
    "Key_WebRefresh",
    "Key_WebSearch",
    "Key_WebStop",
    "Key_WebFavourites",
    "Key_Numpad_Equals",
    "Key_Numpad_Comma",
    "Key_AT",
    "Key_Colon",
    "Key_Underline",
    "Key_Kanji",
    "Key_AX",
    "Key_Unlabeled",
    "Key_Circumflex",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyLookup {
    Ok(ScanKey),
    Unverified(ScanKey),
    Refused(&'static str),
    Unknown,
}

fn from_dik(v: u16) -> ScanKey {
    ScanKey { scan: v & 0x7F, extended: v & 0x80 != 0 }
}

pub fn lookup(name: &str) -> KeyLookup {
    if let Some((_, v)) = TABLE.iter().find(|(n, _)| *n == name) {
        return KeyLookup::Ok(from_dik(*v));
    }
    if let Some((_, v)) = UNVERIFIED.iter().find(|(n, _)| *n == name) {
        return KeyLookup::Unverified(from_dik(*v));
    }
    if let Some(n) = REFUSED.iter().find(|n| **n == name) {
        return KeyLookup::Refused(n);
    }
    KeyLookup::Unknown
}

pub fn table() -> impl Iterator<Item = (&'static str, ScanKey)> {
    TABLE.iter().map(|(n, v)| (*n, from_dik(*v)))
}

/// Parse a user-supplied scancode such as "0x26", "26h" or "38".
pub fn parse_scancode(s: &str) -> Option<u16> {
    let s = s.trim();
    let v = if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u16::from_str_radix(h, 16).ok()?
    } else if let Some(h) = s.strip_suffix('h').or_else(|| s.strip_suffix('H')) {
        u16::from_str_radix(h, 16).ok()?
    } else {
        s.parse().ok()?
    };
    (1..=0x7F).contains(&v).then_some(v)
}
