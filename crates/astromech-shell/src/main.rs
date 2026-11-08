//! # astromech-shell
//!
//! `astromech-shell.exe` was the app window of the two-process releases (up to
//! v1.0.0-alpha.4). The one app is `astromech-core.exe` (decision astromech-6i8s), and this
//! forwarder keeps what still points at the old name working: Start-menu shortcuts the updater
//! leaves behind and taskbar pins. It starts `astromech-core.exe` from its own folder with its
//! own arguments and exits; the app's single-instance guard does the rest (a second start opens
//! the window in the running app).
//!
//! It stays (decision astromech-nade): the updater's `copy_payload` never deletes, so on
//! installs it upgraded the file is there whatever a later release ships, and removing it from
//! the installer would break pins to it.

// GUI subsystem in every build, so a shortcut or pin flashes no console window.
#![windows_subsystem = "windows"]

use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, ExitCode};

/// The app's executable, beside this one.
const CORE: &str = if cfg!(windows) { "astromech-core.exe" } else { "astromech-core" };

/// The command that starts the app: `CORE` in `own_exe`'s folder, with `args` unchanged.
fn command(own_exe: &Path, args: impl IntoIterator<Item = OsString>) -> Command {
    let mut c = Command::new(own_exe.with_file_name(CORE));
    c.args(args);
    c
}

fn main() -> ExitCode {
    let Ok(exe) = std::env::current_exe() else { return ExitCode::FAILURE };
    match command(&exe, std::env::args_os().skip(1)).spawn() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn starts_the_core_beside_it_with_its_own_arguments() {
        let own = PathBuf::from("inst").join("astromech-shell.exe");
        let args = ["--route", "settings", "two words"].map(OsString::from);
        let c = command(&own, args.clone());
        assert_eq!(c.get_program(), PathBuf::from("inst").join(CORE).as_os_str());
        assert_eq!(c.get_args().collect::<Vec<_>>(), args.iter().map(|a| a.as_os_str()).collect::<Vec<_>>());
    }

    #[test]
    fn a_start_with_no_arguments_passes_none() {
        let c = command(&PathBuf::from("inst").join("astromech-shell.exe"), []);
        assert_eq!(c.get_program(), PathBuf::from("inst").join(CORE).as_os_str());
        assert_eq!(c.get_args().count(), 0);
    }
}
