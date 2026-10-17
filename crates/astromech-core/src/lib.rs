//! # astromech-core
//!
//! The long-running core process (ARCHITECTURE.md §18, V1-DESIGN.md §7).
//! - [`app`]: the always-running core (M0): tray, single instance, start with Windows, game
//!   detection with idle/active sessions, the app window, crash marker and diagnostics;
//! - [`session`]: one active session, everything that runs while Elite does;
//! - [`pipeline`]: journal tail (with file rollover) + Status.json watching, feeding
//!   [`hub::StateHub`] and, when switched on, the EDDN feed; read positions persist in `astromech-store`;
//! - [`history`]: keeps the journal history database current (backfill, then new lines);
//! - [`health`]: the binds health check at startup and on every Bindings-folder change;
//! - [`assistant`]: transcript → fuzzy grammar → tools → policy → executor;
//! - [`lookups`]: station lookups by voice (Spansh, with the app's remembered filters);
//! - [`actions`]: optimistic confirmation and background verification against Status.json;
//! - [`tester`]: the voice command tester's dry run (nothing pressed) and its live run;
//! - [`trade`]: trade questions by voice (where to sell or buy a commodity, Ardent Insight);
//! - [`biology`]: biology other players reported (Spansh) for the arrival briefing, and the
//!   Biology page's view of the system (`exobiology`);
//! - [`speech`]: Kokoro TTS / Parakeet STT through the voice worker, or console fallbacks;
//! - [`route`]: following a multi-stop route; the next waypoint goes on the [`clipboard`];
//! - [`callouts`]: delivers the reaction engine's callouts (speech queue, app toasts, Windows
//!   notifications);
//! - [`calibration`]: the per-device mic gate, shared by `--calibrate-files` and the window;
//! - [`setup`]: onboarding's checks and the config edits the window makes;
//! - [`config`]: the `astromech.toml` file (PTT and kill-switch buttons, folders, timings);
//! - [`app::eddn_log`]: the EDDN send log for the Diagnostics page (`astromech-eddn`);
//! - [`logging`], [`crash`], [`diagnostics`], [`control`], [`instance`], [`autostart`],
//!   [`gamewatch`], [`update`]: the M0 infrastructure pieces.

pub mod actions;
pub mod app;
pub mod assistant;
pub mod autostart;
pub mod biology;
pub mod calibration;
pub mod callouts;
pub mod clipboard;
pub mod colonisation;
pub mod config;
pub mod control;
pub mod controls;
pub mod crash;
pub mod diagnostics;
pub mod eval;
pub mod gamewatch;
pub mod health;
pub mod history;
pub mod hub;
pub mod instance;
pub mod logging;
pub mod lookups;
pub mod pipeline;
pub mod route;
pub mod session;
pub mod setup;
pub mod speech;
pub mod summary;
pub mod tester;
pub mod trade;
#[cfg(windows)]
pub mod tray;
pub mod update;
pub mod visits;
