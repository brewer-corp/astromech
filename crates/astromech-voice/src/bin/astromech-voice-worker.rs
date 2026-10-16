//! astromech-voice-worker: runs sherpa-onnx (Parakeet STT, Kokoro TTS) and serves requests over
//! stdin/stdout JSON lines (see `astromech_voice::proto`).
//!
//! On Windows sherpa-onnx and onnxruntime are linked statically into this exe (the
//! static-MT prebuilt libraries), so it needs no DLLs besides Windows' own. The main tool
//! starts it from the folder it was unpacked to. It never touches input devices or the game.

use astromech_voice::proto::{Request, Response};
use sherpa_onnx::{
    GenerationConfig, OfflineRecognizer, OfflineRecognizerConfig, OfflineTts, OfflineTtsConfig,
    OfflineTtsKokoroModelConfig, OfflineTtsModelConfig,
};
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;

struct State {
    stt: Option<OfflineRecognizer>,
    tts: Option<OfflineTts>,
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

fn handle(st: &mut State, req: Request) -> Response {
    match req {
        Request::Info => Response {
            ok: true,
            version: Some(sherpa_onnx::version().to_string()),
            git_sha1: Some(sherpa_onnx::git_sha1().to_string()),
            ..Default::default()
        },
        Request::LoadStt { encoder, decoder, joiner, tokens, provider, threads } => {
            st.stt = None;
            let mut c = OfflineRecognizerConfig::default();
            c.model_config.transducer.encoder = Some(encoder);
            c.model_config.transducer.decoder = Some(decoder);
            c.model_config.transducer.joiner = Some(joiner);
            c.model_config.tokens = Some(tokens);
            c.model_config.model_type = Some("nemo_transducer".into());
            c.model_config.num_threads = threads;
            c.model_config.provider = Some(provider);
            c.decoding_method = Some("greedy_search".into());
            let t = Instant::now();
            match OfflineRecognizer::create(&c) {
                Some(r) => {
                    st.stt = Some(r);
                    Response { ok: true, load_ms: Some(ms(t)), ..Default::default() }
                }
                None => Response::err("OfflineRecognizer::create failed (see worker log)"),
            }
        }
        Request::LoadTts { model, voices, tokens, data_dir, lang, provider, threads } => {
            st.tts = None;
            let c = OfflineTtsConfig {
                model: OfflineTtsModelConfig {
                    kokoro: OfflineTtsKokoroModelConfig {
                        model: Some(model),
                        voices: Some(voices),
                        tokens: Some(tokens),
                        data_dir: Some(data_dir),
                        lang: (!lang.is_empty()).then_some(lang),
                        ..Default::default()
                    },
                    num_threads: threads,
                    debug: false,
                    provider: Some(provider),
                    ..Default::default()
                },
                max_num_sentences: 1,
                silence_scale: 0.2,
                ..Default::default()
            };
            let t = Instant::now();
            match OfflineTts::create(&c) {
                Some(tts) => {
                    let r = Response {
                        ok: true,
                        load_ms: Some(ms(t)),
                        sample_rate: Some(tts.sample_rate()),
                        num_speakers: Some(tts.num_speakers()),
                        ..Default::default()
                    };
                    st.tts = Some(tts);
                    r
                }
                None => Response::err("OfflineTts::create failed (see worker log)"),
            }
        }
        Request::Stt { wav } => {
            let Some(rec) = st.stt.as_ref() else { return Response::err("STT model not loaded") };
            let t = Instant::now();
            let (sr, samples) = match astromech_voice::wav::read_mono_f32(std::path::Path::new(&wav)) {
                Ok(x) => x,
                Err(e) => return Response::err(format!("{e:#}")),
            };
            let read_ms = ms(t);
            let t = Instant::now();
            let stream = rec.create_stream();
            stream.accept_waveform(sr as i32, &samples);
            rec.decode(&stream);
            let text = stream.get_result().map(|r| r.text).unwrap_or_default();
            Response {
                ok: true,
                text: Some(text.trim().to_string()),
                read_ms: Some(read_ms),
                decode_ms: Some(ms(t)),
                audio_ms: Some(samples.len() as f64 * 1000.0 / sr.max(1) as f64),
                ..Default::default()
            }
        }
        Request::Tts { text, sid, speed, out_wav, lang } => {
            let Some(tts) = st.tts.as_ref() else { return Response::err("TTS model not loaded") };
            let first: Arc<Mutex<(Option<f64>, u32)>> = Arc::new(Mutex::new((None, 0)));
            let f2 = first.clone();
            let t = Instant::now();
            let extra = (!lang.is_empty()).then(|| [("lang".to_string(), serde_json::Value::String(lang))].into());
            let cfg = GenerationConfig { sid, speed, extra, ..Default::default() };
            let cb = move |samples: &[f32], _progress: f32| -> bool {
                let mut g = f2.lock().unwrap();
                if g.0.is_none() && !samples.is_empty() {
                    g.0 = Some(t.elapsed().as_secs_f64() * 1000.0);
                }
                g.1 += 1;
                true
            };
            let audio = tts.generate_with_config(&text, &cfg, Some(cb));
            let total = ms(t);
            let Some(audio) = audio else { return Response::err("TTS generation failed") };
            let sr = audio.sample_rate();
            let audio_ms = audio.samples().len() as f64 * 1000.0 / sr.max(1) as f64;
            if !out_wav.is_empty() {
                if let Err(e) =
                    astromech_voice::wav::write_mono_i16(std::path::Path::new(&out_wav), sr as u32, audio.samples())
                {
                    return Response::err(format!("writing {out_wav}: {e:#}"));
                }
            }
            let (first_ms, callbacks) = *first.lock().unwrap();
            Response {
                ok: true,
                first_audio_ms: Some(first_ms.unwrap_or(total)),
                total_ms: Some(total),
                audio_ms: Some(audio_ms),
                rtf: Some(if audio_ms > 0.0 { total / audio_ms } else { f64::NAN }),
                sample_rate: Some(sr),
                callbacks: Some(callbacks),
                ..Default::default()
            }
        }
        Request::Quit => Response { ok: true, ..Default::default() },
    }
}

fn link_mode() -> &'static str {
    if cfg!(windows) {
        "static"
    } else {
        "shared"
    }
}

