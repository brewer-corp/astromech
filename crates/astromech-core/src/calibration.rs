//! Mic calibration (V1-DESIGN.md §6.2 step 3) and the mic level check, shared by the CLI
//! (`--calibrate-files`) and the app window (Settings → Microphone → Calibration, onboarding).
//! Both end in [`store`]: one per-device `min_level_dbfs` gate in SQLite, keyed by the device
//! name the session's recorder reports, so the session finds it again ([`stored_gate`]).

use anyhow::{bail, ensure, Result};
use astromech_store::Store;
use astromech_voice::gain::{self, Calibration};
use serde::Serialize;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// The three phrases onboarding and Settings ask for.
pub const PHRASES: [&str; 3] = ["gear down", "deploy hardpoints", "request docking"];
/// How long each calibration clip records.
pub const SILENCE_MS: u64 = 2000;
pub const PHRASE_MS: u64 = 2500;
/// Name the gate is stored under when no device name is known.
pub const DEFAULT_DEVICE: &str = "default";

/// A recorded clip and the device it came from.
#[derive(Debug, Clone)]
pub struct Clip {
    pub device: String,
    pub rate: u32,
    pub samples: Vec<f32>,
}

pub fn gate_key(device: &str) -> String {
    format!("mic.{device}.min_level_dbfs")
}

/// The device name a calibration is stored under: the recording device the session will open
/// for the `[voice] mic` setting `want` (a name or substring; empty is the Windows default).
pub fn device_name(want: &str, names: &[String], default: Option<&str>) -> String {
    if let Some(n) = gain::pick_name(want, names) {
        return n;
    }
    let want = want.trim();
    if want.is_empty() {
        default.map(str::to_string).unwrap_or_else(|| DEFAULT_DEVICE.into())
    } else {
        want.to_string()
    }
}

/// The device the CLI calibrates: the chosen mic as this PC names it.
pub fn configured_device(want: &str) -> String {
    #[cfg(windows)]
    {
        use astromech_voice::mic::Recorder;
        device_name(want, &Recorder::list_names(), Recorder::default_device_name().as_deref())
    }
    #[cfg(not(windows))]
    {
        device_name(want, &[], None)
    }
}

/// Measure and store the gate for `device`. The one writer for the CLI and the window.
pub fn calibrate_and_store(
    store: &Store,
    device: &str,
    rate: u32,
    noise: &[f32],
    speech: &[Vec<f32>],
) -> Result<Calibration> {
    ensure!(!speech.is_empty(), "calibration needs at least one speech clip");
    let refs: Vec<&[f32]> = speech.iter().map(Vec::as_slice).collect();
    let c = gain::calibrate(noise, &refs, rate);
    store.set(&gate_key(device), &format!("{:.1}", c.min_level_dbfs))?;
    store.set(&format!("mic.{device}.noise_dbfs"), &format!("{:.1}", c.noise_dbfs))?;
    store.set(&format!("mic.{device}.speech_peak_dbfs"), &format!("{:.1}", c.speech_peak_dbfs))?;
    Ok(c)
}

/// The stored gate for `device`, if it was calibrated.
pub fn stored_gate(store: &Store, device: &str) -> Option<f32> {
    store.get(&gate_key(device)).ok().flatten()?.parse().ok()
}

pub fn open_store(data_dir: &Path) -> Result<Store> {
    std::fs::create_dir_all(data_dir)?;
    Ok(Store::open(&data_dir.join("astromech.sqlite"))?)
}

/// `--calibrate-files`: a silent WAV then speech WAVs, all at one sample rate.
pub fn read_wavs(files: &[PathBuf]) -> Result<(u32, Vec<f32>, Vec<Vec<f32>>)> {
    ensure!(files.len() >= 2, "--calibrate-files needs a silent WAV then at least one speech WAV");
    let (rate, noise) = astromech_voice::wav::read_mono_f32(&files[0])?;
    let mut speech = vec![];
    for p in &files[1..] {
        let (r, s) = astromech_voice::wav::read_mono_f32(p)?;
        ensure!(r == rate, "{} is {r} Hz, silence was {rate} Hz", p.display());
        speech.push(s);
    }
    Ok((rate, noise, speech))
}

