//! Every command the window may call answers over a headless core's control API (success
//! criterion one-command-list of mission astromech-kjfs). The window's `call()` accepts only the
//! registry's `window` commands (astromech-96dv); this test starts `astromech-core --world
//! worlds/every-command` and calls each of them, as the shell's proxy sends it, against the
//! world: its recorded network, its game folder and session, and its fakes for the opener, Start
//! with Windows, the microphone, the speaker and the keys. Each call's effect is checked in the
//! reply or in what the world observed. Only `install_update` is not run for real
//! ([`NOT_RUN_FOR_REAL`]).
//!
//! Each example's args are also parsed into the command's args type and written back
//! (`world_args`), and every field of the example must come back with its value: a misspelt
//! field would otherwise parse to nothing and answer ok as a no-op (finding astromech-nq3c).
//!
//! No test here sleeps, reads the wall clock or retries; the only real-time waits are the world
//! core's 30 s guards.

mod support;

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use support::{world, WorldCore};

/// What a command's call must give.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Want {
    /// The envelope is ok, and so is the reply's own `ok` when it has one.
    Ok,
    /// The handler's own error (in the envelope or the reply), containing this text.
    Fails(&'static str),
}

/// A check of a call's reply and of the observations its `sees` matched, in order.
type Check = fn(&Value, &[Value]) -> Result<(), String>;

/// One `window` command with its example args.
struct Case {
    cmd: &'static str,
    args: Option<Value>,
    want: Want,
    /// The world's steps to run, to this mark, before the call.
    step: Option<&'static str>,
    /// Observations the call must cause, in order (each a `world_wait` pattern), from the call on.
    sees: Vec<Value>,
    check: Option<Check>,
    /// Called a second time, which must cause this observation (the kill switch, to re-arm).
    again: Option<Value>,
    /// Why the call doesn't run the command for real, and what stops it first.
    short: Option<&'static str>,
}

impl Case {
    fn new(cmd: &'static str, args: Option<Value>, want: Want) -> Self {
        Self { cmd, args, want, step: None, sees: vec![], check: None, again: None, short: None }
    }
    fn after(mut self, mark: &'static str) -> Self {
        self.step = Some(mark);
        self
    }
    fn sees(mut self, pattern: Value) -> Self {
        self.sees.push(pattern);
        self
    }
    fn check(mut self, f: Check) -> Self {
        self.check = Some(f);
        self
    }
    fn again(mut self, pattern: Value) -> Self {
        self.again = Some(pattern);
        self
    }
    /// Called, but refused before an effect the test can't let happen (`why`).
    fn short(mut self, why: &'static str) -> Self {
        self.short = Some(why);
        self
    }
}

fn ok(cmd: &'static str, args: Option<Value>) -> Case {
    Case::new(cmd, args, Want::Ok)
}

fn fails(cmd: &'static str, args: Option<Value>, error: &'static str) -> Case {
    Case::new(cmd, args, Want::Fails(error))
}

/// Errors that are never a handler's own: the registry's (an unknown command, args that don't
/// parse, a `local` command from the window, a caught panic) and the control API's.
fn not_the_handlers(error: &str) -> bool {
    error.starts_with("unknown command")
        || error.starts_with("bad args for")
        || error.starts_with("not allowed")
        || error.contains("failed with an internal error")
        || error.starts_with("bad request")
        || error == "bad token"
}

/// The `window` commands not run for real: installing an update replaces the running app, so
/// it is called with no update staged and refuses before its effect.
const NOT_RUN_FOR_REAL: &[&str] = &["install_update"];

/// The update the world's recorded feed offers (`worlds/every-command/latest.json`, the project's
/// real signed release feed), and where its zip is fetched from.
const RECORDED_UPDATE: &str = "1.0.0-alpha.4";
const RECORDED_ZIP: &str =
    "https://github.com/brewer-corp/astromech/releases/download/v1.0.0-alpha.4/astromech-1.0.0-alpha.4-windows-x64.zip";

fn http(url: &str) -> Value {
    json!({ "kind": "http", "url": url, "unrecorded": false })
}

fn said(text: &str) -> Value {
    json!({ "kind": "said", "text": text })
}

/// The world's folders, as its config names them.
struct Folders {
    game: PathBuf,
    bindings: PathBuf,
}

