//! The recorded network (astromech-xk0n): a world core's HTTP requests are answered from its
//! `net.json` and its `net` steps, over the control API as the window calls, and none is
//! unrecorded. No test here sleeps, reads the wall clock or retries.

mod support;

use serde_json::{json, Value};
use support::{world, WorldCore};

/// The `http` observations from `since`.
fn requests(core: &WorldCore, since: u64) -> Vec<Value> {
    let seen = core.call("world_observed", Some(json!({ "since": since }))).unwrap();
    seen["observed"].as_array().unwrap().iter().filter(|o| o["kind"] == "http").cloned().collect()
}

/// The window's system-name search is answered from the recorded Spansh body, then, from the
/// `net` step that takes Spansh down, with the lookup's own error for the recorded 503.
#[test]
fn a_window_lookup_is_answered_from_the_recording_then_from_a_net_step() {
    let core = WorldCore::start(&world("network"));
    let names = core.call_window("galaxy_systems", Some(json!({ "prefix": "Shin" }))).unwrap();
    assert_eq!(names, json!(["Shingon", "Shinigami", "Shinarui", "Shinrarta Dezhra"]));
    core.step("spansh-down").unwrap();
    let e = core.call_window("galaxy_systems", Some(json!({ "prefix": "Sol" }))).unwrap_err();
    assert_eq!(e, "Spansh is having trouble (HTTP 503). Try again later.");
    let urls: Vec<(Value, Value)> =
        requests(&core, 0).iter().map(|o| (o["url"].clone(), o["unrecorded"].clone())).collect();
    assert_eq!(
        urls,
        [
            (json!("https://spansh.co.uk/api/systems/field_values/system_names?q=Shin"), json!(false)),
            (json!("https://spansh.co.uk/api/systems/field_values/system_names?q=Sol"), json!(false)),
        ]
    );
    assert!(core.quit().success());
}

/// The downloads (the update feed, the speech models, the neutron star file's checksum) reach the
/// world's replay server, and EDSM's flight log its in-process replay, each answered with the
/// recorded 503; none is unrecorded.
#[test]
fn downloads_and_the_flight_log_are_answered_from_the_recording() {
    let core = WorldCore::start(&world("network"));
    let wait = |url: &str| core.call("world_wait", Some(json!({ "match": { "kind": "http", "url": url } }))).unwrap();
    core.call_window("check_update", None).unwrap();
    assert_eq!(
        wait("https://github.com/brewer-corp/astromech/releases/latest/download/latest.json")["unrecorded"],
        false
    );
    core.call_window("download_models", None).unwrap();
    let stt = astromech_voice::models::STT.url();
    assert_eq!(wait(&stt)["unrecorded"], false);
    let e = core.call_window("neutron_check", None).unwrap_err();
    assert!(e.contains("503"), "{e}");
    let log = core.call_window("edsm_flight_log", None).unwrap();
    assert_eq!(log, json!({"ok": false, "error": "HTTP 503: <html>Service Unavailable</html>"}));
    let all = requests(&core, 0);
    assert!(all.iter().all(|o| o["unrecorded"] == false), "{all:?}");
    let flight = all.iter().find(|o| o["url"].as_str().unwrap().contains("get-logs")).unwrap();
    assert_eq!(flight["url"], "https://www.edsm.net/api-logs-v1/get-logs?commanderName=<redacted>&apiKey=<redacted>");
    assert!(core.quit().success());
}
