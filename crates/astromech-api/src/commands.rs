//! The window's journal and local-file commands, backed by [`State`]: keybinds, materials,
//! engineering, Odyssey, Powerplay, colonisation, exploration, the logbook and the station
//! docked at. None of them uses the star map or the network. Each is a `window` command with
//! typed arguments (the window's names, in snake_case) and a typed reply.

use crate::state::{Engineers, State};
use crate::{Exposure::Window, Registry};
use astromech_keybinds::api::SlotEdit;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Deserialize)]
struct PathArg {
    path: PathBuf,
}

#[derive(Deserialize)]
struct EditArg {
    edit: SlotEdit,
}

#[derive(Deserialize)]
struct CopyArgs {
    src: PathBuf,
    name: String,
}

#[derive(Deserialize)]
struct ActiveArgs {
    name: String,
    contexts: Vec<usize>,
}

#[derive(Deserialize)]
struct BackupArg {
    backup: PathBuf,
}

#[derive(Deserialize)]
struct OverrideArgs {
    device: String,
    control: String,
    joy: Option<String>,
}

#[derive(Deserialize)]
struct PlanArg {
    plan: astromech_engineering::Plan,
}

#[derive(Deserialize)]
struct PicksArg {
    picks: Vec<astromech_odyssey::Pick>,
}

#[derive(Deserialize)]
struct LogbookArgs {
    query: String,
    cmdr: Option<String>,
}

