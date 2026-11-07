---
schema: atelier.review
schema_version: 2
id: astromech-lwic
title: finding on astromech-vucf @4744ece0
target: astromech-vucf
identity: claude-reviewer-1hh9
commit: 4744ece07c9a33b4183c5178bf79bbd8e9b5c504
created_at: 2026-11-07T18:48:12.335663076Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: 64a5cbb048106097b3d086e2555906a9b73850cf
    recorded_at: 2026-11-07T18:49:03.976021877Z
    follow_up: astromech-jkgv
    reason: Noted on jkgv
---

low: crates/astromech-core/src/world/io.rs:142-170 and world/steps.rs:85-89 (untested paths). The focus step and the gui and vehicle reaction effects are covered only by the script's load test (script.rs, the_devices_header_and_steps_load). No world has Elite lose focus and checks that the key is refused, and none toggles GuiFocus or swaps the vehicle through a key. The paste world sets GuiFocus with a file step instead. These are new behaviour with no proof that they run. Suggested follow-up: a focus-loss case in an existing voice world (focus Notepad, then a voice command: no key observed), plus one gui reaction. That fits under jkgv or a later vucf child.
