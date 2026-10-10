//! S9: local STT (Parakeet) and TTS (Kokoro) speed on this PC, via sherpa-onnx in the
//! worker process. Platform-specific pieces (mic, PTT, playback) come in through traits.

use crate::worker::{self, Worker, C_API_LIB};
use anyhow::{anyhow, bail, Context, Result};
use s79_common::catalog::{self, mb, Asset};
use s79_common::download::{self, find_file, Phase, Progress};
use s79_common::phrases::{COMMANDS, CONFIRMATIONS};
use s79_common::proto::Request;
use s79_common::score::{score, Score};
use s79_common::stats::{summarize, Summary};
use serde::Serialize;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub trait Ui {
    fn out(&self, s: &str);
    fn ask(&self, q: &str) -> String;
    fn progress(&self, s: &str) {
        // Pad instead of using ANSI erase codes (old consoles do not interpret them).
        print!("\r{s:<118}");
        let _ = std::io::stdout().flush();
    }
}

pub fn ask_yn(ui: &dyn Ui, q: &str, default: bool) -> bool {
    loop {
        match ui.ask(&format!("{q} [{}] ", if default { "Y/n" } else { "y/N" })).as_str() {
            "" => return default,
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => ui.out("  please answer y or n"),
        }
    }
}

/// A push-to-talk event for the mic test.
pub enum PttEvent {
    Down(Instant),
    Up(Instant),
    /// Esc pressed: stop the mic test.
    Cancel,
}

pub trait PttSource {
    fn describe(&self) -> String;
    fn next(&self, timeout: Duration) -> Option<PttEvent>;
    fn drain(&self);
}

pub trait Mic {
    fn sample_rate(&self) -> u32;
    fn device_name(&self) -> String;
    /// Total mono samples captured so far.
    fn position(&self) -> u64;
    /// Block until a capture callback has delivered audio recorded after `t` (or `max` passes).
    fn wait_after(&self, t: Instant, max: Duration);
    fn take(&self, from: u64, to: u64) -> Vec<f32>;
}

