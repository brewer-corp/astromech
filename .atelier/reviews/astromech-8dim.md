---
schema: atelier.review
schema_version: 2
id: astromech-8dim
title: finding on astromech-vucf @2348c8e9
target: astromech-vucf
identity: claude-reviewer-i1ev
commit: 2348c8e90388cdd90cf1e69e800cfa34ccafd91d
created_at: 2026-11-07T16:22:24.490737809Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: c4701fb3deb40029c1d7505ac1802924738f53b8
    recorded_at: 2026-11-07T16:23:12.424544500Z
    resolution_commit: c4701fb3deb40029c1d7505ac1802924738f53b8
    reason: Reworded
---

low: .atelier/issues/astromech-i1ev.md Done when 5 (reworded at 2348c8e9), and the commit message. The parenthetical says 'the pipeline treats every line appended while a session runs as live, whatever its stamp'. That holds only for the journal classified as the game's own (Class::Game, pipeline.rs:695-698). A Gated journal's appended lines are judged line by line by line_is_fresh (pipeline.rs:703-717), so a stale one there is not live. The rewording is otherwise honest: it matches the code and decision astromech-e1a5 ('recorded journal lines keep their timestamps and replay as live'), changes no in-game behaviour, and the test checks the silent catch-up it names. Fix: say 'every line appended to the game's own journal (Class::Game)', as the test's doc comment already does. Defer to astromech-vucf (records-only).
