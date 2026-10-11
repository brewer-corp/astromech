//! Client side of the voice worker process.

use crate::proto::{Request, Response};
use anyhow::{anyhow, bail, Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

pub const WORKER_EXE: &str = if cfg!(windows) { "astromech-voice-worker.exe" } else { "astromech-voice-worker" };
/// Linux reference runs only: the shared library the worker links against.
pub const C_API_LIB: &str = "libsherpa-onnx-c-api.so";

/// Explain a process exit code in plain words (Windows NTSTATUS codes from the loader).
pub fn explain_exit(code: Option<i32>) -> String {
    match code.map(|c| c as u32) {
        Some(0xC000_0135) => "a DLL it needs was not found (STATUS_DLL_NOT_FOUND)".into(),
        Some(0xC000_007B) => {
            "a DLL it loaded is for the wrong architecture or damaged (STATUS_INVALID_IMAGE_FORMAT)".into()
        }
        Some(0xC000_0139) => "a DLL it loaded is the wrong version (STATUS_ENTRYPOINT_NOT_FOUND)".into(),
        Some(0xC000_0142) => "it failed to initialise (STATUS_DLL_INIT_FAILED)".into(),
        Some(0xC000_0005) => "it crashed (access violation)".into(),
        Some(c) => format!("exit code {c} (0x{c:08X})"),
        None => "it was terminated".into(),
    }
}

/// Startup self-check: run `<worker> --version` (loads the speech engine, no models) so a
/// broken install is reported before any download. Returns the version line.
pub fn preflight(exe: &Path, lib_dir: Option<&Path>) -> Result<String> {
    if !exe.is_file() {
        bail!(
            "{} is missing. Keep astromech-voice-worker{} in the same folder as astromech-core{} (both are in the zip; \
             extract the whole zip, and check your antivirus did not quarantine it).",
            exe.display(),
            std::env::consts::EXE_SUFFIX,
            std::env::consts::EXE_SUFFIX
        );
    }
    let mut cmd = Command::new(exe);
    cmd.arg("--version").stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(d) = lib_dir {
        cmd.current_dir(d);
        if cfg!(unix) {
            cmd.env("LD_LIBRARY_PATH", d);
        }
    }
    hide_window(&mut cmd);
    let out = cmd.output().with_context(|| format!("could not start {}", exe.display()))?;
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !out.status.success() || !text.starts_with("astromech-voice-worker") {
        bail!(
            "the voice worker {} does not start: {}. {}",
            exe.display(),
            explain_exit(out.status.code()),
            String::from_utf8_lossy(&out.stderr).trim().chars().take(400).collect::<String>()
        );
    }
    Ok(text)
}

pub fn hide_window(cmd: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    #[cfg(not(windows))]
    let _ = cmd;
}

pub struct Worker {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<Response>,
    pub log: PathBuf,
    pub flavour: String,
}

impl Worker {
    /// Start the worker. `lib_dir` (Linux only) is where the shared sherpa-onnx libraries are;
    /// on Windows the worker is self-contained and runs from its own folder.
    pub fn spawn(exe: &Path, lib_dir: Option<&Path>, log: &Path, flavour: &str) -> Result<Self> {
        let log_file = std::fs::File::create(log).with_context(|| format!("create {}", log.display()))?;
        let mut cmd = Command::new(exe);
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::from(log_file));
        if let Some(d) = exe.parent() {
            cmd.current_dir(d);
        }
        if let (true, Some(d)) = (cfg!(unix), lib_dir) {
            let old = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
            cmd.env(
                "LD_LIBRARY_PATH",
                if old.is_empty() { d.display().to_string() } else { format!("{}:{old}", d.display()) },
            );
        }
        hide_window(&mut cmd);
        let mut child = cmd.spawn().with_context(|| format!("start {}", exe.display()))?;
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if let Some(Ok(r)) = Response::from_line(&line) {
                    if tx.send(r).is_err() {
                        break;
                    }
                }
            }
        });
        Ok(Worker { child, stdin, rx, log: log.to_path_buf(), flavour: flavour.into() })
    }

    pub fn call(&mut self, req: &Request, timeout: Duration) -> Result<Response> {
        let line = serde_json::to_string(req)?;
        if writeln!(self.stdin, "{line}").and_then(|_| self.stdin.flush()).is_err() {
            bail!("the {} voice worker is not running. {}", self.flavour, self.log_tail());
        }
        match self.rx.recv_timeout(timeout) {
            Ok(r) => Ok(r),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let _ = self.child.kill();
                Err(anyhow!(
                    "the {} voice worker did not answer within {} s (killed). {}",
                    self.flavour,
                    timeout.as_secs(),
                    self.log_tail()
                ))
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                std::thread::sleep(Duration::from_millis(200));
                let status =
                    self.child.try_wait().ok().flatten().map(|s| format!("{s}")).unwrap_or_else(|| "unknown".into());
                Err(anyhow!("the {} voice worker exited (status {status}). {}", self.flavour, self.log_tail()))
            }
        }
    }

    /// Call and turn `ok: false` into an error.
    pub fn call_ok(&mut self, req: &Request, timeout: Duration) -> Result<Response> {
        let r = self.call(req, timeout)?;
        if !r.ok {
            bail!("{} ({})", r.error.clone().unwrap_or_default(), self.log_tail());
        }
        Ok(r)
    }

    pub fn log_text(&self) -> String {
        std::fs::read_to_string(&self.log).unwrap_or_default()
    }

    pub fn log_tail(&self) -> String {
        let t = self.log_text();
        let lines: Vec<&str> = t.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            return format!("Worker log {} is empty.", self.log.display());
        }
        let tail = lines[lines.len().saturating_sub(6)..].join(" | ");
        format!("Last worker log lines: {tail}")
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = writeln!(self.stdin, "{}", serde_json::to_string(&Request::Quit).unwrap());
        let _ = self.stdin.flush();
        for _ in 0..20 {
            if let Ok(Some(_)) = self.child.try_wait() {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_codes_are_explained() {
        assert!(explain_exit(Some(0xC000_0135u32 as i32)).contains("DLL"));
        assert!(explain_exit(Some(3)).contains("exit code 3"));
        assert!(preflight(Path::new("/nonexistent/astromech-voice-worker"), None)
            .unwrap_err()
            .to_string()
            .contains("same folder"));
    }
}
