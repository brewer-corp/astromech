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
closed_at: "2026-11-07T07:50:58.454118224+00:00"
status: "done"
title: "Drop the neutron star file and scan caches when idle"
updated_at: "2026-11-07T07:50:58.454118224+00:00"
---

## Goal

The neutron star file (about 80 MB loaded) and the command layer's scan caches load on first use and are dropped after a while unused. They now live in the always-on core (decision astromech-6i8s; strategy Risks, core memory).

## Done when

- [x] `State` has `release_idle(now, idle)`, following `PreviewSlot::release_idle` (crates/astromech-core/src/speech.rs:142). It drops the loaded neutron file (`Kept` gains a way to unload), each scan cache, and the `Galaxy`'s in-memory map when none was used for `idle`. The core's existing reaper thread (app.rs, the `preview-idle` loop) calls it, and the idle time is a named constant (10 minutes unless the implementer finds a reason).
- [x] A test with an explicit `now` (no sleeps) shows: a plot loads the file; `release_idle` before `idle` keeps it and after `idle` drops it; the next plot loads it again. One test per cache does the same.
- [x] The core logs one line when it drops the neutron file, as the voice preview does.

## Scope

astromech-api State, astromech-galaxy `neutron::Kept` (and the map's holder in `Galaxy`, if it needs an unload), the app.rs reaper. Memory is reported, not bounded (decision astromech-01ft). No memory test.

## Notes

- 2026-11-07 [note] From finding astromech-zmo7 (ndzx review): Runner::galaxy() (crates/astromech-core/src/actions.rs:87; also assistant.rs:261, :292, :319, :461) opens the star map on the speech thread before 'Looking that up.', which now seeds galaxy.db from the bundled starmap.db on the first voice lookup after an install or update, and waits on the holder's lock while a window lookup opens it. Since this child reworks the holder, take the holder inside the spawned lookup thread (or warm it off the speech thread at session start), so the speech thread stays free as actions.rs:77-78 says.

## Close Reason

Implementer report (commit 6f1b48c8 on feature/astromech-bp23). Outcome: astromech_api::State::release_idle(now, idle) (state.rs) drops what has sat unused for idle: each journal cache (materials, odyssey, powerplay follower, engineering, exploration, unsold, colonisation; ForDir now stamps its last use and has release_idle; unsold became a ForDir keyed by the history file), the star map's in-memory map (GalaxyHolder stamps each get and its release_idle calls the new Galaxy::unload_map; galaxy.db stays open), and the neutron star file (neutron::Kept stamps each get that hands the file out; new Kept::release_idle drops it). The idle time is astromech_api::state::IDLE = 10 minutes. The core's preview-idle reaper (app.rs, every 10 s) calls it. Dropping the neutron file logs one line from State: 'neutron star file: <data>/neutron.bin released (unused for 10 minutes)'. From finding zmo7: voice lookups no longer open the star map on the speech thread: Runner::galaxy() is gone; Runner::look_up(answer) says 'Looking that up.', takes the holder on the spawned thread and speaks NO_STAR_MAP from there if it can't open; nearest_site, targets, faction and hotspot lookups use it, and the colonisation answer takes the holder inside its thread. Deviation (spoken): with no star map a voice lookup now says 'Looking that up.' then the no-star-map line (was the line alone), and when both the position and the star map are missing the position line wins. ARCHITECTURE crate tree (astromech-api State) notes the idle drop. Proof: cargo test -p astromech-api --lib 32 passed, new state tests materials/odyssey/powerplay/engineering/system view/unsold/colonisation _dropped_when_idle (one per cache: load, release_idle(now) keeps, release_idle(now+IDLE) drops, next call loads), the_star_map_is_dropped_when_idle (map Arc kept, then read anew), the_neutron_star_file_is_dropped_when_idle (load, kept, dropped with exactly one log line, not again, reload, dropped again: line count 2). Mutations seen failing: State::release_idle emptied fails all 9; idle_since always true fails the 8 kept-before-idle checks. cargo test -p astromech-galaxy --test plot_route 7 passed, new the_kept_file_is_dropped_when_idle_and_loaded_again_by_the_next_plot (plot_route loads it, kept before IDLE, dropped after, next plot loads it again). cargo test -p astromech-core all pass; site_lookup a_star_map_that_would_not_open_is_tried_again_on_the_next_lookup now expects LookingUp resolving to NO_STAR_MAP, 'Looking that up.' then the line, and the open running on a thread other than the caller's; with HEAD's actions.rs/assistant.rs it fails ('not a lookup'). Clippy -D warnings on api, galaxy, core clean; fmt clean. All times in tests are an explicit now (one Instant::now() base after the use, plus IDLE); no sleeps. Open items: (1) the reaper's 10 s loop itself and the live log line are not exercised by a test (production wiring, inspection only). (2) The keybinds Api in State is not dropped (not a scan cache). (3) Memory is reported, not bounded (astromech-01ft); no memory measurement. Review outstanding.
