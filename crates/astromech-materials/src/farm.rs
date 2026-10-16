//! Where to farm raw and encoded materials (manufactured: see `hge`). Community-reported
//! sources (Frontier forums, Steam, Reddit, 2023–2025); the named bodies are starting points,
//! not guarantees.
//!
//! - Raw: Guardian brain trees and crystalline shards drop grade 4 only; the lower grades of a
//!   category are cheapest as a trade down from its grade 4 (1 → 3 per grade).
//! - Encoded: Jameson Crash Site data points (scan, relog, scan) for Modified Consumer and
//!   Cracked Industrial Firmware and Adaptive Encryptors Capture, which trades into any other
//!   encoded category (the rest of the firmware too); wake scanning for the Wake Scans column.

use crate::catalog::{self, Kind, Spec};
use crate::hge::SourceNote;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Encoded materials the Jameson Crash Site data points give directly.
pub const JAMESON: &[&str] = &["adaptiveencryptors", "industrialfirmware", "consumerfirmware"];

/// Encoded material most other encoded shortfalls are traded from.
pub const ENCODED_ANCHOR: &str = "adaptiveencryptors";

/// A well-known grade 4 raw farm (`data/catalog/raw-farms.json`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Farm {
    pub system: String,
    pub id64: i64,
    /// The system's position in ly.
    pub pos: [f64; 3],
    /// Body name without the system prefix, e.g. "C 1 a".
    pub body: String,
    /// "Crystalline Shards" or "Brain Trees".
    pub site: String,
    /// Material id it is farmed for.
    pub material: String,
    /// Light seconds from the arrival star.
    pub arrival_ls: f64,
}

impl Farm {
    /// Full body name, e.g. "HIP 36601 C 1 a".
    pub fn body_name(&self) -> String {
        format!("{} {}", self.system, self.body)
    }
}

#[derive(Deserialize)]
struct FarmsFile {
    checked: String,
    farms: Vec<Farm>,
}

fn farms_file() -> &'static FarmsFile {
    static F: OnceLock<FarmsFile> = OnceLock::new();
    F.get_or_init(|| {
        serde_json::from_str(include_str!("../../../data/catalog/raw-farms.json")).expect("raw-farms.json")
    })
}

/// The well-known grade 4 raw farms.
pub fn farms() -> &'static [Farm] {
    &farms_file().farms
}

/// When every farm was last checked to still have its site (RFC 3339).
pub fn farms_checked() -> &'static str {
    &farms_file().checked
}

const JAMESON_HINT: &str = "HIP 12099 1 B. Scan every data point from the SRV, exit to the main menu and log back in, scan again. Nearest encoded trader: Ray Gateway, Diaguandri.";

/// Grade 4 raw material of the same category: what a lower-grade raw shortfall is traded from.
pub fn raw_anchor(spec: &Spec) -> Option<&'static Spec> {
    if spec.kind != Kind::Raw || !spec.on_trader {
        return None;
    }
    catalog::catalog().iter().find(|s| s.kind == Kind::Raw && s.on_trader && s.group == spec.group && s.grade == 4)
}

