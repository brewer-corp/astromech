---
created_at: "2026-11-05T18:52:09.012627369+00:00"
id: "astromech-eq5k"
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
    id: "astromech-sllo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Suit engineers in one place; tab-row test excludes by role"
updated_at: "2026-11-05T18:52:09.012627369+00:00"
---

## Goal

One home for suit engineers, and a tab-row check that can't be fooled by markup.

## Done when

- The per-engineer mod list from On foot › Engineers (`app/src/modules/odyssey/View.svelte` around line 356) is on the Engineers page's Suit engineers view, and On foot's Engineers view is dropped or becomes a link to `#/unlocks?kind=suit`; old `#/odyssey?view=engineers` links still land (finding astromech-pkd1).
- `app/src/tabrows.test.ts` excludes the Ships rail by what it is (its `nav[aria-label]`) and the Presets grid by its role, not by "has a picture" or "classes differ", so a future tab row with glyphs is still counted (finding astromech-w6gh).
- `pnpm lint && pnpm check && pnpm build && pnpm test` pass; `scripts/ui` shots of On foot and Engineers.

## Context

Deferred findings from the sllo review (reviewer-fable-sllo).
