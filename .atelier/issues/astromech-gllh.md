---
created_at: "2026-11-07T11:57:01.793127219+00:00"
id: "astromech-gllh"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T22:48:53.957593537+00:00"
status: "done"
title: "Every window command runs for real against the headless world core"
updated_at: "2026-11-07T22:48:53.957593537+00:00"
---

## Goal

The one-command-list test runs against the headless core started with a world, and calls every `window` command for real. That completes success criterion one-command-list. Today it is a lib test over a hand-built core (crates/astromech-core/src/app/every_window_command.rs), with eleven commands in `NOT_RUN_FOR_REAL` (:77-89).

## Done when

- [x] The test lives in crates/astromech-core/tests/ and spawns `CARGO_BIN_EXE_astromech-core --world worlds/every-command` (world-mode child's helper). It calls each `window` command with `control::call_window` over the real control API, as the shell sends it. It still fails for a `window` command with no table entry, an entry that isn't one, or a duplicate.
- [x] These run against the world, and the test checks each effect in `world_observed` or in the reply:
  - `check_update` against the recorded feed;
  - `download_models` against a recorded 503: the error is reported and the models are still missing;
  - `autostart_on` and `autostart_off`, which the world's Start with Windows records on every OS;
  - `open_config` and `save_diagnostics`' reveal, which the world's opener records;
  - `mic_check` and `calibrate_record`, which get the world's recorded clip;
  - `voice_preview`, whose preview line is said;
  - `edsm_flight_log`, answered from a recorded, redacted EDSM logs body with the world's key;
  - `voice_test_live` and `exobiology`, which answer from a running session (ro7a open item 3).
- [x] `NOT_RUN_FOR_REAL` is `["install_update"]`, with its reason (it replaces the running app). `install_update` is called with no update staged and refuses before its effect.
- [x] Finding astromech-nq3c is closed by a round-trip check. Each example args value is parsed into its command's arg type and serialised back, and every field in the example must survive with its value. Defaults the type adds don't matter. A misspelt field drops out and fails the check. Mutation: rename a field in `set_folders`' example and the test fails naming it. Production parsing doesn't change. Adding a `Serialize` derive where an arg type lacks one is allowed.
- [x] No-spawn proof: every opener, Start with Windows, speaker and key effect appears in `world_observed`, and world mode's `Boundaries` reach no real one (world-mode and devices children). No request is unrecorded.
- [x] Runs in `cargo test --workspace` on Linux and Windows CI with no sleeps, no wall-clock reads and no retries. The lib-test module and ro7a's `unavailable_server` and `offline()` client are removed, and CONTRIBUTING.md's line about the table names the world.

## Scope

The test, its world and the round-trip check. Don't change command behaviour. If a command can't run for real without a new seam, stop and report it: don't add it to the list.

## Context

ro7a's report (atelier issue show astromech-ro7a --full) gives the current table, seams and open items. Decision astromech-unh9 gives exposure. The world test spawns the binary because the core's process globals (bus, EDDN_SHARING, TESTER, VOICE_PAUSED, VOICE_TURN) would make cores in one test process interfere (decision: world format).

## Notes

- 2026-11-07 [note] From finding astromech-ur4v (xk0n review): Spansh's 180 s polling timeout reads the manual clock (world.rs:504 NO_WAIT, spansh.rs:1050-1061), so a recording that leaves a job queued makes a lookup spin and never return. Cap the polls or add a real-time guard that only fails the call, here where worlds first plot routes.
- 2026-11-07 [note] From finding astromech-gnnh (1hh9 review): CONTRIBUTING.md:256-258 and worlds/gear-down/astromech.toml should give the exact clip recipe (render-voices bm_george, the text, then ffmpeg to 16 kHz mono), since render-voices writes 24 kHz and the world mic needs 16 kHz.
- 2026-11-07 [decision] World mode observes the tray: after each refresh the core asks for, it records tray {line, check} (status line and update item), read only after every earlier observation (Observed::record_at), and the test awaits check_update's and download_models' background threads by the first tray after their last request that shows them done
  Reason: Both commands answer at once and finish on an unregistered thread; the only alternatives were polling status (a retry) or a command-specific hook. The tray is the boundary the installed app shows that state on, and world mode had none.
- 2026-11-07 [decision] The round trip is Registry::round_trip_args, with each command's args type kept as a fn in Types; add_typed and add_request (and app.rs's add) now require A: Serialize + DeserializeOwned, so 20-odd arg types gained a Serialize derive (and LiveRequest a Deserialize). The world core serves it as the local world_args {cmd, args}, and world_commands lists the registry's commands by exposure, both through a new Call::registry()
  Reason: The test runs in another process than the registry, and the arg type is only known where the command is registered; a bound makes every new command round-trippable instead of opt-in
