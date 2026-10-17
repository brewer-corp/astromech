//! Speaking and transcribing. Real implementations use the voice worker process
//! (`astromech-voice-worker`, sherpa-onnx); tests and model-less runs use fakes.

use anyhow::{anyhow, Result};
use astromech_voice::client::Worker;
use astromech_voice::proto::Request;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// Something that says short confirmations. Must not block for long.
pub trait Speaker: Send + Sync {
    fn say(&self, text: &str);
    /// Stop talking now (push-to-talk barge-in). Default: nothing to cut.
    fn interrupt(&self) {}
    /// Speak later lines with this voice (Settings → Voice responses). Default: no voice to change.
    fn set_voice(&self, _voice: &VoiceParams) {}
    /// Does `say` make a sound (Kokoro), or only print? Default: only prints.
    fn speaks_aloud(&self) -> bool {
        false
    }
    /// Still rendering or playing a line (callouts wait for it). Default: never.
    fn busy(&self) -> bool {
        false
    }
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

/// Settings' voice preview while no session runs: a Kokoro started on the first preview, reused
/// for the next ones, and released after [`PREVIEW_IDLE`] without a line (it holds the model in
/// memory) or when a session starts (it has its own).
#[derive(Default)]
pub struct PreviewSlot {
    inner: Mutex<Option<(Arc<dyn Speaker>, std::time::Instant)>>,
}

/// How long an unused preview Kokoro stays loaded.
pub const PREVIEW_IDLE: Duration = Duration::from_secs(60);

impl PreviewSlot {
    /// Say `line`. Under the slot's lock: `route` may answer instead (an error, or speaking through
    /// a running session); otherwise the preview speaker is started if needed with `start`.
    pub fn say(
        &self,
        line: &str,
        route: impl FnOnce() -> Option<Result<()>>,
        start: impl FnOnce() -> Result<Arc<dyn Speaker>>,
    ) -> Result<()> {
        let mut g = self.inner.lock().unwrap();
        if let Some(r) = route() {
            return r;
        }
        let sp = match g.take() {
            Some((sp, _)) => sp,
            None => start()?,
        };
        if !sp.speaks_aloud() {
            return Err(anyhow!("speech output is off"));
        }
        sp.say(line);
        *g = Some((sp, std::time::Instant::now()));
        Ok(())
    }

    pub fn set_voice(&self, v: &VoiceParams) {
        if let Some((sp, _)) = self.inner.lock().unwrap().as_ref() {
            sp.set_voice(v);
        }
    }

    /// Release the preview speaker; true if there was one.
    pub fn take(&self) -> bool {
        self.inner.lock().unwrap().take().is_some()
    }

    /// Release it if nothing was said for `idle`; true if released.
    pub fn release_idle(&self, now: std::time::Instant, idle: Duration) -> bool {
        let mut g = self.inner.lock().unwrap();
        if g.as_ref().is_some_and(|(_, t)| now.saturating_duration_since(*t) >= idle) {
            *g = None;
            return true;
        }
        false
    }

