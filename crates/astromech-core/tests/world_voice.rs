//! Push-to-talk, the kill switch and the paste button in a world: raw-input samples through the
//! core's real `controls::Mapper` and push-to-talk loop on the world clock, a recorded clip on the
//! world's microphone, the script's transcript, and keys and speech observed. No test here
//! sleeps, reads the wall clock or retries; the only real-time waits are the core's 30 s guards.

mod support;

use serde_json::{json, Value};
use support::{world, WorldCore};

/// The `seq` the next observation will have.
fn next(core: &WorldCore) -> u64 {
    core.call("world_observed", None).unwrap()["next"].as_u64().unwrap()
}

/// The first observation from `since` with every field of `pattern`.
fn wait(core: &WorldCore, pattern: Value, since: u64) -> Value {
    core.call("world_wait", Some(json!({ "match": pattern, "since": since }))).unwrap()
}

/// Every observation of `kind` from `since`, as far as the core has got.
fn seen(core: &WorldCore, kind: &str, since: u64) -> Vec<Value> {
    let all = core.call("world_observed", Some(json!({ "since": since }))).unwrap();
    all["observed"].as_array().unwrap().iter().filter(|o| o["kind"] == kind).cloned().collect()
}

fn said(text: &str) -> Value {
    json!({ "kind": "said", "text": text })
}

fn key(name: &str, up: bool) -> Value {
    json!({ "kind": "key", "key": name, "up": up })
}

fn flags(core: &WorldCore) -> u64 {
    core.call("world_game", None).unwrap()["status_flags"].as_u64().unwrap()
}

const GEAR: u64 = 4;

/// Elite in front, push-to-talk held while "gear down" is said, and released: the clip is
/// transcribed, Key_L goes to the game, the game lowers the gear, and the confirmation is said.
#[test]
fn gear_down_by_voice() {
    let core = WorldCore::start(&world("gear-down"));
    core.step("running").unwrap();
    assert_eq!(core.call_window("status", None).unwrap()["voice"], "voice ready");
    assert_eq!(flags(&core) & GEAR, 0, "the gear starts up");
    let since = next(&core);
    core.step("spoken").unwrap();
    // The transcript runs on the speech-to-text thread, which the world doesn't wait for.
    let down = wait(&core, key("Key_L", false), since);
    let up = wait(&core, key("Key_L", true), since);
    let confirmed = wait(&core, said("Landing gear deployed."), since);
    assert!(down["seq"].as_u64() < up["seq"].as_u64() && up["seq"].as_u64() < confirmed["seq"].as_u64());
    assert_eq!(flags(&core) & GEAR, GEAR, "the reaction to Key_L lowered the gear");
    let keys: Vec<Value> = seen(&core, "key", since).iter().map(|k| k["key"].clone()).collect();
    assert_eq!(keys, ["Key_L", "Key_L"], "one press, nothing else");
    assert!(core.quit().success());
}

/// The kill switch stops everything and says so; a voice command then is denied and sends no
/// key. A short hold leaves it killed; a hold of `rearm_hold_ms` re-arms it. A kill during a
/// live run's verify wait cancels it once the clock passes the wait's 25 ms slice.
#[test]
fn the_kill_switch_denies_and_re_arms_by_holding() {
    let core = WorldCore::start(&world("kill-switch"));
    core.step("running").unwrap();
    let since = next(&core);
    core.step("killed").unwrap();
    wait(&core, said("Kill switch on. All actions stopped."), since);

    let voice = next(&core);
    core.step("denied").unwrap();
    wait(&core, said("Kill switch is on. Re-arm it to allow actions."), voice);
    assert_eq!(seen(&core, "key", since), Vec::<Value>::new(), "nothing reached the game");
    assert_eq!(flags(&core) & GEAR, 0);

    // Each release is settled by the step: the loop has said what it says before it returns.
    let short = next(&core);
    core.step("short-hold").unwrap();
    wait(&core, said("Still killed. Hold the kill switch to re-arm."), short);
    let rearm = next(&core);
    core.step("rearmed").unwrap();
    wait(&core, said("Re-armed."), rearm);
    assert!(seen(&core, "said", rearm).iter().all(|s| s["text"] != "Still killed. Hold the kill switch to re-arm."));

    // A live run of a toggle nothing answers: it presses, then waits to verify.
    let live = next(&core);
    let (tx, run) = std::sync::mpsc::channel();
    let contact = core.contact.clone();
    std::thread::spawn(move || {
        let args = Some(json!({"command": "lights.on"}));
        let _ = tx.send(astromech_api::control::call_window_to(&contact, "voice_test_live", args, &mut |_| {}));
    });
    wait(&core, key("Key_Insert", true), live);
    wait(&core, said("Lights on."), live);
    core.step("killed-while-verifying").unwrap();
    core.step("slice").unwrap();
    // A guard only: without the slice's advance the verify wait never ends.
    let reply = run.recv_timeout(std::time::Duration::from_secs(30)).expect("the live run ended").unwrap();
    assert_eq!(reply["message"], "Pressed, then the kill switch cancelled it.", "{reply}");
    assert!(core.quit().success());
}

/// In the galaxy map the paste button sends Ctrl+V and then Enter, under the manual clock and
/// with no advance; with the map closed it sends nothing.
#[test]
fn the_paste_button_pastes_into_the_galaxy_map() {
    let core = WorldCore::start(&world("paste"));
    core.step("running").unwrap();
    let since = next(&core);
    core.step("pasted").unwrap();
    wait(&core, said("Pasted."), since);
    let keys: Vec<(Value, Value)> =
        seen(&core, "key", since).iter().map(|k| (k["key"].clone(), k["up"].clone())).collect();
    let k = |n: &str, up: bool| (json!(n), json!(up));
    assert_eq!(
        keys,
        [
            k("Key_LeftControl", false),
            k("Key_V", false),
            k("Key_V", true),
            k("Key_LeftControl", true),
            k("Key_Enter", false),
            k("Key_Enter", true)
        ]
    );
    let closed = next(&core);
    core.step("refused").unwrap();
    wait(&core, said("Open the galaxy map and select its search box first."), closed);
    assert_eq!(seen(&core, "key", closed), Vec::<Value>::new());
    assert!(core.quit().success());
}
