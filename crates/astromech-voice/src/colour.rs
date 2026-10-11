//! Voice colour (V1-DESIGN.md §6.4, ARCHITECTURE.md §5.10): treat Kokoro PCM as a cockpit PA
//! so replies sit in the same register as Elite's COVAS. Pure and deterministic; unit-tested
//! on synthetic tones. Natural is a no-op. Cockpit / Harsh run a short DSP chain and a pip.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Playback colour. Default **Cockpit**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Colour {
    Natural,
    #[default]
    Cockpit,
    Harsh,
}

impl Colour {
    pub fn as_str(self) -> &'static str {
        match self {
            Colour::Natural => "natural",
            Colour::Cockpit => "cockpit",
            Colour::Harsh => "harsh",
        }
    }
}

/// Kokoro `speed` to send the worker. A user-set speed (anything other than 1.0) is kept.
/// Otherwise Cockpit reads a little slower than conversation, Harsh a little more.
pub fn speed(colour: Colour, configured: f32) -> f32 {
    if (configured - 1.0).abs() > 0.001 {
        return configured;
    }
    match colour {
        Colour::Natural => 1.0,
        Colour::Cockpit => 0.94,
        Colour::Harsh => 0.92,
    }
}

/// Colour the body and, for Cockpit/Harsh, prepend the two-tone pip.
pub fn utterance(samples: &[f32], sr: u32, colour: Colour) -> Vec<f32> {
    let body = apply(samples, sr, colour);
    if colour == Colour::Natural {
        return body;
    }
    let mut out = pip(sr);
    out.extend(body);
    out
}

/// DSP only (no pip). Natural copies. Output is clamped to [-1, 1].
pub fn apply(samples: &[f32], sr: u32, colour: Colour) -> Vec<f32> {
    if samples.is_empty() {
        return vec![];
    }
    match colour {
        Colour::Natural => samples.to_vec(),
        Colour::Cockpit => chain(samples, sr, 4000.0, 1.15, false),
        Colour::Harsh => chain(samples, sr, 3500.0, 1.45, true),
    }
}

/// Two-tone pip (~90 ms): 880 Hz then 1320 Hz, faded, own asset (never game audio).
pub fn pip(sr: u32) -> Vec<f32> {
    let sr = sr.max(1);
    let tone = |hz: f64, ms: u32, amp: f32| -> Vec<f32> {
        let n = (sr as u64 * ms as u64 / 1000) as usize;
        let fade = ((sr as usize) * 5 / 1000).max(1).min(n / 2);
        (0..n)
            .map(|i| {
                let env = if i < fade {
                    i as f32 / fade as f32
                } else if i + fade >= n {
                    (n - 1 - i) as f32 / fade as f32
                } else {
                    1.0
                };
                amp * env * (2.0 * PI * hz * i as f64 / sr as f64).sin() as f32
            })
            .collect()
    };
    let gap = vec![0.0f32; (sr as usize * 10 / 1000).max(1)];
    let mut v = tone(880.0, 40, 0.22);
    v.extend(gap);
    v.extend(tone(1320.0, 40, 0.20));
    v
}

