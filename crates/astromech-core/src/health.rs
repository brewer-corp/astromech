//! Binds health check in the core (KEYBINDS.md §7.6): the full `k0-binds-doctor` check, the
//! active ship preset, and whether each voice tool's action has a keyboard key we can press.
//! Runs at startup and whenever the Bindings folder changes.

use anyhow::{Context, Result};
use astromech_keybinds::active::{self, ActionIndex, KeyCombo};
use astromech_keybinds::doctor::{self, Confidence};
use astromech_keybinds::{devices, game};
use astromech_tools::Toggle;
use notify::{RecursiveMode, Watcher};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    /// "error" (a voice action can't work), "warning", "info"
    pub level: String,
    pub text: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BindsState {
    pub checked_at: String,
    pub bindings_dir: String,
    pub preset: Option<String>,
    pub preset_file: Option<String>,
    pub preset_source: Option<String>,
    pub issues: Vec<Issue>,
    /// Action → keyboard binding for each voice tool (None: not pressable).
    pub tool_keys: Vec<(String, Option<KeyCombo>)>,
    #[serde(skip)]
    pub index: Option<ActionIndex>,
    #[serde(skip)]
    pub doctor: Option<doctor::Report>,
}

impl BindsState {
    pub fn keyboard(&self, action: &str) -> Option<KeyCombo> {
        self.index.as_ref()?.keyboard(action)
    }

    /// Why `action` can't be pressed, in words for speech.
    /// The spoken refusal when a tool's action has no keyboard key. Uses the name Elite's
    /// controls screen shows, not the internal action ID (that goes to the console log).
    pub fn missing_reason(&self, action: &str, noun: &str) -> String {
        let ui = elite_ui_name(action);
        match &self.index {
            None => format!(
                "Cannot switch {}: your Elite keybindings could not be read. Check the health report.",
                noun.to_lowercase()
            ),
            Some(ix) if ix.slots(action).is_empty() => format!(
                "Cannot switch {}: it is not bound to anything in your controls. In Elite, open Options, Controls, \
                 and give {ui} a keyboard key.",
                noun.to_lowercase()
            ),
            Some(_) => format!(
                "Cannot switch {}: it is only bound to your controller, and astromech presses keyboard keys only. \
                 In Elite, open Options, Controls, and give {ui} a keyboard key as its secondary binding.",
                noun.to_lowercase()
            ),
        }
    }

    pub fn summary(&self) -> String {
        let mut o = format!("Binds health check ({}):\n", self.checked_at);
        match (&self.preset, &self.preset_file) {
            (Some(p), Some(f)) => {
                o += &format!(
                    "  active ship preset: \"{p}\" ({}, {})\n",
                    self.preset_source.clone().unwrap_or_default(),
                    Path::new(f).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()
                )
            }
            _ => o += "  active ship preset: NOT RESOLVED\n",
        }
        for (a, k) in &self.tool_keys {
            o += &format!(
                "  {a:<24} {}\n",
                match k {
                    Some(k) if k.modifiers.is_empty() => k.key.clone(),
                    Some(k) => format!("{} + {}", k.modifiers.join(" + "), k.key),
                    None => "no keyboard binding".into(),
                }
            );
        }
        if self.issues.is_empty() {
            o += "  no problems found\n";
        }
        for i in &self.issues {
            o += &format!("  [{}] {}\n", i.level, i.text);
        }
        o
    }
}

/// Run the whole check once.
pub fn run_check(bindings_dir: &Path, game_dir: Option<&Path>, live_devices: bool) -> BindsState {
    let mut st = BindsState {
        checked_at: chrono::Local::now().format("%H:%M:%S").to_string(),
        bindings_dir: bindings_dir.display().to_string(),
        ..Default::default()
    };
    let mut notes = vec![];
    let installs = game::locate_installs(game_dir, &mut notes);
    let inventory = if live_devices { devices::enumerate_live() } else { devices::Inventory::default() };
    let rep = doctor::run(doctor::Inputs {
        bindings_dir,
        installs,
        inventory,
        extra_log_dirs: game::default_logs_dir().into_iter().collect(),
        notes,
    });
    for f in &rep.files {
        for r in f.reasons.iter().filter(|r| r.confidence <= Confidence::Medium) {
            st.issues.push(Issue {
                level: if r.confidence == Confidence::High { "warning" } else { "info" }.into(),
                text: format!("{}: {}", f.file_name, r.title),
            });
        }
    }
    match active::resolve(bindings_dir, game_dir, 1) {
        Ok(a) => {
            for n in &a.notes {
                st.issues.push(Issue { level: "info".into(), text: n.clone() });
            }
            st.preset = Some(a.preset_name.clone());
            st.preset_file = Some(a.path.display().to_string());
            st.preset_source = Some(a.source.clone());
            match ActionIndex::from_xml(&a.text) {
                Ok(ix) => st.index = Some(ix),
                Err(e) => {
                    st.issues.push(Issue { level: "error".into(), text: format!("active preset unreadable: {e}") })
                }
            }
        }
        Err(e) => st.issues.push(Issue { level: "error".into(), text: format!("active ship preset: {e}") }),
    }
    for t in Toggle::all() {
        let sp = t.spec();
        let k = st.keyboard(sp.action);
        if k.is_none() && st.index.is_some() {
            st.issues.push(Issue {
                level: "error".into(),
                text: format!(
                    "{} has no keyboard binding: voice \"{}\" commands can't press it",
                    sp.action,
                    sp.noun.to_lowercase()
                ),
            });
        }
        if let Some(k) = &k {
            use astromech_input::exec::keys::{lookup, KeyLookup};
            for name in k.modifiers.iter().chain(std::iter::once(&k.key)) {
                let why = match lookup(name) {
                    KeyLookup::Refused(_) => Some("it can't be sent as one scan code; bind another key".to_string()),
                    KeyLookup::Unknown => Some("unknown key name".to_string()),
                    _ => None,
                };
                if let Some(why) = why {
                    st.issues.push(Issue {
                        level: "error".into(),
                        text: format!("{}: key {name} can't be sent ({why})", sp.action),
                    });
                }
            }
        }
        st.tool_keys.push((sp.action.to_string(), k));
    }
    st.doctor = Some(rep);
    st
}