/// The window's commands, each with the args it is called with here. Every `window` command
/// of the registry has exactly one entry. They are called in this order.
fn cases(f: &Folders) -> Vec<Case> {
    let preset = f.bindings.join("Mine.4.2.binds");
    let near = |what: Value| {
        let mut v = what;
        v["origin"] = Value::Null;
        v["filters"] = Value::Null;
        Some(v)
    };
    let plan = json!({"plan": {"ship_id": 1, "targets": [], "rolls": [0, 0, 0, 0, 0]}});
    let binding = json!({"slot": "Secondary", "device": "Keyboard", "key": "Key_N", "modifiers": []});
    let edit = json!({"edit": {
        "path": preset, "action": "NightVisionToggle", "slot": "Secondary", "analogue": false, "binding": binding,
    }});
    let filters = json!({
        "pad": null, "pad_set": false, "exclude_carriers": true, "exclude_odyssey": false, "max_age_days": null,
        "max_arrival_ls": null, "max_distance_ly": 42, "min_volume": null,
    });
    vec![
        // Status and the tray's commands.
        ok("status", None),
        ok("ack_crash", None),
        ok("kill_switch", None).sees(said("Kill switch on. All actions stopped.")).again(said("Re-armed.")),
        ok("pause_voice", None),
        ok("resume_voice", None),
        ok("shell_info", None),
        ok("onboarding_done", None),
        ok("find_elite", None),
        fails("install_update", None, "no update ready")
            .short("installing replaces the running app; called with no update staged, so it refuses"),
        // The recorded feed's signature is checked against the built-in key: only then is its
        // newer zip fetched (the world's recording answers it 503).
        ok("check_update", None)
            .sees(http("https://github.com/brewer-corp/astromech/releases/latest/download/latest.json"))
            .sees(http("https://github.com/brewer-corp/astromech/releases/latest/download/latest.json.sig"))
            .sees(http(RECORDED_ZIP))
            .sees(json!({"kind": "tray", "check": "Check for updates"})),
        ok("download_models", None)
            .sees(http(&astromech_voice::models::STT.url()))
            .sees(json!({"kind": "tray", "line": "Elite running · voice ready"})),
        ok("autostart_on", None).sees(json!({"kind": "autostart", "on": true})),
        ok("autostart_off", None).sees(json!({"kind": "autostart", "on": false, "command": null})),
        ok("open_config", None).sees(json!({"kind": "edit"})).check(|_, seen| {
            let path = seen[0]["path"].as_str().unwrap_or_default();
            path.ends_with("astromech.toml").then_some(()).ok_or(format!("opened {path}, not the config"))
        }),
        ok("save_diagnostics", None).sees(json!({"kind": "reveal"})).check(|reply, seen| {
            (seen[0]["path"] == reply["path"] && reply["path"].as_str().is_some_and(|p| p.ends_with(".zip")))
                .then_some(())
                .ok_or(format!("revealed {}, saved {}", seen[0]["path"], reply["path"]))
        }),
        // Settings.
        ok("callouts", None),
        ok("set_callout", Some(json!({"id": "scan", "enabled": false, "cooldown_s": null}))),
        ok("set_ptt", Some(json!({"device": "Keyboard", "key": "Key_F13"}))),
        ok("set_kill_switch", Some(Value::Null)),
        ok("set_mic", Some(json!({"mic": "World microphone"}))),
        ok("set_folders", Some(json!({"bindings_dir": f.bindings, "journal_dir": f.game}))),
        ok("set_voice", Some(json!({"speaker": "bf_emma", "speed": 1.0, "colour": "cockpit"}))),
        ok("set_eddn", Some(json!({"enabled": false}))),
        ok("eddn_log", None),
        ok("set_edsm", Some(json!({"enabled": false, "commander": null, "api_key": null}))),
        ok("edsm_status", None),
        // EDSM's recorded answer to the world's (not real) key.
        fails("edsm_flight_log", None, "Commander name/API Key not found")
            .sees(http("https://www.edsm.net/api-logs-v1/get-logs?commanderName=<redacted>&apiKey=<redacted>")),
        ok("voice_settings", None),
        ok("voice_preview", None).sees(said(astromech_core::speech::PREVIEW_LINE)),
        ok("prospector_thresholds", None),
        ok("prospector_set", Some(json!(r#"{"ore":"platinum","threshold":35}"#))),
        ok("shopping_set", Some(json!(r#"{"ship_id":1,"materials":[{"id":"iron","until":10}]}"#))),
        // The microphone: each recording is the world's clip.
        ok("mic_list", None).check(|r, _| expect(&r["names"], &json!(["World microphone"]))),
        ok("calibrate_start", None),
        ok("calibrate_record", None).after("calibration-clip").check(|r, _| level(&r["level"])),
        fails("calibrate_finish", None, "calibration needs silence"),
        ok("calibrate_cancel", None),
        ok("mic_check", None).after("level-check-clip").check(|r, _| level(&r["level"])),
        // The voice tester.
        ok("voice_test", Some(json!("deploy hardpoints"))),
        ok("voice_test_poll", None),
        ok("voice_test_stop", None),
        // The world's game turns the lights on when Key_Insert goes down, so the run verifies.
        ok("voice_test_live", Some(json!({"command": "lights.on", "from": "Sol"})))
            .sees(json!({"kind": "key", "key": "Key_Insert", "up": false}))
            .sees(json!({"kind": "key", "key": "Key_Insert", "up": true})),
        ok("learned_phrases", None),
        ok("learned_forget", Some(json!("nothing"))),
        ok("exobiology", None),
        // Keybinds.
        ok("binds_state", None),
        ok("binds_refresh_devices", None),
        ok("binds_view", Some(json!({"path": preset}))),
        ok("binds_set_slot", Some(edit)),
        ok("binds_copy", Some(json!({"src": preset, "name": "Copy"}))),
        ok("binds_set_active", Some(json!({"name": "Mine", "contexts": [0, 1, 2, 3]}))),
        ok("binds_set_override", Some(json!({"device": "231D0200", "control": "JoystickX", "joy": null}))),
        fails(
            "binds_restore",
            Some(json!({"backup": f.bindings.join("no-such-backup.binds")})),
            "not one of astromech's backups",
        ),
        ok("binds_reset", None),
        // The journal's pages.
        ok("materials_inventory", None),
        ok("engineering_fleet", None),
        ok("engineering_plan", Some(plan.clone())),
        ok("engineering_stats", Some(plan.clone())),
        ok("engineering_trip", Some(plan)),
        ok("engineering_headlines", None),
        ok("engineering_engineers", None),
        ok("engineering_unlocks", None),
        ok("odyssey_view", None),
        ok("odyssey_shopping", Some(json!({"picks": []}))),
        ok("powerplay_overview", None),
        fails("powerplay_targets", Some(json!({"target": "acquire", "within_ly": 30.0})), "Not pledged to a power"),
        ok("colonisation_projects", None),
        ok("exploration_system", None),
        ok("exploration_unsold", None),
        ok("logbook", Some(json!({"query": "Sol", "cmdr": null}))),
        ok("session_summary", None),
        ok("docked_here", None),
        ok("trade_route_defaults", None),
        ok("outfitting_search", Some(json!({"query": "fsd"}))),
        ok("search_filters", Some(json!({"lookup": "trade.sell"}))),
        ok("set_search_filters", Some(json!({"lookup": "trade.sell", "filters": filters}))),
        // Lookups, answered from the world's recording.
        ok("galaxy_systems", Some(json!({"prefix": "Shin"}))),
        ok("galaxy_nearby", near(json!({"query": "famine"}))),
        ok("galaxy_sites", near(json!({"kind": "brain_trees"}))),
        ok("stations_nearby", near(json!({"want": {"kind": "service", "service": "material_trader"}}))),
        ok("colony_candidates", near(json!({"interest": "rings"}))),
        ok("trade_search", near(json!({"side": "sell", "commodities": ["gold"]}))),
        ok("trade_anywhere", near(json!({"side": "sell", "commodity": "painite"}))),
        // The hold is empty, so no market is looked up.
        ok("trade_cargo", near(json!({}))),
        ok(
            "trade_route",
            near(
                json!({"ask": {"kind": "single", "station": null, "capacity": 64, "credits": 1000, "max_hop_ly": 30}}),
            ),
        ),
        ok("mining_hotspots", near(json!({"want": {"ore": "platinum", "ring_type": null, "reserve": null}}))),
        ok("colonisation_buy", near(json!({"needs": [{"symbol": "steel", "name": "Steel", "tonnes": 10}]}))),
        fails("market_compare", None, "no market seen here yet"),
        // The Route page.
        ok("route", None),
        ok(
            "route_set",
            Some(json!({"name": "Trip", "source": "search", "waypoints": [{"system": "Sol"}, {"system": "Colonia"}]})),
        ),
        ok("route_add", Some(json!({"system": "Deciat", "label": "Farseer", "source": "engineer trip"}))),
        ok("route_skip", None),
        ok("route_back", None),
        ok("route_copy", None).sees(json!({"kind": "clipboard"})),
        ok("route_clear", None),
        fails("route_resume", None, "no route to resume"),
        ok("route_plot_ship", None),
        ok(
            "route_plot",
            Some(json!({"ask": {"plotter": "exact", "to": "Maia", "efficiency": 60, "refuel_every_scoopable": true}})),
        ),
        ok("neutron_file", None),
        fails("neutron_check", None, "503"),
        fails("neutron_download", None, "503"),
        ok("open_external", Some(json!({"target": "https://github.com/brewer-corp/astromech/issues"})))
            .sees(json!({"kind": "open", "target": "https://github.com/brewer-corp/astromech/issues"})),
    ]
}

fn expect(got: &Value, want: &Value) -> Result<(), String> {
    (got == want).then_some(()).ok_or(format!("got {got}, wanted {want}"))
}

/// A level check's reply for the world's clip: a speech level, not silence.
fn level(l: &Value) -> Result<(), String> {
    match l["peak_dbfs"].as_f64() {
        Some(p) if p > -30.0 => Ok(()),
        _ => Err(format!("no speech level in {l}")),
    }
}

/// Where `back` lost or changed a field of `example`: each field of an object (at any depth)
/// must come back with its value; fields `back` adds (the type's defaults) don't matter.
/// Numbers compare by value (`42` and `42.0` are one number).
fn lost(example: &Value, back: &Value, at: &str, out: &mut Vec<String>) {
    match (example, back) {
        (Value::Object(e), Value::Object(b)) => {
            for (k, v) in e {
                let at = format!("{at}.{k}");
                match b.get(k) {
                    Some(bv) => lost(v, bv, &at, out),
                    None => out.push(format!("{at} is lost")),
                }
            }
        }
        (Value::Array(e), Value::Array(b)) if e.len() == b.len() => {
            for (i, (ev, bv)) in e.iter().zip(b).enumerate() {
                lost(ev, bv, &format!("{at}[{i}]"), out);
            }
        }
        (Value::Number(e), Value::Number(b)) if e.as_f64() == b.as_f64() => {}
        (e, b) if e == b => {}
        (e, b) => out.push(format!("{at} is {b}, not {e}")),
    }
}

/// The handler's own error, in the envelope or (for the core's commands) in the reply.
fn handlers_error(got: &Result<Value, String>) -> Option<String> {
    match got {
        Err(e) => Some(e.clone()),
        Ok(v) if v.get("ok") == Some(&json!(false)) => Some(v["error"].as_str().unwrap_or("").to_string()),
        Ok(_) => None,
    }
}

fn next(core: &WorldCore) -> u64 {
    core.call("world_observed", None).unwrap()["next"].as_u64().unwrap()
}

/// Every `window` command of the world core's registry, called through its control API as the
/// shell's proxy sends it, answers ok or with the handler's own error, and does what it does
/// to the world.
#[test]
fn every_window_command_runs_against_the_world() {
    let core = WorldCore::start(&world("every-command"));
    core.step("running").unwrap();
    let status = core.call_window("status", None).unwrap();
    assert_eq!(status["voice"], "voice ready", "{status}");
    let config = PathBuf::from(status["config"].as_str().unwrap());
    let dir = config.parent().unwrap();
    let folders = Folders { game: dir.join("game"), bindings: dir.join("bindings") };
    let cases = cases(&folders);

    // The table is the registry's window commands, each once.
    let commands = core.call("world_commands", None).unwrap();
    let window: Vec<&str> = commands["window"].as_array().unwrap().iter().map(|n| n.as_str().unwrap()).collect();
    let listed: Vec<&str> = cases.iter().map(|c| c.cmd).collect();
    let missing: Vec<&&str> = window.iter().filter(|n| !listed.contains(n)).collect();
    assert!(missing.is_empty(), "window commands with no entry in the table: {missing:?}");
    let extra: Vec<&&str> = listed.iter().filter(|n| !window.contains(n)).collect();
    assert!(extra.is_empty(), "entries that aren't window commands: {extra:?}");
    let mut seen = std::collections::HashSet::new();
    let twice: Vec<&&str> = listed.iter().filter(|n| !seen.insert(**n)).collect();
    assert!(twice.is_empty(), "commands listed twice: {twice:?}");
    let short: Vec<&str> = cases.iter().filter(|c| c.short.is_some()).map(|c| c.cmd).collect();
    assert_eq!(short, NOT_RUN_FOR_REAL, "the commands not run for real");

    // Each example's args survive the trip through the command's args type.
    let mut changed = vec![];
    for c in cases.iter() {
        let Some(args) = &c.args else { continue };
        match core.call("world_args", Some(json!({"cmd": c.cmd, "args": args}))) {
            Ok(back) => lost(args, &back, c.cmd, &mut changed),
            Err(e) => changed.push(format!("{}: its example isn't its args type: {e}", c.cmd)),
        }
    }
    assert!(changed.is_empty(), "example args that don't survive their args type:\n{}", changed.join("\n"));

    let mut wrong = vec![];
    for c in &cases {
        if let Some(mark) = c.step {
            core.step(mark).unwrap();
        }
        let since = next(&core);
        let got = core.call_window(c.cmd, c.args.clone());
        let error = handlers_error(&got);
        let fine = match (&error, c.want) {
            (Some(e), _) if not_the_handlers(e) => false,
            (None, Want::Ok) => true,
            (Some(e), Want::Fails(text)) => e.contains(text),
            _ => false,
        };
        if !fine {
            wrong.push(format!("{}: wanted {:?}, got {got:?}", c.cmd, c.want));
            continue;
        }
        let mut from = since;
        let mut matched = vec![];
        for p in &c.sees {
            match core.call("world_wait", Some(json!({"match": p, "since": from}))) {
                Ok(o) => {
                    from = o["seq"].as_u64().unwrap() + 1;
                    matched.push(o);
                }
                Err(e) => {
                    wrong.push(format!("{}: {e}", c.cmd));
                    break;
                }
            }
        }
        if matched.len() < c.sees.len() {
            continue;
        }
        if let Some(check) = c.check {
            if let Err(e) = check(got.as_ref().unwrap_or(&Value::Null), &matched) {
                wrong.push(format!("{}: {e}", c.cmd));
            }
        }
        if let Some(p) = &c.again {
            let since = next(&core);
            let got = core.call_window(c.cmd, c.args.clone());
            let seen = got.and_then(|_| core.call("world_wait", Some(json!({"match": p, "since": since}))));
            if let Err(e) = seen {
                wrong.push(format!("{} again: {e}", c.cmd));
            }
        }
    }
    assert!(wrong.is_empty(), "{} of {} calls answered wrongly:\n{}", wrong.len(), cases.len(), wrong.join("\n"));

    // The update check ended with its recorded feed verified and its zip refused; the models
    // download with the recorded 503 reported, and the models still missing.
    let status = core.call_window("status", None).unwrap();
    let update = &status["update"];
    assert_eq!((&update["latest"], &update["status"]), (&json!(RECORDED_UPDATE), &json!("error")), "{update}");
    assert!(update["error"].as_str().unwrap().contains("503"), "{update}");
    assert_eq!(status["models_missing"], true, "{status}");
    let log = logged(Path::new(status["log_dir"].as_str().unwrap()));
    assert!(log.contains("speech model download failed") && log.contains("503"), "the download's error is logged");

    // A second start's or the tray's "open the window" is recorded, never started.
    core.call("open_window", Some(json!(null))).unwrap();
    core.call("world_wait", Some(json!({"match": {"kind": "window"}}))).unwrap();

    // Every request was answered from the recording.
    let all = core.call("world_observed", None).unwrap();
    let unrecorded: Vec<&Value> = all["observed"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["kind"] == "http" && o["unrecorded"] != false)
        .collect();
    assert!(unrecorded.is_empty(), "requests the world's recording doesn't hold: {unrecorded:#?}");
    assert!(core.quit().success());
}

/// Every log file under `dir`, read.
fn logged(dir: &Path) -> String {
    let mut text = String::new();
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_file() {
            text += &std::fs::read_to_string(&p).unwrap_or_default();
        }
    }
    text
}
