//! Safe writes to keybind files (M1, docs/V1-DESIGN.md §1.2 "assign in a copy", KEYBINDS.md §7):
//! - every write first copies the file into `<Bindings>/astromech-backups/` (the game ignores
//!   subfolders), then writes a temp file and renames it over the original;
//! - edits change only the one slot element's text; every other byte of the file is kept;
//! - built-in presets are never written: [`copy_preset`] makes a named copy in the Bindings
//!   folder first.

use crate::binds;
use crate::profile::{Modifier, Slot};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

pub const BACKUP_DIR: &str = "astromech-backups";

#[derive(Debug, Clone, Serialize)]
pub struct Backup {
    pub path: PathBuf,
    pub file_name: String,
    /// The original file name it restores to.
    pub original: String,
    pub created: String,
}

fn stamp() -> String {
    chrono::Local::now().format("%Y%m%d-%H%M%S-%3f").to_string()
}

/// Copy `file` into `<its folder>/astromech-backups/<name>.<stamp>.bak`.
pub fn backup(file: &Path) -> Result<PathBuf> {
    let dir = file.parent().context("file has no folder")?.join(BACKUP_DIR);
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let name = file.file_name().context("no file name")?.to_string_lossy().to_string();
    let mut to = dir.join(format!("{name}.{}.bak", stamp()));
    let mut n = 1;
    while to.exists() {
        n += 1;
        to = dir.join(format!("{name}.{}{n}.bak", stamp()));
    }
    std::fs::copy(file, &to).with_context(|| format!("backing up {}", file.display()))?;
    Ok(to)
}

/// Write via a temp file in the same folder, then rename over the target.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension("astromech-tmp");
    std::fs::write(&tmp, bytes).with_context(|| format!("writing {}", tmp.display()))?;
    std::fs::rename(&tmp, path).with_context(|| format!("replacing {}", path.display()))?;
    Ok(())
}

