//! Read-only parsing of `.binds`, `StartPreset*.start` and `DeviceMappings.xml`.
//! Pattern follows the S5 spike's `binds.rs` (roxmltree, BOM-tolerant), extended for diagnostics.

use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct XmlError {
    pub line: u32,
    pub col: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct EncodingInfo {
    /// "UTF-8 BOM", "UTF-16LE BOM", "UTF-16BE BOM" or null.
    pub bom: Option<String>,
    /// Declared encoding in the `<?xml ...?>` header, if any.
    pub declared: Option<String>,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct DeviceUse {
    /// Number of bound slots (Primary/Secondary/Binding) plus modifiers using this device.
    pub binds: u32,
    /// Distinct key names used on this device (e.g. Joy_12, Joy_XAxis).
    pub keys: Vec<String>,
}

/// Name parts of `<stem>.<major>.<minor>.binds`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct FileNameInfo {
    pub stem: String,
    pub suffix: Option<(u32, u32)>,
    /// e.g. "X.4.binds": a single number, not the usual major.minor pair.
    pub odd_suffix: Option<String>,
}

pub fn parse_file_name(name: &str) -> Option<FileNameInfo> {
    let lower = name.to_ascii_lowercase();
    if !lower.ends_with(".binds") {
        return None;
    }
    let base = &name[..name.len() - ".binds".len()];
    let parts: Vec<&str> = base.split('.').collect();
    let num = |s: &str| -> Option<u32> {
        if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) {
            s.parse().ok()
        } else {
            None
        }
    };
    if parts.len() >= 3 {
        if let (Some(a), Some(b)) = (num(parts[parts.len() - 2]), num(parts[parts.len() - 1])) {
            return Some(FileNameInfo {
                stem: parts[..parts.len() - 2].join("."),
                suffix: Some((a, b)),
                odd_suffix: None,
            });
        }
    }
    if parts.len() >= 2 && num(parts[parts.len() - 1]).is_some() {
        return Some(FileNameInfo {
            stem: parts[..parts.len() - 1].join("."),
            suffix: None,
            odd_suffix: Some(parts[parts.len() - 1].to_string()),
        });
    }
    Some(FileNameInfo { stem: base.to_string(), suffix: None, odd_suffix: None })
}

/// Decode bytes to text, recording BOM and encoding problems.
pub fn decode(bytes: &[u8]) -> (String, EncodingInfo) {
    let mut info = EncodingInfo::default();
    let text = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        info.bom = Some("UTF-8 BOM".into());
        utf8_lossy(&bytes[3..], &mut info)
    } else if bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]) {
        let le = bytes[0] == 0xFF;
        info.bom = Some(if le { "UTF-16LE BOM" } else { "UTF-16BE BOM" }.into());
        info.issues.push(
            "File is UTF-16 (often caused by saving from Notepad as 'Unicode' or by PowerShell redirection). Elite's own files are UTF-8."
                .into(),
        );
        let units: Vec<u16> = bytes[2..]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| if le { u16::from_le_bytes(*c) } else { u16::from_be_bytes(*c) })
            .collect();
        String::from_utf16_lossy(&units)
    } else {
        if bytes.contains(&0) {
            info.issues.push("File contains NUL bytes (UTF-16 without a BOM, or a corrupted/partial download).".into());
        }
        utf8_lossy(bytes, &mut info)
    };
    // Declared encoding.
    if let Some(start) = text.find("<?xml") {
        if let Some(end) = text[start..].find("?>") {
            let decl = &text[start..start + end];
            if let Some(p) = decl.find("encoding=") {
                let rest = &decl[p + 9..];
                let q = rest.chars().next().unwrap_or('"');
                if let Some(e) = rest[1..].find(q) {
                    info.declared = Some(rest[1..1 + e].to_string());
                }
            }
        }
    }
    if let (Some(d), Some(b)) = (&info.declared, &info.bom) {
        if b.starts_with("UTF-16") && d.eq_ignore_ascii_case("utf-8") {
            info.issues.push(format!("Header declares encoding=\"{d}\" but the file is {b}."));
        }
    }
    let leading = text.trim_start_matches('\u{feff}');
    if !leading.starts_with('<') && !leading.trim_start().is_empty() {
        let ws = leading.len() - leading.trim_start().len();
        if ws > 0 && leading.trim_start().starts_with("<?xml") {
            info.issues.push("Whitespace before the <?xml ...?> declaration (strict XML parsers reject this).".into());
        }
    }
    (text, info)
}

