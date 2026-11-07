//! The control port on the core's wire (crates/astromech-api/src/control.rs): one JSON line
//! `{token, cmd, args?}` in, `{"ok": true, "value": V}` or `{"ok": false, "error": E}` back, one
//! request per connection, each on its own thread. The endpoint `{port, token, pid, version}` is
//! written to `core.json` in the spike's folder.

use crate::win::{dir, log};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;

pub type Handler = Arc<dyn Fn(&str, Option<Value>) -> Result<Value, String> + Send + Sync>;

fn token() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut s = String::new();
    for _ in 0..2 {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos());
        s += &format!("{:016x}", h.finish());
    }
    s
}

pub fn serve(handler: Handler) -> std::io::Result<()> {
    let l = TcpListener::bind(("127.0.0.1", 0))?;
    let tok = token();
    let ep = json!({
        "port": l.local_addr()?.port(),
        "token": tok,
        "pid": std::process::id(),
        "version": env!("CARGO_PKG_VERSION"),
    });
    std::fs::create_dir_all(dir())?;
    std::fs::write(dir().join("core.json"), ep.to_string())?;
    log(&format!("control: listening on port {}", ep["port"]));
    std::thread::Builder::new().name("control".into()).spawn(move || {
        for s in l.incoming().flatten() {
            let h = handler.clone();
            let tok = tok.clone();
            let _ = std::thread::Builder::new().name("control-conn".into()).spawn(move || {
                let _ = handle(s, &tok, &h);
            });
        }
    })?;
    Ok(())
}

fn handle(s: TcpStream, tok: &str, h: &Handler) -> std::io::Result<()> {
    s.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut line = String::new();
    BufReader::new(s.try_clone()?).take(64 * 1024).read_line(&mut line)?;
    let reply = match serde_json::from_str::<Value>(&line) {
        Ok(r) if r["token"] == tok => match r["cmd"].as_str() {
            Some(cmd) => h(cmd, r.get("args").cloned()),
            None => Err("no cmd".into()),
        },
        Ok(_) => Err("bad token".into()),
        Err(e) => Err(format!("bad request: {e}")),
    };
    let reply = match reply {
        Ok(value) => json!({"ok": true, "value": value}),
        Err(error) => json!({"ok": false, "error": error}),
    };
    (&s).write_all(format!("{reply}\n").as_bytes())
}

/// One request to the running instance (the second start's handover).
pub fn call(cmd: &str, args: Option<Value>) -> Result<Value, String> {
    let ep: Value = serde_json::from_str(&std::fs::read_to_string(dir().join("core.json")).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let port = ep["port"].as_u64().ok_or("no port")? as u16;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let s = TcpStream::connect_timeout(&addr, Duration::from_secs(2)).map_err(|e| e.to_string())?;
    s.set_read_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;
    let mut req = json!({ "token": ep["token"], "cmd": cmd });
    if let Some(a) = args {
        req["args"] = a;
    }
    (&s).write_all(format!("{req}\n").as_bytes()).map_err(|e| e.to_string())?;
    let mut line = String::new();
    BufReader::new(s).read_line(&mut line).map_err(|e| e.to_string())?;
    let mut v: Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
    if v["ok"] == true {
        Ok(v["value"].take())
    } else {
        Err(v["error"].as_str().unwrap_or("no reason").to_string())
    }
}
