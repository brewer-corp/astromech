---
created_at: "2026-10-16T19:05:38.928892788+00:00"
id: "astromech-5sbu"
issue_type: "task"
labels:
- "needs-human"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-5akj"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Land the route follower from the route-clipboard branch"
updated_at: "2026-10-17T16:54:21.046646616+00:00"
---

## Goal

The route follower, the Go button in `Nearby.svelte`, `app/src/lib/route.ts` and `crates/astromech-core/src/clipboard.rs` exist only as uncommitted work on Sumeet's `route-clipboard` checkout (with `docs/PLOTTING.md`). Lookups that send a result to the route follower wait on it.

## Done when

The route follower is merged to main through review.

## Context

Sumeet's work in progress; agents must not touch that checkout. Since this was filed, main gained the shared `SystemChip` in `Nearby.svelte` (astromech-vmot) and other `Nearby.svelte`/engineering changes, so expect a merge.
