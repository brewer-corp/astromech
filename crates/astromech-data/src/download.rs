//! Large files streamed to disk (the neutron star file, about 70 MB), with progress. They
//! don't go through [`crate::Client`]: its answers are text held in memory and cached, and these
//! are published files on GitHub, not a game-data service. Requests carry the shared
//! [`USER_AGENT`] and follow redirects (GitHub's release links redirect to its file host).

use crate::USER_AGENT;
use std::io::{Read, Write};
use std::time::Duration;

/// Why a download failed. `Display` is a sentence for the commander.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    /// Holds why (a TLS or proxy error, DNS, a refused or cut connection) for the log: see
    /// [`DownloadError::detail`].
    #[error("Cannot reach the download. Check your internet connection.")]
    Offline(String),
    #[error("The download stopped answering. Try again.")]
    Timeout,
    #[error("The download is not available (HTTP {0}). Try again later.")]
    Status(u16),
    /// Writing what came in failed (disk full, the file was damaged).
    #[error("{0}")]
    Write(std::io::Error),
}

impl DownloadError {
    /// What went wrong underneath, for the log, where the sentence alone doesn't say.
    pub fn detail(&self) -> Option<&str> {
        match self {
            DownloadError::Offline(why) => Some(why),
            _ => None,
        }
    }
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .try_proxy_from_env(true)
        .timeout_connect(Duration::from_secs(20))
        .timeout_read(Duration::from_secs(60))
        .user_agent(USER_AGENT)
        .build()
}

fn open(url: &str) -> Result<ureq::Response, DownloadError> {
    match agent().get(url).call() {
        Ok(r) => Ok(r),
        Err(ureq::Error::Status(s, _)) => Err(DownloadError::Status(s)),
        Err(ureq::Error::Transport(t)) => {
            let io = std::error::Error::source(&t).and_then(|s| s.downcast_ref::<std::io::Error>());
            Err(match io.map(|e| e.kind()) {
                Some(std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) => DownloadError::Timeout,
                _ => DownloadError::Offline(t.to_string()),
            })
        }
    }
}

fn read_error(e: std::io::Error) -> DownloadError {
    match e.kind() {
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => DownloadError::Timeout,
        _ => DownloadError::Offline(e.to_string()),
    }
}

/// A small text file (a checksum).
pub fn text(url: &str) -> Result<String, DownloadError> {
    let mut s = String::new();
    open(url)?.into_reader().take(1 << 20).read_to_string(&mut s).map_err(read_error)?;
    Ok(s)
}

/// Streams `url` into `out`, calling `progress(bytes so far, total if the server said)` after
/// each chunk. Returns the byte count.
pub fn stream(
    url: &str,
    out: &mut dyn Write,
    mut progress: impl FnMut(u64, Option<u64>),
) -> Result<u64, DownloadError> {
    let resp = open(url)?;
    let total = resp.header("Content-Length").and_then(|s| s.parse().ok());
    let mut r = resp.into_reader();
    let mut buf = vec![0u8; 64 * 1024];
    let mut done = 0u64;
    loop {
        let n = match r.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(read_error(e)),
        };
        out.write_all(&buf[..n]).map_err(DownloadError::Write)?;
        done += n as u64;
        progress(done, total);
    }
    if total.is_some_and(|t| t != done) {
        return Err(DownloadError::Offline(format!("got {done} of {total:?} bytes")));
    }
    Ok(done)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A refused connection keeps its reason (for the log), while `Display` stays the plain sentence.
    #[test]
    fn a_refused_connection_keeps_its_reason() {
        // Nothing listens on a port just bound and dropped.
        let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let e = text(&format!("http://127.0.0.1:{port}/x.sha256")).unwrap_err();
        let why = e.detail().unwrap_or_else(|| panic!("{e:?}"));
        assert!(why.to_ascii_lowercase().contains("refused"), "{why}");
        assert_eq!(e.to_string(), "Cannot reach the download. Check your internet connection.");
    }
}
