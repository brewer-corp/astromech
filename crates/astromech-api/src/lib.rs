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
//! and is logged, and the process carries on. The core's panic hook asks [`panic_is_caught`],
//! so it logs such a panic without leaving a crash report for the next start (but leaves one for
//! a second panic while the first unwinds, which aborts the process). What recovers after the
//! panic:
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

pub mod bindings;
pub mod commands;
pub mod control;
pub mod lookups;
pub mod reply;
pub mod state;

pub use control::Request;
pub use reply::{Answer, Nothing};
pub use state::{preload_beside, Folders, GalaxyHolder, Peek, State};

use astromech_proto::Origin;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{LockResult, Mutex, MutexGuard, PoisonError};
use ts_rs::{TypeVisitor, TS};

/// What this thread is doing, for [`panic_is_caught`].
#[derive(Clone, Copy, PartialEq)]
enum Running {
    /// No handler.
    Idle,
    /// A handler (the innermost, if one ran another through [`Call::run`]).
    Handler,
    /// A handler's panic is unwinding to [`Registry::dispatch`].
    Unwinding,
}

thread_local! {
    static RUNNING: Cell<Running> = const { Cell::new(Running::Idle) };
}

/// For a panic hook: whether the panic starting now will be caught by [`Registry::dispatch`]
/// and answered with an error, so the process carries on. True when this thread runs a handler
/// and no panic is unwinding in it yet; the thread is then marked as unwinding until dispatch
/// catches it. False for a second panic while the first unwinds (Rust aborts the process then),
/// and for a panic outside any handler.
pub fn panic_is_caught() -> bool {
    RUNNING.with(|r| {
        let caught = r.get() == Running::Handler;
        if caught {
            r.set(Running::Unwinding);
        }
        caught
    })
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
    after: std::cell::RefCell<AfterReply>,
}

/// What handlers asked to run once their reply is out ([`Call::after_reply`]).
#[derive(Default)]
#[must_use = "run it once the reply is written"]
pub struct AfterReply(Vec<Box<dyn FnOnce() + Send>>);

impl AfterReply {
    /// Run them, in the order they were asked for.
    pub fn run(self) {
        for f in self.0 {
            f();
        }
    }
}

impl Call<'_> {
    /// Send one progress value to the caller now.
    pub fn progress(&self, v: Value) {
        (self.progress)(v)
    }

    /// Run `f` once the reply has been written to the caller (on the control API, after its
    /// last line is sent; in-process, when [`Registry::dispatch`] returns): for an effect that
    /// would cut the reply off, such as `quit` ending the process.
    pub fn after_reply(&self, f: impl FnOnce() + Send + 'static) {
        self.after.borrow_mut().0.push(Box::new(f));
    }

    /// Run `cmd` through the registry with the caller's origin, as if the caller had asked for
    /// it (its progress, if any, is not passed on).
    pub fn run(&self, cmd: &str, args: Option<Value>) -> Result<Value, String> {
        let r = Request { args, origin: self.origin, ..Request::new(cmd) };
        self.registry.dispatch(&r, &|_| {})
    }
}

type Handler = Box<dyn Fn(&Request, &Call) -> Result<Value, String> + Send + Sync>;

/// One TypeScript type, for [`bindings`]: its name and a walk over it and what it uses.
#[derive(Clone, Copy)]
pub struct Ty {
    name: fn(&ts_rs::Config) -> String,
    walk: fn(&mut bindings::Walk),
}

impl Ty {
    fn of<T: TS + ?Sized + 'static>() -> Self {
        Self { name: |cfg| T::name(cfg), walk: |w| w.visit::<T>() }
    }
}

/// A command's TypeScript types: what the window sends as `args` (`string` for a command that
/// reads the request's `text`), the reply's `value`, and the progress values sent before it.
#[derive(Clone, Copy)]
pub struct Types {
    pub args: Ty,
    pub reply: Ty,
    pub progress: Option<Ty>,
}

struct Entry {
    exposure: Exposure,
    handler: Handler,
    /// None for a command registered with [`Registry::add`], which [`bindings`] refuses.
    types: Option<Types>,
}

/// The commands, by name.
#[derive(Default)]
pub struct Registry {
    entries: BTreeMap<String, Entry>,
}

/// A command just registered with its types, to name the progress values it sends.
pub struct Added<'a>(&'a mut Entry);

