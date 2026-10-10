//! s9-gpu-probe: times the two heavy ONNX graphs of the voice pipeline (the Parakeet
//! encoder and the Kokoro TTS model) with ONNX Runtime on the CPU and, on Windows, on
//! DirectML (any DirectX 12 GPU: AMD, Intel or NVIDIA).
//!
//! It loads Microsoft's official `onnxruntime.dll` (DirectML build) and `DirectML.dll`
//! from full paths given on the command line, so nothing is linked at build time and a
//! missing runtime becomes a reported error instead of a Windows loader dialog.
//!
//! This is a raw-graph benchmark: it feeds realistic input *shapes* (synthetic values) and
//! measures model time only. Feature extraction, the TDT decoder/joiner loop and the
//! phonemiser are cheap on the CPU and are not part of it. The main tool runs it and reads
//! the single result line prefixed with `@@GPU@@ `.

use anyhow::{anyhow, bail, Context, Result};
use ort::session::{builder::GraphOptimizationLevel, Session};
use ort::value::Tensor;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Default)]
struct Args {
    ort: PathBuf,
    directml: Option<PathBuf>,
    encoder: Option<PathBuf>,
    kokoro: Option<PathBuf>,
    voices: Option<PathBuf>,
    tokens: Option<PathBuf>,
    threads: usize,
    runs: usize,
    providers: Vec<String>,
}

