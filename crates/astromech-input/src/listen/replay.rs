//! Replay recorded or synthetic samples through a [`Processor`] (tests, and HID testing
//! without hardware).

use crate::listen::model::{DeviceMeta, Event, Sample};
use crate::listen::tracker::Processor;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct ReplayFile {
    #[serde(default)]
    pub devices: Vec<DeviceMeta>,
    pub samples: Vec<Sample>,
    /// Optional focus timeline `[[t_ms, title], ...]`; otherwise derived from the samples.
    #[serde(default)]
    pub focus: Vec<(f64, String)>,
    #[serde(default)]
    pub duration_ms: Option<f64>,
}

pub struct Replayed {
    pub events: Vec<Event>,
    pub timeline: Vec<(f64, String)>,
    pub end_ms: f64,
}

pub fn replay(path: &Path, p: &mut Processor) -> Result<Replayed> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let f: ReplayFile = serde_json::from_str(&text).context("parsing replay JSON")?;
    for d in f.devices {
        p.add_device(d);
    }
    let mut samples = f.samples;
    samples.sort_by(|a, b| a.t_ms.total_cmp(&b.t_ms));
    let mut timeline = f.focus;
    if timeline.is_empty() {
        for s in &samples {
            if timeline.last().map(|l: &(f64, String)| l.1 != s.fg).unwrap_or(true) {
                timeline.push((s.t_ms, s.fg.clone()));
            }
        }
    }
    let mut events = vec![];
    for s in &samples {
        events.extend(p.process(s));
    }
    let end_ms = f.duration_ms.unwrap_or_else(|| samples.last().map(|s| s.t_ms + 1000.0).unwrap_or(0.0));
    Ok(Replayed { events, timeline, end_ms })
}
