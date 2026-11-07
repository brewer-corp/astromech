//! A headless core in a world, for tests: `astromech-core --world <dir>` started as a child
//! process (decision astromech-e1a5: the core keeps process globals, so each world gets its own
//! process), reached at the control endpoint it prints on stdout.

use astromech_api::control::{self, Contact, Request};
use serde::Deserialize;
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};

/// The committed world `name`, in the repository's `worlds/`.
pub fn world(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../worlds").join(name)
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
    _stdout: BufReader<ChildStdout>,
    pub contact: Contact,
    /// Its app folder, under the world's temp folder.
    pub app_dir: PathBuf,
}

impl WorldCore {
    /// Start the core on `world`, with `env` added to the environment.
    pub fn start(world: &Path, env: &[(&str, &Path)]) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_astromech-core"))
            .arg("--world")
            .arg(world)
            .envs(env.iter().map(|(k, v)| (k, v)))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("start astromech-core --world");
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
        Self { child, _stdout: stdout, contact: control, app_dir }
    }

    /// A local control client's call (no origin), as a test or driver makes it.
    pub fn call(&self, cmd: &str, args: Option<Value>) -> Result<Value, String> {
        control::call_to(&self.contact, Request { args, ..Request::new(cmd) }, None, &mut |_| {})
    }

    /// The window's call, as the shell's proxy sends it (`origin: "app_window"`).
    pub fn call_window(&self, cmd: &str, args: Option<Value>) -> Result<Value, String> {
        control::call_window_to(&self.contact, cmd, args, &mut |_| {})
    }

    /// Ask it to quit, and wait for it to exit.
    pub fn quit(mut self) -> ExitStatus {
        self.call("quit", None).expect("quit");
        self.child.wait().expect("wait for the world core")
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
