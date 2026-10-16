//! What the sender has to know about the running game before it can build a message: the
//! header values (`Fileheader`/`LoadGame`), the `horizons`/`odyssey` flags (`LoadGame` only),
//! the commander name for `uploaderID`, and the last known system for augmentation
//! cross-checks (`Location`, `FSDJump`, `CarrierJump`).

use serde_json::Value;

/// The system from the last `Location`, `FSDJump` or `CarrierJump`.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemFix {
    pub name: String,
    pub address: u64,
    pub pos: [f64; 3],
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameContext {
    /// `uploaderID`: the commander name (`Commander` or `LoadGame`).
    pub commander: Option<String>,
    /// `None` until a `Fileheader` (or a `LoadGame` carrying it) has been seen.
    pub gameversion: Option<String>,
    pub gamebuild: Option<String>,
    /// From `LoadGame` only; `None` means the key was absent, so it must not be sent.
    pub horizons: Option<bool>,
    pub odyssey: Option<bool>,
    pub system: Option<SystemFix>,
    /// The game version came from `Fileheader`, which wins over `LoadGame`.
    version_from_fileheader: bool,
}

impl GameContext {
    /// Feed every journal event, in order.
    pub fn observe(&mut self, ev: &Value) {
        let s = |k: &str| ev.get(k).and_then(Value::as_str).map(str::to_owned);
        match ev.get("event").and_then(Value::as_str) {
            Some("Fileheader") => {
                // Sent as-is (whitespace included), and as "" when absent.
                self.gameversion = Some(s("gameversion").unwrap_or_default());
                self.gamebuild = Some(s("build").unwrap_or_default());
                self.version_from_fileheader = true;
            }
            Some("LoadGame") => {
                if let Some(c) = s("Commander") {
                    self.commander = Some(c);
                }
                self.horizons = ev.get("Horizons").and_then(Value::as_bool);
                self.odyssey = ev.get("Odyssey").and_then(Value::as_bool);
                if !self.version_from_fileheader {
                    if let Some(v) = s("gameversion") {
                        self.gameversion = Some(v);
                        self.gamebuild = Some(s("build").unwrap_or_default());
                    }
                }
            }
            Some("Commander") => {
                if let Some(c) = s("Name") {
                    self.commander = Some(c);
                }
            }
            Some("Location" | "FSDJump" | "CarrierJump") => self.system = system_fix(ev),
            _ => {}
        }
    }

    /// Beta or alpha clients must never reach the live schemas.
    pub fn is_beta(&self) -> bool {
        self.gameversion.as_deref().is_some_and(is_beta_version)
    }
}

/// True for game versions from a non-live client (e.g. `"4.0.0.400 BETA"`).
pub fn is_beta_version(v: &str) -> bool {
    let v = v.to_ascii_lowercase();
    v.contains("beta") || v.contains("alpha")
}

/// A complete fix, or `None`: a partial one can't be trusted for augmentation.
fn system_fix(ev: &Value) -> Option<SystemFix> {
    let pos = ev.get("StarPos")?.as_array()?;
    if pos.len() != 3 {
        return None;
    }
    Some(SystemFix {
        name: ev.get("StarSystem")?.as_str()?.to_owned(),
        address: ev.get("SystemAddress")?.as_u64()?,
        pos: [pos[0].as_f64()?, pos[1].as_f64()?, pos[2].as_f64()?],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn header_and_flags_come_from_fileheader_and_loadgame() {
        let mut c = GameContext::default();
        c.observe(&json!({"event":"Fileheader","gameversion":"4.0.0.1904","build":"r308767/r0 "}));
        c.observe(&json!({"event":"Commander","Name":"Jameson","FID":"F1"}));
        c.observe(&json!({"event":"LoadGame","Commander":"Jameson","Horizons":true,"gameversion":"9.9","build":"x"}));
        assert_eq!(c.gameversion.as_deref(), Some("4.0.0.1904"));
        assert_eq!(c.gamebuild.as_deref(), Some("r308767/r0 "), "whitespace is passed through");
        assert_eq!(c.commander.as_deref(), Some("Jameson"));
        assert_eq!((c.horizons, c.odyssey), (Some(true), None), "absent Odyssey stays unknown");
    }

    #[test]
    fn loadgame_supplies_version_without_a_fileheader() {
        let mut c = GameContext::default();
        c.observe(&json!({"event":"LoadGame","Commander":"J","Horizons":true,"Odyssey":false,"gameversion":"4.0.0.1","build":"b"}));
        assert_eq!(c.gameversion.as_deref(), Some("4.0.0.1"));
        assert_eq!(c.odyssey, Some(false));
    }

    #[test]
    fn beta_versions_are_detected() {
        assert!(is_beta_version("4.0.0.400 BETA"));
        assert!(is_beta_version("3.0.0.100 Alpha"));
        assert!(!is_beta_version("4.0.0.1904"));
    }

    #[test]
    fn partial_location_clears_the_fix() {
        let mut c = GameContext::default();
        c.observe(
            &json!({"event":"FSDJump","StarSystem":"Sol","SystemAddress":10477373803u64,"StarPos":[0.0,0.0,0.0]}),
        );
        assert_eq!(c.system.as_ref().map(|s| s.name.as_str()), Some("Sol"));
        c.observe(&json!({"event":"Location","StarSystem":"Lave"}));
        assert_eq!(c.system, None);
    }
}
