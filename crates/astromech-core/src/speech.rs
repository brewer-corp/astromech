//! Speaking and transcribing. Real implementations use the voice worker process
//! (`astromech-voice-worker`, sherpa-onnx); tests and model-less runs use fakes.

use anyhow::{anyhow, Result};
use astromech_voice::client::Worker;
use astromech_voice::proto::Request;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::time::Duration;

/// Something that says short confirmations. Must not block for long.
pub trait Speaker: Send + Sync {
    fn say(&self, text: &str);
}

/// Prints instead of speaking.
pub struct ConsoleSpeaker;
impl Speaker for ConsoleSpeaker {
    fn say(&self, text: &str) {
        println!("{}  astromech: {text}", chrono::Local::now().format("%H:%M:%S"));
    }
}

/// Records what would have been said (tests).
#[derive(Default)]
pub struct RecordingSpeaker {
    pub said: Mutex<Vec<String>>,
}
impl Speaker for RecordingSpeaker {
    fn say(&self, text: &str) {
        self.said.lock().unwrap().push(text.to_string());
    }
}
impl RecordingSpeaker {
    pub fn take(&self) -> Vec<String> {
        std::mem::take(&mut *self.said.lock().unwrap())
    }
}

/// Speech to text.
pub trait Stt: Send {
    fn transcribe(&mut self, samples: &[f32], rate: u32) -> Result<String>;
}

/// Returns canned transcripts in order (tests).
pub struct FakeStt(pub VecDeque<String>);
impl FakeStt {
    pub fn new(items: &[&str]) -> Self {
        Self(items.iter().map(|s| s.to_string()).collect())
    }
}
impl Stt for FakeStt {
    fn transcribe(&mut self, _samples: &[f32], _rate: u32) -> Result<String> {
        self.0.pop_front().ok_or_else(|| anyhow!("no more canned transcripts"))
    }
}

const CALL: Duration = Duration::from_secs(120);

/// Parakeet TDT through the worker.
pub struct WorkerStt {
    worker: Worker,
    tmp: PathBuf,
}

impl WorkerStt {
    pub fn start(exe: &Path, models_dir: &Path, threads: i32, data_dir: &Path) -> Result<Self> {
        use astromech_voice::models;
        let p = models::stt_paths(&models::model_root(models_dir, &models::STT))?;
        std::fs::create_dir_all(data_dir)?;
        let mut worker = Worker::spawn(exe, None, &data_dir.join("worker-stt.log"), "speech-to-text")?;
        worker.call_ok(&models::load_stt_req(&p, threads), CALL)?;
        Ok(Self { worker, tmp: data_dir.join("ptt.wav") })
    }
}

impl Stt for WorkerStt {
    fn transcribe(&mut self, samples: &[f32], rate: u32) -> Result<String> {
        astromech_voice::wav::write_mono_i16(&self.tmp, rate, samples)?;
        let r = self.worker.call_ok(&Request::Stt { wav: self.tmp.display().to_string() }, Duration::from_secs(20))?;
        Ok(r.text.unwrap_or_default().trim().to_string())
    }
}

/// Kokoro through its own worker, on a background thread with a WAV cache, so `say` returns
/// at once and phrases repeat without re-synthesis. Also prints every line.
pub struct TtsSpeaker {
    tx: Mutex<Sender<String>>,
}

fn cache_name(text: &str) -> String {
    // FNV-1a: stable across runs (std's hasher is randomly seeded).
    let mut h: u64 = 0xcbf29ce484222325;
    for b in text.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}.wav")
}

impl TtsSpeaker {
    pub fn start(
        exe: &Path,
        models_dir: &Path,
        threads: i32,
        data_dir: &Path,
        sid: i32,
        speed: f32,
        prerender: Vec<String>,
    ) -> Result<Self> {
        use astromech_voice::models;
        let p = models::find_tts(models_dir)?;
        let cache = data_dir.join("tts-cache");
        std::fs::create_dir_all(&cache)?;
        let mut worker = Worker::spawn(exe, None, &data_dir.join("worker-tts.log"), "text-to-speech")?;
        worker.call_ok(&models::load_tts_req(&p, threads), CALL)?;
        let (tx, rx) = channel::<String>();
        let pre_tx = tx.clone();
        std::thread::Builder::new().name("tts".into()).spawn(move || {
            let render = |w: &mut Worker, text: &str| -> Option<PathBuf> {
                let f = cache.join(cache_name(text));
                if f.is_file() {
                    return Some(f);
                }
                let req = Request::Tts { text: text.into(), sid, speed, out_wav: f.display().to_string() };
                match w.call_ok(&req, Duration::from_secs(30)) {
                    Ok(_) => Some(f),
                    Err(e) => {
                        eprintln!("text-to-speech failed: {e:#}");
                        None
                    }
                }
            };
            while let Ok(t) = rx.recv() {
                if let Some(text) = t.strip_prefix('\u{0}') {
                    // Pre-render only.
                    render(&mut worker, text);
                    continue;
                }
                if let Some(_f) = render(&mut worker, &t) {
                    #[cfg(windows)]
                    if let Err(e) = astromech_voice::player::play_wav(&_f) {
                        eprintln!("{e:#}");
                    }
                }
            }
        })?;
        for p in prerender {
            let _ = pre_tx.send(format!("\u{0}{p}"));
        }
        Ok(Self { tx: Mutex::new(tx) })
    }
}

