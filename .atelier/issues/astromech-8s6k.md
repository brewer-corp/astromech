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
closed_at: "2026-11-07T06:48:00.142354479+00:00"
status: "done"
title: "Command registry in astromech-api, served by the control API"
updated_at: "2026-11-07T06:48:00.142354479+00:00"
---

## Goal

One command registry in a new crate, `astromech-api`, that every way in dispatches through. The core's control API serves the registry, and the core's own commands are registered in it. This is the expand step: later children move the shell's commands in group by group, and nothing the window does changes yet.

## Done when

- [x] `crates/astromech-api` holds the registry (name, exposure `window` or `local` as in the exposure decision, handler) and the control API, moved from crates/astromech-core/src/control.rs so the server and the client are one module the core, the shell and tests share. `cargo test -p astromech-api` covers:
  - an unknown name and bad args each give an error naming the command;
  - a `local` command with `origin: "app_window"` gives "not allowed";
  - a handler that panics while holding a lock it shares with other calls gives `{"ok": false, "error": …}`, is logged, and the next call to it succeeds (`catch_unwind` plus poison recovery, not `lock().map_err(|_| "busy")`);
  - progress lines arrive before the reply;
  - a handler that answers after a client's timeout still reaches a client with no timeout. The handler waits on a gate. A client with a short timeout fails first, and that failure is the event that releases the gate, so the test has no sleep. A second client with no timeout gets the reply.
- [x] The client's read timeout is per call. The shell's forwarders and proxy use none, so the failure signal is a dropped connection (the core died) or the handler's own error. Today `call_with` gives up at 15 s (app/src-tauri/src/window.rs:45-46) and `control::call` at 10 s (control.rs:121), while a plot or trade route polls Spansh for up to 180 s (crates/astromech-data/src/spansh.rs:963), one Spansh request may take 30 s (transport.rs:89), and the neutron download streams the whole file. Short status-style callers (instance check, `--quit`, second start) keep a bounded timeout. The connect timeout stays at 2 s.
- [x] The wire: a request is `{token, cmd, args?, text?, route?, origin?}`. The reply is zero or more `{"progress": V}` lines, then `{"ok": true, "value": V}` or `{"ok": false, "error": "…"}`. A core command's value is its reply today. This changes every reply on the wire, so every client moves in this child:
  - the shell's `core_call` and `call_with` (window.rs:35-56, 1205-1270): they unwrap `value`, send `origin: "app_window"`, and the `ALLOWED` list is deleted, so the window sees no change;
  - the single-instance check off Windows (crates/astromech-core/src/instance.rs:41): the core answered when the envelope's `ok` is true;
  - `--quit` (main.rs:131) and the second start's `open_window` (app.rs:1226);
  - scripts/onboarding_smoke.py `core_call` and `wait_core`, which read `value`;
  - the Release workflow's PowerShell smoke step (.github/workflows/release.yml:191-200), which checks `$status.ok` (the envelope) and prints and checks `$status.value.status_line`.

  A search for `core.json`, `control::call` and `"cmd"` in scripts/, .github/ and crates/ finds no other client. List what was searched in the Close Reason.
- [x] `Core::handle`'s commands (crates/astromech-core/src/app.rs:714) are registered entries. The existing control-API tests in app.rs (`route_commands_over_the_control_api`, `add_stop_and_set_destination_over_the_control_api` and the others) pass unchanged in what they assert.
- [x] scripts/check-deps.py forbids `astromech-api` from depending on `astromech-input`, `astromech-policy` and `astromech-core`. ARCHITECTURE.md §4 lists the crate and the rule.
- [x] CONTRIBUTING.md's Rust checks pass locally. Windows CI's shell build and clippy (`-p astromech-shell --features window`) and the Release workflow's install smoke and onboarding smoke pass on the PR. This host has no webkit2gtk, so the shell's window feature builds only there.

## Scope

