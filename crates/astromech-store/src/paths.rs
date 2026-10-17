//! Where astromech keeps its files: the one rule for the core (`astromech-core`'s `Config`) and
//! the app window (`astromech-shell`), so both open the same config, data files and logs in every
//! layout (installed, portable, `data_dir` override).

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The config file's name, in the app dir or (portable mode) next to the exe.
pub const CONFIG_FILE: &str = "astromech.toml";

/// Where the core says which config file it uses (in the app dir); see [`Paths::publish_config`].
const CONFIG_IN_USE: &str = "config.path";

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

    /// Tell the app window which config file the core uses (the core's `--config`, or the
    /// default), in `config.path` in the app dir. Written at every core start, so a later start
    /// without `--config` replaces an earlier one's.
    pub fn publish_config(&self, path: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.app_dir)?;
        // Absolute, since the window's working folder isn't the core's.
        let path = std::path::absolute(path)?;
        std::fs::write(self.app_dir.join(CONFIG_IN_USE), path.to_string_lossy().as_bytes())
    }

    /// The config file the running core reads: the one it published, else [`Paths::config_file`].
    /// The window reads its settings (Bindings folder, data folder) from here.
    pub fn config_in_use(&self) -> PathBuf {
        std::fs::read_to_string(self.app_dir.join(CONFIG_IN_USE))
            .ok()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| self.config_file())
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

    /// Carries a user file from the app dir (where the window kept it before `data_dir` applied to
    /// it) to `data_dir`, once: copies `name` only if the app dir has it and `data_dir` doesn't.
    /// Never overwrites or deletes either file. Returns whether it copied.
    pub fn carry_over(&self, name: &str, data_dir: &Path) -> std::io::Result<bool> {
        self.carry_over_with(
            name,
            data_dir,
            |src, dst| std::io::copy(src, dst).map(|_| ()),
            |a, b| std::fs::hard_link(a, b),
        )
    }

    /// `carry_over` with the copy and link steps injectable, so a test can fail the copy partway
    /// or the link as a filesystem without hard links (FAT32, exFAT) does. The copy goes to
    /// a temp file in `data_dir`, which is hard-linked to the final name (that fails if the name
    /// exists, on every platform, so a file that appears meanwhile is never overwritten) and then
    /// removed; a failed copy leaves nothing at the final name and the next call retries.
    fn carry_over_with(
        &self,
        name: &str,
        data_dir: &Path,
        copy: impl FnOnce(&mut std::fs::File, &mut std::fs::File) -> std::io::Result<()>,
        link: impl FnOnce(&Path, &Path) -> std::io::Result<()>,
    ) -> std::io::Result<bool> {
        let (from, to) = (self.app_dir.join(name), data_dir.join(name));
        if from == to || !from.is_file() || to.exists() {
            return Ok(false);
        }
        std::fs::create_dir_all(data_dir)?;
        let tmp = data_dir.join(format!(".{name}.carry-over.tmp"));
        let staged = (|| {
            let mut src = std::fs::File::open(&from)?;
            // The temp name is ours; a leftover from an earlier crash is safe to replace.
            let mut dst = std::fs::File::create(&tmp)?;
            copy(&mut src, &mut dst)?;
            dst.sync_all()
        })();
        let linked = staged.and_then(|()| match link(&tmp, &to) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
            // No hard links here: copy the staged bytes to a target that must not exist yet.
            Err(_) => Self::copy_new(&tmp, &to),
        });
        let _ = std::fs::remove_file(&tmp);
        linked
    }

    /// Copies `from` to `to` only if `to` does not exist (`Ok(false)` if it does). A copy that
    /// fails partway removes the `to` it made, so no truncated file is left.
    fn copy_new(from: &Path, to: &Path) -> std::io::Result<bool> {
        let mut dst = match std::fs::OpenOptions::new().write(true).create_new(true).open(to) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => return Ok(false),
            Err(e) => return Err(e),
        };
        let done = std::fs::File::open(from)
            .and_then(|mut src| std::io::copy(&mut src, &mut dst))
            .and_then(|_| dst.sync_all());
        if let Err(e) = done {
            let _ = std::fs::remove_file(to);
            return Err(e);
        }
        Ok(true)
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
    fn the_window_reads_the_config_the_core_published() {
        let (exe, app, other) =
            (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let local = app.path().to_str().unwrap().to_string();
        let mk = || Paths::new(exe.path().into(), env(&[("XDG_DATA_HOME", &local)]));
        let p = mk();
        // Nothing published: the default.
        assert_eq!(p.config_in_use(), p.config_file());
        // The core run with --config: the window follows it, in a fresh process too.
        let custom = other.path().join("mine.toml");
        std::fs::write(&custom, "[game]\nbindings_dir = \"/pasted\"\n").unwrap();
        p.publish_config(&custom).unwrap();
        assert_eq!(mk().config_in_use(), custom);
        assert!(std::fs::read_to_string(mk().config_in_use()).unwrap().contains("/pasted"));
        // A relative --config is published as an absolute path (the window's folder differs).
        p.publish_config(Path::new("rel.toml")).unwrap();
        assert!(mk().config_in_use().is_absolute() && mk().config_in_use().ends_with("rel.toml"));
        // A later start without --config takes the file back.
        p.publish_config(&p.config_file()).unwrap();
        assert_eq!(mk().config_in_use(), p.config_file());
        // Portable layout: the default is next to the exe, and a publish is followed the same way.
        std::fs::write(exe.path().join("astromech.toml"), "").unwrap();
        let portable = mk();
        assert!(portable.portable());
        portable.publish_config(&portable.config_file()).unwrap();
        assert_eq!(portable.config_in_use(), exe.path().join("astromech.toml"));
        portable.publish_config(&custom).unwrap();
        assert_eq!(mk().config_in_use(), custom);
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

    #[test]
    fn overrides_in_the_old_app_dir_are_found_in_the_data_dir_once() {
        let (app, exe, data) =
            (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let home = app.path().to_str().unwrap().to_string();
        let p = Paths::new(exe.path().into(), env(&[("LOCALAPPDATA", &home)]));
        let old = p.app_dir().join("device-overrides.json");
        std::fs::create_dir_all(p.app_dir()).unwrap();
        std::fs::write(&old, "mine").unwrap();
        let dir = p.data_dir(data.path().to_str().unwrap());
        let new = dir.join("device-overrides.json");

        assert!(p.carry_over("device-overrides.json", &dir).unwrap(), "first call copies");
        assert_eq!(std::fs::read_to_string(&new).unwrap(), "mine");
        assert_eq!(std::fs::read_to_string(&old).unwrap(), "mine", "the old file stays");

        std::fs::write(&new, "edited").unwrap();
        assert!(!p.carry_over("device-overrides.json", &dir).unwrap(), "once only");
        assert_eq!(std::fs::read_to_string(&new).unwrap(), "edited", "never overwritten");
        // Same folder (the default layout), or nothing to carry: no-ops.
        assert!(!p.carry_over("device-overrides.json", p.app_dir()).unwrap());
        assert!(!p.carry_over("absent.json", &dir).unwrap());
    }

    /// A copy that dies partway must not leave a (truncated) file at the target, and the next
    /// call copies it.
    #[test]
    fn a_carry_over_that_fails_partway_leaves_no_target_and_is_retried() {
        use std::io::Write;
        let (app, exe, data) =
            (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let home = app.path().to_str().unwrap().to_string();
        let p = Paths::new(exe.path().into(), env(&[("LOCALAPPDATA", &home)]));
        std::fs::create_dir_all(p.app_dir()).unwrap();
        std::fs::write(p.app_dir().join("device-overrides.json"), "mine").unwrap();
        let dir = data.path().to_path_buf();
        let new = dir.join("device-overrides.json");

        let failed = p.carry_over_with(
            "device-overrides.json",
            &dir,
            |_, dst| {
                dst.write_all(b"mi")?;
                Err(std::io::Error::other("disk full"))
            },
            |a, b| std::fs::hard_link(a, b),
        );
        assert!(failed.is_err());
        assert!(!new.exists(), "no truncated file at the target");
        assert_eq!(std::fs::read_to_string(p.app_dir().join("device-overrides.json")).unwrap(), "mine");

        assert!(p.carry_over("device-overrides.json", &dir).unwrap(), "the next call copies it");
        assert_eq!(std::fs::read_to_string(&new).unwrap(), "mine");
        let left: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(left.len(), 1, "no temp file left behind: {left:?}");
    }

    /// Where hard links aren't supported (FAT32, exFAT) the overrides still arrive, and a file
    /// that appears meanwhile is still never overwritten.
    #[test]
    fn a_carry_over_without_hard_links_still_arrives_and_never_overwrites() {
        let (app, exe, data) =
            (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let home = app.path().to_str().unwrap().to_string();
        let p = Paths::new(exe.path().into(), env(&[("LOCALAPPDATA", &home)]));
        std::fs::create_dir_all(p.app_dir()).unwrap();
        std::fs::write(p.app_dir().join("device-overrides.json"), "mine").unwrap();
        let dir = data.path().to_path_buf();
        let new = dir.join("device-overrides.json");
        let copy = |src: &mut std::fs::File, dst: &mut std::fs::File| std::io::copy(src, dst).map(|_| ());
        let no_links = |_: &Path, _: &Path| Err(std::io::Error::from(std::io::ErrorKind::Unsupported));

        assert!(p.carry_over_with("device-overrides.json", &dir, copy, no_links).unwrap());
        assert_eq!(std::fs::read_to_string(&new).unwrap(), "mine");
        let left: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(left.len(), 1, "no temp file left behind: {left:?}");

        // The user's file appears while the link step fails: it stays as it is.
        std::fs::remove_file(&new).unwrap();
        let racing = |_: &Path, to: &Path| {
            std::fs::write(to, "theirs").unwrap();
            Err(std::io::Error::from(std::io::ErrorKind::Unsupported))
        };
        assert!(!p.carry_over_with("device-overrides.json", &dir, copy, racing).unwrap());
        assert_eq!(std::fs::read_to_string(&new).unwrap(), "theirs");
    }
}
