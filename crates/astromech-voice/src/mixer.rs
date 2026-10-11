//! Output queue for coloured TTS and earcons (ARCHITECTURE.md §5.10).
//!
//! `play` replaces whatever is queued (a new line cuts the previous). `cut` bumps a generation
//! counter and empties the queue so a push-to-talk press can barge in even if the TTS thread is
//! still rendering. Pulling from an empty queue yields silence, so a cpal callback can keep
//! running. Device output is Windows-only (cpal); tests use [`Mixer::silent`].

use crate::colour::resample;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

struct Inner {
    sr: AtomicU32,
    q: Mutex<VecDeque<f32>>,
    gen: AtomicU64,
}

/// Shared playback queue. Clone is cheap (`Arc`).
#[derive(Clone)]
pub struct Mixer {
    inner: Arc<Inner>,
    #[cfg(windows)]
    _stream: Arc<Mutex<Option<cpal::Stream>>>,
}

impl Mixer {
    /// Queue only: no device. `sr` is the rate `pull` emits.
    pub fn silent(sr: u32) -> Self {
        Self {
            inner: Arc::new(Inner {
                sr: AtomicU32::new(sr.max(1)),
                q: Mutex::new(VecDeque::new()),
                gen: AtomicU64::new(0),
            }),
            #[cfg(windows)]
            _stream: Arc::new(Mutex::new(None)),
        }
    }

    /// Try to open the default output device. Always returns a mixer; without a device, `play`
    /// still queues (and `cut` still works) so tests and headless runs are fine.
    pub fn with_default_output() -> Self {
        let m = Self::silent(48_000);
        #[cfg(windows)]
        {
            if let Some(stream) = open_output(m.inner.clone()) {
                *m._stream.lock().unwrap() = Some(stream);
            }
        }
        m
    }

    pub fn sample_rate(&self) -> u32 {
        self.inner.sr.load(Ordering::SeqCst)
    }

    pub fn has_device(&self) -> bool {
        #[cfg(windows)]
        {
            self._stream.lock().unwrap().is_some()
        }
        #[cfg(not(windows))]
        {
            false
        }
    }

    /// Token of the current generation. Capture before a slow render; pass it to [`play`].
    pub fn token(&self) -> u64 {
        self.inner.gen.load(Ordering::SeqCst)
    }

    /// Stop playback immediately. Further [`play`] calls with an older token are ignored.
    pub fn cut(&self) {
        self.inner.gen.fetch_add(1, Ordering::SeqCst);
        self.inner.q.lock().unwrap().clear();
    }

    /// Replace the queue with `samples` unless `token` is stale (a `cut` happened since).
    pub fn play(&self, token: u64, samples: &[f32], sr: u32) {
        if token != self.inner.gen.load(Ordering::SeqCst) {
            return;
        }
        let data = resample(samples, sr, self.sample_rate());
        let mut q = self.inner.q.lock().unwrap();
        if token != self.inner.gen.load(Ordering::SeqCst) {
            return;
        }
        q.clear();
        q.extend(data);
    }

    /// Next `n` samples, zeros if the queue is empty. Used by tests and the cpal callback.
    pub fn pull(&self, n: usize) -> Vec<f32> {
        let mut q = self.inner.q.lock().unwrap();
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(q.pop_front().unwrap_or(0.0));
        }
        out
    }

    pub fn queued(&self) -> usize {
        self.inner.q.lock().unwrap().len()
    }
}

#[cfg(windows)]
fn fill(inner: &Inner, dst: &mut [f32], ch: usize) {
    let mut q = inner.q.lock().unwrap();
    let frames = dst.len() / ch.max(1);
    for i in 0..frames {
        let s = q.pop_front().unwrap_or(0.0);
        for c in 0..ch.max(1) {
            dst[i * ch + c] = s;
        }
    }
}

#[cfg(windows)]
fn open_output(inner: Arc<Inner>) -> Option<cpal::Stream> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let host = cpal::default_host();
    let dev = host.default_output_device()?;
    let sup = dev.default_output_config().ok()?;
    let rate = sup.sample_rate().0;
    let ch = sup.channels().max(1) as usize;
    let cfg: cpal::StreamConfig = sup.config();
    inner.sr.store(rate.max(1), Ordering::SeqCst);
    let err = |e: cpal::Error| eprintln!("output stream: {e}");
    let stream = match sup.sample_format() {
        cpal::SampleFormat::F32 => {
            let i3 = inner.clone();
            dev.build_output_stream::<f32, _, _>(cfg, move |d: &mut [f32], _| fill(&i3, d, ch), err, None)
        }
        cpal::SampleFormat::I16 => {
            let i3 = inner.clone();
            dev.build_output_stream::<i16, _, _>(
                cfg,
                move |d: &mut [i16], _| {
                    let mut tmp = vec![0.0f32; d.len()];
                    fill(&i3, &mut tmp, ch);
                    for (o, s) in d.iter_mut().zip(tmp) {
                        *o = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
                    }
                },
                err,
                None,
            )
        }
        _ => return None,
    }
    .ok()?;
    stream.play().ok()?;
    Some(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_then_cut_is_silence() {
        let m = Mixer::silent(16_000);
        let t = m.token();
        m.play(t, &[1.0; 1000], 16_000);
        assert_eq!(m.queued(), 1000);
        let chunk = m.pull(100);
        assert!(chunk.iter().all(|&x| x == 1.0));
        m.cut();
        assert_eq!(m.queued(), 0);
        assert!(m.pull(16).iter().all(|&x| x == 0.0));
        m.play(t, &[0.5; 10], 16_000);
        assert_eq!(m.queued(), 0, "stale token after cut");
        let t2 = m.token();
        m.play(t2, &[0.5; 8], 16_000);
        assert_eq!(m.queued(), 8);
    }

    #[test]
    fn new_play_replaces() {
        let m = Mixer::silent(8_000);
        let t = m.token();
        m.play(t, &[1.0; 50], 8_000);
        m.play(t, &[0.25; 4], 8_000);
        assert_eq!(m.pull(4), vec![0.25, 0.25, 0.25, 0.25]);
        assert_eq!(m.queued(), 0);
    }
}