fn chain(samples: &[f32], sr: u32, lowpass_hz: f64, drive: f64, grit: bool) -> Vec<f32> {
    let sr_f = sr.max(1) as f64;
    let mut hp = Biquad::highpass(sr_f, 250.0, 0.707);
    let mut hp2 = Biquad::highpass(sr_f, 180.0, 0.707);
    let mut lp = Biquad::lowpass(sr_f, lowpass_hz.min(sr_f * 0.45), 0.707);
    let mut peak = Biquad::peaking(sr_f, 1700.0, 1.0, 4.0);
    let mut env = 0.0f64;
    let atk = (-1.0 / (0.005 * sr_f)).exp();
    let rel = (-1.0 / (0.050 * sr_f)).exp();
    let thresh = db_to_lin(-18.0);
    let ratio = 4.0;
    let makeup = db_to_lin(6.0);
    let delay = ((0.0015 * sr_f).round() as usize).max(1);
    let mut comb = vec![0.0f32; delay];
    let mut ci = 0usize;
    let mix = if grit { 0.28 } else { 0.16 };

    let mut out = Vec::with_capacity(samples.len());
    for &x in samples {
        let mut y = peak.process(lp.process(hp2.process(hp.process(x)))) as f64;
        let a = y.abs();
        env = if a > env { atk * env + (1.0 - atk) * a } else { rel * env + (1.0 - rel) * a };
        let g = if env > thresh { (thresh + (env - thresh) / ratio) / env.max(1e-9) } else { 1.0 };
        y *= g * makeup;
        y = (y * drive).tanh() / drive.tanh();
        let d = comb[ci] as f64;
        comb[ci] = y as f32;
        ci = (ci + 1) % delay;
        let z = (y + mix * d) as f32;
        out.push(z.clamp(-1.0, 1.0));
    }
    if grit {
        telephone(&mut out, sr);
    }
    out
}

/// Down to 16 kHz and back: extra grit for Harsh. No-op if already ≤ 16 kHz.
fn telephone(samples: &mut Vec<f32>, sr: u32) {
    if sr <= 16000 || samples.len() < 4 {
        return;
    }
    let down = resample(samples, sr, 16000);
    *samples = resample(&down, 16000, sr);
}

fn db_to_lin(db: f64) -> f64 {
    10f64.powf(db / 20.0)
}

struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    z1: f64,
    z2: f64,
}

impl Biquad {
    fn highpass(sr: f64, f: f64, q: f64) -> Self {
        let (_sin, cos, alpha) = rbj(sr, f, q);
        let b0 = (1.0 + cos) * 0.5;
        let b1 = -(1.0 + cos);
        let b2 = (1.0 + cos) * 0.5;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos;
        let a2 = 1.0 - alpha;
        Self::norm(b0, b1, b2, a0, a1, a2)
    }
    fn lowpass(sr: f64, f: f64, q: f64) -> Self {
        let (_sin, cos, alpha) = rbj(sr, f, q);
        let b0 = (1.0 - cos) * 0.5;
        let b1 = 1.0 - cos;
        let b2 = (1.0 - cos) * 0.5;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos;
        let a2 = 1.0 - alpha;
        Self::norm(b0, b1, b2, a0, a1, a2)
    }
    fn peaking(sr: f64, f: f64, q: f64, gain_db: f64) -> Self {
        let a = 10f64.powf(gain_db / 40.0);
        let (_sin, cos, alpha) = rbj(sr, f, q);
        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos;
        let a2 = 1.0 - alpha / a;
        Self::norm(b0, b1, b2, a0, a1, a2)
    }
    fn norm(b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64) -> Self {
        Self { b0: b0 / a0, b1: b1 / a0, b2: b2 / a0, a1: a1 / a0, a2: a2 / a0, z1: 0.0, z2: 0.0 }
    }
    fn process(&mut self, x: f32) -> f32 {
        let x = x as f64;
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y as f32
    }
}

fn rbj(sr: f64, f: f64, q: f64) -> (f64, f64, f64) {
    let w0 = 2.0 * PI * f / sr;
    let sin = w0.sin();
    let cos = w0.cos();
    let alpha = sin / (2.0 * q.max(0.01));
    (sin, cos, alpha)
}

