//! Offline eval of labelled push-to-talk clips (V1-DESIGN.md §6.3): Debug-saved clips, or the
//! synthetic set `examples/voice-eval.rs` renders from `tests/voice-eval-phrases.tsv`. Scores
//! each clip's transcript against the grammar and reports by command family and recording
//! condition.

use anyhow::{bail, Context, Result};
use astromech_intent::{evaluate, EvalItem, EvalKind, EvalReport};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

/// The per-family target (ARCHITECTURE.md §14 MVP criterion 2, V1-DESIGN.md §6.3).
pub const TARGET: f32 = 0.95;

/// A labelled clip and what the report groups it by.
#[derive(Debug, Clone)]
pub struct Clip {
    pub item: EvalItem,
    /// Report group; the label's prefix ("landing_gear" for "landing_gear.down") unless the
    /// sidecar names one.
    pub family: String,
    /// How it was recorded (the synthetic set's voice and audio chain); empty for real clips.
    pub condition: String,
    /// What was said, when known (the synthetic set's phrase).
    pub said: Option<String>,
}

/// One line of the phrase set: what to say and the command it must give.
#[derive(Debug, Clone, PartialEq)]
pub struct Phrase {
    /// `<command>-<n>`, n counting that command's phrases from 1.
    pub id: String,
    pub family: String,
    pub label: String,
    pub text: String,
}

/// The default family of a label: the command id up to its first dot.
pub fn family_of(label: &str) -> String {
    label.split('.').next().unwrap_or(label).to_string()
}

/// Read a phrase set: `family<TAB>command<TAB>phrase` lines; `#` lines and blank lines skipped.
pub fn load_phrases(path: &Path) -> Result<Vec<Phrase>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut out = vec![];
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').map(str::trim).collect();
        let [family, label, phrase] = cols[..] else {
            bail!("{}:{}: expected family, command and phrase separated by tabs", path.display(), i + 1);
        };
        if family.is_empty() || label.is_empty() || phrase.is_empty() {
            bail!("{}:{}: empty column", path.display(), i + 1);
        }
        let n = seen.entry(label.to_string()).or_default();
        *n += 1;
        out.push(Phrase {
            id: format!("{label}-{n}"),
            family: family.into(),
            label: label.into(),
            text: phrase.into(),
        });
    }
    Ok(out)
}

/// Load labelled clips from a directory: each `*.json` sidecar, plus optional `labels.json`
/// (clip id to label, overriding the sidecar's).
pub fn load_dir(dir: &Path) -> Result<Vec<Clip>> {
    let mut labels: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let lf = dir.join("labels.json");
    if lf.is_file() {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&lf)?)?;
        if let Some(o) = v.as_object() {
            for (k, val) in o {
                if let Some(s) = val.as_str() {
                    labels.insert(k.clone(), s.to_string());
                }
            }
        }
    }
    let mut clips = vec![];
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().and_then(|e| e.to_str()) == Some("json")
                && p.file_name().and_then(|n| n.to_str()) != Some("labels.json")
        })
        .collect();
    files.sort();
    for p in files {
        let text = std::fs::read_to_string(&p)?;
        let v: Value = serde_json::from_str(&text).with_context(|| format!("parse {}", p.display()))?;
        let id = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
        let field = |k: &str| v.get(k).and_then(|x| x.as_str()).map(str::to_string);
        let transcript = field("transcript").unwrap_or_default();
        let label = labels
            .get(&id)
            .cloned()
            .or_else(|| field("label"))
            .or_else(|| field("command"))
            .unwrap_or_else(|| "nomatch".into());
        if transcript.is_empty() {
            continue;
        }
        clips.push(Clip {
            family: field("family").unwrap_or_else(|| family_of(&label)),
            condition: field("condition").unwrap_or_default(),
            said: field("text"),
            item: EvalItem { id, transcript, label },
        });
    }
    Ok(clips)
}

