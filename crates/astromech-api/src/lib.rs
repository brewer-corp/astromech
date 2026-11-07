//! # astromech-api
//!
//! The command layer: one [`Registry`] of named commands that every way in dispatches
//! through, and the core's local control API ([`control`]) that serves it. The core builds the
//! registry and registers its own handlers alongside the window's commands in [`commands`] and
//! [`lookups`], which share one [`State`].
//!
//! Each command is [`Exposure::Window`] (the app window and local control clients may call it)
//! or [`Exposure::Local`] (local control clients only: `quit`, `open_window`). A request
//! without an origin comes from a local control client; the shell's proxy stamps
//! `origin: "app_window"` on what it forwards, and a `local` command refuses that origin
//! (decision astromech-unh9). The token in `core.json` is the gate, not the origin.
//!
//! Every handler runs behind `catch_unwind`: a panic becomes an error reply naming the command
//! and is logged, and the process carries on. While a handler runs, [`dispatching`] is true on
//! its thread, so the core's panic hook logs such a panic without leaving a crash report for the
//! next start. What recovers after the panic:
//!
//! - A lock the handler held: locked with [`lock`] (or [`Recover`] on any `Mutex` or `RwLock`)
//!   the next caller gets it instead of a poisoned error. This crate's [`State`] and the core's
//!   own state and session state lock this way, so later calls run as normal. A lock taken with
//!   `.lock().unwrap()` stays poisoned and every later call that takes it panics too.
//! - Not what the handler had half done: the state behind the lock is as it left it.
//! - Not a panic on a thread the handler started: that is not caught here, and ends that thread
//!   as any thread's panic does.
//!
//! **Rule:** this crate never depends on `astromech-core`, `astromech-input` or
//! `astromech-policy` (scripts/check-deps.py).

pub mod commands;
pub mod control;
pub mod lookups;
pub mod state;

pub use control::Request;
pub use state::{preload_beside, Folders, GalaxyHolder, Peek, State};

use astromech_proto::Origin;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{LockResult, Mutex, MutexGuard, PoisonError};

thread_local! {
    /// How many handlers this thread is running (a handler may run another, [`Call::run`]).
    static DISPATCHING: Cell<u32> = const { Cell::new(0) };
}

/// Whether this thread is running a command's handler, so a panic now is caught by
/// [`Registry::dispatch`] and answered with an error (the process carries on).
pub fn dispatching() -> bool {
    DISPATCHING.with(|d| d.get() > 0)
}

/// Who may call a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exposure {
    /// The app window and local control clients.
    Window,
    /// Local control clients only (a request with no origin).
    Local,
}

/// What a handler gets besides its request: a way to send progress to the caller before the
/// reply (a `{"progress": V}` line on the control API), and to run another registered command
/// for the same caller.
pub struct Call<'a> {
    registry: &'a Registry,
    origin: Option<Origin>,
    progress: &'a dyn Fn(Value),
}

impl Call<'_> {
    /// Send one progress value to the caller now.
    pub fn progress(&self, v: Value) {
        (self.progress)(v)
    }

    /// Run `cmd` through the registry with the caller's origin, as if the caller had asked for
    /// it (its progress, if any, is not passed on).
    pub fn run(&self, cmd: &str, args: Option<Value>) -> Result<Value, String> {
        let r = Request { args, origin: self.origin, ..Request::new(cmd) };
        self.registry.dispatch(&r, &|_| {})
    }
}

