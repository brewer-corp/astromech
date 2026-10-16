//! Onboarding (V1-DESIGN.md §1.5 #1, §2.2): finding Elite's folders, the config edits the
//! window makes (push-to-talk, kill switch, mic, folders), and whether setup was finished.
//! Edits keep the rest of `astromech.toml`, comments included.

use crate::config::{Button, Config};
use anyhow::{bail, Context, Result};
use astromech_keybinds::game::{self, GameInstall};
use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};

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
}

impl Edit {
    /// From a control-API request: `set_ptt` / `set_kill_switch` with `{device, key}` (or null
    /// to clear), `set_mic` with `{mic}`, `set_folders` with `{bindings_dir}` or `{journal_dir}`.
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

/// Apply edits to the config text, keeping everything else.
pub fn apply(text: &str, edits: &[Edit]) -> Result<String> {
    let mut doc: toml_edit::DocumentMut = text.parse().context("the config file has a syntax error")?;
    for e in edits {
        let (table, key, item) = match e {
            Edit::Ptt(b) => ("controls", "ptt", b.as_ref().map(button_value)),
            Edit::KillSwitch(b) => ("controls", "kill_switch", b.as_ref().map(button_value)),
            Edit::Mic(m) => ("voice", "mic", Some(toml_edit::value(m.as_str()))),
            Edit::BindingsDir(d) => ("game", "bindings_dir", Some(toml_edit::value(d.as_str()))),
            Edit::JournalDir(d) => ("game", "journal_dir", Some(toml_edit::value(d.as_str()))),
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
    }
    let out = doc.to_string();
    // Never write a file the core can't read back.
    toml::from_str::<Config>(&out).context("the edited config doesn't parse")?;
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
    let tmp = path.with_extension("toml.tmp");
    std::fs::write(&tmp, &out).with_context(|| format!("writing {}", tmp.display()))?;
    std::fs::rename(&tmp, path).with_context(|| format!("replacing {}", path.display()))?;
    Config::load(path)
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
        assert!(out.contains("kill_switch = { device = \"Keyboard\", key = \"Key_ScrollLock\" }"));
        assert!(out.contains("# empty: Windows default recording device"), "{out}");
        assert_eq!(out.lines().count(), crate::config::SAMPLE.lines().count());
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
        assert!(!p.with_extension("toml.tmp").exists());
    }

    #[test]
    fn a_broken_config_is_not_overwritten() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("astromech.toml");
        std::fs::write(&p, "[controls\nptt = ").unwrap();
        assert!(write(&p, &[Edit::Mic("x".into())]).is_err());
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "[controls\nptt = ");
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
    fn onboarding_flag() {
        let st = astromech_store::Store::in_memory().unwrap();
        assert!(!onboarded(&st));
        mark_onboarded(&st).unwrap();
        assert!(onboarded(&st));
    }
}