/// What a shortfall of `spec` is best farmed as: itself, or another material to trade from.
pub fn anchor(spec: &'static Spec) -> &'static Spec {
    match spec.kind {
        Kind::Raw => raw_anchor(spec).unwrap_or(spec),
        Kind::Encoded if spec.on_trader && !JAMESON.contains(&spec.id) && spec.group != "Wake Scans" => {
            catalog::lookup(ENCODED_ANCHOR).unwrap_or(spec)
        }
        _ => spec,
    }
}

pub fn sources_for(spec: &Spec) -> Vec<SourceNote> {
    let note = |kind: &str, summary: String, hint: String| SourceNote {
        kind: kind.into(),
        summary,
        hint,
        query: (kind == "wake_scan").then(|| "famine".to_string()),
    };
    let mut out = vec![];
    match spec.kind {
        Kind::Raw if spec.on_trader && spec.grade == 4 => {
            let sites: Vec<String> = farms()
                .iter()
                .filter(|f| f.material == spec.id)
                .map(|f| format!("{} ({})", f.body_name(), f.site.to_lowercase()))
                .collect();
            let site = (!sites.is_empty()).then(|| format!("Reported: {}. ", sites.join("; ")));
            out.push(note(
                "brain_tree",
                "Guardian brain trees and crystalline shards drop grade 4 raw.".into(),
                format!(
                    "{}Shoot them (turrets or flak), then collect with the SRV; limpets no longer work on brain trees.",
                    site.unwrap_or_default()
                ),
            ));
        }
        Kind::Raw if spec.on_trader => {
            if let Some(a) = raw_anchor(spec) {
                out.push(note(
                    "trade_down",
                    format!("Trade down from {} (1 → {} at a raw trader).", a.name, 3i64.pow((4 - spec.grade) as u32)),
                    format!(
                        "Fill {} at brain trees or crystalline shards. Also from surface prospecting and mining.",
                        a.name
                    ),
                ));
            }
        }
        Kind::Encoded if JAMESON.contains(&spec.id) => {
            out.push(note("jameson", "Jameson Crash Site data points.".into(), JAMESON_HINT.into()));
        }
        Kind::Encoded if spec.on_trader && spec.group == "Wake Scans" => {
            out.push(note(
                "wake_scan",
                "Wake scanning where traffic never stops.".into(),
                "A fast ship with a wake scanner: busy stations with a tourist beacon (Luyten's Star works well), or distribution centres in Famine systems. Wakes fade, so get to them early.".into(),
            ));
        }
        Kind::Encoded if spec.on_trader => {
            out.push(note(
                "trade_down",
                "Trade from Adaptive Encryptors Capture (Jameson Crash Site).".into(),
                format!("{JAMESON_HINT} Mission rewards also give encoded data."),
            ));
        }
        _ => {}
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchors_and_notes() {
        let s = |id| catalog::lookup(id).unwrap();
        assert_eq!(anchor(s("arsenic")).id, "polonium");
        assert_eq!(anchor(s("polonium")).id, "polonium");
        assert_eq!(anchor(s("shieldfrequencydata")).id, "adaptiveencryptors");
        assert_eq!(anchor(s("consumerfirmware")).id, "consumerfirmware");
        assert_eq!(anchor(s("dataminedwake")).id, "dataminedwake");
        assert_eq!(anchor(s("pharmaceuticalisolators")).id, "pharmaceuticalisolators");
        assert!(sources_for(s("polonium"))[0].hint.contains("Synuefe BY-F d12-39 A 2 (brain trees)"));
        assert!(sources_for(s("polonium"))[0].hint.contains("HIP 36601 C 1 a (crystalline shards)"));
        assert_eq!(sources_for(s("arsenic"))[0].kind, "trade_down");
        assert!(sources_for(s("arsenic"))[0].summary.contains("1 → 9"));
        assert_eq!(sources_for(s("dataminedwake"))[0].kind, "wake_scan");
        assert_eq!(sources_for(s("industrialfirmware"))[0].kind, "jameson");
        assert!(sources_for(s("chemicalmanipulators")).is_empty());
    }

    #[test]
    fn every_farm_names_a_grade_4_raw_material_and_a_position() {
        assert!(farms().len() >= 9);
        for f in farms() {
            let m = catalog::lookup(&f.material).unwrap_or_else(|| panic!("{} names {}", f.body_name(), f.material));
            assert_eq!((m.kind, m.grade), (Kind::Raw, 4), "{}", f.body_name());
            assert!(f.pos.iter().any(|c| *c != 0.0) && f.arrival_ls > 0.0, "{}", f.body_name());
            assert!(["Crystalline Shards", "Brain Trees"].contains(&f.site.as_str()));
        }
    }
}
