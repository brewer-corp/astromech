---
created_at: "2026-11-07T06:33:38.678821914+00:00"
id: "astromech-k6da"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ndzx"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Drop the neutron star file and scan caches when idle"
updated_at: "2026-11-07T07:43:59.827793607+00:00"
---

## Goal

The neutron star file (about 80 MB loaded) and the command layer's scan caches load on first use and are dropped after a while unused. They now live in the always-on core (decision astromech-6i8s; strategy Risks, core memory).

## Done when

- [ ] `State` has `release_idle(now, idle)`, following `PreviewSlot::release_idle` (crates/astromech-core/src/speech.rs:142). It drops the loaded neutron file (`Kept` gains a way to unload), each scan cache, and the `Galaxy`'s in-memory map when none was used for `idle`. The core's existing reaper thread (app.rs, the `preview-idle` loop) calls it, and the idle time is a named constant (10 minutes unless the implementer finds a reason).
- [ ] A test with an explicit `now` (no sleeps) shows: a plot loads the file; `release_idle` before `idle` keeps it and after `idle` drops it; the next plot loads it again. One test per cache does the same.
- [ ] The core logs one line when it drops the neutron file, as the voice preview does.

## Scope

astromech-api State, astromech-galaxy `neutron::Kept` (and the map's holder in `Galaxy`, if it needs an unload), the app.rs reaper. Memory is reported, not bounded (decision astromech-01ft). No memory test.

## Notes

- 2026-11-07 [note] From finding astromech-zmo7 (ndzx review): Runner::galaxy() (crates/astromech-core/src/actions.rs:87; also assistant.rs:261, :292, :319, :461) opens the star map on the speech thread before 'Looking that up.', which now seeds galaxy.db from the bundled starmap.db on the first voice lookup after an install or update, and waits on the holder's lock while a window lookup opens it. Since this child reworks the holder, take the holder inside the spawned lookup thread (or warm it off the speech thread at session start), so the speech thread stays free as actions.rs:77-78 says.
