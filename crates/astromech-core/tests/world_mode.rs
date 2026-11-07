//! World mode (`astromech-core --world`): the real core headless in a world's temp copy, with its
//! boundaries' fakes recording what reaches them, and the user's own folders never touched.

mod support;

use serde_json::{json, Value};
use support::{world, WorldCore};

/// Every variable the core's folders could fall back to points at one empty folder, which must
/// still be empty after the run.
#[test]
fn the_minimal_world_runs_headless_and_leaves_the_users_folders_alone() {
    let home = tempfile::tempdir().unwrap();
    let env = ["HOME", "XDG_DATA_HOME", "LOCALAPPDATA", "APPDATA", "USERPROFILE"].map(|k| (k, home.path()));
    let core = WorldCore::start(&world("minimal"), &env);

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

    let seen = core.call("world_observed", None).unwrap();
    let kinds: Vec<&Value> = seen["observed"].as_array().unwrap().iter().map(|o| &o["kind"]).collect();
    assert_eq!(kinds, [&json!("edit"), &json!("autostart")], "{seen}");
    assert_eq!(seen["next"], 2);
    let later = core.call("world_observed", Some(json!({"since": 1}))).unwrap();
    assert_eq!(later["observed"], json!([on]));

    // The world's commands are local: the window's origin is refused.
    for cmd in ["world_observed", "world_wait"] {
        let e = core.call_window(cmd, None).unwrap_err();
        assert!(e.starts_with("not allowed"), "{cmd}: {e}");
    }

    let app_dir = core.app_dir.clone();
    let exit = core.quit();
    assert!(exit.success(), "{exit:?}");
    let left: Vec<_> = std::fs::read_dir(home.path()).unwrap().map(|e| e.unwrap().path()).collect();
    assert!(left.is_empty(), "the run wrote into the user's folders: {left:?}");
    // On Windows the core's open files can keep its temp folder; elsewhere it is removed.
    if cfg!(not(windows)) {
        assert!(!app_dir.exists(), "the world's temp folder is removed on quit: {}", app_dir.display());
    }
}
