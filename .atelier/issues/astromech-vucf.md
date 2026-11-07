---
created_at: "2026-11-07T04:51:23.220788707+00:00"
id: "astromech-vucf"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bp23"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "A scripted world for the real core: headless mode, clock, game folder, recorded network"
updated_at: "2026-11-07T11:16:41.492883634+00:00"
---

## Goal

The real core runs headless on Linux against a scripted world: a game folder written over time (journal, Status.json, NavRoute.json and the rest), recorded Spansh and Ardent responses, a clock the test controls, and fake input, audio and clipboard, with faults as inputs. The browser preview can use it through a dev bridge, and `tauri dev` is the documented loop at the PC.

## Done when

- [ ] `astromech --headless --world <script>` (name to settle) runs the core with no window on Linux; CI builds and starts it.
- [ ] The one-command-list test from astromech-ro7a runs against the headless core started with a world script, through its control API. It still calls every `window` command. The commands that test could not yet run for real are its `NOT_RUN_FOR_REAL` list (crates/astromech-core/src/app/every_window_command.rs), each with its missing seam in the table: `check_update`, `install_update`, `download_models` (the speech models), `autostart_on` and `autostart_off`, `open_config`, `save_diagnostics` (its Explorer reveal), `mic_check` and `calibrate_record` (the microphone), `voice_preview`, and `edsm_flight_log`. They now run against the world's recorded or faulted network and the fake opener, recorder and audio. A command that still has no seam is called with args or state that make it refuse before the effect, or is not called. The list left at the end names only those commands, each with its reason. The expected entries are the update install (it replaces the running app) and start with Windows (a Windows registry write). This completes success criterion one-command-list.
- [ ] The core reads time through a clock trait, and a scripted world can advance it.
- [ ] HTTP lookups go through a record-and-replay layer that can also answer with a 503, a timeout or garbage.
- [ ] A dev bridge (Vite middleware) passes the browser's `call()` to the headless core, so `pnpm dev` with a world script shows the real core's output; documented in CONTRIBUTING.md with `tauri dev`.
- [ ] The hand-written fixtures that the real core can now produce are dumped from it, and a test fails when they drift.

## Unknowns

- Which Windows-only parts (sending keys, the keyboard hook, WASAPI audio) still lack Linux fakes.
- The world script format: reuse astromech-mockgame and the replay tests' formats, or a new one.

## Notes

- 2026-11-07 [note] From finding astromech-nq3c (ro7a review): the command-list test's example args are not checked against their arg types, and no arg struct uses deny_unknown_fields, so for all-optional structs (set_folders, set_voice, set_edsm, set_callout's options, binds_set_override's joy) a misspelt field parses to None and answers ok as a no-op; Want::Ok reads only the envelope. When this increment runs the test for real, read back one effect per setter, or round-trip each example through its arg type.
