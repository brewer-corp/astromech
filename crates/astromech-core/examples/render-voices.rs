//! Render one line with each Kokoro voice through the real text-to-speech path
//! ([`TtsSpeaker`]: voice worker, colour chain, WAV cache) and copy the results to
//! `<out>/<voice>.wav`. For voice A/B sets and evidence.
//!
//!   cargo run -p astromech-core --example render-voices -- \
//!     --worker target/debug/astromech-voice-worker --models <models dir> --out <dir>
//!
//! Default voices: the British ones. On Linux the worker needs the sherpa-onnx shared
//! libraries (`LD_LIBRARY_PATH=target/debug`, after building it with `--features sherpa`).

use anyhow::{bail, Result};
use astromech_core::speech::TtsSpeaker;
use astromech_voice::{catalog, colour::Colour};
use clap::Parser;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    worker: PathBuf,
    /// Folder holding the extracted model folders
    #[arg(long)]
    models: PathBuf,
    /// Where `<voice>.wav` files go (a scratch `cache/` folder is made inside it)
    #[arg(long)]
    out: PathBuf,
    #[arg(long, default_value = "Landing gear deployed.")]
    line: String,
    #[arg(long, default_value = "cockpit")]
    colour: String,
    /// Voices to render (default: every British voice)
    #[arg(long, num_args = 1..)]
    voices: Vec<String>,
}

/// The cached WAV the speaker wrote for its one line (not the `.raw.wav` scratch file).
fn rendered(dir: &Path) -> Option<PathBuf> {
    std::fs::read_dir(dir).ok()?.flatten().map(|e| e.path()).find(|p| {
        let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        n.ends_with(".wav") && !n.ends_with(".raw.wav")
    })
}

fn main() -> Result<()> {
    let a = Args::parse();
    let colour = match a.colour.as_str() {
        "natural" => Colour::Natural,
        "cockpit" => Colour::Cockpit,
        "harsh" => Colour::Harsh,
        c => bail!("unknown colour {c:?}"),
    };
    let voices: Vec<String> = if a.voices.is_empty() {
        catalog::KOKORO_VOICES.iter().filter(|v| v.starts_with('b')).map(|v| v.to_string()).collect()
    } else {
        a.voices.clone()
    };
    std::fs::create_dir_all(&a.out)?;
    for v in &voices {
        let data = a.out.join("cache").join(v);
        let _ = std::fs::remove_dir_all(&data);
        let t = Instant::now();
        let _speaker = TtsSpeaker::start(&a.worker, &a.models, 4, &data, v, 1.0, colour, vec![a.line.clone()])?;
        let cache = data.join("tts-cache");
        let wav = loop {
            if let Some(w) = rendered(&cache) {
                break w;
            }
            if t.elapsed() > Duration::from_secs(180) {
                bail!("{v}: nothing rendered after 180 s (see {})", data.join("worker-tts.log").display());
            }
            std::thread::sleep(Duration::from_millis(200));
        };
        // The file appears before the speaker has finished writing it: wait until it stops growing.
        loop {
            let size = std::fs::metadata(&wav)?.len();
            std::thread::sleep(Duration::from_millis(300));
            if size == std::fs::metadata(&wav)?.len() && astromech_voice::wav::read_mono_f32(&wav).is_ok() {
                break;
            }
        }
        let dest = a.out.join(format!("{v}.wav"));
        std::fs::copy(&wav, &dest)?;
        let (sr, samples) = astromech_voice::wav::read_mono_f32(&dest)?;
        println!(
            "{v} (sid {}): {} ({} bytes, {:.2} s at {sr} Hz) in {:.1} s",
            catalog::voice_sid(v).unwrap_or(-1),
            dest.display(),
            std::fs::metadata(&dest)?.len(),
            samples.len() as f64 / sr as f64,
            t.elapsed().as_secs_f64()
        );
    }
    Ok(())
}
