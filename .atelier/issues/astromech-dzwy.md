---
created_at: "2026-10-24T23:27:51.919381107+00:00"
id: "astromech-dzwy"
issue_type: "task"
labels:
- "size-m"
priority: "P2"
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
title: "One face: move the window to variable Saira with tabular figures; retire the static cuts and Share Tech Mono"
updated_at: "2026-10-25T19:02:01.607524386+00:00"
---

## Goal

Use DESIGN.md's one face everywhere: Saira variable at 87.5 % width with tabular figures (DESIGN.md, Typography).

## Done when

- `--font` becomes `--face` at `font-stretch: var(--face-stretch)` after a side-by-side check that page metrics do not shift; the static Saira Semi Condensed and Share Tech Mono imports and dependencies are removed (and `@fontsource/michroma` if still unused).
- Every number in a column, tile, cell or strip uses tabular figures and is right-aligned; paths and logs use the same face.
- Screenshots of Settings, Trade, Ships and Diagnostics before and after under `docs/design/`.

## Context

Saira Semi Condensed (fontsource and the upstream static files) has no tabular figures. `@fontsource-variable/saira` (wdth 50-125) has `tnum`; astromech-c4i0 added it to `tokens.css` as `--face` / `--face-stretch`, used by the mockups only. Share Tech Mono (`--mono`) is used in 32 files, for names, dates, prices and prose as well as paths.