/// What a short recording says about the mic: the level check in onboarding.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LevelCheck {
    pub device: String,
    pub peak_dbfs: f32,
    pub noise_dbfs: f32,
    /// "ok", "quiet" (speech under about −30 dBFS: level correction will boost it) or
    /// "silent" (nothing above the noise: wrong device, muted, or no speech).
    pub verdict: &'static str,
}

pub fn level_check(clip: &Clip) -> LevelCheck {
    let frames = gain::frame_dbfs(&clip.samples, clip.rate);
    let noise = gain::noise_floor_dbfs(&frames);
    let peak = frames.iter().copied().fold(-120.0f32, f32::max);
    let verdict = if peak < -60.0 || peak < noise + gain::SPEECH_MARGIN_DB {
        "silent"
    } else if peak < -30.0 {
        "quiet"
    } else {
        "ok"
    };
    LevelCheck { device: clip.device.clone(), peak_dbfs: round1(peak), noise_dbfs: round1(noise), verdict }
}

fn round1(x: f32) -> f32 {
    (x * 10.0).round() / 10.0
}

/// One calibration in progress from the window: a silent clip, then one clip per phrase.
#[derive(Debug, Default)]
pub struct Run {
    device: Option<String>,
    rate: u32,
    noise: Option<Vec<f32>>,
    speech: Vec<Vec<f32>>,
}

impl Run {
    /// Index of the next clip: 0 is silence, 1..=3 the phrases.
    pub fn step(&self) -> usize {
        usize::from(self.noise.is_some()) + self.speech.len()
    }
    pub fn done(&self) -> bool {
        self.step() > PHRASES.len()
    }
    /// What to say (or not) for the next clip, and for how long it records.
    pub fn next(&self) -> Option<(&'static str, u64)> {
        match self.step() {
            0 => Some(("", SILENCE_MS)),
            n if n <= PHRASES.len() => Some((PHRASES[n - 1], PHRASE_MS)),
            _ => None,
        }
    }
    fn add(&mut self, clip: Clip) -> Result<()> {
        ensure!(!self.done(), "calibration already has every clip");
        match &self.device {
            Some(d) if *d != clip.device => bail!("the microphone changed to {} during calibration", clip.device),
            Some(_) => ensure!(clip.rate == self.rate, "the microphone's sample rate changed"),
            None => {
                self.device = Some(clip.device.clone());
                self.rate = clip.rate;
            }
        }
        if self.noise.is_none() {
            self.noise = Some(clip.samples);
        } else {
            self.speech.push(clip.samples);
        }
        Ok(())
    }
    fn finish(&self, store: &Store) -> Result<(String, Calibration)> {
        ensure!(self.done(), "calibration needs silence and all {} phrases first", PHRASES.len());
        let device = self.device.clone().unwrap_or_else(|| DEFAULT_DEVICE.into());
        let c = calibrate_and_store(store, &device, self.rate, self.noise.as_deref().unwrap_or(&[]), &self.speech)?;
        Ok((device, c))
    }
}

/// Something that records a clip of a given length from the chosen mic.
pub type RecordFn<'a> = &'a dyn Fn(u64) -> Result<Clip>;

fn step_json(run: &Run) -> Value {
    match run.next() {
        Some((phrase, ms)) => json!({"step": run.step(), "steps": PHRASES.len() + 1, "phrase": phrase, "ms": ms}),
        None => json!({"step": run.step(), "steps": PHRASES.len() + 1, "phrase": null, "ms": 0}),
    }
}

fn err(e: impl std::fmt::Display) -> Value {
    json!({"ok": false, "error": e.to_string()})
}

