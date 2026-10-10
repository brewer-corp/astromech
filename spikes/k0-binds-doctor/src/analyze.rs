//! Cross-reference binds files, StartPreset, DeviceMappings and connected devices.
//! Every "reason" is a hypothesis: Frontier does not document the preset-listing rules.

use crate::binds::{self, BindsContent, EncodingInfo, JoyKey, StartPresetFile, XmlError};
use crate::devices::{Controller, Inventory};
use crate::game::{self, GameInstall, LogExcerpt};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize)]
pub struct Reason {
    pub confidence: Confidence,
    pub code: String,
    pub title: String,
    pub explanation: String,
}

fn reason(c: Confidence, code: &str, title: impl Into<String>, expl: impl Into<String>) -> Reason {
    Reason { confidence: c, code: code.into(), title: title.into(), explanation: expl.into() }
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceRef {
    pub name: String,
    pub binds: u32,
    /// "raw VID+PID", "DeviceMappings name", "built-in", "unknown name"
    pub referenced_as: String,
    /// "connected", "missing", "assumed present", "unknown"
    pub status: String,
    pub matched: Option<String>,
    pub problems: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileReport {
    pub path: String,
    pub rel_path: String,
    pub file_name: String,
    pub in_subfolder: bool,
    pub size: u64,
    pub modified: String,
    pub encoding: EncodingInfo,
    pub xml_error: Option<XmlError>,
    pub root_element: Option<String>,
    pub preset_name: Option<String>,
    pub major_version: Option<String>,
    pub minor_version: Option<String>,
    pub file_stem: String,
    pub file_suffix: Option<String>,
    pub keyboard_layout: Option<String>,
    pub element_count: u32,
    pub devices: Vec<DeviceRef>,
    pub keyboard_only: bool,
    pub active_in: Vec<String>,
    pub flags: Vec<String>,
    pub reasons: Vec<Reason>,
    pub verdict: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartLine {
    pub context: String,
    pub preset: String,
    pub resolves_to: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartReport {
    pub file_name: String,
    pub number: Option<u32>,
    pub probably_effective: bool,
    pub raw_line_count: usize,
    pub lines: Vec<StartLine>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstallReport {
    pub source: String,
    pub product_dir: String,
    pub device_mappings: Option<String>,
    pub device_mapping_count: usize,
    pub builtin_presets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NearMiss {
    pub path: String,
    pub why: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub tool: String,
    pub generated_at: String,
    pub disclaimer: String,
    pub bindings_dir: String,
    pub bindings_dir_found: bool,
    pub installs: Vec<InstallReport>,
    pub devices: Inventory,
    pub device_flags: Vec<String>,
    pub start_presets: Vec<StartReport>,
    pub current_version: Option<String>,
    pub current_version_basis: String,
    pub files: Vec<FileReport>,
    pub near_misses: Vec<NearMiss>,
    pub game_backup_files: usize,
    pub binding_loading_errors: Option<LogExcerpt>,
    pub client_logs: Vec<LogExcerpt>,
    pub notes: Vec<String>,
}

pub struct Inputs<'a> {
    pub bindings_dir: &'a Path,
    pub installs: Vec<GameInstall>,
    pub inventory: Inventory,
    pub extra_log_dirs: Vec<PathBuf>,
    pub notes: Vec<String>,
}

struct Scanned {
    path: PathBuf,
    rel: String,
    in_sub: bool,
    size: u64,
    modified: String,
    name: binds::FileNameInfo,
    file_name: String,
    enc: EncodingInfo,
    content: BindsContent,
}

fn walk(dir: &Path, base: &Path, depth: u32, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut v: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    v.sort();
    for p in v {
        if p.is_dir() {
            if depth > 0 {
                walk(&p, base, depth - 1, out);
            }
        } else {
            out.push(p);
        }
    }
}

pub fn run(inp: Inputs) -> Report {
    let mut notes = inp.notes;
    let bdir = inp.bindings_dir;
    let found = bdir.is_dir();
    if !found {
        notes.push("Bindings folder not found. Pass --bindings-dir if your game data lives elsewhere.".into());
    }
    // --- installs: DeviceMappings + built-in presets
    let mut mappings: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut builtins: Vec<String> = vec![];
    let mut installs = vec![];
    for g in &inp.installs {
        let cs = g.product_dir.join("ControlSchemes");
        let dm = cs.join("DeviceMappings.xml");
        let mut ir = InstallReport {
            source: g.source.clone(),
            product_dir: g.product_dir.display().to_string(),
            device_mappings: None,
            device_mapping_count: 0,
            builtin_presets: vec![],
        };
        if let Ok(bytes) = std::fs::read(&dm) {
            let (t, _) = binds::decode(&bytes);
            match binds::parse_device_mappings(&t) {
                Ok(m) => {
                    ir.device_mappings = Some(dm.display().to_string());
                    ir.device_mapping_count = m.len();
                    if mappings.is_empty() {
                        mappings = m;
                    }
                }
                Err(e) => notes.push(format!("DeviceMappings.xml at {} is not valid XML (line {}, col {}): {}", dm.display(), e.line, e.col, e.message)),
            }
        }
        if let Ok(rd) = std::fs::read_dir(&cs) {
            let mut ps: Vec<PathBuf> = rd.flatten().map(|e| e.path()).filter(|p| binds::parse_file_name(&p.file_name().unwrap().to_string_lossy()).is_some()).collect();
            ps.sort();
            for p in ps {
                if let Ok(bytes) = std::fs::read(&p) {
                    let (t, _) = binds::decode(&bytes);
                    if let Some(n) = binds::parse_binds(&t).preset_name {
                        ir.builtin_presets.push(n.clone());
                        if !builtins.contains(&n) {
                            builtins.push(n);
                        }
                    }
                }
            }
        }
        installs.push(ir);
    }
    let have_mappings = !mappings.is_empty();
    // id -> mapped name
    let mut id_to_name: BTreeMap<String, String> = BTreeMap::new();
    for (n, ids) in &mappings {
        for id in ids {
            id_to_name.entry(id.clone()).or_insert_with(|| n.clone());
        }
    }

    // --- device flags
    let inv = inp.inventory;
    let mut device_flags = vec![];
    let mut by_id: BTreeMap<&str, Vec<&Controller>> = BTreeMap::new();
    for c in &inv.controllers {
        by_id.entry(c.elite_id.as_str()).or_default().push(c);
    }
    for (id, v) in &by_id {
        if v.len() > 1 {
            device_flags.push(format!(
                "{} controllers report the same VID:PID {} ({}). Elite identifies devices by VID+PID, so it can't tell them apart: bindings for one will apply to both or the second won't work. Give each a distinct USB PID in the vendor tool (e.g. VKBDevCfg). (If they're one physical device exposing two HID collections, this is a false alarm.)",
                v.len(), id, v[0].product
            ));
        }
    }
    for c in &inv.controllers {
        if c.buttons > 32 {
            device_flags.push(format!(
                "{} ({}) reports {} buttons. Elite reads only the first 32 buttons per device (VKB forum moderator; not Frontier-documented). Buttons 33+ can't be bound unless the device is split into virtual controllers.",
                c.product, c.elite_id, c.buttons
            ));
        }
        if let Some(n) = id_to_name.get(&c.elite_id) {
            device_flags.push(format!("{} ({}) is listed in DeviceMappings.xml, so Elite writes it as \"{}\" rather than {}.", c.product, c.elite_id, n, c.elite_id));
        }
    }

    // --- scan files
    let mut all = vec![];
    if found {
        walk(bdir, bdir, 4, &mut all);
    }
    let mut scanned: Vec<Scanned> = vec![];
    let mut starts: Vec<StartPresetFile> = vec![];
    let mut near = vec![];
    let mut backups = 0;
    let mut ble: Option<LogExcerpt> = None;
    for p in &all {
        let fname = p.file_name().unwrap().to_string_lossy().to_string();
        let low = fname.to_ascii_lowercase();
        let rel = p.strip_prefix(bdir).unwrap_or(p).display().to_string();
        let in_sub = p.parent() != Some(bdir);
        let meta = std::fs::metadata(p).ok();
        if low.ends_with(".backup") {
            backups += 1;
            continue;
        }
        if low.starts_with("startpreset") && low.ends_with(".start") && !in_sub {
            if let Ok(bytes) = std::fs::read(p) {
                let (t, _) = binds::decode(&bytes);
                starts.push(binds::parse_start_preset(&fname, &t));
            }
            continue;
        }
        if low == "bindingloadingerrors.log" && !in_sub {
            if let Ok(bytes) = std::fs::read(p) {
                let t = String::from_utf8_lossy(&bytes).to_string();
                let lines: Vec<String> = t.lines().map(|l| l.to_string()).collect();
                let truncated = lines.len() > 300;
                ble = Some(LogExcerpt {
                    file: p.display().to_string(),
                    modified: meta.as_ref().and_then(|m| m.modified().ok()).map(game::fmt_time).unwrap_or_default(),
                    lines: lines.into_iter().take(300).collect(),
                    truncated,
                });
            }
            continue;
        }
        if let Some(name) = binds::parse_file_name(&fname) {
            let Ok(bytes) = std::fs::read(p) else {
                notes.push(format!("Could not read {}", p.display()));
                continue;
            };
            let (t, enc) = binds::decode(&bytes);
            let content = binds::parse_binds(&t);
            scanned.push(Scanned {
                path: p.clone(),
                rel,
                in_sub,
                size: meta.as_ref().map(|m| m.len()).unwrap_or(0),
                modified: meta.as_ref().and_then(|m| m.modified().ok()).map(game::fmt_time).unwrap_or_default(),
                name,
                file_name: fname,
                enc,
                content,
            });
            continue;
        }
        // Near misses: renamed downloads etc.
        if low.contains(".binds") {
            near.push(NearMiss {
                path: p.display().to_string(),
                why: "Name contains \".binds\" but doesn't end with it (e.g. a browser added \".txt\" or \" (1)\"). The game only looks at files ending in .binds. Rename it to <Name>.<major>.<minor>.binds.".into(),
            });
        } else if low.ends_with(".xml") || low.ends_with(".txt") {
            if let Ok(bytes) = std::fs::read(p) {
                let head = String::from_utf8_lossy(&bytes[..bytes.len().min(4096)]).to_string();
                if head.contains("<Root PresetName") {
                    near.push(NearMiss {
                        path: p.display().to_string(),
                        why: "Looks like a bindings file (has <Root PresetName=...>) but has the wrong extension. The game only looks at .binds files.".into(),
                    });
                }
            }
        }
    }

    // --- current version
    let max_start = starts.iter().filter_map(|s| s.number).max();
    let top: Vec<&Scanned> = scanned.iter().filter(|s| !s.in_sub).collect();
    let (current, basis) = {
        let suffixes = |maj: Option<u32>| top.iter().filter_map(|s| s.name.suffix).filter(|(a, _)| maj.map(|m| *a == m).unwrap_or(true)).max();
        match max_start {
            Some(n) => match suffixes(Some(n)) {
                Some(v) => (Some(v), format!("StartPreset.{n}.start exists, and the newest .{n}.x suffix among your files is .{}.{}", v.0, v.1)),
                None => (Some((n, 0)), format!("StartPreset.{n}.start exists but no file has a .{n}.x suffix; assuming major version {n}")),
            },
            None => match suffixes(None) {
                Some(v) => (Some(v), format!("No numbered StartPreset file; using the highest suffix among your files (.{}.{})", v.0, v.1)),
                None => (None, "Couldn't determine the game's current binds version".to_string()),
            },
        }
    };

    // --- duplicates (same name + same suffix among top-level parsed files)
    let mut dup: BTreeMap<(String, Option<(u32, u32)>), Vec<String>> = BTreeMap::new();
    let mut dup_ci: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for s in &top {
        if let Some(n) = &s.content.preset_name {
            dup.entry((n.clone(), s.name.suffix)).or_default().push(s.file_name.clone());
            dup_ci.entry(n.to_lowercase()).or_default().push(s.file_name.clone());
        }
    }

    // --- StartPreset resolution
    let effective = starts.iter().map(|s| s.number.unwrap_or(0)).max();
    let mut active: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut start_reports = vec![];
    for sp in &starts {
        let eff = Some(sp.number.unwrap_or(0)) == effective;
        let mut lines = vec![];
        for (ctx, name) in &sp.lines {
            let cur: Vec<&&Scanned> = top.iter().filter(|s| s.content.preset_name.as_deref() == Some(name.as_str()) && current.map(|c| s.name.suffix == Some(c)).unwrap_or(true)).collect();
            let other: Vec<&&Scanned> = top.iter().filter(|s| s.content.preset_name.as_deref() == Some(name.as_str()) && !cur.iter().any(|c| c.file_name == s.file_name)).collect();
            let resolves = if !cur.is_empty() {
                if eff {
                    for c in &cur {
                        active.entry(c.file_name.clone()).or_default().push(ctx.clone());
                    }
                }
                let clash = if builtins.iter().any(|b| b == name) { " (also a built-in preset name: which one the game loads is unknown)" } else { "" };
                format!("your file {}{clash}", cur.iter().map(|c| c.file_name.as_str()).collect::<Vec<_>>().join(", "))
            } else if builtins.iter().any(|b| b == name) {
                "a built-in preset (ControlSchemes)".to_string()
            } else if !other.is_empty() {
                format!("only a file with a different version suffix: {}", other.iter().map(|c| c.file_name.as_str()).collect::<Vec<_>>().join(", "))
            } else if !have_mappings && builtins.is_empty() {
                "not one of your files (built-in presets unknown: game install not found)".to_string()
            } else {
                "NOTHING found (no file or built-in with this PresetName)".to_string()
            };
            lines.push(StartLine { context: ctx.clone(), preset: name.clone(), resolves_to: resolves });
        }
        start_reports.push(StartReport { file_name: sp.file_name.clone(), number: sp.number, probably_effective: eff, raw_line_count: sp.raw_line_count, lines });
    }
    start_reports.sort_by_key(|s| std::cmp::Reverse(s.number.unwrap_or(0)));

    // --- per-file reports
    let has_keyboard = !inv.keyboards.is_empty();
    let mut files = vec![];
    for s in &scanned {
        let c = &s.content;
        let mut reasons = vec![];
        let mut flags = vec![];
        let mut devs = vec![];
        let mut missing = vec![];
        let mut unknown = vec![];
        for (name, u) in &c.devices {
            let mut d = DeviceRef { name: name.clone(), binds: u.binds, referenced_as: String::new(), status: String::new(), matched: None, problems: vec![] };
            let mut matched: Option<&Controller> = None;
            if name == "Keyboard" || name == "Mouse" {
                d.referenced_as = "built-in".into();
                d.status = if (name == "Keyboard" && has_keyboard) || (name == "Mouse" && !inv.mice.is_empty()) { "connected".into() } else { "assumed present".into() };
            } else if let Some(id) = binds::parse_raw_id(name) {
                d.referenced_as = "raw VID+PID".into();
                matched = inv.controllers.iter().find(|c| c.elite_id == id);
                d.status = if matched.is_some() { "connected".into() } else { "missing".into() };
                if let Some(mn) = id_to_name.get(&id) {
                    d.problems.push(format!("DeviceMappings.xml names this id \"{mn}\"; the game probably writes and expects \"{mn}\" instead of {id} (hypothesis)."));
                }
            } else if let Some(ids) = mappings.get(name) {
                d.referenced_as = "DeviceMappings name".into();
                matched = inv.controllers.iter().find(|c| ids.contains(&c.elite_id));
                let xinput = name == "GamePad" && !inv.xinput_slots.is_empty();
                d.status = if matched.is_some() || xinput { "connected".into() } else { "missing".into() };
                if xinput && matched.is_none() {
                    d.matched = Some(format!("XInput pad in slot {}", inv.xinput_slots[0]));
                }
            } else {
                d.referenced_as = "unknown name".into();
                d.status = "unknown".into();
                if !have_mappings {
                    d.problems.push("Can't check: DeviceMappings.xml wasn't found (use --game-dir).".into());
                } else {
                    d.problems.push("Not in DeviceMappings.xml and not a VID+PID; the game may not recognise it.".into());
                }
            }
            if let Some(m) = matched {
                d.matched = Some(format!("{} ({}, {} buttons, axes {}, {} hats)", m.product, m.elite_id, m.buttons, m.axes.join("/"), m.hats));
            }
            // key checks
            for k in &u.keys {
                match binds::classify_key(k) {
                    JoyKey::Button(n) => {
                        if n > 32 {
                            d.problems.push(format!("{k}: above button 32; Elite reportedly ignores buttons 33+."));
                        }
                        if let Some(m) = matched {
                            if n > m.buttons {
                                d.problems.push(format!("{k}: the connected device reports only {} buttons.", m.buttons));
                            }
                        }
                    }
                    JoyKey::Pov(n) => {
                        if let Some(m) = matched {
                            if n > m.hats {
                                d.problems.push(format!("{k}: the connected device reports {} hat(s).", m.hats));
                            }
                        }
                    }
                    JoyKey::Axis(a) => {
                        if let Some(m) = matched {
                            if !m.axes.contains(&a) {
                                d.problems.push(format!("{k}: the connected device has no {a} axis (it reports {}).", m.axes.join("/")));
                            }
                        }
                    }
                    JoyKey::Other => {}
                }
            }
            if d.status == "missing" {
                missing.push(name.clone());
            }
            if d.status == "unknown" {
                unknown.push(name.clone());
            }
            devs.push(d);
        }
        let non_kbm = c.devices.keys().filter(|d| *d != "Keyboard" && *d != "Mouse").count();
        let keyboard_only = c.xml_error.is_none() && non_kbm == 0 && !c.devices.is_empty();
        if keyboard_only {
            flags.push("Keyboard/mouse-only preset (no controller bindings).".into());
        }
        let raw: Vec<&String> = c.devices.keys().filter(|d| binds::parse_raw_id(d).is_some()).collect();
        let named: Vec<&String> = c.devices.keys().filter(|d| mappings.contains_key(*d)).collect();
        if !raw.is_empty() {
            flags.push(format!("References devices by raw VID+PID: {}", raw.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
        }
        if !named.is_empty() {
            flags.push(format!("References devices by DeviceMappings name: {}", named.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
        }
        if !c.duplicate_actions.is_empty() {
            flags.push(format!("Repeated actions (game uses the first): {}", c.duplicate_actions.join(", ")));
        }

        if s.in_sub {
            reasons.push(reason(Confidence::High, "WRONG_FOLDER", "File is in a subfolder", format!("It's in \"{}\". The game is believed to read only the Bindings folder itself, so files in subfolders are ignored. Move it up one level (this tool won't move it for you).", s.rel)));
        }
        if let Some(e) = &c.xml_error {
            reasons.push(reason(Confidence::High, "MALFORMED_XML", format!("Not valid XML (line {}, column {})", e.line, e.col), format!("The file can't be parsed: {}. A broken file is very likely skipped by the game. Often caused by an incomplete download or a manual edit.", e.message)));
        } else if c.root_element.as_deref() != Some("Root") {
            reasons.push(reason(Confidence::High, "WRONG_ROOT", "Not a bindings file", format!("The top element is <{}>, not <Root>.", c.root_element.clone().unwrap_or_default())));
        } else if c.preset_name.as_deref().map(|n| n.trim().is_empty()).unwrap_or(true) {
            reasons.push(reason(Confidence::High, "NO_PRESET_NAME", "No PresetName", "<Root> has no PresetName attribute; the game lists presets by this name."));
        }
        if !missing.is_empty() {
            reasons.push(reason(Confidence::High, "MISSING_DEVICES", format!("Device(s) not connected: {}", missing.join(", ")), "Community reports (Frontier forums, Steam guide) agree: a preset is only offered when every device it references is present. Plug the device in before starting the game, or check its USB PID hasn't changed."));
        }
        if !unknown.is_empty() {
            reasons.push(reason(Confidence::Medium, "UNKNOWN_DEVICES", format!("Unrecognised device name(s): {}", unknown.join(", ")), "These names aren't raw VID+PID ids and aren't in the game's DeviceMappings.xml (or that file wasn't found). The game may treat them as missing devices."));
        }
        for i in &s.enc.issues {
            let title = i.split(" (").next().unwrap_or(i).trim_end_matches('.').to_string();
            reasons.push(reason(Confidence::Medium, "ENCODING", title, i.clone()));
        }
        if s.enc.issues.is_empty() && s.enc.bom.as_deref() == Some("UTF-8 BOM") {
            reasons.push(reason(Confidence::Low, "UTF8_BOM", "UTF-8 byte-order mark", "The file starts with a UTF-8 BOM. Frontier's own files don't have one; it's probably harmless."));
        }
        match (s.name.suffix, current) {
            (None, _) if s.name.odd_suffix.is_some() => reasons.push(reason(Confidence::Medium, "ODD_SUFFIX", "Unusual version suffix", format!("File name ends in .{}.binds; user presets normally end in .<major>.<minor>.binds (e.g. .4.2.binds).", s.name.odd_suffix.clone().unwrap()))),
            (None, cur) => reasons.push(reason(Confidence::Medium, "NO_VERSION_SUFFIX", "No version in the file name", format!("User presets are normally named <Name>.<major>.<minor>.binds{}. A plain .binds file may be ignored.", cur.map(|c| format!(" (yours: .{}.{}.binds)", c.0, c.1)).unwrap_or_default()))),
            (Some(v), Some(cur)) if v.0 < cur.0 => reasons.push(reason(Confidence::High, "OLD_VERSION", format!("Old version suffix .{}.{} (game uses .{}.{})", v.0, v.1, cur.0, cur.1), if v.0 <= 3 { "This is a Horizons/3.x-era file. The current game reads its own version's files, so it won't list this one. Copying it to the new suffix may work but older files lack newer actions (on-foot etc.)." } else { "Older major version than the game currently uses; probably not listed." })),
            (Some(v), Some(cur)) if v.0 == cur.0 && v.1 < cur.1 => reasons.push(reason(Confidence::Medium, "OLDER_MINOR", format!("Older minor version .{}.{} (newest here .{}.{})", v.0, v.1, cur.0, cur.1), "The game may only list files with the newest suffix. Other files in your folder use a newer one.")),
            (Some(v), Some(cur)) if v > cur => reasons.push(reason(Confidence::Low, "NEWER_VERSION", format!("Newer suffix .{}.{} than the rest of your files", v.0, v.1), "Possibly made with a newer game build; unusual but probably fine.")),
            _ => {}
        }
        if c.xml_error.is_none() {
            if let (Some(a), Some(b)) = (&c.major_version, &c.minor_version) {
                if let Some((x, y)) = s.name.suffix {
                    if a.parse::<u32>().ok() != Some(x) || b.parse::<u32>().ok() != Some(y) {
                        reasons.push(reason(Confidence::Medium, "VERSION_ATTR_MISMATCH", format!("Inside says {a}.{b}, file name says {x}.{y}"), "MajorVersion/MinorVersion in <Root> should match the file-name suffix (Frontier forum advice)."));
                    }
                }
            } else if c.root_element.as_deref() == Some("Root") {
                reasons.push(reason(Confidence::Low, "NO_VERSION_ATTRS", "No MajorVersion/MinorVersion", "<Root> lacks the version attributes the game writes."));
            }
        }
        if !s.in_sub {
            if let Some(n) = &c.preset_name {
                let same = &dup[&(n.clone(), s.name.suffix)];
                if same.len() > 1 {
                    let others: Vec<&str> = same.iter().filter(|f| **f != s.file_name).map(|s| s.as_str()).collect();
                    reasons.push(reason(Confidence::Medium, "DUPLICATE_PRESET_NAME", format!("PresetName \"{n}\" also used by {}", others.join(", ")), "Two files with the same PresetName and version: the game shows one entry, so one file hides the other. Which wins isn't documented. Give each a unique PresetName."));
                } else if dup_ci[&n.to_lowercase()].len() > 1 && dup_ci[&n.to_lowercase()].iter().any(|f| f != &s.file_name && top.iter().any(|t| &t.file_name == f && t.content.preset_name.as_deref() != Some(n.as_str()) && t.name.suffix == s.name.suffix)) {
                    reasons.push(reason(Confidence::Low, "CASE_DUPLICATE", "PresetName differs from another only by letter case", "May collide if the game compares names case-insensitively (unknown)."));
                }
                if builtins.iter().any(|b| b == n) && n != "Custom" {
                    reasons.push(reason(Confidence::Medium, "BUILTIN_NAME", format!("PresetName \"{n}\" is a built-in preset name"), "It may be hidden behind (or replace) the built-in preset of the same name. Use a unique name."));
                }
                if n.chars().any(|ch| ch == ' ' || !ch.is_ascii()) {
                    reasons.push(reason(Confidence::Low, "ODD_NAME", "PresetName has spaces or non-ASCII characters", "Community guides advise plain ASCII without spaces; unconfirmed whether it matters."));
                }
                if !n.eq_ignore_ascii_case(&s.name.stem) {
                    reasons.push(reason(Confidence::Low, "NAME_MISMATCH", format!("File name \"{}\" differs from PresetName \"{n}\"", s.name.stem), "The menu shows the PresetName, so this is probably harmless, but guides recommend keeping them equal."));
                }
            }
        }
        reasons.sort_by(|a, b| a.confidence.cmp(&b.confidence));
        let act = active.get(&s.file_name).cloned().unwrap_or_default();
        let verdict = if reasons.iter().any(|r| r.confidence != Confidence::Low) {
            format!("Likely hidden: {}", reasons[0].title)
        } else if !act.is_empty() {
            format!("Looks OK, and it's active ({})", act.join(", "))
        } else {
            "No obvious problem found: it should be listed if its devices are connected when the game starts".into()
        };
        files.push(FileReport {
            path: s.path.display().to_string(),
            rel_path: s.rel.clone(),
            file_name: s.file_name.clone(),
            in_subfolder: s.in_sub,
            size: s.size,
            modified: s.modified.clone(),
            encoding: s.enc.clone(),
            xml_error: c.xml_error.clone(),
            root_element: c.root_element.clone(),
            preset_name: c.preset_name.clone(),
            major_version: c.major_version.clone(),
            minor_version: c.minor_version.clone(),
            file_stem: s.name.stem.clone(),
            file_suffix: s.name.suffix.map(|(a, b)| format!("{a}.{b}")).or(s.name.odd_suffix.clone()),
            keyboard_layout: c.keyboard_layout.clone(),
            element_count: c.element_count,
            devices: devs,
            keyboard_only,
            active_in: act,
            flags,
            reasons,
            verdict,
        });
    }
    // Problem files first, then by name.
    files.sort_by_key(|f| (f.reasons.first().map(|r| r.confidence.clone()).unwrap_or(Confidence::Low), f.reasons.is_empty(), f.rel_path.to_lowercase()));

    // --- logs
    let mut client_logs = vec![];
    let mut log_dirs: Vec<PathBuf> = inp.installs.iter().map(|g| g.product_dir.join("Logs")).collect();
    log_dirs.extend(inp.extra_log_dirs);
    for d in log_dirs {
        client_logs.extend(game::scan_logs(&d, 3));
    }
    if starts.is_empty() && found {
        notes.push("No StartPreset*.start file found: the game may be using defaults, or has never saved a preset choice.".into());
    }
    Report {
        tool: format!("k0-binds-doctor {}", env!("CARGO_PKG_VERSION")),
        generated_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string(),
        disclaimer: "Read-only. Frontier doesn't document how the game decides which presets to list, so every reason here is a hypothesis drawn from community reports; this run tests them against your files. Confidence: high = widely reported cause, medium = plausible, low = probably harmless.".into(),
        bindings_dir: bdir.display().to_string(),
        bindings_dir_found: found,
        installs,
        devices: inv,
        device_flags,
        start_presets: start_reports,
        current_version: current.map(|(a, b)| format!("{a}.{b}")),
        current_version_basis: basis,
        files,
        near_misses: near,
        game_backup_files: backups,
        binding_loading_errors: ble,
        client_logs,
        notes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::{elite_id, Controller};

    fn ctl(id: &str, buttons: u32, axes: &[&str]) -> Controller {
        let vid = u16::from_str_radix(&id[..4], 16).unwrap();
        let pid = u16::from_str_radix(&id[4..], 16).unwrap();
        Controller { vid: id[..4].into(), pid: id[4..].into(), elite_id: elite_id(vid, pid), product: "P".into(), manufacturer: String::new(), usage: 4, kind: "Joystick".into(), buttons, axes: axes.iter().map(|s| s.to_string()).collect(), hats: 1, other_values: vec![] }
    }
    fn b(name: &str, dev: &str, key: &str, ver: &str) -> String {
        let (a, c) = ver.split_once('.').unwrap();
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<Root PresetName=\"{name}\" MajorVersion=\"{a}\" MinorVersion=\"{c}\">\n<X><Primary Device=\"{dev}\" Key=\"{key}\" /></X>\n</Root>")
    }
    fn codes(r: &Report, f: &str) -> Vec<String> {
        r.files.iter().find(|x| x.rel_path.replace('\\', "/") == f).unwrap_or_else(|| panic!("{f}")).reasons.iter().map(|x| x.code.clone()).collect()
    }

    #[test]
    fn end_to_end() {
        let d = tempfile::tempdir().unwrap();
        let bd = d.path().join("Bindings");
        let gd = d.path().join("Products/elite-dangerous-odyssey-64");
        std::fs::create_dir_all(bd.join("sub")).unwrap();
        std::fs::create_dir_all(gd.join("ControlSchemes")).unwrap();
        std::fs::write(gd.join("ControlSchemes/DeviceMappings.xml"), "<Root><T16000M><VID>044F</VID><PID>B10A</PID></T16000M><GamePad><VID>045E</VID><PID>028E</PID></GamePad></Root>").unwrap();
        std::fs::write(gd.join("ControlSchemes/KeyboardMouseOnly.binds"), b("KeyboardMouseOnly", "Keyboard", "Key_A", "4.0")).unwrap();
        let w = |n: &str, s: String| std::fs::write(bd.join(n), s).unwrap();
        w("Good.4.2.binds", b("Good", "231D0200", "Joy_40", "4.2"));
        w("Axis.4.2.binds", b("Axis", "231D0200", "Joy_UAxis", "4.2"));
        w("DupA.4.2.binds", b("Same", "Keyboard", "Key_A", "4.2"));
        w("DupB.4.2.binds", b("Same", "Keyboard", "Key_B", "4.2"));
        w("Same.4.0.binds", b("Same", "Keyboard", "Key_B", "4.0"));
        w("Old.3.0.binds", b("Old", "Keyboard", "Key_A", "3.0"));
        w("Miss.4.2.binds", b("Miss", "T16000M", "Joy_1", "4.2"));
        w("Pad.4.2.binds", b("Pad", "GamePad", "Pad_A", "4.2"));
        w("Raw.4.2.binds", b("Raw", "044FB10A", "Joy_1", "4.2"));
        w("Bad.4.2.binds", "<Root PresetName=\"Bad\">\n<X>\n</Root>".into());
        w("Builtin.4.2.binds", b("KeyboardMouseOnly", "Keyboard", "Key_A", "4.2"));
        w("Mism.4.2.binds", b("Mism", "Keyboard", "Key_A", "4.0"));
        w("sub/Deep.4.2.binds", b("Deep", "Keyboard", "Key_A", "4.2"));
        w("x.4.2.binds (1)", b("x", "Keyboard", "Key_A", "4.2"));
        w("StartPreset.4.start", "Good\nGhost\nKeyboardMouseOnly\nOld\n".into());
        let inv = Inventory { source: "test".into(), controllers: vec![ctl("231D0200", 128, &["X", "Y"]), ctl("231D0200", 31, &["X"])], xinput_slots: vec![0], ..Default::default() };
        let r = run(Inputs { bindings_dir: &bd, installs: vec![GameInstall { source: "t".into(), product_dir: gd.clone() }], inventory: inv, extra_log_dirs: vec![], notes: vec![] });
        assert_eq!(r.current_version.as_deref(), Some("4.2"));
        assert!(codes(&r, "Good.4.2.binds").is_empty());
        let good = r.files.iter().find(|f| f.file_name == "Good.4.2.binds").unwrap();
        assert!(good.devices[0].problems.iter().any(|p| p.contains("above button 32")));
        assert_eq!(good.active_in, vec!["General"]);
        let ax = r.files.iter().find(|f| f.file_name == "Axis.4.2.binds").unwrap();
        assert!(ax.devices[0].problems.iter().any(|p| p.contains("no U axis")));
        assert!(codes(&r, "DupA.4.2.binds").contains(&"DUPLICATE_PRESET_NAME".to_string()));
        assert!(!codes(&r, "Same.4.0.binds").contains(&"DUPLICATE_PRESET_NAME".to_string()));
        assert_eq!(codes(&r, "Old.3.0.binds"), vec!["OLD_VERSION"]);
        assert_eq!(codes(&r, "Miss.4.2.binds"), vec!["MISSING_DEVICES"]);
        assert!(codes(&r, "Pad.4.2.binds").is_empty(), "XInput slot satisfies GamePad");
        let raw = r.files.iter().find(|f| f.file_name == "Raw.4.2.binds").unwrap();
        assert!(raw.devices[0].problems[0].contains("\"T16000M\""));
        assert_eq!(codes(&r, "Bad.4.2.binds")[0], "MALFORMED_XML");
        assert!(codes(&r, "Builtin.4.2.binds").contains(&"BUILTIN_NAME".to_string()));
        assert!(codes(&r, "Mism.4.2.binds").contains(&"VERSION_ATTR_MISMATCH".to_string()));
        assert_eq!(codes(&r, "sub/Deep.4.2.binds")[0], "WRONG_FOLDER");
        assert_eq!(r.near_misses.len(), 1);
        assert_eq!(r.device_flags.len(), 2, "{:?}", r.device_flags); // duplicate id + >32 buttons
        let sp = &r.start_presets[0];
        assert!(sp.lines[1].resolves_to.starts_with("NOTHING"));
        assert!(sp.lines[2].resolves_to.contains("also a built-in"));
        assert!(sp.lines[3].resolves_to.starts_with("only a file with a different version"));
        // Files are only read: the folder content is unchanged.
        assert_eq!(std::fs::read_dir(&bd).unwrap().count(), 15);
    }
}
