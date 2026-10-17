//! Synthetic voice-recognition eval (ARCHITECTURE.md §14 MVP criterion 2; a stand-in for real
//! VR clips, V1-DESIGN.md §6.3). Renders every phrase of `tests/voice-eval-phrases.tsv` with
//! Kokoro in several voices, makes each rendering into four clips (clean, through the Cockpit
//! colour chain, with cockpit-like noise, and both), transcribes them with Parakeet, and scores
//! the transcripts with the `--eval` path ([`astromech_core::eval`]).
//!
//!   scripts/voice-eval.sh            # builds the worker, then runs this
//!
//! Writes into `--out` (default `results/voice-eval`, git-ignored): `audio/<voice>/` (the
//! Kokoro renderings are cached there; delete them to re-render), `clips/` (one JSON sidecar
//! per clip, the `--eval` input) and `report.md`. Never commit any of it.

use anyhow::{bail, Context, Result};
use astromech_core::eval::{self, Clip, Phrase};
use astromech_intent::EvalItem;
use astromech_voice::client::Worker;
use astromech_voice::proto::Request;
use astromech_voice::{catalog, colour, models, wav};
use clap::Parser;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
struct Args {
    /// The sherpa-onnx voice worker (`cargo build -p astromech-voice --features sherpa`)
    #[arg(long)]
    worker: PathBuf,
    /// Folder holding the extracted model folders
    #[arg(long)]
    models: PathBuf,
    /// Download Parakeet and Kokoro (about 830 MB, SHA-256 checked) if they are missing
    #[arg(long)]
    download: bool,
    #[arg(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/voice-eval-phrases.tsv"))]
    phrases: PathBuf,
    #[arg(long, default_value = "results/voice-eval")]
    out: PathBuf,
    /// Kokoro voices to render with: American and British, female and male
    #[arg(long, num_args = 1.., default_values = ["af_heart", "am_michael", "bf_emma", "bm_george"])]
    voices: Vec<String>,
    /// Speech-to-noise ratio of the noisy conditions, in dB
    #[arg(long, default_value_t = 10.0)]
    snr_db: f32,
    #[arg(long, default_value_t = 4)]
    threads: i32,
}

/// The audio chains each rendering goes through.
const CONDITIONS: [&str; 4] = ["clean", "cockpit", "noise", "cockpit+noise"];

/// Silence (or noise) around each phrase, as a push-to-talk clip has.
const PAD_S: f32 = 0.3;

fn main() -> Result<()> {
    let a = Args::parse();
    let phrases = eval::load_phrases(&a.phrases)?;
    let missing = models::missing(&a.models);
    if !missing.is_empty() {
        if !a.download {
            bail!("speech models missing in {} (pass --download to fetch them)", a.models.display());
        }
        std::fs::create_dir_all(&a.models)?;
        for m in &missing {
            println!("Downloading {} ({:.0} MB)", m.label, catalog::mb(m.size));
            models::download(&a.models, m, &mut |_| {})?;
        }
    }
    let stt = models::find_stt(&a.models)?;
    let tts = models::find_tts(&a.models)?;

    let audio = a.out.join("audio");
    let clips = a.out.join("clips");
    let _ = std::fs::remove_dir_all(&clips);
    std::fs::create_dir_all(&clips)?;
    let worker = std::path::absolute(&a.worker)?;
    let mut w = Worker::spawn(&worker, worker.parent(), &std::path::absolute(a.out.join("worker.log"))?, "eval")?;
    let long = Duration::from_secs(300);
    w.call_ok(&models::load_tts_req(&tts, a.threads), long).context("loading Kokoro")?;
    w.call_ok(&models::load_stt_req(&stt, a.threads), long).context("loading Parakeet")?;

    let t = Instant::now();
    for voice in &a.voices {
        let sid = catalog::voice_sid(voice).with_context(|| format!("unknown Kokoro voice {voice:?}"))?;
        let dir = audio.join(voice);
        std::fs::create_dir_all(&dir)?;
        for p in &phrases {
            let raw = dir.join(format!("{}.wav", p.id));
            if !raw.is_file() {
                let req = Request::Tts {
                    text: format!("{}.", p.text),
                    sid,
                    speed: 1.0,
                    out_wav: std::path::absolute(&raw)?.display().to_string(),
                    lang: catalog::voice_lang(voice).into(),
                };
                w.call_ok(&req, Duration::from_secs(60)).with_context(|| format!("rendering {:?}", p.text))?;
            }
            let (sr, speech) = wav::read_mono_f32(&raw)?;
            for cond in CONDITIONS {
                let samples = condition(&speech, sr, cond, a.snr_db, &format!("{voice}.{}", p.id));
                let clip = dir.join(format!("{}.{}.wav", p.id, cond.replace('+', "-")));
                wav::write_mono_i16(&clip, sr, &samples)?;
                let r = w.call_ok(&Request::Stt { wav: std::path::absolute(&clip)?.display().to_string() }, long)?;
                let sidecar = serde_json::json!({
                    "transcript": r.text.unwrap_or_default(),
                    "label": p.label,
                    "family": p.family,
                    "condition": cond,
                    "text": p.text,
                });
                let id = format!("{voice}.{}.{}", cond.replace('+', "-"), p.id);
                std::fs::write(clips.join(format!("{id}.json")), serde_json::to_string_pretty(&sidecar)?)?;
            }
        }
        println!("{voice}: {} phrases in {:.0} s", phrases.len(), t.elapsed().as_secs_f64());
    }

    let run = eval::run(&clips)?;
    let text_only = eval::score(phrases.iter().map(text_clip).collect());
    let report = report(&a, &run, &text_only);
    let path = a.out.join("report.md");
    std::fs::write(&path, report)?;
    println!("{}\n", run.report.summary());
    println!("{}", run.table("Family", |c| c.family.clone()));
    println!("Report: {}", path.display());
    Ok(())
}

