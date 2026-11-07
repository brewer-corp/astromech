//! Every command the window may call answers over a running core's control API (success
//! criterion one-command-list of mission astromech-kjfs). The window's `call()` accepts only the
//! registry's `window` commands (astromech-96dv); this test calls each of them, as the shell's
//! proxy sends it, on a core serving its registry on the real control API.
//!
//! No network and no external program: the remote-data client's transport refuses, the neutron
//! star file comes from a local server that answers 503, and `open_external` opens with the
//! test's opener. A command whose effect has no seam yet is called with state that makes it
//! refuse before the effect, or is not called; [`Case::short`] and [`Case::not_called`] give
//! the missing seam. Those are the commands not yet run for real, which the scripted world of
//! astromech-vucf runs.

use super::tests::{core_with_state, offline, CONFIG_EDITS};
use super::*;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

/// What a command's call must give.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Want {
    /// The envelope is ok, and so is the reply's own `ok` when it has one.
    Ok,
    /// The handler's own error (in the envelope or the reply), containing this text.
    Fails(&'static str),
    /// Not called: the effect has no seam and no state stops it first on every platform this
    /// test runs on (Windows CI too). The text names the seam. An allowance for this test only:
    /// astromech-vucf's headless core calls every command.
    NotCalled(&'static str),
}

/// One `window` command with its example args.
struct Case {
    cmd: &'static str,
    args: Option<Value>,
    want: Want,
    /// Why the call doesn't run the command for real: the effect with no seam, and what stops
    /// it first.
    short: Option<&'static str>,
}

impl Case {
    fn new(cmd: &'static str, args: Option<Value>, want: Want) -> Self {
        Self { cmd, args, want, short: None }
    }
    /// Called, but refused before an effect that has no seam yet (`why`).
    fn short(mut self, why: &'static str) -> Self {
        self.short = Some(why);
        self
    }
    fn not_called(cmd: &'static str, why: &'static str) -> Self {
        Self { cmd, args: None, want: Want::NotCalled(why), short: Some(why) }
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

/// The `window` commands this test does not run for real, each refused before its effect or
/// not called; the table gives each one's missing seam. astromech-vucf's scripted world gives
/// them seams and takes them off this list.
const NOT_RUN_FOR_REAL: &[&str] = &[
    "check_update",
    "install_update",
    "download_models",
    "autostart_on",
    "autostart_off",
    "open_config",
    "save_diagnostics",
    "edsm_flight_log",
    "voice_preview",
    "calibrate_record",
    "mic_check",
];

/// Answers every request on 127.0.0.1 with a 503 until the test process ends.
fn unavailable_server() -> String {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", l.local_addr().unwrap());
    std::thread::spawn(move || {
        for s in l.incoming() {
            let Ok(mut s) = s else { continue };
            let mut r = BufReader::new(s.try_clone().unwrap());
            let mut line = String::new();
            while r.read_line(&mut line).is_ok_and(|n| n > 2) {
                line.clear();
            }
            let _ = write!(s, "HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        }
    });
    base
}

const PRESET: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="Mine" MajorVersion="4" MinorVersion="2">
	<KeyboardLayout>en-US</KeyboardLayout>
	<NightVisionToggle>
		<Primary Device="Keyboard" Key="Key_V" />
		<Secondary Device="{NoDevice}" Key="" />
	</NightVisionToggle>
</Root>
"#;

const JOURNAL: &str = r#"{ "timestamp":"2026-11-01T10:00:00Z", "event":"Fileheader", "part":1, "gameversion":"4.2.0", "build":"r1" }
{ "timestamp":"2026-11-01T10:00:01Z", "event":"Commander", "FID":"F1", "Name":"Test" }
{ "timestamp":"2026-11-01T10:00:01Z", "event":"Materials", "Raw":[{"Name":"iron","Count":30}], "Manufactured":[], "Encoded":[] }
{ "timestamp":"2026-11-01T10:00:02Z", "event":"LoadGame", "Commander":"Test", "FID":"F1", "Ship":"asp", "ShipID":1, "Credits":1000 }
{ "timestamp":"2026-11-01T10:00:03Z", "event":"Loadout", "Ship":"asp", "ShipID":1, "ShipName":"", "ShipIdent":"", "CargoCapacity":64, "FuelCapacity":{"Main":32.0,"Reserve":0.63}, "UnladenMass":400.6, "MaxJumpRange":30.0, "Modules":[{"Slot":"PowerPlant","Item":"int_powerplant_size4_class5","On":true,"Priority":0},{"Slot":"FrameShiftDrive","Item":"int_hyperdrive_size5_class5","On":true,"Priority":0}] }
{ "timestamp":"2026-11-01T10:00:04Z", "event":"Location", "StarSystem":"Sol", "SystemAddress":10477373803, "StarPos":[0.0,0.0,0.0], "Docked":true, "StationName":"Abraham Lincoln", "StationType":"Orbis", "MarketID":128016640 }
"#;

/// The window's commands, each with the args it is called with here. Every `window` command
/// of the registry has exactly one entry.
fn cases(d: &Path) -> Vec<Case> {
    let preset = d.join("Bindings").join("Mine.4.2.binds");
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
    // The remote-data client's error when its transport refuses.
    let offline = "Cannot reach";
    vec![
        // Status and the tray's commands.
        ok("status", None),
        ok("ack_crash", None),
        ok("kill_switch", None),
        ok("pause_voice", None),
        ok("resume_voice", None),
        ok("shell_info", None),
        ok("onboarding_done", None),
        ok("find_elite", None),
        ok("check_update", None).short(
            "the update check fetches from GitHub with no transport seam; called while a check is \
             marked running, so its thread stops first",
        ),
        fails("install_update", None, "no update ready").short(
            "installing replaces the running app, with no seam; called with no update staged, so it \
             refuses",
        ),
        ok("download_models", None).short(
            "the speech models download has no transport seam; called while a download is marked \
             running, so it returns first",
        ),
        Case::not_called("autostart_on", "start with Windows writes the real HKCU Run key, with no registry seam"),
        Case::not_called("autostart_off", "start with Windows writes the real HKCU Run key, with no registry seam"),
        Case::not_called("open_config", "it opens the config in Notepad (xdg-open elsewhere), with no opener seam"),
        fails("save_diagnostics", None, "os error").short(
            "the saved zip is revealed in Explorer (xdg-open elsewhere) with no opener seam; called \
             with a file where its folder goes, so saving fails first",
        ),
        // Settings.
        ok("callouts", None),
        ok("set_callout", Some(json!({"id": "scan", "enabled": false, "cooldown_s": null}))),
        ok("set_ptt", Some(json!({"device": "Keyboard", "key": "Key_F13"}))),
        ok("set_kill_switch", Some(Value::Null)),
        ok("set_mic", Some(json!({"mic": "USB"}))),
        ok("set_folders", Some(json!({"bindings_dir": d.join("Bindings"), "journal_dir": d.join("journals")}))),
        ok("set_voice", Some(json!({"speaker": "bf_emma", "speed": 1.0, "colour": "cockpit"}))),
        ok("set_eddn", Some(json!({"enabled": false}))),
        ok("eddn_log", None),
        ok("set_edsm", Some(json!({"enabled": false, "commander": null, "api_key": null}))),
        ok("edsm_status", None),
        fails("edsm_flight_log", None, "set your EDSM commander name and API key first")
            .short("EDSM's HTTP client has no transport seam; called without an EDSM key, so it refuses"),
        ok("voice_settings", None),
        fails("voice_preview", None, "speech output is off").short(
            "the preview speaks through Kokoro and the speakers, with no audio seam; called with \
             speech output off, so it refuses",
        ),
        ok("prospector_thresholds", None),
        ok("prospector_set", Some(json!(r#"{"ore":"platinum","threshold":35}"#))),
        ok("shopping_set", Some(json!(r#"{"ship_id":1,"materials":[{"id":"iron","until":10}]}"#))),
        // The microphone.
        ok("mic_list", None),
        fails("calibrate_record", None, "calibration hasn't started").short(
            "recording takes the real microphone, with no recorder seam; called before a calibration \
             starts, so it refuses",
        ),
        ok("calibrate_start", None),
        fails("calibrate_finish", None, "calibration needs silence"),
        ok("calibrate_cancel", None),
        Case::not_called("mic_check", "it records from the real microphone on Windows, with no recorder seam"),
        // The voice tester.
        ok("voice_test", Some(json!("deploy hardpoints"))),
        ok("voice_test_poll", None),
        ok("voice_test_stop", None),
        fails("voice_test_live", Some(json!({"command": "hardpoints.deploy", "from": "Sol"})), "Elite isn't running"),
        ok("learned_phrases", None),
        ok("learned_forget", Some(json!("nothing"))),
        fails("exobiology", None, "Elite isn't running"),
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
            Some(json!({"backup": d.join("no-such-backup.binds")})),
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
        // Lookups: the remote-data client refuses.
        fails("galaxy_systems", Some(json!({"prefix": "Sol"})), offline),
        fails("galaxy_nearby", near(json!({"query": "famine"})), offline),
        fails("galaxy_sites", near(json!({"kind": "brain_trees"})), offline),
        fails("stations_nearby", near(json!({"want": {"kind": "service", "service": "material_trader"}})), offline),
        fails("colony_candidates", near(json!({"interest": "rings"})), offline),
        fails("trade_search", near(json!({"side": "sell", "commodities": ["gold"]})), offline),
        fails("trade_anywhere", near(json!({"side": "sell", "commodity": "gold"})), offline),
        // The hold is empty, so no market is looked up.
        ok("trade_cargo", near(json!({}))),
        fails(
            "trade_route",
            near(
                json!({"ask": {"kind": "single", "station": null, "capacity": 64, "credits": 1000, "max_hop_ly": 30}}),
            ),
            offline,
        ),
        fails(
            "mining_hotspots",
            near(json!({"want": {"ore": "platinum", "ring_type": null, "reserve": null}})),
            offline,
        ),
        fails(
            "colonisation_buy",
            near(json!({"needs": [{"symbol": "steel", "name": "Steel", "tonnes": 10}]})),
            offline,
        ),
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
        ok("route_copy", None),
        ok("route_clear", None),
        fails("route_resume", None, "no route to resume"),
        ok("route_plot_ship", None),
        fails(
            "route_plot",
            Some(json!({"ask": {"plotter": "exact", "to": "Maia", "efficiency": 60, "refuel_every_scoopable": true}})),
            offline,
        ),
        ok("neutron_file", None),
        fails("neutron_check", None, "503"),
        fails("neutron_download", None, "503"),
        ok("open_external", Some(json!({"target": "https://github.com/brewer-corp/astromech/issues"}))),
    ]
}

/// Every `window` command of a core's registry, called through its control API as the shell's
/// proxy sends it, answers: ok, or the handler's own error.
#[test]
fn every_window_command_answers_over_the_control_api() {
    let _g = CONFIG_EDITS.lock().unwrap_or_else(|e| e.into_inner());
    let d = tempfile::tempdir().unwrap();
    let journals = d.path().join("journals");
    let bindings = d.path().join("Bindings");
    let data = d.path().join("data");
    for dir in [&journals, &bindings, &data, &d.path().join("game")] {
        std::fs::create_dir_all(dir).unwrap();
    }
    std::fs::write(journals.join("Journal.2026-11-01T100000.01.log"), JOURNAL).unwrap();
    std::fs::write(journals.join("Status.json"), r#"{"Balance": 12345}"#).unwrap();
    std::fs::write(bindings.join("Mine.4.2.binds"), PRESET).unwrap();
    std::fs::write(bindings.join("StartPreset.4.start"), "Mine\nMine\nMine\nMine").unwrap();
    astromech_history::History::open(&data.join("history.sqlite"))
        .unwrap()
        .import(&journals, &Default::default())
        .unwrap();
    let config = format!(
        "data_dir = '{}'\n[game]\njournal_dir = '{}'\nbindings_dir = '{}'\ngame_dir = '{}'\n\
         [voice]\ntts = false\nmodels_dir = '{}'\nworker = '{}'\n",
        data.display(),
        journals.display(),
        bindings.display(),
        d.path().join("game").display(),
        d.path().join("models").display(),
        d.path().join("no-worker").display(),
    );
    let opened = Arc::new(Mutex::new(Vec::<String>::new()));
    let o = opened.clone();
    let neutron = unavailable_server();
    let core = core_with_state(d.path(), &config, offline(), |s| {
        s.with_opener(move |t| {
            o.lock().recover().push(t.to_string());
            Ok(())
        })
        .with_devices(Default::default)
        .with_neutron_url(&neutron)
    });
    // The state that stops the commands with no seam before their effect.
    core.update.lock().recover().snap.status = UpStatus::Checking;
    *core.models.lock().recover() = Some("downloading speech models…".into());
    std::fs::write(core.app_dir.join("diagnostics"), "a file where the diagnostics folder goes").unwrap();

    let app = d.path().join("app");
    let reg = Arc::new(core.commands());
    let _srv = control::serve(&app, reg.clone()).unwrap();
    assert!(control::read_endpoint(&app).is_some_and(|e| !e.token.is_empty()), "core.json names the port and token");

    let cases = cases(d.path());
    let window: Vec<&str> = reg.commands().filter(|(_, e, _)| *e == Exposure::Window).map(|(n, _, _)| n).collect();
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

    let mut wrong = vec![];
    for c in &cases {
        if let Want::NotCalled(_) = c.want {
            continue;
        }
        let got = control::call_window(&app, c.cmd, c.args.clone(), &mut |_| {});
        // The handler's own error is in the envelope or, for the core's commands, in the reply.
        let error = match &got {
            Err(e) => Some(e.clone()),
            Ok(v) if v.get("ok") == Some(&json!(false)) => Some(v["error"].as_str().unwrap_or("").to_string()),
            Ok(_) => None,
        };
        let fine = match (&error, c.want) {
            (Some(e), _) if not_the_handlers(e) => false,
            (None, Want::Ok) => true,
            (Some(e), Want::Fails(text)) => e.contains(text),
            _ => false,
        };
        if !fine {
            wrong.push(format!("{}: wanted {:?}, got {got:?}", c.cmd, c.want));
        }
    }
    assert!(wrong.is_empty(), "{} of {} calls answered wrongly:\n{}", wrong.len(), cases.len(), wrong.join("\n"));
    assert_eq!(*opened.lock().recover(), ["https://github.com/brewer-corp/astromech/issues"]);
}
