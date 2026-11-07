---
schema: atelier.review
schema_version: 1
id: astromech-t2ca
title: finding on astromech-jdks @f8fc601a
target: astromech-jdks
identity: claude-reviewer-jdks
commit: f8fc601a97c50c538f8fdc4a44d42a163063544d
created_at: 2026-11-07T00:47:12.403925627Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/ARCHITECTURE.md:352 The crate tree (lines ~330-356) lists every crate and now the window as a workspace member, but not the new crates/astromech-mockgame, so ARCHITECTURE doesn't describe the new layout the Done when asks for (CONTRIBUTING does). Fix: add an 'astromech-mockgame/  Windows-only mock of Elite's input and Status.json for the smoke/criteria harnesses (bin mockgame; not shipped)' line. Also §13 line 1174 calls the binary astromech-mockgame.exe; it is mockgame.exe.
