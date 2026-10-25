---
created_at: "2026-10-24T23:27:52.364860453+00:00"
id: "astromech-dfvp"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Cells mean editable: ember cells for every editable value, hull for read-only things"
updated_at: "2026-10-25T19:02:01.158865111+00:00"
---

## Goal

Make real controls look clickable by keeping the ember (brown) cell for editable values only and putting read-only things on the darker hull or the window (DESIGN.md, Components: Surfaces and cells).

## Done when

- `--cell` appears only on inputs, dropdowns, choices, binding slots and Settings values; `--cell-opt` is gone.
- Read-only tiles use `--surface`; zero or unavailable tiles are Void inside a 1 px `--line` outline with `--orange-dim` text, and the selected tile is a solid orange fill, per `docs/design/design-system-materials-after.png`.
- Engineers cards lose the 4 px orange side border; unlocked state is shown by brightness.
- Screenshots of Materials, Ships, Engineers and Mining before and after.

## Context

The audit found `--cell` / `--cell-opt` on read-only material tiles, ship specs, engineer cards, chips, the Mining tally and summaries (`var(--cell)` in 41 files). DESIGN.md (revised 25 Oct) makes every editable value an ember cell (`--cell`, including what used the grey `--cell-opt`) and read-only grouping `--surface` (hull, `#1a1310`).
