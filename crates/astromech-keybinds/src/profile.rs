//! Keybind profiles for the app (M1, docs/V1-DESIGN.md §3, KEYBINDS.md §7): the presets in the
//! Bindings folder (and the game's built-in ones, read-only), which is active per context, and a
//! full view of one preset as list rows with inline health issues.

use crate::binds;
use crate::catalog::{self, Catalog, VOICE_ACTIONS};
use crate::devices::Inventory;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

pub const CONTEXTS: [&str; 4] = ["General", "Ship", "SRV", "On foot"];
/// Elite reads only Joy_1..Joy_32 per device (KEYBINDS.md §2.6, rule D-J33).
pub const MAX_JOY: u32 = 32;
/// Keys the route paste sends in the galaxy map (`Runner::paste`: Ctrl+V, then Enter).
const PASTE_KEYS: &[&str] = &["Key_V", "Key_Enter"];
pub const VKB_VID: &str = "231D";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProfileFile {
    pub path: PathBuf,
    pub file_name: String,
    pub version: Option<(u32, u32)>,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileSummary {
    pub name: String,
    /// "yours" (Bindings folder) or "built-in" (game install, read-only).
    pub source: String,
    pub editable: bool,
    /// The file the game uses: the highest version suffix.
    pub path: PathBuf,
    pub files: Vec<ProfileFile>,
    /// Contexts (General, Ship, SRV, On foot) this preset is active for.
    pub active_for: Vec<String>,
    /// Device IDs the preset references.
    pub devices: Vec<String>,
    /// One-line problems from the health check (shown on the Presets page).
    pub problems: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartPreset {
    pub path: Option<PathBuf>,
    /// Per context: preset name (4 entries; a one-line file applies to all).
    pub names: Vec<String>,
}

/// Read the newest `StartPreset*.start` in `dir`.
pub fn start_preset(dir: &Path) -> StartPreset {
    let mut best: Option<(u32, PathBuf, Vec<String>)> = None;
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let low = name.to_ascii_lowercase();
            if !(low.starts_with("startpreset") && low.ends_with(".start")) {
                continue;
            }
            let Ok(b) = std::fs::read(e.path()) else { continue };
            let sp = binds::parse_start_preset(&name, &binds::decode(&b).0);
            let n = sp.number.unwrap_or(0);
            if best.as_ref().is_none_or(|b| n >= b.0) {
                best = Some((n, e.path(), sp.lines.iter().map(|l| l.1.clone()).collect()));
            }
        }
    }
    match best {
        Some((_, p, lines)) => {
            let names = if lines.len() == 1 { vec![lines[0].clone(); 4] } else { lines };
            StartPreset { path: Some(p), names }
        }
        None => StartPreset { path: None, names: vec![] },
    }
}