fn utf8_lossy(bytes: &[u8], info: &mut EncodingInfo) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(e) => {
            info.issues.push(format!(
                "Invalid UTF-8 at byte {} (file may be ANSI/Windows-1252 encoded; non-ASCII characters will be garbled).",
                e.valid_up_to()
            ));
            String::from_utf8_lossy(bytes).into_owned()
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct BindsContent {
    pub xml_error: Option<XmlError>,
    pub root_element: Option<String>,
    pub preset_name: Option<String>,
    pub major_version: Option<String>,
    pub minor_version: Option<String>,
    pub keyboard_layout: Option<String>,
    /// Number of direct children of <Root> (actions + settings).
    pub element_count: u32,
    pub devices: BTreeMap<String, DeviceUse>,
    /// Action names that appear more than once (the game uses the first, per BindingLoadingErrors.log wording).
    pub duplicate_actions: Vec<String>,
}

/// Parse a decoded .binds document. Never fails: errors are recorded.
pub fn parse_binds(text: &str) -> BindsContent {
    let mut out = BindsContent::default();
    let text = text.trim_start_matches('\u{feff}');
    let doc = match roxmltree::Document::parse(text) {
        Ok(d) => d,
        Err(e) => {
            let p = e.pos();
            let mut err = XmlError { line: p.row, col: p.col, message: e.to_string() };
            if matches!(e, roxmltree::Error::UnexpectedEndOfStream) {
                // roxmltree gives no position here; report the end of the file.
                err.line = text.lines().count().max(1) as u32;
                err.col = text.lines().last().map(|l| l.chars().count() as u32 + 1).unwrap_or(1);
                err.message = "file ends in the middle of the XML (truncated download or copy?)".into();
            }
            out.xml_error = Some(err);
            return out;
        }
    };
    let root = doc.root_element();
    out.root_element = Some(root.tag_name().name().to_string());
    out.preset_name = root.attribute("PresetName").map(str::to_string);
    out.major_version = root.attribute("MajorVersion").map(str::to_string);
    out.minor_version = root.attribute("MinorVersion").map(str::to_string);
    let mut seen: BTreeMap<String, u32> = BTreeMap::new();
    for child in root.children().filter(|n| n.is_element()) {
        out.element_count += 1;
        let name = child.tag_name().name().to_string();
        if name == "KeyboardLayout" {
            out.keyboard_layout = child.text().map(|t| t.trim().to_string());
        }
        *seen.entry(name).or_default() += 1;
    }
    out.duplicate_actions = seen.into_iter().filter(|(_, c)| *c > 1).map(|(n, _)| n).collect();
    let mut keys: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
    for node in root.descendants().filter(|n| n.is_element()) {
        let tag = node.tag_name().name();
        if !matches!(tag, "Primary" | "Secondary" | "Binding" | "Modifier") {
            continue;
        }
        let (Some(dev), Some(key)) = (node.attribute("Device"), node.attribute("Key")) else { continue };
        let (dev, key) = (dev.trim(), key.trim());
        if dev.is_empty() || dev == "{NoDevice}" || key.is_empty() {
            continue;
        }
        let u = out.devices.entry(dev.to_string()).or_default();
        u.binds += 1;
        keys.entry(dev.to_string()).or_default().insert(key.to_string());
    }
    for (dev, ks) in keys {
        if let Some(u) = out.devices.get_mut(&dev) {
            u.keys = ks.into_iter().collect();
        }
    }
    out
}

/// What a joystick key name refers to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoyKey {
    Button(u32),
    Pov(u32),
    Axis(String),
    Other,
}

pub fn classify_key(key: &str) -> JoyKey {
    if let Some(rest) = key.strip_prefix("Joy_") {
        if let Ok(n) = rest.parse::<u32>() {
            return JoyKey::Button(n);
        }
        if let Some(p) = rest.strip_prefix("POV") {
            let digits: String = p.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = digits.parse() {
                return JoyKey::Pov(n);
            }
        }
        if let Some(a) = rest.strip_suffix("Axis") {
            return JoyKey::Axis(a.to_ascii_uppercase());
        }
    }
    JoyKey::Other
}

/// `StartPreset*.start`.
#[derive(Debug, Clone, Serialize)]
pub struct StartPresetFile {
    pub file_name: String,
    /// The number in `StartPreset.<n>.start`, if any.
    pub number: Option<u32>,
    pub lines: Vec<(String, String)>, // (context label, preset name)
    pub raw_line_count: usize,
}

