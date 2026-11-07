---
created_at: "2026-11-07T04:51:23.220788707+00:00"
id: "astromech-vucf"
issue_type: "feature"
labels:
- "size-l"
workflow_branch:
  branch_base: main
  work_branch: astromech-vucf
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-1hh9"
  - kind: "issue"
    id: "astromech-7149"
  - kind: "issue"
    id: "astromech-d2nq"
  - kind: "issue"
    id: "astromech-gllh"
  - kind: "issue"
    id: "astromech-i1ev"
  - kind: "issue"
    id: "astromech-jkgv"
  - kind: "issue"
    id: "astromech-u8ge"
  - kind: "issue"
    id: "astromech-xk0n"
  - kind: "issue"
    id: "astromech-zgoo"
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
status: "todo"
title: "A scripted world for the real core: headless mode, clock, game folder, recorded network"
updated_at: "2026-11-07T12:59:45.770282485+00:00"
---

## Goal

The real core runs headless on Linux against a scripted world, in the format of decision astromech-e1a5. The world is:
- a game folder written over time (journal, Status.json, NavRoute.json and the rest);
- recorded Spansh, Ardent and EDSM responses;
- a clock the test controls;
- fake input, audio and clipboard, with faults as inputs.
The browser preview can use it through a dev bridge, and `tauri dev` is the documented loop at the PC.

## Done when

- [ ] The core is built by one assembly that takes its boundaries (`app::assemble` with `Boundaries`). The installed app's `run` and world mode both use it, and astromech-y6d6 builds on it.
- [ ] `astromech-core --world <dir>` runs the core with no window or tray, on a temp copy of the world, on Linux and Windows. It prints its control endpoint on stdout. A test in `cargo test --workspace` starts the built binary, and the user's folders stay untouched.
- [ ] The one-command-list test from astromech-ro7a runs against the headless core started with a world, through its control API, and calls every `window` command. These commands, which ro7a couldn't run for real, now run against the world's recorded or faulted network and its fake opener, Start with Windows, recorder and speaker:
  - `check_update`, `download_models`;
  - `autostart_on`, `autostart_off`;
  - `open_config`, `save_diagnostics`;
  - `mic_check`, `calibrate_record`;
  - `voice_preview`, `edsm_flight_log`.
  The list left at the end is `install_update` alone (it replaces the running app), called with no update staged so it refuses before its effect. This completes success criterion one-command-list.
- [ ] The core reads the time and waits through one clock trait. In world mode the script advances a manual clock, whose `advance` returns only when the core's timer threads are parked again. Clippy rejects a new direct read or sleep in production code.
- [ ] In world mode every HTTP request is answered from the world's recording, and a request it doesn't hold is refused and reported. Lookups can be answered with a 503, a timeout, a connection failure or garbage. A record mode captures real answers, with keys and commander names redacted.
- [ ] In world mode, on every OS:
  - push-to-talk, the kill switch and the paste button run from raw-input samples on the world clock;
  - the mic from recorded WAVs;
  - speech-to-text from the script;
  - key presses go to the world's reactions;
  - speech out is recorded.
  A gear-down-by-voice world, a kill-switch world (kill, a denied command, re-arm by holding) and a paste world pass. On Windows CI the in-game push-to-talk path is exercised through the real input bus.
- [ ] A dev bridge (Vite middleware) passes the browser's `call()` to the headless core, so `ASTROMECH_WORLD=<dir> pnpm dev` shows the real core's output. It is documented in CONTRIBUTING.md with `tauri dev`.
- [ ] The preview fixtures the real core can produce are dumped from it, and a test fails when they drift.

## Background

The Unknowns are resolved in the children. Which Windows-only parts lacked Linux fakes is answered in the assembly, world-mode and devices children; the world script format is decision astromech-e1a5.

## Notes

- 2026-11-07 [note] From finding astromech-nq3c (ro7a review): the command-list test's example args are not checked against their arg types, and no arg struct uses deny_unknown_fields, so for all-optional structs (set_folders, set_voice, set_edsm, set_callout's options, binds_set_override's joy) a misspelt field parses to None and answers ok as a no-op; Want::Ok reads only the envelope. When this increment runs the test for real, read back one effect per setter, or round-trip each example through its arg type.
- 2026-11-07 [note] From finding astromech-cq1w (7149 review): astromech-7149's Windows-only changes (start_voice's data parameter, tray.rs core.opener.reveal, RunKey's private get/set, WindowsNotifier per session) have only been read, not compiled. Before vucf merges, its PR's Windows CI (test and clippy on windows-latest) must pass, then tick 7149's Done when 3.
