//! # astromech-voice
//!
//! Local voice (ARCHITECTURE.md §5.10, S9 results in §17.6): Parakeet TDT speech-to-text and
//! Kokoro text-to-speech through sherpa-onnx, running in a separate worker process
//! (`astromech-voice-worker`, feature `sherpa`) that talks JSON lines over stdin/stdout.
//! - [`proto`]: the worker protocol.
//! - [`score`]: transcript normalisation and word error rate (S9 accuracy scoring).
//! - [`wav`]: mono WAV I/O.
//!
//! Defaults from S9: CPU, **4 inference threads**. Fuzzy matching to the command grammar lives
//! in `astromech-intent` (planned).
//!
//! **Rule:** this crate never depends on `astromech-input`. Push-to-talk edges arrive from the
//! core; actions leave through `astromech-tools` → `astromech-policy`.

pub mod proto;
pub mod score;
pub mod wav;

/// Default inference threads (S9 thread sweep on a Ryzen 9 7900X: 4 was best).
pub const DEFAULT_THREADS: i32 = 4;