pub const START_CONTEXTS: [&str; 4] = ["General", "Ship", "SRV", "On-foot"];

pub fn parse_start_preset(file_name: &str, text: &str) -> StartPresetFile {
    let text = text.trim_start_matches('\u{feff}');
    let raw: Vec<&str> = text.lines().collect();
    let names: Vec<String> = raw.iter().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
    let lines = if names.len() == 1 {
        vec![("All contexts (single-line file)".to_string(), names[0].clone())]
    } else {
        names
            .iter()
            .enumerate()
            .map(|(i, n)| {
                (START_CONTEXTS.get(i).map(|s| s.to_string()).unwrap_or(format!("Extra line {}", i + 1)), n.clone())
            })
            .collect()
    };
    let lower = file_name.to_ascii_lowercase();
    let number = lower.strip_prefix("startpreset.").and_then(|r| r.strip_suffix(".start")).and_then(|n| n.parse().ok());
    StartPresetFile { file_name: file_name.to_string(), number, lines, raw_line_count: raw.len() }
}

/// DeviceMappings.xml: device name -> list of "VVVVPPPP" ids (primary first, then alternatives).
pub fn parse_device_mappings(text: &str) -> Result<BTreeMap<String, Vec<String>>, XmlError> {
    let text = text.trim_start_matches('\u{feff}');
    let doc = roxmltree::Document::parse(text).map_err(|e| {
        let p = e.pos();
        XmlError { line: p.row, col: p.col, message: e.to_string() }
    })?;
    let mut map = BTreeMap::new();
    let id_of = |n: roxmltree::Node| -> Option<String> {
        let v = n.children().find(|c| c.has_tag_name("VID"))?.text()?.trim().to_ascii_uppercase();
        let p = n.children().find(|c| c.has_tag_name("PID"))?.text()?.trim().to_ascii_uppercase();
        Some(format!("{:0>4}{:0>4}", v, p))
    };
    for dev in doc.root_element().children().filter(|n| n.is_element()) {
        let mut ids = vec![];
        if let Some(id) = id_of(dev) {
            ids.push(id);
        }
        for alt in dev.children().filter(|c| c.has_tag_name("Alternative")) {
            if let Some(id) = id_of(alt) {
                ids.push(id);
            }
        }
        map.insert(dev.tag_name().name().to_string(), ids);
    }
    Ok(map)
}