New crate; crates/astromech-core (control.rs goes, app.rs registers, instance.rs, main.rs); app/src-tauri/src/window.rs `core_call` and `call_with` only; scripts/onboarding_smoke.py; .github/workflows/release.yml's smoke step; scripts/check-deps.py; ARCHITECTURE §4. No shell command moves yet. The handlers run on the control API's per-connection thread (control.rs:87-92), which is the worker thread. Tauri's `spawn_blocking` is the worker thread when the window calls in process (astromech-y6d6).

## Context

Exposure decision (linked). bp23's Background holds the design notes. astromech-proto's `Origin` (crates/astromech-proto/src/lib.rs:15-23) is the origin type. astromech-proto must stay dependency-free, so request and reply types that name domain types live in astromech-api.

## Unknowns

- Whether `text` and `route` fold into `args` now. Keep both on the wire unless astromech-se44 needs otherwise.

## Close Reason

Implementer report (commit 10774f6d on feature/astromech-bp23). Outcome: crates/astromech-api holds the Registry (name, Exposure Window/Local, handler; add_typed parses args or gives 'bad args for <cmd>'; dispatch runs each handler under catch_unwind and logs a panic; lock() recovers poisoned state) and the control module moved from astromech-core/src/control.rs (server and client together; reply = progress lines then {ok,value}/{ok:false,error}; Request gains origin). Core::commands registers the 54 core commands (quit, open_window local; the 52 window ones equal the shell's deleted ALLOWED list, checked by script). Client timeout is per call: control::call(dir, req, Option<Duration>, on_progress); call_short (10 s) for the instance check, --quit and second start; the shell's call_with has none and stamps origin app_window; the shell's watchdog/core_status status check keeps 10 s (status-style). Connect stays 2 s. Core::handle survives only as a #[cfg(test)] helper that dispatches through the registry, so app.rs tests run the real entries; their struct literals gained origin: None, assertions unchanged.
Proof: cargo test -p astromech-api 7/7 (unknown+bad args, local refuses app_window, panic under shared lock then next call succeeds and log line written, progress before reply, reply after a short client's timeout reaches a no-timeout client; gate opened by the short client's failure). Mutations seen failing: progress not sent (test fails), catch_unwind+poison recovery removed (test fails). cargo test --workspace 1263 passed 0 failed; cargo clippy --workspace --all-targets -D warnings clean; cargo fmt --check clean; check-deps OK and fails when astromech-api gets astromech-policy; check-guide OK; cargo deny OK. End to end on Linux: real astromech-core --background under a temp LOCALAPPDATA, the shell's status/call_with code compiled in a scratch crate against astromech-api: status value, quit from app_window 'not allowed', unknown command error, route_add value; onboarding_smoke.py core_call/wait_core read value and raise on error; raw envelope {ok,value}; second start (instance check off Windows + open_window) logged 'asked it to open the window'; --quit exit 0 and core.json removed.
Clients searched: grep for core.json, control::call and "cmd" in scripts/, .github/, crates/, app/src-tauri/ and app/*.config. Hits beyond the moved clients: webview_cost.py and test_webview_cost.py (mention core.json, use --quit only), astromech-voice proto.rs (its own worker protocol's serde tag), astromech-store paths.rs (doc comment). No other client.
Open items: (1) Windows CI only: the shell's window feature build and clippy, the Release install smoke (PowerShell now checks $status.ok then prints/checks $status.value.status_line) and the onboarding smoke; stand-ins above (stub build, scratch-crate compile of the shell's client functions, Linux run against the real core). (2) A caught handler panic still trips the core's panic hook (crash.rs), which writes the crash marker, so the next start offers Save diagnostics; unchanged from before, worth deciding in a later child. (3) Core handlers still use lock().unwrap(); a panic under one of the core's own mutexes would make later calls to it fail (as errors now, not dropped connections) until restart; poison recovery for moved state is bp23 Background work. (4) The progress test guards a deadlock with a 60 s recv_timeout that only fires on failure. (5) ARCHITECTURE §18.2's policy sentence (unh9 Consequences) not rewritten here; astromech-l2yy item 3 points at the removed control.rs Request doc comment. (6) astromech-api logs through astromech-log directly rather than an injected callback. Review outstanding.