/// Register the commands on `reg`, all sharing `state`.
pub fn register(state: &Arc<State>, reg: &mut Registry) {
    // Keybinds (M1): presets are read and edited through astromech-keybinds (every write backs up
    // first); the core reloads on its own Bindings-folder watch.
    let s = state.clone();
    reg.add_typed("binds_state", Window, move |(), _| s.binds(false, |a| Ok(a.state())));
    let s = state.clone();
    reg.add_typed("binds_refresh_devices", Window, move |(), _| {
        s.binds(false, |a| {
            a.refresh_devices();
            Ok(a.state())
        })
    });
    // Forget the discovered folders and devices, e.g. after onboarding set the Bindings folder.
    let s = state.clone();
    reg.add_typed("binds_reset", Window, move |(), _| s.binds(true, |a| Ok(a.state())));
    let s = state.clone();
    reg.add_typed("binds_view", Window, move |x: PathArg, _| s.binds(false, |a| a.view(&x.path)));
    let s = state.clone();
    reg.add_typed("binds_set_slot", Window, move |x: EditArg, _| s.binds(false, |a| a.set_slot(&x.edit)));
    let s = state.clone();
    reg.add_typed("binds_copy", Window, move |x: CopyArgs, _| s.binds(false, |a| a.copy_preset(&x.src, &x.name)));
    let s = state.clone();
    reg.add_typed("binds_set_active", Window, move |x: ActiveArgs, _| {
        s.binds(false, |a| a.set_active(&x.name, &x.contexts))
    });
    let s = state.clone();
    reg.add_typed("binds_restore", Window, move |x: BackupArg, _| s.binds(false, |a| a.restore(&x.backup)));
    let s = state.clone();
    reg.add_typed("binds_set_override", Window, move |x: OverrideArgs, _| {
        s.binds(false, |a| a.set_override(&x.device, &x.control, x.joy.as_deref()))
    });

    // Engineering materials from the journal, laid out like the in-game traders.
    let s = state.clone();
    reg.add_typed("materials_inventory", Window, move |(), _| s.materials());

    // Every owned ship with its modules and current engineering.
    let s = state.clone();
    reg.add_typed("engineering_fleet", Window, move |(), _| Ok(s.engineering()?.fleet.clone()));
    // Materials a build plan needs against the current inventory, with trades and farming.
    let s = state.clone();
    reg.add_typed("engineering_plan", Window, move |x: PlanArg, _| {
        let st = s.engineering()?;
        let ship = st.fleet.ships.iter().find(|s| s.id == x.plan.ship_id).ok_or("ship not in the fleet")?;
        Ok(astromech_engineering::evaluate(ship, &x.plan, &st.ledger))
    });
    // A ship's stats as fitted and with a build plan done.
    let s = state.clone();
    reg.add_typed("engineering_stats", Window, move |x: PlanArg, _| {
        let st = s.engineering()?;
        let ship = st.fleet.ships.iter().find(|s| s.id == x.plan.ship_id).ok_or("ship not in the fleet")?;
        astromech_ship::compare(ship, &x.plan.targets).map_err(|e| e.to_string())
    });
    // Shield, DPS and jump range for every ship card, from the current modules.
    let s = state.clone();
    reg.add_typed("engineering_headlines", Window, move |(), _| Ok(astromech_ship::headlines(&s.engineering()?.fleet)));
    // Ship engineers: where they are, access and rank.
    let s = state.clone();
    reg.add_typed("engineering_engineers", Window, move |(), _| {
        let st = s.engineering()?;
        Ok(Engineers { engineers: st.engineers.clone(), here: st.here.clone() })
    });
    // The engineer unlock guide: every ship and suit engineer as a tech tree with the
    // commander's progress and what is left to unlock each.
    let s = state.clone();
    reg.add_typed("engineering_unlocks", Window, move |(), _| Ok(s.engineering()?.unlocks.clone()));

    // Odyssey micro-resources: inventory, engineers, and the blueprint picker's choices.
    let s = state.clone();
    reg.add_typed("odyssey_view", Window, move |(), _| {
        let (h, p) = s.odyssey()?;
        Ok(astromech_odyssey::view(&h, &p, "ShipLocker.json"))
    });
    // What the picked suit/weapon upgrades and modifications need, against the inventory.
    let s = state.clone();
    reg.add_typed("odyssey_shopping", Window, move |x: PicksArg, _| {
        let (h, p) = s.odyssey()?;
        Ok(astromech_odyssey::shopping(&x.picks, &h, &p.progress))
    });

    // Powerplay standing from the journals.
    let s = state.clone();
    reg.add_typed("powerplay_overview", Window, move |(), _| Ok(s.powerplay()?.overview()));

    // Colonisation projects, being built first, each with what is left to buy.
    let s = state.clone();
    reg.add_typed("colonisation_projects", Window, move |(), _| s.colonisation());

    let s = state.clone();
    reg.add_typed("exploration_system", Window, move |(), _| Ok(s.exploration_system()));
    let s = state.clone();
    reg.add_typed("exploration_unsold", Window, move |(), _| s.exploration_unsold());

    // The logbook from the history the core keeps: places visited matching `query`, their
    // counts and first/last visit, and the timeline.
    let s = state.clone();
    reg.add_typed("logbook", Window, move |x: LogbookArgs, _| {
        s.open_history()?.logbook(x.cmdr.as_deref(), &x.query, 200).map_err(|e| e.to_string())
    });
    // The latest game session's summary (credits by activity, per hour, jumps, bounties);
    // `null` before the first session.
    let s = state.clone();
    reg.add_typed("session_summary", Window, move |(), _| {
        s.open_history()?.session(None, None).map_err(|e| e.to_string())
    });

    // The station docked at: services, economies, faction and its state, pads, and which
    // modules its outfitting sells that the ship flown would be better for. `docked: false`
    // when not docked.
    let s = state.clone();
    reg.add_typed("docked_here", Window, move |(), _| {
        let st = s.engineering()?;
        let ship = st.fleet.ships.iter().find(|s| s.current);
        let state = astromech_station::state_from_journal_dir(&s.journal_dir());
        let book = astromech_station::OutfittingBook::open_read_only(&s.data_dir().join(astromech_station::DB_FILE))
            .map_err(|e| e.to_string())?;
        Ok(astromech_station::DockedHere::new(&state, ship, book.as_ref()))
    });

    // The current ship's and commander's numbers for a trade route: cargo capacity and jump
    // range from the latest `Loadout`, credits from `Status.json` (else the latest `LoadGame`),
    // and the station docked at.
    let s = state.clone();
    reg.add_typed("trade_route_defaults", Window, move |(), _| {
        let dir = s.journal_dir();
        let gs = astromech_station::state_from_journal_dir(&dir);
        let balance = std::fs::read(dir.join("Status.json"))
            .ok()
            .and_then(|b| serde_json::from_slice::<serde_json::Value>(&b).ok())
            .and_then(|v| v["Balance"].as_i64());
        Ok(astromech_market::route::Defaults {
            capacity: gs.cargo_capacity,
            credits: balance.or(gs.credits),
            range: gs.max_jump_range.filter(|r| *r > 1.0),
            station: if gs.docked { gs.station.clone() } else { None },
            system: gs.system.clone(),
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::{call, serve, Request};
    use crate::Folders;
    use astromech_proto::Origin;
    use serde_json::{json, Value};

    const PRESET: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="Mine" MajorVersion="4" MinorVersion="2">
	<KeyboardLayout>en-US</KeyboardLayout>
	<NightVisionToggle>
		<Primary Device="Keyboard" Key="Key_V" />
		<Secondary Device="{NoDevice}" Key="" />
	</NightVisionToggle>
</Root>
"#;

    /// The window (through the shell, `app_window`) and another local client edit the same
    /// presets in turn, each seeing the other's edit, also after the other reset the keybinds.
    #[test]
    fn two_callers_in_turn_see_each_others_binds_edits() {
        let d = tempfile::tempdir().unwrap();
        let bindings = d.path().join("Bindings");
        std::fs::create_dir_all(&bindings).unwrap();
        std::fs::write(bindings.join("Mine.4.2.binds"), PRESET).unwrap();
        std::fs::write(bindings.join("StartPreset.4.start"), "Mine\nMine\nMine\nMine").unwrap();
        let b = bindings.clone();
        let state = Arc::new(State::new(d.path().join("data"), move || Folders {
            bindings: Some(b.clone()),
            ..Folders::default()
        }));
        let mut reg = Registry::new();
        register(&state, &mut reg);
        let app = d.path().join("app");
        let _srv = serve(&app, Arc::new(reg)).unwrap();
        let ask = |origin: Option<Origin>, cmd: &str, args: Option<Value>| {
            call(&app, Request { args, origin, ..Request::new(cmd) }, None, &mut |_| {}).unwrap()
        };
        let (window, other) = (Some(Origin::AppWindow), None);

        let path = ask(window, "binds_state", None)["presets"][0]["path"].clone();
        let night = |v: &Value| v.to_string().contains("Key_N");
        assert!(!night(&ask(other, "binds_view", Some(json!({ "path": path })))));
        let edit = json!({ "edit": {
            "path": path, "action": "NightVisionToggle", "slot": "Secondary", "analogue": false,
            "binding": { "slot": "Secondary", "device": "Keyboard", "key": "Key_N", "modifiers": [] },
        }});
        ask(window, "binds_set_slot", Some(edit));
        assert!(night(&ask(other, "binds_view", Some(json!({ "path": path })))));
        let reset = ask(other, "binds_reset", None);
        assert_eq!(reset["backups"].as_array().unwrap().len(), 1, "{reset}");
        assert!(night(&ask(window, "binds_view", Some(json!({ "path": path })))));
        assert_eq!(ask(window, "binds_state", None)["backups"], reset["backups"]);
    }
}