pub fn list_backups(bindings_dir: &Path) -> Vec<Backup> {
    let dir = bindings_dir.join(BACKUP_DIR);
    let Ok(rd) = std::fs::read_dir(&dir) else { return vec![] };
    let mut v: Vec<Backup> = rd
        .flatten()
        .filter_map(|e| {
            let file_name = e.file_name().to_string_lossy().to_string();
            let stem = file_name.strip_suffix(".bak")?.to_string();
            let (original, st) = stem.rsplit_once('.')?;
            let (original, st) = (original.to_string(), st.to_string());
            let created = e
                .metadata()
                .and_then(|m| m.modified())
                .map(|t| chrono::DateTime::<chrono::Local>::from(t).format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or(st);
            Some(Backup { path: e.path(), file_name, original, created })
        })
        .collect();
    v.sort_by(|a, b| b.created.cmp(&a.created).then(b.file_name.cmp(&a.file_name)));
    v
}

/// Put a backup back (the current file is backed up first).
pub fn restore(bindings_dir: &Path, backup_file: &Path) -> Result<PathBuf> {
    let b = list_backups(bindings_dir)
        .into_iter()
        .find(|b| b.path == backup_file)
        .context("not one of astromech's backups")?;
    let target = bindings_dir.join(&b.original);
    if target.exists() {
        backup(&target)?;
    }
    let bytes = std::fs::read(&b.path)?;
    write_atomic(&target, &bytes)?;
    Ok(target)
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;")
}

fn slot_xml(slot: &str, b: Option<&Slot>, indent: &str) -> String {
    match b.filter(|s| s.bound()) {
        None => format!("<{slot} Device=\"{{NoDevice}}\" Key=\"\" />"),
        Some(s) if s.modifiers.is_empty() => {
            format!("<{slot} Device=\"{}\" Key=\"{}\" />", esc(&s.device), esc(&s.key))
        }
        Some(s) => {
            let mut x = format!("<{slot} Device=\"{}\" Key=\"{}\">", esc(&s.device), esc(&s.key));
            for Modifier { device, key } in &s.modifiers {
                x += &format!("\n{indent}\t<Modifier Device=\"{}\" Key=\"{}\" />", esc(device), esc(key));
            }
            x + &format!("\n{indent}</{slot}>")
        }
    }
}

fn line_indent(text: &str, pos: usize) -> String {
    let start = text[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
    text[start..pos].chars().take_while(|c| c.is_whitespace()).collect()
}

/// The new file text with one slot of `action` replaced (`binding: None` clears it). Adds the
/// action (both slots) before `</Root>` if the file doesn't have it.
pub fn set_slot_text(text: &str, action: &str, slot: &str, binding: Option<&Slot>, analogue: bool) -> Result<String> {
    if !matches!(slot, "Primary" | "Secondary" | "Binding") {
        bail!("unknown slot {slot}");
    }
    if !action.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        bail!("bad action name {action:?}");
    }
    let body = text.trim_start_matches('\u{feff}');
    let off = text.len() - body.len();
    let doc = roxmltree::Document::parse(body).context("the file isn't valid XML; not editing it")?;
    let root = doc.root_element();
    let el = root.children().find(|n| n.is_element() && n.tag_name().name() == action);
    let mut out = String::with_capacity(text.len() + 200);
    match el {
        Some(el) => {
            let Some(s) = el.children().find(|n| n.is_element() && n.tag_name().name() == slot) else {
                // Slot element missing: insert it as the first child.
                let r = el.range();
                let open_end = body[r.start..].find('>').context("bad element")? + r.start + 1;
                let ind = line_indent(body, r.start);
                out.push_str(&text[..off + open_end]);
                out.push_str(&format!("\n{ind}\t{}", slot_xml(slot, binding, &format!("{ind}\t"))));
                out.push_str(&text[off + open_end..]);
                return Ok(out);
            };
            let r = s.range();
            let ind = line_indent(body, r.start);
            out.push_str(&text[..off + r.start]);
            out.push_str(&slot_xml(slot, binding, &ind));
            out.push_str(&text[off + r.end..]);
        }
        None => {
            let r = root.range();
            let close = body[..r.end].rfind("</").context("no closing root tag")?;
            let nl = if body.contains("\r\n") { "\r\n" } else { "\n" };
            let mut block = format!("\t<{action}>{nl}");
            let slots: &[&str] = if analogue { &["Binding"] } else { &["Primary", "Secondary"] };
            for s in slots {
                let b = if *s == slot { binding } else { None };
                block += &format!("\t\t{}{nl}", slot_xml(s, b, "\t\t").replace('\n', nl));
            }
            block += &format!("\t</{action}>{nl}");
            out.push_str(&text[..off + close]);
            out.push_str(&block);
            out.push_str(&text[off + close..]);
        }
    }
    Ok(out)
}

#[derive(Debug, Clone, Serialize)]
pub struct EditResult {
    pub path: PathBuf,
    pub backup: PathBuf,
}

/// Set one slot in a preset file in the Bindings folder (backed up first).
pub fn set_slot(
    bindings_dir: &Path,
    file: &Path,
    action: &str,
    slot: &str,
    binding: Option<&Slot>,
    analogue: bool,
) -> Result<EditResult> {
    let canon_dir = bindings_dir.canonicalize().context("Bindings folder")?;
    let canon = file.canonicalize().context("preset file")?;
    if canon.parent() != Some(canon_dir.as_path()) {
        bail!("only presets in your Bindings folder can be edited (make a copy of a built-in preset first)");
    }
    let bytes = std::fs::read(file)?;
    let (text, enc) = binds::decode(&bytes);
    if enc.bom.as_deref().is_some_and(|b| b.starts_with("UTF-16")) {
        bail!("this file is UTF-16; astromech only edits UTF-8 presets (the game writes UTF-8)");
    }
    let new = set_slot_text(&text, action, slot, binding, analogue)?;
    roxmltree::Document::parse(new.trim_start_matches('\u{feff}')).context("edit produced invalid XML; not written")?;
    let backup = backup(file)?;
    let mut out = Vec::new();
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) && !new.starts_with('\u{feff}') {
        out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    }
    out.extend_from_slice(new.as_bytes());
    write_atomic(file, &out)?;
    Ok(EditResult { path: file.into(), backup })
}

/// Valid preset names: what the game shows in its list. Letters, digits, spaces, `-` and `_`.
pub fn valid_name(name: &str) -> bool {
    let n = name.trim();
    !n.is_empty() && n.len() <= 40 && n.chars().all(|c| c.is_ascii_alphanumeric() || " -_".contains(c))
}

