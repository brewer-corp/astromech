//! # astromech-voice
//!
//! Local voice (ARCHITECTURE.md §5.10, S9 results in §17.6): Parakeet TDT speech-to-text and
//! Kokoro text-to-speech through sherpa-onnx, running in a separate worker process
//! (`astromech-voice-worker`, feature `sherpa`) that talks JSON lines over stdin/stdout.
//! - [`proto`]: the worker protocol.
//! - [`score`]: transcript normalisation and word error rate (S9 accuracy scoring).
//! - [`speakable`]: reading rules applied to text before TTS.
//! - [`wav`]: mono WAV I/O.
//! - [`colour`]: Cockpit PA treatment of Kokoro PCM (V1-DESIGN.md §6.4).
//! - [`gain`]: per-clip level correction and mic-name matching (V1-DESIGN.md §6.2).
//! - [`mixer`]: barge-in output queue (cpal on Windows).
//! - [`client`]: spawning and talking to the worker; [`models`]/[`catalog`]/[`download`]: the
//!   model files (S9 layout, SHA-256-verified downloads); `mic` (cpal) and `player`
//!   (PlaySoundW fallback) on Windows.
//!
//! Defaults from S9: CPU, **4 inference threads**. Fuzzy matching to the command grammar lives
//! in `astromech-intent` (planned).
//!
//! **Rule:** this crate never depends on `astromech-input`. Push-to-talk edges arrive from the
//! core; actions leave through `astromech-tools` → `astromech-policy`.

pub mod catalog;
pub mod client;
pub mod colour;
pub mod download;
pub mod gain;
#[cfg(windows)]
pub mod mic;
pub mod mixer;
pub mod models;
#[cfg(windows)]
pub mod player;
pub mod proto;
pub mod score;
pub mod speakable;
pub mod wav;

/// Default inference threads (S9 thread sweep on a Ryzen 9 7900X: 4 was best).
pub const DEFAULT_THREADS: i32 = 4;
