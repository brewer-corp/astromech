//! What the app window needs from the keybinds code, as one object with JSON-friendly results
//! (the Tauri shell exposes each method as a command). Everything that writes goes through
//! [`crate::edit`] (backups first).

use crate::devices::Inventory;
use crate::edit::{self, Backup, EditResult};
use crate::profile::{self, ProfileSummary, Slot, View};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub struct Api {
    pub bindings_dir: PathBuf,
    pub builtin_dirs: Vec<PathBuf>,
    pub inventory: Inventory,
    /// Where per-user overrides live (`%LOCALAPPDATA%\astromech\data`).
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct State {
    pub bindings_dir: PathBuf,
    pub bindings_found: bool,
    pub presets: Vec<ProfileSummary>,
    /// Active preset name per context (General, Ship, SRV, On foot).
    pub active: Vec<String>,
    pub backups: Vec<Backup>,
    pub inventory: Inventory,
    pub overrides: Overrides,
    pub notes: Vec<String>,
}

/// "That's not where I pressed": per device, sheet control → the Joy name it really sends.
pub type Overrides = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Debug, Clone, Deserialize)]
pub struct SlotEdit {
    pub path: PathBuf,
    pub action: String,
    pub slot: String,
    pub analogue: bool,
    /// None clears the slot.
    pub binding: Option<Slot>,
}

impl Api {
    pub fn discover(bindings_dir: Option<PathBuf>, game_dir: Option<&Path>, data_dir: PathBuf) -> Result<Api> {
        let bindings_dir =
            bindings_dir.or_else(crate::game::default_bindings_dir).context("can't find the Bindings folder")?;
        let mut notes = vec![];
        let builtin_dirs = crate::game::locate_installs(game_dir, &mut notes)
            .into_iter()
            .map(|g| g.product_dir.join("ControlSchemes"))
            .filter(|p| p.is_dir())
            .collect();
        Ok(Api { bindings_dir, builtin_dirs, inventory: crate::devices::enumerate_live(), data_dir })
    }

    pub fn refresh_devices(&mut self) {
        self.inventory = crate::devices::enumerate_live();
    }

    pub fn state(&self) -> State {
        let inv = (!self.inventory.controllers.is_empty()).then_some(&self.inventory);
        let mut notes = vec![];
        if self.builtin_dirs.is_empty() {
            notes.push("Game install not found: built-in presets aren't listed.".into());
        }
        State {
            bindings_dir: self.bindings_dir.clone(),
            bindings_found: self.bindings_dir.is_dir(),
            presets: profile::list(&self.bindings_dir, &self.builtin_dirs, inv),
            active: profile::start_preset(&self.bindings_dir).names,
            backups: edit::list_backups(&self.bindings_dir),
            inventory: self.inventory.clone(),
            overrides: self.overrides(),
            notes,
        }
    }

    fn allowed(&self, p: &Path) -> bool {
        let Ok(c) = p.canonicalize() else { return false };
        std::iter::once(&self.bindings_dir)
            .chain(self.builtin_dirs.iter())
            .filter_map(|d| d.canonicalize().ok())
            .any(|d| c.parent() == Some(d.as_path()))
    }

    pub fn view(&self, path: &Path) -> Result<View> {
        if !self.allowed(path) {
            bail!("not a preset in the Bindings folder or the game's ControlSchemes");
        }
        let editable = path
            .parent()
            .and_then(|p| p.canonicalize().ok())
            .zip(self.bindings_dir.canonicalize().ok())
            .is_some_and(|(a, b)| a == b);
        let inv = (!self.inventory.controllers.is_empty()).then_some(&self.inventory);
        profile::view(path, editable, inv).map_err(anyhow::Error::msg)
    }

    pub fn set_slot(&self, e: &SlotEdit) -> Result<EditResult> {
        edit::set_slot(&self.bindings_dir, &e.path, &e.action, &e.slot, e.binding.as_ref(), e.analogue)
    }

    pub fn copy_preset(&self, src: &Path, name: &str) -> Result<PathBuf> {
        if !self.allowed(src) {
            bail!("not a preset astromech knows about");
        }
        edit::copy_preset(&self.bindings_dir, src, name)
    }

    pub fn set_active(&self, name: &str, contexts: &[usize]) -> Result<EditResult> {
        if !self.state().presets.iter().any(|p| p.name == name) {
            bail!("no preset called {name:?}");
        }
        edit::set_active(&self.bindings_dir, name, contexts)
    }

    pub fn restore(&self, backup: &Path) -> Result<PathBuf> {
        edit::restore(&self.bindings_dir, backup)
    }

    fn overrides_path(&self) -> PathBuf {
        self.data_dir.join("device-overrides.json")
    }

    pub fn overrides(&self) -> Overrides {
        std::fs::read_to_string(self.overrides_path())
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_default()
    }

    /// Record (or with `joy: None`, forget) that `control` on `device` sends `joy`.
    pub fn set_override(&self, device: &str, control: &str, joy: Option<&str>) -> Result<Overrides> {
        let mut o = self.overrides();
        match joy {
            Some(j) => {
                o.entry(device.into()).or_default().insert(control.into(), j.into());
            }
            None => {
                if let Some(m) = o.get_mut(device) {
                    m.remove(control);
                }
                o.retain(|_, m| !m.is_empty());
            }
        }
        std::fs::create_dir_all(&self.data_dir)?;
        edit::write_atomic(&self.overrides_path(), serde_json::to_string_pretty(&o)?.as_bytes())?;
        Ok(o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::tests::{inv, X};

    #[test]
    fn api_round_trip() {
        let d = tempfile::tempdir().unwrap();
        let b = d.path().join("Bindings");
        std::fs::create_dir_all(&b).unwrap();
        std::fs::write(b.join("DualVKB.4.2.binds"), X).unwrap();
        std::fs::write(b.join("StartPreset.4.start"), "DualVKB\nDualVKB\nDualVKB\nDualVKB").unwrap();
        let api = Api {
            bindings_dir: b.clone(),
            builtin_dirs: vec![],
            inventory: inv(&["231D3201"]),
            data_dir: d.path().join("data"),
        };
        let st = api.state();
        assert_eq!(st.presets[0].name, "DualVKB");
        let p = st.presets[0].path.clone();
        assert!(api.view(&p).unwrap().editable);
        assert!(api.view(&d.path().join("x.binds")).is_err());
        let e = SlotEdit {
            path: p.clone(),
            action: "NightVisionToggle".into(),
            slot: "Secondary".into(),
            analogue: false,
            binding: Some(Slot {
                slot: "Secondary".into(),
                device: "Keyboard".into(),
                key: "Key_N".into(),
                modifiers: vec![],
            }),
        };
        api.set_slot(&e).unwrap();
        assert_eq!(api.state().backups.len(), 1);
        let c = api.copy_preset(&p, "Mine").unwrap();
        api.set_active("Mine", &[1]).unwrap();
        assert_eq!(api.state().active[1], "Mine");
        assert!(api.set_active("Nope", &[1]).is_err());
        assert!(c.exists());
        let o = api.set_override("231D3201", "A1_PUSH", Some("Joy_30")).unwrap();
        assert_eq!(o["231D3201"]["A1_PUSH"], "Joy_30");
        assert!(api.set_override("231D3201", "A1_PUSH", None).unwrap().is_empty());
    }
}
