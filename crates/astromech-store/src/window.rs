//! The parts of `astromech.toml` the app window reads itself, from the file the core uses
//! ([`Paths::config_in_use`]). Everything else in the file is the core's.

use crate::Paths;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(default)]
pub struct Game {
    pub bindings_dir: String,
    pub game_dir: String,
    pub journal_dir: String,
}

#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(default)]
pub struct WindowConfig {
    pub game: Game,
    pub data_dir: String,
}

impl WindowConfig {
    /// The core's config, from where the core reads it; defaults if it is missing or unreadable.
    pub fn load(paths: &Paths) -> Self {
        std::fs::read_to_string(paths.config_in_use()).ok().and_then(|t| toml::from_str(&t).ok()).unwrap_or_default()
    }
}

/// The core's data folder as the window finds it (the `data_dir` of [`WindowConfig::load`]), where
/// the window opens every file it shares with the core, the remote-data cache among them.
pub fn data_dir(paths: &Paths) -> PathBuf {
    paths.data_dir(&WindowConfig::load(paths).data_dir)
}