- 2026-11-07 [decision] The lookups in worlds/every-command are answered from existing recorded fixtures (crates/astromech-data, -galaxy and -market tests), matched by URL and a part of the body sent, not from a fresh record-mode capture; examples moved to fit them (galaxy_systems Shin, trade_anywhere painite, set_mic the world microphone, voice_test_live lights.on)
  Reason: A record-mode capture of the same calls was 14 MB (Spansh station and system searches, Ardent's Sol market at 7.5 MB); the fixtures are real recordings of the same endpoints and keep the world small
- 2026-11-07 [decision] edsm_flight_log is answered with EDSM's real answer to the world's placeholder commander and key (msgnum 203, Commander name/API Key not found), recorded with --record-net; the test wants that handler error
  Reason: A real flight log needs a real key and holds dated travel history (finding astromech-9rru); the 203 still runs the request, redaction and the answer's parsing for real
- 2026-11-07 [decision] The core lib tests' offline() client is gone: their boundaries() lookups replay an empty recording (astromech_data::replay), the same refusal as a world with no net.json, and core_with overrides remote
  Reason: Done when 6 retires it; one fake network instead of two
- 2026-11-07 [decision] Spansh job polling with Polling::every zero stops after spansh::NO_WAIT_POLLS (100) polls with Error::Timeout (finding astromech-ur4v)
  Reason: A poll count bounds it without the clock moving; with a nonzero wait the timeout already bounds it, so the installed app is unchanged
- 2026-11-07 [note] Finding astromech-n5cr: setters now read their effect back (.reads), lookups and reads check their reply's content from the recording (.has), and the test fails for a net.json exchange no request used (http observations carry the answering exchange's index). Without a read-back: ack_crash (no crash in the world), learned_forget (nothing is learned without a spoken correction), set_eddn (switching it on would upload to EDDN, which has no recorded answer; off is the world's start), shopping_set (read only by the session's callouts; its reply's count is checked). set_kill_switch now sets a button and set_folders a dotted spelling of the world's own folder, so each edit reads back as made; set_edsm sets commander Jameson; binds_set_active the copy; binds_set_override a Joy name.
- 2026-11-07 [note] Finding astromech-6iev: set_voice's example (bf_emma, 1.0, cockpit) was the default, so its read-back couldn't fail; it now sets bm_george, 1.25 (exact in f32) and harsh. The first status entry pins the world's start for each setter's read-back (ptt, kill switch and mic unset, not onboarded, voice not paused, autostart off, the default voice, EDSM commander Test); the other setters were checked against it: callouts, prospector thresholds, search filters, binds and the route already read their start before the change, and set_folders' dotted path can't be the start.

## Report

Implementer report (branch astromech-vucf). Outcome: crates/astromech-core/tests/every_window_command.rs starts `astromech-core --world worlds/every-command` with WorldCore, steps Elite to running, and calls all 102 `window` commands with `call_window` (kill_switch twice: kill, then re-arm), each with a Want (ok or the handler's own error) and its effect: observations awaited in order with `world_wait` from the call on, and/or a reply check. The table is checked against the world core's registry (`world_commands`): no entry, an entry that isn't a window command, or a duplicate fails. Each example's args go through `world_args` (Registry::round_trip_args) and every field must come back with its value (numbers by value). NOT_RUN_FOR_REAL is ["install_update"], called before check_update with no update staged ("no update ready"). Effects: check_update fetches the recorded real signed latest.json and .sig, verifies against the built-in key and requests the 1.0.0-alpha.4 zip (recorded 503), then status shows latest 1.0.0-alpha.4, status error, a 503 error; download_models requests the STT model (503), the failure is in the log and models_missing stays true; autostart on/off, open_config's edit of the config, save_diagnostics' reveal of the zip it replies with, voice_preview's line said, mic_list naming the world microphone, calibrate_record and mic_check giving a speech level from gear-down's clip, edsm_flight_log's recorded EDSM 203 ("EDSM 203: Commander name/API Key not found", request redacted), voice_test_live lights.on pressing Key_Insert (the world's reaction turns the lights on so it verifies), exobiology from the session, route_copy's clipboard, open_external's open; every lookup answers ok from recorded fixtures; open_window (local) is observed as `window`; no http observation is unrecorded. World additions: `world_commands`, `world_args`, and the tray fake (`tray {line, check}`, read after every earlier observation) that the two background commands are awaited on. ur4v: Spansh polling with no wait stops after NO_WAIT_POLLS. Removed: the lib-test module, unavailable_server, offline() (test boundaries replay an empty recording). Docs: CONTRIBUTING's table line names the world; gnnh's clip recipe in CONTRIBUTING and worlds/gear-down/astromech.toml; world.rs docs; ARCHITECTURE §6 world mode and §18.2. 90pi was already done (Assembly lists the app window). .gitattributes keeps worlds/*/latest.json and .sig byte for byte (-text): the real feed has CRLF line ends and its signature is over those bytes, so the repo-wide eol=lf would break it.
Deviations: (1) EDSM's flight log answer is the real 203 refusal, not a log body (decision note; 9rru). (2) Lookups answer from existing fixtures, not a new capture (14 MB; decision note); several examples changed to fit them. (3) The tray observation is new world surface (decision note). (4) The registry's args bound now needs Serialize + DeserializeOwned (decision note); TypeScript bindings are unchanged (bindings test passes).
Surprises: a live record-mode capture showed Ardent's real `/v2/system/name/Sol/commodities` answer (15,638 rows, 7.5 MB) now has rows with null commodityName, marketId and prices, and trade_route fails on it with "Ardent Insight sent an answer astromech does not understand" (the fixture predates that). Not fixed here (Scope); worth an issue. 2460: the session's known-biology Spansh search is one of the world's recorded requests (spansh-bodies-biology.json).
Proof: `cargo test -p astromech-core --test every_window_command` passes in 0.36-0.49 s; 30 runs in a row 30/30 (11.4 s total), and 30 more 6 at a time 30/30. Mutations, each reverted: shell_info panics -> "shell_info: wanted Ok, got Err(shell_info failed with an internal error: mutant)"; a new window command shell_info2 -> "window commands with no entry in the table: [shell_info2]"; world_observed added to the table -> "entries that aren't window commands: [world_observed]"; status listed twice -> "commands listed twice: [status]"; set_folders' journal_dir spelt journal_folder -> "set_folders.journal_folder is lost"; voice_preview saying nothing -> "voice_preview: world_wait: nothing matching {kind: said, text: Landing gear deployed.} ..."; one hex digit of latest.json.sig changed -> check_update never requests the zip. ur4v: a_job_polled_with_no_wait_gives_up_without_the_clock_moving passes; without the cap it spins until killed (timeout 20 s). strace -f -e execve,connect of the test: two execve (the test binary, astromech-core --world), 283 connects, all to 127.0.0.1. Workspace: cargo test --workspace --locked 1370 passed, 0 failed; cargo clippy --workspace --all-targets --locked -D warnings clean; cargo fmt --all --check clean; check-deps OK (32 crates); cargo deny check ok; check-guide 0 problems. No app/ change, so pnpm not run.
Open items: (1) Done when 6's Windows CI half is unseen here (the test, world_mode's tray filter and the new derives on windows-latest), so that line is unticked; the world's fakes are the same on every OS. (2) The tray observation and the arg bounds are new surface for review. (3) Ardent null rows above. Review outstanding.