impl Speaker for TtsSpeaker {
    fn say(&self, text: &str) {
        ConsoleSpeaker.say(text);
        let _ = self.tx.lock().unwrap().send(text.to_string());
    }
}

/// Every fixed phrase the slice can say (pre-rendered at startup so confirmations are instant).
pub fn fixed_phrases() -> Vec<String> {
    let mut v = vec![];
    for t in astromech_tools::Toggle::all() {
        let s = t.spec();
        for w in [s.on_word, s.off_word] {
            v.push(format!("{} {w}.", s.noun));
        }
        v.push(format!("{} didn't respond.", s.noun));
    }
    for s in [
        "Didn't catch that.",
        "Cancelled.",
        "Already on it.",
        "Kill switch on. All actions stopped.",
        "Re-armed.",
        "Elite isn't the active window, so I didn't press anything.",
    ] {
        v.push(s.into());
    }
    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn cache_names_are_stable() {
        assert_eq!(super::cache_name("Landing gear down."), super::cache_name("Landing gear down."));
        assert_ne!(super::cache_name("a"), super::cache_name("b"));
        assert!(super::fixed_phrases().contains(&"Landing gear down.".to_string()));
    }
}

/// Level statistics of one PTT clip.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClipStats {
    pub audio_ms: u64,
    /// Whole-clip RMS, dBFS.
    pub rms_dbfs: f32,
    /// Loudest 30 ms frame, dBFS (the gate uses this: pre-roll silence doesn't dilute it).
    pub peak_frame_dbfs: f32,
}

fn dbfs(rms: f64) -> f32 {
    if rms <= 1e-9 {
        -120.0
    } else {
        (20.0 * rms.log10()) as f32
    }
}

pub fn clip_stats(samples: &[f32], rate: u32) -> ClipStats {
    let rate = rate.max(1);
    let rms = |s: &[f32]| {
        if s.is_empty() {
            0.0
        } else {
            (s.iter().map(|&x| (x as f64).powi(2)).sum::<f64>() / s.len() as f64).sqrt()
        }
    };
    let frame = (rate as usize * 30 / 1000).max(1);
    let peak = samples.chunks(frame).map(rms).fold(0.0, f64::max);
    ClipStats {
        audio_ms: samples.len() as u64 * 1000 / rate as u64,
        rms_dbfs: dbfs(rms(samples)),
        peak_frame_dbfs: dbfs(peak),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    Pass,
    /// PTT held for less than `min_hold_ms`.
    TooShort,
    /// Loudest frame below `min_level_dbfs`.
    TooQuiet,
}

/// Should this clip go to speech-to-text? Dropped clips are logged, never spoken about.
pub fn gate(held_ms: u64, st: &ClipStats, min_hold_ms: u64, min_level_dbfs: f32) -> Gate {
    if held_ms < min_hold_ms {
        Gate::TooShort
    } else if st.peak_frame_dbfs < min_level_dbfs {
        Gate::TooQuiet
    } else {
        Gate::Pass
    }
}

#[cfg(test)]
mod gate_tests {
    use super::*;

    fn tone(ms: u64, amp: f32) -> Vec<f32> {
        (0..16 * ms).map(|i| amp * (i as f32 * 0.2).sin()).collect()
    }

    #[test]
    fn gates_short_quiet_and_passes_speech() {
        let mut clip = vec![0.0005f32; 16 * 150]; // pre-roll room noise (-66 dBFS)
        clip.extend(tone(600, 0.1)); // speech-level (-23 dBFS RMS)
        let st = clip_stats(&clip, 16000);
        assert_eq!(st.audio_ms, 750);
        assert!(st.peak_frame_dbfs > -25.0 && st.rms_dbfs < st.peak_frame_dbfs);
        assert_eq!(gate(600, &st, 300, -45.0), Gate::Pass);
        // A bounce: ~10 ms held, pre/post-roll only.
        let quiet = clip_stats(&vec![0.0005f32; 16 * 360], 16000);
        assert_eq!(gate(10, &quiet, 300, -45.0), Gate::TooShort);
        assert_eq!(gate(800, &quiet, 300, -45.0), Gate::TooQuiet);
        assert_eq!(clip_stats(&[], 16000).peak_frame_dbfs, -120.0);
    }
}
