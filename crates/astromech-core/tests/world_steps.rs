//! World steps (`world_step`): the game folder, the game process and the clock, each step
//! settled before the call returns. No test here sleeps, reads the wall clock or retries; the
//! only real-time waits are the core's 30 s guards, which fail a stuck call.
#![allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)

mod support;

use serde_json::{json, Value};
use support::{world, world_with, WorldCore};

/// `worlds/jump`'s steps, with the jump line replaced by `jump`.
fn jump_steps(jump: &str) -> String {
    let steps = std::fs::read_to_string(world("jump").join("world.jsonl")).unwrap();
    let line = steps.lines().find(|l| l.contains("\"FSDJump\"")).unwrap();
    steps.replace(line, jump)
}

/// The FSDJump to Alpha Centauri, stamped `at`.
fn jump_at(at: &str) -> String {
    format!(
        r#"{{"journal": {{ "timestamp":"{at}", "event":"FSDJump", "StarSystem":"Alpha Centauri", "SystemAddress":1458376315610, "StarPos":[3.03125,-0.09375,3.15625], "JumpDist":4.38, "FuelUsed":0.4, "FuelLevel":31.6 }}}}"#
    )
}

/// Elite running in Sol, and the route Sol → Alpha Centauri → Barnard's Star set from the
/// window: the next stop is Alpha Centauri, already on the clipboard. Returns the `seq` the next
/// observation will have.
fn running_with_a_route(core: &WorldCore) -> u64 {
    core.step("running").unwrap();
    let status = core.call_window("status", None).unwrap();
    assert_eq!((&status["game"], &status["active"]), (&json!("running"), &json!(true)), "{status}");
    let waypoints = json!([{"system": "Sol"}, {"system": "Alpha Centauri"}, {"system": "Barnard's Star"}]);
    let r = core.call_window("route_set", Some(json!({"name": "Trip", "waypoints": waypoints}))).unwrap();
    assert_eq!(r["view"]["next"]["system"], "Alpha Centauri", "in Sol, so the first stop is passed: {r}");
    let copied = core.call("world_wait", Some(json!({"match": {"kind": "clipboard"}}))).unwrap();
    assert_eq!(copied["text"], "Alpha Centauri");
    core.call("world_observed", None).unwrap()["next"].as_u64().unwrap()
}

fn next_stop(core: &WorldCore) -> Value {
    core.call_window("route", None).unwrap()["view"]["next"]["system"].clone()
}

fn clipboard_since(core: &WorldCore, since: u64) -> Vec<Value> {
    let seen = core.call("world_observed", Some(json!({"since": since}))).unwrap();
    seen["observed"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["kind"] == "clipboard")
        .map(|o| o["text"].clone())
        .collect()
}

/// The jump to the route's next stop moves the route on, copies the stop after it and says it.
#[test]
fn a_jump_to_the_next_stop_copies_the_one_after() {
    let core = WorldCore::start(&world("jump"));
    let since = running_with_a_route(&core);
    core.step("jumped").unwrap();
    let copied = core.call("world_wait", Some(json!({"match": {"kind": "clipboard"}, "since": since}))).unwrap();
    assert_eq!(copied["text"], "Barnard's Star", "{copied}");
    assert_eq!(next_stop(&core), "Barnard's Star");
    // And the callout says it, through the world's speaker.
    let said = json!({"kind": "said", "text": "Next waypoint. Barnard's Star."});
    core.call("world_wait", Some(json!({"match": said, "since": since}))).unwrap();
    assert!(core.quit().success());
}

/// The same jump stamped two hours before the clock, written while Elite is closed, is history
/// when Elite starts again: the session's catch-up reads it silently, so nothing is copied and the
/// route stays where it was. (Appended while the session runs, a line of the game's own journal
/// is live whatever its stamp: the pipeline's rule for the journal the game writes.)
#[test]
fn a_jump_stamped_hours_before_the_clock_is_caught_up_silently() {
    let old = jump_at("2026-11-07T10:00:30Z");
    let steps = jump_steps(&format!("{{\"game\": \"exited\"}}\n{old}\n{{\"game\": \"running\"}}"));
    let w = world_with("jump", Some(&steps));
    let core = WorldCore::start(w.path());
    let since = running_with_a_route(&core);
    core.step("jumped").unwrap();
    assert_eq!(core.call_window("status", None).unwrap()["active"], true);
    let state = core.call("world_game", None).unwrap();
    assert_eq!(state["system"], "Alpha Centauri", "the catch-up read the jump: {state}");
    assert_eq!(clipboard_since(&core, since), Vec::<Value>::new());
    assert_eq!(next_stop(&core), "Alpha Centauri");
    assert!(core.quit().success());
}

