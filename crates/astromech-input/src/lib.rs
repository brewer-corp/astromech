//! # astromech-input
//!
//! - [`listen`]: read-only background input. Raw Input `RIDEV_INPUTSINK` on a hidden window
//!   for HID joysticks/gamepads, keyboard and mouse buttons, plus XInput polling. Events carry
//!   Elite names (`Joy_N`, `Joy_POV1Up`, `Joy_XAxis`). S7 showed it works while Elite has focus
//!   (ARCHITECTURE.md §17.7). Seed of push-to-talk and the key tester; from
//!   `spikes/s7-input-listener`.
//! - [`exec`]: the executor. DirectInput scancode table, `SendInput` with scancodes, focus and
//!   UIPI integrity checks, a mock game for tests. From `spikes/s5-status-latency`.
//!
//! **Rule:** only `astromech-policy` may call [`exec`]. `astromech-voice` and `astromech-tools`
//! must not depend on this crate (checked by `scripts/check-deps.py`).

pub mod exec;
pub mod listen;

/// The game's window title.
pub const ELITE_TITLE: &str = "Elite - Dangerous (CLIENT)";
