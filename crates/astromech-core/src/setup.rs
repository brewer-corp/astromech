//! Onboarding (V1-DESIGN.md §1.5 #1, §2.2): finding Elite's folders, the config edits the
//! window makes (push-to-talk, kill switch, mic, folders), and whether setup was finished.
//! Edits keep the rest of `astromech.toml`, comments included.

use crate::config::{Button, Config};
use anyhow::{bail, Context, Result};
use astromech_keybinds::game::{self, GameInstall};
use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Where Elite keeps what astromech reads. Bindings and journals appear the first time the game
/// runs on this account; the install is only needed for built-in preset names.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Found {
    pub bindings_dir: Option<String>,
    pub bindings_found: bool,
    pub journal_dir: Option<String>,
    pub journal_found: bool,
    pub install: Option<String>,
    pub install_source: Option<String>,
    /// Bindings or journals found: astromech can work.
    pub ok: bool,
}

pub fn found_in(bindings: Option<PathBuf>, journal: Option<PathBuf>, installs: &[GameInstall]) -> Found {
    let bindings_found = bindings.as_deref().is_some_and(Path::is_dir);
    let journal_found = journal.as_deref().is_some_and(Path::is_dir);
    let first = installs.first();
    Found {
        bindings_dir: bindings.map(|p| p.display().to_string()),
        bindings_found,
        journal_dir: journal.map(|p| p.display().to_string()),
        journal_found,
        install: first.map(|g| g.product_dir.display().to_string()),
        install_source: first.map(|g| g.source.clone()),
        ok: bindings_found || journal_found,
    }
}

pub fn find_elite(cfg: &Config) -> Found {
    let installs = game::locate_installs(cfg.game_dir().as_deref(), &mut vec![]);
    found_in(cfg.bindings_dir(), cfg.journal_dir(), &installs)
}

/// One change to `astromech.toml` from the window.
#[derive(Debug, Clone, PartialEq)]
pub enum Edit {
    Ptt(Option<Button>),
    KillSwitch(Option<Button>),
    Mic(String),
    BindingsDir(String),
    JournalDir(String),
    /// Kokoro v1.0 voice name (Settings → Voice responses).
    Speaker(String),
    Speed(f32),
    Colour(astromech_voice::colour::Colour),
    /// `[eddn] enabled` (Settings → Data sharing).
    Eddn(bool),
    /// `[edsm] enabled` and `[edsm] commander` (Settings → Data sharing).
    Edsm(bool),
    EdsmCommander(String),
    /// `[callouts.<id>] enabled` (Settings → Callouts).
    CalloutEnabled(String, bool),
    /// `[callouts.<id>] cooldown_s`; None goes back to the callout's own cooldown.
    CalloutCooldown(String, Option<f64>),
}

/// The longest cooldown Settings accepts (a day).
pub const MAX_COOLDOWN_S: f64 = 86_400.0;

/// Kokoro speeds Settings may set (1.0 is the colour's own pace).
pub const SPEEDS: std::ops::RangeInclusive<f32> = 0.7..=1.3;

