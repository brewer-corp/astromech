---
created_at: "2026-10-15T20:21:44.890105596+00:00"
id: "astromech-kmd2"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Inara profile sync"
updated_at: "2026-10-15T20:21:44.890105596+00:00"
---

## Goal

Batched events to Inara: location, ship and loadout, ranks, reputation, credits, engineers, materials, missions and CG progress. At most one send per ~35 s; send on start and end, `Docked` and `FSDJump`.

## Done when

Fixture events produce batches within the rate limit, and a live send is accepted (evidence).

## Context

**Blocker:** Inara's author must whitelist the app name. Ask early, since it has lead time. Per-user key in the OS keyring. Inara has no galaxy or market reads for apps.
