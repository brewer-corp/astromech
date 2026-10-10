//! Locating the Bindings folder, game installs, built-in presets and logs. Read-only.

use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct GameInstall {
    pub source: String,
    /// The product folder that contains ControlSchemes (e.g. ...\Products\elite-dangerous-odyssey-64).
    pub product_dir: PathBuf,
}

pub fn default_bindings_dir() -> Option<PathBuf> {
    let la = std::env::var_os("LOCALAPPDATA")?;
    Some(PathBuf::from(la).join("Frontier Developments").join("Elite Dangerous").join("Options").join("Bindings"))
}

pub fn default_logs_dir() -> Option<PathBuf> {
    let la = std::env::var_os("LOCALAPPDATA")?;
    Some(PathBuf::from(la).join("Frontier Developments").join("Elite Dangerous").join("Logs"))
}

/// Extract library paths from Steam's libraryfolders.vdf (`"path"  "D:\\SteamLibrary"`).
pub fn parse_library_folders(vdf: &str) -> Vec<PathBuf> {
    let mut out = vec![];
    for line in vdf.lines() {
        let t = line.trim();
        if !t.starts_with("\"path\"") {
            continue;
        }
        let rest = t["\"path\"".len()..].trim();
        if let Some(s) = rest.strip_prefix('"').and_then(|r| r.rfind('"').map(|e| &r[..e])) {
            out.push(PathBuf::from(s.replace("\\\\", "\\")));
        }
    }
    out
}

/// Epic manifest (*.item, JSON): InstallLocation when DisplayName mentions Elite.
pub fn parse_epic_manifest(json: &str) -> Option<PathBuf> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let name = v.get("DisplayName")?.as_str()?;
    if !name.to_ascii_lowercase().contains("elite") {
        return None;
    }
    Some(PathBuf::from(v.get("InstallLocation")?.as_str()?))
}

#[cfg(windows)]
fn steam_path_from_registry() -> Option<PathBuf> {
    use windows_sys::Win32::System::Registry::*;
    let w = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let mut buf = [0u16; 1024];
    let mut len = (buf.len() * 2) as u32;
    let r = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            w("Software\\Valve\\Steam").as_ptr(),
            w("SteamPath").as_ptr(),
            RRF_RT_REG_SZ,
            core::ptr::null_mut(),
            buf.as_mut_ptr() as *mut _,
            &mut len,
        )
    };
    if r != 0 {
        return None;
    }
    let n = buf.iter().position(|&c| c == 0).unwrap_or(0);
    Some(PathBuf::from(String::from_utf16_lossy(&buf[..n]).replace('/', "\\")))
}
#[cfg(not(windows))]
fn steam_path_from_registry() -> Option<PathBuf> {
    None
}

/// Find product folders (containing ControlSchemes) under a root, up to 3 levels deep.
pub fn find_products(root: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    fn walk(p: &Path, depth: u32, out: &mut Vec<PathBuf>) {
        if p.join("ControlSchemes").is_dir() {
            out.push(p.to_path_buf());
            return;
        }
        if depth == 0 {
            return;
        }
        if let Ok(rd) = std::fs::read_dir(p) {
            let mut kids: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
            kids.sort();
            for k in kids {
                walk(&k, depth - 1, out);
            }
        }
    }
    walk(root, 3, &mut out);
    // Prefer Odyssey/Live product first.
    out.sort_by_key(|p| {
        let n = p.file_name().map(|s| s.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
        (!n.contains("odyssey"), n)
    });
    out
}

pub fn locate_installs(override_dir: Option<&Path>, notes: &mut Vec<String>) -> Vec<GameInstall> {
    let mut roots: Vec<(String, PathBuf)> = vec![];
    if let Some(d) = override_dir {
        roots.push(("--game-dir".into(), d.to_path_buf()));
    } else {
        let mut steam_roots = vec![];
        if let Some(p) = steam_path_from_registry() {
            steam_roots.push(p);
        }
        steam_roots.push(PathBuf::from(r"C:\Program Files (x86)\Steam"));
        steam_roots.push(PathBuf::from(r"C:\Program Files\Steam"));
        let mut libs = vec![];
        for s in &steam_roots {
            libs.push(s.clone());
            if let Ok(t) = std::fs::read_to_string(s.join("steamapps").join("libraryfolders.vdf")) {
                libs.extend(parse_library_folders(&t));
            }
        }
        for l in libs {
            roots.push(("Steam".into(), l.join("steamapps").join("common").join("Elite Dangerous")));
        }
        let pd = std::env::var_os("PROGRAMDATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"));
        if let Ok(rd) = std::fs::read_dir(pd.join("Epic").join("EpicGamesLauncher").join("Data").join("Manifests")) {
            for e in rd.flatten() {
                if e.path().extension().map(|x| x == "item").unwrap_or(false) {
                    if let Some(p) = std::fs::read_to_string(e.path()).ok().and_then(|t| parse_epic_manifest(&t)) {
                        roots.push(("Epic".into(), p));
                    }
                }
            }
        }
        for p in [r"C:\Program Files (x86)\Frontier", r"C:\Program Files\Frontier"] {
            roots.push(("Frontier launcher".into(), PathBuf::from(p)));
        }
        if let Some(la) = std::env::var_os("LOCALAPPDATA") {
            roots.push(("Frontier launcher".into(), PathBuf::from(la).join("Frontier_Developments")));
        }
    }
    let mut out: Vec<GameInstall> = vec![];
    for (src, r) in roots {
        if !r.is_dir() {
            continue;
        }
        for p in find_products(&r) {
            if !out.iter().any(|g| g.product_dir == p) {
                out.push(GameInstall { source: src.clone(), product_dir: p });
            }
        }
    }
    if out.is_empty() {
        notes.push("No game install with a ControlSchemes folder was found; DeviceMappings.xml and built-in preset names are unavailable. Use --game-dir.".into());
    }
    out
}