impl Edit {
    /// From a control-API request: `set_ptt` / `set_kill_switch` with `{device, key}` (or null
    /// to clear), `set_mic` with `{mic}`, `set_folders` with `{bindings_dir}` or `{journal_dir}`,
    /// `set_voice` with any of `{speaker, speed, colour}`, `set_eddn` with `{enabled}`, `set_edsm`
    /// with `{enabled}` and/or `{commander}` (its `api_key` goes to the keyring, not here; with
    /// only a key there is nothing to edit), `set_callout` with `{id}` and `enabled` and/or
    /// `cooldown_s` (null: the callout's own).
    pub fn from_request(cmd: &str, args: Option<&Value>) -> Result<Vec<Edit>> {
        let null = Value::Null;
        let a = args.unwrap_or(&null);
        let button = || -> Result<Option<Button>> {
            if a.is_null() {
                return Ok(None);
            }
            let b: Button = serde_json::from_value(a.clone()).context("expected {device, key}")?;
            if b.device.trim().is_empty() || b.key.trim().is_empty() {
                bail!("expected {{device, key}}");
            }
            Ok(Some(b))
        };
        let text = |k: &str| a.get(k).and_then(Value::as_str).map(|s| s.trim().to_string());
        Ok(match cmd {
            "set_ptt" => vec![Edit::Ptt(button()?)],
            "set_kill_switch" => vec![Edit::KillSwitch(button()?)],
            "set_mic" => vec![Edit::Mic(text("mic").context("expected {mic}")?)],
            "set_folders" => {
                let mut v = vec![];
                if let Some(d) = text("bindings_dir") {
                    v.push(Edit::BindingsDir(d));
                }
                if let Some(d) = text("journal_dir") {
                    v.push(Edit::JournalDir(d));
                }
                if v.is_empty() {
                    bail!("expected {{bindings_dir}} or {{journal_dir}}");
                }
                v
            }
            "set_voice" => {
                let mut v = vec![];
                if let Some(sp) = text("speaker") {
                    if astromech_voice::catalog::voice_sid(&sp).is_none() {
                        bail!("unknown voice {sp:?}");
                    }
                    v.push(Edit::Speaker(sp));
                }
                if let Some(x) = a.get("speed").filter(|x| !x.is_null()) {
                    let x = x.as_f64().context("speed must be a number")? as f32;
                    if !SPEEDS.contains(&x) {
                        bail!("speed must be between {} and {}", SPEEDS.start(), SPEEDS.end());
                    }
                    v.push(Edit::Speed(x));
                }
                if let Some(c) = a.get("colour").filter(|x| !x.is_null()) {
                    v.push(Edit::Colour(
                        serde_json::from_value(c.clone()).context("colour is natural, cockpit or harsh")?,
                    ));
                }
                if v.is_empty() {
                    bail!("expected {{speaker}}, {{speed}} or {{colour}}");
                }
                v
            }
            "set_eddn" => {
                vec![Edit::Eddn(a.get("enabled").and_then(Value::as_bool).context("expected {enabled: true|false}")?)]
            }
            "set_edsm" => {
                let mut v = vec![];
                if let Some(on) = a.get("enabled").filter(|x| !x.is_null()) {
                    v.push(Edit::Edsm(on.as_bool().context("enabled must be true or false")?));
                }
                if let Some(c) = a.get("commander").filter(|x| !x.is_null()) {
                    v.push(Edit::EdsmCommander(c.as_str().context("commander must be text")?.trim().to_string()));
                }
                if v.is_empty() && a.get("api_key").is_none() {
                    bail!("expected {{enabled}}, {{commander}} or {{api_key}}");
                }
                v
            }
            "set_callout" => {
                let id = text("id").context("expected {id}")?;
                if !astromech_announce::catalog().iter().any(|i| i.id == id) {
                    bail!("unknown callout {id:?}");
                }
                let mut v = vec![];
                if let Some(on) = a.get("enabled").filter(|x| !x.is_null()) {
                    v.push(Edit::CalloutEnabled(id.clone(), on.as_bool().context("enabled must be true or false")?));
                }
                match a.get("cooldown_s") {
                    None => {}
                    Some(Value::Null) => v.push(Edit::CalloutCooldown(id.clone(), None)),
                    Some(x) => {
                        let secs = x.as_f64().context("cooldown_s must be a number of seconds")?;
                        if !(0.0..=MAX_COOLDOWN_S).contains(&secs) {
                            bail!("cooldown_s must be between 0 and {MAX_COOLDOWN_S} seconds");
                        }
                        v.push(Edit::CalloutCooldown(id.clone(), Some(secs)));
                    }
                }
                if v.is_empty() {
                    bail!("expected {{enabled}} or {{cooldown_s}}");
                }
                v
            }
            c => bail!("unknown command {c:?}"),
        })
    }
}

fn button_value(b: &Button) -> toml_edit::Item {
    let mut t = toml_edit::InlineTable::new();
    t.insert("device", b.device.as_str().into());
    t.insert("key", b.key.as_str().into());
    toml_edit::value(t)
}