/// The window's calibration and mic-check commands, over the control API:
/// - `calibrate_start`: forget any run in progress; reply with the first step;
/// - `calibrate_record`: record the next clip; reply with its level and the next step;
/// - `calibrate_finish`: compute and store the gate; reply with the result;
/// - `calibrate_cancel`;
/// - `mic_check`: record a short clip and say whether speech is loud enough.
pub fn handle(cmd: &str, run: &Mutex<Option<Run>>, record: RecordFn, store: &dyn Fn() -> Result<Store>) -> Value {
    let mut g = run.lock().unwrap();
    match cmd {
        "calibrate_start" => {
            let r = Run::default();
            let v = step_json(&r);
            *g = Some(r);
            json!({"ok": true, "phrases": PHRASES, "next": v})
        }
        "calibrate_record" => {
            let Some(r) = g.as_mut() else { return err("calibration hasn't started") };
            let Some((_, ms)) = r.next() else { return err("calibration already has every clip") };
            let clip = match record(ms) {
                Ok(c) => c,
                Err(e) => return err(format!("{e:#}")),
            };
            let level = level_check(&clip);
            if let Err(e) = r.add(clip) {
                return err(e);
            }
            json!({"ok": true, "level": level, "next": step_json(r)})
        }
        "calibrate_finish" => {
            let Some(r) = g.as_ref() else { return err("calibration hasn't started") };
            let res = store().and_then(|s| r.finish(&s));
            match res {
                Ok((device, c)) => {
                    *g = None;
                    json!({
                        "ok": true,
                        "device": device,
                        "noise_dbfs": round1(c.noise_dbfs),
                        "speech_peak_dbfs": round1(c.speech_peak_dbfs),
                        "min_level_dbfs": round1(c.min_level_dbfs),
                        "gain_db": round1(c.gain_db),
                    })
                }
                Err(e) => err(format!("{e:#}")),
            }
        }
        "calibrate_cancel" => {
            *g = None;
            json!({"ok": true})
        }
        "mic_check" => match record(PHRASE_MS) {
            Ok(c) => json!({"ok": true, "level": level_check(&c)}),
            Err(e) => err(format!("{e:#}")),
        },
        c => err(format!("unknown command {c:?}")),
    }
}

/// Record `ms` of audio from the `[voice] mic` device (Windows).
#[cfg(windows)]
pub fn record_mic(want: &str, ms: u64) -> Result<Clip> {
    use astromech_voice::mic::Recorder;
    use std::time::{Duration, Instant};
    let want = want.trim();
    let m = Recorder::start_named((!want.is_empty()).then_some(want))?;
    m.wait_after(Instant::now(), Duration::from_millis(1000));
    let from = m.position();
    std::thread::sleep(Duration::from_millis(ms));
    let samples = m.take(from, m.position());
    ensure!(!samples.is_empty(), "no audio from {} (is it muted or in use?)", m.device_name());
    Ok(Clip { device: m.device_name(), rate: m.sample_rate(), samples })
}

#[cfg(not(windows))]
pub fn record_mic(_want: &str, _ms: u64) -> Result<Clip> {
    bail!("recording needs Windows")
}

