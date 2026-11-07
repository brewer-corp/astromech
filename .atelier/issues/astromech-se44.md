---
created_at: "2026-11-07T06:33:38.680504277+00:00"
id: "astromech-se44"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ndzx"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T09:07:34.633814673+00:00"
status: "done"
title: "Export every command's request and reply types to TypeScript (ts-rs)"
updated_at: "2026-11-07T09:07:34.633814673+00:00"
---

## Goal

Every registered command's request and reply types are exported to TypeScript from Rust, so the window and its fake can be typed against the real core's shapes. Today a fixture can drift from the core unseen.

## Done when

- [ ] Types are derived with `ts-rs` on every astromech-api handler's args and reply, the domain types they return, and the core handlers' args and replies.
- [ ] A registry test writes the files:
  - ts-rs's types go to `app/src/lib/api/types/`;
  - `app/src/lib/api/commands.ts` maps each command name to `{ args, reply, exposure, progress? }`, using the generated type names.

  `cargo test -p astromech-api` fails when the checked-in files differ from what it would write. CONTRIBUTING.md names the command that updates them.
- [ ] The core commands whose replies are built with `json!` reply with typed structs of the same shape. They are in crates/astromech-core/src/app.rs, calibration.rs, callouts.rs, learned.rs, biology.rs and setup.rs.
  - The shapes test `setup_and_calibration_reply_shapes_match_the_window_fake` and the existing control-API tests pass unchanged.
  - No reply's JSON changes: a test compares a few before and after.
- [ ] `serde_json::Value`, which ts-rs types as `JsonValue`, is used only for args or replies named in an allow-list in the registry test with a reason each. The test fails for any other. An expected entry is `set_search_filters`' empty reply only if it can't be `()`. The Close Reason lists the final allow-list.
- [ ] Generated files and the TypeScript check:
  - `cd app && pnpm -s check` passes with the generated files included;
  - `pnpm -s lint` passes, with `commands.ts` linted and `app/src/lib/api/types/` (ts-rs output, not hand-written) excluded in the eslint config, named there;
  - `cargo deny check` passes with the new dependency;
  - `app/src/lib/api/client.ts`, the unused WebSocket placeholder, is deleted.

## Scope

Rust, the generated `app/src/lib/api/` files and the eslint ignore entry. The window starts using them in the typed child.

There are two error paths, and that is accepted. A failed call is the envelope's `{"ok": false, "error"}`. A core command that reports a failure inside its reply (`{"ok": false, "error": …}` within `value`) keeps doing so, typed as such. Folding those into the envelope would rewrite the `.ok` checks in many window modules that astromech-xil2 has just moved. It needs no Rust-side design and can be done later as its own item if wanted.

## Context

ts-rs over tauri-specta: once the shell forwards everything through one proxy, commands aren't Tauri commands. The same types must serve the control API, the headless core and later WASM. ARCHITECTURE.md:421 marks specta 2 as a pre-release. The moved handlers return typed structs already (the two move children).

## Notes

- 2026-11-07 [note] Decision (implementer): the whole-set drift test is in astromech-core (cargo test -p astromech-core --lib bindings), with astromech-api's own test checking only its part (its type files and its commands.ts lines). Reason: the core's handlers and their reply types live in astromech-core, and scripts/check-deps.py forbids astromech-api from depending on astromech-core, dev-dependencies included (it reads every dependency kind from cargo metadata). Moving the core's reply types into astromech-api would move the route follower, update, tester and calibration types out of the core. The generator (astromech_api::bindings) is in astromech-api. For the plan owner to accept or change.

## Report

