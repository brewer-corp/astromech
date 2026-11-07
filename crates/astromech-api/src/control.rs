//! The core's local control API: JSON lines over TCP on 127.0.0.1, one request per
//! connection, served from the [`Registry`]. The port and a per-run token are written to
//! `<app dir>/core.json`, which only this user can read; every request must carry the token.
//! The server ([`serve`]) and the client ([`call`]) live here together, so the core, the shell
//! and tests speak one wire.
//!
//! A request is `{token, cmd, args?, text?, route?, origin?}`. The reply is zero or more
//! `{"progress": V}` lines, then `{"ok": true, "value": V}` or `{"ok": false, "error": "…"}`.
//! Each connection is served on its own thread, which runs the handler.

use crate::Registry;
use anyhow::{Context, Result};
use astromech_proto::Origin;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

pub const FILE: &str = "core.json";

/// The read timeout for short status-style callers (the instance check, `--quit`, a second
/// start). Callers that run a command for the window wait without one: a plot polls Spansh
/// for minutes, and a dropped connection means the core died.
pub const SHORT: Duration = Duration::from_secs(10);

const CONNECT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Endpoint {
    pub port: u16,
    pub token: String,
    pub pid: u32,
    pub version: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {
    /// Filled in by [`call`] from `core.json`.
    pub token: String,
    pub cmd: String,
    /// The command's arguments (e.g. `set_ptt` with `{device, key}`, `route_set` the route).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
    /// A text argument: the voice tester's phrase or command id, a JSON change (`prospector_set`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The page to show (`open_window`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Who the request is for: `app_window` when the shell forwards the page's call; none from a
    /// local control client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
}

impl Request {
    pub fn new(cmd: &str) -> Self {
        Self { cmd: cmd.into(), ..Default::default() }
    }
}

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
pub fn serve(app_dir: &Path, registry: Arc<Registry>) -> Result<Server> {
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
            let reg = registry.clone();
            let tok = tok.clone();
            let _ = std::thread::Builder::new().name("control-conn".into()).spawn(move || {
                let _ = handle(s, &tok, &reg);
            });
        }
    })?;
    Ok(Server { endpoint, file })
}

/// One line on the connection. A client that went away (it timed out) is not an error here.
fn send(mut s: &TcpStream, v: &Value) -> std::io::Result<()> {
    s.write_all(format!("{v}\n").as_bytes())
}

fn handle(s: TcpStream, tok: &str, reg: &Registry) -> Result<()> {
    s.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut line = String::new();
    BufReader::new(s.try_clone()?).take(64 * 1024).read_line(&mut line)?;
    let reply = match serde_json::from_str::<Request>(&line) {
        Ok(r) if r.token == tok => reg.dispatch(&r, &|v| {
            let _ = send(&s, &json!({ "progress": v }));
        }),
        Ok(_) => Err("bad token".into()),
        Err(e) => Err(format!("bad request: {e}")),
    };
    let reply = match reply {
        Ok(value) => json!({"ok": true, "value": value}),
        Err(error) => json!({"ok": false, "error": error}),
    };
    send(&s, &reply)?;
    Ok(())
}

pub fn read_endpoint(app_dir: &Path) -> Option<Endpoint> {
    serde_json::from_str(&std::fs::read_to_string(app_dir.join(FILE)).ok()?).ok()
}

/// Send one request to the running core and wait for its reply: the command's value, or the
/// error the core gave or why it couldn't be reached. Progress values go to `on_progress` as
/// they arrive. `timeout` bounds each wait for a line; `None` waits until the core answers or
/// the connection drops.
pub fn call(
    app_dir: &Path,
    mut req: Request,
    timeout: Option<Duration>,
    on_progress: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    let ep = read_endpoint(app_dir).ok_or("the core is not running")?;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], ep.port));
    let s = TcpStream::connect_timeout(&addr, CONNECT).map_err(|e| format!("connecting to the core: {e}"))?;
    s.set_read_timeout(timeout).map_err(|e| e.to_string())?;
    req.token = ep.token;
    let line = serde_json::to_string(&req).map_err(|e| e.to_string())?;
    (&s).write_all(format!("{line}\n").as_bytes()).map_err(|e| format!("sending to the core: {e}"))?;
    let mut lines = BufReader::new(s).lines();
    loop {
        let line = match lines.next() {
            Some(l) => l.map_err(|e| format!("waiting for the core's reply to {}: {e}", req.cmd))?,
            None => return Err(format!("the core closed the connection before answering {}", req.cmd)),
        };
        let mut v: Value = serde_json::from_str(&line).map_err(|e| format!("reading the core's reply: {e}"))?;
        if let Some(p) = v.get_mut("progress") {
            on_progress(p.take());
        } else if v["ok"] == true {
            return Ok(v["value"].take());
        } else {
            return Err(v["error"].as_str().unwrap_or("the core gave no reason").to_string());
        }
    }
}