/// Set (or, with None, remove) `key` in `[callouts.<id>]`, creating the tables; an emptied
/// `[callouts.<id>]` goes too.
fn set_callout_key(doc: &mut toml_edit::DocumentMut, id: &str, key: &str, item: Option<toml_edit::Item>) -> Result<()> {
    if !doc.contains_key("callouts") {
        let mut t = toml_edit::Table::new();
        t.set_implicit(true);
        doc["callouts"] = toml_edit::Item::Table(t);
    }
    let callouts = doc["callouts"].as_table_like_mut().context("[callouts] isn't a table")?;
    match item {
        Some(v) => {
            if !callouts.contains_key(id) {
                callouts.insert(id, toml_edit::table());
            }
            let t = callouts.get_mut(id).and_then(|i| i.as_table_like_mut());
            t.with_context(|| format!("[callouts.{id}] isn't a table"))?.insert(key, v);
        }
        None => {
            if let Some(t) = callouts.get_mut(id).and_then(|i| i.as_table_like_mut()) {
                t.remove(key);
                if t.is_empty() {
                    callouts.remove(id);
                }
            }
        }
    }
    Ok(())
}

/// Apply edits to the config text, keeping everything else.
pub fn apply(text: &str, edits: &[Edit]) -> Result<String> {
    let mut doc: toml_edit::DocumentMut = text.parse().context("the config file has a syntax error")?;
    for e in edits {
        if let Edit::CalloutEnabled(id, _) | Edit::CalloutCooldown(id, _) = e {
            let (key, item) = match e {
                Edit::CalloutEnabled(_, on) => ("enabled", Some(toml_edit::value(*on))),
                Edit::CalloutCooldown(_, c) => ("cooldown_s", c.map(toml_edit::value)),
                _ => unreachable!(),
            };
            set_callout_key(&mut doc, id, key, item)?;
            continue;
        }
        let (table, key, item) = match e {
            Edit::Ptt(b) => ("controls", "ptt", b.as_ref().map(button_value)),
            Edit::KillSwitch(b) => ("controls", "kill_switch", b.as_ref().map(button_value)),
            Edit::Mic(m) => ("voice", "mic", Some(toml_edit::value(m.as_str()))),
            Edit::BindingsDir(d) => ("game", "bindings_dir", Some(toml_edit::value(d.as_str()))),
            Edit::JournalDir(d) => ("game", "journal_dir", Some(toml_edit::value(d.as_str()))),
            Edit::Speaker(sp) => ("voice", "speaker", Some(toml_edit::value(sp.as_str()))),
            // Two decimals, so 0.9 is written as 0.9 and not 0.8999999761581421.
            Edit::Speed(x) => ("voice", "speed", Some(toml_edit::value((f64::from(*x) * 100.0).round() / 100.0))),
            Edit::Colour(c) => ("voice", "colour", Some(toml_edit::value(c.as_str()))),
            Edit::Eddn(on) => ("eddn", "enabled", Some(toml_edit::value(*on))),
            Edit::Edsm(on) => ("edsm", "enabled", Some(toml_edit::value(*on))),
            Edit::EdsmCommander(c) => ("edsm", "commander", Some(toml_edit::value(c.as_str()))),
            Edit::CalloutEnabled(..) | Edit::CalloutCooldown(..) => unreachable!("handled above"),
        };
        if !doc.contains_key(table) {
            doc[table] = toml_edit::table();
        }
        let t = doc[table].as_table_like_mut().with_context(|| format!("[{table}] isn't a table"))?;
        match item {
            Some(mut v) => {
                // Replace the value in place, so the key's comment line and a trailing comment stay.
                match t.get_mut(key) {
                    Some(old) => {
                        if let (Some(o), Some(n)) = (old.as_value(), v.as_value_mut()) {
                            *n.decor_mut() = o.decor().clone();
                        }
                        *old = v;
                    }
                    None => {
                        t.insert(key, v);
                    }
                }
            }
            None => {
                t.remove(key);
            }
        }
        if matches!(e, Edit::Speaker(_)) {
            // A v0.19 speaker ID would only be read when the name is missing; drop it.
            t.remove("speaker_id");
        }
    }
    let out = doc.to_string();
    // Never write a file the core can't read back.
    let c = Config::parse(&out).context("the edited config doesn't parse")?;
    if let (Some(p), Some(k)) = (&c.controls.ptt, &c.controls.kill_switch) {
        if p.device.eq_ignore_ascii_case(&k.device) && p.key.eq_ignore_ascii_case(&k.key) {
            bail!("push-to-talk and the kill switch can't be the same button");
        }
    }
    Ok(out)
}

