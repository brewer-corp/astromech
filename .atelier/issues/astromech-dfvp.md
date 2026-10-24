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
title: "Cells mean editable: take cell fills off read-only tiles, specs, chips and cards"
updated_at: "2026-10-24T23:27:52.364860453+00:00"
---

## Goal

Make real controls look clickable again by keeping the brown binding cell and grey option cell for editable values only (DESIGN.md, Components: Surfaces and cells).

## Done when

- `--cell` appears only on keybind slots; `--cell-opt` only on inputs, dropdowns, choices and Settings values.
- Read-only tiles use `--surface`; unavailable or zero tiles are dimmed (`--text-dim`) inside a 1 px `--line` outline, per the Materials mockup (`docs/design/design-system-materials-after.png`).
- Engineers cards lose the 4 px orange side border; unlocked state is shown by brightness.
- Screenshots of Materials, Ships, Engineers and Mining before and after.

## Context

The audit found `--cell` / `--cell-opt` on read-only material tiles, ship specs, engineer cards, chips, the Mining tally and summaries (`var(--cell)` is used in 41 files). DESIGN.md puts read-only grouping on `--surface` (hull) or directly on the window.
