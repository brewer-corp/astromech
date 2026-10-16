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
    /// Stop talking now (push-to-talk barge-in). Default: nothing to cut.
    fn interrupt(&self) {}
    /// Speak later lines with this voice (Settings → Voice responses). Default: no voice to change.
    fn set_voice(&self, _voice: &VoiceParams) {}
}

/// What Kokoro renders a line with. Settings can change it while voice runs; the next line uses it.
#[derive(Debug, Clone, PartialEq)]
pub struct VoiceParams {
    /// Kokoro v1.0 voice name, e.g. `bf_emma`.
    pub speaker: String,
    pub sid: i32,
    /// espeak-ng language the text is phonemised with.
    pub lang: &'static str,
    /// The speed sent to Kokoro: the configured one, or the colour's own at 1.0.
    pub speed: f32,
    pub colour: astromech_voice::colour::Colour,
}

impl VoiceParams {
    pub fn new(speaker: &str, speed: f32, colour: astromech_voice::colour::Colour) -> Result<Self> {
        use astromech_voice::{catalog, colour};
        let sid = catalog::voice_sid(speaker).ok_or_else(|| anyhow!("unknown Kokoro voice {speaker:?}"))?;
        Ok(Self {
            speaker: speaker.to_string(),
            sid,
            lang: catalog::voice_lang(speaker),
            speed: colour::speed(colour, speed),
            colour,
        })
    }

    pub fn from_config(v: &crate::config::Voice) -> Result<Self> {
        Self::new(&v.speaker, v.speed, v.colour)
    }
}

/// One voice the Settings picker offers: English Kokoro voices, British first.
#[derive(Debug, Clone, serde::Serialize, PartialEq)]
pub struct VoiceChoice {
    pub name: &'static str,
    /// "Emma"
    pub label: String,
    /// "British" or "American"
    pub accent: &'static str,
    /// "female" or "male"
    pub gender: &'static str,
}

/// The voices Settings offers: British, then American (astromech speaks English only).
pub fn voice_choices() -> Vec<VoiceChoice> {
    let mut v: Vec<VoiceChoice> = astromech_voice::catalog::KOKORO_VOICES
        .iter()
        .filter_map(|&name| {
            let mut c = name.chars();
            let accent = match c.next()? {
                'b' => "British",
                'a' => "American",
                _ => return None,
            };
            let gender = if c.next()? == 'f' { "female" } else { "male" };
            let base = name.get(3..)?;
            let label = base[..1].to_uppercase() + &base[1..];
            Some(VoiceChoice { name, label, accent, gender })
        })
        .collect();
    v.sort_by_key(|c| (c.accent != "British", c.name));
    v
}

/// The line the Settings preview speaks (a fixed phrase, so it's usually cached already).
pub const PREVIEW_LINE: &str = "Landing gear deployed.";

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

/// Renders one line to a WAV file: the Kokoro worker, or a fake in tests.
pub trait Synth: Send + 'static {
    fn tts(&mut self, text: &str, voice: &VoiceParams, out_wav: &Path) -> Result<()>;
}

impl Synth for Worker {
    fn tts(&mut self, text: &str, voice: &VoiceParams, out_wav: &Path) -> Result<()> {
        let req = Request::Tts {
            text: text.into(),
            sid: voice.sid,
            speed: voice.speed,
            out_wav: out_wav.display().to_string(),
            lang: voice.lang.into(),
        };
        self.call_ok(&req, Duration::from_secs(30)).map(|_| ())
    }
}

/// Kokoro through its own worker, on a background thread with a WAV cache, so `say` returns
/// at once and phrases repeat without re-synthesis. Playback goes through [`astromech_voice::mixer`]
/// (coloured, barge-in). Also prints every line. The voice can change while it runs
/// ([`Speaker::set_voice`]); each line is rendered with the voice current when it was said, and
/// the cache is keyed by model, voice, speed and colour.
pub struct TtsSpeaker {
    tx: Mutex<Sender<Job>>,
    mixer: astromech_voice::mixer::Mixer,
    voice: Mutex<VoiceParams>,
    prerender: Vec<String>,
}

enum Job {
    Speak(String, VoiceParams),
    Warm(String, VoiceParams),
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
    #[allow(clippy::too_many_arguments)]
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
        use astromech_voice::models;
        let voice = VoiceParams::new(speaker, speed, colour)?;
        let p = models::find_tts(models_dir)?;
        let model =
            p.model.parent().and_then(|d| d.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let mut worker = Worker::spawn(exe, None, &data_dir.join("worker-tts.log"), "text-to-speech")?;
        worker.call_ok(&models::load_tts_req(&p, threads), CALL)?;
        Self::with_synth(
            worker,
            &model,
            data_dir,
            voice,
            astromech_voice::mixer::Mixer::with_default_output(),
            prerender,
        )
    }

