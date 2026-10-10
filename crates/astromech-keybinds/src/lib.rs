//! # astromech-keybinds
//!
//! Everything about Elite Dangerous keybinding files (KEYBINDS.md):
//! - [`binds`]: `.binds` XML, `StartPreset*.start`, `DeviceMappings.xml` parsing; Elite key names.
//! - [`devices`]: connected controller inventory (HID + XInput on Windows; JSON fakes for tests).
//! - [`game`]: locating the Bindings folder, game installs (Steam/Epic/Frontier) and logs.
//! - [`doctor`]: the **health check** (KEYBINDS.md §7.6): device match, duplicate PresetName,
//!   version suffix, XML/encoding, subfolders and look-alikes, StartPreset resolution,
//!   `Joy_33+` binds, `BindingLoadingErrors.log`.
//! - [`report`]: console/HTML rendering and username redaction.
//!
//! Seeded from the `k0-binds-doctor` spike (spikes/k0-binds-doctor). **Read-only**: nothing
//! here writes to game folders.

pub mod binds;
pub mod devices;
pub mod doctor;
pub mod game;
pub mod report;