pub trait Player {
    fn play_wav(&self, path: &Path) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct S9Config {
    pub base: PathBuf,
    /// s9-voice-worker (next to the main exe).
    pub worker_src: PathBuf,
    /// s9-gpu-probe (next to the main exe), for the optional DirectML test.
    pub probe_src: PathBuf,
    pub threads: i32,
    pub sid: i32,
    pub speed: f32,
    pub phrases: usize,
    pub tts_asset: Asset,
    /// Optional DirectML GPU test. None = ask (Windows with a GPU listed).
    pub gpu: Option<bool>,
    pub gpu_present: bool,
    pub sweep: bool,
    pub yes: bool,
    pub audio_dir: PathBuf,
    pub log_dir: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SynthRow {
    pub phrase: String,
    pub transcript: String,
    pub score: Option<Score>,
    pub tts_ms: f64,
    pub audio_ms: f64,
    pub stt_decode_ms: f64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MicRow {
    pub phrase: String,
    pub transcript: String,
    pub score: Option<Score>,
    pub audio_ms: f64,
    pub peak: f32,
    pub rms: f32,
    /// PTT release -> audio up to the release is in our buffer.
    pub release_to_audio_ms: f64,
    pub wav_write_ms: f64,
    pub worker_read_ms: f64,
    pub worker_decode_ms: f64,
    /// PTT release -> transcript back in the main process (the number that matters).
    pub release_to_transcript_ms: f64,
    pub wav: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TtsRow {
    pub text: String,
    pub first_audio_ms: f64,
    pub total_ms: f64,
    pub audio_ms: f64,
    pub rtf: f64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ProviderResult {
    pub provider: String,
    pub runtime: String,
    pub threads: i32,
    pub ok: bool,
    pub error: Option<String>,
    pub stt_load_ms: Option<f64>,
    pub tts_load_ms: Option<f64>,
    /// Decode time of the saved recordings (mic if any, otherwise synthetic).
    pub stt_decode_ms: Option<Summary>,
    pub stt_audio_set: String,
    pub tts_first_audio_ms: Option<Summary>,
    pub tts_rtf: Option<Summary>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct S9Results {
    /// `s9-voice-worker --version` (startup self-check).
    pub worker_version: Option<String>,
    pub sherpa_onnx_version: Option<String>,
    pub sherpa_onnx_git_sha1: Option<String>,
    pub stt_model: String,
    pub tts_model: String,
    pub tts_sample_rate: Option<i32>,
    pub tts_speakers: Option<i32>,
    pub tts_sid: i32,
    pub downloads: Vec<String>,
    pub mic_device: Option<String>,
    pub mic_sample_rate: Option<u32>,
    pub ptt: Option<String>,
    pub synthetic: Vec<SynthRow>,
    pub synthetic_exact: Option<String>,
    pub mic: Vec<MicRow>,
    pub mic_exact: Option<String>,
    pub release_to_transcript_ms: Option<Summary>,
    pub tts: Vec<TtsRow>,
    pub tts_first_audio_ms: Option<Summary>,
    pub tts_rtf: Option<Summary>,
    pub voice_sounded_ok: Option<String>,
    pub providers: Vec<ProviderResult>,
    /// Optional DirectML test: s9-gpu-probe's result (raw ONNX graph timings, CPU vs DirectML).
    pub gpu_probe: Option<serde_json::Value>,
    pub gpu_summary: Vec<String>,
    pub notes: Vec<String>,
    pub error: Option<String>,
}

pub struct SttPaths {
    pub encoder: PathBuf,
    pub decoder: PathBuf,
    pub joiner: PathBuf,
    pub tokens: PathBuf,
}

pub struct TtsPaths {
    pub model: PathBuf,
    pub voices: PathBuf,
    pub tokens: PathBuf,
    pub data_dir: PathBuf,
}

fn model_root(base: &Path, a: &Asset) -> PathBuf {
    base.join(a.dest).join(a.stem())
}

fn pick(root: &Path, prefix: &str) -> Result<PathBuf> {
    find_file(root, &|n| n.starts_with(prefix) && n.ends_with(".int8.onnx"))
        .or_else(|| find_file(root, &|n| n.starts_with(prefix) && n.ends_with(".onnx")))
        .ok_or_else(|| anyhow!("no {prefix}*.onnx under {}", root.display()))
}

pub fn stt_paths(root: &Path) -> Result<SttPaths> {
    Ok(SttPaths {
        encoder: pick(root, "encoder")?,
        decoder: pick(root, "decoder")?,
        joiner: pick(root, "joiner")?,
        tokens: find_file(root, &|n| n == "tokens.txt").ok_or_else(|| anyhow!("no tokens.txt under {}", root.display()))?,
    })
}

pub fn tts_paths(root: &Path) -> Result<TtsPaths> {
    let model = find_file(root, &|n| n.starts_with("model") && n.ends_with(".onnx")).ok_or_else(|| anyhow!("no model*.onnx under {}", root.display()))?;
    let dir = model.parent().unwrap().to_path_buf();
    let need = |p: PathBuf| -> Result<PathBuf> {
        if p.exists() {
            Ok(p)
        } else {
            Err(anyhow!("missing {}", p.display()))
        }
    };
    Ok(TtsPaths { voices: need(dir.join("voices.bin"))?, tokens: need(dir.join("tokens.txt"))?, data_dir: need(dir.join("espeak-ng-data"))?, model })
}

fn s(p: &Path) -> String {
    p.display().to_string()
}

pub fn load_stt_req(p: &SttPaths, provider: &str, threads: i32) -> Request {
    Request::LoadStt { encoder: s(&p.encoder), decoder: s(&p.decoder), joiner: s(&p.joiner), tokens: s(&p.tokens), provider: provider.into(), threads }
}

pub fn load_tts_req(p: &TtsPaths, provider: &str, threads: i32) -> Request {
    Request::LoadTts { model: s(&p.model), voices: s(&p.voices), tokens: s(&p.tokens), data_dir: s(&p.data_dir), provider: provider.into(), threads }
}

pub fn progress_line(label: &str, p: &Progress) -> String {
    let secs = p.elapsed.as_secs_f64().max(0.001);
    let rate = p.done as f64 / secs;
    let pct = if p.total > 0 { p.done as f64 * 100.0 / p.total as f64 } else { 0.0 };
    let verb = match p.phase {
        Phase::Download => "Downloading",
        Phase::Verify => "Checking",
        Phase::Extract => "Unpacking",
    };
    let eta = if rate > 0.0 && p.total > p.done { format!(", ~{:.0} s left", (p.total - p.done) as f64 / rate) } else { String::new() };
    format!("  {verb} {label}: {:.1} / {:.1} MB ({pct:.0}%), {:.1} MB/s{eta}", mb(p.done), mb(p.total), rate / 1e6)
}

/// Download (with a size warning) whatever is missing. Returns false if the user declined.
pub fn ensure_assets(ui: &dyn Ui, base: &Path, assets: &[Asset], yes: bool, log: &mut Vec<String>) -> Result<bool> {
    let missing: Vec<&Asset> = assets.iter().filter(|a| !download::is_installed(base, a)).collect();
    if missing.is_empty() {
        return Ok(true);
    }
    let total: u64 = missing.iter().map(|a| a.size).sum();
    ui.out("\n  These files are needed (official sherpa-onnx GitHub releases / Microsoft NuGet, checked with SHA-256):");
    for a in &missing {
        ui.out(&format!("    {:>7.1} MB  {}", mb(a.size), a.label));
    }
    ui.out(&format!(
        "    -------\n    {:>7.1} MB  total download; about {} of free disk space needed.\n  They go into {} (models\\ and runtimes\\). Delete those folders to remove them.",
        mb(total),
        if mb(total) > 200.0 { format!("{:.1} GB", mb(total) * 2.2 / 1000.0) } else { format!("{:.0} MB", mb(total) * 2.5) },
        base.display()
    ));
    if !yes && !ask_yn(ui, "  Download now?", true) {
        return Ok(false);
    }
    for a in missing {
        let t = Instant::now();
        download::ensure(base, a, &mut |p| ui.progress(&progress_line(a.label, &p))).with_context(|| format!("getting {}", a.file))?;
        ui.progress("");
        ui.out(&format!("  OK  {} ({:.0} s)", a.label, t.elapsed().as_secs_f64()));
        log.push(format!("{} sha256 {} OK", a.file, a.sha256));
    }
    Ok(true)
}

/// Where the worker's shared libraries are: none on Windows (statically linked), the
/// downloaded runtime's lib folder on Linux.
pub fn runtime_lib_dir(base: &Path, rt: Option<&Asset>) -> Result<Option<PathBuf>> {
    let Some(a) = rt else { return Ok(None) };
    let root = base.join(a.dest);
    let lib = find_file(&root, &|n| n == C_API_LIB).ok_or_else(|| anyhow!("{C_API_LIB} not found under {}", root.display()))?;
    Ok(Some(lib.parent().unwrap().to_path_buf()))
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

const LOAD_TIMEOUT: Duration = Duration::from_secs(300);
const CALL_TIMEOUT: Duration = Duration::from_secs(120);

fn exact_ratio<'a>(scores: impl Iterator<Item = &'a Option<Score>>) -> Option<String> {
    let v: Vec<&Score> = scores.flatten().collect();
    if v.is_empty() {
        return None;
    }
    let exact = v.iter().filter(|s| s.exact).count();
    let loose = v.iter().filter(|s| s.loose).count();
    let errs: usize = v.iter().map(|s| s.word_errors).sum();
    let words: usize = v.iter().map(|s| s.ref_words).sum();
    Some(format!("{exact}/{} exact ({loose} ignoring spaces), WER {:.1}%", v.len(), errs as f64 * 100.0 / words.max(1) as f64))
}

pub struct S9Io<'a> {
    pub ui: &'a dyn Ui,
    pub mic: Option<&'a dyn Mic>,
    pub ptt: Option<&'a dyn PttSource>,
    pub player: Option<&'a dyn Player>,
}

/// Run S9. `save` is called after every step so partial results survive a crash.
pub fn run_s9(cfg: &S9Config, io: &S9Io, res: &mut S9Results, save: &mut dyn FnMut(&S9Results)) -> Result<()> {
    let ui = io.ui;
    res.tts_sid = cfg.sid;
    let cpu_rt = catalog::cpu_runtime();
    let stt_a = catalog::STT_PARAKEET_V2_INT8;
    let tts_a = cfg.tts_asset;
    res.stt_model = stt_a.label.into();
    res.tts_model = tts_a.label.into();
    // Startup self-check before any download: the worker must be there and must start.
    // (On Linux it needs the runtime first, so it is checked after the download there.)
    if cpu_rt.is_none() {
        let v = worker::preflight(&cfg.worker_src, None)?;
        ui.out(&format!("  Voice worker OK: {v}"));
        res.worker_version = Some(v);
    }
    let mut needed: Vec<Asset> = cpu_rt.into_iter().collect();
    needed.extend([stt_a, tts_a]);
    if !ensure_assets(ui, &cfg.base, &needed, cfg.yes, &mut res.downloads)? {
        res.error = Some("downloads declined".into());
        return Ok(());
    }
    let stt_p = stt_paths(&model_root(&cfg.base, &stt_a))?;
    let tts_p = tts_paths(&model_root(&cfg.base, &tts_a))?;
    std::fs::create_dir_all(&cfg.audio_dir)?;
    std::fs::create_dir_all(&cfg.log_dir)?;

    // ---- CPU worker ----
    let wlib = runtime_lib_dir(&cfg.base, cpu_rt.as_ref())?;
    if res.worker_version.is_none() {
        res.worker_version = Some(worker::preflight(&cfg.worker_src, wlib.as_deref())?);
    }
    let runtime_label = cpu_rt.map(|a| a.file).unwrap_or("sherpa-onnx linked into s9-voice-worker.exe");
    ui.out(&format!("\n  Starting the voice worker (CPU, {} threads)...", cfg.threads));
    let mut w = Worker::spawn(&cfg.worker_src, wlib.as_deref(), &cfg.log_dir.join("worker-cpu.log"), "CPU")?;
    let info = w.call_ok(&Request::Info, CALL_TIMEOUT)?;
    res.sherpa_onnx_version = info.version;
    res.sherpa_onnx_git_sha1 = info.git_sha1;
    let mut cpu = ProviderResult { provider: "cpu".into(), runtime: runtime_label.into(), threads: cfg.threads, ..Default::default() };
    let r = w.call_ok(&load_stt_req(&stt_p, "cpu", cfg.threads), LOAD_TIMEOUT)?;
    cpu.stt_load_ms = r.load_ms;
    ui.out(&format!("  Parakeet loaded in {:.1} s", r.load_ms.unwrap_or(0.0) / 1000.0));
    let r = w.call_ok(&load_tts_req(&tts_p, "cpu", cfg.threads), LOAD_TIMEOUT)?;
    cpu.tts_load_ms = r.load_ms;
    res.tts_sample_rate = r.sample_rate;
    res.tts_speakers = r.num_speakers;
    ui.out(&format!("  Kokoro loaded in {:.1} s ({} voices, {} Hz)", r.load_ms.unwrap_or(0.0) / 1000.0, r.num_speakers.unwrap_or(0), r.sample_rate.unwrap_or(0)));
    // Warm-up (first inference allocates; not counted).
    let warm = cfg.audio_dir.join("warmup.wav");
    w.call_ok(&Request::Tts { text: "Systems ready.".into(), sid: cfg.sid, speed: cfg.speed, out_wav: s(&warm) }, CALL_TIMEOUT)?;
    w.call_ok(&Request::Stt { wav: s(&warm) }, CALL_TIMEOUT)?;
    save(res);

    // ---- Test 0: synthetic TTS -> STT (no microphone involved) ----
    ui.out("\n  Test 0 (automatic, no mic): Kokoro speaks each command, Parakeet transcribes it.");
    let mut synth_wavs = vec![];
    for (i, phrase) in COMMANDS.iter().take(cfg.phrases).enumerate() {
        let wav = cfg.audio_dir.join(format!("synth-{:02}.wav", i + 1));
        let t = w.call_ok(&Request::Tts { text: (*phrase).into(), sid: cfg.sid, speed: cfg.speed, out_wav: s(&wav) }, CALL_TIMEOUT)?;
        let r = w.call_ok(&Request::Stt { wav: s(&wav) }, CALL_TIMEOUT)?;
        let text = r.text.unwrap_or_default();
        let sc = score(phrase, &text);
        ui.out(&format!("    {} {:<28} -> {:<32} decode {:>6.0} ms for {:>5.0} ms audio", if sc.exact { "OK " } else if sc.loose { "~  " } else { "-- " }, phrase, format!("\"{text}\""), r.decode_ms.unwrap_or(0.0), r.audio_ms.unwrap_or(0.0)));
        res.synthetic.push(SynthRow { phrase: (*phrase).into(), transcript: text, score: Some(sc), tts_ms: t.total_ms.unwrap_or(0.0), audio_ms: r.audio_ms.unwrap_or(0.0), stt_decode_ms: r.decode_ms.unwrap_or(0.0) });
        synth_wavs.push(wav);
    }
    res.synthetic_exact = exact_ratio(res.synthetic.iter().map(|r| &r.score));
    if let Some(e) = &res.synthetic_exact {
        ui.out(&format!("    {e}"));
    }
    save(res);

    // ---- Test 1: microphone + PTT ----
    let mut mic_wavs = vec![];
    if let (Some(mic), Some(ptt)) = (io.mic, io.ptt) {
        res.mic_device = Some(mic.device_name());
        res.mic_sample_rate = Some(mic.sample_rate());
        res.ptt = Some(ptt.describe());
        ui.out(&format!(
            "\n  Test 1 (you speak): microphone \"{}\" at {} Hz, push-to-talk = {}.\n  For each line: hold PTT, say the phrase, release. We time PTT release -> transcript. Esc stops early.",
            mic.device_name(),
            mic.sample_rate(),
            ptt.describe()
        ));
        ui.ask("  Press Enter when ready...");
        let sr = mic.sample_rate() as u64;
        let mut i = 0;
        let mut retries = 0;
        'phr: while i < cfg.phrases.min(COMMANDS.len()) {
            let phrase = COMMANDS[i];
            ui.out(&format!("\n  [{}/{}] Hold PTT and say:   \"{phrase}\"", i + 1, cfg.phrases.min(COMMANDS.len())));
            ptt.drain();
            let down = loop {
                match ptt.next(Duration::from_secs(120)) {
                    Some(PttEvent::Down(t)) => break t,
                    Some(PttEvent::Cancel) | None => {
                        ui.out("  Mic test stopped.");
                        break 'phr;
                    }
                    Some(PttEvent::Up(_)) => continue,
                }
            };
            let pos_down = mic.position();
            ui.progress("  ...recording (release PTT when done)");
            let up = loop {
                match ptt.next(Duration::from_secs(20)) {
                    Some(PttEvent::Up(t)) => break t,
                    Some(PttEvent::Cancel) | None => {
                        ui.progress("");
                        ui.out("  Mic test stopped.");
                        break 'phr;
                    }
                    Some(PttEvent::Down(_)) => continue,
                }
            };
            mic.wait_after(up, Duration::from_millis(200));
            let pos_end = mic.position();
            let preroll = sr / 5;
            let samples = mic.take(pos_down.saturating_sub(preroll), pos_end);
            let t_audio = Instant::now();
            ui.progress("");
            let held = up.saturating_duration_since(down);
            if held < Duration::from_millis(300) || samples.len() < (sr as usize) / 4 {
                ui.out("  That was too short - hold PTT while you speak. Again:");
                retries += 1;
                if retries > 3 {
                    i += 1;
                    retries = 0;
                }
                continue;
            }
            retries = 0;
            let peak = samples.iter().fold(0f32, |m, x| m.max(x.abs()));
            let rms = (samples.iter().map(|x| x * x).sum::<f32>() / samples.len() as f32).sqrt();
            let wav = cfg.audio_dir.join(format!("mic-{:02}.wav", i + 1));
            let tw = Instant::now();
            s79_common::wav::write_mono_i16(&wav, mic.sample_rate(), &samples)?;
            let wav_write_ms = ms(tw);
            let r = w.call_ok(&Request::Stt { wav: s(&wav) }, CALL_TIMEOUT)?;
            let total = up.elapsed().as_secs_f64() * 1000.0;
            let text = r.text.clone().unwrap_or_default();
            let sc = score(phrase, &text);
            ui.out(&format!(
                "  {} heard \"{text}\"   release->transcript {total:.0} ms (decode {:.0} ms, {:.1} s of audio){}",
                if sc.exact { "OK " } else if sc.loose { "~  " } else { "-- " },
                r.decode_ms.unwrap_or(0.0),
                samples.len() as f64 / sr as f64,
                if peak < 0.02 { "   [very quiet - is the right microphone selected?]" } else { "" }
            ));
            res.mic.push(MicRow {
                phrase: phrase.into(),
                transcript: text,
                score: Some(sc),
                audio_ms: samples.len() as f64 * 1000.0 / sr as f64,
                peak,
                rms,
                release_to_audio_ms: (t_audio - up).as_secs_f64() * 1000.0,
                wav_write_ms,
                worker_read_ms: r.read_ms.unwrap_or(0.0),
                worker_decode_ms: r.decode_ms.unwrap_or(0.0),
                release_to_transcript_ms: total,
                wav: wav.file_name().unwrap().to_string_lossy().into(),
            });
            mic_wavs.push(wav);
            save(res);
            i += 1;
        }
        res.mic_exact = exact_ratio(res.mic.iter().map(|r| &r.score));
        res.release_to_transcript_ms = summarize(&res.mic.iter().map(|r| r.release_to_transcript_ms).collect::<Vec<_>>());
        if let (Some(e), Some(s)) = (&res.mic_exact, &res.release_to_transcript_ms) {
            ui.out(&format!("\n  Mic test: {e}; PTT release -> transcript ms: {s}"));
        }
        save(res);
    } else {
        res.notes.push("Test 1 (microphone) skipped: no microphone/PTT on this platform or disabled".into());
    }

    // ---- Test 2: TTS confirmations ----
    ui.out(&format!("\n  Test 2: Kokoro speaks {} confirmations{}.", CONFIRMATIONS.len(), if io.player.is_some() { " (you should hear each one)" } else { "" }));
    for (i, text) in CONFIRMATIONS.iter().enumerate() {
        let wav = cfg.audio_dir.join(format!("tts-{:02}.wav", i + 1));
        let r = w.call_ok(&Request::Tts { text: (*text).into(), sid: cfg.sid, speed: cfg.speed, out_wav: s(&wav) }, CALL_TIMEOUT)?;
        let row = TtsRow { text: (*text).into(), first_audio_ms: r.first_audio_ms.unwrap_or(0.0), total_ms: r.total_ms.unwrap_or(0.0), audio_ms: r.audio_ms.unwrap_or(0.0), rtf: r.rtf.unwrap_or(f64::NAN) };
        ui.out(&format!("    first audio {:>5.0} ms, total {:>5.0} ms for {:>5.0} ms of speech (RTF {:.3})  \"{text}\"", row.first_audio_ms, row.total_ms, row.audio_ms, row.rtf));
        res.tts.push(row);
        if let Some(p) = io.player {
            if let Err(e) = p.play_wav(&wav) {
                ui.out(&format!("    (playback failed: {e:#})"));
            }
        }
    }
    res.tts_first_audio_ms = summarize(&res.tts.iter().map(|r| r.first_audio_ms).collect::<Vec<_>>());
    res.tts_rtf = summarize(&res.tts.iter().map(|r| r.rtf).collect::<Vec<_>>());
    if io.player.is_some() {
        let a = ui.ask("  Did the voice play and sound clear? [y/n, or type a comment] ");
        res.voice_sounded_ok = Some(a);
    }
    cpu.ok = true;
    let decode_set: Vec<PathBuf> = if mic_wavs.is_empty() { synth_wavs.clone() } else { mic_wavs.clone() };
    let set_name = if mic_wavs.is_empty() { "synthetic" } else { "mic" };
    cpu.stt_audio_set = set_name.into();
    cpu.stt_decode_ms = summarize(&if mic_wavs.is_empty() { res.synthetic.iter().map(|r| r.stt_decode_ms).collect::<Vec<_>>() } else { res.mic.iter().map(|r| r.worker_decode_ms).collect() });
    cpu.tts_first_audio_ms = res.tts_first_audio_ms.clone();
    cpu.tts_rtf = res.tts_rtf.clone();
    res.providers.push(cpu);
    save(res);

    // ---- CPU thread sweep (same audio, no playback) ----
    if cfg.sweep {
        let max = std::thread::available_parallelism().map(|n| n.get() as i32).unwrap_or(4);
        let cands: Vec<i32> = [1, 2, 4, 8].into_iter().filter(|t| *t <= max && *t != cfg.threads).collect();
        if !cands.is_empty() {
            ui.out(&format!("\n  CPU thread sweep ({:?} threads; re-decodes the same audio, no playback)...", cands));
        }
        for t in cands {
            let p = bench_provider(&mut w, "cpu", runtime_label, t, &stt_p, &tts_p, &decode_set, set_name, cfg, &cfg.audio_dir);
            ui.out(&format!("    {}", provider_line(&p)));
            res.providers.push(p);
            save(res);
        }
    }
    drop(w);

    // ---- Optional GPU test: DirectML (any DirectX 12 GPU: AMD, Intel, NVIDIA) ----
    let try_gpu = match cfg.gpu {
        Some(v) => v,
        None if cfg.gpu_present && cfg!(windows) => {
            let mbs: f64 = catalog::gpu_runtime().iter().filter(|a| !download::is_installed(&cfg.base, a)).map(|a| mb(a.size)).sum();
            ui.out(&format!(
                "\n  Optional GPU test (DirectML, works with AMD/Intel/NVIDIA, no driver installs): times the Parakeet\n  \
                 encoder and Kokoro on your GPU vs the CPU. Downloads {mbs:.0} MB from Microsoft (NuGet). Takes ~1 minute."
            ));
            ask_yn(ui, "  Run the GPU test?", true)
        }
        None => false,
    };
    if try_gpu {
        let r = run_gpu_probe(cfg, ui, &stt_p, &tts_p, &mut res.downloads);
        match r {
            Ok((v, lines)) => {
                for l in &lines {
                    ui.out(&format!("    {l}"));
                }
                res.gpu_summary = lines;
                res.gpu_probe = Some(v);
            }
            Err(e) => {
                ui.out(&format!("    GPU test failed: {e:#}"));
                res.gpu_summary = vec![format!("GPU test failed: {e:#}")];
            }
        }
        save(res);
    } else {
        res.notes.push("GPU (DirectML) test not run".into());
    }
    Ok(())
}

/// Run s9-gpu-probe (CPU vs DirectML on the raw ONNX graphs). Returns its JSON and summary lines.
fn run_gpu_probe(cfg: &S9Config, ui: &dyn Ui, stt_p: &SttPaths, tts_p: &TtsPaths, downloads: &mut Vec<String>) -> Result<(serde_json::Value, Vec<String>)> {
    if !cfg!(windows) && cfg.gpu != Some(true) {
        bail!("DirectML is Windows-only");
    }
    if !cfg.probe_src.is_file() {
        bail!("{} is missing - keep it next to the main exe (it is in the zip)", cfg.probe_src.display());
    }
    let assets = catalog::gpu_runtime();
    if !ensure_assets(ui, &cfg.base, &assets, true, downloads)? {
        bail!("download declined");
    }
    let dir = cfg.base.join(assets[0].dest);
    let ort = std::env::var_os("S9_PROBE_ORT").map(PathBuf::from).unwrap_or_else(|| dir.join(assets[0].file));
    let mut cmd = std::process::Command::new(&cfg.probe_src);
    cmd.arg("--ort").arg(&ort);
    if cfg!(windows) {
        cmd.arg("--directml").arg(dir.join(assets[1].file));
    }
    cmd.arg("--encoder").arg(&stt_p.encoder).arg("--kokoro").arg(&tts_p.model).arg("--voices").arg(&tts_p.voices).arg("--tokens").arg(&tts_p.tokens);
    cmd.arg("--threads").arg(cfg.threads.to_string()).arg("--runs").arg("5");
    let log = cfg.log_dir.join("gpu-probe.log");
    cmd.stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::from(std::fs::File::create(&log)?));
    if let Some(d) = cfg.probe_src.parent() {
        cmd.current_dir(d);
    }
    worker::hide_window(&mut cmd);
    ui.out("  Running the GPU test (CPU first, then DirectML)...");
    let mut child = cmd.spawn().with_context(|| format!("start {}", cfg.probe_src.display()))?;
    let mut stdout = child.stdout.take().unwrap();
    let reader = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = std::io::Read::read_to_string(&mut stdout, &mut s);
        s
    });
    let t0 = Instant::now();
    let status = loop {
        if let Some(st) = child.try_wait()? {
            break st;
        }
        if t0.elapsed() > Duration::from_secs(900) {
            let _ = child.kill();
            bail!("the GPU test did not finish within 15 minutes (killed); see {}", log.display());
        }
        std::thread::sleep(Duration::from_millis(200));
    };
    let out = reader.join().unwrap_or_default();
    let line = out.lines().find_map(|l| l.strip_prefix("@@GPU@@ ")).ok_or_else(|| anyhow!("the GPU test ended without a result: {}; see {}", worker::explain_exit(status.code()), log.display()))?;
    let v: serde_json::Value = serde_json::from_str(line)?;
    Ok((v.clone(), gpu_summary(&v)))
}

/// Human-readable lines from the probe's JSON (also used for the final summary).
pub fn gpu_summary(v: &serde_json::Value) -> Vec<String> {
    let mut lines = vec![];
    if let Some(e) = v["error"].as_str() {
        lines.push(format!("GPU test could not run: {e}"));
        return lines;
    }
    // (model, label) -> cpu median, for the speed-up column.
    let mut cpu: std::collections::HashMap<(String, String), f64> = Default::default();
    for p in v["providers"].as_array().into_iter().flatten() {
        let prov = p["provider"].as_str().unwrap_or("?");
        let models = p["models"].as_array().cloned().unwrap_or_default();
        let errs: Vec<&str> = models.iter().filter_map(|m| m["error"].as_str()).collect();
        if !models.is_empty() && errs.len() == models.len() && errs.iter().all(|e| *e == errs[0]) {
            lines.push(format!("{prov:<8} FAILED - {}", errs[0].chars().take(400).collect::<String>()));
            continue;
        }
        for m in &models {
            let model = m["model"].as_str().unwrap_or("?");
            if !m["ok"].as_bool().unwrap_or(false) {
                lines.push(format!("{prov:<8} {model}: FAILED - {}", m["error"].as_str().unwrap_or("?")));
                continue;
            }
            let placement = m["placement"].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join("; ")).unwrap_or_default();
            for t in m["timings"].as_array().into_iter().flatten() {
                let label = t["label"].as_str().unwrap_or("").to_string();
                let med = t["median_ms"].as_f64().unwrap_or(f64::NAN);
                let key = (model.to_string(), label.clone());
                let speed = if prov == "cpu" {
                    cpu.insert(key, med);
                    String::new()
                } else {
                    cpu.get(&key).map(|c| format!("  ({:.1}x vs CPU)", c / med)).unwrap_or_default()
                };
                lines.push(format!("{prov:<8} {model:<16} {label:<26} median {med:>6.0} ms (min {:.0}){speed}", t["min_ms"].as_f64().unwrap_or(f64::NAN)));
            }
            if prov != "cpu" && !placement.is_empty() {
                lines.push(format!("{:<8} {model:<16} placement: {}", "", placement.chars().take(300).collect::<String>()));
            }
        }
    }
    lines
}

