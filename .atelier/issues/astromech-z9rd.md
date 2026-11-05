---
created_at: "2026-11-05T09:13:45.620126617+00:00"
id: "astromech-z9rd"
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
    id: "astromech-f7i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Hotspot ores: one list shared by the app and the core, or a test that they agree"
updated_at: "2026-11-05T09:13:45.620126617+00:00"
---

## Goal

The Mining hotspots finder's ore suggestions cannot drift from the ores the core searches for.

## Done when

- `ORES` in app/src/lib/hotspots.ts is generated from, or checked by a test against, the Rust ore list used by the hotspot search and the voice `{mineral}` slot.
- `pnpm test` (and the Rust test if the check lives there) pass.

## Context

Deferred finding astromech-otpp from the f7i0 review.
