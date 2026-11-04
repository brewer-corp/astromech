---
created_at: "2026-11-04T22:42:52.491961752+00:00"
id: "astromech-zirr"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-zirr
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:03:48.402921464+00:00"
status: "done"
title: "Window config: ignore a config.path left by a core that is gone"
updated_at: "2026-11-04T23:03:48.402921464+00:00"
---

## Goal

A config.path left behind by a crashed, killed or older core doesn't make the window and the core it starts disagree on folders.

## Done when

- config.path records the publishing core's pid (or another liveness check), and the window ignores a note whose core is gone, with a test for a leftover note naming a deleted --config file (R1, astromech-69h9).
- docs/ARCHITECTURE.md no longer says the window can't see --config (R2, astromech-sb1n).

## Context

Deferred review findings R1 (astromech-69h9) and R2 (astromech-sb1n) on bug/astromech-f2k1: crates/astromech-store/src/paths.rs:108, docs/ARCHITECTURE.md:1774.

## Notes

- 2026-11-04 [decision] Liveness of a config.path note = the pid it records runs (Windows: OpenProcess + GetExitCodeProcess STILL_ACTIVE; elsewhere /proc/<pid>/exe) and that process's exe file name matches the recorded one (case-insensitive).
  Reason: Cheap and on the window's startup path without blocking; the exe-name match covers pid reuse by another program. The note lives in astromech-store, which can't see core.json's format or the core's instance mutex (astromech-core depends on store).
  Rejected: Connect to the control endpoint in core.json: a dead port takes about 2 s to fail on Windows (astromech-k8yk), and config_in_use runs on the window's setup path; core.json is also written after config.path, so a window opened in between would wrongly ignore a live note.
  Rejected: Compare core.json's pid: core.json is left behind by the same crash, so it would agree with a stale note.
  Rejected: Process start time: also fixes pid reuse, but needs more Windows API for little gain over the exe name.
- 2026-11-04 [decision] A config.path in the old format (just a path, no pid) is ignored, so the window uses the default file.
  Reason: Only a core from before this change writes it, and an update stops that core before the new one starts and overwrites the note; so an old-format note is always a leftover (the upgrade case in R1).
  Rejected: Trust old-format notes as before: keeps the R1 upgrade leftover bug.

## Close Reason

Implemented on bug/astromech-zirr @a1848909. Paths::publish_config now writes config.path as TOML {config, pid, exe} (exe = the publisher's exe file name). Paths::config_in_use follows a note only while that pid runs and its exe file name matches (case-insensitive): Windows via OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION) + GetExitCodeProcess == STILL_ACTIVE + QueryFullProcessImageNameW; elsewhere /proc/<pid>/exe (dev only). A note from a crashed or killed core, one whose pid now belongs to another program, or one in the old path-only format is ignored, so the window uses the default file, as the core it starts does. astromech-store gains a cfg(windows) windows-sys dependency (Win32_Foundation, Win32_System_Threading); Cargo.lock and app/src-tauri/Cargo.lock each gain that one line (the app lock checked with cargo metadata --locked --offline). No shell code changed. docs/ARCHITECTURE.md §18.5 Paths now describes config.path and its liveness rule instead of 'the window can't see that' (R2). Test: store paths::tests::a_note_left_by_a_core_that_is_gone_is_ignored covers a killed pid, a reused pid under another exe name and an old-format note, each naming a deleted --config file, plus a live note; it failed with the liveness filter removed. Evidence astromech-vwue. Decisions: two decision notes on this issue (liveness check; old-format notes ignored). Not compiled here: the cfg(windows) exe_name (no Windows target on this host); Windows CI builds it. Open items: a live core whose exe was renamed while running would have its note ignored (Windows exe name comes from the running image; unlikely). Independent review outstanding.
