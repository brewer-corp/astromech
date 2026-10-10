//! Microphone capture with cpal (WASAPI shared mode, default input device).
//! Keeps the last 60 s of mono audio in a ring buffer; the S9 mic test cuts the PTT window
//! out of it. Audio never leaves this PC; recordings are saved as WAV next to the exe.

use anyhow::{anyhow, Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

struct Ring {
    buf: VecDeque<f32>,
    /// Total samples ever pushed; buf holds [total - buf.len(), total).
    total: u64,
    last_cb: Option<Instant>,
}

pub struct Recorder {
    ring: Arc<(Mutex<Ring>, Condvar)>,
    _stream: cpal::Stream,
    rate: u32,
    name: String,
    pub channels: u16,
    pub format: String,
}

fn push(ring: &Arc<(Mutex<Ring>, Condvar)>, cap: usize, ch: usize, data: impl Iterator<Item = f32>) {
    let (m, cv) = &**ring;
    let mut r = m.lock().unwrap();
    let mut acc = 0f32;
    let mut k = 0usize;
    for s in data {
        acc += s;
        k += 1;
        if k == ch {
            r.buf.push_back(acc / ch as f32);
            r.total += 1;
            acc = 0.0;
            k = 0;
        }
    }
    while r.buf.len() > cap {
        r.buf.pop_front();
    }
    r.last_cb = Some(Instant::now());
    cv.notify_all();
}

impl Recorder {
    pub fn default_device_name() -> Option<String> {
        let host = cpal::default_host();
        let d = host.default_input_device()?;
        d.description().ok().map(|x| x.name().to_string())
    }

    pub fn start() -> Result<Self> {
        let host = cpal::default_host();
        let dev = host.default_input_device().ok_or_else(|| anyhow!("no default microphone (input device) found"))?;
        let name = dev.description().map(|d| d.name().to_string()).unwrap_or_else(|_| "default input".into());
        let sup = dev.default_input_config().context("query microphone format")?;
        let rate = sup.sample_rate();
        let channels = sup.channels();
        let format = format!("{:?}", sup.sample_format());
        let config: cpal::StreamConfig = sup.config();
        let ring = Arc::new((Mutex::new(Ring { buf: VecDeque::new(), total: 0, last_cb: None }), Condvar::new()));
        let cap = rate as usize * 60;
        let ch = channels.max(1) as usize;
        let err = |e| eprintln!("microphone stream error: {e}");
        let r2 = ring.clone();
        let stream = match sup.sample_format() {
            cpal::SampleFormat::F32 => dev.build_input_stream::<f32, _, _>(
                config,
                move |d: &[f32], _| push(&r2, cap, ch, d.iter().copied()),
                err,
                None,
            ),
            cpal::SampleFormat::I16 => dev.build_input_stream::<i16, _, _>(
                config,
                move |d: &[i16], _| push(&r2, cap, ch, d.iter().map(|&v| v as f32 / 32768.0)),
                err,
                None,
            ),
            cpal::SampleFormat::I32 => dev.build_input_stream::<i32, _, _>(
                config,
                move |d: &[i32], _| push(&r2, cap, ch, d.iter().map(|&v| v as f32 / 2147483648.0)),
                err,
                None,
            ),
            cpal::SampleFormat::U16 => dev.build_input_stream::<u16, _, _>(
                config,
                move |d: &[u16], _| push(&r2, cap, ch, d.iter().map(|&v| (v as f32 - 32768.0) / 32768.0)),
                err,
                None,
            ),
            f => return Err(anyhow!("unsupported microphone sample format {f:?}")),
        }
        .context("open microphone stream")?;
        stream.play().context("start microphone stream")?;
        Ok(Recorder { ring, _stream: stream, rate, name, channels, format })
    }
}

impl Recorder {
    pub fn sample_rate(&self) -> u32 {
        self.rate
    }
    pub fn device_name(&self) -> String {
        self.name.clone()
    }
    pub fn position(&self) -> u64 {
        self.ring.0.lock().unwrap().total
    }
    pub fn wait_after(&self, t: Instant, max: Duration) {
        let (m, cv) = &*self.ring;
        let end = Instant::now() + max;
        let mut r = m.lock().unwrap();
        while r.last_cb.map(|c| c <= t).unwrap_or(true) {
            let now = Instant::now();
            if now >= end {
                return;
            }
            r = cv.wait_timeout(r, end - now).unwrap().0;
        }
    }
    pub fn take(&self, from: u64, to: u64) -> Vec<f32> {
        let r = self.ring.0.lock().unwrap();
        let start = r.total - r.buf.len() as u64;
        let from = from.max(start);
        let to = to.min(r.total);
        if to <= from {
            return vec![];
        }
        r.buf.range((from - start) as usize..(to - start) as usize).copied().collect()
    }
}
