---
schema: atelier.review
schema_version: 2
id: astromech-8t2s
title: finding on astromech-vucf @18b6c603
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 18b6c6030f3b5d477a2ab3ff9d7a0df6b1305674
created_at: 2026-11-07T15:21:03.982287223Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/world.rs:413-416 core.history.stop() runs while the control API still serves: dropping Started only removes core.json, and the listener thread keeps accepting. A set_folders (or any call that runs follow_history) arriving between stop and dir.close() starts a new follower that reopens history.sqlite, bringing back the 'Directory not empty' flake. Nothing in today's tests does that, since callers quit and wait. Fix: stop accepting calls (or make follow a no-op after stop) before stopping the follower. World mode only; the installed path doesn't call Following::stop. Defer to astromech-vucf.
