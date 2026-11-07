//! `world.jsonl`: its header and steps, loaded (the format is in [`crate::world`]'s doc comment).

use anyhow::{anyhow, bail, Context, Result};
use astromech_input::listen::model::{DeviceMeta, SampleData};
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// The steps file in a world.
pub const FILE: &str = "world.jsonl";

/// A loaded `world.jsonl`.
#[derive(Debug)]
pub struct Script {
    /// The header's `start`: the clock's UTC time before any step.
    pub start: SystemTime,
    /// The header's `devices`: the controllers the Raw Input listener would find.
    pub devices: Vec<DeviceMeta>,
    /// The header's `reactions`: what the game does when a key goes down.
    pub reactions: Vec<Reaction>,
    /// The header's `foreground`: the window in front before any `focus` step.
    pub foreground: Option<String>,
    /// Each step with its line number.
    pub steps: Vec<(usize, Step)>,
}

/// What the game does when a key goes down: the mock game's key rules
/// (`{"scan": 38, "ext": 0, "flag": 4}`, `"gui": 6`, `"vehicle": true`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction {
    pub scan: u16,
    pub ext: bool,
    pub effect: Effect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    /// Toggle this Status flag.
    Flag(u64),
    /// Set GuiFocus.
    Gui(u64),
    /// Swap the in-ship and in-SRV flags, closing any panel.
    Vehicle,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct KeyRule {
    scan: u16,
    #[serde(default)]
    ext: u8,
    flag: Option<u64>,
    gui: Option<u64>,
    #[serde(default)]
    vehicle: bool,
}

/// A `press` or `release` step's raw-input sample, a listen::model `Sample` without its time:
/// that is the world clock's.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub dev: String,
    /// The window in front as the listener saw it; missing: the world's foreground window.
    #[serde(default)]
    pub fg: Option<String>,
    #[serde(default)]
    pub injected: bool,
    pub data: SampleData,
}

impl PartialEq for Input {
    fn eq(&self, o: &Self) -> bool {
        serde_json::to_value(self).ok() == serde_json::to_value(o).ok()
    }
}

/// What a `game` step makes the process list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Game {
    Running,
    Launcher,
    Exited,
}

#[derive(Debug, PartialEq)]
pub enum Step {
    /// Append `line` to the journal `file` (None: the current one). `stamp`: the line's
    /// `timestamp`, if it has one that parses.
    Journal {
        file: Option<String>,
        line: String,
        stamp: Option<SystemTime>,
    },
    /// Write the companion file `name` (in the game folder).
    File {
        name: String,
        body: Vec<u8>,
        partial: bool,
    },
    Game(Game),
    Advance(Duration),
    Mark(String),
    /// Add an exchange to the world's network recording.
    Net(astromech_data::replay::Exchange),
    /// The window in front from now on (None: none).
    Focus(Option<String>),
    /// A raw-input sample with a button down.
    Press(Input),
    /// A raw-input sample with it up again.
    Release(Input),
    /// A clip on the microphone (mono samples at [`super::io::MIC_RATE`] Hz).
    Audio(Vec<f32>),
    /// What speech-to-text hears for the next clip.
    Say(String),
}

const KINDS: [&str; 11] =
    ["journal", "file", "game", "advance", "mark", "net", "focus", "press", "release", "audio", "say"];

/// The header's fields.
const HEADER: [&str; 4] = ["start", "devices", "reactions", "foreground"];

type Fields = BTreeMap<String, Box<RawValue>>;

impl Script {
    /// Load `world.jsonl`'s text. `dir` is the world folder, which `file` steps' `from` names
    /// payloads in. Errors name the line.
    pub fn parse(text: &str, dir: &Path) -> Result<Self> {
        let mut lines = text.lines().enumerate().map(|(i, l)| (i + 1, l)).filter(|(_, l)| !l.trim().is_empty());
        let (n, header) = lines.next().ok_or_else(|| anyhow!("{FILE}: empty; its first line is the header"))?;
        let h = header_fields(header).with_context(|| format!("{FILE} line {n} (the header)"))?;
        let mut steps = vec![];
        let mut marks = std::collections::HashSet::new();
        for (n, line) in lines {
            let step = step(line, dir).with_context(|| format!("{FILE} line {n}"))?;
            if let Step::Mark(m) = &step {
                if !marks.insert(m.clone()) {
                    bail!("{FILE} line {n}: the mark {m:?} is already used");
                }
            }
            steps.push((n, step));
        }
        Ok(Self { start: h.start, devices: h.devices, reactions: h.reactions, foreground: h.foreground, steps })
    }
}

