//! # astromech-shell
//!
//! The app window (docs/V1-DESIGN.md §7.1): a thin Tauri v2 shell around the Svelte app in
//! `app/`, in `window.rs`.
//!
//! Tauri is behind the `window` feature, which `tauri.conf.json` (`build.features`) turns on for
//! `pnpm tauri build` and `tauri dev`. Without it, as in a plain `cargo build --workspace` or
//! `cargo test --workspace`, this is a stub: on Linux, Tauri needs the webkit2gtk system
//! libraries, and the core's checks shouldn't.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "window")]
mod window;

#[cfg(feature = "window")]
fn main() {
    window::main();
}

#[cfg(not(feature = "window"))]
fn main() {
    eprintln!("astromech-shell was built without the window: build it with `pnpm tauri build` or `--features window`");
    std::process::exit(2);
}