    /// The speaker around any [`Synth`] (tests pass a fake).
    pub fn with_synth(
        mut synth: impl Synth,
        model: &str,
        data_dir: &Path,
        voice: VoiceParams,
        mixer: astromech_voice::mixer::Mixer,
        prerender: Vec<String>,
    ) -> Result<Self> {
        use astromech_voice::colour;
        let model = model.to_string();
        let cache = data_dir.join("tts-cache");
        std::fs::create_dir_all(&cache)?;
        let mix = mixer.clone();
        let (tx, rx) = channel::<Job>();
        std::thread::Builder::new().name("tts".into()).spawn(move || {
            let mut render = |text: &str, v: &VoiceParams| -> Option<(u32, Vec<f32>, PathBuf)> {
                let f = cache.join(cache_name(text, v.colour, &model, &v.speaker, v.speed));
                if f.is_file() {
                    return astromech_voice::wav::read_mono_f32(&f).ok().map(|(sr, s)| (sr, s, f));
                }
                let raw = f.with_extension("raw.wav");
                if let Err(e) = synth.tts(text, v, &raw) {
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
                let out = colour::utterance(&samples, sr, v.colour);
                if let Err(e) = astromech_voice::wav::write_mono_i16(&f, sr, &out) {
                    eprintln!("text-to-speech cache: {e:#}");
                }
                Some((sr, out, f))
            };
            while let Ok(job) = rx.recv() {
                match job {
                    Job::Warm(t, v) => {
                        let _ = render(&t, &v);
                    }
                    Job::Speak(t, v) => {
                        let token = mix.token();
                        if let Some((sr, samples, f)) = render(&t, &v) {
                            if mix.has_device() {
                                mix.play(token, &samples, sr);
                            } else {
                                #[cfg(windows)]
                                if let Err(e) = astromech_voice::player::play_wav_async(&f) {
                                    eprintln!("{e:#}");
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
        for p in &prerender {
            let _ = tx.send(Job::Warm(p.clone(), voice.clone()));
        }
        Ok(Self { tx: Mutex::new(tx), mixer, voice: Mutex::new(voice), prerender })
    }
}

impl Speaker for TtsSpeaker {
    fn say(&self, text: &str) {
        ConsoleSpeaker.say(text);
        let v = self.voice.lock().unwrap().clone();
        let _ = self.tx.lock().unwrap().send(Job::Speak(text.to_string(), v));
    }
    fn interrupt(&self) {
        self.mixer.cut();
        #[cfg(windows)]
        astromech_voice::player::stop();
    }
    fn set_voice(&self, voice: &VoiceParams) {
        *self.voice.lock().unwrap() = voice.clone();
        // Render the fixed phrases in the new voice in the background, as at startup.
        let tx = self.tx.lock().unwrap();
        for p in &self.prerender {
            let _ = tx.send(Job::Warm(p.clone(), voice.clone()));
        }
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
        "Didn't catch that.",
    ] {
        v.push(s.into());
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// Records which voice rendered each line and writes a short tone.
    struct FakeSynth(Arc<Mutex<Vec<(String, String)>>>);
    impl Synth for FakeSynth {
        fn tts(&mut self, text: &str, voice: &VoiceParams, out_wav: &Path) -> Result<()> {
            self.0.lock().unwrap().push((text.to_string(), voice.speaker.clone()));
            let tone: Vec<f32> = (0..2400).map(|i| 0.2 * (i as f32 * 0.1).sin()).collect();
            astromech_voice::wav::write_mono_i16(out_wav, 24000, &tone)
        }
    }

    fn wait_for(log: &Mutex<Vec<(String, String)>>, n: usize) -> Vec<(String, String)> {
        let t0 = std::time::Instant::now();
        while log.lock().unwrap().len() < n && t0.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(5));
        }
        log.lock().unwrap().clone()
    }

    /// astromech-orel: changing the speaker changes the voice of the next reply; a line already
    /// rendered in that voice comes from the cache.
    #[test]
    fn set_voice_changes_the_next_reply() {
        use astromech_voice::colour::Colour;
        let d = tempfile::tempdir().unwrap();
        let log = Arc::new(Mutex::new(vec![]));
        let emma = VoiceParams::new("bf_emma", 1.0, Colour::Cockpit).unwrap();
        let s = TtsSpeaker::with_synth(
            FakeSynth(log.clone()),
            "kokoro-test",
            d.path(),
            emma,
            astromech_voice::mixer::Mixer::silent(24000),
            vec![],
        )
        .unwrap();
        s.say(PREVIEW_LINE);
        assert_eq!(wait_for(&log, 1), [(PREVIEW_LINE.into(), "bf_emma".into())]);
        s.set_voice(&VoiceParams::new("bm_george", 1.0, Colour::Cockpit).unwrap());
        s.say(PREVIEW_LINE);
        s.say(PREVIEW_LINE); // cached in George's voice now: not rendered again
        s.say("Cancelled.");
        let got = wait_for(&log, 3);
        assert_eq!(
            got,
            [
                (PREVIEW_LINE.into(), "bf_emma".into()),
                (PREVIEW_LINE.into(), "bm_george".into()),
                ("Cancelled.".into(), "bm_george".into()),
            ]
        );
    }

    #[test]
    fn voice_choices_are_english_british_first() {
        let v = voice_choices();
        assert_eq!(v.first().map(|c| c.name), Some("bf_alice"));
        let first_american = v.iter().position(|c| c.accent == "American").unwrap();
        assert!(v[..first_american].iter().all(|c| c.accent == "British"));
        assert_eq!(first_american, 8);
        assert!(v.iter().all(|c| c.name.starts_with('a') || c.name.starts_with('b')));
        let george = v.iter().find(|c| c.name == "bm_george").unwrap();
        assert_eq!((george.label.as_str(), george.gender), ("George", "male"));
        assert!(VoiceParams::new("xx_nobody", 1.0, Default::default()).is_err());
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
