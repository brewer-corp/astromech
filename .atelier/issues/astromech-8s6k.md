---
created_at: "2026-11-07T06:33:38.680185002+00:00"
id: "astromech-8s6k"
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
    id: "astromech-unh9"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Command registry in astromech-api, served by the control API"
updated_at: "2026-11-07T06:35:26.634640276+00:00"
---

## Goal

One command registry in a new crate, `astromech-api`, that every way in dispatches through. The core's control API serves the registry, and the core's own commands are registered in it. This is the expand step: later children move the shell's commands in group by group, and nothing the window does changes yet.

## Done when

- [ ] `crates/astromech-api` holds the registry (name, exposure `window` or `local` as in the exposure decision, handler) and the control API, moved from crates/astromech-core/src/control.rs so the server and the client are one module the core, the shell and tests share. `cargo test -p astromech-api` covers:
  - an unknown name and bad args each give an error naming the command;
  - a `local` command with `origin: "app_window"` gives "not allowed";
  - a handler that panics while holding a lock it shares with other calls gives `{"ok": false, "error": …}`, is logged, and the next call to it succeeds (`catch_unwind` plus poison recovery, not `lock().map_err(|_| "busy")`);
  - progress lines arrive before the reply;
  - a handler that answers after a client's timeout still reaches a client with no timeout. The handler waits on a gate. A client with a short timeout fails first, and that failure is the event that releases the gate, so the test has no sleep. A second client with no timeout gets the reply.
- [ ] The client's read timeout is per call. The shell's forwarders and proxy use none, so the failure signal is a dropped connection (the core died) or the handler's own error. Today `call_with` gives up at 15 s (app/src-tauri/src/window.rs:45-46) and `control::call` at 10 s (control.rs:121), while a plot or trade route polls Spansh for up to 180 s (crates/astromech-data/src/spansh.rs:963), one Spansh request may take 30 s (transport.rs:89), and the neutron download streams the whole file. Short status-style callers (instance check, `--quit`, second start) keep a bounded timeout. The connect timeout stays at 2 s.
- [ ] The wire: a request is `{token, cmd, args?, text?, route?, origin?}`. The reply is zero or more `{"progress": V}` lines, then `{"ok": true, "value": V}` or `{"ok": false, "error": "…"}`. A core command's value is its reply today. This changes every reply on the wire, so every client moves in this child:
  - the shell's `core_call` and `call_with` (window.rs:35-56, 1205-1270): they unwrap `value`, send `origin: "app_window"`, and the `ALLOWED` list is deleted, so the window sees no change;
  - the single-instance check off Windows (crates/astromech-core/src/instance.rs:41): the core answered when the envelope's `ok` is true;
  - `--quit` (main.rs:131) and the second start's `open_window` (app.rs:1226);
  - scripts/onboarding_smoke.py `core_call` and `wait_core`, which read `value`;
  - the Release workflow's PowerShell smoke step (.github/workflows/release.yml:191-200), which checks `$status.ok` (the envelope) and prints and checks `$status.value.status_line`.

  A search for `core.json`, `control::call` and `"cmd"` in scripts/, .github/ and crates/ finds no other client. List what was searched in the Close Reason.
- [ ] `Core::handle`'s commands (crates/astromech-core/src/app.rs:714) are registered entries. The existing control-API tests in app.rs (`route_commands_over_the_control_api`, `add_stop_and_set_destination_over_the_control_api` and the others) pass unchanged in what they assert.
- [ ] scripts/check-deps.py forbids `astromech-api` from depending on `astromech-input`, `astromech-policy` and `astromech-core`. ARCHITECTURE.md §4 lists the crate and the rule.
- [ ] CONTRIBUTING.md's Rust checks pass locally. Windows CI's shell build and clippy (`-p astromech-shell --features window`) and the Release workflow's install smoke and onboarding smoke pass on the PR. This host has no webkit2gtk, so the shell's window feature builds only there.

## Scope

New crate; crates/astromech-core (control.rs goes, app.rs registers, instance.rs, main.rs); app/src-tauri/src/window.rs `core_call` and `call_with` only; scripts/onboarding_smoke.py; .github/workflows/release.yml's smoke step; scripts/check-deps.py; ARCHITECTURE §4. No shell command moves yet. The handlers run on the control API's per-connection thread (control.rs:87-92), which is the worker thread. Tauri's `spawn_blocking` is the worker thread when the window calls in process (astromech-y6d6).

## Context

Exposure decision (linked). bp23's Background holds the design notes. astromech-proto's `Origin` (crates/astromech-proto/src/lib.rs:15-23) is the origin type. astromech-proto must stay dependency-free, so request and reply types that name domain types live in astromech-api.

## Unknowns

- Whether `text` and `route` fold into `args` now. Keep both on the wire unless astromech-se44 needs otherwise.
