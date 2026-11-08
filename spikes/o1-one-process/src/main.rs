//! Spike O1 (Atelier astromech-wgtr): can one Tauri process hold astromech's tray with no
//! window, survive the window crashing, free WebView2 when the window closes, and leave
//! astromech's own Raw Input listener receiving keys? Throwaway code: scripts/one_process_probe.py
//! drove it on Windows CI, in a workflow deleted once its checks moved to scripts/webview_cost.py
//! (astromech-xz7i). The probe was removed with crash restart (astromech-lkwj).
//!
//! astromech-spike [--background] [--route X] [--plain-tray]
//!                 [--sink-before-build | --sink-after-build] [--crash-hook] [--restarted]
//!
//! - `--background`: start with no window (Start with Windows). Without it the window opens.
//! - `--route X`: open the window at page X. A second start hands this to the running one.
//! - `--plain-tray`: the same tray through the `tray-icon` crate and a Win32 pump, without
//!   Tauri, as crates/astromech-core/src/tray.rs runs it today (the cost baseline).
//! - `--sink-before-build` / `--sink-after-build`: start astromech's Raw Input listener before
//!   or after Tauri builds its event loop, and count the keys it sees (`ui_state.sink_keys`).
//! - `--crash-hook`: allow the `crash` command and register for restart with Windows.
//! - `--restarted`: only marks the command line Windows restarts us with.
//!
//! The endpoint is `%LOCALAPPDATA%\astromech-spike\core.json` and the log `spike.log` next to it.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
mod app;
#[cfg(windows)]
mod control;
#[cfg(windows)]
mod plain;
#[cfg(windows)]
mod win;

#[cfg(not(windows))]
fn main() {
    eprintln!("astromech-spike runs on Windows only");
    std::process::exit(2);
}

#[cfg(windows)]
fn main() {
    win::main()
}
