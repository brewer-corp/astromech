---
created_at: "2026-10-15T20:36:13.651413158+00:00"
id: "astromech-9uqb"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-j4c9"
  - kind: "issue"
    id: "astromech-kv9k"
  - kind: "issue"
    id: "astromech-qw2m"
  - kind: "issue"
    id: "astromech-zig3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Watch the companion files"
updated_at: "2026-10-15T20:36:13.651413158+00:00"
---

## Goal

Read `Market.json`, `Outfitting.json`, `Shipyard.json`, `ModulesInfo.json`, `ShipLocker.json`, `Backpack.json` and `FCMaterials.json` when their journal event fires, and keep the latest snapshot of each in game state. The EDDN commodity, outfitting and shipyard messages come from these files.

## Done when

Replaying a journal fixture with each event, against fixture companion files, leaves the matching snapshot in game state; a file that's missing or older than its event is reported, not used.

## Context

`Cargo.json` and `NavRoute.json` are already watched. The game rewrites a file around its event, so read after the event, with a short retry.
