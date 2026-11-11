---
schema: atelier.item
schema_version: 3
id: astromech-mqqa
title: The update helper is the core's child, so taskkill /T of astromech-core.exe can end it
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T19:49:09.913521117Z
---

## Goal

The update helper never ends itself. `spawn_apply` (crates/astromech-core/src/update.rs) starts `astromech-apply-<pid>.exe` from the running core; DETACHED and CREATE_NEW_PROCESS_GROUP do not change its parent, so it is in the core's process tree. `apply_cli` waits for the core with `wait_pid` for up to 45 s, then `stop_locked_exes` runs `taskkill /F /T /IM astromech-core.exe`. If the core is still running then (a slow shutdown, a hang), `/T` ends the helper with it: no copy, no relaunch, and astromech is left off with no `apply-error.txt`.

Consider dropping `/T` for `astromech-core.exe` only. Mind what the tree covers now that the one app owns the window: its WebView2 processes (`msedgewebview2.exe`, which lock nothing in the install folder but should not be left behind) and the voice worker (ended by name already). A kill without `/T` leaves WebView2 to notice its host is gone; check that it does.

## Done when

- [ ] A core that outlives `wait_pid` is ended without ending the helper, which then copies and relaunches; shown on Windows (a test core that ignores quit, or a unit test of the arguments plus a Windows CI step).
- [ ] No `msedgewebview2.exe` of the app is left after that kill (matched by command line, as scripts/webview_cost.py does).
- [ ] The `LOCKED_EXES` comment in update.rs says what the kill does.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-81t0