/// Edit `astromech.toml` in place (creating it if needed) and return the config it now holds.
pub fn write(path: &Path, edits: &[Edit]) -> Result<Config> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
    };
    let out = apply(&text, edits)?;
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    // A name of its own, so a writer outside [`edit`]'s lock can't rename another's half-written file.
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let tmp = path.with_extension(format!("toml.{}-{n}.tmp", std::process::id()));
    let res = std::fs::write(&tmp, &out)
        .with_context(|| format!("writing {}", tmp.display()))
        .and_then(|()| std::fs::rename(&tmp, path).with_context(|| format!("replacing {}", path.display())));
    if res.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    res?;
    Config::load(path)
}

/// Read, edit, write and use the config as one step: the control API serves each connection on
/// its own thread, so two quick edits (a double click, `set_mic` right after `set_ptt`) must not
/// interleave and lose one, or leave the in-memory config older than the file.
pub fn edit(lock: &Mutex<()>, path: &Path, edits: &[Edit], use_config: impl FnOnce(Config)) -> Result<()> {
    let _g = lock.lock().unwrap_or_else(|e| e.into_inner());
    let c = write(path, edits)?;
    use_config(c);
    Ok(())
}

const ONBOARDED: &str = "onboarding.done";

/// Has the user finished (or skipped) onboarding on this PC?
pub fn onboarded(store: &astromech_store::Store) -> bool {
    store.get(ONBOARDED).ok().flatten().is_some()
}