struct Header {
    start: SystemTime,
    devices: Vec<DeviceMeta>,
    reactions: Vec<Reaction>,
    foreground: Option<String>,
}

fn fields(line: &str) -> Result<Fields> {
    serde_json::from_str(line).context("not a JSON object")
}

fn header_fields(line: &str) -> Result<Header> {
    let f = fields(line)?;
    if let Some(k) = f.keys().find(|k| !HEADER.contains(&k.as_str())) {
        bail!("unknown header field `{k}`; the header has `start` and optionally {}", HEADER[1..].join(", "));
    }
    let start: String = parse(&f, "start")?.ok_or_else(|| anyhow!("the header needs `start`, an RFC 3339 time"))?;
    let start = rfc3339(&start).with_context(|| format!("start {start:?}"))?;
    let rules: Vec<KeyRule> = parse(&f, "reactions")?.unwrap_or_default();
    let mut reactions = vec![];
    for (i, r) in rules.into_iter().enumerate() {
        // The mock game's order: the vehicle swap (which closes any panel), the flag, GuiFocus.
        let effects = [r.vehicle.then_some(Effect::Vehicle), r.flag.map(Effect::Flag), r.gui.map(Effect::Gui)];
        let effects: Vec<Effect> = effects.into_iter().flatten().collect();
        if effects.is_empty() || r.ext > 1 {
            bail!("reaction {i}: give `scan`, `ext` 0 or 1, and one or more of `flag`, `gui` and `vehicle`");
        }
        reactions.extend(effects.into_iter().map(|effect| Reaction { scan: r.scan, ext: r.ext == 1, effect }));
    }
    Ok(Header {
        start,
        devices: parse(&f, "devices")?.unwrap_or_default(),
        reactions,
        foreground: parse::<Option<String>>(&f, "foreground")?.flatten(),
    })
}

fn rfc3339(t: &str) -> Result<SystemTime> {
    Ok(chrono::DateTime::parse_from_rfc3339(t).context("not an RFC 3339 time")?.to_utc().into())
}

/// Field `k` of `f`, as `T`, if it is there.
fn parse<T: serde::de::DeserializeOwned>(f: &Fields, k: &str) -> Result<Option<T>> {
    f.get(k).map(|v| serde_json::from_str(v.get()).with_context(|| format!("`{k}`"))).transpose()
}

