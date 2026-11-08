//! The core's local control API: JSON lines over TCP on 127.0.0.1, one request per
//! connection, served from the [`Registry`]. The port and a per-run token are written to
//! `<app dir>/core.json`, which only this user can read; every request must carry the token.
//! The server ([`serve`]) and the client ([`call`]) live here together, so the core, its local
//! clients (a second start, `--quit`) and tests speak one wire.
//!
//! A request is `{token, cmd, args?, text?, route?, origin?}`. The reply is zero or more
//! `{"progress": V}` lines, then `{"ok": true, "value": V}` or `{"ok": false, "error": "…"}`.
//! Each connection is served on its own thread, which runs the handler.

use crate::Registry;
use anyhow::{Context, Result};
use astromech_proto::{Clock, Origin};
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

/// Where a core's control API answers: its port on 127.0.0.1 and the run's token. World mode
/// prints it on stdout (`{"control": Contact, "app_dir": …}`), and its callers use [`call_to`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub port: u16,
    pub token: String,
}

impl Endpoint {
    pub fn contact(&self) -> Contact {
        Contact { port: self.port, token: self.token.clone() }
    }
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
    /// Who the request is for: `app_window` for the window's calls ([`window_request`]); none
    /// from a local control client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
}

impl Request {
    pub fn new(cmd: &str) -> Self {
        Self { cmd: cmd.into(), ..Default::default() }
    }
}

