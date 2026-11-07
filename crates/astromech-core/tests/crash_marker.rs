//! The crash marker (astromech_core::crash): a panic the command layer catches leaves none, a
//! panic anywhere else does. Its own test binary, as the panic hook is the process's.

use astromech_api::{Exposure, Registry, Request};
use astromech_core::crash;

#[test]
fn only_a_panic_the_command_layer_does_not_catch_leaves_a_crash_marker() {
    let app = tempfile::tempdir().unwrap();
    crash::install(app.path());

    let mut reg = Registry::new();
    reg.add("boom", Exposure::Window, |_, _| panic!("a handler's bug"));
    let e = reg.dispatch(&Request::new("boom"), &|_| {}).unwrap_err();
    assert!(e.contains("a handler's bug"), "{e}");
    assert_eq!(crash::pending(app.path()), None, "caught and answered: no crash report");

    let ended = std::thread::Builder::new().name("worker".into()).spawn(|| panic!("a thread's bug")).unwrap().join();
    assert!(ended.is_err());
    let marker = crash::pending(app.path()).expect("an uncaught panic leaves a crash marker");
    assert!(marker.contains("thread: worker") && marker.contains("a thread's bug"), "{marker}");
}

/// Set in the child process [`a_second_panic_while_a_caught_one_unwinds_leaves_a_crash_marker`]
/// starts: the app folder its crash marker goes in.
const CHILD_APP_DIR: &str = "ASTROMECH_CRASH_TEST_APP_DIR";

/// A handler's panic whose unwinding panics again aborts the process (Rust can't unwind
/// through two panics), so the command layer never answers it: it leaves a crash marker. The
/// abort runs in a child process (this test binary again, running only this test).
#[test]
fn a_second_panic_while_a_caught_one_unwinds_leaves_a_crash_marker() {
    if let Some(app) = std::env::var_os(CHILD_APP_DIR) {
        crash::install(std::path::Path::new(&app));
        struct PanicsOnDrop;
        impl Drop for PanicsOnDrop {
            fn drop(&mut self) {
                panic!("a second bug while unwinding");
            }
        }
        let mut reg = Registry::new();
        reg.add("boom", Exposure::Window, |_, _| {
            let _d = PanicsOnDrop;
            panic!("a handler's bug")
        });
        let _ = reg.dispatch(&Request::new("boom"), &|_| {});
        unreachable!("the second panic aborts");
    }
    let app = tempfile::tempdir().unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "a_second_panic_while_a_caught_one_unwinds_leaves_a_crash_marker", "--nocapture"])
        .env(CHILD_APP_DIR, app.path())
        .output()
        .unwrap();
    assert!(!child.status.success(), "the child aborted");
    let marker = crash::pending(app.path()).expect("the abort leaves a crash marker");
    // The last hook's: the runtime's own panic about the destructor that panicked.
    assert!(marker.contains("thread: a_second_panic_while_a_caught_one_unwinds"), "{marker}");
}
