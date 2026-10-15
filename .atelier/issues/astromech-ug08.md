---
created_at: "2026-10-15T20:30:24.601279549+00:00"
id: "astromech-ug08"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-fcul"
  - kind: "issue"
    id: "astromech-oprb"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Generalise the \"search near me\" UI kit"
updated_at: "2026-10-15T20:30:24.601279549+00:00"
---

## Goal

Make one search kit every lookup uses: the origin is the current system or any typed system with autocomplete (Spansh `field_values/system_names`); results show distance, jumps at current range, pad size, arrival distance and data age; **Go** sends a result to the route follower.

## Done when

The faction-state lookup runs on the kit with a typed origin and with the current one, and Go puts the result on the route follower (UI check with `scripts/ui`).

## Context

`app/src/lib/ui/Nearby.svelte` and its Go button already do this for faction lookups. Jumps come from `astromech-galaxy` (A* at ship range).