/// The game starts a new journal, its lines stamped tens of seconds after the clock: each step
/// moves the clock to its line's stamp before appending it, and the pipeline follows the new
/// journal and copies the stop after the jump. Appended before the clock moved, each line would
/// be stamped more than `AHEAD_SECS` after now, which the pipeline takes for a clock running
/// ahead: the new journal isn't the one being written, the old one (last written 53 s ago) still
/// is, and the clock moving on later doesn't make it look again, so the jump is never read.
#[test]
fn a_later_stamped_line_moves_the_clock_first() {
    // The clock is at 12:00:03 after `running` (one poll).
    let header = r#"{"journal": { "timestamp":"2026-11-07T12:01:00Z", "event":"Fileheader", "part":1, "language":"English/UK", "Odyssey":true, "gameversion":"4.1.0.100", "build":"r300000/r0 " }, "file": "Journal.2026-11-07T120100.01.log"}"#;
    let steps = jump_steps(&format!("{header}\n{}", jump_at("2026-11-07T12:01:30Z")));
    let w = world_with("jump", Some(&steps));
    let core = WorldCore::start(w.path());
    let since = running_with_a_route(&core);
    core.step("jumped").unwrap();
    // The route desk copies on the pipeline's thread, as it handles the line: the step has
    // returned, so it is done.
    assert_eq!(clipboard_since(&core, since), [json!("Barnard's Star")]);
    let state = core.call("world_game", None).unwrap();
    assert_eq!(state["system"], "Alpha Centauri", "{state}");
    assert!(core.quit().success());
}

const START: &str = r#"{"start": "2026-11-07T12:00:00Z"}"#;

/// A world on `worlds/jump`'s game folder with these steps after the header.
fn steps(lines: &[&str]) -> tempfile::TempDir {
    world_with("jump", Some(&format!("{START}\n{}\n", lines.join("\n"))))
}

fn status(core: &WorldCore) -> (Value, Value) {
    let s = core.call_window("status", None).unwrap();
    (s["game"].clone(), s["active"].clone())
}

/// `running`, `launcher` and `exited`: each returns once the session has started or gone, and
/// `status` then reports the game state.
#[test]
fn game_steps_start_and_stop_the_session() {
    let w = steps(&[
        r#"{"game": "launcher"}"#,
        r#"{"mark": "launcher"}"#,
        r#"{"game": "running"}"#,
        r#"{"mark": "running"}"#,
        r#"{"game": "exited"}"#,
        r#"{"mark": "exited"}"#,
    ]);
    let core = WorldCore::start(w.path());
    assert_eq!(status(&core), (json!("not_running"), json!(false)));
    core.step("launcher").unwrap();
    assert_eq!(status(&core), (json!("launcher"), json!(true)), "the launcher pre-warms");
    core.step("running").unwrap();
    assert_eq!(status(&core), (json!("running"), json!(true)));
    assert_eq!(core.call("world_game", None).unwrap()["system"], "Sol", "the session caught up");
    core.step("exited").unwrap();
    assert_eq!(status(&core), (json!("not_running"), json!(false)));
    assert_eq!(core.call("world_game", None).unwrap(), Value::Null);
    assert!(core.quit().success());
}

/// `world_gate {hold: true}` on its own connection, on a thread: it sends "held" into `held` once
/// the gate is held, and returns the reply ("reached": the watcher is stopped at the gate).
fn hold_gate(
    core: &std::sync::Arc<WorldCore>,
    held: std::sync::mpsc::Sender<Value>,
) -> std::thread::JoinHandle<Result<Value, String>> {
    let core = core.clone();
    std::thread::spawn(move || {
        let req = astromech_api::control::Request {
            args: Some(json!({"hold": true})),
            ..astromech_api::control::Request::new("world_gate")
        };
        astromech_api::control::call_to(&core.contact, req, None, &mut |p| held.send(p).unwrap())
    })
}

/// `world_step` to the end on its own connection, on a thread, its reply sent into the channel
/// returned.
fn step_on_a_thread(
    core: &std::sync::Arc<WorldCore>,
) -> (std::thread::JoinHandle<()>, std::sync::mpsc::Receiver<Result<Value, String>>) {
    let (tx, rx) = std::sync::mpsc::channel();
    let core = core.clone();
    (std::thread::spawn(move || tx.send(core.call("world_step", None)).unwrap()), rx)
}

