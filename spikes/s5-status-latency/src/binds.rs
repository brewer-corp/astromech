//! StartPreset + .binds discovery and parsing.

use crate::keys::{self, KeyLookup, ScanKey};
use anyhow::{anyhow, bail, Context, Result};
use std::path::{Path, PathBuf};

/// Contents of StartPreset*.start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartPreset {
    pub file: PathBuf,
    /// Non-empty trimmed lines, in file order.
    pub lines: Vec<String>,
}

impl StartPreset {
    pub fn parse(file: PathBuf, text: &str) -> Result<Self> {
        let text = text.trim_start_matches('\u{feff}');
        let lines: Vec<String> = text
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        if lines.is_empty() {
            bail!("{} is empty", file.display());
        }
        Ok(Self { file, lines })
    }

    /// Preset used for the Ship context.
    /// Multi-line (Odyssey/4.x) files have one line per context: General, Ship, SRV, OnFoot.
    /// Older files have a single line used for everything.
    pub fn ship_preset(&self) -> &str {
        if self.lines.len() >= 2 { &self.lines[1] } else { &self.lines[0] }
    }

    pub fn context_names(&self) -> Vec<(&'static str, &str)> {
        const CTX: [&str; 4] = ["General", "Ship", "SRV", "OnFoot"];
        if self.lines.len() == 1 {
            return vec![("All", self.lines[0].as_str())];
        }
        self.lines
            .iter()
            .enumerate()
            .map(|(i, l)| (CTX.get(i).copied().unwrap_or("Extra"), l.as_str()))
            .collect()
    }

    pub fn all_same(&self) -> bool {
        self.lines.iter().all(|l| l == &self.lines[0])
    }
}

/// Find the newest StartPreset*.start in the bindings folder.
pub fn find_start_preset(dir: &Path) -> Result<StartPreset> {
    let mut candidates = vec![];
    for e in std::fs::read_dir(dir).with_context(|| format!("cannot read bindings folder {}", dir.display()))? {
        let e = e?;
        let name = e.file_name().to_string_lossy().to_string();
        let lower = name.to_ascii_lowercase();
        if lower.starts_with("startpreset") && lower.ends_with(".start") && e.path().is_file() {
            let mtime = e.metadata().and_then(|m| m.modified()).ok();
            candidates.push((mtime, e.path()));
        }
    }
    candidates.sort();
    let (_, path) = candidates
        .pop()
        .ok_or_else(|| anyhow!("no StartPreset*.start file in {} (run Elite once, or pass --binds-file)", dir.display()))?;
    let text = std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    StartPreset::parse(path, &text)
}

/// Version suffix of "<preset>.<major>.<minor>.binds" (or (0,0) for "<preset>.binds").
pub fn binds_version(file_name: &str, preset: &str) -> Option<(u32, u32)> {
    let lower = file_name.to_ascii_lowercase();
    let p = preset.to_ascii_lowercase();
    let rest = lower.strip_prefix(&p)?;
    let mid = rest.strip_suffix(".binds")?;
    if mid.is_empty() {
        return Some((0, 0));
    }
    let mid = mid.strip_prefix('.')?;
    let mut it = mid.split('.');
    let major = it.next()?.parse().ok()?;
    let minor = match it.next() { Some(m) => m.parse().ok()?, None => 0 };
    if it.next().is_some() {
        return None;
    }
    Some((major, minor))
}