fn step(line: &str, dir: &Path) -> Result<Step> {
    let f = fields(line)?;
    let kinds: Vec<&str> = f.keys().map(String::as_str).filter(|k| KINDS.contains(k)).collect();
    let kind = match kinds.as_slice() {
        [k] => *k,
        [] => bail!("not a step: expected one of {}", KINDS.join(", ")),
        // `file` is also the journal step's file name.
        ["file", "journal"] => "journal",
        many => bail!("more than one step kind: {}", many.join(", ")),
    };
    let allowed: &[&str] = match kind {
        "journal" => &["journal", "file"],
        "file" => &["file", "json", "text", "from", "partial"],
        k => &[k],
    };
    if let Some(k) = f.keys().find(|k| !allowed.contains(&k.as_str())) {
        bail!("unknown field `{k}` in a `{kind}` step");
    }
    Ok(match kind {
        "journal" => {
            let raw = f["journal"].get();
            // A string is the line itself (any text, even a malformed one); an object is
            // written as it appears here.
            let line = match serde_json::from_str::<String>(raw) {
                Ok(s) => s,
                Err(_) if raw.starts_with('{') => raw.to_string(),
                Err(_) => bail!("`journal` is the line: a JSON object, or a string"),
            };
            if line.contains('\n') {
                bail!("a journal line can't hold a line break");
            }
            let file: Option<String> = parse(&f, "file")?;
            if let Some(name) = &file {
                plain_name(name)?;
                if !name.starts_with("Journal.") || !name.ends_with(".log") {
                    bail!("{name:?} isn't a journal file name (Journal.<stamp>.<part>.log)");
                }
            }
            let stamp = serde_json::from_str::<serde_json::Value>(&line)
                .ok()
                .and_then(|v| v.get("timestamp")?.as_str().map(String::from))
                .and_then(|t| rfc3339(&t).ok());
            Step::Journal { file, line, stamp }
        }
        "file" => {
            let name: String = parse(&f, "file")?.context("`file` is the file's name")?;
            plain_name(&name)?;
            if name.starts_with("Journal.") {
                bail!("{name:?} is a journal: use a `journal` step");
            }
            let body = match (f.get("json"), parse::<String>(&f, "text")?, parse::<String>(&f, "from")?) {
                (Some(j), None, None) => j.get().as_bytes().to_vec(),
                (None, Some(t), None) => t.into_bytes(),
                (None, None, Some(p)) => {
                    std::fs::read(dir.join(&p)).with_context(|| format!("reading the payload {p}"))?
                }
                _ => bail!("a `file` step has one of `json`, `text` or `from`"),
            };
            Step::File { name, body, partial: parse(&f, "partial")?.unwrap_or(false) }
        }
        "game" => Step::Game(match parse::<String>(&f, "game")?.as_deref() {
            Some("running") => Game::Running,
            Some("launcher") => Game::Launcher,
            Some("exited") => Game::Exited,
            _ => bail!("`game` is \"running\", \"launcher\" or \"exited\""),
        }),
        "advance" => {
            let d: String = parse(&f, "advance")?.context("`advance`")?;
            Step::Advance(duration(&d).with_context(|| format!("advance {d:?}"))?)
        }
        "mark" => Step::Mark(parse::<String>(&f, "mark")?.context("`mark` is its name")?),
        "net" => Step::Net(
            astromech_data::replay::Exchange::parse(f["net"].get(), dir)
                .map_err(|e| anyhow!("`net` is one exchange in net.json's form: {e}"))?,
        ),
        "focus" => {
            Step::Focus(parse::<Option<String>>(&f, "focus").context("`focus` is a window title, or null")?.flatten())
        }
        "press" => Step::Press(parse(&f, "press")?.context("`press`")?),
        "release" => Step::Release(parse(&f, "release")?.context("`release`")?),
        "audio" => {
            let p: String = parse(&f, "audio")?.context("`audio` is a WAV payload")?;
            let (rate, samples) = astromech_voice::wav::read_mono_f32(&dir.join(&p))
                .with_context(|| format!("reading the WAV payload {p}"))?;
            if rate != super::io::MIC_RATE {
                bail!(
                    "{p} is at {rate} Hz: record or convert it at {} Hz, the world microphone's rate",
                    super::io::MIC_RATE
                );
            }
            Step::Audio(samples)
        }
        "say" => Step::Say(parse::<String>(&f, "say")?.context("`say` is the transcript")?),
        _ => unreachable!("one of KINDS"),
    })
}

/// A file name in the game folder: no folders.
fn plain_name(name: &str) -> Result<()> {
    let p = Path::new(name);
    if name.is_empty() || p.file_name().map(|f| f != p.as_os_str()).unwrap_or(true) {
        bail!("{name:?}: give a file name in the game folder, with no folder");
    }
    Ok(())
}

