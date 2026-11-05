---
created_at: "2026-11-05T16:36:17.207659045+00:00"
id: "astromech-0k63"
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
    id: "astromech-yxtz"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route page: clipboard failure as a red ! line, off the old option cell"
updated_at: "2026-11-05T16:36:17.207659045+00:00"
---

## Goal

The Route page's clipboard failure reads as DESIGN.md's hard error and is legible.

## Done when

- `app/src/modules/route/View.svelte` (around line 151) shows a clipboard write failure as a red "!" plus words (no copy glyph standing in for the marker), on a surface where the red text meets 4.5:1 rather than `--cell-opt` (3.62:1).
- Best done with astromech-dfvp, which removes `--cell-opt`; a component test covers the failure line; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding astromech-fzd8 from the yxtz review (reviewer-sonnet-yxtz).
