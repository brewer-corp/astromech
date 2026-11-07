//! # astromech-app
//!
//! `astromech-core.exe`, the one app (decision astromech-6i8s): the core, its tray and the
//! window in one process. Its command line is the core's ([`astromech_core::cli`]); the app it
//! runs is the Tauri host in `host.rs`, which builds the core with `astromech_core::app` and
//! adds the tray (`tray.rs`) and the window around it.
//!
//! Tauri is behind the `window` feature, which `tauri.conf.json` (`build.features`) turns on for
//! `pnpm tauri build` and `tauri dev`. Without it, as in a plain `cargo build --workspace` or
//! `cargo test --workspace`, the same binary runs the core headless ([`astromech_core::app::run`]):
//! on Linux, Tauri needs the webkit2gtk system libraries, and the core's checks and scripts
//! shouldn't.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "window")]
mod host;
#[cfg(feature = "window")]
mod tray;

#[cfg(feature = "window")]
fn main() {
    astromech_core::cli::main(host::run)
}

#[cfg(not(feature = "window"))]
fn main() {
    astromech_core::cli::main(astromech_core::app::run)
}
