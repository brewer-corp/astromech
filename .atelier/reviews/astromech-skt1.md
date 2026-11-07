---
schema: atelier.review
schema_version: 2
id: astromech-skt1
title: finding on astromech-vucf @50e59010
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 50e59010c8f51ecae55b994a55df7e4d1680b986
created_at: 2026-11-07T13:50:05.208365261Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/astromech-d2nq.md Done when 5 and crates/astromech-core/src/world.rs:700-706 d2nq is closed done with Done when 5's Windows half unrun (the branch has no CI run). Read on Windows paths, world mode reaches no real folder: Paths come from a closure that answers only LOCALAPPDATA under the temp dir, every config folder is set, secrets are MemorySecrets, autostart never touches HKCU. But world.dir.close() runs while the core is alive (its detached threads hold Arc<Core> with open SQLite files and the log), so on Windows every run leaks its temp world; the test checks removal only off Windows, which is acceptable for an ephemeral runner. Fix: the Windows CI run of world_mode must pass before vucf merges, and the leak stays recorded as known. Defer to astromech-vucf.