pub fn mark_onboarded(store: &astromech_store::Store) -> Result<()> {
    store.set(ONBOARDED, env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

const MIGRATED: &str = "onboarding.migrated";

/// At startup. Decided once, on the first start of a build with onboarding: someone upgrading
/// already has a config with push-to-talk, so they aren't walked through setup again. After that
/// first start only `onboarding.done` counts, because the wizard itself writes push-to-talk at its
/// buttons step and a restart there (headset, reboot, crash) must not skip the remaining steps.
pub fn onboarded_at_start(store: &astromech_store::Store, config_file_exists: bool, cfg: &Config) -> bool {
    if onboarded(store) {
        return true;
    }
    if store.get(MIGRATED).ok().flatten().is_some() {
        return false;
    }
    let _ = store.set(MIGRATED, env!("CARGO_PKG_VERSION"));
    if config_file_exists && cfg.controls.ptt.is_some() {
        return mark_onboarded(store).is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(d: &str, k: &str) -> Button {
        Button { device: d.into(), key: k.into() }
    }

    #[test]
    fn edits_keep_comments_and_other_settings() {
        let out =
            apply(crate::config::SAMPLE, &[Edit::Ptt(Some(b("Keyboard", "Key_F13"))), Edit::Mic("Example USB".into())])
                .unwrap();
        let c: Config = toml::from_str(&out).unwrap();
        assert_eq!(c.controls.ptt, Some(b("Keyboard", "Key_F13")));
        assert_eq!(c.voice.mic, "Example USB");
        // Untouched lines are byte-for-byte the same, comments included.
        assert!(out.contains("# Kill switch: press to cancel everything"));
        assert!(out.contains("# kill_switch = { device = \"Keyboard\", key = \"Key_ScrollLock\" }"));
        assert!(out.contains("# empty: Windows default recording device"), "{out}");
        // Push-to-talk is commented out in the sample, so it's added as one new line.
        assert_eq!(out.lines().count(), crate::config::SAMPLE.lines().count() + 1);
        assert!(out.contains("# ptt = { device = \"231D3201\""));
    }

    #[test]
    fn callout_edits_write_the_table_and_remove_it_when_empty() {
        let on = |id: &str| Edit::CalloutEnabled(id.into(), false);
        let out =
            apply("# top\n[eddn]\nenabled = false\n", &[on("scan"), Edit::CalloutCooldown("scan".into(), Some(45.0))])
                .unwrap();
        assert!(out.contains("# top") && out.contains("[eddn]"), "{out}");
        let c = Config::parse(&out).unwrap();
        assert_eq!(c.callouts.0["scan"].enabled, Some(false));
        assert_eq!(c.callouts.0["scan"].cooldown_s, Some(45.0));
        // Changing a value again replaces it; clearing both leaves no empty table behind.
        let out = apply(&out, &[Edit::CalloutCooldown("scan".into(), Some(10.0))]).unwrap();
        assert_eq!(Config::parse(&out).unwrap().callouts.0["scan"].cooldown_s, Some(10.0));
        let out = apply(&out, &[Edit::CalloutCooldown("scan".into(), None), Edit::CalloutEnabled("scan".into(), true)])
            .unwrap();
        assert_eq!(Config::parse(&out).unwrap().callouts.0["scan"].cooldown_s, None);
    }

    #[test]
    fn writes_a_new_file_and_clears_a_button() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("sub").join("astromech.toml");
        let c =
            write(&p, &[Edit::Ptt(Some(b("231D3201", "Joy_4"))), Edit::KillSwitch(Some(b("Keyboard", "Key_Pause")))])
                .unwrap();
        assert_eq!(c.controls.kill_switch, Some(b("Keyboard", "Key_Pause")));
        let c = write(&p, &[Edit::KillSwitch(None), Edit::BindingsDir("D:\\Bindings".into())]).unwrap();
        assert_eq!(c.controls.ptt, Some(b("231D3201", "Joy_4")));
        assert_eq!(c.controls.kill_switch, None);
        assert_eq!(c.game.bindings_dir, "D:\\Bindings");
        let files: Vec<_> = std::fs::read_dir(p.parent().unwrap()).unwrap().flatten().map(|e| e.file_name()).collect();
        assert_eq!(files, vec![std::ffi::OsString::from("astromech.toml")], "a temp file was left behind");
    }

    #[test]
    fn a_broken_config_is_not_overwritten() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("astromech.toml");
        std::fs::write(&p, "[controls\nptt = ").unwrap();
        assert!(write(&p, &[Edit::Mic("x".into())]).is_err());
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "[controls\nptt = ");
    }

    /// astromech-orel: the voice set in Settings is in the file, so it survives a restart.
    #[test]
    fn voice_settings_persist() {
        use astromech_voice::colour::Colour;
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("astromech.toml");
        std::fs::write(&p, "[voice]\nspeaker_id = 7 # old\n").unwrap();
        let j = serde_json::json!({"speaker": "bm_george", "speed": 0.9, "colour": "natural"});
        let edits = Edit::from_request("set_voice", Some(&j)).unwrap();
        write(&p, &edits).unwrap();
        let c = Config::load(&p).unwrap();
        assert_eq!((c.voice.speaker.as_str(), c.voice.speed, c.voice.colour), ("bm_george", 0.9, Colour::Natural));
        let text = std::fs::read_to_string(&p).unwrap();
        assert!(text.contains("speed = 0.9\n") && !text.contains("speaker_id"), "{text}");
        // Bad values are refused before anything is written.
        for bad in [r#"{"speaker":"xx_nobody"}"#, r#"{"speed":3.0}"#, r#"{"colour":"loud"}"#, "{}"] {
            let j: Value = serde_json::from_str(bad).unwrap();
            assert!(Edit::from_request("set_voice", Some(&j)).is_err(), "{bad}");
        }
    }

    #[test]
    fn the_edsm_settings_are_saved_without_the_key() {
        let j = |s: &str| serde_json::from_str::<Value>(s).unwrap();
        let e = Edit::from_request("set_edsm", Some(&j(r#"{"enabled":true,"commander":" Jameson ","api_key":"k"}"#)))
            .unwrap();
        assert_eq!(e, [Edit::Edsm(true), Edit::EdsmCommander("Jameson".into())]);
        assert_eq!(Edit::from_request("set_edsm", Some(&j(r#"{"api_key":"k"}"#))).unwrap(), []);
        assert!(Edit::from_request("set_edsm", Some(&j(r#"{"enabled":1}"#))).is_err());
        assert!(Edit::from_request("set_edsm", None).is_err());
        let out = apply(crate::config::SAMPLE, &e).unwrap();
        let c = Config::parse(&out).unwrap();
        assert!(c.edsm.enabled && c.edsm.commander == "Jameson");
        assert!(!out.contains("api_key") && !out.contains("= \"k\""), "{out}");
        assert!(
            out.lines().any(|l| l.starts_with("commander = \"Jameson\"") && l.contains("# your commander name")),
            "{out}"
        );
    }

    #[test]
    fn the_eddn_switch_is_saved_and_keeps_its_comment() {
        let j = |s: &str| serde_json::from_str::<Value>(s).unwrap();
        let on = Edit::from_request("set_eddn", Some(&j(r#"{"enabled":true}"#))).unwrap();
        assert_eq!(on, [Edit::Eddn(true)]);
        assert!(Edit::from_request("set_eddn", Some(&j(r#"{"enabled":"yes"}"#))).is_err());
        assert!(Edit::from_request("set_eddn", None).is_err());
        let out = apply(crate::config::SAMPLE, &on).unwrap();
        assert!(Config::parse(&out).unwrap().eddn.enabled);
        assert!(out.contains("enabled = true               # share market"), "{out}");
        let off = apply(&out, &[Edit::Eddn(false)]).unwrap();
        assert!(!Config::parse(&off).unwrap().eddn.enabled);
        // A config without the section gets one.
        assert!(Config::parse(&apply("", &on).unwrap()).unwrap().eddn.enabled);
    }

    #[test]
    fn requests_become_edits() {
        let j = |s: &str| serde_json::from_str::<Value>(s).unwrap();
        assert_eq!(
            Edit::from_request("set_ptt", Some(&j(r#"{"device":"Keyboard","key":"Key_F13"}"#))).unwrap(),
            vec![Edit::Ptt(Some(b("Keyboard", "Key_F13")))]
        );
        assert_eq!(Edit::from_request("set_kill_switch", None).unwrap(), vec![Edit::KillSwitch(None)]);
        assert!(Edit::from_request("set_ptt", Some(&j(r#"{"device":"","key":"Joy_1"}"#))).is_err());
        assert_eq!(Edit::from_request("set_mic", Some(&j(r#"{"mic":" "}"#))).unwrap(), vec![Edit::Mic(String::new())]);
        assert_eq!(
            Edit::from_request("set_folders", Some(&j(r#"{"journal_dir":"D:\\J"}"#))).unwrap(),
            vec![Edit::JournalDir("D:\\J".into())]
        );
        assert!(Edit::from_request("set_folders", Some(&j("{}"))).is_err());
        assert!(Edit::from_request("quit", None).is_err());
    }

    #[test]
    fn finds_elite_from_its_folders() {
        // No Elite install at all: nothing found, nothing to show but the defaults.
        let none = found_in(None, None, &[]);
        assert!(!none.ok && !none.bindings_found && none.install.is_none());
        let d = tempfile::tempdir().unwrap();
        let missing = found_in(Some(d.path().join("Bindings")), Some(d.path().join("Journals")), &[]);
        assert!(!missing.ok);
        assert!(missing.bindings_dir.unwrap().ends_with("Bindings"));
        std::fs::create_dir(d.path().join("Journals")).unwrap();
        let install = GameInstall { source: "Steam".into(), product_dir: d.path().join("Products").join("odyssey") };
        let f = found_in(Some(d.path().join("Bindings")), Some(d.path().join("Journals")), &[install]);
        assert!(f.ok && f.journal_found && !f.bindings_found);
        assert_eq!(f.install_source.as_deref(), Some("Steam"));
    }

    #[test]
    fn concurrent_edits_all_survive_in_file_and_memory() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("astromech.toml");
        std::fs::write(&p, crate::config::SAMPLE).unwrap();
        let lock = Mutex::new(());
        let mem = std::sync::RwLock::new(Config::load(&p).unwrap());
        std::thread::scope(|s| {
            for t in 0..2 {
                let (lock, mem, p) = (&lock, &mem, &p);
                s.spawn(move || {
                    for i in 0..100 {
                        let e = if t == 0 {
                            Edit::Ptt(Some(b("Keyboard", &format!("Key_F{}", i % 12 + 1))))
                        } else {
                            Edit::Mic(format!("mic {i}"))
                        };
                        edit(lock, p, &[e], |c| *mem.write().unwrap() = c).unwrap();
                    }
                });
            }
        });
        let file = Config::load(&p).unwrap();
        let mem = mem.into_inner().unwrap();
        for c in [&file, &mem] {
            assert_eq!(c.controls.ptt, Some(b("Keyboard", "Key_F4")));
            assert_eq!(c.voice.mic, "mic 99");
        }
        let leftovers: Vec<_> = std::fs::read_dir(d.path()).unwrap().flatten().map(|e| e.file_name()).collect();
        assert_eq!(leftovers.len(), 1, "{leftovers:?}");
    }

    #[test]
    fn ptt_and_kill_switch_must_differ() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("astromech.toml");
        write(&p, &[Edit::Ptt(Some(b("231D3201", "Joy_4")))]).unwrap();
        let e = write(&p, &[Edit::KillSwitch(Some(b("231d3201", "joy_4")))]).unwrap_err();
        assert!(format!("{e:#}").contains("can't be the same button"), "{e:#}");
        assert_eq!(Config::load(&p).unwrap().controls.kill_switch, None);
        write(&p, &[Edit::KillSwitch(Some(b("231D3201", "Joy_5")))]).unwrap();
    }

    fn with_ptt() -> Config {
        toml::from_str("[controls]\nptt = { device = \"231D3201\", key = \"Joy_4\" }").unwrap()
    }

    #[test]
    fn upgrading_users_with_push_to_talk_skip_onboarding() {
        let st = astromech_store::Store::in_memory().unwrap();
        // First start with onboarding: an existing config with push-to-talk is an upgrade.
        assert!(onboarded_at_start(&st, true, &with_ptt()));
        assert!(onboarded(&st));
        assert!(onboarded_at_start(&st, true, &with_ptt()));
    }

    #[test]
    fn fresh_installs_need_onboarding() {
        let st = astromech_store::Store::in_memory().unwrap();
        assert!(!onboarded_at_start(&st, false, &Config::default()));
        let st = astromech_store::Store::in_memory().unwrap();
        // A config file without push-to-talk (e.g. --write-config's sample) still needs setup.
        let sample: Config = toml::from_str(crate::config::SAMPLE).unwrap();
        assert!(!onboarded_at_start(&st, true, &sample));
        assert!(!onboarded(&st));
    }

    #[test]
    fn ptt_written_by_the_wizard_then_restart_is_still_not_onboarded() {
        let st = astromech_store::Store::in_memory().unwrap();
        // First launch of a fresh install: no config yet.
        assert!(!onboarded_at_start(&st, false, &Config::default()));
        // The wizard's buttons step writes push-to-talk, then the PC restarts mid-setup.
        assert!(!onboarded_at_start(&st, true, &with_ptt()));
        assert!(!onboarded(&st));
        // Finishing the wizard is what marks it done.
        mark_onboarded(&st).unwrap();
        assert!(onboarded_at_start(&st, true, &with_ptt()));
    }

    #[test]
    fn onboarding_flag() {
        let st = astromech_store::Store::in_memory().unwrap();
        assert!(!onboarded(&st));
        mark_onboarded(&st).unwrap();
        assert!(onboarded(&st));
    }
}
