//! Cue tones (generated in memory) and WAV playback via PlaySoundW.

use crate::s7::Cue;
use anyhow::{bail, Result};
use std::path::Path;
use std::sync::OnceLock;
use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FILENAME, SND_MEMORY, SND_NODEFAULT, SND_SYNC};

const RATE: u32 = 22050;

/// (frequency Hz, duration ms); frequency 0 = silence.
fn notes(c: Cue) -> &'static [(f32, u32)] {
    match c {
        Cue::Tick => &[(1000.0, 40)],
        Cue::Start => &[(523.0, 110), (659.0, 110), (784.0, 160)],
        Cue::Ptt => &[(880.0, 160)],
        Cue::Kill => &[(392.0, 110), (0.0, 70), (392.0, 110)],
        Cue::Phase => &[(660.0, 160), (990.0, 220)],
        Cue::Done => &[(784.0, 110), (659.0, 110), (523.0, 200)],
        Cue::Error => &[(196.0, 450)],
    }
}

pub fn tone_wav(seq: &[(f32, u32)]) -> Vec<u8> {
    let mut pcm: Vec<i16> = vec![];
    for &(f, dur) in seq {
        let n = (RATE as u64 * dur as u64 / 1000) as usize;
        for i in 0..n {
            let t = i as f32 / RATE as f32;
            // 8 ms fade in/out to avoid clicks
            let fade = ((i.min(n - i) as f32) / (RATE as f32 * 0.008)).min(1.0);
            let v = if f > 0.0 { (t * f * std::f32::consts::TAU).sin() * 0.35 * fade } else { 0.0 };
            pcm.push((v * 32767.0) as i16);
        }
    }
    let data_len = (pcm.len() * 2) as u32;
    let mut w = Vec::with_capacity(44 + data_len as usize);
    w.extend_from_slice(b"RIFF");
    w.extend_from_slice(&(36 + data_len).to_le_bytes());
    w.extend_from_slice(b"WAVEfmt ");
    w.extend_from_slice(&16u32.to_le_bytes());
    w.extend_from_slice(&1u16.to_le_bytes());
    w.extend_from_slice(&1u16.to_le_bytes());
    w.extend_from_slice(&RATE.to_le_bytes());
    w.extend_from_slice(&(RATE * 2).to_le_bytes());
    w.extend_from_slice(&2u16.to_le_bytes());
    w.extend_from_slice(&16u16.to_le_bytes());
    w.extend_from_slice(b"data");
    w.extend_from_slice(&data_len.to_le_bytes());
    for s in pcm {
        w.extend_from_slice(&s.to_le_bytes());
    }
    w
}

static CUES: OnceLock<Vec<(Cue, Vec<u8>)>> = OnceLock::new();

pub fn play_cue(c: Cue) {
    let all = CUES.get_or_init(|| [Cue::Tick, Cue::Start, Cue::Ptt, Cue::Kill, Cue::Phase, Cue::Done, Cue::Error].into_iter().map(|c| (c, tone_wav(notes(c)))).collect());
    if let Some((_, w)) = all.iter().find(|(k, _)| *k == c) {
        // The buffer is 'static, so asynchronous playback is safe.
        unsafe { PlaySoundW(w.as_ptr() as *const u16, std::ptr::null_mut(), SND_MEMORY | SND_ASYNC | SND_NODEFAULT) };
    }
}

pub struct WinPlayer;

impl crate::voice::Player for WinPlayer {
    fn play_wav(&self, path: &Path) -> Result<()> {
        let w = super::wide(&path.display().to_string());
        let ok = unsafe { PlaySoundW(w.as_ptr(), std::ptr::null_mut(), SND_FILENAME | SND_SYNC | SND_NODEFAULT) };
        if ok == 0 {
            bail!("PlaySoundW failed for {}", path.display());
        }
        Ok(())
    }
}