#[derive(Debug, Clone, Serialize)]
pub struct LogExcerpt {
    pub file: String,
    pub modified: String,
    pub lines: Vec<String>,
    pub truncated: bool,
}

const LOG_WORDS: [&str; 9] = ["bind", "preset", "controller", "joystick", "device", "guid", "directinput", "hotas", "input"];

/// Grep the newest few *.log files in a folder for binding/device related lines.
pub fn scan_logs(dir: &Path, max_files: usize) -> Vec<LogExcerpt> {
    let Ok(rd) = std::fs::read_dir(dir) else { return vec![] };
    let mut files: Vec<(std::time::SystemTime, PathBuf)> = rd
        .flatten()
        .filter(|e| e.path().extension().map(|x| x.eq_ignore_ascii_case("log")).unwrap_or(false))
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    files.sort();
    files.reverse();
    let mut out = vec![];
    for (mt, p) in files.into_iter().take(max_files) {
        let Ok(bytes) = std::fs::read(&p) else { continue };
        let text = String::from_utf8_lossy(&bytes);
        let mut lines = vec![];
        let mut truncated = false;
        for l in text.lines() {
            let low = l.to_ascii_lowercase();
            if LOG_WORDS.iter().any(|w| low.contains(w)) {
                if lines.len() >= 60 {
                    truncated = true;
                    break;
                }
                lines.push(l.chars().take(400).collect());
            }
        }
        if !lines.is_empty() {
            out.push(LogExcerpt { file: p.display().to_string(), modified: fmt_time(mt), lines, truncated });
        }
    }
    out
}

pub fn fmt_time(t: std::time::SystemTime) -> String {
    let dt: chrono::DateTime<chrono::Local> = t.into();
    dt.format("%Y-%m-%d %H:%M:%S %z").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vdf() {
        let v = r#""libraryfolders"
{
	"0"
	{
		"path"		"C:\\Program Files (x86)\\Steam"
		"label"		""
	}
	"1"
	{
		"path"		"D:\\SteamLibrary"
	}
}"#;
        assert_eq!(parse_library_folders(v), vec![PathBuf::from(r"C:\Program Files (x86)\Steam"), PathBuf::from(r"D:\SteamLibrary")]);
    }
    #[test]
    fn epic() {
        assert_eq!(
            parse_epic_manifest(r#"{"DisplayName":"Elite Dangerous","InstallLocation":"D:\\Epic\\EliteDangerous"}"#),
            Some(PathBuf::from(r"D:\Epic\EliteDangerous"))
        );
        assert_eq!(parse_epic_manifest(r#"{"DisplayName":"Fortnite","InstallLocation":"x"}"#), None);
    }
    #[test]
    fn products() {
        let d = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(d.path().join("Products/elite-dangerous-64/ControlSchemes")).unwrap();
        std::fs::create_dir_all(d.path().join("Products/elite-dangerous-odyssey-64/ControlSchemes")).unwrap();
        let p = find_products(d.path());
        assert_eq!(p.len(), 2);
        assert!(p[0].ends_with("elite-dangerous-odyssey-64"));
    }
}
