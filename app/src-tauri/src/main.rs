//! # astromech-shell
//!
//! The app window (docs/ARCHITECTURE.md §18): a thin Tauri v2 shell that shows the Svelte app
//! from `app/`. It holds no game logic; the app talks to `astromech-core` over the local
//! WebSocket API. Closing the window does not stop the core.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default().run(tauri::generate_context!()).expect("error while running the astromech app window");
}
