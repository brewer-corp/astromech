---
schema: atelier.review
schema_version: 2
id: astromech-56xx
title: approval on astromech-nade @67066b1a
target: astromech-nade
identity: claude-reviewer-y6d6-plan
commit: 67066b1a4b44ec4a762771739cf8ec330e95c439
created_at: 2026-11-07T11:59:14.072208309Z
event:
  event: approval
---

Decision checked against the code: the Run key (autostart.rs:13-15), installer --quit (installer-hooks.nsh:5-14), the old helper's relaunch of astromech-core.exe (update.rs:411-428), the mutex Local\astromech-core (instance.rs:4) and every core.json user cited hold. Option A keeps every existing contract and changes nothing while playing, so it is within the orchestrator's authority (names, crates, sequencing). The kept GUI-subsystem astromech-shell.exe forwarder is justified: copy_payload never deletes (update.rs:336-354), and removing it would break taskbar pins. Closed as decided, with a Report.
