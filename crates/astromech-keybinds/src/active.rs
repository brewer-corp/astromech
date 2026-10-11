//! The active preset and its action → binding index.
//!
//! Resolves which preset Elite will load for a context (StartPreset*.start, as in the health
//! check), finds the file (your `.binds` files first, then the game's built-in ControlSchemes)
//! and indexes it by action. Read-only.

use crate::{binds, game};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// StartPreset line order (4.x): General, Ship, SRV, OnFoot.
pub const CONTEXTS: [&str; 4] = ["General", "Ship", "SRV", "OnFoot"];

#[derive(Debug, Clone, Serialize)]
pub struct ActivePreset {
    pub preset_name: String,
    pub context: String,
    pub start_preset_file: String,
    /// "your file" or "built-in preset"
    pub source: String,
    pub path: PathBuf,
    #[serde(skip)]
    pub text: String,
    /// Elite device ID (VIDPID) -> DeviceMappings.xml name, from the game install.
    #[serde(skip)]
    pub id_to_name: BTreeMap<String, String>,
    pub notes: Vec<String>,
}

fn read_text(p: &Path) -> Option<String> {
    std::fs::read(p).ok().map(|b| binds::decode(&b).0)
}

/// Resolve the active preset for one context (`context_line`: 0 General, 1 Ship, 2 SRV, 3 OnFoot).
pub fn resolve(bindings_dir: &Path, game_dir: Option<&Path>, context_line: usize) -> Result<ActivePreset, String> {
    let mut notes = vec![];
    let rd = std::fs::read_dir(bindings_dir)
        .map_err(|e| format!("cannot read Bindings folder {}: {e}", bindings_dir.display()))?;
    let mut starts = vec![];
    let mut files: Vec<(PathBuf, Option<String>, Option<(u32, u32)>)> = vec![];
    for e in rd.flatten() {
        let p = e.path();
        if !p.is_file() {
            continue;
        }
        let name = e.file_name().to_string_lossy().to_string();
        let low = name.to_ascii_lowercase();
        if low.starts_with("startpreset") && low.ends_with(".start") {
            if let Some(t) = read_text(&p) {
                starts.push(binds::parse_start_preset(&name, &t));
            }
        } else if let Some(fi) = binds::parse_file_name(&name) {
            let pn = read_text(&p).and_then(|t| binds::parse_binds(&t).preset_name);
            files.push((p, pn, fi.suffix));
        }
    }
    starts.sort_by_key(|s| s.number.unwrap_or(0));
    let sp = starts.pop().ok_or("no StartPreset*.start file in the Bindings folder")?;
    let (context, name) =
        sp.lines.get(context_line).or_else(|| sp.lines.first()).cloned().ok_or("StartPreset file is empty")?;
    let installs = game::locate_installs(game_dir, &mut notes);
    let mut id_to_name = BTreeMap::new();
    for g in &installs {
        if let Some(t) = read_text(&g.product_dir.join("ControlSchemes").join("DeviceMappings.xml")) {
            if let Ok(m) = binds::parse_device_mappings(&t) {
                for (n, ids) in m {
                    for id in ids {
                        id_to_name.entry(id).or_insert(n.clone());
                    }
                }
                break;
            }
        }
    }
    let mut mine: Vec<&(PathBuf, Option<String>, Option<(u32, u32)>)> =
        files.iter().filter(|f| f.1.as_deref() == Some(name.as_str())).collect();
    mine.sort_by_key(|f| f.2);
    let (path, source) = if let Some(f) = mine.last() {
        if mine.len() > 1 {
            notes.push(format!("{} files use PresetName \"{name}\"; using the highest version suffix.", mine.len()));
        }
        (Some(f.0.clone()), "your file")
    } else {
        let mut found = None;
        'outer: for g in &installs {
            if let Ok(rd) = std::fs::read_dir(g.product_dir.join("ControlSchemes")) {
                for e in rd.flatten() {
                    let p = e.path();
                    if binds::parse_file_name(&e.file_name().to_string_lossy()).is_some()
                        && read_text(&p).and_then(|t| binds::parse_binds(&t).preset_name).as_deref()
                            == Some(name.as_str())
                    {
                        found = Some(p);
                        break 'outer;
                    }
                }
            }
        }
        (found, "built-in preset")
    };
    let path = path.ok_or_else(|| {
        format!("active {context} preset \"{name}\" not found among your .binds files or the game's built-in presets")
    })?;
    let text = read_text(&path).ok_or("cannot read preset file")?;
    Ok(ActivePreset {
        preset_name: name,
        context,
        start_preset_file: sp.file_name.clone(),
        source: source.into(),
        path,
        text,
        id_to_name,
        notes,
    })
}

/// One bound slot of an action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Slot {
    /// "Primary", "Secondary" or "Binding" (axes)
    pub slot: String,
    pub device: String,
    pub key: String,
    /// (device, key) of each modifier that must be held.
    pub modifiers: Vec<(String, String)>,
    /// `<Hold Value="1"/>`-style hold flag on the action, if any.
    pub hold: bool,
}

