---
created_at: "2026-11-05T08:26:02.937067106+00:00"
id: "astromech-rfn8"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-jp36"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Lookup rows: leave out a text column whose cell is the same on every row"
updated_at: "2026-11-05T08:26:02.937067106+00:00"
---

## Goal

Text fact columns keep useful width at 1100 px: a column that says the same thing on every row (e.g. Outfitting's Module once class and rating are chosen) is dropped and its value moves to the constraint summary.

## Done when

- SearchKit omits a text fact column whose cell is identical on every shown row, and names that value in the summary line; a test covers it.
- A `scripts/ui` shot of Outfitting at 1100 px with class and rating chosen shows Faction wider than the ~40 px it gets today.
- `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding astromech-a400 from the jp36 review (reviewer-sonnet-jp36), which decided against fixed widths (they push the facts out) and a width floor (it triggers the compact row).
