---
created_at: "2026-11-05T15:11:03.296420676+00:00"
id: "astromech-5s9h"
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
    id: "astromech-kwoi"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Engineering build plan: link to the Faction states finder instead of embedding Nearby"
updated_at: "2026-11-05T15:11:03.296420676+00:00"
---

## Goal

The engineering build plan sends the commander to Places for "where is X", like Materials does (decision astromech-g8d5, item 5), instead of embedding its own lookup list.

## Done when

- `modules/engineering/Build.svelte` (around lines 425 and 451) links into `#/places/faction-states/<query>` with the input filled instead of embedding `<Nearby>`; `Nearby.svelte` is removed if nothing else uses it.
- A component test follows the link; `pnpm lint && pnpm check && pnpm build && pnpm test` pass; a `scripts/ui` screenshot of the build plan.

## Context

Deferred finding astromech-tc9s from the kwoi review.
