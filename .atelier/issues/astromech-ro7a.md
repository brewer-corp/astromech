---
created_at: "2026-11-07T06:33:38.679175382+00:00"
id: "astromech-ro7a"
issue_type: "task"
labels:
- "size-s"
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
closed_at: "2026-11-07T11:09:05.627776039+00:00"
status: "done"
title: "Test: every window command answers over a running core's control API"
updated_at: "2026-11-07T11:16:41.922479558+00:00"
---

## Goal

A test lists the window's commands and calls each one through a running core's control API, so a command the window uses can't be missing from the command layer (success criterion one-command-list).

## Done when

- [x] An integration test in crates/astromech-core builds a core over a temp app, data and journal folder, serves its registry on the real control API (TCP, token, `core.json`), and calls every `window`-exposure command with example args from a table in the test. The test fails when a `window` command has no entry in the table.
- [x] Each reply is `ok`, or an error that is the handler's own (for example "journal folder not found"). It is never "unknown command", bad args, "not allowed" or an internal error (a caught panic).
- [x] No network and no external program: the remote-data client runs over a transport that refuses, and the opener is the test's. Commands whose effect has no seam yet (speech-model and neutron downloads, the update check and install, start with Windows, `open_config`, `save_diagnostics`' Explorer reveal, the microphone and voice preview, and EDSM's flight log) are called with args or state that make them refuse before the effect, or are listed by name with the missing seam. The scripted world in astromech-vucf replaces that list with real calls.
- [x] The test runs in `cargo test --workspace` on Linux, with no sleeps and no wall-clock waits.

## Scope

A test, plus the example args table. Do not build a headless binary or a world script here: that is astromech-vucf.

## Context

"The window's commands" is the registry's `window` entries. astromech-96dv makes `call()` accept only those names, so the window can't call a command outside them. Together the two children prove one-command-list: everything the window can call is in this list, and everything in this list answers over the control API. The headless half of that criterion comes with astromech-vucf (its Done when runs this test against the headless core).

## Notes

- 2026-11-07 [note] Decision (implementer): the test is a module of astromech-core's lib tests (crates/astromech-core/src/app/every_window_command.rs), not a file under crates/astromech-core/tests/. Reason: a Core with the test's seams (in-memory secrets, refusing remote client, test opener and devices, no clipboard) is built only by app.rs's test helper over the private api_state; a public constructor belongs with astromech-vucf's headless mode, which will start the core this test then talks to. The test still goes through the real control API (serve, core.json, token, TCP, control::call_window). Also (implementer): the neutron check and download run for real against a local server answering 503 (State::with_neutron_url made public), so they are off the not-run-for-real list; astromech-vucf's Done when line now names the exact list. For the plan owner to accept or change.
- 2026-11-07 [decision] Accepted: the test is a module in astromech-core's lib tests, not tests/, because the seams it needs go through the crate's private api_state; it still goes over the real control API
  Reason: Reviewer astromech-ro7a pass agreed; vucf's headless mode brings the public constructor

## Report

Implementer report (commit 5ffa4f1d on feature/astromech-bp23). Outcome: crates/astromech-core/src/app/every_window_command.rs (a lib test module, see the decision note) builds a core over a temp app, data, journal, Bindings and game folder (app.rs's test helper, now core_with_state), serves core.commands() with control::serve (TCP on 127.0.0.1, token, core.json, checked read back) and calls each window command with control::call_window, as the shell's proxy sends it (origin app_window, a string arg as text). The table has 102 entries: example args and Want::Ok, Want::Fails(<the handler's own error text>) or Want::NotCalled(<missing seam>). Before calling, the test fails for a window command with no entry, an entry that isn't a window command, or one listed twice. A reply fails when its envelope error or in-reply {ok:false,error} is unknown command, bad args for, not allowed, '<cmd> failed with an internal error', bad request or bad token, or doesn't match its Want. Seams: the remote-data client refuses (Cannot reach Spansh/Ardent Insight); the neutron check and download hit a local server answering 503; open_external opens with the test's opener, which must get exactly the one GitHub link; binds read an empty device list; the clipboard is the helper's no-op route desk; secrets in memory; voice tts off. State::with_opener, with_devices and with_neutron_url are public (were cfg(test)). Not run for real (NOT_RUN_FOR_REAL, asserted equal to the table's): check_update (called while a check is marked running, so its thread returns first), install_update (no update staged), download_models (a download marked running), save_diagnostics (a file where its folder goes, so the save fails before the reveal), edsm_flight_log (no key), voice_preview (speech output off), calibrate_record (before a calibration starts); not called: autostart_on, autostart_off (real HKCU Run key on Windows CI), open_config (Notepad/xdg-open), mic_check (records on Windows). 73 of the 98 calls answer ok, 25 with the handler's own error.
Deviations: (1) A lib test module, not crates/astromech-core/tests/ (decision note). (2) The neutron check and download run for real against the local 503 server, so they are off the list the spec named; astromech-vucf's Done when now lists the exact commands and points at NOT_RUN_FOR_REAL. (3) The core tests' offline() client has Limiter::none(): with the default limiter the refused lookups waited about 7 s for request spacing.
Proof: cargo test -p astromech-core --lib every_window passes, 0.15-0.22 s over 4 runs. Mutations, each reverted: shell_info panics -> 'shell_info: wanted Ok, got Err(shell_info failed with an internal error: mutant)'; PrefixArg's field renamed -> 'galaxy_systems: wanted Fails(Cannot reach), got Err(bad args for galaxy_systems: missing field start)'; route_copy's entry removed -> 'window commands with no entry in the table: [route_copy]'; a new window command shell_info2 registered -> 'window commands with no entry in the table: [shell_info2]'. strace -f of the test binary: 98 connects to the control port and 2 to the 503 server, all 127.0.0.1, no other connect, one execve (the test binary itself); every file created, renamed or removed is under the temp dir. cargo test --workspace --locked: 1302 passed, 0 failed, the test included. cargo clippy --workspace --all-targets --locked -D warnings clean; cargo fmt --all --check clean; check-deps OK (32 crates); cargo deny check ok; check-guide 0 problems. No app/ change, so pnpm not run. Docs: CONTRIBUTING (a new window command needs a table entry) and ARCHITECTURE §18.2.
Open items: (1) Windows CI runs this test too and is unseen here: there mic_list lists the real recording devices and status reads the Run key (both read only), and save_diagnostics' error is Windows' 'os error 183' (the table matches 'os error'). (2) binds_restore is called with a path that isn't a backup, so it answers its own 'not one of astromech's backups' rather than restoring. (3) voice_test_live and exobiology answer 'Elite isn't running' (no session); their session paths are vucf's. (4) check_update's background thread is not observed; the busy check that stops it is read from code. Review outstanding.