    pub fn loaded(&self) -> bool {
        self.inner.lock().unwrap().is_some()
    }
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
    queue: Arc<(Mutex<Queue>, Condvar)>,
    mixer: astromech_voice::mixer::Mixer,
    /// The current voice, shared with the render thread so it can skip stale warm-ups.
    voice: Arc<Mutex<VoiceParams>>,
    prerender: Vec<String>,
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

/// Lines waiting to render. Replies (`speak`) always go before warm-ups, so a voice change's
/// background re-render of the fixed phrases never delays the next reply or a preview.
#[derive(Default)]
struct Queue {
    /// Each reply carries its [`Line`], so it counts as busy until played (or dropped).
    speak: VecDeque<(String, VoiceParams, Line)>,
    warm: VecDeque<(String, VoiceParams)>,
    closed: bool,
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
        let queue: Arc<(Mutex<Queue>, Condvar)> = Arc::default();
        let current = Arc::new(Mutex::new(voice));
        let (q, cur) = (queue.clone(), current.clone());
        let activity = Arc::new(Activity::default());
        let act = activity.clone();
        std::thread::Builder::new().name("tts".into()).spawn(move || {
            // If this thread ends (a panic included), the speaker stops counting lines and the
            // replies still queued are dropped, so busy() can't stay true.
            let _alive = act.worker();
            let _drain = DrainOnExit(q.clone());
            let mut render = |text: &str, v: &VoiceParams| -> Option<(u32, Vec<f32>, PathBuf)> {
                // Reading rules (names, numbers, units); the cache key follows the spoken text.
                let text = &astromech_voice::speakable::normalise(text);
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
            loop {
                let job = {
                    let (m, cv) = &*q;
                    let mut g = m.lock().unwrap();
                    loop {
                        if let Some((t, v, line)) = g.speak.pop_front() {
                            break Some((Some(line), (t, v)));
                        }
                        if let Some(j) = g.warm.pop_front() {
                            break Some((None, j));
                        }
                        if g.closed {
                            break None;
                        }
                        g = cv.wait(g).unwrap();
                    }
                };
                let Some((line, (t, v))) = job else { break };
                match line {
                    None => {
                        // A warm-up for a voice that's no longer selected would never be played.
                        if *cur.lock().unwrap() == v {
                            let _ = render(&t, &v);
                        }
                    }
                    Some(_line) => {
                        // `_line` ends this reply when the arm is left, also by a panic.
                        let token = mix.token();
                        if let Some((sr, samples, f)) = render(&t, &v) {
                            if mix.has_device() {
                                mix.play(token, &samples, sr);
                            } else if token == mix.token() {
                                #[cfg(windows)]
                                match astromech_voice::player::play_wav_async(&f) {
                                    // PlaySoundW gives no end signal: busy for the line's length.
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
        let s = Self { queue, mixer, voice: current, prerender, activity, stopped_logged: AtomicBool::new(false) };
        s.warm_up();
        Ok(s)
    }

    fn push(&self, f: impl FnOnce(&mut Queue)) {
        let (m, cv) = &*self.queue;
        // A render thread that panicked may have poisoned the lock; the queue is still sound.
        f(&mut m.lock().unwrap_or_else(|e| e.into_inner()));
        cv.notify_one();
    }

    /// Render the fixed phrases in the current voice, in the background.
    fn warm_up(&self) {
        let v = self.voice.lock().unwrap().clone();
        self.push(|q| q.warm.extend(self.prerender.iter().map(|p| (p.clone(), v.clone()))));
    }
}

/// Drops the replies still queued when the render thread ends (their [`Line`]s end with them).
struct DrainOnExit(Arc<(Mutex<Queue>, Condvar)>);

impl Drop for DrainOnExit {
    fn drop(&mut self) {
        self.0 .0.lock().unwrap_or_else(|e| e.into_inner()).speak.clear();
    }
}

impl Drop for TtsSpeaker {
    fn drop(&mut self) {
        self.push(|q| q.closed = true);
    }
}

impl Speaker for TtsSpeaker {
    fn say(&self, text: &str) {
        ConsoleSpeaker.say(text);
        let Some(line) = self.activity.begin() else {
            if !self.stopped_logged.swap(true, Ordering::SeqCst) {
                crate::logging::log("text-to-speech stopped (see worker-tts.log); replies are printed only");
            }
            return;
        };
        let v = self.voice.lock().unwrap().clone();
        self.push(|q| q.speak.push_back((text.to_string(), v, line)));
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
    fn set_voice(&self, voice: &VoiceParams) {
        *self.voice.lock().unwrap() = voice.clone();
        // Render the fixed phrases in the new voice in the background, as at startup; earlier
        // voices' warm-ups still queued are dropped.
        self.push(|q| q.warm.retain(|(_, v)| v == voice));
        self.warm_up();
    }
    fn speaks_aloud(&self) -> bool {
        true
    }
}

/// Starts a speaker on the first line said once `ready` holds, for a session that began before
/// the text-to-speech model finished downloading (astromech-mcdt). Until then lines are printed.
/// The start runs in the background; lines said meanwhile are queued and passed on in order, and
/// a voice chosen meanwhile is applied first. If the start fails, lines are printed from then on.
pub struct LazySpeaker {
    ready: Box<dyn Fn() -> bool + Send + Sync>,
    start: Arc<dyn Fn() -> Result<Arc<dyn Speaker>> + Send + Sync>,
    state: Arc<Mutex<Lazy>>,
}

enum Lazy {
    Waiting { voice: Option<VoiceParams> },
    Starting { lines: Vec<String>, voice: Option<VoiceParams> },
    Running(Arc<dyn Speaker>),
    Failed,
}

impl LazySpeaker {
    pub fn new(
        ready: impl Fn() -> bool + Send + Sync + 'static,
        start: impl Fn() -> Result<Arc<dyn Speaker>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            ready: Box::new(ready),
            start: Arc::new(start),
            state: Arc::new(Mutex::new(Lazy::Waiting { voice: None })),
        }
    }
}

impl Speaker for LazySpeaker {
    fn say(&self, text: &str) {
        let mut g = self.state.lock().unwrap();
        match &mut *g {
            Lazy::Running(s) => s.say(text),
            Lazy::Starting { lines, .. } => lines.push(text.to_string()),
            Lazy::Failed => ConsoleSpeaker.say(text),
            Lazy::Waiting { .. } if !(self.ready)() => ConsoleSpeaker.say(text),
            Lazy::Waiting { voice } => {
                *g = Lazy::Starting { lines: vec![text.to_string()], voice: voice.take() };
                let (state, start) = (self.state.clone(), self.start.clone());
                let r = std::thread::Builder::new().name("tts-start".into()).spawn(move || {
                    let r = start();
                    let mut g = state.lock().unwrap();
                    let Lazy::Starting { lines, voice } = std::mem::replace(&mut *g, Lazy::Failed) else {
                        return;
                    };
                    let s: Arc<dyn Speaker> = match r {
                        Ok(s) => {
                            if let Some(v) = &voice {
                                s.set_voice(v);
                            }
                            *g = Lazy::Running(s.clone());
                            s
                        }
                        Err(_) => Arc::new(ConsoleSpeaker),
                    };
                    for l in lines {
                        s.say(&l);
                    }
                });
                if let Err(e) = r {
                    eprintln!("text-to-speech start: {e}");
                    *g = Lazy::Failed;
                    ConsoleSpeaker.say(text);
                }
            }
        }
    }
    fn interrupt(&self) {
        match &mut *self.state.lock().unwrap() {
            Lazy::Running(s) => s.interrupt(),
            Lazy::Starting { lines, .. } => lines.clear(),
            _ => {}
        }
    }
    fn set_voice(&self, v: &VoiceParams) {
        match &mut *self.state.lock().unwrap() {
            Lazy::Running(s) => s.set_voice(v),
            Lazy::Starting { voice, .. } | Lazy::Waiting { voice } => *voice = Some(v.clone()),
            Lazy::Failed => {}
        }
    }
    fn speaks_aloud(&self) -> bool {
        match &*self.state.lock().unwrap() {
            Lazy::Running(s) => s.speaks_aloud(),
            Lazy::Starting { .. } => true,
            Lazy::Waiting { .. } => (self.ready)(),
            Lazy::Failed => false,
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
        let (tx, rx) = std::sync::mpsc::channel::<Line>();
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

    /// A synth that panics on its first line (a crashed renderer).
    struct PanicSynth;
    impl Synth for PanicSynth {
        fn tts(&mut self, _: &str, _: &VoiceParams, _: &Path) -> Result<()> {
            panic!("renderer crashed");
        }
    }

    /// Busy while a reply renders, and not stuck busy when the render thread panics with
    /// replies still queued; later lines are printed only.
    #[test]
    fn tts_speaker_busy_through_render_and_after_a_panic() {
        use astromech_voice::colour::Colour;
        let d = tempfile::tempdir().unwrap();
        let emma = || VoiceParams::new("bf_emma", 1.0, Colour::Cockpit).unwrap();
        let mixer = || astromech_voice::mixer::Mixer::silent(24000);
        let log = Arc::new(Mutex::new(vec![]));
        let slow =
            TtsSpeaker::with_synth(SlowSynth(log.clone()), "kokoro-test", d.path(), emma(), mixer(), vec![]).unwrap();
        assert!(!slow.busy());
        slow.say("Cargo scan.");
        assert!(slow.busy(), "busy while the reply renders");
        wait_for(&log, 1);
        // Rendered; with no output device (and not on Windows) nothing plays, so it frees up.
        let t = std::time::Instant::now();
        while slow.busy() && t.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(!slow.busy());

        let crash = TtsSpeaker::with_synth(PanicSynth, "kokoro-test", d.path(), emma(), mixer(), vec![]).unwrap();
        crash.say("First.");
        crash.say("Second.");
        let t = std::time::Instant::now();
        while crash.busy() && t.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(!crash.busy(), "a dead renderer must not leave the speaker busy");
        crash.say("Third.");
        assert!(!crash.busy());
    }

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

    /// Like FakeSynth, but each render takes a while (Kokoro on a CPU).
    struct SlowSynth(Arc<Mutex<Vec<(String, String)>>>);
    impl Synth for SlowSynth {
        fn tts(&mut self, text: &str, voice: &VoiceParams, out_wav: &Path) -> Result<()> {
            std::thread::sleep(Duration::from_millis(20));
            FakeSynth(self.0.clone()).tts(text, voice, out_wav)
        }
    }

    /// Review R1: a voice change re-renders the fixed phrases in the background, but a reply said
    /// after it goes first, and warm-ups for voices no longer selected are skipped.
    #[test]
    fn replies_go_before_the_warm_up_backlog() {
        use astromech_voice::colour::Colour;
        let d = tempfile::tempdir().unwrap();
        let log = Arc::new(Mutex::new(vec![]));
        let v = |n: &str| VoiceParams::new(n, 1.0, Colour::Cockpit).unwrap();
        let phrases: Vec<String> = (0..12).map(|i| format!("Phrase {i}.")).collect();
        let s = TtsSpeaker::with_synth(
            SlowSynth(log.clone()),
            "kokoro-test",
            d.path(),
            v("bf_emma"),
            astromech_voice::mixer::Mixer::silent(24000),
            phrases.clone(),
        )
        .unwrap();
        // Click through two voices to a third, then preview.
        s.set_voice(&v("bf_lily"));
        s.set_voice(&v("bm_daniel"));
        s.set_voice(&v("bm_george"));
        s.say(PREVIEW_LINE);
        // Wait for George's twelve warm-ups and the reply (plus at most one stale line).
        let t0 = std::time::Instant::now();
        while log.lock().unwrap().iter().filter(|(_, v)| v == "bm_george").count() < 13
            && t0.elapsed() < Duration::from_secs(5)
        {
            std::thread::sleep(Duration::from_millis(5));
        }
        let got = log.lock().unwrap().clone();
        let at = got.iter().position(|(t, _)| t == PREVIEW_LINE).unwrap();
        // Only a warm-up already rendering (and, on a loaded machine, one picked up between the
        // last voice change and the reply) can go before it; not the backlog of twelve.
        assert!(at <= 2, "reply rendered at {at}: {got:?}");
        assert_eq!(got[at].1, "bm_george");
        // Lily and Daniel were never rendered beyond one in-progress line; George got all twelve.
        let by = |n: &str| got.iter().filter(|(_, v)| v == n).count();
        assert!(by("bf_lily") + by("bm_daniel") + by("bf_emma") <= 1, "{got:?}");
        assert_eq!(got.iter().filter(|(t, v)| v == "bm_george" && t != PREVIEW_LINE).count(), 12, "{got:?}");
    }

    /// astromech-mcdt: a session that started before the model was downloaded prints lines, then
    /// starts Kokoro on the first line said after the download and speaks it and the lines that
    /// follow, in order and in the voice chosen meanwhile, without being restarted.
    #[test]
    fn lazy_speaker_starts_once_the_model_is_ready() {
        use astromech_voice::colour::Colour;
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        let d = tempfile::tempdir().unwrap();
        let dir = d.path().to_path_buf();
        let log = Arc::new(Mutex::new(vec![]));
        let ready = Arc::new(AtomicBool::new(false));
        let starts = Arc::new(AtomicUsize::new(0));
        let (r, n, l) = (ready.clone(), starts.clone(), log.clone());
        let s = LazySpeaker::new(
            move || r.load(Ordering::SeqCst),
            move || {
                n.fetch_add(1, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(50)); // loading the model
                let v = VoiceParams::new("bf_emma", 1.0, Colour::Cockpit)?;
                let mixer = astromech_voice::mixer::Mixer::silent(24000);
                Ok(Arc::new(TtsSpeaker::with_synth(FakeSynth(l.clone()), "kokoro-test", &dir, v, mixer, vec![])?)
                    as Arc<dyn Speaker>)
            },
        );
        s.say("Before the download.");
        s.set_voice(&VoiceParams::new("bm_george", 1.0, Colour::Cockpit).unwrap());
        assert!(!s.speaks_aloud());
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(starts.load(Ordering::SeqCst), 0);
        assert!(log.lock().unwrap().is_empty());
        // The download finishes.
        ready.store(true, Ordering::SeqCst);
        assert!(s.speaks_aloud());
        s.say("First.");
        s.say("Second."); // said while Kokoro loads
        assert_eq!(wait_for(&log, 2), [("First.".into(), "bm_george".into()), ("Second.".into(), "bm_george".into())]);
        s.say("Third.");
        assert_eq!(wait_for(&log, 3)[2], ("Third.".into(), "bm_george".into()));
        assert_eq!(starts.load(Ordering::SeqCst), 1);
    }

    struct Loud(RecordingSpeaker);
    impl Speaker for Loud {
        fn say(&self, text: &str) {
            self.0.say(text)
        }
        fn speaks_aloud(&self) -> bool {
            true
        }
    }

    /// Review R2/R4: the preview Kokoro starts once, is reused, can be refused or routed, is
    /// released when idle, and a speaker that only prints is reported as speech being off.
    #[test]
    fn preview_slot_lifecycle() {
        let slot = PreviewSlot::default();
        let starts = std::cell::Cell::new(0);
        let loud = Arc::new(Loud(RecordingSpeaker::default()));
        let start = || {
            starts.set(starts.get() + 1);
            Ok(loud.clone() as Arc<dyn Speaker>)
        };
        slot.say(PREVIEW_LINE, || None, start).unwrap();
        slot.say(PREVIEW_LINE, || None, start).unwrap();
        assert_eq!(starts.get(), 1);
        assert_eq!(loud.0.take().len(), 2);
        // While a session starts, previews are refused without loading anything.
        assert!(slot.take());
        let e = slot.say(PREVIEW_LINE, || Some(Err(anyhow!("voice is starting"))), start).unwrap_err();
        assert_eq!(e.to_string(), "voice is starting");
        assert!(!slot.loaded() && starts.get() == 1);
        // Idle release.
        slot.say(PREVIEW_LINE, || None, start).unwrap();
        let now = std::time::Instant::now();
        assert!(!slot.release_idle(now, PREVIEW_IDLE));
        assert!(slot.release_idle(now + PREVIEW_IDLE, PREVIEW_IDLE));
        assert!(!slot.loaded());
        // Speech output off: a speaker that only prints.
        let e = slot.say(PREVIEW_LINE, || None, || Ok(Arc::new(ConsoleSpeaker) as Arc<dyn Speaker>)).unwrap_err();
        assert_eq!(e.to_string(), "speech output is off");
        assert!(!slot.loaded());
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
