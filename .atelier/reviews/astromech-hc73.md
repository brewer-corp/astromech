---
schema: atelier.review
schema_version: 1
id: astromech-hc73
title: 'Low, pre-existing in both parents, not introduced by merge 9c7644e: when an older or already-read journal becomes newest mid-session, Tailer::open (crates/astromech-core/src/pipeline.rs:252-258) re-reads it with live=false. The callout engine now stays silent for those lines (correct), but Engine::journal (crates/astromech-announce/src/lib.rs:245-249) still updates its docking mode from them. If that journal ends between DockingGranted and a docking-end event (the game quit on approach), docking quiet stays on and non-urgent speech is muted until the next Docked/DockingCancelled. The hub''s GameState is rewound the same way by main''s replay design. Suggested later fix: for a mid-session replay (caught_up && !live), snapshot and restore the engine''s mode, or skip the replay for anything except the EDDN feed.'
target: astromech-cmhv
identity: review-agent (cmhv merge review)
commit: ba665fd1a287b479b2fc19ae66b2d3f69cffdecb
created_at: 2026-10-16T22:35:02.010535284Z
event:
  event: finding
  recommendation: defer
---

Low, pre-existing in both parents, not introduced by merge 9c7644e: when an older or already-read journal becomes newest mid-session, Tailer::open (crates/astromech-core/src/pipeline.rs:252-258) re-reads it with live=false. The callout engine now stays silent for those lines (correct), but Engine::journal (crates/astromech-announce/src/lib.rs:245-249) still updates its docking mode from them. If that journal ends between DockingGranted and a docking-end event (the game quit on approach), docking quiet stays on and non-urgent speech is muted until the next Docked/DockingCancelled. The hub's GameState is rewound the same way by main's replay design. Suggested later fix: for a mid-session replay (caught_up && !live), snapshot and restore the engine's mode, or skip the replay for anything except the EDDN feed.
