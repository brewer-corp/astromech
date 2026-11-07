//! World mode (`astromech-core --world`): the real core headless in a world's temp copy, with its
//! boundaries' fakes recording what reaches them, and the user's own folders never touched.

mod support;

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read};
use support::{files_in, world, WorldCore};

/// Every variable the core's folders could fall back to points at the helper's empty home,
/// which is still empty after the run ([`WorldCore::quit`] checks it once the core has exited).
#[test]
fn the_minimal_world_runs_headless_and_leaves_the_users_folders_alone() {
    let core = WorldCore::start(&world("minimal"));

    let status = core.call_window("status", None).unwrap();
    assert_eq!(status["app_dir"], json!(core.app_dir), "{status}");
    let config = status["config"].as_str().unwrap().to_string();

    // The window's call opens the config in the editor: the world's opener records it.
    let r = core.call_window("open_config", None).unwrap();
    assert_eq!(r["ok"], true, "{r}");
    let edit = core.call("world_wait", Some(json!({"match": {"kind": "edit"}}))).unwrap();
    assert_eq!(edit["path"], json!(config), "{edit}");
    assert!(std::path::Path::new(&config).is_file(), "the world's config is in its temp copy");

    let r = core.call_window("autostart_on", None).unwrap();
    assert_eq!(r["ok"], true, "{r}");
    let on = core.call("world_wait", Some(json!({"match": {"kind": "autostart", "on": true}}))).unwrap();
    assert!(on["command"].as_str().unwrap().ends_with("--background"), "{on}");
    assert_eq!(core.call_window("status", None).unwrap()["autostart"], true, "status reads the world's Run value");

    // A second start's or the tray's "open the window" is recorded, never started.
    core.call("open_window", None).unwrap();
    let window = core.call("world_wait", Some(json!({"match": {"kind": "window"}}))).unwrap();
    assert_eq!(window["route"], Value::Null, "{window}");

    // Besides the tray's refreshes (autostart_on asks for one).
    let seen = core.call("world_observed", None).unwrap();
    let all = seen["observed"].as_array().unwrap();
    assert_eq!(seen["next"], all.len());
    let kinds: Vec<&Value> = all.iter().map(|o| &o["kind"]).filter(|k| *k != "tray").collect();
    assert_eq!(kinds, [&json!("edit"), &json!("autostart"), &json!("window")], "{seen}");
    let later = core.call("world_observed", Some(json!({"since": 1}))).unwrap();
    let later: Vec<&Value> = later["observed"].as_array().unwrap().iter().filter(|o| o["kind"] != "tray").collect();
    assert_eq!(later, [&on, &window]);

    // The world's commands are local: the window's origin is refused.
    for cmd in ["world_observed", "world_wait"] {
        let e = core.call_window(cmd, None).unwrap_err();
        assert!(e.starts_with("not allowed"), "{cmd}: {e}");
    }

    assert!(files_in(core.home()).is_empty(), "the run wrote into the user's folders: {:?}", files_in(core.home()));
    let app_dir = core.app_dir.clone();
    let exit = core.quit();
    assert!(exit.success(), "{exit:?}");
    // On Windows the core's open files can keep its temp folder; elsewhere it is removed.
    if cfg!(not(windows)) {
        assert!(!app_dir.exists(), "the world's temp folder is removed on quit: {}", app_dir.display());
    }
}

/// A world that switches EDDN and EDSM on, with a commander and a key, has them on: its
/// uploads go to its recorded network, never the real services (astromech-xk0n).
#[test]
fn a_worlds_sharing_switches_are_its_own() {
    let w = tempfile::tempdir().unwrap();
    std::fs::write(
        w.path().join("astromech.toml"),
        "[eddn]\nenabled = true\n[edsm]\nenabled = true\ncommander = \"Jameson\"\n",
    )
    .unwrap();
    std::fs::write(w.path().join("secrets.json"), r#"{"edsm-api-key": "k3y"}"#).unwrap();
    let core = WorldCore::start(w.path());
    let eddn = core.call_window("eddn_log", None).unwrap();
    assert_eq!(eddn["enabled"], true, "{eddn}");
    let edsm = core.call_window("edsm_status", None).unwrap();
    assert_eq!((&edsm["enabled"], &edsm["uploading"]), (&json!(true), &json!(true)), "{edsm}");
    assert_eq!((&edsm["commander"], &edsm["has_key"]), (&json!("Jameson"), &json!(true)), "the rest is the world's");
    assert!(core.quit().success());
}

/// Set by [`a_killed_test_process_ends_its_world_core`] for the copy of this test binary it
/// starts and kills.
const KILLED_PARENT: &str = "ASTROMECH_TEST_KILLED_PARENT";

/// The parent the kill test starts: a world core, its app folder and home on stdout, then
/// nothing until it is killed.
#[test]
#[ignore = "started by a_killed_test_process_ends_its_world_core"]
fn parent_for_the_kill_test() {
    if std::env::var_os(KILLED_PARENT).is_none() {
        return;
    }
    let core = WorldCore::start(&world("minimal"));
    println!("world-core {} {}", core.app_dir.display(), core.home().display());
    let _ = std::io::stdin().read_line(&mut String::new());
}

/// A test process killed outright (a CI cancel) can't drop its [`WorldCore`]; the core sees its
/// stdin close and quits, removing its temp world. The parent's stderr is a pipe the core
/// inherits too, so reading it to its end returns once both have exited: no wait on time.
#[test]
fn a_killed_test_process_ends_its_world_core() {
    let mut parent = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["parent_for_the_kill_test", "--exact", "--ignored", "--nocapture", "--test-threads=1"])
        .env(KILLED_PARENT, "1")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let mut out = BufReader::new(parent.stdout.take().unwrap());
    let mut stderr = parent.stderr.take().unwrap();
    let line = loop {
        let mut l = String::new();
        assert_ne!(out.read_line(&mut l).unwrap(), 0, "the parent exited before starting its world core");
        // libtest's "test … " may come first on the line.
        if let Some((_, rest)) = l.trim_end().split_once("world-core ") {
            break rest.to_string();
        }
    };
    let (app_dir, home) = line.split_once(' ').unwrap();
    parent.kill().unwrap();
    parent.wait().unwrap();
    // A guard only (finding astromech-1uyn): if the core outlives its parent, its end of the pipe
    // stays open and this read would wait forever.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut err = String::new();
        let _ = stderr.read_to_string(&mut err);
        let _ = tx.send(err);
    });
    let err = rx
        .recv_timeout(std::time::Duration::from_secs(30))
        .expect("the world core outlived its killed parent: the end of its stdin didn't quit it");
    // The killed parent couldn't remove its (empty) home.
    let home = std::path::Path::new(home);
    assert!(files_in(home).is_empty(), "the world core wrote into its home: {:?}", files_in(home));
    std::fs::remove_dir(home).unwrap();
    if cfg!(not(windows)) {
        assert!(!std::path::Path::new(app_dir).exists(), "the world core left its temp world {app_dir}: {err}");
    }
}
