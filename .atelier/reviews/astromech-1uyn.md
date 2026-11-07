---
schema: atelier.review
schema_version: 2
id: astromech-1uyn
title: finding on astromech-vucf @a43fb01a
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: a43fb01a8e58abe7e80d9805f29557c3ae0182c9
created_at: 2026-11-07T14:01:43.697970854Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/world.rs:387-393 and tests/world_mode.rs:120-134 world mode now quits as soon as its stdin reaches end of file, so a starter that passes stdin as null or ignored (a Node spawn with stdio 'ignore', nohup, a GUI-subsystem release build with no stdin handle) gets a core that quits at once. jkgv's Vite plugin and yis2's driver must pipe stdin and hold it. Separately, the kill test waits for stderr to close with no bound: if a regression stops world mode quitting on EOF, the test hangs until CI's job timeout and leaves the core running, instead of failing. Fix: name the stdin contract in jkgv's Done when; optionally bound the kill test's wait with a hang guard like world_wait's. Defer to astromech-jkgv.
