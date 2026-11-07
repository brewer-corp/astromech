---
schema: atelier.review
schema_version: 2
id: astromech-klw0
title: finding on astromech-bp23 @e0fac0ea
target: astromech-bp23
identity: claude-reviewer-i8vr
commit: e0fac0ea40744ef51d4ea921bc10933300547cd8
created_at: 2026-11-07T09:47:38.183761736Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/astromech-api/src/control.rs:190-202 (call_window) with app/src/App.svelte:97-99 and app/src/lib/core.ts:163 (i8vr) The window's status poll now has no read timeout. For a core that accepts but never answers, the header keeps the last good status (e.g. running) forever instead of 'not running' after 10 s, and the watchdog doesn't rescue it on Windows: start_core's new core finds the Local\astromech-core mutex held (instance.rs:21, app.rs:1498), so it exits. Each 2 s tick (App, plus Diagnostics, Settings, Logbook, Mining) also parks a spawn_blocking thread and a socket for good, and after Tokio's 512-thread blocking cap every window call queues behind them. This changes what the app shows while playing (strategy Authority) with no decision. Cheap restore: in call_window use Some(SHORT) when cmd == "status" (8s6k: status-style callers keep a bounded timeout), and reword i8vr's first Done when line to say so. That's simpler than a Promise.race in coreStatus, which would leave the thread parked.
