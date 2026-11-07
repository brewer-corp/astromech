---
schema: atelier.review
schema_version: 2
id: astromech-80w8
title: finding on astromech-y6d6 @b5af3bc1
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: b5af3bc135de96d0aa8250e4b14f524c941710d7
created_at: 2026-11-07T19:47:19.607552214Z
event:
  event: finding
  recommendation: no_action
---

info: crates/astromech-core/src/update.rs:697-700. The LOCKED_EXES assertions in an_update_over_a_two_process_install_replaces_its_window_with_the_forwarder restate the constant's own literals, so they can't fail independently; the copy_payload half of the test is the real check. No consequence.