/// Clips and their scores (`report.rows[i]` is `clips[i]`).
pub struct Run {
    pub clips: Vec<Clip>,
    pub report: EvalReport,
}

/// Correct and wrong-execute counts for one group of clips.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    pub clips: usize,
    /// Executed or asked the labelled command, or rejected a `nomatch` clip.
    pub correct: usize,
    pub wrong_execute: usize,
    pub miss: usize,
    pub wrong_ask: usize,
}

impl Score {
    pub fn rate(&self) -> f32 {
        if self.clips == 0 {
            0.0
        } else {
            self.correct as f32 / self.clips as f32
        }
    }
    fn add(&mut self, kind: EvalKind) {
        self.clips += 1;
        match kind {
            EvalKind::CorrectExecuteOrAsk | EvalKind::CorrectReject => self.correct += 1,
            EvalKind::WrongExecute => self.wrong_execute += 1,
            EvalKind::Miss => self.miss += 1,
            EvalKind::WrongAsk => self.wrong_ask += 1,
        }
    }
}

fn pct(x: f32) -> String {
    format!("{:.1}%", 100.0 * x)
}

impl Run {
    /// Scores grouped by `key`, in key order.
    pub fn by(&self, key: impl Fn(&Clip) -> String) -> BTreeMap<String, Score> {
        let mut m: BTreeMap<String, Score> = BTreeMap::new();
        for (c, row) in self.clips.iter().zip(&self.report.rows) {
            m.entry(key(c)).or_default().add(row.kind);
        }
        m
    }

    /// The groups scoring below [`TARGET`].
    pub fn below_target(&self, key: impl Fn(&Clip) -> String) -> Vec<(String, Score)> {
        self.by(key).into_iter().filter(|(_, s)| s.rate() < TARGET).collect()
    }

    /// A Markdown table of scores by `key`, flagging groups below the target.
    pub fn table(&self, heading: &str, key: impl Fn(&Clip) -> String) -> String {
        let mut s = format!("| {heading} | Clips | Correct | Rate | Wrong execute | Miss | Wrong ask | |\n");
        s += "|---|---:|---:|---:|---:|---:|---:|---|\n";
        for (k, sc) in self.by(key) {
            let flag = if sc.rate() < TARGET { "below 95%" } else { "" };
            let _ = writeln!(
                s,
                "| {k} | {} | {} | {} | {} | {} | {} | {flag} |",
                sc.clips,
                sc.correct,
                pct(sc.rate()),
                sc.wrong_execute,
                sc.miss,
                sc.wrong_ask
            );
        }
        s
    }

    /// The full report: summary, family, condition and command tables, then every clip that
    /// was not scored correct.
    pub fn markdown(&self) -> String {
        let r = &self.report;
        let mut s = format!(
            "{}\n\nCorrect means executed or asked the expected command (or said nothing for a `nomatch` \
             clip). Target: at least {:.0}% per family, wrong executes near 0.\n\n",
            r.summary(),
            100.0 * TARGET
        );
        s += "## By family\n\n";
        s += &self.table("Family", |c| c.family.clone());
        if self.clips.iter().any(|c| !c.condition.is_empty()) {
            s += "\n## By condition\n\n";
            s += &self.table("Condition", |c| c.condition.clone());
            s += "\n## By family and condition\n\n";
            s += &self.table("Family / condition", |c| format!("{} / {}", c.family, c.condition));
        }
        s += "\n## By command\n\n";
        s += &self.table("Command", |c| c.item.label.clone());
        s += "\n## Not correct\n\n";
        let wrong: Vec<_> = self
            .clips
            .iter()
            .zip(&r.rows)
            .filter(|(_, row)| !matches!(row.kind, EvalKind::CorrectExecuteOrAsk | EvalKind::CorrectReject))
            .collect();
        if wrong.is_empty() {
            s += "None.\n";
        } else {
            s += "| Clip | Said | Heard | Expected | Got | Decision | Kind |\n|---|---|---|---|---|---|---|\n";
            for (c, row) in wrong {
                let _ = writeln!(
                    s,
                    "| {} | {} | {} | {} | {} | {:?} | {:?} |",
                    row.id,
                    c.said.as_deref().unwrap_or(""),
                    row.transcript,
                    row.label,
                    if row.got.is_empty() { "-" } else { &row.got },
                    row.decision,
                    row.kind
                );
            }
        }
        s
    }
}

