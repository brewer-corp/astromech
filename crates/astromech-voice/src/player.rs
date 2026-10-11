//! WAV playback via `PlaySoundW` (Windows).

use anyhow::{bail, Result};
use std::path::Path;
use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_FILENAME, SND_NODEFAULT, SND_SYNC};

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Play a WAV file and wait until it ends.
pub fn play_wav(path: &Path) -> Result<()> {
    let w = wide(&path.display().to_string());
    let ok = unsafe { PlaySoundW(w.as_ptr(), std::ptr::null_mut(), SND_FILENAME | SND_SYNC | SND_NODEFAULT) };
    if ok == 0 {
        bail!("PlaySoundW failed for {}", path.display());
    }
    Ok(())
}