fn parse_args() -> Result<Args> {
    let mut a = Args { threads: 4, runs: 5, providers: vec!["cpu".into(), "directml".into()], ..Default::default() };
    let mut it = std::env::args().skip(1);
    while let Some(k) = it.next() {
        let mut v = || it.next().ok_or_else(|| anyhow!("{k} needs a value"));
        match k.as_str() {
            "--ort" => a.ort = v()?.into(),
            "--directml" => a.directml = Some(v()?.into()),
            "--encoder" => a.encoder = Some(v()?.into()),
            "--kokoro" => a.kokoro = Some(v()?.into()),
            "--voices" => a.voices = Some(v()?.into()),
            "--tokens" => a.tokens = Some(v()?.into()),
            "--threads" => a.threads = v()?.parse()?,
            "--runs" => a.runs = v()?.parse()?,
            "--providers" => a.providers = v()?.split(',').map(|s| s.trim().to_string()).collect(),
            "-h" | "--help" => {
                println!("s9-gpu-probe {}: internal helper for s7-s9-input-voice (run it with `s7-s9-input-voice.exe s9 --gpu`).", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => bail!("unknown argument {k}"),
        }
    }
    if a.ort.as_os_str().is_empty() {
        bail!("--ort <path to onnxruntime.dll> is required");
    }
    Ok(a)
}

#[derive(Debug, Default, Serialize)]
struct Timing {
    label: String,
    median_ms: f64,
    min_ms: f64,
    max_ms: f64,
    runs: usize,
}

#[derive(Debug, Default, Serialize)]
struct ModelResult {
    model: String,
    ok: bool,
    error: Option<String>,
    session_ms: Option<f64>,
    /// ORT's own node-placement summary for this session (from its verbose log).
    placement: Vec<String>,
    timings: Vec<Timing>,
    /// Kokoro only: length of the generated audio.
    audio_ms: Option<f64>,
}

#[derive(Debug, Default, Serialize)]
struct ProviderResult {
    provider: String,
    ok: bool,
    error: Option<String>,
    models: Vec<ModelResult>,
}

#[derive(Debug, Default, Serialize)]
struct Output {
    ok: bool,
    error: Option<String>,
    ort_path: String,
    directml_path: Option<String>,
    ort_build_info: Option<String>,
    vc_runtime: Option<String>,
    threads: usize,
    providers: Vec<ProviderResult>,
}

#[cfg_attr(not(windows), allow(dead_code))]
fn first_line(s: &str) -> String {
    s.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim().to_string()
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

fn timing(label: &str, mut v: Vec<f64>) -> Timing {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    let median = if n == 0 { f64::NAN } else if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 };
    Timing { label: label.into(), median_ms: median, min_ms: v.first().copied().unwrap_or(f64::NAN), max_ms: v.last().copied().unwrap_or(f64::NAN), runs: n }
}

/// Deterministic pseudo-random N(0,1)-ish values (normalised log-mel features look like this).
fn features(n: usize, seed: u64) -> Vec<f32> {
    let mut s = seed;
    (0..n)
        .map(|_| {
            let mut acc = 0.0f32;
            for _ in 0..4 {
                s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                acc += ((s >> 33) as f32 / (1u64 << 31) as f32) - 0.5;
            }
            acc * 1.7
        })
        .collect()
}

/// Map an IPA string to Kokoro token ids using its tokens.txt ("<symbol> <id>" per line).
fn kokoro_tokens(tokens_txt: &Path, ipa: &str) -> Result<Vec<i64>> {
    let text = std::fs::read_to_string(tokens_txt).with_context(|| format!("read {}", tokens_txt.display()))?;
    let mut map = std::collections::HashMap::new();
    for line in text.lines() {
        let Some(i) = line.rfind(' ') else { continue };
        let (sym, id) = (&line[..i], line[i + 1..].trim());
        let sym = if sym.is_empty() { " " } else { sym };
        if let (Some(c), Ok(id)) = (sym.chars().next(), id.parse::<i64>()) {
            if sym.chars().count() == 1 {
                map.insert(c, id);
            }
        }
    }
    let mut ids = vec![0i64];
    ids.extend(ipa.chars().filter_map(|c| map.get(&c).copied()));
    ids.push(0);
    if ids.len() < 10 {
        bail!("tokens.txt did not map the test phonemes (got {} ids)", ids.len());
    }
    Ok(ids)
}

type Log = Arc<Mutex<Vec<String>>>;

fn builder(provider: &str, threads: usize, log: &Log) -> Result<ort::session::builder::SessionBuilder> {
    let e = |e: ort::Error<ort::session::builder::SessionBuilder>| anyhow!("{e}");
    let l2 = log.clone();
    let mut b = Session::builder()
        .map_err(|e| anyhow!("{e}"))?
        .with_optimization_level(GraphOptimizationLevel::Level3)
        .map_err(e)?
        .with_intra_threads(threads)
        .map_err(e)?
        .with_log_level(ort::logging::LogLevel::Verbose)
        .map_err(e)?
        .with_logger(Arc::new(move |_lvl, _cat, _id, _loc, msg: &str| {
            let m = msg.trim();
            if m.contains("placed on [") || m.contains("Number of nodes") || m.contains("DML") && m.contains("adapter") {
                if let Ok(mut v) = l2.lock() {
                    if v.len() < 40 {
                        v.push(m.lines().next().unwrap_or("").chars().take(200).collect());
                    }
                }
            }
        }))
        .map_err(e)?;
    if provider == "directml" {
        b = add_directml(b)?;
    }
    Ok(b)
}

#[cfg(windows)]
fn add_directml(b: ort::session::builder::SessionBuilder) -> Result<ort::session::builder::SessionBuilder> {
    use ort::ep::{self, ExecutionProvider};
    let e = |e: ort::Error<ort::session::builder::SessionBuilder>| anyhow!("{e}");
    // DirectML needs sequential execution and no memory patterns.
    let mut b = b.with_parallel_execution(false).map_err(e)?.with_memory_pattern(false).map_err(e)?;
    // Preferred: let DirectML pick the high-performance GPU (DXCore, Windows 10 2004+).
    // Fallback: DXGI adapter 0 (the GPU driving the main display).
    let dml = ep::DirectML::default().with_performance_preference(ep::directml::PerformancePreference::HighPerformance).with_device_filter(ep::directml::DeviceFilter::Gpu);
    if let Err(e1) = dml.register(&mut b) {
        let e1 = e1.to_string();
        ep::DirectML::default().with_device_id(0).register(&mut b).map_err(|e2| anyhow!("registering DirectML failed: {} / adapter 0: {}", first_line(&e1), first_line(&e2.to_string())))?;
    }
    Ok(b)
}

#[cfg(not(windows))]
fn add_directml(_b: ort::session::builder::SessionBuilder) -> Result<ort::session::builder::SessionBuilder> {
    bail!("DirectML is Windows-only")
}

fn run_encoder(path: &Path, provider: &str, a: &Args) -> ModelResult {
    let mut r = ModelResult { model: "parakeet-encoder".into(), ..Default::default() };
    let log: Log = Arc::new(Mutex::new(vec![]));
    let res = (|| -> Result<()> {
        let t = Instant::now();
        let mut s = builder(provider, a.threads, &log)?.commit_from_file(path).map_err(|e| anyhow!("loading {}: {e}", path.display()))?;
        r.session_ms = Some(ms(t));
        // ~1.3 s (a typical command) and ~3 s of audio at a 10 ms hop.
        for (label, frames) in [("1.3 s audio", 130usize), ("3 s audio", 300)] {
            let x = features(128 * frames, frames as u64);
            let mut v = vec![];
            for i in 0..(a.runs + 1) {
                let feats = Tensor::from_array(([1usize, 128, frames], x.clone()))?;
                let len = Tensor::from_array(([1usize], vec![frames as i64]))?;
                let t = Instant::now();
                let out = s.run(ort::inputs!["audio_signal" => feats, "length" => len])?;
                let (shape, _) = out["outputs"].try_extract_tensor::<f32>()?;
                if shape.len() != 3 || shape[1] != 1024 {
                    bail!("unexpected encoder output shape {shape:?}");
                }
                if i > 0 {
                    v.push(ms(t)); // first run is warm-up
                }
            }
            r.timings.push(timing(label, v));
        }
        Ok(())
    })();
    r.placement = log.lock().map(|v| v.clone()).unwrap_or_default();
    match res {
        Ok(()) => r.ok = true,
        Err(e) => r.error = Some(format!("{e:#}")),
    }
    r
}

fn run_kokoro(path: &Path, voices: &Path, tokens: &Path, provider: &str, a: &Args) -> ModelResult {
    let mut r = ModelResult { model: "kokoro".into(), ..Default::default() };
    let log: Log = Arc::new(Mutex::new(vec![]));
    let res = (|| -> Result<()> {
        // "Landing gear deployed." in espeak-style IPA.
        let ids = kokoro_tokens(tokens, "lˈændɪŋ ɡˈɪɹ dᵻplˈɔɪd.")?;
        let vb = std::fs::read(voices).with_context(|| format!("read {}", voices.display()))?;
        let floats: Vec<f32> = vb.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)).collect();
        let row = (ids.len() - 2).min(floats.len() / 256 - 1);
        let style: Vec<f32> = floats[row * 256..row * 256 + 256].to_vec(); // voice 0
        let t = Instant::now();
        let mut s = builder(provider, a.threads, &log)?.commit_from_file(path).map_err(|e| anyhow!("loading {}: {e}", path.display()))?;
        r.session_ms = Some(ms(t));
        let mut v = vec![];
        for i in 0..(a.runs + 1) {
            let tok = Tensor::from_array(([1usize, ids.len()], ids.clone()))?;
            let st = Tensor::from_array(([1usize, 256], style.clone()))?;
            let sp = Tensor::from_array(([1usize], vec![1.0f32]))?;
            let t = Instant::now();
            let out = s.run(ort::inputs!["tokens" => tok, "style" => st, "speed" => sp])?;
            let (_, audio) = out["audio"].try_extract_tensor::<f32>()?;
            let el = ms(t);
            if audio.len() < 2400 {
                bail!("Kokoro produced only {} samples", audio.len());
            }
            r.audio_ms = Some(audio.len() as f64 / 24.0);
            if i > 0 {
                v.push(el);
            }
        }
        r.timings.push(timing(&format!("one sentence ({} tokens)", ids.len()), v));
        Ok(())
    })();
    r.placement = log.lock().map(|v| v.clone()).unwrap_or_default();
    match res {
        Ok(()) => r.ok = true,
        Err(e) => r.error = Some(format!("{e:#}")),
    }
    r
}

