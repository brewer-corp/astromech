---
schema: atelier.item
schema_version: 3
id: astromech-ttq7
title: Narrow host::run's clock-lint allow to its one Instant::now, and point stale shell-proxy comments at host.rs
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T15:37:54.152132500Z
---

## Goal

The clock lint catches a real-time read added to the one app's event loop, and comments no longer describe the removed shell proxy.

## Done when

- [ ] The clock-lint allow in `app/src-tauri/src/host.rs` (around line 368) covers only the `STARTED.get_or_init(Instant::now)` statement, not all of `host::run` (review finding astromech-3rf9).
- [ ] The comments in `app/src/lib/proxy.browser.test.ts:6-7` and `crates/astromech-api/src/lib.rs:10` describe `host.rs`'s in-process `call`, not the removed shell proxy and `window.rs` (review finding astromech-4r9s).
- [ ] `scripts/webview_cost.py` fails when a run that read the tray as `rect` later reads `fallback`, and keeps DEGRADED only for a run that reads `fallback` throughout (review finding astromech-m6f9).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-3rf9