/// `clock`'s time is mixed in with the OS-seeded hashers.
fn token(clock: &dyn Clock) -> String {
    use std::hash::{BuildHasher, Hasher};
    // RandomState is seeded from the OS RNG; two of them give 128 bits.
    let mut s = String::new();
    for _ in 0..2 {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u128(clock.utc().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos());
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

/// Listen on an ephemeral localhost port and publish it in `<app_dir>/core.json`. The token's
/// time is read on `clock`.
pub fn serve(app_dir: &Path, registry: Arc<Registry>, clock: &dyn Clock) -> Result<Server> {
    let l = TcpListener::bind(("127.0.0.1", 0)).context("binding the local control port")?;
    let endpoint = Endpoint {
        port: l.local_addr()?.port(),
        token: token(clock),
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
    let (reply, after) = match serde_json::from_str::<Request>(&line) {
        Ok(r) if r.token == tok => reg.dispatch_then(&r, &|v| {
            let _ = send(&s, &json!({ "progress": v }));
        }),
        Ok(_) => (Err("bad token".into()), Default::default()),
        Err(e) => (Err(format!("bad request: {e}")), Default::default()),
    };
    let reply = match reply {
        Ok(value) => json!({"ok": true, "value": value}),
        Err(error) => json!({"ok": false, "error": error}),
    };
    let sent = send(&s, &reply);
    // The reply is out: what would cut it off (quit ending the process) runs only now.
    after.run();
    Ok(sent?)
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
    req: Request,
    timeout: Option<Duration>,
    on_progress: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    let ep = read_endpoint(app_dir).ok_or("the core is not running")?;
    call_to(&ep.contact(), req, timeout, on_progress)
}

/// [`call`] to the core at `to`, without reading `core.json`.
pub fn call_to(
    to: &Contact,
    mut req: Request,
    timeout: Option<Duration>,
    on_progress: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], to.port));
    let s = TcpStream::connect_timeout(&addr, CONNECT).map_err(|e| format!("connecting to the core: {e}"))?;
    s.set_read_timeout(timeout).map_err(|e| e.to_string())?;
    req.token = to.token.clone();
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

/// The window's request for `cmd`: stamped `origin: "app_window"`, so the registry refuses the
/// `local` commands. A string `args` is the request's `text` (the voice tester's phrase or
/// command id, a JSON change such as `prospector_set`'s), the commands whose args type is
/// `string` in the generated map. The window's `call` (app/src-tauri) dispatches it in process.
pub fn window_request(cmd: &str, args: Option<Value>) -> Request {
    let (text, args) = match args {
        Some(Value::String(s)) => (Some(s), None),
        a => (None, a),
    };
    Request { args, text, origin: Some(Origin::AppWindow), ..Request::new(cmd) }
}

/// [`window_request`] over the control API, for tests that call the core as the window does:
/// no read timeout (a plot polls Spansh for minutes; a dropped connection means the core died),
/// except `status`, which keeps the [`SHORT`] bound of status-style callers. Progress values go
/// to `on_progress`.
pub fn call_window(
    app_dir: &Path,
    cmd: &str,
    args: Option<Value>,
    on_progress: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    call(app_dir, window_request(cmd, args), window_timeout(cmd), on_progress)
}

/// [`call_window`] to the core at `to`, without reading `core.json`.
pub fn call_window_to(
    to: &Contact,
    cmd: &str,
    args: Option<Value>,
    on_progress: &mut dyn FnMut(Value),
) -> Result<Value, String> {
    call_to(to, window_request(cmd, args), window_timeout(cmd), on_progress)
}

/// The read timeout of [`call_window`]: none, except `status`'s [`SHORT`].
fn window_timeout(cmd: &str) -> Option<Duration> {
    (cmd == "status").then_some(SHORT)
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
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
        let srv = serve(d.path(), Arc::new(reg), &astromech_proto::SystemClock).unwrap();
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
        let _srv = serve(d.path(), Arc::new(reg), &astromech_proto::SystemClock).unwrap();
        let mut got = vec![];
        let v = call(d.path(), Request::new("job"), None, &mut |p| {
            got.push(p);
            seen_tx.send(()).unwrap();
        });
        assert_eq!(v, Ok(json!("done")));
        assert_eq!(got, vec![json!("halfway")]);
    }

    /// An effect the handler defers with `after_reply` runs only once the reply is sent: it
    /// waits for the client to have read the reply, which a hook run before the reply never sees
    /// (the guard then fails the call), as `quit` ending the process would cut the reply off.
    #[test]
    fn an_after_reply_effect_runs_once_the_reply_is_written() {
        let d = tempfile::tempdir().unwrap();
        let (read_tx, read_rx) = channel::<()>();
        let read_rx = Arc::new(Mutex::new(read_rx));
        let (ran_tx, ran_rx) = channel::<bool>();
        let ran_tx = Mutex::new(ran_tx);
        let mut reg = Registry::new();
        reg.add("quit", Exposure::Local, move |_, call| {
            let (seen, ran) = (read_rx.clone(), crate::lock(&ran_tx).clone());
            call.after_reply(move || {
                let read = crate::lock(&seen).recv_timeout(Duration::from_secs(60)).is_ok();
                let _ = ran.send(read);
            });
            Ok(json!("bye"))
        });
        let _srv = serve(d.path(), Arc::new(reg), &astromech_proto::SystemClock).unwrap();
        assert_eq!(call_short(d.path(), "quit", None), Ok(json!("bye")));
        read_tx.send(()).unwrap();
        assert!(ran_rx.recv().unwrap(), "the effect ran before the client had the reply");
    }

    /// In-process, the effect runs as dispatch returns, after the handler's reply is made.
    #[test]
    fn dispatch_runs_the_after_reply_effects_before_it_returns() {
        let ran = Arc::new(Mutex::new(vec![]));
        let mut reg = Registry::new();
        let r = ran.clone();
        reg.add("twice", Exposure::Window, move |_, call| {
            for n in [1, 2] {
                let r = r.clone();
                call.after_reply(move || crate::lock(&r).push(n));
            }
            assert!(crate::lock(&r).is_empty(), "nothing runs before the reply");
            Ok(json!("done"))
        });
        assert_eq!(reg.dispatch(&Request::new("twice"), &|_| {}), Ok(json!("done")));
        assert_eq!(*crate::lock(&ran), vec![1, 2]);
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
        let _srv = serve(d.path(), Arc::new(reg), &astromech_proto::SystemClock).unwrap();
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

    #[test]
    fn the_windows_call_is_stamped_app_window_and_takes_text_as_a_string() {
        let d = tempfile::tempdir().unwrap();
        let mut reg = Registry::new();
        reg.add("echo", Exposure::Window, |r, call| {
            call.progress(json!(1));
            call.progress(json!(2));
            Ok(json!({"args": r.args, "text": r.text, "origin": r.origin}))
        });
        reg.add("quit", Exposure::Local, |_, _| Ok(json!("bye")));
        reg.add("fail", Exposure::Window, |_, _| Err("no journal".into()));
        let _srv = serve(d.path(), Arc::new(reg), &astromech_proto::SystemClock).unwrap();
        let mut got = vec![];
        let v = call_window(d.path(), "echo", Some(json!({"within_ly": 20})), &mut |p| got.push(p)).unwrap();
        assert_eq!(v, json!({"args": {"within_ly": 20}, "text": null, "origin": "app_window"}));
        assert_eq!(got, vec![json!(1), json!(2)]);
        let v = call_window(d.path(), "echo", Some(json!("gear down")), &mut no_progress).unwrap();
        assert_eq!(v, json!({"args": null, "text": "gear down", "origin": "app_window"}));
        let v = call_window(d.path(), "echo", None, &mut no_progress).unwrap();
        assert_eq!(v, json!({"args": null, "text": null, "origin": "app_window"}));
        // The handler's error and the registry's refusals are the call's error.
        assert_eq!(call_window(d.path(), "fail", None, &mut no_progress), Err("no journal".into()));
        let e = call_window(d.path(), "quit", None, &mut no_progress).unwrap_err();
        assert!(e.starts_with("not allowed"), "{e}");
        let e = call_window(d.path(), "nope", None, &mut no_progress).unwrap_err();
        assert!(e.contains("unknown command"), "{e}");
    }

    #[test]
    fn the_windows_request_dispatched_in_process_is_the_same() {
        // What the window's `call` (app/src-tauri) does: no socket, the registry directly.
        let mut reg = Registry::new();
        reg.add("echo", Exposure::Window, |r, call| {
            call.progress(json!(1));
            Ok(json!({"args": r.args, "text": r.text, "origin": r.origin}))
        });
        reg.add("quit", Exposure::Local, |_, _| Ok(json!("bye")));
        let got = Mutex::new(vec![]);
        let progress = |p| got.lock().unwrap().push(p);
        let v = reg.dispatch(&window_request("echo", Some(json!({"within_ly": 20}))), &progress).unwrap();
        assert_eq!(v, json!({"args": {"within_ly": 20}, "text": null, "origin": "app_window"}));
        assert_eq!(*got.lock().unwrap(), vec![json!(1)]);
        let v = reg.dispatch(&window_request("echo", Some(json!("gear down"))), &|_| {}).unwrap();
        assert_eq!(v, json!({"args": null, "text": "gear down", "origin": "app_window"}));
        let e = reg.dispatch(&window_request("quit", None), &|_| {}).unwrap_err();
        assert!(e.starts_with("not allowed"), "{e}");
    }

    #[test]
    fn the_windows_status_call_gives_up_on_a_core_that_never_answers() {
        let d = tempfile::tempdir().unwrap();
        // A core that accepts the connection and reads the request but never answers.
        let l = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let ep = Endpoint { port: l.local_addr().unwrap().port(), token: "t".into(), pid: 1, version: "x".into() };
        std::fs::write(d.path().join(FILE), serde_json::to_string(&ep).unwrap()).unwrap();
        std::thread::spawn(move || {
            for s in l.incoming().flatten() {
                // Held open until the client hangs up.
                let _ = std::io::copy(&mut &s, &mut std::io::sink());
            }
        });
        let (tx, rx) = channel();
        let dir = d.path().to_path_buf();
        std::thread::spawn(move || tx.send(call_window(&dir, "status", None, &mut no_progress)));
        // The client's own bound ends the call; this wait only stops the test hanging without it.
        let r = rx.recv_timeout(SHORT + Duration::from_secs(20)).expect("the status call never gave up");
        let e = r.unwrap_err();
        assert!(e.starts_with("waiting for the core's reply to status"), "{e}");
    }
}