/// Linear resample. Used by the mixer and the Harsh grit step.
pub fn resample(samples: &[f32], from: u32, to: u32) -> Vec<f32> {
    if samples.is_empty() {
        return vec![];
    }
    if from == 0 || to == 0 || from == to {
        return samples.to_vec();
    }
    let n = ((samples.len() as u64 * to as u64) / from as u64).max(1) as usize;
    if n == 1 {
        return vec![samples[0]];
    }
    let last = samples.len() - 1;
    let scale = last as f64 / (n - 1) as f64;
    (0..n)
        .map(|i| {
            let x = i as f64 * scale;
            let j = (x.floor() as usize).min(last);
            let f = (x - j as f64) as f32;
            let a = samples[j];
            let b = samples.get(j + 1).copied().unwrap_or(a);
            a + (b - a) * f
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rms(s: &[f32]) -> f32 {
        if s.is_empty() {
            return 0.0;
        }
        (s.iter().map(|&x| x * x).sum::<f32>() / s.len() as f32).sqrt()
    }

    fn sine(sr: u32, hz: f32, ms: u32, amp: f32) -> Vec<f32> {
        let n = sr as usize * ms as usize / 1000;
        (0..n).map(|i| amp * (2.0 * PI as f32 * hz * i as f32 / sr as f32).sin()).collect()
    }

    fn tail(s: &[f32], skip: usize) -> &[f32] {
        &s[skip.min(s.len())..]
    }

    #[test]
    fn natural_is_identity() {
        let s = sine(24000, 1000.0, 200, 0.5);
        let o = apply(&s, 24000, Colour::Natural);
        assert_eq!(o.len(), s.len());
        assert!(s.iter().zip(&o).all(|(a, b)| (a - b).abs() < 1e-6));
    }

    #[test]
    fn cockpit_kills_bass_and_air_keeps_presence() {
        let sr = 24000;
        let bass = apply(&sine(sr, 80.0, 400, 0.6), sr, Colour::Cockpit);
        let mid = apply(&sine(sr, 1000.0, 400, 0.6), sr, Colour::Cockpit);
        let air = apply(&sine(sr, 8000.0, 400, 0.6), sr, Colour::Cockpit);
        // Skip filter startup.
        let n = sr as usize / 20;
        let (b, m, a) = (rms(tail(&bass, n)), rms(tail(&mid, n)), rms(tail(&air, n)));
        assert!(m > 0.12, "1 kHz should remain, rms={m}");
        assert!(b < 0.35 * m, "80 Hz should be below presence, bass={b} mid={m}");
        assert!(a < 0.35 * m, "8 kHz should be below presence, air={a} mid={m}");
    }

    #[test]
    fn harsh_is_narrower_than_cockpit() {
        let sr = 24000;
        let t = sine(sr, 8000.0, 400, 0.6);
        let c = apply(&t, sr, Colour::Cockpit);
        let h = apply(&t, sr, Colour::Harsh);
        let n = sr as usize / 10;
        assert!(rms(tail(&h, n)) <= rms(tail(&c, n)) + 1e-4);
    }

    #[test]
    fn output_stays_in_range() {
        let s: Vec<f32> = (0..8000).map(|i| (i as f32 * 0.17).sin()).collect();
        for c in [Colour::Natural, Colour::Cockpit, Colour::Harsh] {
            let o = apply(&s, 24000, c);
            assert!(o.iter().all(|x| x.abs() <= 1.0), "{c:?}");
        }
    }

    #[test]
    fn utterance_adds_pip_except_natural() {
        let s = sine(24000, 1000.0, 100, 0.4);
        let n = utterance(&s, 24000, Colour::Natural);
        let c = utterance(&s, 24000, Colour::Cockpit);
        assert_eq!(n.len(), s.len());
        assert!(c.len() > s.len() + 24_000 * 80 / 1000);
        assert!(pip(24000).iter().any(|&x| x.abs() > 0.05));
    }

    #[test]
    fn default_speed() {
        assert_eq!(speed(Colour::Natural, 1.0), 1.0);
        assert!((speed(Colour::Cockpit, 1.0) - 0.94).abs() < 1e-6);
        assert_eq!(speed(Colour::Cockpit, 1.2), 1.2);
        assert_eq!(Colour::default(), Colour::Cockpit);
        assert_eq!(Colour::Cockpit.as_str(), "cockpit");
    }

    #[test]
    fn resample_roundtrip_length() {
        let s = sine(24000, 440.0, 100, 0.5);
        let up = resample(&s, 24000, 48000);
        assert!((up.len() as i32 - 4800).abs() <= 2);
        let back = resample(&up, 48000, 24000);
        assert!((back.len() as i32 - s.len() as i32).abs() <= 2);
    }
}
