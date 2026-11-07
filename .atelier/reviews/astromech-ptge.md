---
schema: atelier.review
schema_version: 2
id: astromech-ptge
title: finding on astromech-vucf @7045a4d2
target: astromech-vucf
identity: claude-reviewer-jkgv
commit: 7045a4d27585ba1223a8c6459d18bfb3d7274cda
created_at: 2026-11-07T20:55:34.955030589Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: ed6eeec2db1985a6d8b5d96a141b66097a3d7c94
    recorded_at: 2026-11-07T21:47:59.557031060Z
    resolution_commit: 3784aa01fb4b03880cca4fd7828d08768964b612
    reason: Taken now; fixed in 3784aa01
---

low: crates/astromech-core/tests/preview_fixtures.rs:640-650 The coverage check compares only top-level keys. Where parts address nested keys (ships.json /headlines/1, /3 and /7 and /cases/N/*, trade.json /anywhere/* and /route/*), a hand-added sibling such as /headlines/9 or /cases/6 is neither compared nor flagged, so 'a hand edit to a dumped fixture fails it' does not hold for additions inside a composite. Today every nested key is covered, so nothing is hidden yet. Fix: check coverage at each level a part's pointer descends through.
