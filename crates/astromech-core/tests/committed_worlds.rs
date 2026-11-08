//! What a committed world (`worlds/**`) may hold (finding astromech-9rru): record mode redacts keys
//! and commander names, but a world recorded against the real EDSM needs a real key in its
//! `secrets.json`, and a recorded flight log is the commander's dated travel history. So:
//!
//! - every value in a `secrets.json` is a placeholder: it says so (`not-a-real…`, `fake`,
//!   `placeholder`), or is empty;
//! - no `net.json` holds an EDSM get-logs answer with any logs in it (a recorded body, an `in_turn`
//!   answer or a `from` file). A get-logs answer that is a fault, an error or an empty log is fine.
#![allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)

use serde_json::Value;
use std::path::{Path, PathBuf};

/// Why `dir` (a folder of worlds) holds something a commit mustn't; empty if nothing.
fn problems(dir: &Path) -> Vec<String> {
    let mut out = vec![];
    for f in files(dir) {
        let name = f.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        let shown = f.strip_prefix(dir).unwrap_or(&f).display().to_string().replace('\\', "/");
        match name {
            "secrets.json" => out.extend(secrets(&f).into_iter().map(|p| format!("{shown}: {p}"))),
            "net.json" => out.extend(flight_logs(&f).into_iter().map(|p| format!("{shown}: {p}"))),
            _ => {}
        }
    }
    out
}

fn files(dir: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    let mut todo = vec![dir.to_path_buf()];
    while let Some(d) = todo.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                todo.push(p);
            } else {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn placeholder(v: &str) -> bool {
    let v = v.to_ascii_lowercase();
    v.is_empty() || ["not-a-real", "fake", "placeholder"].iter().any(|w| v.contains(w))
}

fn secrets(f: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(f).unwrap();
    let Ok(Value::Object(o)) = serde_json::from_str::<Value>(&text) else {
        return vec!["not a JSON object of names and values".into()];
    };
    o.iter()
        .filter(|(_, v)| !v.as_str().is_some_and(placeholder))
        .map(|(k, _)| format!("{k:?} isn't a placeholder (write e.g. \"not-a-real-key\")"))
        .collect()
}

fn flight_logs(f: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(f).unwrap();
    let Ok(rec) = serde_json::from_str::<Value>(&text) else { return vec!["not JSON".into()] };
    let empty = vec![];
    let exchanges = rec["exchanges"].as_array().unwrap_or(&empty);
    let mut out = vec![];
    for (i, x) in exchanges.iter().enumerate() {
        if !x["url"].as_str().is_some_and(|u| u.contains("get-logs")) {
            continue;
        }
        let answers: Vec<&Value> = match x["in_turn"].as_array() {
            Some(a) => a.iter().collect(),
            None => vec![x],
        };
        for a in answers {
            let body = match (&a["json"], a["text"].as_str(), a["from"].as_str()) {
                (Value::Null, Some(t), _) => serde_json::from_str(t).unwrap_or(Value::Null),
                (Value::Null, None, Some(from)) => std::fs::read_to_string(f.with_file_name(from))
                    .ok()
                    .and_then(|t| serde_json::from_str(&t).ok())
                    .unwrap_or(Value::Null),
                (j, _, _) => j.clone(),
            };
            if body["logs"].as_array().is_some_and(|l| !l.is_empty()) {
                out.push(format!("exchange {i} is an EDSM flight log with entries (a commander's travel history)"));
            }
        }
    }
    out
}

#[test]
fn committed_worlds_hold_no_real_key_and_no_flight_log() {
    let worlds = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../worlds");
    assert!(worlds.join("every-command/secrets.json").is_file(), "the worlds are at {}", worlds.display());
    let p = problems(&worlds);
    assert!(p.is_empty(), "committed worlds hold what they mustn't:\n{}", p.join("\n"));
}

/// The check fails on each thing it is for.
#[test]
fn the_check_finds_a_real_key_and_a_flight_log() {
    let d = tempfile::tempdir().unwrap();
    let w = d.path().join("w");
    std::fs::create_dir_all(&w).unwrap();
    std::fs::write(w.join("secrets.json"), r#"{"edsm-api-key": "not-a-real-key", "other": "0123456789abcdef"}"#)
        .unwrap();
    std::fs::write(w.join("logs.json"), r#"{"msgnum":100,"logs":[{"system":"Sol","date":"2026-11-01 10:00:00"}]}"#)
        .unwrap();
    std::fs::write(
        w.join("net.json"),
        r#"{"exchanges": [
            {"url": "https://www.edsm.net/api-logs-v1/get-logs", "status": 503, "text": "<html>down</html>"},
            {"url": "https://www.edsm.net/api-logs-v1/get-logs?commanderName=<redacted>", "json": {"msgnum": 100, "logs": []}},
            {"url": "https://www.edsm.net/api-logs-v1/get-logs", "json": {"msgnum": 100, "logs": [{"system": "Sol"}]}},
            {"url": "https://www.edsm.net/api-logs-v1/get-logs", "in_turn": [{"fault": "timeout"}, {"text": "{\"logs\": [{\"system\": \"Sol\"}]}"}]},
            {"url": "https://www.edsm.net/api-logs-v1/get-logs", "from": "logs.json"},
            {"url": "https://www.spansh.co.uk/api/x", "json": {"logs": [1]}}
        ]}"#,
    )
    .unwrap();
    assert_eq!(
        problems(d.path()),
        vec![
            "w/net.json: exchange 2 is an EDSM flight log with entries (a commander's travel history)",
            "w/net.json: exchange 3 is an EDSM flight log with entries (a commander's travel history)",
            "w/net.json: exchange 4 is an EDSM flight log with entries (a commander's travel history)",
            "w/secrets.json: \"other\" isn't a placeholder (write e.g. \"not-a-real-key\")",
        ]
    );
}