/// `"250ms"`, `"3s"`, `"1.5s"`, `"2m"`, `"2h"`.
fn duration(s: &str) -> Result<Duration> {
    let s = s.trim();
    let at =
        s.find(|c: char| !(c.is_ascii_digit() || c == '.')).ok_or_else(|| anyhow!("give a unit: ms, s, m or h"))?;
    let (n, unit) = s.split_at(at);
    let n: f64 = n.parse().context("not a number")?;
    let secs = match unit.trim() {
        "ms" => n / 1000.0,
        "s" => n,
        "m" => n * 60.0,
        "h" => n * 3600.0,
        u => bail!("unknown unit {u:?}: ms, s, m or h"),
    };
    Duration::try_from_secs_f64(secs).context("not a duration")
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;

    const HEADER: &str = r#"{"start": "2026-11-07T12:00:00Z"}"#;

    fn load(steps: &str) -> Result<Script> {
        Script::parse(&format!("{HEADER}\n{steps}"), Path::new("/nowhere"))
    }

    fn err(steps: &str) -> String {
        format!("{:#}", load(steps).unwrap_err())
    }

    #[test]
    fn the_header_and_each_step_kind_load() {
        let s = load(concat!(
            r#"{"game": "running"}"#,
            "\n\n",
            r#"{"journal": {"timestamp":"2026-11-07T12:00:05Z", "event":"FSDJump"}, "file": "Journal.2026-11-07T120000.01.log"}"#,
            "\n",
            r#"{"journal": "not json"}"#,
            "\n",
            r#"{"file": "Status.json", "json": {"Flags": 4}}"#,
            "\n",
            r#"{"file": "Status.json", "text": "{\"Fl", "partial": true}"#,
            "\n",
            r#"{"advance": "1.5s"}"#,
            "\n",
            r#"{"advance": "250ms"}"#,
            "\n",
            r#"{"mark": "jumped"}"#,
            "\n",
            r#"{"game": "exited"}"#,
            "\n",
            r#"{"net": {"url": "/api/systems/field_values/system_names", "status": 503}}"#,
        ))
        .unwrap();
        assert_eq!(s.start, rfc3339("2026-11-07T12:00:00Z").unwrap());
        let lines: Vec<usize> = s.steps.iter().map(|(n, _)| *n).collect();
        assert_eq!(lines, [2, 4, 5, 6, 7, 8, 9, 10, 11, 12], "blank lines are skipped, the numbers kept");
        let steps: Vec<Step> = s.steps.into_iter().map(|(_, s)| s).collect();
        assert_eq!(steps[0], Step::Game(Game::Running));
        assert_eq!(
            steps[1],
            Step::Journal {
                file: Some("Journal.2026-11-07T120000.01.log".into()),
                line: r#"{"timestamp":"2026-11-07T12:00:05Z", "event":"FSDJump"}"#.into(),
                stamp: Some(rfc3339("2026-11-07T12:00:05Z").unwrap()),
            },
            "the line is verbatim, as written in the step"
        );
        assert_eq!(steps[2], Step::Journal { file: None, line: "not json".into(), stamp: None });
        assert_eq!(
            steps[3],
            Step::File { name: "Status.json".into(), body: br#"{"Flags": 4}"#.to_vec(), partial: false }
        );
        assert_eq!(steps[4], Step::File { name: "Status.json".into(), body: br#"{"Fl"#.to_vec(), partial: true });
        assert_eq!(steps[5], Step::Advance(Duration::from_millis(1500)));
        assert_eq!(steps[6], Step::Advance(Duration::from_millis(250)));
        assert_eq!(steps[7], Step::Mark("jumped".into()));
        assert_eq!(steps[8], Step::Game(Game::Exited));
        let net = r#"{"url": "/api/systems/field_values/system_names", "status": 503}"#;
        assert_eq!(steps[9], Step::Net(astromech_data::replay::Exchange::parse(net, Path::new("/nowhere")).unwrap()));
    }

    #[test]
    fn a_file_step_reads_its_payload_from_the_world() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("market.json"), "{}").unwrap();
        let s = Script::parse(&format!("{HEADER}\n{{\"file\": \"Market.json\", \"from\": \"market.json\"}}"), d.path())
            .unwrap();
        assert_eq!(s.steps[0].1, Step::File { name: "Market.json".into(), body: b"{}".to_vec(), partial: false });
    }

    #[test]
    fn a_malformed_line_fails_the_load_with_its_line_number() {
        assert!(err("{\"game\": \"running\"}\nnot json").starts_with("world.jsonl line 3: not a JSON object"));
        assert!(err(r#"{"game": "paused"}"#).starts_with("world.jsonl line 2: `game` is"));
        assert!(err(r#"{"advance": "3"}"#).contains("line 2: advance \"3\": give a unit"));
        assert!(err(r#"{"advance": "3 days"}"#).contains("unknown unit"));
        assert!(err(r#"{"wait": "3s"}"#).contains("line 2: not a step"));
        assert!(err(r#"{"game": "running", "mark": "x"}"#).contains("more than one step kind"));
        assert!(err(r#"{"mark": "x", "colour": 1}"#).contains("unknown field `colour` in a `mark` step"));
        assert!(err(r#"{"file": "Status.json"}"#).contains("one of `json`, `text` or `from`"));
        assert!(err(r#"{"file": "../Status.json", "text": ""}"#).contains("with no folder"));
        assert!(err(r#"{"file": "Journal.2026-11-07T120000.01.log", "text": ""}"#).contains("use a `journal` step"));
        assert!(err(r#"{"journal": {}, "file": "Status.json"}"#).contains("isn't a journal file name"));
        assert!(err(r#"{"journal": 3}"#).contains("`journal` is the line"));
        assert!(err(r#"{"journal": "a\nb"}"#).contains("line break"));
        assert!(err(r#"{"net": {"status": 503}}"#)
            .contains("line 2: `net` is one exchange in net.json's form: an exchange needs `url`"));
        assert!(err("{\"mark\": \"x\"}\n{\"mark\": \"x\"}").contains("line 3: the mark \"x\" is already used"));
        let e = format!("{:#}", Script::parse("", Path::new(".")).unwrap_err());
        assert!(e.contains("empty"), "{e}");
        let e = format!("{:#}", Script::parse(r#"{"start": "noon"}"#, Path::new(".")).unwrap_err());
        assert!(e.starts_with("world.jsonl line 1 (the header): start \"noon\""), "{e}");
        let e = format!("{:#}", Script::parse(r#"{"game": "running"}"#, Path::new(".")).unwrap_err());
        assert!(e.contains("unknown header field `game`"), "{e}");
    }

    #[test]
    fn the_devices_header_and_steps_load() {
        let d = tempfile::tempdir().unwrap();
        astromech_voice::wav::write_mono_i16(&d.path().join("clip.wav"), 16_000, &[0.0, 0.5, -0.5]).unwrap();
        astromech_voice::wav::write_mono_i16(&d.path().join("fast.wav"), 24_000, &[0.0]).unwrap();
        let header = r#"{"start": "2026-11-07T12:00:00Z", "foreground": "Elite - Dangerous (CLIENT)",
            "devices": [{"key": "hid:vkb-l", "kind": "hid", "vid": 8989, "pid": 12801, "product": "VKB"}],
            "reactions": [{"scan": 38, "ext": 0, "flag": 4}, {"scan": 66, "gui": 6, "flag": 1}, {"scan": 64, "ext": 1, "vehicle": true}]}"#
            .replace('\n', "");
        let steps = [
            r#"{"focus": "Notepad"}"#,
            r#"{"focus": null}"#,
            r#"{"press": {"dev": "hid:vkb-l", "data": {"type": "hid", "buttons": [4]}}}"#,
            r#"{"release": {"dev": "hid:vkb-l", "fg": "Notepad", "data": {"type": "hid", "buttons": []}}}"#,
            r#"{"audio": "clip.wav"}"#,
            r#"{"say": "gear down"}"#,
        ];
        let s = Script::parse(&format!("{header}\n{}", steps.join("\n")), d.path()).unwrap();
        assert_eq!(s.foreground.as_deref(), Some("Elite - Dangerous (CLIENT)"));
        assert_eq!(s.devices[0].elite_id().as_deref(), Some("231D3201"));
        assert_eq!(
            s.reactions,
            [
                Reaction { scan: 38, ext: false, effect: Effect::Flag(4) },
                Reaction { scan: 66, ext: false, effect: Effect::Flag(1) },
                Reaction { scan: 66, ext: false, effect: Effect::Gui(6) },
                Reaction { scan: 64, ext: true, effect: Effect::Vehicle },
            ]
        );
        let steps: Vec<Step> = s.steps.into_iter().map(|(_, s)| s).collect();
        assert_eq!(steps[0], Step::Focus(Some("Notepad".into())));
        assert_eq!(steps[1], Step::Focus(None));
        let Step::Press(p) = &steps[2] else { panic!("{:?}", steps[2]) };
        assert_eq!((p.dev.as_str(), p.fg.as_deref()), ("hid:vkb-l", None));
        assert!(matches!(&p.data, SampleData::Hid { buttons, .. } if buttons == &[4]));
        let Step::Release(r) = &steps[3] else { panic!("{:?}", steps[3]) };
        assert_eq!(r.fg.as_deref(), Some("Notepad"));
        let Step::Audio(a) = &steps[4] else { panic!("{:?}", steps[4]) };
        assert_eq!(a.len(), 3);
        assert_eq!(steps[5], Step::Say("gear down".into()));

        let bad = |step: &str| format!("{:#}", Script::parse(&format!("{HEADER}\n{step}"), d.path()).unwrap_err());
        assert!(bad(r#"{"audio": "fast.wav"}"#).contains("at 24000 Hz: record or convert it at 16000 Hz"));
        assert!(bad(r#"{"press": {"t_ms": 5, "dev": "x", "data": {"type": "hid"}}}"#).contains("unknown field `t_ms`"));
        let bad_header = |h: &str| format!("{:#}", Script::parse(h, d.path()).unwrap_err());
        assert!(bad_header(r#"{"start": "2026-11-07T12:00:00Z", "reactions": [{"scan": 38}]}"#)
            .contains("reaction 0: give `scan`"));
        assert!(bad_header(r#"{"start": "2026-11-07T12:00:00Z", "mic": "x"}"#).contains("unknown header field `mic`"));
    }
}