impl Added<'_> {
    /// The command sends `P` values through [`Call::progress`] before it answers.
    pub fn progress<P: TS + ?Sized + 'static>(self) {
        if let Some(t) = &mut self.0.types {
            t.progress = Some(Ty::of::<P>());
        }
    }
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, name: &str, exposure: Exposure, handler: Handler, types: Option<Types>) -> Added<'_> {
        use std::collections::btree_map::Entry::{Occupied, Vacant};
        match self.entries.entry(name.to_string()) {
            Occupied(_) => panic!("command {name:?} registered twice"),
            Vacant(v) => Added(v.insert(Entry { exposure, handler, types })),
        }
    }

    /// Register `name` without TypeScript types (tests' own commands): the handler reads what
    /// it needs from the request (`args`, `text`, `route`). [`bindings`] refuses a registry
    /// with such a command. Panics if `name` is already registered: two handlers for one name
    /// is a bug.
    pub fn add(
        &mut self,
        name: &str,
        exposure: Exposure,
        handler: impl Fn(&Request, &Call) -> Result<Value, String> + Send + Sync + 'static,
    ) -> &mut Self {
        self.insert(name, exposure, Box::new(handler), None);
        self
    }

    /// Register `name` with typed arguments and reply: the request's `args` (null when absent,
    /// so `()` takes none) parsed as `A`, or a "bad args" error naming the command; the reply
    /// is `R` as JSON.
    pub fn add_typed<A: DeserializeOwned + TS + 'static, R: Serialize + TS + 'static>(
        &mut self,
        name: &str,
        exposure: Exposure,
        handler: impl Fn(A, &Call) -> Result<R, String> + Send + Sync + 'static,
    ) -> Added<'_> {
        self.add_request::<A, R>(name, exposure, move |r, call| {
            let args = serde_json::from_value(r.args.clone().unwrap_or(Value::Null))
                .map_err(|e| format!("bad args for {}: {e}", r.cmd))?;
            handler(args, call)
        })
    }

    /// Register `name` with a typed reply and a handler that reads the request itself: `A` is
    /// what the window sends as `args` for it, `String` when the handler reads the request's
    /// `text` (the window's `call(cmd, "…")`), `()` when it reads neither.
    pub fn add_request<A: TS + ?Sized + 'static, R: Serialize + TS + 'static>(
        &mut self,
        name: &str,
        exposure: Exposure,
        handler: impl Fn(&Request, &Call) -> Result<R, String> + Send + Sync + 'static,
    ) -> Added<'_> {
        let types = Types { args: Ty::of::<A>(), reply: Ty::of::<R>(), progress: None };
        let handler: Handler = Box::new(move |r, call| {
            let reply = handler(r, call)?;
            serde_json::to_value(reply).map_err(|e| format!("{}: the reply can't be written as JSON: {e}", r.cmd))
        });
        self.insert(name, exposure, handler, Some(types))
    }

    /// Every command: its name, who may call it and its types (None if registered with
    /// [`Registry::add`]).
    pub fn commands(&self) -> impl Iterator<Item = (&str, Exposure, Option<&Types>)> {
        self.entries.iter().map(|(n, e)| (n.as_str(), e.exposure, e.types.as_ref()))
    }

    /// Run the request's command: its value, or why it didn't run or failed. `progress` gets
    /// the progress values the command sends before it answers.
    pub fn dispatch(&self, r: &Request, progress: &dyn Fn(Value)) -> Result<Value, String> {
        let (reply, after) = self.dispatch_then(r, progress);
        after.run();
        reply
    }

    /// [`Registry::dispatch`], handing back what the handler asked to run after its reply
    /// ([`Call::after_reply`]) for the caller to run once the reply is written.
    pub fn dispatch_then(&self, r: &Request, progress: &dyn Fn(Value)) -> (Result<Value, String>, AfterReply) {
        let Some(Entry { exposure, handler, .. }) = self.entries.get(&r.cmd) else {
            return (Err(format!("unknown command {:?}", r.cmd)), AfterReply::default());
        };
        if *exposure == Exposure::Local && r.origin.is_some() {
            return (Err(format!("not allowed: {} is for local control clients only", r.cmd)), AfterReply::default());
        }
        let call = Call { registry: self, origin: r.origin, progress, after: Default::default() };
        let outer = RUNNING.with(|r| r.replace(Running::Handler));
        let ran = catch_unwind(AssertUnwindSafe(|| handler(r, &call)));
        RUNNING.with(|r| r.set(outer));
        let reply = ran.unwrap_or_else(|panic| {
            let why = panic
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "no message".into());
            astromech_log::log(&format!("command {} panicked: {why}", r.cmd));
            Err(format!("{} failed with an internal error: {why}", r.cmd))
        });
        (reply, call.after.into_inner())
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

    #[derive(Deserialize, ts_rs::TS)]
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

    /// [`panic_is_caught`] (as a panic hook calls it) is true for the first panic in a handler,
    /// in a command it runs too, and false for a second one while the first unwinds, outside a
    /// handler, and after the handler has answered.
    #[test]
    fn a_panic_is_caught_only_once_inside_a_handler() {
        let mut reg = Registry::new();
        reg.add("inner", Exposure::Window, |_, _| Ok(json!(panic_is_caught())));
        reg.add("outer", Exposure::Window, |_, call| {
            let inner = call.run("inner", None)?;
            Ok(json!([inner, panic_is_caught(), panic_is_caught()]))
        });
        assert!(!panic_is_caught());
        let once = Ok(json!([true, true, false]));
        assert_eq!(reg.dispatch(&req("outer", None, None), &|_| {}), once);
        assert!(!panic_is_caught());
        assert_eq!(reg.dispatch(&req("outer", None, None), &|_| {}), once, "each call starts afresh");
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