pub type Shared = Arc<RwLock<BindsState>>;

/// Watch the Bindings folder and re-run the check (debounced 500 ms) on any change.
pub struct HealthWatch {
    _watcher: notify::RecommendedWatcher,
}

pub fn watch(
    bindings_dir: PathBuf,
    game_dir: Option<PathBuf>,
    live_devices: bool,
    shared: Shared,
    on_change: impl Fn(&BindsState) + Send + 'static,
) -> Result<HealthWatch> {
    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();
    let mut w = notify::recommended_watcher(tx).context("creating Bindings watcher")?;
    w.watch(&bindings_dir, RecursiveMode::Recursive).with_context(|| format!("watching {}", bindings_dir.display()))?;
    std::thread::Builder::new().name("health".into()).spawn(move || {
        while let Ok(first) = rx.recv() {
            if first.is_err() {
                continue;
            }
            let until = Instant::now() + Duration::from_millis(500);
            while let Some(left) = until.checked_duration_since(Instant::now()) {
                if rx.recv_timeout(left).is_err() {
                    break;
                }
            }
            let st = run_check(&bindings_dir, game_dir.as_deref(), live_devices);
            on_change(&st);
            *shared.write().unwrap() = st;
        }
    })?;
    Ok(HealthWatch { _watcher: w })
}

/// Elite's controls-screen label for an action (for spoken hints).
pub fn elite_ui_name(action: &str) -> &str {
    match action {
        "LandingGearToggle" => "Landing Gear",
        "ShipSpotLightToggle" => "Ship Lights",
        "DeployHardpointToggle" => "Deploy Hardpoints",
        "ToggleCargoScoop" => "Cargo Scoop",
        "NightVisionToggle" => "Night Vision",
        "ToggleFlightAssist" => "Flight Assist",
        "ToggleButtonUpInput" => "Silent Running",
        "PlayerHUDModeToggle" => "HUD Mode",
        "GalaxyMapOpen" => "Galaxy Map",
        "SystemMapOpen" => "System Map",
        "UI_Back" => "UI Back",
        "FocusLeftPanel" => "Left Panel",
        "FocusRightPanel" => "Right Panel",
        "FocusCommsPanel" => "Comms Panel",
        "FocusRadarPanel" => "Role Panel",
        "SetSpeedZero" => "Throttle to Zero",
        "AutoBreakBuggyButton" => "Handbrake",
        "ToggleDriveAssist" => "Drive Assist",
        a => a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_missing_keyboard_binding() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(
            d.path().join("Pads.4.2.binds"),
            r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="Pads" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="231D3201" Key="Joy_4" /><Secondary Device="{NoDevice}" Key="" /></LandingGearToggle>
 <ShipSpotLightToggle><Primary Device="Keyboard" Key="Key_L" /><Secondary Device="{NoDevice}" Key="" /></ShipSpotLightToggle>
</Root>"#,
        )
        .unwrap();
        std::fs::write(d.path().join("StartPreset.4.start"), "Pads\nPads\nPads\nPads\n").unwrap();
        let st = run_check(d.path(), Some(&d.path().join("nogame")), false);
        assert_eq!(st.preset.as_deref(), Some("Pads"));
        assert!(st.keyboard("LandingGearToggle").is_none());
        assert_eq!(st.keyboard("ShipSpotLightToggle").unwrap().key, "Key_L");
        assert!(st
            .issues
            .iter()
            .any(|i| i.level == "error" && i.text.starts_with("LandingGearToggle has no keyboard binding")));
        assert!(st.missing_reason("LandingGearToggle", "Landing gear").contains("only bound to your controller"));
        assert!(st.missing_reason("NightVisionToggle", "Night vision").contains("is not bound"));
        assert!(st.summary().contains("no keyboard binding"));
    }
}