/// Load `DirectML.dll` by full path first, so `onnxruntime.dll`'s import of "DirectML.dll"
/// binds to this copy (DirectML 1.15.x, which this onnxruntime build expects) and not to the
/// older in-box copy in System32.
#[cfg(windows)]
fn preload(dml: Option<&Path>, ort_dir: &Path) -> Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::System::LibraryLoader::{LoadLibraryExW, SetDllDirectoryW, LOAD_WITH_ALTERED_SEARCH_PATH};
    let w = |p: &Path| p.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<u16>>();
    unsafe {
        SetDllDirectoryW(w(ort_dir).as_ptr());
        if let Some(d) = dml {
            if LoadLibraryExW(w(d).as_ptr(), std::ptr::null_mut(), LOAD_WITH_ALTERED_SEARCH_PATH).is_null() {
                bail!("could not load {} (error {})", d.display(), std::io::Error::last_os_error());
            }
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn preload(_dml: Option<&Path>, _ort_dir: &Path) -> Result<()> {
    Ok(())
}

/// The official onnxruntime.dll needs the Visual C++ 2015-2022 runtime (x64).
#[cfg(windows)]
fn vc_runtime() -> std::result::Result<String, String> {
    use windows_sys::Win32::System::LibraryLoader::LoadLibraryW;
    let mut missing = vec![];
    for n in ["vcruntime140.dll", "vcruntime140_1.dll", "msvcp140.dll", "msvcp140_1.dll"] {
        let w: Vec<u16> = n.encode_utf16().chain(Some(0)).collect();
        if unsafe { LoadLibraryW(w.as_ptr()) }.is_null() {
            missing.push(n);
        }
    }
    if missing.is_empty() {
        Ok("Visual C++ 2015-2022 runtime found".into())
    } else {
        Err(format!(
            "the Microsoft Visual C++ 2015-2022 Redistributable (x64) is not installed (missing {}); onnxruntime.dll needs it. \
             It is the vc_redist.x64.exe from Microsoft (most games install it).",
            missing.join(", ")
        ))
    }
}

#[cfg(not(windows))]
fn vc_runtime() -> std::result::Result<String, String> {
    Ok("n/a".into())
}

fn main() {
    let mut out = Output::default();
    let res = (|| -> Result<()> {
        let a = parse_args()?;
        out.threads = a.threads;
        out.ort_path = a.ort.display().to_string();
        out.directml_path = a.directml.as_ref().map(|p| p.display().to_string());
        if !a.ort.is_file() {
            bail!("{} not found", a.ort.display());
        }
        match vc_runtime() {
            Ok(s) => out.vc_runtime = Some(s),
            Err(e) => {
                out.vc_runtime = Some(e.clone());
                bail!(e);
            }
        }
        preload(a.directml.as_deref(), a.ort.parent().unwrap_or(Path::new(".")))?;
        ort::init_from(&a.ort).map_err(|e| anyhow!("loading {}: {e}", a.ort.display()))?.with_name("s9-gpu-probe").commit();
        out.ort_build_info = Some(ort::info().to_string());
        for p in &a.providers {
            let mut pr = ProviderResult { provider: p.clone(), ..Default::default() };
            eprintln!("--- provider {p}");
            if let Some(enc) = &a.encoder {
                pr.models.push(run_encoder(enc, p, &a));
            }
            if let (Some(k), Some(v), Some(t)) = (&a.kokoro, &a.voices, &a.tokens) {
                pr.models.push(run_kokoro(k, v, t, p, &a));
            }
            pr.ok = !pr.models.is_empty() && pr.models.iter().all(|m| m.ok);
            if !pr.ok {
                pr.error = pr.models.iter().find_map(|m| m.error.clone());
            }
            out.providers.push(pr);
        }
        Ok(())
    })();
    match res {
        Ok(()) => out.ok = true,
        Err(e) => out.error = Some(format!("{e:#}")),
    }
    println!("@@GPU@@ {}", serde_json::to_string(&out).unwrap());
}
