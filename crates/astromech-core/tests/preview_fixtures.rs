//! The browser preview's fixtures (app/src/preview/fixtures/) are the real core's replies, dumped
//! from a headless core in a world, and this test fails when one drifts (decision astromech-1zhg:
//! fixtures are recorded inputs, never invented replies).
//!
//! [`fixtures`] lists every file in that folder. For each file, or each top-level key of a
//! composite one, it gives the window command and args the headless core answers it with, in a
//! world under `worlds/preview/` (synthetic journals and recorded network) at a mark of its
//! steps, or why the core doesn't produce it. The test calls each as the window does
//! (`origin: "app_window"`), masks the volatile fields (the file's own value is kept), and
//! compares the reply with the file. A hand edit to a dumped fixture fails it.
//!
//! Refresh the files after a deliberate change to the core or a world with
//! `ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-core --test preview_fixtures`, then check
//! the diff, and that `pnpm check` and `pnpm test` in app/ still pass.

mod support;

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use support::{world, WorldCore};

/// Where a part of a fixture comes from.
enum From {
    /// The core's reply to `cmd` with `args` in the world `worlds/preview/<world>`, once its steps
    /// have run to the mark `at` (none: before any step), at `pick` in the reply (a JSON pointer;
    /// "" is all of it). A string `"$/<pointer>"` in `args` stands for the file's value there (the
    /// sample's own input, such as the plan it asks for). `keep`: pointers into the part whose
    /// value is volatile (a path in the world's temp folder, a label); the file's is kept.
    Core { world: &'static str, at: Option<&'static str>, cmd: &'static str, args: Value, pick: &'static str, keep: &'static [&'static str] },
    /// Not a reply but the sample's own input to one (`"$/…"` above), kept as written.
    Input(&'static str),
    /// Not produced by the core, and why.
    Not(&'static str),
}

/// A part of a fixture: `key` is a JSON pointer into the file ("" is the whole file).
struct Part {
    key: &'static str,
    from: From,
}

fn core(world: &'static str, at: Option<&'static str>, cmd: &'static str, args: Value) -> From {
    From::Core { world, at, cmd, args, pick: "", keep: &[] }
}

fn whole(from: From) -> Vec<Part> {
    vec![Part { key: "", from }]
}

/// Every fixture file, by name, and where each part of it comes from.
fn fixtures() -> BTreeMap<&'static str, Vec<Part>> {
    let mut t = BTreeMap::new();
    t.insert(
        "logbook.json",
        whole(core("history", Some("imported"), "logbook", json!({"query": "", "cmdr": null}))),
    );
    t.insert("session.json", whole(core("history", Some("imported"), "session_summary", Value::Null)));
    t.insert(
        "mining.json",
        whole(From::Core { world: "mining", at: Some("mined"), cmd: "status", args: Value::Null, pick: "/mining", keep: &[] }),
    );
    t.insert(
        "materials.json",
        whole(From::Core { world: "materials", at: None, cmd: "materials_inventory", args: Value::Null, pick: "", keep: &["/source"] }),
    );
    t.insert("unlocks.json", whole(core("unlocks", None, "engineering_unlocks", Value::Null)));
    t.insert(
        "engineering.json",
        vec![
            Part {
                key: "/fleet",
                from: From::Core { world: "engineering", at: None, cmd: "engineering_fleet", args: Value::Null, pick: "", keep: &["/source"] },
            },
            Part { key: "/plan_input", from: From::Input("the plan the sample's numbers are for") },
            Part { key: "/plan", from: core("engineering", None, "engineering_plan", json!({"plan": "$/plan_input"})) },
            Part { key: "/engineers", from: core("engineering", None, "engineering_engineers", Value::Null) },
            Part { key: "/trip", from: core("engineering", None, "engineering_trip", json!({"plan": "$/plan_input"})) },
        ],
    );
    t.insert(
        "odyssey.json",
        vec![
            Part {
                key: "/view",
                from: From::Core { world: "odyssey", at: None, cmd: "odyssey_view", args: Value::Null, pick: "", keep: &["/source"] },
            },
            Part { key: "/picks", from: From::Input("the picks the sample's shopping list is for") },
            Part { key: "/shopping", from: core("odyssey", None, "odyssey_shopping", json!({"picks": "$/picks"})) },
        ],
    );
    t
}

/// The folder of the fixtures.
fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/src/preview/fixtures")
}

/// `v` with every `"$/<pointer>"` string replaced by the file's value there.
fn fill(v: &Value, file: &Value) -> Value {
    match v {
        Value::String(s) if s.starts_with("$/") => {
            file.pointer(&s[1..]).unwrap_or_else(|| panic!("args name {s}, which the file doesn't have")).clone()
        }
        Value::Array(a) => Value::Array(a.iter().map(|v| fill(v, file)).collect()),
        Value::Object(o) => Value::Object(o.iter().map(|(k, v)| (k.clone(), fill(v, file))).collect()),
        v => v.clone(),
    }
}