/// Score clips against the app's grammar.
pub fn score(clips: Vec<Clip>) -> Run {
    let items: Vec<EvalItem> = clips.iter().map(|c| c.item.clone()).collect();
    let report = evaluate(&crate::assistant::grammar(), &items);
    Run { clips, report }
}

pub fn run(dir: &Path) -> Result<Run> {
    Ok(score(load_dir(dir)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_sidecars_and_labels() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("a.json"), r#"{"transcript":"Gear down.","command":"landing_gear.down"}"#)
            .unwrap();
        std::fs::write(d.path().join("b.json"), r#"{"transcript":"banana"}"#).unwrap();
        std::fs::write(d.path().join("labels.json"), r#"{"b":"nomatch"}"#).unwrap();
        let r = run(d.path()).unwrap();
        assert_eq!(r.report.total, 2);
        assert_eq!(r.report.correct_execute_or_ask, 1);
        assert_eq!(r.report.correct_reject, 1);
        assert_eq!(r.clips[0].family, "landing_gear");
        assert_eq!(r.clips[1].family, "nomatch");
    }

    fn clip(id: &str, transcript: &str, label: &str, family: &str, condition: &str) -> Clip {
        Clip {
            item: EvalItem { id: id.into(), transcript: transcript.into(), label: label.into() },
            family: family.into(),
            condition: condition.into(),
            said: Some(transcript.into()),
        }
    }

    #[test]
    fn groups_by_family_and_condition_and_lists_misses() {
        let run = score(vec![
            clip("a", "gear down", "landing_gear.down", "ship", "clean"),
            clip("b", "lights on", "lights.on", "ship", "noise"),
            // A deliberately wrong expectation: the grammar hears lights off, not lights on.
            clip("c", "lights off", "lights.on", "ship", "noise"),
            clip("d", "next waypoint", "route.next", "route", "clean"),
        ]);
        let fam = run.by(|c| c.family.clone());
        assert_eq!(fam["ship"], Score { clips: 3, correct: 2, wrong_execute: 1, miss: 0, wrong_ask: 0 });
        assert_eq!(fam["route"].rate(), 1.0);
        let cond = run.by(|c| c.condition.clone());
        assert_eq!((cond["clean"].correct, cond["noise"].correct), (2, 1));
        assert_eq!(
            run.below_target(|c| c.family.clone()).iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>(),
            ["ship"]
        );
        let md = run.markdown();
        assert!(md.contains("| ship | 3 | 2 | 66.7% | 1 | 0 | 0 | below 95% |"), "{md}");
        assert!(md.contains("| route | 1 | 1 | 100.0% |"), "{md}");
        assert!(
            md.contains("| c | lights off | lights off | lights.on | lights.off | Execute | WrongExecute |"),
            "{md}"
        );
        assert!(!md.contains("| a |"), "correct clips are not listed: {md}");
    }

    #[test]
    fn phrase_ids_count_per_command() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("p.tsv");
        std::fs::write(&p, "# comment\nship\tlights.on\tlights on\n\nship\tlights.on\tturn on lights\n").unwrap();
        let v = load_phrases(&p).unwrap();
        assert_eq!(v.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), ["lights.on-1", "lights.on-2"]);
        std::fs::write(&p, "ship lights.on lights on\n").unwrap();
        assert!(load_phrases(&p).is_err(), "spaces are not tabs");
    }
}
