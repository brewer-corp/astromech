---
created_at: "2026-10-16T19:21:37.805038533+00:00"
id: "astromech-9243"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Shield and DPS headline stats on the Ships cards"
updated_at: "2026-10-16T19:29:46.498113122+00:00"
---

## Goal

Each ship card on the Ships page shows shield strength and DPS next to jump range, from the ship calculations (astromech-ship), so the fleet can be compared at a glance. Split from astromech-3njr, which wired current and planned stats into the ship detail.

## Done when

Every card with a known loadout shows shield strength (MJ) and DPS from `astromech_ship::stats` on its current modules, and jump range from the same calculations; a ship with unresolved modules is marked incomplete; a component test with the fixture fleet (`app/src/lib/fixtures/ships.json`) shows the Krait MkII combat build at 1428 MJ and 107.2 DPS (crates/astromech-ship/tests/coriolis.rs).

## Scope

Current stats only; sustained DPS waits on decision astromech-bnnh.

## Context

The detail asks the shell for one ship at a time (`engineering_stats`). The cards need every ship at once: either a fleet-wide shell command or stats added to the fleet the shell already returns. Keep shell changes thin; `app/src-tauri` builds only on Windows CI.

## Notes

- 2026-10-16 [note] Also (review of 3njr, R4): a hull missing from coriolis-data makes compare return UnknownShip, shown as raw error text; give it the Incomplete treatment on the cards and in the detail.
