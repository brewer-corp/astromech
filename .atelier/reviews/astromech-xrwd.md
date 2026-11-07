---
schema: atelier.review
schema_version: 2
id: astromech-xrwd
title: finding on astromech-bp23 @8197eae3
target: astromech-bp23
identity: claude-reviewer-8s6k
commit: 8197eae3aaea51f8468babfeca01b516675004b2
created_at: 2026-11-07T06:52:31.103117677Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/crash.rs:16 The core's panic hook still runs for a handler panic that Registry::dispatch catches, so it writes crash.marker and the next start offers 'astromech crashed last time' though the core kept running. Unchanged from before; decide in a later child (e.g. dispatch marks the thread so the hook only logs, or the marker says the command name).
