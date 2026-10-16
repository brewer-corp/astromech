//! # astromech-core
//!
//! The long-running core process (ARCHITECTURE.md §18, V1-DESIGN.md §7).
//! - [`app`]: the always-running core (M0): tray, single instance, start with Windows, game
//!   detection with idle/active sessions, the app window, crash marker and diagnostics;
//! - [`session`]: one active session, everything that runs while Elite does;
//! - [`pipeline`]: journal tail (with file rollover) + Status.json watching, feeding
//!   [`hub::StateHub`]; read positions persist in `astromech-store`;
//! - [`health`]: the binds health check at startup and on every Bindings-folder change;
//! - [`assistant`]: transcript → fuzzy grammar → tools → policy → executor;
//! - [`lookups`]: station lookups by voice (Spansh, with the app's remembered filters);
//! - [`actions`]: optimistic confirmation and background verification against Status.json;
//! - [`tester`]: the voice command tester's dry run (nothing pressed) and its live run;
//! - [`speech`]: Kokoro TTS / Parakeet STT through the voice worker, or console fallbacks;
//! - [`calibration`]: the per-device mic gate, shared by `--calibrate-files` and the window;
//! - [`setup`]: onboarding's checks and the config edits the window makes;
//! - [`config`]: the `astromech.toml` file (PTT and kill-switch buttons, folders, timings);
//! - [`logging`], [`crash`], [`diagnostics`], [`control`], [`instance`], [`autostart`],
//!   [`gamewatch`], [`update`]: the M0 infrastructure pieces.

pub mod actions;
pub mod app;
pub mod assistant;
pub mod autostart;
pub mod calibration;
pub mod config;
pub mod control;
pub mod controls;
pub mod crash;
pub mod diagnostics;
pub mod eval;
pub mod gamewatch;
pub mod health;
pub mod hub;
pub mod instance;
pub mod logging;
pub mod lookups;
pub mod pipeline;
pub mod session;
pub mod setup;
pub mod speech;
pub mod tester;
#[cfg(windows)]
pub mod tray;
pub mod update;