/// Pick the .binds file for `preset` with the highest version suffix (ties: newest).
pub fn find_binds_file(dir: &Path, preset: &str) -> Result<PathBuf> {
    let mut best: Option<((u32, u32), Option<std::time::SystemTime>, PathBuf)> = None;
    let mut all = vec![];
    for e in std::fs::read_dir(dir)? {
        let e = e?;
        let name = e.file_name().to_string_lossy().to_string();
        if name.to_ascii_lowercase().ends_with(".binds") {
            all.push(name.clone());
        }
        if let Some(v) = binds_version(&name, preset) {
            let mtime = e.metadata().and_then(|m| m.modified()).ok();
            let cand = (v, mtime, e.path());
            if best.as_ref().is_none_or(|b| (cand.0, cand.1) > (b.0, b.1)) {
                best = Some(cand);
            }
        }
    }
    match best {
        Some((_, _, p)) => Ok(p),
        None => bail!(
            "no .binds file for preset '{preset}' in {}.\n\
             If '{preset}' is one of Frontier's built-in presets, the game keeps it in the install folder; \
             change any control in Options > Controls and save so the game writes a custom preset here, \
             or pass --binds-file <path>. .binds files present: {}",
            dir.display(),
            if all.is_empty() { "none".to_string() } else { all.join(", ") }
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub device: String,
    pub key: String,
    pub modifiers: Vec<(String, String)>, // (device, key)
}

impl Binding {
    pub fn is_empty(&self) -> bool {
        self.key.is_empty() || self.device == "{NoDevice}"
    }
    pub fn describe(&self) -> String {
        if self.is_empty() {
            return "(unbound)".into();
        }
        let mut s = String::new();
        for (d, k) in &self.modifiers {
            s += &format!("{k}[{d}] + ");
        }
        s += &format!("{}[{}]", self.key, self.device);
        s
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionBinds {
    pub primary: Option<Binding>,
    pub secondary: Option<Binding>,
}

#[derive(Debug, Clone)]
pub struct BindsDoc {
    pub preset_name: Option<String>,
    pub version: Option<String>,
    pub keyboard_layout: Option<String>,
    xml: String,
}

impl BindsDoc {
    pub fn parse(xml: &str) -> Result<Self> {
        let xml = xml.trim_start_matches('\u{feff}').to_string();
        let doc = roxmltree::Document::parse(&xml).context("binds file is not valid XML")?;
        let root = doc.root_element();
        if root.tag_name().name() != "Root" {
            bail!("unexpected root element <{}> (expected <Root>)", root.tag_name().name());
        }
        let preset_name = root.attribute("PresetName").map(str::to_string);
        let version = match (root.attribute("MajorVersion"), root.attribute("MinorVersion")) {
            (Some(a), Some(b)) => Some(format!("{a}.{b}")),
            _ => None,
        };
        let keyboard_layout = root
            .children()
            .find(|n| n.has_tag_name("KeyboardLayout"))
            .and_then(|n| n.text())
            .map(|t| t.trim().to_string());
        Ok(Self { preset_name, version, keyboard_layout, xml })
    }

    pub fn action(&self, name: &str) -> Option<ActionBinds> {
        let doc = roxmltree::Document::parse(&self.xml).ok()?;
        let node = doc.root_element().children().find(|n| n.has_tag_name(name))?;
        let read = |slot: &str| {
            node.children().find(|n| n.has_tag_name(slot)).map(|b| Binding {
                device: b.attribute("Device").unwrap_or("").trim().to_string(),
                key: b.attribute("Key").unwrap_or("").trim().to_string(),
                modifiers: b
                    .children()
                    .filter(|m| m.has_tag_name("Modifier"))
                    .map(|m| {
                        (
                            m.attribute("Device").unwrap_or("").trim().to_string(),
                            m.attribute("Key").unwrap_or("").trim().to_string(),
                        )
                    })
                    .collect(),
            })
        };
        Some(ActionBinds { primary: read("Primary"), secondary: read("Secondary") })
    }
}

/// A keyboard chord we can press.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub source: String, // "Primary" / "Secondary" / "--scancode"
    pub key_name: String,
    pub key: ScanKey,
    pub modifiers: Vec<(String, ScanKey)>,
    pub notes: Vec<String>,
}

impl Chord {
    pub fn describe(&self) -> String {
        let mut s = String::new();
        for (n, k) in &self.modifiers {
            s += &format!("{n} ({k}) + ");
        }
        s += &format!("{} ({})", self.key_name, self.key);
        s
    }
}

fn map_key(name: &str, notes: &mut Vec<String>) -> std::result::Result<ScanKey, String> {
    match keys::lookup(name) {
        KeyLookup::Ok(k) => Ok(k),
        KeyLookup::Unverified(k) => {
            notes.push(format!("{name} -> {k} is a best-guess mapping (unverified)"));
            Ok(k)
        }
        KeyLookup::Refused(_) => Err(format!("{name} is not supported by this spike (Pause/media/PC-98 key)")),
        KeyLookup::Unknown => Err(format!("{name} is not in the scancode table")),
    }
}

fn try_binding(slot: &str, b: &Binding) -> std::result::Result<Chord, String> {
    if b.is_empty() {
        return Err(format!("{slot}: unbound"));
    }
    if b.device != "Keyboard" {
        return Err(format!("{slot}: bound to device '{}' ({}), not the keyboard", b.device, b.key));
    }
    let mut notes = vec![];
    let key = map_key(&b.key, &mut notes).map_err(|e| format!("{slot}: {e}"))?;
    let mut modifiers = vec![];
    for (d, k) in &b.modifiers {
        if d != "Keyboard" {
            return Err(format!("{slot}: modifier {k} is on device '{d}', not the keyboard"));
        }
        let mk = map_key(k, &mut notes).map_err(|e| format!("{slot}: modifier {e}"))?;
        modifiers.push((k.clone(), mk));
    }
    Ok(Chord { source: slot.to_string(), key_name: b.key.clone(), key, modifiers, notes })
}

/// Primary if it's a usable keyboard binding, otherwise Secondary.
pub fn select_keyboard_chord(action: &str, ab: &ActionBinds) -> Result<Chord> {
    let mut reasons = vec![];
    for (slot, b) in [("Primary", &ab.primary), ("Secondary", &ab.secondary)] {
        match b {
            Some(b) => match try_binding(slot, b) {
                Ok(c) => {
                    let mut c = c;
                    if slot == "Secondary" {
                        c.notes.insert(0, format!("Primary not usable: {}", reasons.join("; ")));
                    }
                    return Ok(c);
                }
                Err(e) => reasons.push(e),
            },
            None => reasons.push(format!("{slot}: missing")),
        }
    }
    bail!(
        "{action} has no usable keyboard binding ({}).\n\
         Fix: in Elite, Options > Controls, give {action} a keyboard key (the Secondary slot is fine, \
         so your HOTAS Primary stays), or run with --scancode <code> [--extended] to press a key you choose.",
        reasons.join("; ")
    )
}