/// Load both models with `provider`/`threads` on `w`, then time decoding `wavs` and
/// synthesising the confirmations (no playback).
#[allow(clippy::too_many_arguments)]
fn bench_provider(w: &mut Worker, provider: &str, runtime: &str, threads: i32, stt_p: &SttPaths, tts_p: &TtsPaths, wavs: &[PathBuf], set_name: &str, cfg: &S9Config, dir: &Path) -> ProviderResult {
    let mut p = ProviderResult { provider: provider.into(), runtime: runtime.into(), threads, stt_audio_set: set_name.into(), ..Default::default() };
    let r = (|| -> Result<()> {
        let l = w.call_ok(&load_stt_req(stt_p, provider, threads), LOAD_TIMEOUT)?;
        p.stt_load_ms = l.load_ms;
        if let Some(first) = wavs.first() {
            w.call_ok(&Request::Stt { wav: s(first) }, CALL_TIMEOUT)?; // warm-up
        }
        let mut d = vec![];
        for wav in wavs {
            d.push(w.call_ok(&Request::Stt { wav: s(wav) }, CALL_TIMEOUT)?.decode_ms.unwrap_or(f64::NAN));
        }
        p.stt_decode_ms = summarize(&d);
        let l = w.call_ok(&load_tts_req(tts_p, provider, threads), LOAD_TIMEOUT)?;
        p.tts_load_ms = l.load_ms;
        let scratch = dir.join(format!("bench-{provider}-{threads}.wav"));
        w.call_ok(&Request::Tts { text: "Systems ready.".into(), sid: cfg.sid, speed: cfg.speed, out_wav: String::new() }, CALL_TIMEOUT)?;
        let (mut first, mut rtf) = (vec![], vec![]);
        for text in CONFIRMATIONS {
            let r = w.call_ok(&Request::Tts { text: (*text).into(), sid: cfg.sid, speed: cfg.speed, out_wav: s(&scratch) }, CALL_TIMEOUT)?;
            first.push(r.first_audio_ms.unwrap_or(f64::NAN));
            rtf.push(r.rtf.unwrap_or(f64::NAN));
        }
        let _ = std::fs::remove_file(&scratch);
        p.tts_first_audio_ms = summarize(&first);
        p.tts_rtf = summarize(&rtf);
        Ok(())
    })();
    match r {
        Ok(()) => p.ok = true,
        Err(e) => p.error = Some(format!("{e:#}")),
    }
    p
}

