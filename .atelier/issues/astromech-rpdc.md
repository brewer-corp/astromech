---
created_at: "2026-11-05T17:21:38.789261647+00:00"
id: "astromech-rpdc"
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
    id: "astromech-0k63"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route strip: clipboard failure as the shared Failure line"
updated_at: "2026-11-05T17:21:38.789261647+00:00"
---

## Goal

A failed clipboard write reads the same on the route strip as on the Route page.

## Done when

- `app/src/modules/route/Strip.svelte` (around lines 63, 92 and 117) shows a failed clipboard write as `<Failure inline>`: red "!", the words and Try again that copies again, matching the Route page (finding astromech-rtf9).
- `Strip.test.ts` covers it; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding from the 0k63 review (reviewer-fable-dfvp).