/// Copy a preset (yours or built-in) into the Bindings folder as `<name>.<major>.<minor>.binds`
/// with `PresetName="<name>"`.
pub fn copy_preset(bindings_dir: &Path, src: &Path, name: &str) -> Result<PathBuf> {
    let name = name.trim();
    if !valid_name(name) {
        bail!("use letters, digits, spaces, - and _ (up to 40 characters)");
    }
    let (text, _) = binds::decode(&std::fs::read(src)?);
    let body = text.trim_start_matches('\u{feff}');
    let doc = roxmltree::Document::parse(body).context("the preset isn't valid XML")?;
    let root = doc.root_element();
    let major = root.attribute("MajorVersion").unwrap_or("4");
    let minor = root.attribute("MinorVersion").unwrap_or("0");
    let target = bindings_dir.join(format!("{name}.{major}.{minor}.binds"));
    if target.exists() {
        bail!("{} already exists", target.file_name().unwrap_or_default().to_string_lossy());
    }
    let others = crate::profile::list(bindings_dir, &[], None);
    if others.iter().any(|p| p.name.eq_ignore_ascii_case(name)) {
        bail!("a preset called {name:?} already exists");
    }
    // Replace the PresetName attribute value in the root's start tag only.
    let r = root.range();
    let tag_end = body[r.start..].find('>').context("bad root tag")? + r.start;
    let tag = &body[r.start..tag_end];
    let new_tag = match tag.find("PresetName=\"") {
        Some(i) => {
            let v0 = i + "PresetName=\"".len();
            let v1 = tag[v0..].find('"').context("bad PresetName")? + v0;
            format!("{}{}{}", &tag[..v0], esc(name), &tag[v1..])
        }
        None => format!("<Root PresetName=\"{}\"{}", esc(name), &tag["<Root".len()..]),
    };
    let new = format!("{}{}{}", &body[..r.start], new_tag, &body[tag_end..]);
    write_atomic(&target, new.as_bytes())?;
    Ok(target)
}

