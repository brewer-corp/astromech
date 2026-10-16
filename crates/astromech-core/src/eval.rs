//! Offline eval of Debug-saved PTT clips (V1-DESIGN.md §6.3).

use anyhow::{Context, Result};
use astromech_intent::{evaluate, EvalItem, EvalReport};
use serde_json::Value;
use std::path::Path;

/// Load labelled items from a clips directory: each `*.json` sidecar, plus optional `labels.json`.
pub fn load_dir(dir: &Path) -> Result<Vec<EvalItem>> {
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
    let mut items = vec![];
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
        let transcript = v.get("transcript").and_then(|x| x.as_str()).unwrap_or("").to_string();
        let label = labels
            .get(&id)
            .cloned()
            .or_else(|| v.get("label").and_then(|x| x.as_str()).map(str::to_string))
            .or_else(|| v.get("command").and_then(|x| x.as_str()).map(str::to_string))
            .unwrap_or_else(|| "nomatch".into());
        if transcript.is_empty() {
            continue;
        }
        items.push(EvalItem { id, transcript, label });
    }
    Ok(items)
}

pub fn run(dir: &Path) -> Result<EvalReport> {
    let items = load_dir(dir)?;
    let g = crate::assistant::grammar();
    Ok(evaluate(&g, &items))
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
        assert_eq!(r.total, 2);
        assert_eq!(r.correct_execute_or_ask, 1);
        assert_eq!(r.correct_reject, 1);
    }
}
