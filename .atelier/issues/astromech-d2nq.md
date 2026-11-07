---
created_at: "2026-11-07T11:57:01.794229152+00:00"
id: "astromech-d2nq"
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
    id: "astromech-7149"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-e1a5"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Headless world mode: astromech-core --world, fakes for the assembly's boundaries, observations"
updated_at: "2026-11-07T11:57:01.794229152+00:00"
---

## Goal

`astromech-core --world <dir>` runs the real core with no window or tray, on a temp copy of a world (decision: world format), with fakes for the boundaries the assembly takes. It prints how to reach it on stdout. Tests, the dev bridge and yis2's driver start this binary and talk to it over the control API.

## Done when

- [ ] `--world <dir>` copies the world to a fresh temp directory and builds `Boundaries` there:
  - `Paths` with app, log and exe folders under the temp dir;
  - a config naming every folder (data, models, journal, Bindings, game) under it;
  - fakes for the process list, opener, clipboard, notifier, Start with Windows, and secrets in memory seeded from the world.
  It then calls `assemble` and `start`, the same start function `run` uses (game watch, idle reaper, history, serving). It doesn't call what only `run` adds: the instance lock, `autostart::apply_default`, the update thread, the window and the tray.
- [ ] Once the control API is up, the binary prints one JSON line on stdout: `{"control":{"port":…,"token":…},"app_dir":…}`. This is the only endpoint source the tests and the bridge use. It runs until `quit`.
- [ ] The fakes record what reaches them, in order, as observations. `world_observed {since}` and `world_wait {match}` are registered only in world mode, as `local` commands (decision astromech-unh9: refused with the window's origin). `world_wait` is woken by the observation itself, not by polling, and has a hang guard that fails the call.
- [ ] A test helper in crates/astromech-core/tests/support/ spawns `CARGO_BIN_EXE_astromech-core --world <dir>`, reads the stdout line, and calls commands with `astromech_api::control`.
- [ ] A test over `worlds/minimal` runs with HOME, XDG_DATA_HOME, LOCALAPPDATA, APPDATA and USERPROFILE all pointing at one empty folder. It checks that:
  - `status` answers;
  - `open_config` appears in `world_observed` with the world's config path;
  - `autostart_on` appears as Start with Windows set;
  - after `quit` the process exits 0, and the empty folder is still empty.
  It runs in `cargo test --workspace` on Linux and on Windows CI.
- [ ] The no-spawn proof for world mode: every opener, Start with Windows and notifier effect appears in `world_observed`, and the real ones are not reachable from `Boundaries` in world mode. This is checked in review against the world's `Boundaries` construction. The speaker and platform are added by the devices child.

## Scope

The flag, the world directory loader (seed folders and config; steps come with the script child), the fakes named above, the two observation commands and the test helper. Don't change `run`.

## Context

The assembly API comes from the assembly child. On Linux `ui_loop` already runs without a tray (app.rs:1685-1698). The journal folder falls back to `%USERPROFILE%` (astromech-store/src/paths.rs:150-160), hence USERPROFILE in the empty-folder test. The flag moves to the one executable in astromech-y6d6.
