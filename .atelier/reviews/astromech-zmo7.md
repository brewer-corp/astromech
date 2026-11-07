---
schema: atelier.review
schema_version: 2
id: astromech-zmo7
title: finding on astromech-bp23 @36ac2e58
target: astromech-bp23
identity: claude-reviewer-ndzx
commit: 36ac2e58e02be587b011992816354b14f2ca84fd
created_at: 2026-11-07T07:42:03.652222838Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/actions.rs:87 (also assistant.rs:261, :292, :319, :461) Runner::galaxy() opens the star map on the speech thread before 'Looking that up.', and the open now seeds galaxy.db from the bundled preload (starmap.db, ~10 MB gz) once per preload build, i.e. on the first lookup after an install or update; it also waits on GalaxyHolder's lock while a window lookup is opening it. Voice commands queue behind that for its duration, though actions.rs:77-78 says the speech thread is free at once. Before, the session opened galaxy.db eagerly at session start with no preload. Fix: take the holder inside the spawned lookup thread (speak NO_STAR_MAP from there), or warm the holder off-thread when the session starts. Defer to astromech-k6da, which reworks the holder for idle release.