/// From running to exited with nothing between: the `exited` step's own polls are the only
/// time that passes. The watcher is held at the gate on the step's second poll (the one that
/// sees the game gone twice and drops the session) until the rest of the core is parked: the
/// pipeline then waits on the clock with its deadline still ahead, and no step advances it
/// again. Stopping the pipeline and the callout speaker wakes their waits, so the session goes;
/// if a stop waited for its timed wait to run out, the step would never settle, and would fail
/// at its guard.
#[test]
fn the_game_exiting_stops_the_session_with_no_more_time() {
    let w = steps(&[r#"{"game": "running"}"#, r#"{"mark": "running"}"#, r#"{"game": "exited"}"#]);
    let core = std::sync::Arc::new(WorldCore::start(w.path()));
    core.step("running").unwrap();
    assert_eq!(status(&core), (json!("running"), json!(true)));
    let (held_tx, held) = std::sync::mpsc::channel();
    let gate = hold_gate(&core, held_tx);
    assert_eq!(held.recv().unwrap(), "held");
    let (step, stepped) = step_on_a_thread(&core);
    assert_eq!(gate.join().unwrap(), Ok(json!("reached")), "the step's first poll");
    // Through the first poll (one miss), held at the second.
    assert_eq!(core.call("world_gate", Some(json!({"hold": true}))).unwrap(), "reached");
    assert!(stepped.try_recv().is_err(), "the step returned before the watcher's second poll");
    assert_eq!(status(&core), (json!("running"), json!(true)), "one miss doesn't stop the session");
    assert_eq!(core.call("world_gate", Some(json!({"hold": false}))).unwrap(), "released");
    let r = stepped.recv().unwrap().unwrap();
    step.join().unwrap();
    assert_eq!(r["ran"], 1, "{r}");
    assert_eq!(status(&core), (json!("not_running"), json!(false)));
    let core = std::sync::Arc::try_unwrap(core).ok().unwrap();
    assert!(core.quit().success());
}

/// Status.json written whole, then cut off part way, then whole again. A partial write settles
/// (the pipeline read it and couldn't parse it), and the flags stay the last whole file's.
#[test]
fn a_partial_status_write_settles_and_keeps_the_last_flags() {
    let w = steps(&[
        r#"{"game": "running"}"#,
        r#"{"file": "Status.json", "json": { "timestamp":"2026-11-07T12:00:05Z", "event":"Status", "Flags":4, "Flags2":0, "GuiFocus":0 }}"#,
        r#"{"mark": "gear"}"#,
        r#"{"file": "Status.json", "json": { "timestamp":"2026-11-07T12:00:06Z", "event":"Status", "Flags":16777216, "Flags2":0, "GuiFocus":0 }, "partial": true}"#,
        r#"{"mark": "partial"}"#,
        r#"{"file": "Status.json", "json": { "timestamp":"2026-11-07T12:00:07Z", "event":"Status", "Flags":16777216, "Flags2":0, "GuiFocus":0 }}"#,
        r#"{"mark": "supercruise"}"#,
        r#"{"file": "Status.json", "json": { "timestamp":"2026-11-07T12:00:08Z", "event":"Status", "Flags":4, "Flags2":0, "GuiFocus":0 }, "partial": true}"#,
        r#"{"file": "notes.txt", "text": "a file the pipeline doesn't read"}"#,
        r#"{"mark": "end"}"#,
    ]);
    let core = WorldCore::start(w.path());
    let flags = |core: &WorldCore| core.call("world_game", None).unwrap()["status_flags"].clone();
    core.step("gear").unwrap();
    assert_eq!(flags(&core), 4);
    core.step("partial").unwrap();
    assert_eq!(flags(&core), 4, "a file cut off part way changes nothing");
    core.step("supercruise").unwrap();
    assert_eq!(flags(&core), 16777216);
    // Never followed by a whole one, and a file the pipeline ignores: both settle.
    core.step("end").unwrap();
    assert_eq!(flags(&core), 16777216);
    assert!(core.quit().success());
}

/// The race: the game watch is held at a gate in its poll (a world-only hold, over the control
/// API) while the `running` step runs on another connection. The step has woken the watcher, and
/// the watcher has reached the gate, but it hasn't polled, so the step can't have returned: the
/// test reads its reply channel while the gate is still held, with no wait. Once the gate is let
/// go, the step returns, and the session is running.
#[test]
fn a_game_step_waits_for_the_watcher_it_woke() {
    let w = steps(&[r#"{"game": "running"}"#]);
    let core = std::sync::Arc::new(WorldCore::start(w.path()));
    let (held_tx, held) = std::sync::mpsc::channel();
    let gate = hold_gate(&core, held_tx);
    assert_eq!(held.recv().unwrap(), "held");
    let (step, stepped) = step_on_a_thread(&core);
    assert_eq!(gate.join().unwrap(), Ok(json!("reached")), "the watcher reached the gate");
    assert!(stepped.try_recv().is_err(), "the step returned while the watcher was held at the gate");
    assert_eq!(status(&core).0, "not_running", "the watcher hasn't polled");
    assert_eq!(core.call("world_gate", Some(json!({"hold": false}))).unwrap(), "released");
    stepped.recv().unwrap().unwrap();
    step.join().unwrap();
    assert_eq!(status(&core), (json!("running"), json!(true)));
    let core = std::sync::Arc::try_unwrap(core).ok().unwrap();
    assert!(core.quit().success());
}

/// The world's commands are local: the window's origin is refused.
#[test]
fn the_step_commands_are_local() {
    let core = WorldCore::start(&world("jump"));
    for cmd in ["world_step", "world_game", "world_gate"] {
        let e = core.call_window(cmd, None).unwrap_err();
        assert!(e.starts_with("not allowed"), "{cmd}: {e}");
    }
    let e = core.step("nowhere").unwrap_err();
    assert!(e.contains("no mark \"nowhere\" ahead"), "{e}");
    assert!(core.quit().success());
}
