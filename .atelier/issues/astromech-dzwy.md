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
title: "Figures face: tabular Saira for numbers; Share Tech Mono only for machine text"
updated_at: "2026-10-24T23:27:51.919381107+00:00"
---

## Goal

Numbers in the window line up and match the text face, as DESIGN.md (Typography, The Tabular Rule and The Machine Text Rule) requires.

## Done when

- Decide whether `--font` itself becomes Saira Variable at 87.5 % (one face, one file) after a side-by-side check that page metrics do not shift; if so, drop the static Saira Semi Condensed imports (and `@fontsource/michroma` if still unused).
- Every number in a column, tile, cell or strip uses tabular figures and is right-aligned.
- `var(--mono)` remains only for file paths, raw bind tokens, journal event names, logs and hashes; every other use is replaced.
- Screenshots of Settings, Trade, Ships and Diagnostics before and after under `docs/design/`.

## Context

Saira Semi Condensed (fontsource and the upstream static files) has no tabular figures. The variable Saira (`@fontsource-variable/saira`, wdth 50-125) has `tnum`; DESIGN.md uses it at `font-stretch: 87.5%` as the figures face, and astromech-c4i0 added it to `app/src/design/tokens.css` as `--font-num` / `--num-stretch` for the mockups only. Share Tech Mono (`--mono`) is used in 32 files for names, dates, prices and prose.
