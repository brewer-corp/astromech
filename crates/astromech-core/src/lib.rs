//! # astromech-core
//!
//! The long-running core process (ARCHITECTURE.md §18). In the gear-down slice it wires:
//! - [`pipeline`]: journal tail (with file rollover) + Status.json watching, notifications only,
//!   feeding [`hub::StateHub`] (the published `GameState` snapshots); read positions persist in
//!   `astromech-store`;
//! - [`health`]: the binds health check at startup and on every Bindings-folder change, plus the
//!   active ship preset's action → keyboard index;
//! - [`assistant`]: transcript → fuzzy grammar → tools → policy → executor, with
//!   "Did you mean …?" confirmation;
//! - [`actions`]: optimistic confirmation, background verification against Status.json
//!   (2 s timeout, one retry, spoken failure only);
//! - [`speech`]: Kokoro TTS / Parakeet STT through the voice worker, or console fallbacks;
//! - [`config`]: the `astromech.toml` file (PTT and kill-switch buttons, folders, timings).

pub mod actions;
pub mod assistant;
pub mod config;
pub mod controls;
pub mod health;
pub mod hub;
pub mod pipeline;
pub mod speech;
