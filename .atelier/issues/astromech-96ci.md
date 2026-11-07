---
created_at: "2026-11-07T06:53:57.601749599+00:00"
id: "astromech-96ci"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ndzx"
    type: "blocked_by"
  - kind: "review"
    id: "astromech-emsi"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T07:55:21.799883323+00:00"
status: "done"
title: "A panic the command layer catches leaves the core working and starts no crash report"
updated_at: "2026-11-07T07:55:21.799883323+00:00"
---

## Goal

A command that panics inside the core answers with an error, and the core then carries on as if nothing happened: its own locks recover, and the next start does not offer a crash report for a panic that never stopped the process.

## Done when

- [ ] The core's handlers and the session threads they share state with lock through `astromech_api::lock` (or the same poison recovery), so a panic under the `session` lock or any other core lock no longer makes later `status`, `voice_test`, exobiology and similar calls fail until restart. A test in crates/astromech-core panics a handler while it holds the session lock, then calls `status` over the control API and gets its normal reply.
- [ ] A panic that the registry catches writes no `crash.marker` (crates/astromech-core/src/crash.rs:16), while a panic outside dispatch still does; a test covers both.
- [ ] astromech-api's doc comment on panics says exactly what recovers.

## Context

Findings astromech-emsi and astromech-xrwd from the astromech-8s6k review. Both behaviours predate the registry (before it, the connection dropped). Do it after astromech-ndzx, so the state that moves into the layer's `State` is already covered there.

## Close Reason

Implementer report (commit 6547b879 on feature/astromech-bp23). Outcome: (1) astromech-api gains Recover, a trait on LockResult whose recover() is the poison recovery lock() uses (lock is now m.lock().recover()). Every .lock().unwrap(), .read().unwrap() and .write().unwrap() in astromech-core's src became .recover() in the files that have them outside tests: app.rs (the Core's session, cfg, ui, game, voice, crash, models, update, config_watch locks), hub.rs (StateHub), callouts.rs and background.rs (the session's shared prospector, shopping and known-bio state), assistant.rs, actions.rs, health.rs (binds), calibration.rs, speech.rs (PreviewSlot), route.rs (the follower desk), tester.rs, history.rs, pipeline.rs, main.rs and tray.rs (Windows only). A mechanical rewrite, so the tests in those files changed too. Left as they were: setup, summary, controls, known_bodies, lookups (unwraps in tests only) and session.rs's Windows-only controls bus, which no handler touches. (2) Registry::dispatch counts the handlers running on its thread (thread-local, nested Call::run included) around catch_unwind. New pub fn astromech_api::dispatching(). The core's panic hook (crash.rs) logs 'panic in a command on thread T (answered with an error): <info>' and writes no crash.marker while it's true, and is unchanged otherwise. (3) astromech-api's crate doc lists what recovers (locks taken with lock/Recover: the api State, the core and session state), what doesn't (a .lock().unwrap() lock, half-done state, a panic on a thread the handler started), and says the hook leaves no crash report. Proof: new app.rs test a_panic_under_the_session_lock_leaves_the_core_working: a test handler added to core.commands() panics holding core.session; over the control API it answers with an error naming the panic, the lock is poisoned, then status answers ok/active false and exobiology answers 'Elite isn't running.'. With app.rs's recover() put back to unwrap() it fails ('status failed with an internal error: ... PoisonError'). New integration test binary tests/crash_marker.rs (its own process because the hook is global): crash::install, then a dispatched handler panic leaves no marker and a named thread's panic leaves one naming the thread and message. With HEAD's crash.rs it fails (marker written for the caught panic). New api test dispatching_is_true_only_while_a_handler_runs (true in the handler and a nested Call::run, false after, false after a panic). cargo test -p astromech-api 33 passed; cargo test -p astromech-core all passed; clippy -D warnings on api and core clean; fmt clean. Open items: (1) tray.rs (Windows only) isn't compiled on this host; it is the same mechanical rewrite with its import, and Windows CI compiles it. (2) Locks in other crates the core calls into (policy, input, voice) still unwrap; not core state, so out of scope. (3) A panic on a thread a handler spawned still writes the marker, as any uncaught panic does. Review outstanding.
