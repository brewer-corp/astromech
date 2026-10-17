//! `astromech.toml`: which buttons do what, where the game's files are, timings.

use anyhow::{Context, Result};
use astromech_input::listen::model::Event;
use astromech_store::Paths;
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
    /// Pastes the clipboard (the next waypoint) into the galaxy map's search box. Pick a
    /// button Elite doesn't use: its own binding would fire too.
    pub paste: Option<Button>,
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
    /// Kokoro v1.0 voice by name ("bf_emma"; `astromech_voice::catalog::KOKORO_VOICES`).
    /// Missing or unknown: the default voice (see [`Voice::settle_speaker`]).
    #[serde(default)]
    pub speaker: String,
    /// Kokoro v0.19 speaker ID from configs written before v1.0. Read once, turned into
    /// `speaker`, never written.
    #[serde(default, skip_serializing)]
    pub speaker_id: Option<i32>,
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

/// Sharing game data with EDDN (`astromech-eddn`). Off unless switched on in Settings → Data
/// sharing (`set_eddn`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Eddn {
    pub enabled: bool,
    /// Send to the `/test` schemas. Debug builds always do.
    pub test: bool,
}

/// Uploading the journal to EDSM (`astromech_eddn::edsm`), for the commander's EDSM flight log
/// and map. Off unless switched on in Settings → Data sharing (`set_edsm`). The API key is in
/// the OS keyring, never here.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Edsm {
    pub enabled: bool,
    /// The commander name as registered on EDSM.
    pub commander: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub game: Game,
    pub controls: Controls,
    pub voice: Voice,
    pub actions: Actions,
    pub update: crate::update::Settings,
    /// Per-callout toggle and cooldown: `[callouts.<id>]` with `enabled` and `cooldown_s`.
    pub callouts: astromech_announce::Settings,
    pub eddn: Eddn,
    pub edsm: Edsm,
    /// Where astromech keeps its database, logs and TTS cache. Empty: `astromech-data` next to the exe.
    pub data_dir: String,
}

