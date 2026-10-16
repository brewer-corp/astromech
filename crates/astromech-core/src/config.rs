//! `astromech.toml`: which buttons do what, where the game's files are, timings.

use anyhow::{Context, Result};
use astromech_input::listen::model::Event;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A button on any device, in Elite's naming.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Button {
    /// "Keyboard", "Mouse", "GamePad", or an Elite device ID such as "231D3201".
    pub device: String,
    /// "Joy_4", "Key_RightControl", "Mouse_4", "Pad_A" ...
    pub key: String,
}

impl Button {
    /// Does a listener event come from this button?
    pub fn matches(&self, e: &Event) -> bool {
        let Some(name) = e.elite_name.as_deref() else { return false };
        if !name.eq_ignore_ascii_case(&self.key) {
            return false;
        }
        match self.device.to_ascii_lowercase().as_str() {
            "keyboard" => e.kind == "key",
            "mouse" => e.kind == "mouse",
            "gamepad" => e.kind == "xinput",
            d => e.kind == "button" && e.elite_id.as_deref().map(|i| i.eq_ignore_ascii_case(d)).unwrap_or(false),
        }
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on {}", self.key, self.device)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Game {
    /// Journal + Status.json folder. Empty: `%USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous`.
    pub journal_dir: String,
    /// Empty: `%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings`.
    pub bindings_dir: String,
    /// Optional game install folder (for built-in presets and DeviceMappings.xml).
    pub game_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Controls {
    pub ptt: Option<Button>,
    pub kill_switch: Option<Button>,
    /// Hold the kill-switch button this long to re-arm after a kill.
    pub rearm_hold_ms: u64,
    /// A PTT release followed by a press within this window is bounce: one push, one clip.
    pub debounce_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Voice {
    /// Folder that contains the extracted model folders. Empty: `models` next to the exe.
    pub models_dir: String,
    /// The voice worker exe. Empty: next to astromech-core.exe.
    pub worker: String,
    pub threads: i32,
    /// Speak confirmations with Kokoro (otherwise print them).
    pub tts: bool,
    pub speaker_id: i32,
    pub speed: f32,
    /// Kokoro colour (V1-DESIGN.md §6.4). Default cockpit.
    pub colour: astromech_voice::colour::Colour,
    /// Audio kept from before the PTT press, and after release (ms).
    pub pre_roll_ms: u64,
    pub post_roll_ms: u64,
    /// PTT holds shorter than this are dropped silently (a tap or bounce, not speech).
    pub min_hold_ms: u64,
    /// Clips whose loudest 30 ms frame is below this level (dBFS) are dropped silently.
    pub min_level_dbfs: f32,
    /// Input device name (substring). Empty: Windows default recording device.
    pub mic: String,
    /// Save gated PTT clips (WAV + sidecar JSON) under `data/clips` for the eval harness.
    pub save_clips: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Actions {
    pub hold_ms: u64,
    pub verify_timeout_ms: u64,
    /// Deprecated and ignored: commands are never pressed again (see ARCHITECTURE.md §5.5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
    /// Log key presses instead of sending them.
    pub dry_run: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub game: Game,
    pub controls: Controls,
    pub voice: Voice,
    pub actions: Actions,
    pub update: crate::update::Settings,
    /// Where astromech keeps its database, logs and TTS cache. Empty: `astromech-data` next to the exe.
    pub data_dir: String,
}

impl Default for Controls {
    fn default() -> Self {
        Self { ptt: None, kill_switch: None, rearm_hold_ms: 1000, debounce_ms: 150 }
    }
}
impl Default for Voice {
    fn default() -> Self {
        Self {
            models_dir: String::new(),
            worker: String::new(),
            threads: astromech_voice::DEFAULT_THREADS,
            tts: true,
            speaker_id: 0,
            speed: 1.0,
            colour: astromech_voice::colour::Colour::Cockpit,
            pre_roll_ms: 150,
            post_roll_ms: 200,
            min_hold_ms: 300,
            min_level_dbfs: -45.0,
            mic: String::new(),
            save_clips: false,
        }
    }
}
impl Default for Actions {
    fn default() -> Self {
        Self { hold_ms: 50, verify_timeout_ms: 2000, retries: None, dry_run: false }
    }
}

pub const SAMPLE: &str = include_str!("../astromech.sample.toml");

fn opt(s: &str) -> Option<PathBuf> {
    let s = s.trim();
    (!s.is_empty()).then(|| PathBuf::from(s))
}

/// Files next to the exe that look like a mis-named `astromech.toml` (Windows hides
/// extensions, so Notepad's "astromech.toml.txt" shows as "astromech.toml").
pub fn look_alikes(dir: &Path) -> Vec<PathBuf> {
    let Ok(rd) = std::fs::read_dir(dir) else { return vec![] };
    let mut v: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .filter(|p| {
            let n = p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
            n != "astromech.toml"
                && n != "astromech.sample.toml"
                && n.starts_with("astromech")
                && (n.contains(".toml") || n.ends_with(".txt") || n.ends_with(".tml") || !n.contains('.'))
                && !n.ends_with(".exe")
        })
        .collect();
    v.sort();
    v
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let t = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        toml::from_str(&t).with_context(|| format!("parsing {}", path.display()))
    }

    pub fn exe_dir() -> PathBuf {
        std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| ".".into())
    }

    pub fn journal_dir(&self) -> Option<PathBuf> {
        opt(&self.game.journal_dir).or_else(|| {
            std::env::var_os("USERPROFILE")
                .map(|h| PathBuf::from(h).join("Saved Games").join("Frontier Developments").join("Elite Dangerous"))
        })
    }

    pub fn bindings_dir(&self) -> Option<PathBuf> {
        opt(&self.game.bindings_dir).or_else(astromech_keybinds::game::default_bindings_dir)
    }

    pub fn game_dir(&self) -> Option<PathBuf> {
        opt(&self.game.game_dir)
    }

    /// Per-user data folder: `%LOCALAPPDATA%\astromech\data`. The installer puts the program
    /// in `%LOCALAPPDATA%\astromech` (Tauri's per-user default); config, logs, models and data
    /// live in its `data` subfolder, which upgrades and uninstalls leave alone unless "Delete the
    /// application data" is ticked.
    pub fn app_dir() -> PathBuf {
        let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).or_else(|| {
            std::env::var_os("XDG_DATA_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share")))
        });
        base.map(|b| b.join("astromech").join("data")).unwrap_or_else(|| Self::exe_dir().join("astromech-data"))
    }

    /// Portable layout (the gear-down preview zip): an `astromech.toml` next to the exe. Then
    /// data and models stay next to the exe as before.
    pub fn portable() -> bool {
        Self::exe_dir().join("astromech.toml").is_file()
    }

    /// Where the config file is when `--config` isn't given.
    pub fn default_path() -> PathBuf {
        if Self::portable() {
            Self::exe_dir().join("astromech.toml")
        } else {
            Self::app_dir().join("astromech.toml")
        }
    }

    pub fn models_dir(&self) -> PathBuf {
        opt(&self.voice.models_dir).unwrap_or_else(|| {
            let local = Self::exe_dir().join("models");
            if Self::portable() || local.is_dir() {
                local
            } else {
                Self::app_dir().join("models")
            }
        })
    }

    pub fn worker_exe(&self) -> PathBuf {
        opt(&self.voice.worker).unwrap_or_else(|| Self::exe_dir().join(astromech_voice::client::WORKER_EXE))
    }

    pub fn data_dir(&self) -> PathBuf {
        opt(&self.data_dir).unwrap_or_else(|| {
            if Self::portable() {
                Self::exe_dir().join("astromech-data")
            } else {
                Self::app_dir()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(kind: &str, id: Option<&str>, name: &str) -> Event {
        Event {
            t_ms: 0.0,
            dev: String::new(),
            device: String::new(),
            elite_id: id.map(str::to_string),
            kind: kind.into(),
            control: String::new(),
            elite_name: Some(name.into()),
            down: Some(true),
            axis: None,
            foreground: String::new(),
            elite_focused: true,
            injected: false,
            actions: vec![],
            note: None,
        }
    }

    #[test]
    fn finds_look_alike_config_names() {
        let d = tempfile::tempdir().unwrap();
        for n in [
            "astromech.toml.txt",
            "astromech.toml.toml",
            "astromech (1).toml",
            "astromech",
            "astromech.sample.toml",
            "astromech-core.exe",
            "other.toml",
        ] {
            std::fs::write(d.path().join(n), "").unwrap();
        }
        let got: Vec<String> =
            look_alikes(d.path()).iter().map(|p| p.file_name().unwrap().to_string_lossy().to_string()).collect();
        assert_eq!(got, vec!["astromech", "astromech (1).toml", "astromech.toml.toml", "astromech.toml.txt"]);
    }

    #[test]
    fn sample_parses_and_buttons_match() {
        let c: Config = toml::from_str(SAMPLE).unwrap();
        // The sample leaves push-to-talk to the user (onboarding treats a set one as set up).
        assert!(c.controls.ptt.is_none());
        let line = SAMPLE.lines().find(|l| l.starts_with("# ptt = ")).unwrap();
        let ptt = toml::from_str::<Controls>(line.trim_start_matches("# ")).unwrap().ptt.unwrap();
        assert_eq!(ptt, Button { device: "231D3201".into(), key: "Joy_4".into() });
        assert!(ptt.matches(&ev("button", Some("231D3201"), "Joy_4")));
        assert!(!ptt.matches(&ev("button", Some("231D0200"), "Joy_4")));
        assert!(!ptt.matches(&ev("button", Some("231D3201"), "Joy_5")));
        let k = Button { device: "Keyboard".into(), key: "Key_RightControl".into() };
        assert!(k.matches(&ev("key", None, "Key_RightControl")));
        assert!(!k.matches(&ev("button", Some("Keyboard"), "Key_RightControl")));
        assert_eq!((c.actions.hold_ms, c.actions.verify_timeout_ms, c.actions.retries), (50, 2000, None));
        assert_eq!(c.voice.threads, 4);
        assert_eq!(c.voice.colour, astromech_voice::colour::Colour::Cockpit);
        assert!(c.update.check);
        assert_eq!(c.update.apply, crate::update::ApplyWhen::Ask);
        let d: Config = toml::from_str("").unwrap();
        assert!(d.controls.ptt.is_none() && d.voice.tts);
        assert_eq!(d.voice.colour, astromech_voice::colour::Colour::Cockpit);
    }
}
