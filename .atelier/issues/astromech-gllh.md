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
  relates:
  - kind: "issue"
    id: "astromech-1hh9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xk0n"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Every window command runs for real against the headless world core"
updated_at: "2026-11-07T11:57:01.793127219+00:00"
---

## Goal

The one-command-list test runs against the headless core started with a world, and calls every `window` command for real. That completes success criterion one-command-list. Today it is a lib test over a hand-built core (crates/astromech-core/src/app/every_window_command.rs), with eleven commands in `NOT_RUN_FOR_REAL` (:77-89).

## Done when

- [ ] The test lives in crates/astromech-core/tests/ and spawns `CARGO_BIN_EXE_astromech-core --world worlds/every-command` (world-mode child's helper). It calls each `window` command with `control::call_window` over the real control API, as the shell sends it. It still fails for a `window` command with no table entry, an entry that isn't one, or a duplicate.
- [ ] These run against the world, and the test checks each effect in `world_observed` or in the reply:
  - `check_update` against the recorded feed;
  - `download_models` against a recorded 503: the error is reported and the models are still missing;
  - `autostart_on` and `autostart_off`, which the world's Start with Windows records on every OS;
  - `open_config` and `save_diagnostics`' reveal, which the world's opener records;
  - `mic_check` and `calibrate_record`, which get the world's recorded clip;
  - `voice_preview`, whose preview line is said;
  - `edsm_flight_log`, answered from a recorded, redacted EDSM logs body with the world's key;
  - `voice_test_live` and `exobiology`, which answer from a running session (ro7a open item 3).
- [ ] `NOT_RUN_FOR_REAL` is `["install_update"]`, with its reason (it replaces the running app). `install_update` is called with no update staged and refuses before its effect.
- [ ] Finding astromech-nq3c is closed by a round-trip check. Each example args value is parsed into its command's arg type and serialised back, and every field in the example must survive with its value. Defaults the type adds don't matter. A misspelt field drops out and fails the check. Mutation: rename a field in `set_folders`' example and the test fails naming it. Production parsing doesn't change. Adding a `Serialize` derive where an arg type lacks one is allowed.
- [ ] No-spawn proof: every opener, Start with Windows, speaker and key effect appears in `world_observed`, and world mode's `Boundaries` reach no real one (world-mode and devices children). No request is unrecorded.
- [ ] Runs in `cargo test --workspace` on Linux and Windows CI with no sleeps, no wall-clock reads and no retries. The lib-test module and ro7a's `unavailable_server` and `offline()` client are removed, and CONTRIBUTING.md's line about the table names the world.

## Scope

The test, its world and the round-trip check. Don't change command behaviour. If a command can't run for real without a new seam, stop and report it: don't add it to the list.

## Context

ro7a's report (atelier issue show astromech-ro7a --full) gives the current table, seams and open items. Decision astromech-unh9 gives exposure. The world test spawns the binary because the core's process globals (bus, EDDN_SHARING, TESTER, VOICE_PAUSED, VOICE_TURN) would make cores in one test process interfere (decision: world format).