impl Default for Controls {
    fn default() -> Self {
        Self { ptt: None, kill_switch: None, paste: None, rearm_hold_ms: 1000, debounce_ms: 150 }
    }
}
impl Default for Voice {
    fn default() -> Self {
        Self {
            models_dir: String::new(),
            worker: String::new(),
            threads: astromech_voice::DEFAULT_THREADS,
            tts: true,
            speaker: astromech_voice::catalog::DEFAULT_VOICE.into(),
            speaker_id: None,
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
impl Voice {
    /// Make `speaker` a Kokoro v1.0 voice name: keep a known name; otherwise take the v1.0 voice
    /// matching an old v0.19 `speaker_id`; otherwise the default voice.
    pub fn settle_speaker(&mut self) {
        use astromech_voice::catalog;
        let old = self.speaker_id.take();
        if catalog::voice_sid(&self.speaker).is_some() {
            return;
        }
        let named = self.speaker.trim().is_empty().then(|| old.and_then(catalog::voice_from_v0_19)).flatten();
        self.speaker = named.unwrap_or(catalog::DEFAULT_VOICE).to_string();
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

/// One value of `astromech.toml` that was ignored (the default is used for it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// Dotted key, such as `controls.rearm_hold_ms`.
    pub key: String,
    /// 1-based line in the file.
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {} ({})", self.line, self.key, self.message)
    }
}

/// The key path of the innermost value in `t` whose text covers byte `pos`.
fn locate(t: &dyn toml_edit::TableLike, pos: usize, path: &mut Vec<String>) -> bool {
    let covers = |item: &toml_edit::Item| item.span().is_some_and(|s| s.contains(&pos));
    for (k, item) in t.iter() {
        path.push(k.to_string());
        // A standard table's span is only its header, so look inside it whatever its span says.
        if let Some(sub) = item.as_table_like() {
            if locate(sub, pos, path) || covers(item) {
                return true;
            }
        } else if covers(item) {
            return true;
        }
        path.pop();
    }
    false
}

/// The item at `path` below `t`.
fn at_path<'a>(t: &'a dyn toml_edit::TableLike, path: &[String]) -> Option<&'a toml_edit::Item> {
    let (first, rest) = path.split_first()?;
    let item = t.get(first)?;
    if rest.is_empty() {
        Some(item)
    } else {
        at_path(item.as_table_like()?, rest)
    }
}

/// Remove the item at `path` below `t`.
fn remove_at(t: &mut dyn toml_edit::TableLike, path: &[String]) -> bool {
    match path {
        [] => false,
        [last] => t.remove(last).is_some(),
        [first, rest @ ..] => {
            t.get_mut(first).and_then(|i| i.as_table_like_mut()).is_some_and(|sub| remove_at(sub, rest))
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        Ok(Self::load_report(path)?.0)
    }

    /// Like [`Config::load`], and the values that were ignored. A file that isn't TOML is an error.
    pub fn load_report(path: &Path) -> Result<(Self, Vec<Problem>)> {
        let t = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        Self::parse_report(&t).with_context(|| format!("parsing {}", path.display()))
    }

    /// Parse `astromech.toml` text, bringing old settings up to date. Any bad value is an error.
    pub fn parse(text: &str) -> Result<Self> {
        let mut c: Self = toml::from_str(text)?;
        c.voice.settle_speaker();
        Ok(c)
    }

    /// Parse `astromech.toml` text, dropping each value that doesn't fit its setting (a wrong
    /// type, say) so it keeps its default and the rest of the file still applies.
    pub fn parse_report(text: &str) -> Result<(Self, Vec<Problem>)> {
        let original = toml_edit::Document::parse(text.to_string())?;
        let mut doc = original.clone().into_mut();
        let mut problems = vec![];
        loop {
            let current = doc.to_string();
            let err = match toml::from_str::<Self>(&current) {
                Ok(mut c) => {
                    c.voice.settle_speaker();
                    return Ok((c, problems));
                }
                Err(e) => e,
            };
            // Find the failing value in the current text, then drop it from `doc`.
            let reparsed = toml_edit::Document::parse(current.as_str())?;
            let mut path = vec![];
            let found = err.span().is_some_and(|s| locate(reparsed.as_table(), s.start, &mut path));
            if !found || !remove_at(doc.as_table_mut(), &path) {
                return Err(err.into());
            }
            // The line is in the file as written, not in the text with earlier values dropped.
            let line = at_path(original.as_table(), &path)
                .and_then(|i| i.span())
                .map(|s| text[..s.start.min(text.len())].matches('\n').count() + 1)
                .unwrap_or(1);
            problems.push(Problem { key: path.join("."), line, message: err.message().to_string() });
        }
    }

    pub fn journal_dir(&self) -> Option<PathBuf> {
        Paths::current().journal_dir(&self.game.journal_dir)
    }

    pub fn bindings_dir(&self) -> Option<PathBuf> {
        opt(&self.game.bindings_dir).or_else(astromech_keybinds::game::default_bindings_dir)
    }

    pub fn game_dir(&self) -> Option<PathBuf> {
        opt(&self.game.game_dir)
    }

    pub fn models_dir(&self) -> PathBuf {
        opt(&self.voice.models_dir).unwrap_or_else(|| {
            let paths = Paths::current();
            let local = paths.exe_dir().join("models");
            if paths.portable() || local.is_dir() {
                local
            } else {
                paths.app_dir().join("models")
            }
        })
    }

    pub fn worker_exe(&self) -> PathBuf {
        opt(&self.voice.worker).unwrap_or_else(|| Paths::current().exe_dir().join(astromech_voice::client::WORKER_EXE))
    }

    /// The data folder for this config (`astromech_store::Paths::data_dir`).
    pub fn data_dir(&self) -> PathBuf {
        Paths::current().data_dir(&self.data_dir)
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
        let c = Config::parse(SAMPLE).unwrap();
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
        assert_eq!(c.voice.speaker, "bf_emma");
        assert!(c.update.check);
        assert_eq!(c.update.apply, crate::update::ApplyWhen::Ask);
        assert!(!c.eddn.enabled && !c.eddn.test);
        let d = Config::parse("").unwrap();
        assert!(d.controls.ptt.is_none() && d.voice.tts);
        assert!(!d.eddn.enabled, "nothing is sent to EDDN unless switched on");
        assert_eq!(d.voice.colour, astromech_voice::colour::Colour::Cockpit);
        assert_eq!(d.voice.speaker, astromech_voice::catalog::DEFAULT_VOICE);
    }

    #[test]
    fn one_bad_value_costs_only_that_value() {
        let text = "[controls]\nptt = { device = \"231D3201\", key = \"Joy_4\" }\n\
                    rearm_hold_ms = \"slow\"\ndebounce_ms = 90\n\n[voice]\nthreads = 2\nspeed = true\n\n\
                    [eddn]\nenabled = true\n";
        assert!(Config::parse(text).is_err(), "the strict parse still rejects it");
        let (c, problems) = Config::parse_report(text).unwrap();
        assert_eq!(c.controls.ptt.unwrap().key, "Joy_4");
        assert_eq!((c.controls.debounce_ms, c.voice.threads, c.eddn.enabled), (90, 2, true));
        assert_eq!((c.controls.rearm_hold_ms, c.voice.speed), (1000, 1.0), "the bad keys take their defaults");
        let got: Vec<_> = problems.iter().map(|p| (p.key.as_str(), p.line)).collect();
        assert_eq!(got, [("controls.rearm_hold_ms", 3), ("voice.speed", 8)]);
        assert!(problems[0].to_string().starts_with("line 3: controls.rearm_hold_ms"), "{}", problems[0]);
        // A bad value inside an inline table drops that table, not the file.
        let (c, p) =
            Config::parse_report("[controls]\nptt = { device = 5, key = \"Joy_4\" }\nrearm_hold_ms = 7").unwrap();
        assert!(c.controls.ptt.is_none() && c.controls.rearm_hold_ms == 7 && p.len() == 2, "{p:?}");
        assert_eq!((p[0].key.as_str(), p[1].key.as_str()), ("controls.ptt.device", "controls.ptt"));
        assert!(Config::parse_report("[controls]\nptt = 1").unwrap().1[0].key.starts_with("controls.ptt"));
        // A whole section of the wrong type.
        let (c, p) = Config::parse_report("eddn = 3\n[edsm]\nenabled = true").unwrap();
        assert!(c.edsm.enabled && !c.eddn.enabled);
        assert_eq!((p[0].key.as_str(), p[0].line), ("eddn", 1));
        // A clean file reports nothing; a file that isn't TOML is still an error.
        assert!(Config::parse_report(SAMPLE).unwrap().1.is_empty());
        assert!(Config::parse_report("[controls\nptt = ").is_err());
    }

    #[test]
    fn old_speaker_id_migrates_to_a_v1_voice_name() {
        let speaker = |toml: &str| Config::parse(toml).unwrap().voice.speaker;
        // v0.19 IDs 7-10 are the British voices, all also in v1.0 (at other IDs).
        assert_eq!(speaker("[voice]\nspeaker_id = 7"), "bf_emma");
        assert_eq!(speaker("[voice]\nspeaker_id = 9"), "bm_george");
        assert_eq!(speaker("[voice]\nspeaker_id = 1"), "af_bella");
        // 0 is v0.19's "af" blend, which v1.0 lacks; out-of-range IDs match nothing either.
        assert_eq!(speaker("[voice]\nspeaker_id = 0"), "bf_emma");
        assert_eq!(speaker("[voice]\nspeaker_id = 42"), "bf_emma");
        // A name wins over a leftover ID; an unknown name falls back to the default.
        assert_eq!(speaker("[voice]\nspeaker = \"bm_fable\"\nspeaker_id = 7"), "bm_fable");
        assert_eq!(speaker("[voice]\nspeaker = \"nobody\""), "bf_emma");
        assert_eq!(speaker("[voice]\ntts = true"), "bf_emma");
        // The ID is not written back; the name is.
        let c = Config::parse("[voice]\nspeaker_id = 10").unwrap();
        assert_eq!(c.voice.speaker_id, None);
        let out = toml::to_string(&c).unwrap();
        assert!(out.contains("speaker = \"bm_lewis\"") && !out.contains("speaker_id"), "{out}");
    }
}
