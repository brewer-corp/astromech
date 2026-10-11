//! Read-only: resolve the active ship preset (StartPreset, as in k0-binds-doctor) and build a
//! reverse index (device, key) -> bound Elite actions. Nothing is ever written.

use crate::listen::model::{DevKind, DeviceMeta};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Default)]
pub struct BindsIndex {
    pub preset_name: String,
    pub context: String,
    pub start_preset_file: String,
    pub source: String,
    pub file: String,
    pub bound_slots: usize,
    /// "<device>|<key>" -> actions
    #[serde(skip)]
    pub map: BTreeMap<(String, String), Vec<String>>,
    #[serde(skip)]
    pub id_to_name: BTreeMap<String, String>,
    pub notes: Vec<String>,
}

impl BindsIndex {
    pub fn lookup(&self, d: Option<&DeviceMeta>, key: &str) -> Vec<String> {
        let Some(d) = d else { return vec![] };
        let mut names = vec![];
        match d.kind {
            DevKind::Keyboard => names.push("Keyboard".to_string()),
            DevKind::Mouse => names.push("Mouse".to_string()),
            DevKind::Xinput => names.push("GamePad".to_string()),
            DevKind::Hid => {
                if let Some(id) = d.elite_id() {
                    if let Some(n) = self.id_to_name.get(&id) {
                        names.push(n.clone());
                    }
                    names.push(id);
                }
            }
        }
        let mut out = vec![];
        for n in names {
            if let Some(v) = self.map.get(&(n, key.to_string())) {
                out.extend(v.iter().cloned());
            }
        }
        out
    }
}

/// Build the index from .binds XML text.
pub fn index_xml(text: &str) -> Result<BTreeMap<(String, String), Vec<String>>, String> {
    let text = text.trim_start_matches('\u{feff}');
    let doc = roxmltree::Document::parse(text).map_err(|e| e.to_string())?;
    let mut map: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for action in doc.root_element().children().filter(|n| n.is_element()) {
        let an = action.tag_name().name();
        for slot in action
            .children()
            .filter(|n| n.is_element() && matches!(n.tag_name().name(), "Primary" | "Secondary" | "Binding"))
        {
            let (Some(dev), Some(key)) = (slot.attribute("Device"), slot.attribute("Key")) else { continue };
            if dev == "{NoDevice}" || key.is_empty() {
                continue;
            }
            let mods: Vec<String> = slot
                .children()
                .filter(|m| m.has_tag_name("Modifier"))
                .map(|m| format!("{} on {}", m.attribute("Key").unwrap_or(""), m.attribute("Device").unwrap_or("")))
                .collect();
            let label = format!(
                "{an} ({}){}",
                slot.tag_name().name(),
                if mods.is_empty() { String::new() } else { format!(" + modifier {}", mods.join(", ")) }
            );
            map.entry((dev.to_string(), key.to_string())).or_default().push(label);
            for m in slot.children().filter(|m| m.has_tag_name("Modifier")) {
                if let (Some(md), Some(mk)) = (m.attribute("Device"), m.attribute("Key")) {
                    map.entry((md.to_string(), mk.to_string()))
                        .or_default()
                        .push(format!("modifier for {an} ({})", slot.tag_name().name()));
                }
            }
        }
    }
    Ok(map)
}

pub fn load(bindings_dir: &Path, game_dir: Option<&Path>) -> Result<BindsIndex, String> {
    let a = astromech_keybinds::active::resolve(bindings_dir, game_dir, 1)?;
    let map = index_xml(&a.text).map_err(|e| format!("{}: {e}", a.path.display()))?;
    Ok(BindsIndex {
        preset_name: a.preset_name,
        context: a.context,
        start_preset_file: a.start_preset_file,
        source: a.source,
        file: a.path.display().to_string(),
        bound_slots: map.values().map(|v| v.len()).sum(),
        map,
        id_to_name: a.id_to_name,
        notes: a.notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn index_and_load() {
        let d = tempfile::tempdir().unwrap();
        let x = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="VKB" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="231D3201" Key="Joy_4" /><Secondary Device="Keyboard" Key="Key_L" /></LandingGearToggle>
 <ThrottleAxis><Binding Device="231D3201" Key="Joy_YAxis" /></ThrottleAxis>
 <CycleNextTarget><Primary Device="231D0200" Key="Joy_POV1Up"><Modifier Device="231D3201" Key="Joy_4" /></Primary><Secondary Device="{NoDevice}" Key="" /></CycleNextTarget>
</Root>"#;
        std::fs::write(d.path().join("VKB.4.2.binds"), x).unwrap();
        std::fs::write(d.path().join("Other.4.2.binds"), x.replace("\"VKB\"", "\"Other\"")).unwrap();
        std::fs::write(d.path().join("StartPreset.4.start"), "Other\nVKB\nOther\nOther\n").unwrap();
        std::fs::write(d.path().join("StartPreset.start"), "Other\n").unwrap();
        let ix = load(d.path(), Some(&d.path().join("nogame"))).unwrap();
        assert_eq!(ix.preset_name, "VKB");
        assert_eq!(ix.context, "Ship");
        assert!(ix.file.ends_with("VKB.4.2.binds"));
        let dev = DeviceMeta {
            key: "x".into(),
            kind: DevKind::Hid,
            vid: 0x231D,
            pid: 0x3201,
            product: String::new(),
            usage: 4,
            buttons_declared: 0,
            axes_declared: vec![],
            hats_declared: 0,
        };
        let a = ix.lookup(Some(&dev), "Joy_4");
        assert_eq!(a, vec!["LandingGearToggle (Primary)", "modifier for CycleNextTarget (Primary)"]);
        assert_eq!(ix.lookup(Some(&dev), "Joy_YAxis"), vec!["ThrottleAxis (Binding)"]);
        let r = DeviceMeta { pid: 0x0200, ..dev.clone() };
        assert_eq!(ix.lookup(Some(&r), "Joy_POV1Up"), vec!["CycleNextTarget (Primary) + modifier Joy_4 on 231D3201"]);
        let kb = DeviceMeta { kind: DevKind::Keyboard, ..dev };
        assert_eq!(ix.lookup(Some(&kb), "Key_L"), vec!["LandingGearToggle (Secondary)"]);
    }
}
