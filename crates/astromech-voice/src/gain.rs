//! Per-clip gain normalisation and mic calibration (V1-DESIGN.md §6.2).
//!
//! After the existing gate: measure speech (loudest 30 ms frames above the noise floor), apply
//! gain so those peaks sit near **−6 dBFS** (max **+30 dB**), with a soft limiter. Frames under
//! the noise floor + 6 dB are left alone so hiss is not turned into words.

/// Target peak for speech frames, dBFS.
pub const TARGET_PEAK_DBFS: f32 = -6.0;
/// Never boost more than this.
pub const MAX_GAIN_DB: f32 = 30.0;
/// Frames this far above the noise floor count as speech (and receive gain).
pub const SPEECH_MARGIN_DB: f32 = 6.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Normalised {
    pub samples: Vec<f32>,
    pub gain_db: f32,
    pub speech_peak_dbfs: f32,
    pub noise_dbfs: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Calibration {
    pub noise_dbfs: f32,
    pub speech_peak_dbfs: f32,
    /// Gate threshold to store per device: between noise and speech.
    pub min_level_dbfs: f32,
    /// Suggested extra boost if the automatic per-clip cap is not enough (informational).
    pub gain_db: f32,
}

fn dbfs(rms: f64) -> f32 {
    if rms <= 1e-9 {
        -120.0
    } else {
        (20.0 * rms.log10()) as f32
    }
}

fn rms(s: &[f32]) -> f64 {
    if s.is_empty() {
        0.0
    } else {
        (s.iter().map(|&x| (x as f64).powi(2)).sum::<f64>() / s.len() as f64).sqrt()
    }
}

/// RMS of each 30 ms frame, dBFS.
pub fn frame_dbfs(samples: &[f32], rate: u32) -> Vec<f32> {
    let frame = (rate.max(1) as usize * 30 / 1000).max(1);
    samples.chunks(frame).map(|c| dbfs(rms(c))).collect()
}

/// Noise floor: median of the quietest fifth of frames (or the quietest frame if few).
pub fn noise_floor_dbfs(frames: &[f32]) -> f32 {
    if frames.is_empty() {
        return -120.0;
    }
    let mut v = frames.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let n = (v.len() / 5).max(1);
    let slice = &v[..n];
    slice[slice.len() / 2]
}

fn limiter(x: f32) -> f32 {
    // Soft knee around 0.89 (−1 dBFS).
    const KNEE: f32 = 0.89;
    let a = x.abs();
    if a <= KNEE {
        x
    } else {
        let s = x.signum();
        let t = ((a - KNEE) / (1.0 - KNEE)).clamp(0.0, 4.0);
        s * (KNEE + (1.0 - KNEE) * t.tanh())
    }
}

/// Boost speech frames toward −6 dBFS. Quiet frames stay put.
pub fn normalise(samples: &[f32], rate: u32) -> Normalised {
    let frames = frame_dbfs(samples, rate);
    let noise = noise_floor_dbfs(&frames);
    let speech_cut = noise + SPEECH_MARGIN_DB;
    let speech_peak = frames.iter().copied().filter(|f| *f >= speech_cut).fold(-120.0f32, f32::max);
    if speech_peak <= -119.0 {
        return Normalised { samples: samples.to_vec(), gain_db: 0.0, speech_peak_dbfs: -120.0, noise_dbfs: noise };
    }
    let gain_db = (TARGET_PEAK_DBFS - speech_peak).clamp(0.0, MAX_GAIN_DB);
    let lin = 10f32.powf(gain_db / 20.0);
    let frame = (rate.max(1) as usize * 30 / 1000).max(1);
    let mut out = Vec::with_capacity(samples.len());
    for (i, chunk) in samples.chunks(frame).enumerate() {
        let loud = frames.get(i).copied().unwrap_or(-120.0) >= speech_cut;
        for &s in chunk {
            out.push(if loud { limiter(s * lin) } else { s });
        }
    }
    Normalised { samples: out, gain_db, speech_peak_dbfs: speech_peak, noise_dbfs: noise }
}

/// Three-phrase calibration: a silent clip plus speech clips (V1-DESIGN.md §6.2 step 3).
pub fn calibrate(noise: &[f32], speech: &[&[f32]], rate: u32) -> Calibration {
    let noise_dbfs = noise_floor_dbfs(&frame_dbfs(noise, rate));
    let mut speech_peak = -120.0f32;
    for c in speech {
        for f in frame_dbfs(c, rate) {
            speech_peak = speech_peak.max(f);
        }
    }
    let gate = (noise_dbfs + SPEECH_MARGIN_DB).min(speech_peak - 6.0);
    let min_level_dbfs = gate.clamp(-60.0, -25.0);
    let gain_db = (TARGET_PEAK_DBFS - speech_peak).clamp(0.0, MAX_GAIN_DB);
    Calibration { noise_dbfs, speech_peak_dbfs: speech_peak, min_level_dbfs, gain_db }
}

/// Pick an input device by exact (case-insensitive) name, else substring.
pub fn pick_name(want: &str, names: &[String]) -> Option<String> {
    let want = want.trim();
    if want.is_empty() {
        return None;
    }
    let lower = want.to_lowercase();
    names
        .iter()
        .find(|n| n.eq_ignore_ascii_case(want))
        .or_else(|| names.iter().find(|n| n.to_lowercase().contains(&lower)))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(ms: u64, amp: f32) -> Vec<f32> {
        (0..16 * ms).map(|i| amp * (i as f32 * 0.2).sin()).collect()
    }

    #[test]
    fn quiet_speech_is_boosted_hiss_is_not() {
        let mut clip = vec![0.0005f32; 16 * 150]; // ~−66 dBFS hiss
        clip.extend(tone(400, 0.011)); // ~−39 dBFS, the VR case
        let n = normalise(&clip, 16000);
        assert!(n.gain_db > 25.0 && n.gain_db <= MAX_GAIN_DB, "gain {}", n.gain_db);
        let speech = &n.samples[16 * 150..];
        let peak = speech.iter().fold(0.0f32, |a, x| a.max(x.abs()));
        assert!(peak > 0.3 && peak < 1.0, "peak {peak}");
        let hiss = n.samples[..16 * 150].iter().fold(0.0f32, |a, x| a.max(x.abs()));
        assert!(hiss < 0.002, "hiss was amplified: {hiss}");
    }

    #[test]
    fn already_loud_is_not_boosted() {
        let clip = tone(400, 0.5);
        let n = normalise(&clip, 16000);
        assert_eq!(n.gain_db, 0.0);
    }

    #[test]
    fn calibration_sets_gate_between_noise_and_speech() {
        let silent = vec![0.0004f32; 16 * 1000];
        let speech = tone(800, 0.02);
        let c = calibrate(&silent, &[&speech], 16000);
        assert!(c.noise_dbfs < -60.0, "{}", c.noise_dbfs);
        assert!(c.speech_peak_dbfs > c.min_level_dbfs, "{c:?}");
        assert!(c.min_level_dbfs > c.noise_dbfs, "{c:?}");
        assert!(c.gain_db > 0.0);
    }

    #[test]
    fn pick_name_exact_then_substring() {
        let names = vec!["Microphone (BlackShark V2)".into(), "Steam Streaming Microphone".into()];
        assert!(pick_name("BlackShark", &names).unwrap().contains("BlackShark"));
        assert_eq!(pick_name("steam streaming microphone", &names).as_deref(), Some("Steam Streaming Microphone"));
        assert!(pick_name("not-a-mic", &names).is_none());
        assert!(pick_name("", &names).is_none());
    }
}
