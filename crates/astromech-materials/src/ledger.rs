//! Fold journal material events into a count per material (ARCHITECTURE.md §18.5).

use crate::catalog::{self, Kind, Spec};
use astromech_journal::{JournalEvent, JournalLine, MaterialStack};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Ledger {
    pub counts: BTreeMap<String, i64>,
    pub names: BTreeMap<String, String>,
    pub snapshot_ts: Option<String>,
}

impl Ledger {
    pub fn apply(&mut self, ts: &str, ev: &JournalEvent) {
        match ev {
            JournalEvent::Materials { raw, manufactured, encoded } => {
                self.counts.clear();
                self.names.clear();
                self.snapshot_ts = Some(ts.to_string());
                for it in raw.iter().chain(manufactured).chain(encoded) {
                    self.set(it, it.count);
                }
            }
            JournalEvent::MaterialDelta { items }
            | JournalEvent::EngineerCraft { ingredients: items, preview: false, .. } => {
                for it in items {
                    self.add(it, it.count);
                }
            }
            JournalEvent::Extra(astromech_journal::ExtraEvent::MissionCompleted { materials_reward, .. }) => {
                for it in materials_reward {
                    self.add(it, it.count);
                }
            }
            _ => {}
        }
    }

    fn id_of(&self, it: &MaterialStack) -> String {
        catalog::lookup(&it.name).map(|s| s.id.to_string()).unwrap_or_else(|| catalog::key(&it.name))
    }

    fn remember_name(&mut self, id: &str, it: &MaterialStack) {
        if let Some(n) = it.name_localised.as_deref().filter(|s| !s.is_empty()) {
            self.names.insert(id.to_string(), n.to_string());
        } else if let Some(s) = catalog::lookup(&it.name) {
            self.names.entry(id.to_string()).or_insert_with(|| s.name.to_string());
        } else if !self.names.contains_key(id) {
            self.names.insert(id.to_string(), it.name.clone());
        }
    }

    fn set(&mut self, it: &MaterialStack, count: i64) {
        if it.name.is_empty() {
            return;
        }
        let id = self.id_of(it);
        self.remember_name(&id, it);
        self.counts.insert(id, count.max(0));
    }

    fn add(&mut self, it: &MaterialStack, delta: i64) {
        if it.name.is_empty() || delta == 0 {
            return;
        }
        let id = self.id_of(it);
        self.remember_name(&id, it);
        let n = self.counts.entry(id).or_insert(0);
        *n = n.saturating_add(delta).max(0);
    }

    pub fn count(&self, name: &str) -> i64 {
        let id = catalog::lookup(name).map(|s| s.id.to_string()).unwrap_or_else(|| catalog::key(name));
        self.counts.get(&id).copied().unwrap_or(0)
    }

    pub fn display_name(&self, spec: &Spec) -> String {
        self.names.get(spec.id).cloned().unwrap_or_else(|| spec.name.to_string())
    }

    pub fn unknown(&self) -> Vec<(String, String, i64)> {
        self.counts
            .iter()
            .filter(|(id, n)| **n > 0 && catalog::lookup(id).is_none())
            .map(|(id, n)| (id.clone(), self.names.get(id).cloned().unwrap_or_else(|| id.clone()), *n))
            .collect()
    }

    pub fn kind_totals(&self, kind: Kind) -> (i64, i64) {
        let mut held = 0;
        let mut cap = 0;
        for s in catalog::catalog().iter().filter(|s| s.kind == kind && s.on_trader) {
            held += self.count(s.id);
            cap += catalog::capacity(s.grade);
        }
        (held, cap)
    }
}

/// Apply lines in order.
pub fn replay_lines<'a>(lines: impl IntoIterator<Item = &'a str>) -> Ledger {
    let mut l = Ledger::default();
    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if let Ok(j) = astromech_journal::parse_line(t) {
            l.apply(&j.timestamp, &j.event);
        }
    }
    l
}

/// NDJSON journal, or a JSON array of event objects.
pub fn replay_bytes(bytes: &[u8]) -> Ledger {
    let s = String::from_utf8_lossy(bytes);
    let t = s.trim_start();
    if t.starts_with('[') {
        if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(t) {
            let mut l = Ledger::default();
            for v in arr {
                let ts = v.get("timestamp").and_then(|x| x.as_str()).unwrap_or("").to_string();
                if let Ok(line) = serde_json::to_string(&v) {
                    if let Ok(j) = astromech_journal::parse_line(&line) {
                        l.apply(&ts, &j.event);
                    }
                }
            }
            return l;
        }
    }
    replay_lines(s.lines())
}

pub fn replay_path(path: &Path) -> Ledger {
    std::fs::read(path).map(|b| replay_bytes(&b)).unwrap_or_default()
}

/// From the last `Materials` snapshot in the folder, then later deltas. Older files after that
/// snapshot are ignored.
pub fn replay_journal_dir(dir: &Path) -> Ledger {
    replay_events(&events_from_last_snapshot(dir))
}

pub fn events_from_last_snapshot(dir: &Path) -> Vec<JournalLine> {
    let files = astromech_journal::journal_logs(dir);
    let mut suffix: Vec<Vec<JournalLine>> = vec![];
    for f in files.iter().rev() {
        let ev = astromech_journal::parse_file(f);
        if let Some(i) = ev.iter().rposition(|l| matches!(l.event, JournalEvent::Materials { .. })) {
            suffix.push(ev[i..].to_vec());
            break;
        }
        suffix.push(ev);
    }
    suffix.reverse();
    suffix.into_iter().flatten().collect()
}

pub fn replay_events(events: &[JournalLine]) -> Ledger {
    let mut l = Ledger::default();
    for e in events {
        l.apply(&e.timestamp, &e.event);
    }
    l
}