fn main() {
    use std::io::IsTerminal;
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "astromech-voice-worker {}: internal helper for s7-s9-input-voice. Not meant to be run by hand.",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    // Startup self-check used by the main tool (and handy by hand): proves the speech
    // engine is linked and loads, without loading any model.
    if args.iter().any(|a| a == "--version") {
        println!(
            "astromech-voice-worker {} sherpa-onnx {} ({}) linked {}",
            env!("CARGO_PKG_VERSION"),
            sherpa_onnx::version(),
            sherpa_onnx::git_sha1(),
            link_mode()
        );
        return;
    }
    // Double-clicked in Explorer: explain instead of sitting in a silent console window.
    if args.is_empty() && std::io::stdin().is_terminal() {
        println!("astromech-voice-worker is a helper process for s7-s9-input-voice.exe and does nothing on its own.");
        println!("Run s7-s9-input-voice.exe instead (keep both exes in the same folder).");
        println!("\nPress Enter to close.");
        let mut s = String::new();
        let _ = std::io::stdin().read_line(&mut s);
        return;
    }
    let mut st = State { stt: None, tts: None };
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let (resp, quit) = match serde_json::from_str::<Request>(&line) {
            Ok(Request::Quit) => (handle(&mut st, Request::Quit), true),
            Ok(req) => {
                // A panic inside one request must not kill the worker silently.
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handle(&mut st, req)));
                (r.unwrap_or_else(|_| Response::err("worker panicked while handling the request")), false)
            }
            Err(e) => (Response::err(format!("bad request: {e}")), false),
        };
        if writeln!(out, "{}", resp.to_line()).and_then(|_| out.flush()).is_err() || quit {
            break;
        }
    }
}