Implementer report (commits 960792f2 and ddbf1145 on feature/astromech-bp23). Outcome: ts-rs 12.0.1 (MIT; features serde-json-impl) is a workspace dependency. astromech-api's Registry::add_typed and the new add_request (handler reads the request itself; args type String for a command that reads the request's text) record each command's ts-rs types; Added::progress::<P>() names progress values (neutron_download: NeutronProgress). Registry::add stays untyped for tests' own commands, and bindings refuses a registry with one. astromech_api::bindings walks every command's args/reply/progress types, writes one file per exported type to app/src/lib/api/types/ (272 files) and app/src/lib/api/commands.ts (type Commands = { name: { args; reply; exposure: "window"|"local"; progress? } }, importing only the types its lines name), reports two Rust types that would share a TS file and names both, and finds JsonValue per (command, part). Large ints are number. astromech_api::reply::{Answer<T>, Done<T>, Failed, Nothing} type a core command's in-reply {ok: true, ...} / {ok: false, error} (TS: Done<T> = {ok: true} & T; Answer<T> = Done<T> | Failed). Every domain type the commands reach derives ts_rs::TS where defined (22 crates' types). 30 same-named types were renamed for TS only (#[ts(rename)]), e.g. PlotAsk/TradeRouteAsk, RouteView/BindsView, UpdateStatus/EngineerStatus, CoreStatus, EdsmVisit. The 16 Option fields with skip_serializing_if and no default got #[ts(optional)], so absent reads as absent. The api's own arg structs use optional_fields = nullable. Core: every json! reply in app.rs, calibration.rs, callouts.rs, learned.rs, biology.rs, setup.rs (and hub's mining run) is a typed struct (Status/CoreStatus, Applies, RouteReply, Outbox<EddnHead|EdsmHead>, Ores, CalloutList, Phrases, Biology, Started/Recorded/Calibrated/Checked, MicList, VoiceSettings, Tried, Poll, Ran, Saved, ShoppingSet, FindElite, FlightLog, Previewed). Core::commands registers all 54 through add_request with declared arg types: route_set/route_add parse RouteSetArgs/RouteAddArgs; the config edits keep Edit::from_request and declare setup::args::{Mic, Folders, Voice, Eddn, Edsm, Callout} (TS Set*Args; Option<Button> for set_ptt/set_kill_switch), with a test that each typed sample is accepted by from_request; voice_test_live declares LiveRequest = string | {command, from}. No-arg core commands ignore their args, as before. calibration::handle and status_json/mining_json stay as JSON wrappers for the tests that index them. app/src/lib/api/client.ts deleted. eslint ignores src/lib/api/types/ with a comment naming it as ts-rs output; commands.ts is linted. CONTRIBUTING names the update command (ASTROMECH_UPDATE_BINDINGS=1 cargo test -p astromech-core --lib bindings). ARCHITECTURE crate tree, dependency table and §5 types line updated. Final JsonValue allow-list (JSON_VALUES in crates/astromech-core/src/app.rs): route_plot reply only, because its view is route_set's reply, which astromech-api reads through Call::run and can't name (the route follower's types are the core's). set_search_filters replies () (null) and needs no entry. Accepted, as the spec says: two error paths. A failed call is the envelope's {ok:false,error}. Core commands still report their own failures inside the value (Answer).
Deviations: (1) The whole-set drift test is in astromech-core (cargo test -p astromech-core --lib bindings), because the core's handlers and reply types are in astromech-core and check-deps forbids astromech-api depending on astromech-core, dev-deps included. cargo test -p astromech-api checks its own part: its type files exactly, and its commands.ts lines. A hand edit to a core line or a stale file is caught only by the core test. Decision note on this issue. (2) A route_set/route_add arg of the wrong JSON type now gives 'bad waypoints: <serde error>' / 'bad stop: ...' inside the reply. A route_set with neither text nor waypoints says 'bad waypoints: expected waypoints or text' (was a serde message). A non-string name is now an error (was treated as empty). (3) status built its struct while holding a lock that status_line takes again. The first version deadlocked every test using status; it now takes each lock on its own.
Proof: core_replies_keep_their_json runs a 40-call script of the core's commands (status, callouts, set_* ok and error, prospector, shopping, voice, learned, find_elite, mic_list, calibrate_*, eddn/edsm, exobiology, voice_test(_live), install_update, route_*). Its fixture was recorded from the json! replies in commit 960792f2, before any core change, and the typed replies are identical. setup_and_calibration_reply_shapes_match_the_window_fake and the control-API tests pass unchanged. Edited tests only wrap helper results in to_value: learned, biology, callouts, the eddn_log test and voice_tester.rs. Drift seen failing: adding a field to ShellInfo fails both -p astromech-api ('differs: types/ShellInfo.ts') and the core test. A stale types/Stale.ts plus a hand edit of commands.ts fails the core test ('stale', 'differs: commands.ts'). A hand edit of binds_state's line fails the api test. All reverted, then green. Allow-list seen failing: a serde_json::Value field on save_diagnostics' reply ('a serde_json::Value (JsonValue) in [(save_diagnostics, reply)]'). A stale entry for set_search_filters ('no longer holds a JsonValue'). Both reverted. pnpm -s check covers the generated files: a planted type error in types/ShellInfo.ts is reported. Lint skips types/ and reports an unused const planted in commands.ts. Both reverted. The generated types were also checked by tsc against the fixture's 40 replies, which all assign, with the deliberate bad args and the masked pid as the only errors. The same check against the preview fixtures (scratch files, deleted) found only literal widening and fixture drift. cargo test --workspace: 1299 passed, 0 failed, 2 ignored. cargo clippy --workspace --all-targets -D warnings clean. cargo fmt --all --check clean. check-deps OK (32 crates). check-guide OK. cargo deny check: advisories, bans, licenses and sources ok. cd app && pnpm -s check (0 errors), pnpm -s lint (clean), pnpm -s test (73 files, 528 tests) pass.
Open items: (1) Fixture drift for astromech-96dv. The preview fixtures lack fields the real types always send: nearby.json query.by_population, plot.json ship.fsd, the plot waypoints' jumps, stations*.json filters.max_distance_ly/min_volume, engineering.json/ships.json ships' market_id, exobiology.json genera.analysed, trade.json cargo per_commodity.partial, voice.json live_from. (2) Windows-only code (tray.rs install_update call, the cfg(windows) mic_list branch) isn't compiled on this host; Windows CI. (3) commands.ts gives no-arg commands args: null; the window sends null, but a caller sending {} still works, since such args are ignored. (4) ts-rs output carries Rust doc comments as JSDoc, so a doc edit changes the checked-in files. (5) The window doesn't use the types yet (astromech-96dv). Review outstanding.