/// Recording devices for the mic picker: names, the Windows default, and which one the
/// `[voice] mic` setting picks.
pub fn mic_list(want: &str) -> Value {
    #[cfg(windows)]
    let (names, default) =
        (astromech_voice::mic::Recorder::list_names(), astromech_voice::mic::Recorder::default_device_name());
    #[cfg(not(windows))]
    let (names, default): (Vec<String>, Option<String>) = (vec![], None);
    let chosen = gain::pick_name(want, &names);
    json!({"ok": true, "names": names, "default": default, "setting": want.trim(), "chosen": chosen})
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(ms: u64, amp: f32) -> Vec<f32> {
        (0..16 * ms).map(|i| amp * (i as f32 * 0.2).sin()).collect()
    }

    fn write_wav(p: &Path, s: &[f32]) {
        astromech_voice::wav::write_mono_i16(p, 16000, s).unwrap();
    }

    fn settings(st: &Store, device: &str) -> Vec<Option<String>> {
        ["min_level_dbfs", "noise_dbfs", "speech_peak_dbfs"]
            .iter()
            .map(|k| st.get(&format!("mic.{device}.{k}")).unwrap())
            .collect()
    }

    /// Done-when for astromech-ny3y: the window's calibration (record → finish over the control
    /// commands) and `--calibrate-files` store the same gate, under the same device key, for the
    /// same audio.
    #[test]
    fn window_and_cli_store_the_same_gate() {
        let d = tempfile::tempdir().unwrap();
        let silence = vec![0.0004f32; 16 * 2000];
        let phrases = [tone(900, 0.02), tone(700, 0.03), tone(1100, 0.015)];
        let mut files = vec![d.path().join("silent.wav")];
        write_wav(&files[0], &silence);
        for (i, p) in phrases.iter().enumerate() {
            files.push(d.path().join(format!("p{i}.wav")));
            write_wav(&files[i + 1], p);
        }
        // The WAVs are 16-bit; give the window path exactly what the CLI reads back.
        let (rate, noise, speech) = read_wavs(&files).unwrap();
        let device = "Microphone (Example USB Mic)";

        let cli = Store::in_memory().unwrap();
        calibrate_and_store(&cli, &device_name("example usb", &[device.to_string()], None), rate, &noise, &speech)
            .unwrap();

        let win = Store::open(&d.path().join("win.sqlite")).unwrap();
        let clips: Vec<Vec<f32>> = std::iter::once(noise.clone()).chain(speech.iter().cloned()).collect();
        let next = std::cell::Cell::new(0usize);
        let record = |_ms: u64| {
            let i = next.get();
            next.set(i + 1);
            Ok(Clip { device: device.into(), rate, samples: clips[i].clone() })
        };
        let run = Mutex::new(None);
        let open = || Ok(Store::open(&d.path().join("win.sqlite"))?);
        assert_eq!(handle("calibrate_start", &run, &record, &open)["next"]["step"], 0);
        for i in 0..4 {
            let v = handle("calibrate_record", &run, &record, &open);
            assert_eq!(v["ok"], true, "{v}");
            assert_eq!(v["next"]["step"], i + 1);
        }
        let v = handle("calibrate_finish", &run, &record, &open);
        assert_eq!(v["ok"], true, "{v}");
        assert_eq!(v["device"], device);

        assert!(settings(&cli, device).iter().all(Option::is_some));
        assert_eq!(settings(&win, device), settings(&cli, device));
        // ...and the session reads it back for the device it opens.
        assert_eq!(stored_gate(&win, device), stored_gate(&cli, device));
        assert!(stored_gate(&win, device).is_some());
    }

    #[test]
    fn device_name_matches_what_the_session_opens() {
        let names = vec!["Microphone (Example USB Mic)".to_string(), "Headset Microphone (Example VR)".to_string()];
        assert_eq!(device_name("usb mic", &names, None), names[0]);
        assert_eq!(device_name("", &names, Some(&names[1])), names[1]);
        // No device list (CLI off Windows, or the device is unplugged): the setting as typed.
        assert_eq!(device_name(" Example ", &[], None), "Example");
        assert_eq!(device_name("", &[], None), DEFAULT_DEVICE);
    }

    #[test]
    fn steps_and_errors() {
        let run = Mutex::new(None);
        let open = || Ok(Store::in_memory()?);
        let no_mic = |_ms: u64| -> Result<Clip> { bail!("recording needs Windows") };
        assert_eq!(handle("calibrate_record", &run, &no_mic, &open)["ok"], false);
        let v = handle("calibrate_start", &run, &no_mic, &open);
        assert_eq!(v["phrases"][0], "gear down");
        assert_eq!(v["next"]["ms"], SILENCE_MS);
        let v = handle("calibrate_record", &run, &no_mic, &open);
        assert_eq!(v["error"], "recording needs Windows");
        assert_eq!(handle("calibrate_finish", &run, &no_mic, &open)["ok"], false);
        assert_eq!(handle("mic_check", &run, &no_mic, &open)["ok"], false);
        // Switching devices mid-run is refused.
        let n = std::cell::Cell::new(0);
        let flaky = |_ms: u64| {
            n.set(n.get() + 1);
            Ok(Clip { device: format!("mic {}", n.get()), rate: 16000, samples: tone(100, 0.1) })
        };
        handle("calibrate_start", &run, &flaky, &open);
        assert_eq!(handle("calibrate_record", &run, &flaky, &open)["ok"], true);
        let v = handle("calibrate_record", &run, &flaky, &open);
        assert!(v["error"].as_str().unwrap().contains("changed"), "{v}");
        assert_eq!(handle("calibrate_cancel", &run, &flaky, &open)["ok"], true);
        assert!(run.lock().unwrap().is_none());
    }

    #[test]
    fn level_check_verdicts() {
        let clip = |s: Vec<f32>| Clip { device: "m".into(), rate: 16000, samples: s };
        let mut speech = vec![0.0005f32; 16 * 500];
        speech.extend(tone(800, 0.3));
        assert_eq!(level_check(&clip(speech)).verdict, "ok");
        let mut quiet = vec![0.0005f32; 16 * 500];
        quiet.extend(tone(800, 0.011)); // about −39 dBFS, the VR headset case
        assert_eq!(level_check(&clip(quiet)).verdict, "quiet");
        assert_eq!(level_check(&clip(vec![0.0005f32; 16 * 2000])).verdict, "silent");
        assert_eq!(level_check(&clip(vec![])).verdict, "silent");
    }
}
