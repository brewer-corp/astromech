//! Status.json and journal helpers.

use std::path::{Path, PathBuf};

pub const FLAG_DOCKED: u64 = 1 << 0;
pub const FLAG_LANDED: u64 = 1 << 1;
pub const FLAG_GEAR: u64 = 1 << 2;
pub const FLAG_SHIELDS_UP: u64 = 1 << 3;
pub const FLAG_SUPERCRUISE: u64 = 1 << 4;
pub const FLAG_FLIGHT_ASSIST_OFF: u64 = 1 << 5;
pub const FLAG_LIGHTS: u64 = 1 << 8;
pub const FLAG_SILENT_RUNNING: u64 = 1 << 10;
pub const FLAG_SRV_HANDBRAKE: u64 = 1 << 12;
pub const FLAG_SRV_DRIVE_ASSIST: u64 = 1 << 15;
/// Hostile ships nearby or under attack (manual §14 bit 22, "IsInDanger").
pub const FLAG_IN_DANGER: u64 = 1 << 22;
pub const FLAG_BEING_INTERDICTED: u64 = 1 << 23;
pub const FLAG_IN_MAIN_SHIP: u64 = 1 << 24;
pub const FLAG_IN_SRV: u64 = 1 << 26;
pub const FLAG_HUD_ANALYSIS: u64 = 1 << 27;
pub const FLAG_NIGHT_VISION: u64 = 1 << 28;

// GuiFocus values, from Frontier's Player Journal Manual (v38, p. 97, "GuiFocus values"; the same
// table since v30): 1 is InternalPanel (right hand side), 2 is ExternalPanel (left hand side).
pub const GUI_NONE: u64 = 0;
/// The left panel (navigation, contacts): the manual's ExternalPanel.
pub const GUI_LEFT: u64 = 2;
/// The right panel (ship systems): the manual's InternalPanel.
pub const GUI_RIGHT: u64 = 1;
pub const GUI_COMMS: u64 = 3;
pub const GUI_ROLE: u64 = 4;
pub const GUI_GALAXY: u64 = 6;
pub const GUI_SYSTEM: u64 = 7;
/// The Full Spectrum System scanner.
pub const GUI_FSS: u64 = 9;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatusSnap {
    pub flags: u64,
    pub gui_focus: Option<u64>,
    pub fuel_main: Option<f64>,
    pub fuel_reservoir: Option<f64>,
    pub cargo: Option<f64>,
    pub legal_state: Option<String>,
    pub body_name: Option<String>,
    pub balance: Option<i64>,
    /// Position over or on a body (`Latitude`, `Longitude` in degrees; `Altitude` in metres,
    /// `Heading` in degrees). Present only near a planet's surface.
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub heading: Option<f64>,
    /// The body's radius in metres (`PlanetRadius`, Odyssey), present with the position.
    pub planet_radius: Option<f64>,
}

impl StatusSnap {
    /// Latitude and longitude, when Status.json gives both.
    pub fn lat_lon(&self) -> Option<(f64, f64)> {
        Some((self.latitude?, self.longitude?))
    }
}

/// Parse Status.json contents. Returns None for partial/invalid writes or when there are no Flags
/// (e.g. at the main menu).
pub fn parse_status(bytes: &[u8]) -> Option<StatusSnap> {
    let v: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let flags = v.get("Flags")?.as_u64()?;
    let gui_focus = v.get("GuiFocus").and_then(|g| g.as_u64());
    let fuel = v.get("Fuel");
    Some(StatusSnap {
        flags,
        gui_focus,
        fuel_main: fuel.and_then(|f| f.get("FuelMain")).and_then(|x| x.as_f64()),
        fuel_reservoir: fuel.and_then(|f| f.get("FuelReservoir")).and_then(|x| x.as_f64()),
        cargo: v.get("Cargo").and_then(|x| x.as_f64()),
        legal_state: v.get("LegalState").and_then(|x| x.as_str()).map(str::to_string),
        body_name: v.get("BodyName").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(str::to_string),
        balance: v.get("Balance").and_then(|x| x.as_i64()),
        latitude: v.get("Latitude").and_then(|x| x.as_f64()),
        longitude: v.get("Longitude").and_then(|x| x.as_f64()),
        altitude: v.get("Altitude").and_then(|x| x.as_f64()),
        heading: v.get("Heading").and_then(|x| x.as_f64()),
        planet_radius: v.get("PlanetRadius").and_then(|x| x.as_f64()),
    })
}

/// Read + parse; any error (sharing violation, partial write) -> None.
pub fn read_status(path: &Path) -> Option<StatusSnap> {
    let bytes = std::fs::read(path).ok()?;
    parse_status(&bytes)
}

/// Read with a few quick retries (used after a change notification).
pub fn read_status_retry(path: &Path, tries: u32) -> Option<StatusSnap> {
    for i in 0..tries {
        if let Some(s) = read_status(path) {
            return Some(s);
        }
        if i + 1 < tries {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameBuild {
    pub journal: PathBuf,
    pub gameversion: Option<String>,
    pub build: Option<String>,
}

/// Newest Journal.*.log (by modified time) and its Fileheader.
pub fn latest_fileheader(dir: &Path) -> Option<GameBuild> {
    let mut journals = vec![];
    for e in std::fs::read_dir(dir).ok()? {
        let e = e.ok()?;
        let n = e.file_name().to_string_lossy().to_string();
        if n.starts_with("Journal.") && n.ends_with(".log") {
            let m = e.metadata().and_then(|m| m.modified()).ok();
            journals.push((m, n, e.path()));
        }
    }
    journals.sort();
    let (_, _, path) = journals.pop()?;
    let text = std::fs::read(&path).ok()?;
    let text = String::from_utf8_lossy(&text);
    for line in text.lines().take(5) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
            if v.get("event").and_then(|e| e.as_str()) == Some("Fileheader") {
                return Some(GameBuild {
                    journal: path.clone(),
                    gameversion: v.get("gameversion").and_then(|x| x.as_str()).map(str::to_string),
                    build: v.get("build").and_then(|x| x.as_str()).map(|s| s.trim().to_string()),
                });
            }
        }
    }
    Some(GameBuild { journal: path, gameversion: None, build: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_on_a_body_is_read() {
        let s = parse_status(
            br#"{"timestamp":"2026-10-17T12:00:00Z","event":"Status","Flags":2097152,"Flags2":16,
                "Latitude":-12.345678,"Longitude":101.5,"Altitude":0,"Heading":271,
                "BodyName":"Synthetic 1 a","PlanetRadius":2964202.5}"#,
        )
        .unwrap();
        assert_eq!(s.lat_lon(), Some((-12.345678, 101.5)));
        assert_eq!((s.altitude, s.heading, s.planet_radius), (Some(0.0), Some(271.0), Some(2964202.5)));
        let s = parse_status(br#"{"Flags":16777216}"#).unwrap();
        assert_eq!((s.lat_lon(), s.planet_radius), (None, None));
    }
}