pub fn provider_line(p: &ProviderResult) -> String {
    if !p.ok {
        return format!("{} ({} threads): FAILED - {}", p.provider, p.threads, p.error.clone().unwrap_or_default());
    }
    format!(
        "{} ({} thr): STT decode median {:.0} ms ({} audio), TTS first-audio median {:.0} ms, RTF median {:.3}",
        p.provider,
        p.threads,
        p.stt_decode_ms.as_ref().map(|s| s.median).unwrap_or(f64::NAN),
        p.stt_audio_set,
        p.tts_first_audio_ms.as_ref().map(|s| s.median).unwrap_or(f64::NAN),
        p.tts_rtf.as_ref().map(|s| s.median).unwrap_or(f64::NAN),
    )
}

pub fn default_threads() -> i32 {
    let n = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    if n >= 8 { 4 } else { 2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_path_discovery() {
        let d = tempfile::tempdir().unwrap();
        let root = d.path().join("m");
        std::fs::create_dir_all(root.join("espeak-ng-data")).unwrap();
        for f in ["encoder.int8.onnx", "decoder.int8.onnx", "joiner.int8.onnx", "tokens.txt", "model.onnx", "voices.bin"] {
            std::fs::write(root.join(f), b"x").unwrap();
        }
        let p = stt_paths(&root).unwrap();
        assert!(p.encoder.ends_with("encoder.int8.onnx"));
        let t = tts_paths(&root).unwrap();
        assert!(t.data_dir.ends_with("espeak-ng-data"));
        assert!(stt_paths(&d.path().join("nope")).is_err());
    }

    #[test]
    fn progress_text() {
        let p = Progress { phase: Phase::Download, done: 50_000_000, total: 100_000_000, elapsed: Duration::from_secs(5) };
        let l = progress_line("X", &p);
        assert!(l.contains("50.0 / 100.0 MB (50%)"), "{l}");
        assert!(l.contains("10.0 MB/s"), "{l}");
        assert!(l.contains("~5 s left"), "{l}");
    }

    /// End-to-end mic flow against the real worker and models, with a fake microphone that
    /// "plays" earlier synthetic recordings and a fake PTT. Runs only when S9_IT_BASE points at
    /// a folder with installed models/runtimes and S9_IT_WORKER at a worker binary.
    #[test]
    fn mic_flow_end_to_end_with_real_worker() {
        use std::sync::{Arc, Mutex, OnceLock};
        let (Ok(base), Ok(worker)) = (std::env::var("S9_IT_BASE"), std::env::var("S9_IT_WORKER")) else {
            eprintln!("skipped: set S9_IT_BASE and S9_IT_WORKER");
            return;
        };
        let base = PathBuf::from(base);
        let wavs: Vec<PathBuf> = (1..=3)
            .map(|i| find_file(&base.join("s9-audio"), &|n| n == format!("synth-{i:02}.wav")).expect("run bench once first"))
            .collect();
        let sr = 24000u32;
        let mut buf: Vec<f32> = vec![0.0; sr as usize / 2];
        let mut sched: Vec<(f64, bool)> = vec![];
        for w in &wavs {
            let (r, x) = s79_common::wav::read_mono_f32(w).unwrap();
            assert_eq!(r, sr);
            let t0 = buf.len() as f64 / sr as f64;
            buf.extend_from_slice(&x);
            let t1 = buf.len() as f64 / sr as f64;
            sched.push((t0 - 0.05, true));
            sched.push((t1 + 0.1, false));
            buf.extend(std::iter::repeat_n(0.0, sr as usize * 6 / 10));
        }
        let clock: Arc<OnceLock<Instant>> = Arc::new(OnceLock::new());
        struct FMic { buf: Vec<f32>, sr: u32, clock: Arc<OnceLock<Instant>> }
        impl Mic for FMic {
            fn sample_rate(&self) -> u32 { self.sr }
            fn device_name(&self) -> String { "fake".into() }
            fn position(&self) -> u64 {
                let t = self.clock.get().map(|c| c.elapsed().as_secs_f64()).unwrap_or(0.0);
                ((t * self.sr as f64) as u64).min(self.buf.len() as u64)
            }
            fn wait_after(&self, _t: Instant, _m: Duration) {}
            fn take(&self, a: u64, b: u64) -> Vec<f32> { self.buf[a as usize..(b as usize).min(self.buf.len())].to_vec() }
        }
        struct FPtt { sched: Mutex<Vec<(f64, bool)>>, clock: Arc<OnceLock<Instant>> }
        impl PttSource for FPtt {
            fn describe(&self) -> String { "fake PTT".into() }
            fn drain(&self) {}
            fn next(&self, _t: Duration) -> Option<PttEvent> {
                let c = *self.clock.get_or_init(Instant::now);
                let mut s = self.sched.lock().unwrap();
                if s.is_empty() { return Some(PttEvent::Cancel); }
                let (t, down) = s.remove(0);
                let at = c + Duration::from_secs_f64(t.max(0.0));
                if let Some(d) = at.checked_duration_since(Instant::now()) { std::thread::sleep(d); }
                Some(if down { PttEvent::Down(at) } else { PttEvent::Up(at) })
            }
        }
        struct FPlayer(Mutex<u32>);
        impl Player for FPlayer {
            fn play_wav(&self, p: &Path) -> Result<()> { assert!(p.is_file()); *self.0.lock().unwrap() += 1; Ok(()) }
        }
        struct FUi;
        impl Ui for FUi {
            fn out(&self, s: &str) { println!("{s}"); }
            fn ask(&self, _q: &str) -> String { String::new() }
            fn progress(&self, _s: &str) {}
        }
        let mic = FMic { buf, sr, clock: clock.clone() };
        let ptt = FPtt { sched: Mutex::new(sched), clock };
        let player = FPlayer(Mutex::new(0));
        let tmp = tempfile::tempdir().unwrap();
        let cfg = S9Config {
            base: base.clone(),
            worker_src: PathBuf::from(worker),
            probe_src: PathBuf::from("none"),
            threads: 4,
            sid: 0,
            speed: 1.0,
            phrases: 3,
            tts_asset: catalog::TTS_KOKORO_EN,
            gpu: Some(false),
            gpu_present: false,
            sweep: false,
            yes: true,
            audio_dir: tmp.path().join("audio"),
            log_dir: tmp.path().join("logs"),
        };
        let io = S9Io { ui: &FUi, mic: Some(&mic), ptt: Some(&ptt), player: Some(&player) };
        let mut res = S9Results::default();
        run_s9(&cfg, &io, &mut res, &mut |_| {}).unwrap();
        assert_eq!(res.mic.len(), 3, "{:?}", res.mic);
        for r in &res.mic {
            assert!(r.score.as_ref().unwrap().loose, "{} -> {}", r.phrase, r.transcript);
            assert!(r.release_to_transcript_ms > 0.0 && r.release_to_transcript_ms < 5000.0);
            println!("{:<22} release->transcript {:.0} ms (decode {:.0} ms)", r.phrase, r.release_to_transcript_ms, r.worker_decode_ms);
        }
        assert_eq!(*player.0.lock().unwrap(), CONFIRMATIONS.len() as u32);
        assert_eq!(res.voice_sounded_ok.as_deref(), Some(""));
    }
}
