//! Speaking and transcribing. Real implementations use the voice worker process
//! (`astromech-voice-worker`, sherpa-onnx); tests and model-less runs use fakes.

use anyhow::{anyhow, Result};
use astromech_voice::client::Worker;
use astromech_voice::proto::Request;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Something that says short confirmations. Must not block for long.
pub trait Speaker: Send + Sync {
    fn say(&self, text: &str);
    /// Stop talking now (push-to-talk barge-in). Default: nothing to cut.
    fn interrupt(&self) {}
    /// Still rendering or playing a line (callouts wait for it). Default: never.
    fn busy(&self) -> bool {
        false
    }
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
/// at once and phrases repeat without re-synthesis. Playback goes through [`astromech_voice::mixer`]
/// (coloured, barge-in). Also prints every line.
pub struct TtsSpeaker {
    tx: Mutex<Sender<Job>>,
    mixer: astromech_voice::mixer::Mixer,
    activity: Arc<Activity>,
    stopped_logged: AtomicBool,
}

/// What a speaker is doing, for [`Speaker::busy`]: lines handed over and not yet playing, and
/// when the line playing without the mixer (Windows `PlaySoundW`, no output device) ends.
#[derive(Default)]
pub struct Activity {
    rendering: AtomicUsize,
    ends: Mutex<Option<Instant>>,
    /// The thread that renders lines has stopped (it panicked); lines are printed only.
    stopped: AtomicBool,
}

/// One line in progress; finished when dropped, including when rendering panics.
pub struct Line(Arc<Activity>);

impl Drop for Line {
    fn drop(&mut self) {
        self.0.rendering.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Marks the activity stopped when the rendering thread ends (dropped on panic too).
pub struct Alive(Arc<Activity>);

impl Drop for Alive {
    fn drop(&mut self) {
        self.0.stopped.store(true, Ordering::SeqCst);
    }
}

impl Activity {
    /// A line was handed over. Returns None (and counts nothing) once the renderer stopped.
    pub fn begin(self: &Arc<Self>) -> Option<Line> {
        if self.stopped.load(Ordering::SeqCst) {
            return None;
        }
        self.rendering.fetch_add(1, Ordering::SeqCst);
        Some(Line(self.clone()))
    }

    /// Guard for the rendering thread's lifetime.
    pub fn worker(self: &Arc<Self>) -> Alive {
        Alive(self.clone())
    }

    /// A line of `len` started playing outside the mixer at `now`.
    pub fn playing(&self, now: Instant, len: Duration) {
        *self.ends.lock().unwrap() = Some(now + len);
    }

    /// Playback was cut.
    pub fn cut(&self) {
        *self.ends.lock().unwrap() = None;
    }

    /// `mixer_queued`: samples still in the mixer.
    pub fn busy(&self, now: Instant, mixer_queued: usize) -> bool {
        self.rendering.load(Ordering::SeqCst) > 0
            || mixer_queued > 0
            || self.ends.lock().unwrap().is_some_and(|t| now < t)
    }
}

enum Job {
    Speak(String, Line),
    Warm(String),
}

/// Cache file for one rendered line. `model` names the model folder, so a model update
/// (Kokoro v0.19 to v1.0, fp32 to int8) never plays audio rendered by the old one.
fn cache_name(text: &str, colour: astromech_voice::colour::Colour, model: &str, speaker: &str, speed: f32) -> String {
    // FNV-1a: stable across runs (std's hasher is randomly seeded).
    let mut h: u64 = 0xcbf29ce484222325;
    let key = format!("{text}\0{}\0{model}\0{speaker}\0{speed:.3}", colour.as_str());
    for b in key.as_bytes() {
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
        speaker: &str,
        speed: f32,
        colour: astromech_voice::colour::Colour,
        prerender: Vec<String>,
    ) -> Result<Self> {
        use astromech_voice::{catalog, colour, models};
        let sid = catalog::voice_sid(speaker).ok_or_else(|| anyhow!("unknown Kokoro voice {speaker:?}"))?;
        let lang = catalog::voice_lang(speaker);
        let speaker = speaker.to_string();
        let p = models::find_tts(models_dir)?;
        let model =
            p.model.parent().and_then(|d| d.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let cache = data_dir.join("tts-cache");
        std::fs::create_dir_all(&cache)?;
        let mut worker = Worker::spawn(exe, None, &data_dir.join("worker-tts.log"), "text-to-speech")?;
        worker.call_ok(&models::load_tts_req(&p, threads), CALL)?;
        let speed = colour::speed(colour, speed);
        let mixer = astromech_voice::mixer::Mixer::with_default_output();
        let mix = mixer.clone();
        let (tx, rx) = channel::<Job>();
        let pre_tx = tx.clone();
        let activity = Arc::new(Activity::default());
        let act = activity.clone();
        std::thread::Builder::new().name("tts".into()).spawn(move || {
            let _alive = act.worker();
            let render = |w: &mut Worker, text: &str| -> Option<(u32, Vec<f32>, PathBuf)> {
                // Reading rules (names, numbers, units); the cache key follows the spoken text.
                let text = &astromech_voice::speakable::normalise(text);
                let f = cache.join(cache_name(text, colour, &model, &speaker, speed));
                if f.is_file() {
                    return astromech_voice::wav::read_mono_f32(&f).ok().map(|(sr, s)| (sr, s, f));
                }
                let raw = f.with_extension("raw.wav");
                let req = Request::Tts {
                    text: text.into(),
                    sid,
                    speed,
                    out_wav: raw.display().to_string(),
                    lang: lang.into(),
                };
                if let Err(e) = w.call_ok(&req, Duration::from_secs(30)) {
                    eprintln!("text-to-speech failed: {e:#}");
                    let _ = std::fs::remove_file(&raw);
                    return None;
                }
                let (sr, samples) = match astromech_voice::wav::read_mono_f32(&raw) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("text-to-speech wav: {e:#}");
                        let _ = std::fs::remove_file(&raw);
                        return None;
                    }
                };
                let _ = std::fs::remove_file(&raw);
                let out = colour::utterance(&samples, sr, colour);
                if let Err(e) = astromech_voice::wav::write_mono_i16(&f, sr, &out) {
                    eprintln!("text-to-speech cache: {e:#}");
                }
                Some((sr, out, f))
            };
            while let Ok(job) = rx.recv() {
                match job {
                    Job::Warm(t) => {
                        let _ = render(&mut worker, &t);
                    }
                    Job::Speak(t, _line) => {
                        // `_line` ends this line when the arm is left, also by a panic.
                        let token = mix.token();
                        if let Some((sr, samples, f)) = render(&mut worker, &t) {
                            if mix.has_device() {
                                mix.play(token, &samples, sr);
                            } else if token == mix.token() {
                                #[cfg(windows)]
                                match astromech_voice::player::play_wav_async(&f) {
                                    Ok(()) => act.playing(
                                        Instant::now(),
                                        Duration::from_secs_f64(samples.len() as f64 / sr.max(1) as f64),
                                    ),
                                    Err(e) => eprintln!("{e:#}"),
                                }
                                #[cfg(not(windows))]
                                {
                                    let _ = (sr, samples, f);
                                }
                            }
                        }
                    }
                }
            }
        })?;
        for p in prerender {
            let _ = pre_tx.send(Job::Warm(p));
        }
        Ok(Self { tx: Mutex::new(tx), mixer, activity, stopped_logged: AtomicBool::new(false) })
    }
}

impl Speaker for TtsSpeaker {
    fn say(&self, text: &str) {
        ConsoleSpeaker.say(text);
        // A send to a stopped thread hands the line back, and dropping it ends it.
        let sent = match self.activity.begin() {
            Some(line) => self.tx.lock().unwrap().send(Job::Speak(text.to_string(), line)).is_ok(),
            None => false,
        };
        if !sent && !self.stopped_logged.swap(true, Ordering::SeqCst) {
            crate::logging::log("text-to-speech stopped (see worker-tts.log); replies are printed only");
        }
    }
    fn busy(&self) -> bool {
        self.activity.busy(Instant::now(), self.mixer.queued())
    }
    fn interrupt(&self) {
        self.mixer.cut();
        self.activity.cut();
        #[cfg(windows)]
        astromech_voice::player::stop();
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
        v.push(format!("{} did not respond.", s.noun));
    }
    for s in [
        "Say again.",
        "Cancelled.",
        "Already on it.",
        "Kill switch on. All actions stopped.",
        "Re-armed.",
        "Elite is not the active window. Nothing pressed.",
        "Silent running produces heat. Confirm?",
        "Silent running on.",
        "Flight assist on.",
        "Flight assist off.",
        "HUD analysis mode.",
        "HUD combat mode.",
        "Galaxy map open.",
        "System map open.",
        "Panel closed.",
        "Throttle zero.",
        "No route plotted.",
        "Say again.",
    ] {
        v.push(s.into());
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn busy_while_a_line_renders_or_plays_without_a_device() {
        let a = Arc::new(Activity::default());
        let t0 = Instant::now();
        assert!(!a.busy(t0, 0));
        let line = a.begin().unwrap();
        assert!(a.busy(t0, 0));
        drop(line);
        assert!(!a.busy(t0, 0));
        assert!(a.busy(t0, 10), "samples left in the mixer");
        // No output device: PlaySoundW plays a 1.5 s line; busy until it ends, or until cut.
        a.playing(t0, Duration::from_millis(1500));
        assert!(a.busy(t0 + Duration::from_millis(1400), 0));
        assert!(!a.busy(t0 + Duration::from_millis(1500), 0));
        a.playing(t0, Duration::from_millis(1500));
        a.cut();
        assert!(!a.busy(t0, 0));
    }

    #[test]
    fn a_panicking_renderer_does_not_leave_the_speaker_busy() {
        let a = Arc::new(Activity::default());
        let (tx, rx) = channel::<Line>();
        tx.send(a.begin().unwrap()).unwrap();
        tx.send(a.begin().unwrap()).unwrap(); // still queued when the thread dies
        let act = a.clone();
        let t = std::thread::spawn(move || {
            let _alive = act.worker();
            let _line = rx.recv().unwrap();
            panic!("render failed");
        });
        assert!(t.join().is_err());
        assert!(!a.busy(Instant::now(), 0));
        // Later lines are not counted (they are printed only).
        assert!(a.begin().is_none());
        assert!(!a.busy(Instant::now(), 0));
        drop(tx);
    }

    #[test]
    fn cache_names_are_stable() {
        use astromech_voice::colour::Colour;
        let name =
            |colour, model, speaker, speed| super::cache_name("Landing gear deployed.", colour, model, speaker, speed);
        let v1 = "kokoro-multi-lang-v1_0";
        assert_eq!(name(Colour::Cockpit, v1, "bf_emma", 0.94), name(Colour::Cockpit, v1, "bf_emma", 0.94));
        assert_ne!(name(Colour::Cockpit, v1, "bf_emma", 0.94), name(Colour::Natural, v1, "bf_emma", 1.0));
        assert_ne!(name(Colour::Cockpit, v1, "bf_emma", 0.94), name(Colour::Cockpit, v1, "bm_george", 0.94));
        assert_ne!(
            name(Colour::Cockpit, v1, "bf_emma", 0.94),
            name(Colour::Cockpit, "kokoro-int8-multi-lang-v1_0", "bf_emma", 0.94)
        );
        assert!(super::fixed_phrases().contains(&"Landing gear deployed.".to_string()));
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
