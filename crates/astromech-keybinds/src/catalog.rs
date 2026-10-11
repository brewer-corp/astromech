//! The action catalogue: every Elite binding name with a readable mixed-case label, a list group
//! and a context (Ship, SRV, On foot, General), from EDRefCard's control list
//! (`data/catalog/controls.json`, MIT). Plus the activity lists (`data/catalog/activities.json`,
//! from docs/V1-DESIGN.md §3.3).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub id: String,
    pub name: String,
    pub group: String,
    pub context: String,
    pub group_order: u32,
    pub order: u32,
    pub analogue: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityAction {
    pub id: String,
    pub important: bool,
    #[serde(default)]
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub title: String,
    pub actions: Vec<ActivityAction>,
}

#[derive(Deserialize)]
struct ControlsFile {
    controls: Vec<Control>,
}
#[derive(Deserialize)]
struct ActivitiesFile {
    activities: Vec<Activity>,
}

pub struct Catalog {
    pub controls: Vec<Control>,
    pub activities: Vec<Activity>,
    by_id: HashMap<String, usize>,
}

/// Actions the voice copilot presses (they need a keyboard key: the health rule "No keyboard key").
pub const VOICE_ACTIONS: &[&str] = &[
    "LandingGearToggle",
    "ShipSpotLightToggle",
    "DeployHardpointToggle",
    "ToggleCargoScoop",
    "NightVisionToggle",
    "ToggleFlightAssist",
    "ToggleButtonUpInput",
    "PlayerHUDModeToggle",
    "FocusLeftPanel",
    "FocusRightPanel",
    "SetSpeedZero",
];

impl Catalog {
    pub fn get() -> &'static Catalog {
        static C: OnceLock<Catalog> = OnceLock::new();
        C.get_or_init(|| {
            let c: ControlsFile = serde_json::from_str(include_str!("../../../data/catalog/controls.json"))
                .expect("bundled controls.json");
            let a: ActivitiesFile = serde_json::from_str(include_str!("../../../data/catalog/activities.json"))
                .expect("bundled activities.json");
            let by_id = c.controls.iter().enumerate().map(|(i, c)| (c.id.clone(), i)).collect();
            Catalog { controls: c.controls, activities: a.activities, by_id }
        })
    }

    pub fn control(&self, id: &str) -> Option<&Control> {
        self.by_id.get(id).map(|&i| &self.controls[i])
    }
}

/// A readable label for an action id not in the catalogue: "ToggleFooBar_Buggy" → "Toggle foo bar (SRV)".
pub fn humanize(id: &str) -> String {
    let (base, suffix) = match id.rsplit_once('_') {
        Some((b, "Buggy")) => (b, " (SRV)"),
        Some((b, "Humanoid")) => (b, " (on foot)"),
        _ => (id, ""),
    };
    let mut out = String::new();
    let mut prev_lower = false;
    for ch in base.chars() {
        if ch == '_' {
            out.push(' ');
            prev_lower = false;
            continue;
        }
        if ch.is_uppercase() && prev_lower {
            out.push(' ');
        }
        prev_lower = ch.is_lowercase() || ch.is_ascii_digit();
        out.push(ch);
    }
    let mut words: Vec<String> = out.split_whitespace().map(String::from).collect();
    for (i, w) in words.iter_mut().enumerate() {
        if i > 0 && w.chars().skip(1).all(|c| c.is_lowercase()) {
            *w = w.to_lowercase();
        }
    }
    words.join(" ") + suffix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_catalogue_loads() {
        let c = Catalog::get();
        assert!(c.controls.len() > 400);
        assert_eq!(c.control("LandingGearToggle").unwrap().name, "Landing gear");
        assert_eq!(c.control("LandingGearToggle").unwrap().context, "Ship");
        assert!(c.activities.iter().any(|a| a.id == "exploration-fss-dss"));
        for a in &c.activities {
            for x in &a.actions {
                assert!(c.control(&x.id).is_some(), "{} in {} not in the catalogue", x.id, a.title);
            }
        }
        for v in VOICE_ACTIONS {
            assert!(c.control(v).is_some(), "{v}");
        }
    }

    #[test]
    fn humanizes_ids() {
        assert_eq!(humanize("ToggleFooBar_Buggy"), "Toggle foo bar (SRV)");
        assert_eq!(humanize("UI_Select"), "UI select");
    }
}