/// A keyboard binding ready to press: modifiers first, then the key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KeyCombo {
    pub key: String,
    pub modifiers: Vec<String>,
    pub slot: String,
}

#[derive(Debug, Clone, Default)]
pub struct ActionIndex {
    pub actions: BTreeMap<String, Vec<Slot>>,
}

impl ActionIndex {
    pub fn from_xml(text: &str) -> Result<Self, String> {
        let text = text.trim_start_matches('\u{feff}');
        let doc = roxmltree::Document::parse(text).map_err(|e| e.to_string())?;
        let mut actions: BTreeMap<String, Vec<Slot>> = BTreeMap::new();
        for action in doc.root_element().children().filter(|n| n.is_element()) {
            let name = action.tag_name().name().to_string();
            let hold = action
                .children()
                .any(|n| n.has_tag_name("Hold") && n.attribute("Value").map(|v| v == "1").unwrap_or(false));
            for s in action
                .children()
                .filter(|n| n.is_element() && matches!(n.tag_name().name(), "Primary" | "Secondary" | "Binding"))
            {
                let (Some(dev), Some(key)) = (s.attribute("Device"), s.attribute("Key")) else { continue };
                if dev == "{NoDevice}" || key.is_empty() {
                    continue;
                }
                let modifiers = s
                    .children()
                    .filter(|m| m.has_tag_name("Modifier"))
                    .filter_map(|m| Some((m.attribute("Device")?.to_string(), m.attribute("Key")?.to_string())))
                    .collect();
                actions.entry(name.clone()).or_default().push(Slot {
                    slot: s.tag_name().name().into(),
                    device: dev.into(),
                    key: key.into(),
                    modifiers,
                    hold,
                });
            }
        }
        Ok(Self { actions })
    }

    pub fn slots(&self, action: &str) -> &[Slot] {
        self.actions.get(action).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// The keyboard binding we can press for `action`: a Keyboard slot whose modifiers are
    /// all keyboard keys too (Primary preferred). `None` if only joystick/mouse binds exist.
    pub fn keyboard(&self, action: &str) -> Option<KeyCombo> {
        let mut v: Vec<&Slot> = self
            .slots(action)
            .iter()
            .filter(|s| s.device == "Keyboard" && s.modifiers.iter().all(|m| m.0 == "Keyboard"))
            .collect();
        v.sort_by_key(|s| s.slot != "Primary");
        v.first().map(|s| KeyCombo {
            key: s.key.clone(),
            modifiers: s.modifiers.iter().map(|m| m.1.clone()).collect(),
            slot: s.slot.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const X: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="DualVKB" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="231D3201" Key="Joy_4" /><Secondary Device="Keyboard" Key="Key_L" /></LandingGearToggle>
 <ShipSpotLightToggle><Primary Device="Keyboard" Key="Key_L"><Modifier Device="Keyboard" Key="Key_LeftShift" /></Primary><Secondary Device="{NoDevice}" Key="" /></ShipSpotLightToggle>
 <DeployHardpointToggle><Primary Device="231D0200" Key="Joy_2" /><Secondary Device="Keyboard" Key="Key_U"><Modifier Device="231D3201" Key="Joy_5" /></Secondary></DeployHardpointToggle>
 <ThrottleAxis><Binding Device="231D3201" Key="Joy_YAxis" /></ThrottleAxis>
</Root>"#;

    #[test]
    fn keyboard_bindings() {
        let ix = ActionIndex::from_xml(X).unwrap();
        assert_eq!(ix.keyboard("LandingGearToggle").unwrap().key, "Key_L");
        let l = ix.keyboard("ShipSpotLightToggle").unwrap();
        assert_eq!((l.key.as_str(), l.modifiers.clone()), ("Key_L", vec!["Key_LeftShift".to_string()]));
        assert!(ix.keyboard("DeployHardpointToggle").is_none(), "joystick modifier: not pressable");
        assert!(ix.keyboard("NightVisionToggle").is_none());
        assert_eq!(ix.slots("ThrottleAxis")[0].key, "Joy_YAxis");
    }

    #[test]
    fn resolves_ship_context() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("DualVKB.4.2.binds"), X).unwrap();
        std::fs::write(d.path().join("DualVKB.4.1.binds"), X).unwrap();
        std::fs::write(d.path().join("StartPreset.4.start"), "Other\nDualVKB\nDualVKB\nDualVKB\n").unwrap();
        let a = resolve(d.path(), Some(&d.path().join("nogame")), 1).unwrap();
        assert_eq!((a.preset_name.as_str(), a.context.as_str(), a.source.as_str()), ("DualVKB", "Ship", "your file"));
        assert!(a.path.ends_with("DualVKB.4.2.binds"));
        assert!(a.notes.iter().any(|n| n.contains("2 files use PresetName")));
        assert!(resolve(d.path(), Some(&d.path().join("nogame")), 0).is_err(), "General preset 'Other' is missing");
    }
}
