//! WAV helpers (mono f32 in, 16-bit PCM out).

use anyhow::{Context, Result};
use std::path::Path;

pub fn write_mono_i16(path: &Path, sample_rate: u32, samples: &[f32]) -> Result<()> {
    let spec =
        hound::WavSpec { channels: 1, sample_rate, bits_per_sample: 16, sample_format: hound::SampleFormat::Int };
    let mut w = hound::WavWriter::create(path, spec).with_context(|| format!("create {}", path.display()))?;
    for &s in samples {
        w.write_sample((s.clamp(-1.0, 1.0) * 32767.0).round() as i16)?;
    }
    w.finalize()?;
    Ok(())
}

/// Read a WAV file as mono f32 (channels averaged). Returns (sample_rate, samples).
pub fn read_mono_f32(path: &Path) -> Result<(u32, Vec<f32>)> {
    let mut r = hound::WavReader::open(path).with_context(|| format!("open {}", path.display()))?;
    let spec = r.spec();
    let ch = spec.channels.max(1) as usize;
    let interleaved: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => r.samples::<f32>().collect::<Result<_, _>>()?,
        hound::SampleFormat::Int => {
            let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
            r.samples::<i32>().map(|s| s.map(|v| v as f32 * scale)).collect::<Result<_, _>>()?
        }
    };
    let mono = interleaved.chunks(ch).map(|c| c.iter().sum::<f32>() / ch as f32).collect();
    Ok((spec.sample_rate, mono))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("a.wav");
        let s: Vec<f32> = (0..1600).map(|i| (i as f32 / 10.0).sin() * 0.5).collect();
        write_mono_i16(&p, 16000, &s).unwrap();
        let (sr, back) = read_mono_f32(&p).unwrap();
        assert_eq!(sr, 16000);
        assert_eq!(back.len(), s.len());
        assert!(s.iter().zip(&back).all(|(a, b)| (a - b).abs() < 1e-3));
    }
}
