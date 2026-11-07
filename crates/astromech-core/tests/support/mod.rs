//! A headless core in a world, for tests: `astromech-core --world <dir>` started as a child
//! process (decision astromech-e1a5: the core keeps process globals, so each world gets its own
//! process), reached at the control endpoint it prints on stdout.
//!
//! The child never sees the user's folders: every variable a folder could fall back to points
//! at an empty home the helper owns, and [`WorldCore::quit`] fails if anything was written there.
//! Its stdin is a pipe the helper holds, so if the test process dies, even killed outright, the
//! core reads end of file and quits. The proxy variables are cleared for it too, a second guard
//! beside world mode's own (its downloads never take a proxy), so nothing it asks can leave the
//! machine through one.

// Each test binary that includes this module uses part of it.
#![allow(dead_code)]

use astromech_api::control::{self, Contact, Request};
use serde::Deserialize;
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};

/// The variables the core's folders (and the libraries' defaults) could fall back to.
pub const HOME_VARS: [&str; 8] = [
    "HOME",
    "XDG_DATA_HOME",
    "XDG_CONFIG_HOME",
    "XDG_CACHE_HOME",
    "XDG_STATE_HOME",
    "LOCALAPPDATA",
    "APPDATA",
    "USERPROFILE",
];

/// The variables ureq takes a proxy from.
pub const PROXY_VARS: [&str; 6] = ["ALL_PROXY", "all_proxy", "HTTPS_PROXY", "https_proxy", "HTTP_PROXY", "http_proxy"];

/// The committed world `name`, in the repository's `worlds/`.
pub fn world(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../worlds").join(name)
}

/// A copy of the committed world `name` in a temp folder, with its `world.jsonl` replaced by
/// `steps` (when given): a variant of a world for one test.
pub fn world_with(name: &str, steps: Option<&str>) -> tempfile::TempDir {
    fn copy(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).expect("make the folder");
        for e in std::fs::read_dir(from).expect("read the world") {
            let e = e.expect("an entry");
            if e.file_type().expect("its type").is_dir() {
                copy(&e.path(), &to.join(e.file_name()));
            } else {
                std::fs::copy(e.path(), to.join(e.file_name())).expect("copy a file");
            }
        }
    }
    let d = tempfile::Builder::new().prefix("astromech-test-world-").tempdir().expect("a temp folder");
    copy(&world(name), d.path());
    if let Some(steps) = steps {
        std::fs::write(d.path().join("world.jsonl"), steps).expect("write world.jsonl");
    }
    d
}

/// The line world mode prints once its control API is up.
#[derive(Deserialize)]
struct Line {
    control: Contact,
    app_dir: PathBuf,
}

/// A running world core. Dropping it kills the process if it is still running.
pub struct WorldCore {
    child: Child,
    _stdin: ChildStdin,
    _stdout: BufReader<ChildStdout>,
    home: tempfile::TempDir,
    pub contact: Contact,
    /// Its app folder, under the world's temp folder.
    pub app_dir: PathBuf,
}

/// The files in `dir`, if any.
pub fn files_in(dir: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir).expect("read the folder").map(|e| e.expect("an entry").path()).collect()
}

impl WorldCore {
    /// Start the core on `world`, with its own empty home and no proxy variables.
    pub fn start(world: &Path) -> Self {
        Self::start_with_env(world, &[])
    }

    /// [`WorldCore::start`], with `env` set for the core after the helper's own variables (a
    /// test that sets the proxy variables to check world mode ignores them).
    pub fn start_with_env(world: &Path, env: &[(&str, &str)]) -> Self {
        let home = tempfile::Builder::new().prefix("astromech-test-home-").tempdir().expect("an empty home");
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_astromech-core"));
        for k in PROXY_VARS {
            cmd.env_remove(k);
        }
        let mut child = cmd
            .arg("--world")
            .arg(world)
            .envs(HOME_VARS.map(|k| (k, home.path())))
            .envs(env.iter().copied())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("start astromech-core --world");
        let stdin = child.stdin.take().expect("its stdin");
        let mut stdout = BufReader::new(child.stdout.take().expect("its stdout"));
        let mut line = String::new();
        let read = stdout.read_line(&mut line);
        if read.as_ref().map_or(true, |n| *n == 0) {
            let status = child.wait();
            panic!(
                "astromech-core --world {} printed no endpoint line ({read:?}); it exited {status:?}",
                world.display()
            );
        }
        let Line { control, app_dir } = match serde_json::from_str(&line) {
            Ok(l) => l,
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("astromech-core --world printed {line:?}, not its endpoint line: {e}");
            }
        };
        Self { child, _stdin: stdin, _stdout: stdout, home, contact: control, app_dir }
    }

    /// The empty folder every home variable points at.
    pub fn home(&self) -> &Path {
        self.home.path()
    }

    /// A local control client's call (no origin), as a test or driver makes it.
    pub fn call(&self, cmd: &str, args: Option<Value>) -> Result<Value, String> {
        control::call_to(&self.contact, Request { args, ..Request::new(cmd) }, None, &mut |_| {})
    }

    /// The window's call, as the shell's proxy sends it (`origin: "app_window"`).
    pub fn call_window(&self, cmd: &str, args: Option<Value>) -> Result<Value, String> {
        control::call_window_to(&self.contact, cmd, args, &mut |_| {})
    }

    /// `world_step` to the mark `to`.
    pub fn step(&self, to: &str) -> Result<Value, String> {
        self.call("world_step", Some(serde_json::json!({ "to": to })))
    }

    /// Ask it to quit and wait for it to exit; panics if it wrote anything into its home.
    pub fn quit(mut self) -> ExitStatus {
        self.call("quit", None).expect("quit");
        let status = self.child.wait().expect("wait for the world core");
        let left = files_in(self.home.path());
        assert!(left.is_empty(), "the world core wrote into its home: {left:?}");
        status
    }
}

impl Drop for WorldCore {
    fn drop(&mut self) {
        if let Ok(None) = self.child.try_wait() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}