/// The first few paths where `a` and `b` differ, as `path: a ≠ b`.
fn differences(a: &Value, b: &Value, at: String, out: &mut Vec<String>) {
    if out.len() >= 6 {
        return;
    }
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            for k in x.keys().chain(y.keys().filter(|k| !x.contains_key(*k))) {
                let none = Value::Null;
                let (p, q) = (x.get(k), y.get(k));
                if p.is_none() || q.is_none() {
                    let show = |v: Option<&Value>| v.map_or("(missing)".to_string(), short);
                    out.push(format!("{at}/{k}: {} ≠ {}", show(p), show(q)));
                } else {
                    differences(p.unwrap_or(&none), q.unwrap_or(&none), format!("{at}/{k}"), out);
                }
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (p, q)) in x.iter().zip(y).enumerate() {
                differences(p, q, format!("{at}/{i}"), out);
            }
        }
        _ if a != b => out.push(format!("{}: {} ≠ {}", if at.is_empty() { "/" } else { &at }, short(a), short(b))),
        _ => {}
    }
}

fn short(v: &Value) -> String {
    let s = v.to_string();
    if s.chars().count() > 80 {
        format!("{}…", s.chars().take(80).collect::<String>())
    } else {
        s
    }
}

/// Set `ptr` in `v` to `new` ("" replaces `v`).
fn put(v: &mut Value, ptr: &str, new: Value) {
    *v.pointer_mut(ptr).unwrap_or_else(|| panic!("no {ptr} to write")) = new;
}

#[test]
fn every_preview_fixture_is_the_cores_reply_or_says_why_not() {
    let update = std::env::var_os("ASTROMECH_UPDATE_FIXTURES").is_some_and(|v| v == "1");
    let table = fixtures();
    let dir = folder();
    let on_disk: BTreeSet<String> = std::fs::read_dir(&dir)
        .expect("read the fixtures folder")
        .map(|e| e.expect("an entry").file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".json"))
        .collect();
    let listed: BTreeSet<String> = table.keys().map(|k| k.to_string()).collect();
    let mut problems = vec![];
    for n in on_disk.difference(&listed) {
        problems.push(format!("{n} has no entry in the table"));
    }
    for n in listed.difference(&on_disk) {
        problems.push(format!("the table lists {n}, which isn't in the folder"));
    }

    let mut files: BTreeMap<&str, Value> = BTreeMap::new();
    for name in table.keys().filter(|n| on_disk.contains(**n)) {
        let text = std::fs::read_to_string(dir.join(name)).unwrap_or_else(|e| panic!("read {name}: {e}"));
        files.insert(name, serde_json::from_str(&text).unwrap_or_else(|e| panic!("{name} isn't JSON: {e}")));
    }

    let table: BTreeMap<_, _> = table.into_iter().filter(|(n, _)| files.contains_key(n)).collect();
    // A composite file's parts cover its top-level keys, each once.
    for (name, parts) in &table {
        if parts.iter().any(|p| p.key.is_empty()) {
            assert_eq!(parts.len(), 1, "{name}: a whole-file part must be its only one");
            continue;
        }
        let tops: BTreeSet<String> = parts.iter().map(|p| p.key.split('/').nth(1).unwrap_or_default().to_string()).collect();
        let keys: BTreeSet<String> = files[name].as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
        if tops != keys {
            problems.push(format!("{name}: the table's parts cover {tops:?}, the file has {keys:?}"));
        }
    }

    // One world core per world, its steps run in the table's order.
    let mut cores: BTreeMap<&str, (WorldCore, Option<&str>)> = BTreeMap::new();
    let mut changed: BTreeSet<&str> = BTreeSet::new();
    for (name, parts) in &table {
        for part in parts {
            let From::Core { world: w, at, cmd, args, pick, keep } = &part.from else { continue };
            let (core, reached) = cores.entry(w).or_insert_with(|| (WorldCore::start(&world(&format!("preview/{w}"))), None));
            if at != reached {
                let to = at.unwrap_or_else(|| panic!("{name}{}: a part before any step after one at {reached:?}", part.key));
                core.step(to).unwrap_or_else(|e| panic!("{w}: world_step to {to}: {e}"));
                *reached = Some(to);
            }
            let file = &files[name];
            let args = fill(args, file);
            let args = if args.is_null() { None } else { Some(args) };
            let reply = core.call_window(cmd, args).unwrap_or_else(|e| panic!("{name}{}: {cmd}: {e}", part.key));
            let mut got = reply.pointer(pick).unwrap_or_else(|| panic!("{name}{}: {cmd}'s reply has no {pick}", part.key)).clone();
            let had = file.pointer(part.key).cloned();
            for k in keep.iter() {
                if let (Some(kept), Some(slot)) = (had.as_ref().and_then(|h| h.pointer(k)), got.pointer_mut(k)) {
                    *slot = kept.clone();
                }
            }
            if had.as_ref() == Some(&got) {
                continue;
            }
            if update {
                put(files.get_mut(name).expect("read above"), part.key, got);
                changed.insert(name);
            } else {
                let mut d = vec![];
                differences(had.as_ref().unwrap_or(&Value::Null), &got, String::new(), &mut d);
                problems.push(format!("{name}{} is not {w}'s {cmd} reply (fixture ≠ core):\n    {}", part.key, d.join("\n    ")));
            }
        }
    }
    for (w, (core, _)) in cores {
        assert!(core.quit().success(), "the {w} world core didn't quit cleanly");
    }
    for name in changed {
        let text = serde_json::to_string_pretty(&files[name]).expect("serialise") + "\n";
        std::fs::write(dir.join(name), text).unwrap_or_else(|e| panic!("write {name}: {e}"));
        eprintln!("rewrote {name}");
    }
    assert!(
        problems.is_empty(),
        "{}\n\nRefresh with ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-core --test preview_fixtures after a deliberate change",
        problems.join("\n")
    );
}