/// The phrase's own text as the transcript: what the grammar does with perfect recognition.
fn text_clip(p: &Phrase) -> Clip {
    Clip {
        item: EvalItem { id: p.id.clone(), transcript: p.text.clone(), label: p.label.clone() },
        family: p.family.clone(),
        condition: String::new(),
        said: Some(p.text.clone()),
    }
}

fn report(a: &Args, run: &eval::Run, text_only: &eval::Run) -> String {
    let mut s = String::from("# Voice eval: synthetic phrase set\n\n");
    s += &format!(
        "{} phrases from `{}`, rendered with Kokoro v1.0 ({}) and transcribed with Parakeet TDT 0.6B v2 \
         int8. Conditions: {}; noise is pink noise at {} dB SNR with {} s of lead-in and tail. \
         Generated by `scripts/voice-eval.sh` (astromech {}).\n\n",
        text_only.clips.len(),
        a.phrases.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        a.voices.join(", "),
        CONDITIONS.join(", "),
        a.snr_db,
        PAD_S,
        env!("CARGO_PKG_VERSION"),
    );
    s += &run.markdown();
    s += "\n## By voice\n\n";
    s += &run.table("Voice", |c| c.item.id.split('.').next().unwrap_or_default().to_string());
    s += "\n## Grammar only (phrase text, no audio)\n\n";
    s += "The phrases typed rather than spoken: a miss here is the grammar's, not recognition's.\n\n";
    s += &format!("{}\n\n", text_only.report.summary());
    s += &text_only.table("Family", |c| c.family.clone());
    s
}

/// One recording condition of a rendering. Deterministic: the noise is seeded from `seed`.
fn condition(speech: &[f32], sr: u32, cond: &str, snr_db: f32, seed: &str) -> Vec<f32> {
    let body =
        if cond.starts_with("cockpit") { colour::apply(speech, sr, colour::Colour::Cockpit) } else { speech.to_vec() };
    let pad = vec![0.0; (sr as f32 * PAD_S) as usize];
    let mut out: Vec<f32> = pad.iter().chain(&body).chain(&pad).copied().collect();
    if cond.ends_with("noise") {
        let rms = (body.iter().map(|x| x * x).sum::<f32>() / body.len().max(1) as f32).sqrt();
        let level = rms / 10f32.powf(snr_db / 20.0);
        let noise = pink(out.len(), seed);
        let nrms = (noise.iter().map(|x| x * x).sum::<f32>() / noise.len().max(1) as f32).sqrt().max(1e-9);
        for (o, n) in out.iter_mut().zip(noise) {
            *o += n * level / nrms;
        }
    }
    out
}

/// Pink noise (Paul Kellet's filter over xorshift white noise).
fn pink(n: usize, seed: &str) -> Vec<f32> {
    // FNV-1a of the seed, never zero.
    let mut x = seed.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3)) | 1;
    let mut b = [0f32; 7];
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let white = (x >> 40) as f32 / (1u64 << 24) as f32 * 2.0 - 1.0;
            b[0] = 0.99886 * b[0] + white * 0.0555179;
            b[1] = 0.99332 * b[1] + white * 0.0750759;
            b[2] = 0.96900 * b[2] + white * 0.153852;
            b[3] = 0.86650 * b[3] + white * 0.3104856;
            b[4] = 0.55000 * b[4] + white * 0.5329522;
            b[5] = -0.7616 * b[5] - white * 0.0168980;
            let y = b[..6].iter().sum::<f32>() + b[6] + white * 0.5362;
            b[6] = white * 0.115926;
            y
        })
        .collect()
}