/// [`call`] for a short status-style caller: the command alone, bounded by [`SHORT`].
pub fn call_short(app_dir: &Path, cmd: &str, route: Option<&str>) -> Result<Value, String> {
    let req = Request { route: route.map(String::from), ..Request::new(cmd) };
    call(app_dir, req, Some(SHORT), &mut |_| {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Exposure;
    use std::sync::mpsc::channel;
    use std::sync::{Condvar, Mutex};

    fn no_progress(_: Value) {}

    #[test]
    fn round_trip_envelope_and_token() {
        let d = tempfile::tempdir().unwrap();
        let mut reg = Registry::new();
        reg.add("open_window", Exposure::Local, |r, _| Ok(json!({"ok": true, "route": r.route})));
        let srv = serve(d.path(), Arc::new(reg)).unwrap();
        // The handler's reply is the value, and the request's fields reach it.
        let v = call_short(d.path(), "open_window", Some("settings")).unwrap();
        assert_eq!(v, json!({"ok": true, "route": "settings"}));
        // Registry errors come back as the envelope's error.
        let e = call_short(d.path(), "nope", None).unwrap_err();
        assert!(e.contains("unknown command \"nope\""), "{e}");
        let e = call(
            d.path(),
            Request { origin: Some(Origin::AppWindow), ..Request::new("open_window") },
            None,
            &mut no_progress,
        )
        .unwrap_err();
        assert!(e.starts_with("not allowed"), "{e}");
        // A wrong token is refused.
        let mut ep = read_endpoint(d.path()).unwrap();
        ep.token = "nope".into();
        std::fs::write(d.path().join(FILE), serde_json::to_string(&ep).unwrap()).unwrap();
        assert_eq!(call_short(d.path(), "open_window", None), Err("bad token".into()));
        // Restore so Drop removes the file.
        std::fs::write(d.path().join(FILE), serde_json::to_string(&srv.endpoint).unwrap()).unwrap();
        drop(srv);
        assert!(!d.path().join(FILE).exists());
        assert_eq!(call_short(d.path(), "open_window", None), Err("the core is not running".into()));
    }

    #[test]
    fn optional_fields_may_be_absent_or_null() {
        let r: Request = serde_json::from_str(r#"{"token":"t","cmd":"voice_test","text":"gear down"}"#).unwrap();
        assert_eq!(r.text.as_deref(), Some("gear down"));
        // The window sends `"text": null` for commands without one.
        let r: Request = serde_json::from_str(r#"{"token":"t","cmd":"status","text":null,"origin":null}"#).unwrap();
        assert_eq!((r.text, r.origin), (None, None));
        let r: Request = serde_json::from_str(r#"{"token":"t","cmd":"status","origin":"app_window"}"#).unwrap();
        assert_eq!(r.origin, Some(Origin::AppWindow));
    }

    #[test]
    fn progress_lines_arrive_before_the_reply() {
        let d = tempfile::tempdir().unwrap();
        // The handler sends progress, then waits until the client has seen it: if progress were
        // held back until the reply, the client would never see it and the handler would give up.
        let (seen_tx, seen_rx) = channel::<()>();
        let seen_rx = Mutex::new(seen_rx);
        let mut reg = Registry::new();
        reg.add("job", Exposure::Window, move |_, call| {
            call.progress(json!("halfway"));
            crate::lock(&seen_rx)
                .recv_timeout(Duration::from_secs(60))
                .map_err(|_| "the progress line never reached the client".to_string())?;
            Ok(json!("done"))
        });
        let _srv = serve(d.path(), Arc::new(reg)).unwrap();
        let mut got = vec![];
        let v = call(d.path(), Request::new("job"), None, &mut |p| {
            got.push(p);
            seen_tx.send(()).unwrap();
        });
        assert_eq!(v, Ok(json!("done")));
        assert_eq!(got, vec![json!("halfway")]);
    }

    /// Opened once, wakes every waiter.
    #[derive(Default)]
    struct Gate(Mutex<bool>, Condvar);
    impl Gate {
        fn open(&self) {
            *crate::lock(&self.0) = true;
            self.1.notify_all();
        }
        fn wait(&self) {
            let mut open = crate::lock(&self.0);
            while !*open {
                open = self.1.wait(open).unwrap_or_else(|p| p.into_inner());
            }
        }
    }

    #[test]
    fn a_reply_after_one_clients_timeout_still_reaches_a_client_without_one() {
        let d = tempfile::tempdir().unwrap();
        let gate = Arc::new(Gate::default());
        let (entered_tx, entered_rx) = channel::<()>();
        let entered_tx = Mutex::new(entered_tx);
        let mut reg = Registry::new();
        let g = gate.clone();
        reg.add("plot", Exposure::Window, move |_, _| {
            crate::lock(&entered_tx).send(()).unwrap();
            g.wait();
            Ok(json!("plotted"))
        });
        let _srv = serve(d.path(), Arc::new(reg)).unwrap();
        // The patient client is waiting inside the handler before the impatient one calls.
        let dir = d.path().to_path_buf();
        let patient = std::thread::spawn(move || call(&dir, Request::new("plot"), None, &mut no_progress));
        entered_rx.recv().unwrap();
        // The impatient client gives up while the handler still waits, and that opens the gate.
        let short = call(d.path(), Request::new("plot"), Some(Duration::from_millis(50)), &mut no_progress);
        assert!(short.unwrap_err().starts_with("waiting for the core's reply to plot"));
        gate.open();
        assert_eq!(patient.join().unwrap(), Ok(json!("plotted")));
    }
}