fn modified(p: &Path) -> String {
    std::fs::metadata(p)
        .and_then(|m| m.modified())
        .map(|t| chrono::DateTime::<chrono::Local>::from(t).format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn scan(dir: &Path, source: &str, out: &mut BTreeMap<String, ProfileSummary>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        let file_name = e.file_name().to_string_lossy().to_string();
        let Some(fi) = binds::parse_file_name(&file_name) else { continue };
        if !p.is_file() {
            continue;
        }
        let Ok(b) = std::fs::read(&p) else { continue };
        let content = binds::parse_binds(&binds::decode(&b).0);
        let name = content.preset_name.clone().unwrap_or_else(|| fi.stem.clone());
        let key = format!("{source}\u{1}{name}");
        let s = out.entry(key).or_insert_with(|| ProfileSummary {
            name: name.clone(),
            source: source.into(),
            editable: source == "yours",
            path: p.clone(),
            files: vec![],
            active_for: vec![],
            devices: vec![],
            problems: vec![],
        });
        let f = ProfileFile { path: p.clone(), file_name, version: fi.suffix, modified: modified(&p) };
        let newest = s.files.iter().all(|o| o.version <= f.version);
        if newest {
            s.path = p.clone();
            s.devices = content.devices.keys().filter(|d| !is_generic(d)).cloned().collect();
        }
        s.files.push(f);
        if let Some(err) = content.xml_error {
            s.problems.push(format!(
                "{}: XML error at line {}: {}",
                s.files.last().unwrap().file_name,
                err.line,
                err.message
            ));
        }
    }
}

pub fn is_generic(device: &str) -> bool {
    matches!(device, "Keyboard" | "Mouse" | "GamePad" | "{NoDevice}" | "")
}

/// All presets: yours (Bindings folder) first, then the game's built-ins (if the install is known).
pub fn list(bindings_dir: &Path, builtin_dirs: &[PathBuf], inv: Option<&Inventory>) -> Vec<ProfileSummary> {
    let mut m = BTreeMap::new();
    scan(bindings_dir, "yours", &mut m);
    for d in builtin_dirs {
        scan(d, "built-in", &mut m);
    }
    let sp = start_preset(bindings_dir);
    let mut v: Vec<ProfileSummary> = m.into_values().collect();
    // A name in StartPreset picks your file before a built-in of the same name.
    for (i, ctx) in CONTEXTS.iter().enumerate() {
        let Some(n) = sp.names.get(i) else { continue };
        if let Some(p) = v.iter_mut().find(|p| &p.name == n && p.source == "yours") {
            p.active_for.push(ctx.to_string());
        } else if let Some(p) = v.iter_mut().find(|p| &p.name == n) {
            p.active_for.push(ctx.to_string());
        }
    }
    for p in &mut v {
        p.files.sort_by_key(|f| std::cmp::Reverse(f.version));
        if p.files.len() > 1 && p.source == "yours" {
            p.problems.push(format!(
                "{} files use this name; the game uses {}",
                p.files.len(),
                p.path.file_name().unwrap_or_default().to_string_lossy()
            ));
        }
        if let Some(inv) = inv {
            if let Some(w) = pid_mismatch(&p.devices, inv) {
                p.problems.push(w);
            }
            let missing: Vec<&String> = p.devices.iter().filter(|d| !connected(inv, d)).collect();
            if !missing.is_empty() && !inv.controllers.is_empty() {
                p.problems.push(format!(
                    "Device not connected: {}. The game hides a preset when a device it uses is missing.",
                    missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
                ));
            }
        }
    }
    v.sort_by_key(|p| (p.source != "yours", p.active_for.is_empty(), p.name.to_lowercase()));
    v
}

pub fn connected(inv: &Inventory, device: &str) -> bool {
    is_generic(device) || inv.controllers.iter().any(|c| c.elite_id.eq_ignore_ascii_case(device))
}

/// A preset uses a VKB device ID that isn't connected while a different VKB ID is: the stick's
/// PID changed (other firmware or a VKBDevCfg profile). Warning only: we don't edit IDs.
pub fn pid_mismatch(devices: &[String], inv: &Inventory) -> Option<String> {
    let used: Vec<&String> =
        devices.iter().filter(|d| d.len() == 8 && d.to_ascii_uppercase().starts_with(VKB_VID)).collect();
    let present: Vec<&str> = inv
        .controllers
        .iter()
        .map(|c| c.elite_id.as_str())
        .filter(|id| id.to_ascii_uppercase().starts_with(VKB_VID))
        .collect();
    let missing: Vec<&&String> = used.iter().filter(|d| !present.iter().any(|p| p.eq_ignore_ascii_case(d))).collect();
    let extra: Vec<&&str> = present.iter().filter(|p| !used.iter().any(|d| d.eq_ignore_ascii_case(p))).collect();
    if missing.is_empty() || extra.is_empty() {
        return None;
    }
    Some(format!(
        "This preset expects VKB device {}, but the connected VKB device is {}. The stick's PID changed (different firmware or a VKBDevCfg profile). Reflash the firmware or set the PID back in VKBDevCfg; astromech doesn't change device IDs.",
        missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "),
        extra.iter().map(|s| **s).collect::<Vec<_>>().join(", ")
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Modifier {
    pub device: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Slot {
    /// "Primary", "Secondary" or "Binding" (axes)
    pub slot: String,
    /// Empty when unbound.
    pub device: String,
    pub key: String,
    pub modifiers: Vec<Modifier>,
}

impl Slot {
    pub fn bound(&self) -> bool {
        !self.device.is_empty() && self.device != "{NoDevice}" && !self.key.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    /// "no_keyboard", "joy_over_32", "device_missing", "conflict"
    pub code: String,
    /// "error", "warning" or "info"
    pub level: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Row {
    pub id: String,
    pub name: String,
    pub group: String,
    pub context: String,
    pub analogue: bool,
    pub slots: Vec<Slot>,
    pub issues: Vec<Issue>,
    /// False when the action isn't in the file (the game then uses no binding).
    pub in_file: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Group {
    pub name: String,
    pub context: String,
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone, Serialize)]
pub struct View {
    pub name: String,
    pub path: PathBuf,
    pub editable: bool,
    pub groups: Vec<Group>,
    pub devices: Vec<String>,
    pub problems: Vec<String>,
    pub issue_count: usize,
}

/// Parse every action element (one with Primary/Secondary/Binding children).
pub fn parse_actions(text: &str) -> Result<BTreeMap<String, Vec<Slot>>, String> {
    let doc = roxmltree::Document::parse(text.trim_start_matches('\u{feff}')).map_err(|e| e.to_string())?;
    let mut out = BTreeMap::new();
    for a in doc.root_element().children().filter(|n| n.is_element()) {
        let slots: Vec<Slot> = a
            .children()
            .filter(|n| n.is_element() && matches!(n.tag_name().name(), "Primary" | "Secondary" | "Binding"))
            .map(|s| {
                let dev = s.attribute("Device").unwrap_or("");
                Slot {
                    slot: s.tag_name().name().into(),
                    device: if dev == "{NoDevice}" { String::new() } else { dev.into() },
                    key: s.attribute("Key").unwrap_or("").into(),
                    modifiers: s
                        .children()
                        .filter(|m| m.has_tag_name("Modifier"))
                        .filter_map(|m| {
                            Some(Modifier { device: m.attribute("Device")?.into(), key: m.attribute("Key")?.into() })
                        })
                        .collect(),
                }
            })
            .collect();
        if !slots.is_empty() {
            out.insert(a.tag_name().name().to_string(), slots);
        }
    }
    Ok(out)
}

fn joy_number(key: &str) -> Option<u32> {
    key.strip_prefix("Joy_")?.parse().ok()
}

/// Which controls are live at the same time. The game has separate modes (FSS, galaxy map,
/// cameras, menus) that reuse flight buttons on purpose, so a "conflict" is only reported inside
/// one mode.
fn mode(r: &Row) -> String {
    match r.group.as_str() {
        "Flight" | "Ship systems" | "Weapons and combat" => "ship".into(),
        g if g.starts_with("SRV") => "srv".into(),
        g if g.starts_with("On foot") => "foot".into(),
        g => g.into(),
    }
}

fn ctx_order(c: &str) -> usize {
    CONTEXTS.iter().position(|x| *x == c).unwrap_or(9)
}

/// The full list view of one preset file, with health issues on each row.
pub fn view(path: &Path, editable: bool, inv: Option<&Inventory>) -> Result<View, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let text = binds::decode(&bytes).0;
    let content = binds::parse_binds(&text);
    let actions = parse_actions(&text)?;
    let cat = Catalog::get();
    let mut rows: Vec<(u32, u32, Row)> = vec![];
    for c in &cat.controls {
        let (slots, in_file) = match actions.get(&c.id) {
            Some(s) => (s.clone(), true),
            None => (empty_slots(c.analogue), false),
        };
        rows.push((
            c.group_order,
            c.order,
            Row {
                id: c.id.clone(),
                name: c.name.clone(),
                group: c.group.clone(),
                context: c.context.clone(),
                analogue: c.analogue,
                slots,
                issues: vec![],
                in_file,
            },
        ));
    }
    for (id, slots) in &actions {
        if cat.control(id).is_none() {
            rows.push((
                900,
                0,
                Row {
                    id: id.clone(),
                    name: catalog::humanize(id),
                    group: "Other".into(),
                    context: "General".into(),
                    analogue: slots.iter().any(|s| s.slot == "Binding"),
                    slots: slots.clone(),
                    issues: vec![],
                    in_file: true,
                },
            ));
        }
    }
    // Conflicts: one control (with the same modifiers) on two actions of the same context.
    let mut uses: HashMap<(String, String, String, Vec<Modifier>), Vec<String>> = HashMap::new();
    for (_, _, r) in &rows {
        for s in r.slots.iter().filter(|s| s.bound()) {
            let mut m = s.modifiers.clone();
            m.sort();
            uses.entry((mode(r), s.device.clone(), s.key.clone(), m)).or_default().push(r.name.clone());
        }
    }
    let mut count = 0;
    for (_, _, r) in &mut rows {
        let bound: Vec<&Slot> = r.slots.iter().filter(|s| s.bound()).collect();
        if VOICE_ACTIONS.contains(&r.id.as_str())
            && !bound.iter().any(|s| s.device == "Keyboard" && s.modifiers.iter().all(|m| m.device == "Keyboard"))
        {
            r.issues.push(Issue {
                code: "no_keyboard".into(),
                level: "error".into(),
                text: "No keyboard key: voice commands can't press this. Put a key in the empty slot.".into(),
            });
        }
        if r.group == "Galaxy map" || r.id == "GalaxyMapOpen" {
            if let Some(s) = bound.iter().find(|s| s.device == "Keyboard" && PASTE_KEYS.contains(&s.key.as_str())) {
                r.issues.push(Issue {
                    code: "paste_key".into(),
                    level: "warning".into(),
                    text: format!(
                        "{} is also what the route paste presses (Ctrl+V, then Enter) in the galaxy map. If the search box is not focused, this action runs instead.",
                        s.key.strip_prefix("Key_").unwrap_or(&s.key)
                    ),
                });
            }
        }
        for s in &bound {
            if let Some(n) = joy_number(&s.key).filter(|n| *n > MAX_JOY) {
                r.issues.push(Issue {
                    code: "joy_over_32".into(),
                    level: "error".into(),
                    text: format!("Joy {n}: ignored. Elite reads only buttons 1-32 on each device."),
                });
            }
            if let Some(inv) = inv {
                if !inv.controllers.is_empty() && !connected(inv, &s.device) {
                    r.issues.push(Issue {
                        code: "device_missing".into(),
                        level: "warning".into(),
                        text: format!("{} isn't connected.", s.device),
                    });
                }
            }
            let mut m = s.modifiers.clone();
            m.sort();
            if let Some(others) = uses.get(&(mode(r), s.device.clone(), s.key.clone(), m)) {
                let others: Vec<&String> = others.iter().filter(|o| **o != r.name).collect();
                if !others.is_empty() {
                    r.issues.push(Issue {
                        code: "conflict".into(),
                        level: "info".into(),
                        text: format!(
                            "Same control as {}.",
                            others.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
                        ),
                    });
                }
            }
        }
        count += r.issues.iter().filter(|i| i.level != "info").count();
    }
    rows.sort_by_key(|r| (ctx_order(&r.2.context), r.0, r.1));
    let mut groups: Vec<Group> = vec![];
    for (_, _, r) in rows {
        match groups.last_mut() {
            Some(g) if g.name == r.group => g.rows.push(r),
            _ => groups.push(Group { name: r.group.clone(), context: r.context.clone(), rows: vec![r] }),
        }
    }
    let devices: Vec<String> = content.devices.keys().filter(|d| !is_generic(d)).cloned().collect();
    let mut problems = vec![];
    if let Some(inv) = inv {
        problems.extend(pid_mismatch(&devices, inv));
    }
    Ok(View {
        name: content.preset_name.unwrap_or_default(),
        path: path.into(),
        editable,
        groups,
        devices,
        problems,
        issue_count: count,
    })
}

pub fn empty_slots(analogue: bool) -> Vec<Slot> {
    let e = |n: &str| Slot { slot: n.into(), device: String::new(), key: String::new(), modifiers: vec![] };
    if analogue {
        vec![e("Binding")]
    } else {
        vec![e("Primary"), e("Secondary")]
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::devices::Controller;

    pub const X: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="DualVKB" MajorVersion="4" MinorVersion="2">
	<KeyboardLayout>en-US</KeyboardLayout>
	<LandingGearToggle>
		<Primary Device="231D3201" Key="Joy_4" />
		<Secondary Device="Keyboard" Key="Key_L" />
	</LandingGearToggle>
	<NightVisionToggle>
		<Primary Device="231D3201" Key="Joy_33" />
		<Secondary Device="{NoDevice}" Key="" />
	</NightVisionToggle>
	<ShipSpotLightToggle>
		<Primary Device="231D3201" Key="Joy_4" />
		<Secondary Device="Keyboard" Key="Key_L">
			<Modifier Device="Keyboard" Key="Key_LeftShift" />
		</Secondary>
	</ShipSpotLightToggle>
	<ThrottleAxis>
		<Binding Device="231D3201" Key="Joy_YAxis" />
		<Inverted Value="0" />
		<Deadzone Value="0.00000000" />
	</ThrottleAxis>
	<SomethingNew>
		<Primary Device="Keyboard" Key="Key_F9" />
		<Secondary Device="{NoDevice}" Key="" />
	</SomethingNew>
</Root>
"#;

    pub fn inv(ids: &[&str]) -> Inventory {
        Inventory {
            source: "test".into(),
            controllers: ids
                .iter()
                .map(|id| Controller {
                    vid: id[..4].into(),
                    pid: id[4..].into(),
                    elite_id: id.to_string(),
                    product: "VKB".into(),
                    manufacturer: "VKB".into(),
                    usage: 4,
                    kind: "joystick".into(),
                    buttons: 128,
                    axes: vec![],
                    hats: 1,
                    other_values: vec![],
                })
                .collect(),
            ..Default::default()
        }
    }

    fn row<'a>(v: &'a View, id: &str) -> &'a Row {
        v.groups.iter().flat_map(|g| &g.rows).find(|r| r.id == id).unwrap()
    }

    #[test]
    fn view_rows_and_issues() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("DualVKB.4.2.binds");
        std::fs::write(&p, X).unwrap();
        let v = view(&p, true, Some(&inv(&["231D3201", "231D0200"]))).unwrap();
        assert_eq!(v.name, "DualVKB");
        let gear = row(&v, "LandingGearToggle");
        assert_eq!(gear.name, "Landing gear");
        assert_eq!(gear.slots[1].key, "Key_L");
        assert!(gear.issues.iter().any(|i| i.code == "conflict" && i.text.contains("Ship lights")));
        let nv = row(&v, "NightVisionToggle");
        let codes: Vec<&str> = nv.issues.iter().map(|i| i.code.as_str()).collect();
        assert!(codes.contains(&"joy_over_32") && codes.contains(&"no_keyboard"), "{codes:?}");
        // Shift+L is a different control from L: no conflict between their keyboard slots.
        assert!(row(&v, "ShipSpotLightToggle").issues.iter().all(|i| !i.text.contains("Key_L")));
        assert!(row(&v, "ThrottleAxis").analogue);
        assert!(!row(&v, "SetSpeed50").in_file);
        assert_eq!(row(&v, "SomethingNew").group, "Other");
        assert!(v.groups.first().unwrap().context == "Ship" || v.groups.first().unwrap().context == "General");
        assert!(v.issue_count >= 2);
    }

    #[test]
    fn paste_keys_in_the_galaxy_map_are_flagged() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("Keys.4.2.binds");
        std::fs::write(
            &p,
            r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="Keys" MajorVersion="4" MinorVersion="2">
	<CamZoomIn>
		<Primary Device="Keyboard" Key="Key_V">
			<Modifier Device="Keyboard" Key="Key_LeftControl" />
		</Primary>
		<Secondary Device="{NoDevice}" Key="" />
	</CamZoomIn>
	<GalaxyMapHome>
		<Primary Device="231D3201" Key="Joy_3" />
		<Secondary Device="Keyboard" Key="Key_Enter" />
	</GalaxyMapHome>
	<CamZoomOut>
		<Primary Device="Keyboard" Key="Key_B" />
		<Secondary Device="{NoDevice}" Key="" />
	</CamZoomOut>
	<UI_Select>
		<Primary Device="Keyboard" Key="Key_Enter" />
		<Secondary Device="{NoDevice}" Key="" />
	</UI_Select>
	<CamYawLeft>
		<Primary Device="231D3201" Key="Joy_8" />
		<Secondary Device="{NoDevice}" Key="" />
	</CamYawLeft>
</Root>"#,
        )
        .unwrap();
        let v = view(&p, true, None).unwrap();
        let flagged = |id: &str| row(&v, id).issues.iter().any(|i| i.code == "paste_key");
        assert!(flagged("CamZoomIn"), "Ctrl+V on a galaxy map action");
        assert!(flagged("GalaxyMapHome"), "Enter in the secondary slot");
        assert!(!flagged("CamZoomOut"), "another key");
        assert!(!flagged("UI_Select"), "Enter on an action outside the galaxy map");
        assert!(!flagged("CamYawLeft"), "a joystick button");
        let text = &row(&v, "CamZoomIn").issues.iter().find(|i| i.code == "paste_key").unwrap().text;
        assert!(text.starts_with("V is also"), "{text}");
        assert_eq!(row(&v, "GalaxyMapHome").issues.iter().find(|i| i.code == "paste_key").unwrap().level, "warning");
    }

    #[test]
    fn device_and_pid_warnings() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("DualVKB.4.2.binds"), X).unwrap();
        std::fs::write(d.path().join("DualVKB.4.1.binds"), X).unwrap();
        std::fs::write(d.path().join("StartPreset.4.start"), "DualVKB\nDualVKB\nOther\nDualVKB\n").unwrap();
        let ps = list(d.path(), &[], Some(&inv(&["231D0201"])));
        assert_eq!(ps.len(), 1);
        let p = &ps[0];
        assert!(p.path.ends_with("DualVKB.4.2.binds"));
        assert_eq!(p.active_for, ["General", "Ship", "On foot"]);
        assert!(p
            .problems
            .iter()
            .any(|x| x.contains("PID changed") && x.contains("231D3201") && x.contains("231D0201")));
        assert!(p.problems.iter().any(|x| x.contains("2 files")));
        assert!(pid_mismatch(&["231D3201".into()], &inv(&["231D3201"])).is_none());
        let v = view(&p.path, true, Some(&inv(&["231D0201"]))).unwrap();
        assert!(row(&v, "LandingGearToggle").issues.iter().any(|i| i.code == "device_missing"));
    }
}
