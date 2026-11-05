---
created_at: "2026-11-05T09:33:13.845723719+00:00"
id: "astromech-k8t4"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "astromech-et0g"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Mining hotspots: an 'Any ring type' action when a reach-limited answer is empty"
updated_at: "2026-11-05T09:33:13.845723719+00:00"
---

## Goal

The Mining hotspots finder never shows an empty list without a way forward (DESIGN.md States: Empty is one prose line plus one action).

## Done when

- When the request budget runs out and the ring-type filter leaves no rings ("No rings within 32 ly."), the empty line offers ANY RING TYPE, which clears the ring choice and searches again; the finder passes the action into the search kit.
- A component test covers it; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding astromech-et0g from the f7i0 fix review (reviewer-sonnet-f7i0): only the ring-type check can empty a reach-limited answer, so clearing it always widens.
