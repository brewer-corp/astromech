//! # astromech-api
//!
//! The command layer: one [`Registry`] of named commands that every way in dispatches
//! through, and the core's local control API ([`control`]) that serves it. The core builds the
//! registry and registers its own handlers; the shell's commands move in later
//! (astromech-bp23).
//!
//! Each command is [`Exposure::Window`] (the app window and local control clients may call it)
//! or [`Exposure::Local`] (local control clients only: `quit`, `open_window`). A request
//! without an origin comes from a local control client; the shell's proxy stamps
//! `origin: "app_window"` on what it forwards, and a `local` command refuses that origin
//! (decision astromech-unh9). The token in `core.json` is the gate, not the origin.
//!
//! Every handler runs behind `catch_unwind`: a panic becomes an error reply, is logged, and the
//! next call runs as normal. State a handler shares with other calls is locked with [`lock`],
//! which recovers a lock a panicking handler poisoned instead of failing every later call.
//!
//! **Rule:** this crate never depends on `astromech-core`, `astromech-input` or
//! `astromech-policy` (scripts/check-deps.py).

pub mod commands;
pub mod control;
pub mod state;

pub use control::Request;
pub use state::{Folders, State};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Mutex, MutexGuard};

/// Who may call a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exposure {
    /// The app window and local control clients.
    Window,
    /// Local control clients only (a request with no origin).
    Local,
}

/// Sends one progress value to the caller before the reply (a `{"progress": V}` line on the
/// control API).
pub type Progress<'a> = &'a dyn Fn(Value);

type Handler = Box<dyn Fn(&Request, Progress) -> Result<Value, String> + Send + Sync>;

/// The commands, by name.
#[derive(Default)]
pub struct Registry {
    entries: BTreeMap<String, (Exposure, Handler)>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register `name`. The handler reads what it needs from the request (`args`, `text`,
    /// `route`). Panics if `name` is already registered: two handlers for one name is a bug.
    pub fn add(
        &mut self,
        name: &str,
        exposure: Exposure,
        handler: impl Fn(&Request, Progress) -> Result<Value, String> + Send + Sync + 'static,
    ) -> &mut Self {
        let old = self.entries.insert(name.to_string(), (exposure, Box::new(handler)));
        assert!(old.is_none(), "command {name:?} registered twice");
        self
    }

    /// Register `name` with typed arguments and reply: the request's `args` (null when absent,
    /// so `()` takes none) parsed as `A`, or a "bad args" error naming the command; the reply
    /// is `R` as JSON.
    pub fn add_typed<A: DeserializeOwned, R: Serialize>(
        &mut self,
        name: &str,
        exposure: Exposure,
        handler: impl Fn(A, Progress) -> Result<R, String> + Send + Sync + 'static,
    ) -> &mut Self {
        self.add(name, exposure, move |r, progress| {
            let args = serde_json::from_value(r.args.clone().unwrap_or(Value::Null))
                .map_err(|e| format!("bad args for {}: {e}", r.cmd))?;
            let reply = handler(args, progress)?;
            serde_json::to_value(reply).map_err(|e| format!("{}: the reply can't be written as JSON: {e}", r.cmd))
        })
    }

    /// Run the request's command: its value, or why it didn't run or failed.
    pub fn dispatch(&self, r: &Request, progress: Progress) -> Result<Value, String> {
        let Some((exposure, handler)) = self.entries.get(&r.cmd) else {
            return Err(format!("unknown command {:?}", r.cmd));
        };
        if *exposure == Exposure::Local && r.origin.is_some() {
            return Err(format!("not allowed: {} is for local control clients only", r.cmd));
        }
        catch_unwind(AssertUnwindSafe(|| handler(r, progress))).unwrap_or_else(|panic| {
            let why = panic
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "no message".into());
            astromech_log::log(&format!("command {} panicked: {why}", r.cmd));
            Err(format!("{} failed with an internal error: {why}", r.cmd))
        })
    }
}

/// Lock state shared between calls, recovering it if a handler panicked while holding it.
/// The handler that panicked got an error; the state is as it left it.
pub fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_proto::Origin;
    use serde::Deserialize;
    use serde_json::json;
    use std::sync::Arc;

    fn req(cmd: &str, args: Option<Value>, origin: Option<Origin>) -> Request {
        Request { cmd: cmd.into(), args, origin, ..Default::default() }
    }

    #[derive(Deserialize)]
    struct Add {
        a: i64,
        b: i64,
    }

    fn registry() -> Registry {
        let mut reg = Registry::new();
        reg.add_typed("add", Exposure::Window, |x: Add, _| Ok(json!(x.a + x.b)));
        reg.add("quit", Exposure::Local, |_, _| Ok(json!("bye")));
        reg
    }

    #[test]
    fn unknown_names_and_bad_args_name_the_command() {
        let reg = registry();
        let e = reg.dispatch(&req("nope", None, None), &|_| {}).unwrap_err();
        assert!(e.contains("\"nope\""), "{e}");
        let e = reg.dispatch(&req("add", Some(json!({"a": "one", "b": 2})), None), &|_| {}).unwrap_err();
        assert!(e.starts_with("bad args for add:"), "{e}");
        let e = reg.dispatch(&req("add", None, None), &|_| {}).unwrap_err();
        assert!(e.starts_with("bad args for add:"), "{e}");
        assert_eq!(reg.dispatch(&req("add", Some(json!({"a": 1, "b": 2})), None), &|_| {}), Ok(json!(3)));
    }

    #[test]
    fn a_local_command_refuses_the_app_window() {
        let reg = registry();
        let e = reg.dispatch(&req("quit", None, Some(Origin::AppWindow)), &|_| {}).unwrap_err();
        assert!(e.starts_with("not allowed"), "{e}");
        // A local control client (no origin) may call it, and the window may call `window` ones.
        assert_eq!(reg.dispatch(&req("quit", None, None), &|_| {}), Ok(json!("bye")));
        let add = req("add", Some(json!({"a": 1, "b": 1})), Some(Origin::AppWindow));
        assert_eq!(reg.dispatch(&add, &|_| {}), Ok(json!(2)));
    }

    #[test]
    fn a_panic_holding_a_shared_lock_is_an_error_and_the_next_call_runs() {
        let logs = tempfile::tempdir().unwrap();
        astromech_log::init(logs.path(), "api-test", false, Box::new(|s| s.to_string()));
        let shared = Arc::new(Mutex::new(0));
        let mut reg = Registry::new();
        let s = shared.clone();
        reg.add("count", Exposure::Window, move |r, _| {
            let mut n = lock(&s);
            *n += 1;
            if r.text.as_deref() == Some("panic") {
                panic!("count exploded");
            }
            Ok(json!(*n))
        });
        let mut boom = req("count", None, None);
        boom.text = Some("panic".into());
        let e = reg.dispatch(&boom, &|_| {}).unwrap_err();
        assert!(e.contains("count") && e.contains("count exploded"), "{e}");
        assert!(shared.is_poisoned());
        assert_eq!(reg.dispatch(&req("count", None, None), &|_| {}), Ok(json!(2)));
        let log = std::fs::read_to_string(astromech_log::file(logs.path(), "api-test")).unwrap();
        assert!(log.contains("command count panicked: count exploded"), "{log}");
    }
}
