//! Where astromech keeps its files: the one rule for the core (`astromech-core`'s `Config`) and
//! the app window (`astromech-shell`), so both open the same config, data files and logs in every
//! layout (installed, portable, `data_dir` override).

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The config file's name, in the app dir or (portable mode) next to the exe.
pub const CONFIG_FILE: &str = "astromech.toml";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paths {
    exe_dir: PathBuf,
    app_dir: PathBuf,
    portable: bool,
    /// `%USERPROFILE%`, under which the game keeps its journals.
    user_profile: Option<PathBuf>,
}

impl Paths {
    /// The paths for the running exe and this process's environment.
    pub fn current() -> Self {
        let exe_dir =
            std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| ".".into());
        Self::new(exe_dir, |k| std::env::var_os(k))
    }

    /// The paths for an exe in `exe_dir`, reading environment variables through `env`.
    pub fn new(exe_dir: PathBuf, env: impl Fn(&str) -> Option<OsString>) -> Self {
        let base = env("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| env("XDG_DATA_HOME").map(PathBuf::from))
            .or_else(|| env("HOME").map(|h| PathBuf::from(h).join(".local").join("share")));
        let app_dir = base.map(|b| b.join("astromech").join("data")).unwrap_or_else(|| exe_dir.join("astromech-data"));
        let portable = exe_dir.join(CONFIG_FILE).is_file();
        Self { exe_dir, app_dir, portable, user_profile: env("USERPROFILE").map(PathBuf::from) }
    }

    /// The folder holding the running exe.
    pub fn exe_dir(&self) -> &Path {
        &self.exe_dir
    }

    /// Per-user data folder: `%LOCALAPPDATA%\astromech\data`. The installer puts the program in
    /// `%LOCALAPPDATA%\astromech` (Tauri's per-user default); config, logs, models and data live
    /// in its `data` subfolder, which upgrades and uninstalls leave alone unless "Delete the
    /// application data" is ticked. Off Windows: `$XDG_DATA_HOME` or `~/.local/share`. It also
    /// holds the core's endpoint file (`core.json`), whatever the layout.
    pub fn app_dir(&self) -> &Path {
        &self.app_dir
    }

    /// Portable layout (the gear-down preview zip): an `astromech.toml` next to the exe. Then
    /// config, data, logs and models stay next to the exe.
    pub fn portable(&self) -> bool {
        self.portable
    }

    /// Where the config file is when `--config` isn't given.
    pub fn config_file(&self) -> PathBuf {
        if self.portable {
            self.exe_dir.join(CONFIG_FILE)
        } else {
            self.app_dir.join(CONFIG_FILE)
        }
    }

    /// The folder holding `astromech.sqlite` and the other data files (`galaxy.db`,
    /// `history.sqlite`, `outfitting.sqlite`, the remote-data cache): the config's `data_dir` if
    /// set, else `exe_dir/astromech-data` in portable mode, else the app dir.
    pub fn data_dir(&self, configured: &str) -> PathBuf {
        let c = configured.trim();
        if !c.is_empty() {
            c.into()
        } else if self.portable {
            self.exe_dir.join("astromech-data")
        } else {
            self.app_dir.clone()
        }
    }

    /// The log folder, where the core writes `astromech-core.log` and the app window
    /// `astromech-shell.log`: `logs` under `exe_dir/astromech-data` in portable mode, else under
    /// the app dir. A `data_dir` override doesn't move it.
    pub fn log_dir(&self) -> PathBuf {
        if self.portable { self.exe_dir.join("astromech-data") } else { self.app_dir.clone() }.join("logs")
    }

    /// The game's journal folder: the config's `game.journal_dir` if set, else
    /// `%USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous`; `None` without either.
    pub fn journal_dir(&self, configured: &str) -> Option<PathBuf> {
        let c = configured.trim();
        if !c.is_empty() {
            return Some(c.into());
        }
        self.user_profile.as_ref().map(|h| h.join("Saved Games").join("Frontier Developments").join("Elite Dangerous"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<OsString> + 'a {
        move |k| vars.iter().find(|(n, _)| *n == k).map(|(_, v)| OsString::from(v))
    }

    /// The core logs into `Paths::log_dir` (`astromech-core`'s `app::run`) and so does the app
    /// window (`astromech-shell`'s `main`), so both files sit in one folder, which the
    /// diagnostics bundle zips, in every layout.
    #[test]
    fn the_core_and_the_shell_log_into_one_folder_in_every_layout() {
        use astromech_log::{file, CORE, SHELL};
        let exe = tempfile::tempdir().unwrap();
        let vars = [("LOCALAPPDATA", "/l")];
        let installed = Paths::new(exe.path().into(), env(&vars));
        std::fs::write(exe.path().join("astromech.toml"), "").unwrap();
        let portable = Paths::new(exe.path().into(), env(&vars));
        let portable_logs = exe.path().join("astromech-data").join("logs");
        // (layout, data_dir override, expected log folder)
        for (p, data_dir, want) in [
            (&installed, "", Path::new("/l/astromech/data/logs")),
            (&installed, "/d", Path::new("/l/astromech/data/logs")),
            (&portable, "", portable_logs.as_path()),
            (&portable, "/d", portable_logs.as_path()),
        ] {
            let logs = p.log_dir();
            assert_eq!(logs, want, "data_dir {data_dir:?}");
            assert_ne!(logs.parent(), Some(Path::new(data_dir)), "the override doesn't move the logs");
            assert_eq!(file(&logs, CORE).parent(), file(&logs, SHELL).parent());
            assert_eq!(file(&logs, SHELL), want.join("astromech-shell.log"));
        }
        // Without LOCALAPPDATA, XDG_DATA_HOME or HOME, both fall back next to the exe.
        assert_eq!(Paths::new(exe.path().into(), env(&[])).log_dir(), portable_logs);
    }

    #[test]
    fn journal_dir_is_the_configured_one_else_under_the_user_profile() {
        let exe = tempfile::tempdir().unwrap();
        let p = Paths::new(exe.path().into(), env(&[("USERPROFILE", "/u")]));
        assert_eq!(p.journal_dir(" /j "), Some(PathBuf::from("/j")));
        assert_eq!(p.journal_dir(""), Some(PathBuf::from("/u/Saved Games/Frontier Developments/Elite Dangerous")));
        assert_eq!(Paths::new(exe.path().into(), env(&[])).journal_dir(""), None);
    }

    #[test]
    fn default_layout_uses_the_app_dir() {
        let exe = tempfile::tempdir().unwrap();
        let p = Paths::new(exe.path().into(), env(&[("LOCALAPPDATA", "/l"), ("XDG_DATA_HOME", "/x"), ("HOME", "/h")]));
        let app = Path::new("/l/astromech/data");
        assert!(!p.portable());
        assert_eq!(p.app_dir(), app);
        assert_eq!(p.config_file(), app.join("astromech.toml"));
        assert_eq!(p.data_dir(""), app);
        assert_eq!(p.data_dir("  "), app);
        assert_eq!(p.log_dir(), app.join("logs"));
    }

    #[test]
    fn app_dir_falls_back_through_xdg_home_and_the_exe_dir() {
        let exe = tempfile::tempdir().unwrap();
        let at = |vars: &[(&str, &str)]| Paths::new(exe.path().into(), env(vars)).app_dir().to_path_buf();
        assert_eq!(at(&[("XDG_DATA_HOME", "/x"), ("HOME", "/h")]), Path::new("/x/astromech/data"));
        assert_eq!(at(&[("HOME", "/h")]), Path::new("/h/.local/share/astromech/data"));
        assert_eq!(at(&[]), exe.path().join("astromech-data"));
    }

    #[test]
    fn portable_mode_keeps_everything_next_to_the_exe() {
        let exe = tempfile::tempdir().unwrap();
        std::fs::write(exe.path().join("astromech.toml"), "").unwrap();
        let p = Paths::new(exe.path().into(), env(&[("LOCALAPPDATA", "/l")]));
        let data = exe.path().join("astromech-data");
        assert!(p.portable());
        assert_eq!(p.app_dir(), Path::new("/l/astromech/data"), "the endpoint file stays in the app dir");
        assert_eq!(p.config_file(), exe.path().join("astromech.toml"));
        assert_eq!(p.data_dir(""), data);
        assert_eq!(p.log_dir(), data.join("logs"));
    }

    #[test]
    fn the_data_dir_override_wins_in_every_layout() {
        let exe = tempfile::tempdir().unwrap();
        let installed = Paths::new(exe.path().into(), env(&[("LOCALAPPDATA", "/l")]));
        assert_eq!(installed.data_dir(" /d "), Path::new("/d"));
        assert_eq!(installed.log_dir(), Path::new("/l/astromech/data/logs"));
        std::fs::write(exe.path().join("astromech.toml"), "").unwrap();
        let portable = Paths::new(exe.path().into(), env(&[("LOCALAPPDATA", "/l")]));
        assert_eq!(portable.data_dir("/d"), Path::new("/d"));
    }
}