/// Make `name` the active preset for the given contexts (indexes 0 General, 1 Ship, 2 SRV,
/// 3 On foot) in the newest StartPreset file (backed up first). Creates `StartPreset.4.start`
/// if there is none.
pub fn set_active(bindings_dir: &Path, name: &str, contexts: &[usize]) -> Result<EditResult> {
    let sp = crate::profile::start_preset(bindings_dir);
    let mut names = if sp.names.len() == 4 { sp.names.clone() } else { vec![name.to_string(); 4] };
    for &c in contexts {
        if c < 4 {
            names[c] = name.to_string();
        }
    }
    let path = sp.path.clone().unwrap_or_else(|| bindings_dir.join("StartPreset.4.start"));
    let backup = if path.exists() { backup(&path)? } else { PathBuf::new() };
    let nl = std::fs::read_to_string(&path).ok().filter(|t| t.contains("\r\n")).map(|_| "\r\n").unwrap_or("\n");
    write_atomic(&path, names.join(nl).as_bytes())?;
    Ok(EditResult { path, backup })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::tests::X;
    use crate::profile::{parse_actions, view};

    fn b(dev: &str, key: &str) -> Slot {
        Slot { slot: "Primary".into(), device: dev.into(), key: key.into(), modifiers: vec![] }
    }

    #[test]
    fn edits_only_the_slot() {
        let out = set_slot_text(X, "NightVisionToggle", "Secondary", Some(&b("Keyboard", "Key_N")), false).unwrap();
        let a = parse_actions(&out).unwrap();
        assert_eq!(a["NightVisionToggle"][1].key, "Key_N");
        assert_eq!(a["NightVisionToggle"][0].key, "Joy_33");
        // Every other line is byte-identical.
        let diff: Vec<(&str, &str)> = X.lines().zip(out.lines()).filter(|(a, b)| a != b).collect();
        assert_eq!(
            diff,
            vec![(
                "\t\t<Secondary Device=\"{NoDevice}\" Key=\"\" />",
                "\t\t<Secondary Device=\"Keyboard\" Key=\"Key_N\" />"
            )]
        );
        // Clearing a slot with modifiers.
        let out = set_slot_text(X, "ShipSpotLightToggle", "Secondary", None, false).unwrap();
        assert!(parse_actions(&out).unwrap()["ShipSpotLightToggle"][1].device.is_empty());
        assert!(!out.contains("Key_LeftShift"));
        // With a modifier.
        let mut m = b("231D0200", "Joy_3");
        m.modifiers.push(Modifier { device: "231D3201".into(), key: "Joy_5".into() });
        let out = set_slot_text(X, "LandingGearToggle", "Primary", Some(&m), false).unwrap();
        let a = parse_actions(&out).unwrap();
        assert_eq!(a["LandingGearToggle"][0].modifiers[0].key, "Joy_5");
        assert_eq!(a["LandingGearToggle"][1].key, "Key_L");
    }

    #[test]
    fn adds_missing_actions_and_axes() {
        let out = set_slot_text(X, "SetSpeed50", "Secondary", Some(&b("Keyboard", "Key_5")), false).unwrap();
        let a = parse_actions(&out).unwrap();
        assert_eq!(a["SetSpeed50"].len(), 2);
        assert!(!a["SetSpeed50"][0].bound());
        assert_eq!(a["SetSpeed50"][1].key, "Key_5");
        assert!(out.trim_end().ends_with("</Root>"));
        let out = set_slot_text(X, "ThrottleAxis", "Binding", Some(&b("231D0200", "Joy_ZAxis")), true).unwrap();
        assert!(out.contains("<Inverted Value=\"0\" />"), "axis settings kept");
        assert_eq!(parse_actions(&out).unwrap()["ThrottleAxis"][0].key, "Joy_ZAxis");
        assert!(set_slot_text(X, "Bad\"Name", "Primary", None, false).is_err());
        assert!(set_slot_text(X, "LandingGearToggle", "Tertiary", None, false).is_err());
    }

    #[test]
    fn file_edit_backup_restore_copy_and_activate() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("DualVKB.4.2.binds");
        std::fs::write(&p, X).unwrap();
        let r = set_slot(d.path(), &p, "NightVisionToggle", "Secondary", Some(&b("Keyboard", "Key_N")), false).unwrap();
        assert_eq!(std::fs::read_to_string(&r.backup).unwrap(), X);
        let v = view(&p, true, None).unwrap();
        let nv = v.groups.iter().flat_map(|g| &g.rows).find(|r| r.id == "NightVisionToggle").unwrap();
        assert!(nv.issues.iter().all(|i| i.code != "no_keyboard"));
        let bs = list_backups(d.path());
        assert_eq!(bs.len(), 1);
        assert_eq!(bs[0].original, "DualVKB.4.2.binds");
        restore(d.path(), &bs[0].path).unwrap();
        assert_eq!(std::fs::read_to_string(&p).unwrap(), X);
        assert_eq!(list_backups(d.path()).len(), 2, "restoring backs up the current file first");

        // Copy, then edits go to the copy.
        let c = copy_preset(d.path(), &p, "DualVKB astromech").unwrap();
        assert!(c.ends_with("DualVKB astromech.4.2.binds"));
        assert!(std::fs::read_to_string(&c).unwrap().contains("PresetName=\"DualVKB astromech\""));
        assert!(copy_preset(d.path(), &p, "DualVKB astromech").is_err());
        assert!(copy_preset(d.path(), &p, "bad/name").is_err());

        // Files outside the Bindings folder are refused.
        let other = tempfile::tempdir().unwrap();
        let q = other.path().join("X.4.2.binds");
        std::fs::write(&q, X).unwrap();
        assert!(set_slot(d.path(), &q, "LandingGearToggle", "Primary", None, false).is_err());

        std::fs::write(d.path().join("StartPreset.4.start"), "DualVKB\r\nDualVKB\r\nDualVKB\r\nDualVKB").unwrap();
        set_active(d.path(), "DualVKB astromech", &[1, 2]).unwrap();
        assert_eq!(
            std::fs::read_to_string(d.path().join("StartPreset.4.start")).unwrap(),
            "DualVKB\r\nDualVKB astromech\r\nDualVKB astromech\r\nDualVKB"
        );
        let ps = crate::profile::list(d.path(), &[], None);
        let cp = ps.iter().find(|p| p.name == "DualVKB astromech").unwrap();
        assert_eq!(cp.active_for, ["Ship", "SRV"]);
    }
}