type Handler = Box<dyn Fn(&Request, &Call) -> Result<Value, String> + Send + Sync>;

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
        handler: impl Fn(&Request, &Call) -> Result<Value, String> + Send + Sync + 'static,
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
        handler: impl Fn(A, &Call) -> Result<R, String> + Send + Sync + 'static,
    ) -> &mut Self {
        self.add(name, exposure, move |r, call| {
            let args = serde_json::from_value(r.args.clone().unwrap_or(Value::Null))
                .map_err(|e| format!("bad args for {}: {e}", r.cmd))?;
            let reply = handler(args, call)?;
            serde_json::to_value(reply).map_err(|e| format!("{}: the reply can't be written as JSON: {e}", r.cmd))
        })
    }

    /// Run the request's command: its value, or why it didn't run or failed. `progress` gets
    /// the progress values the command sends before it answers.
    pub fn dispatch(&self, r: &Request, progress: &dyn Fn(Value)) -> Result<Value, String> {
        let Some((exposure, handler)) = self.entries.get(&r.cmd) else {
            return Err(format!("unknown command {:?}", r.cmd));
        };
        if *exposure == Exposure::Local && r.origin.is_some() {
            return Err(format!("not allowed: {} is for local control clients only", r.cmd));
        }
        let call = Call { registry: self, origin: r.origin, progress };
        DISPATCHING.with(|d| d.set(d.get() + 1));
        let ran = catch_unwind(AssertUnwindSafe(|| handler(r, &call)));
        DISPATCHING.with(|d| d.set(d.get() - 1));
        ran.unwrap_or_else(|panic| {
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
    m.lock().recover()
}

/// [`lock`]'s recovery for any lock's result: `m.lock().recover()`, `rw.read().recover()`.
pub trait Recover<G> {
    /// The guard, even if a panic poisoned the lock.
    fn recover(self) -> G;
}

impl<G> Recover<G> for LockResult<G> {
    fn recover(self) -> G {
        self.unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use astromech_proto::Origin;
    use serde::Deserialize;
    use serde_json::json;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, OnceLock};

    /// This test binary's log file (one for every test: the log sink is the process's).
    pub(crate) fn test_log() -> &'static Path {
        static DIR: OnceLock<(tempfile::TempDir, PathBuf)> = OnceLock::new();
        let (_, file) = DIR.get_or_init(|| {
            let d = tempfile::tempdir().unwrap();
            astromech_log::init(d.path(), "api-test", false, Box::new(|s| s.to_string()));
            let file = astromech_log::file(d.path(), "api-test");
            (d, file)
        });
        file
    }

    pub(crate) fn logged() -> String {
        std::fs::read_to_string(test_log()).unwrap_or_default()
    }

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
        test_log();
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
        let log = logged();
        assert!(log.contains("command count panicked: count exploded"), "{log}");
    }

    /// [`dispatching`] is true on a handler's thread while it runs (and in a command it runs),
    /// and false once it has answered, however it ended.
    #[test]
    fn dispatching_is_true_only_while_a_handler_runs() {
        test_log();
        let mut reg = Registry::new();
        reg.add("inner", Exposure::Window, |_, _| Ok(json!(dispatching())));
        reg.add("outer", Exposure::Window, |r, call| {
            if r.text.is_some() {
                panic!("a handler's bug");
            }
            Ok(json!([dispatching(), call.run("inner", None)?, dispatching()]))
        });
        assert!(!dispatching());
        assert_eq!(reg.dispatch(&req("outer", None, None), &|_| {}), Ok(json!([true, true, true])));
        assert!(!dispatching());
        let mut boom = req("outer", None, None);
        boom.text = Some("panic".into());
        assert!(reg.dispatch(&boom, &|_| {}).is_err());
        assert!(!dispatching(), "cleared after a panic too");
    }

    /// A handler runs another command through the registry for the same caller: the window
    /// can't reach a `local` command that way.
    #[test]
    fn a_handler_runs_another_command_as_its_caller() {
        let mut reg = registry();
        reg.add("twice", Exposure::Window, |r, call| {
            let one = call.run("add", r.args.clone())?;
            Ok(json!([one, call.run("quit", None)?]))
        });
        let args = Some(json!({"a": 2, "b": 3}));
        let local = reg.dispatch(&req("twice", args.clone(), None), &|_| {});
        assert_eq!(local, Ok(json!([5, "bye"])));
        let e = reg.dispatch(&req("twice", args, Some(Origin::AppWindow)), &|_| {}).unwrap_err();
        assert!(e.starts_with("not allowed: quit"), "{e}");
    }
}