/// "231D0200" style raw device id?
pub fn parse_raw_id(device: &str) -> Option<String> {
    if device.len() == 8 && device.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(device.to_ascii_uppercase())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_names() {
        let f = parse_file_name("Custom.4.2.binds").unwrap();
        assert_eq!((f.stem.as_str(), f.suffix), ("Custom", Some((4, 2))));
        let f = parse_file_name("My.Setup.3.0.BINDS").unwrap();
        assert_eq!((f.stem.as_str(), f.suffix), ("My.Setup", Some((3, 0))));
        let f = parse_file_name("Plain.binds").unwrap();
        assert_eq!((f.stem.as_str(), f.suffix, f.odd_suffix), ("Plain", None, None));
        let f = parse_file_name("Odd.4.binds").unwrap();
        assert_eq!(f.odd_suffix.as_deref(), Some("4"));
        assert!(parse_file_name("Custom.4.2.binds.123.backup").is_none());
        assert!(parse_file_name("x.binds.txt").is_none());
    }

    #[test]
    fn decode_variants() {
        let (t, i) = decode(b"\xEF\xBB\xBF<Root/>");
        assert_eq!(t, "<Root/>");
        assert_eq!(i.bom.as_deref(), Some("UTF-8 BOM"));
        assert!(i.issues.is_empty());
        let mut u16le = vec![0xFF, 0xFE];
        for c in "<?xml version=\"1.0\" encoding=\"UTF-8\" ?><Root/>".encode_utf16() {
            u16le.extend_from_slice(&c.to_le_bytes());
        }
        let (t, i) = decode(&u16le);
        assert!(t.ends_with("<Root/>"));
        assert_eq!(i.declared.as_deref(), Some("UTF-8"));
        assert_eq!(i.issues.len(), 2, "{:?}", i.issues);
        let (_, i) = decode(b"<Root PresetName=\"\xE9\"/>");
        assert!(i.issues[0].contains("Invalid UTF-8"));
    }

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="MyVkb" MajorVersion="4" MinorVersion="2">
	<KeyboardLayout>en-GB</KeyboardLayout>
	<YawAxisRaw>
		<Binding Device="231D0200" Key="Joy_RZAxis" />
		<Inverted Value="0" />
	</YawAxisRaw>
	<LandingGearToggle>
		<Primary Device="Keyboard" Key="Key_L" />
		<Secondary Device="231D3201" Key="Joy_33">
			<Modifier Device="231D3201" Key="Joy_5" />
		</Secondary>
	</LandingGearToggle>
	<ShipSpotLightToggle>
		<Primary Device="{NoDevice}" Key="" />
		<Secondary Device="231D0200" Key="Joy_POV2Up" />
	</ShipSpotLightToggle>
	<ShipSpotLightToggle>
		<Primary Device="Keyboard" Key="Key_K" />
	</ShipSpotLightToggle>
</Root>"#;

    #[test]
    fn parse_sample() {
        let c = parse_binds(SAMPLE);
        assert!(c.xml_error.is_none());
        assert_eq!(c.preset_name.as_deref(), Some("MyVkb"));
        assert_eq!((c.major_version.as_deref(), c.minor_version.as_deref()), (Some("4"), Some("2")));
        assert_eq!(c.keyboard_layout.as_deref(), Some("en-GB"));
        assert_eq!(c.devices["231D3201"].binds, 2);
        assert_eq!(c.devices["231D3201"].keys, vec!["Joy_33", "Joy_5"]);
        assert_eq!(c.devices["231D0200"].keys, vec!["Joy_POV2Up", "Joy_RZAxis"]);
        assert_eq!(c.devices["Keyboard"].binds, 2);
        assert!(!c.devices.contains_key("{NoDevice}"));
        assert_eq!(c.duplicate_actions, vec!["ShipSpotLightToggle"]);
    }

    #[test]
    fn malformed_position() {
        let c = parse_binds("<?xml version=\"1.0\"?>\n<Root PresetName=\"x\">\n  <A>\n</Root>");
        let e = c.xml_error.unwrap();
        assert_eq!(e.line, 4);
        assert!(e.col >= 1);
        let c = parse_binds("<Root PresetName=\"x\">\n  <A>\n    <Primary Device=\"Keyb");
        let e = c.xml_error.unwrap();
        assert_eq!((e.line, e.col), (3, 26));
        assert!(e.message.contains("truncated"));
        let c = parse_binds("<Root PresetName=\"x\">\n  <A Value=\"a & b\"/>\n</Root>");
        assert_eq!(c.xml_error.unwrap().line, 2);
    }

    #[test]
    fn keys() {
        assert_eq!(classify_key("Joy_12"), JoyKey::Button(12));
        assert_eq!(classify_key("Joy_POV3Left"), JoyKey::Pov(3));
        assert_eq!(classify_key("Joy_RXAxis"), JoyKey::Axis("RX".into()));
        assert_eq!(classify_key("Joy_UAxis"), JoyKey::Axis("U".into()));
        assert_eq!(classify_key("Key_A"), JoyKey::Other);
        assert_eq!(classify_key("Pad_A"), JoyKey::Other);
    }

    #[test]
    fn start_preset() {
        let s = parse_start_preset("StartPreset.4.start", "\u{feff}A\nB\r\nC\nD\n");
        assert_eq!(s.number, Some(4));
        assert_eq!(s.lines[1], ("Ship".to_string(), "B".to_string()));
        assert_eq!(s.lines[3].0, "On-foot");
        let s = parse_start_preset("StartPreset.start", "Custom\n");
        assert_eq!(s.number, None);
        assert!(s.lines[0].0.starts_with("All contexts"));
    }

    #[test]
    fn device_mappings() {
        let m = parse_device_mappings(
            "<Root><GamePad><PID>028E</PID><VID>045E</VID><Alternative><PID>02ff</PID><VID>045e</VID></Alternative></GamePad><T16000M><VID>044F</VID><PID>B10A</PID></T16000M></Root>",
        )
        .unwrap();
        assert_eq!(m["GamePad"], vec!["045E028E", "045E02FF"]);
        assert_eq!(m["T16000M"], vec!["044FB10A"]);
        assert_eq!(parse_raw_id("231d0200").as_deref(), Some("231D0200"));
        assert!(parse_raw_id("SaitekX52").is_none());
    }
}
