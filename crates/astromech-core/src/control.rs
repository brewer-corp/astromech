//! The core's local control API (M0): JSON lines over TCP on 127.0.0.1, one request per
//! connection. The port and a per-run token are written to `<app dir>/core.json`, which only
//! this user can read; every request must carry the token. Used by the app window (status,
//! diagnostics, crash restart) and by a second `astromech-core` start (open the window, then
//! exit). The full WebSocket API (astromech-proto) replaces this as the app grows.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

pub const FILE: &str = "core.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Endpoint {
    pub port: u16,
    pub token: String,
    pub pid: u32,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub token: String,
    /// "status", "open_window", "save_diagnostics", "ack_crash", "quit", "check_update", "install_update",
    /// "eddn_log", "voice_test", "voice_test_poll", "voice_test_stop", "voice_test_live",
    /// "prospector_thresholds", "prospector_set" (text: the change as JSON), "route", "route_set"
    /// (args: the route), "route_skip", "route_back", "route_clear", "route_copy", ...
    pub cmd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Argument for the voice tester: the phrase ("voice_test") or command id ("voice_test_live").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Arguments for setup commands (e.g. `set_ptt` with `{device, key}`) and `route_set`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
}

pub type Handler = Arc<dyn Fn(&Request) -> Value + Send + Sync>;

fn token() -> String {
    use std::hash::{BuildHasher, Hasher};
    // RandomState is seeded from the OS RNG; two of them give 128 bits.
    let mut s = String::new();
    for _ in 0..2 {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos());
        s += &format!("{:016x}", h.finish());
    }
    s
}

pub struct Server {
    pub endpoint: Endpoint,
    file: PathBuf,
}

impl Drop for Server {
    fn drop(&mut self) {
        // Only remove the file if it is still ours.
        if read_endpoint(self.file.parent().unwrap_or(Path::new("."))).as_ref() == Some(&self.endpoint) {
            let _ = std::fs::remove_file(&self.file);
        }
    }
}

/// Listen on an ephemeral localhost port and publish it in `<app_dir>/core.json`.
pub fn serve(app_dir: &Path, handler: Handler) -> Result<Server> {
    let l = TcpListener::bind(("127.0.0.1", 0)).context("binding the local control port")?;
    let endpoint = Endpoint {
        port: l.local_addr()?.port(),
        token: token(),
        pid: std::process::id(),
        version: env!("CARGO_PKG_VERSION").into(),
    };
    std::fs::create_dir_all(app_dir)?;
    let file = app_dir.join(FILE);
    std::fs::write(&file, serde_json::to_string(&endpoint)?)?;
    let tok = endpoint.token.clone();
    std::thread::Builder::new().name("control".into()).spawn(move || {
        for s in l.incoming().flatten() {
            let h = handler.clone();
            let tok = tok.clone();
            let _ = std::thread::Builder::new().name("control-conn".into()).spawn(move || {
                let _ = handle(s, &tok, &h);
            });
        }
    })?;
    Ok(Server { endpoint, file })
}

fn handle(s: TcpStream, tok: &str, h: &Handler) -> Result<()> {
    s.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut line = String::new();
    BufReader::new(s.try_clone()?).take(64 * 1024).read_line(&mut line)?;
    let resp = match serde_json::from_str::<Request>(&line) {
        Ok(r) if r.token == tok => h(&r),
        Ok(_) => serde_json::json!({"ok": false, "error": "bad token"}),
        Err(e) => serde_json::json!({"ok": false, "error": format!("bad request: {e}")}),
    };
    let mut s = s;
    writeln!(s, "{resp}")?;
    Ok(())
}

pub fn read_endpoint(app_dir: &Path) -> Option<Endpoint> {
    serde_json::from_str(&std::fs::read_to_string(app_dir.join(FILE)).ok()?).ok()
}

/// Send one command to the running core.
pub fn call(app_dir: &Path, cmd: &str, route: Option<&str>) -> Result<Value> {
    let Some(ep) = read_endpoint(app_dir) else { bail!("the core isn't running (no {FILE})") };
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], ep.port));
    let mut s = TcpStream::connect_timeout(&addr, Duration::from_secs(2)).context("connecting to the core")?;
    s.set_read_timeout(Some(Duration::from_secs(10)))?;
    let req = Request { token: ep.token, cmd: cmd.into(), route: route.map(String::from), text: None, args: None };
    writeln!(s, "{}", serde_json::to_string(&req)?)?;
    let mut line = String::new();
    BufReader::new(s).read_line(&mut line)?;
    serde_json::from_str(&line).context("reading the core's reply")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_token() {
        let d = tempfile::tempdir().unwrap();
        let srv =
            serve(d.path(), Arc::new(|r: &Request| serde_json::json!({"ok": true, "cmd": r.cmd, "route": r.route})))
                .unwrap();
        let v = call(d.path(), "open_window", Some("settings")).unwrap();
        assert_eq!(v["cmd"], "open_window");
        assert_eq!(v["route"], "settings");
        // A wrong token is refused.
        let mut ep = read_endpoint(d.path()).unwrap();
        ep.token = "nope".into();
        std::fs::write(d.path().join(FILE), serde_json::to_string(&ep).unwrap()).unwrap();
        assert_eq!(call(d.path(), "status", None).unwrap()["ok"], false);
        // Restore so Drop removes the file.
        std::fs::write(d.path().join(FILE), serde_json::to_string(&srv.endpoint).unwrap()).unwrap();
        drop(srv);
        assert!(!d.path().join(FILE).exists());
        assert!(call(d.path(), "status", None).is_err());
    }

    #[test]
    fn text_argument_is_optional() {
        let r: Request = serde_json::from_str(r#"{"token":"t","cmd":"voice_test","text":"gear down"}"#).unwrap();
        assert_eq!(r.text.as_deref(), Some("gear down"));
        // The window sends `"text": null` for commands without one.
        let r: Request = serde_json::from_str(r#"{"token":"t","cmd":"status","text":null}"#).unwrap();
        assert_eq!(r.text, None);
        let r: Request = serde_json::from_str(r#"{"token":"t","cmd":"status"}"#).unwrap();
        assert_eq!(r.text, None);
    }
}
