---
created_at: "2026-11-05T16:36:17.207659045+00:00"
id: "astromech-0k63"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-0k63
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
closed_at: "2026-11-05T17:45:01.918439013+00:00"
status: "done"
title: "Route page: clipboard failure as a red ! line, off the old option cell"
updated_at: "2026-11-05T17:45:01.918439013+00:00"
---

## Goal

The Route page's clipboard failure reads as DESIGN.md's hard error and is legible.

## Done when

- `app/src/modules/route/View.svelte` (around line 151) shows a clipboard write failure as a red "!" plus words (no copy glyph standing in for the marker), on a surface where the red text meets 4.5:1 rather than `--cell-opt` (3.62:1).
- Best done with astromech-dfvp, which removes `--cell-opt`; a component test covers the failure line; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding astromech-fzd8 from the yxtz review (reviewer-sonnet-yxtz).

## Notes

- 2026-11-05 [note] Done on task/astromech-dfvp with dfvp: the clipboard failure is Failure.svelte's red ! line with Try again (re-copies) on the Route page's Hull card (alarm on --surface 4.81:1); test in route/View.test.ts; before/after docs/design/cells-route-clipboard-*.png.

## Close Reason

Implementer report. The Route page's clipboard failure is Failure.svelte's red "!" plus "Not on the clipboard: <why>" with Try again (copies again), no copy glyph, on the Hull card instead of --cell-opt (red 4.81:1, was 3.62:1). A test in route/View.test.ts covers the line and Try again and fails on the old view; the routes guide is updated; screenshots docs/design/cells-route-clipboard-{before,after}.png. Built on task/astromech-dfvp (c29350a2), reviewed with it by reviewer-fable-dfvp (approval astromech-jn11 on dfvp) and merged to main in de071398. Finding rtf9 (the route strip shows the same failure as an orange line) deferred to astromech-rpdc.
