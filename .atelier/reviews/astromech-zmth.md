---
schema: atelier.review
schema_version: 2
id: astromech-zmth
title: finding on astromech-vucf @50e59010
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 50e59010c8f51ecae55b994a55df7e4d1680b986
created_at: 2026-11-07T13:50:05.998007973Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/tests/support/mod.rs:866-874, 913-919 children are killed on drop (also on panic) and reaped, but a test binary killed outright (SIGKILL, CI cancel) orphans the core forever, since its stdin is null and it never learns its parent went, and a killed core leaves its temp world. Fix: pipe stdin and have world mode quit on stdin EOF. Defer to astromech-vucf.
