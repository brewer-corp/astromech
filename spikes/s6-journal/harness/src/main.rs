//! S6: parse journal lines with ed-journals 0.13.0 and report failures per event.
//! Usage: s6-harness <file-or-dir>... ; also accepts .json arrays of {"line": "..."} (independent samples).
use ed_journals::logs::{LogEvent, LogEventContent};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default)]
struct Tally { ok: u64, failed: u64, unknown_fallback: u64, examples: Vec<String> }

fn check(line: &str, t: &mut BTreeMap<String, Tally>) {
    let line = line.trim().trim_start_matches('\u{feff}');
    if line.is_empty() { return; }
    let v: serde_json::Value = match serde_json::from_str(line) { Ok(v) => v, Err(_) => return };
    let ev = v.get("event").and_then(|e| e.as_str()).unwrap_or("<none>").to_string();
    let e = t.entry(ev.clone()).or_default();
    match serde_json::from_str::<LogEvent>(line) {
        Ok(le) => {
            #[cfg(feature = "unknown")]
            if matches!(le.content, LogEventContent::Unknown(_)) { e.unknown_fallback += 1; return; }
            let _ = &le; let _ = std::any::type_name::<LogEventContent>();
            e.ok += 1
        }
        Err(err) => {
            e.failed += 1;
            if std::env::var_os("S6_VERBOSE").is_some() {
                let ts = v.get("timestamp").and_then(|x| x.as_str()).unwrap_or("?");
                println!("FAIL\t{ts}\t{ev}\t{}", err.to_string().chars().take(140).collect::<String>());
            }
            if e.examples.len() < 2 { e.examples.push(format!("{err}")); }
        }
    }
}

fn walk(p: &Path, t: &mut BTreeMap<String, Tally>, files: &mut u64) {
    if p.is_dir() {
        let mut v: Vec<_> = std::fs::read_dir(p).unwrap().flatten().map(|e| e.path()).collect();
        v.sort();
        for c in v { walk(&c, t, files); }
    } else if p.extension().is_some_and(|e| e == "log") {
        *files += 1;
        let s = String::from_utf8_lossy(&std::fs::read(p).unwrap()).into_owned();
        for l in s.lines() { check(l, t); }
    } else if p.extension().is_some_and(|e| e == "json") {
        let s = std::fs::read_to_string(p).unwrap();
        let arr: Vec<serde_json::Value> = serde_json::from_str(&s).unwrap();
        for o in arr { if let Some(l) = o.get("line").and_then(|l| l.as_str()) { check(l, t); } }
    }
}

fn main() {
    let mut t = BTreeMap::new();
    let mut files = 0;
    for a in std::env::args().skip(1) { walk(Path::new(&a), &mut t, &mut files); }
    let (ok, failed, unk): (u64, u64, u64) = t.values().fold((0, 0, 0), |a, x| (a.0 + x.ok, a.1 + x.failed, a.2 + x.unknown_fallback));
    println!("files={files} events_total={} ok={ok} failed={failed} unknown_fallback={unk} distinct_events={}", ok + failed + unk, t.len());
    for (ev, x) in &t {
        if x.failed > 0 || x.unknown_fallback > 0 {
            println!("  {ev}: ok={} failed={} unknown={} {}", x.ok, x.failed, x.unknown_fallback, x.examples.first().map(|s| s.chars().take(160).collect::<String>()).unwrap_or_default());
        }
    }
}
