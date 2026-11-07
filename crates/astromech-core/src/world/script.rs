//! `world.jsonl`: its header and steps, loaded (the format is in [`crate::world`]'s doc comment).

use anyhow::{anyhow, bail, Context, Result};
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
    /// Each step with its line number.
    pub steps: Vec<(usize, Step)>,
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
}

/// The step kinds the other children of astromech-vucf add.
const RESERVED: [(&str, &str); 5] = [
    ("focus", "astromech-1hh9"),
    ("press", "astromech-1hh9"),
    ("release", "astromech-1hh9"),
    ("audio", "astromech-1hh9"),
    ("say", "astromech-1hh9"),
];

/// The header fields the devices child adds.
const RESERVED_HEADER: [&str; 3] = ["devices", "reactions", "foreground"];

const KINDS: [&str; 6] = ["journal", "file", "game", "advance", "mark", "net"];

type Fields = BTreeMap<String, Box<RawValue>>;

impl Script {
    /// Load `world.jsonl`'s text. `dir` is the world folder, which `file` steps' `from` names
    /// payloads in. Errors name the line.
    pub fn parse(text: &str, dir: &Path) -> Result<Self> {
        let mut lines = text.lines().enumerate().map(|(i, l)| (i + 1, l)).filter(|(_, l)| !l.trim().is_empty());
        let (n, header) = lines.next().ok_or_else(|| anyhow!("{FILE}: empty; its first line is the header"))?;
        let start = header_start(header).with_context(|| format!("{FILE} line {n} (the header)"))?;
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
        Ok(Self { start, steps })
    }
}

fn fields(line: &str) -> Result<Fields> {
    serde_json::from_str(line).context("not a JSON object")
}

fn header_start(line: &str) -> Result<SystemTime> {
    let f = fields(line)?;
    for k in f.keys() {
        if RESERVED_HEADER.contains(&k.as_str()) {
            bail!("the header field `{k}` is not supported yet (astromech-1hh9)");
        }
        if k != "start" {
            bail!("unknown header field `{k}`; the header is {{\"start\": \"<RFC 3339 time>\"}}");
        }
    }
    let start: String = parse(&f, "start")?.ok_or_else(|| anyhow!("the header needs `start`, an RFC 3339 time"))?;
    rfc3339(&start).with_context(|| format!("start {start:?}"))
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
    if let Some((k, owner)) = RESERVED.iter().find(|(k, _)| f.contains_key(*k)) {
        bail!("the step kind `{k}` is not supported yet ({owner})");
    }
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
    fn the_other_childrens_kinds_are_reserved() {
        for (k, owner) in RESERVED {
            let e = err(&format!("{{\"{k}\": {{}}}}"));
            assert!(e.contains(&format!("the step kind `{k}` is not supported yet ({owner})")), "{e}");
        }
        let e = format!(
            "{:#}",
            Script::parse(r#"{"start": "2026-11-07T12:00:00Z", "devices": []}"#, Path::new(".")).unwrap_err()
        );
        assert!(e.contains("`devices` is not supported yet"), "{e}");
    }
}
