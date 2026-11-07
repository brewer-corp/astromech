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
