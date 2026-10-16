---
created_at: "2026-10-15T20:36:13.651807874+00:00"
id: "astromech-pxy2"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-1imw"
  - kind: "issue"
    id: "astromech-2u1x"
  - kind: "issue"
    id: "astromech-4c6q"
  - kind: "issue"
    id: "astromech-7ix7"
  - kind: "issue"
    id: "astromech-osjr"
  - kind: "issue"
    id: "astromech-tw2y"
  - kind: "issue"
    id: "astromech-xqdl"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:13:19.087431117+00:00"
status: "done"
title: "Generalise the \"search near me\" UI kit"
updated_at: "2026-10-16T19:13:19.087431117+00:00"
---

## Goal

Make one search kit every lookup uses: the origin is the current system or any typed system with autocomplete (Spansh `field_values/system_names`); results show distance, jumps at current range, pad size, arrival distance and data age. Sending a result to the route follower with Go is astromech-5akj (the route follower is not on main yet).

## Done when

The faction-state lookup runs on the kit with a typed origin and with the current one (UI check with `scripts/ui`).

## Context

`app/src/lib/ui/Nearby.svelte` already does this for faction lookups. Jumps come from `astromech-galaxy` (A* at ship range).

## Close Reason

The faction-state lookup runs on the new SearchKit with the current system and with a typed origin (Spansh autocomplete); UI-checked in the browser fallback, Rust paths tested on recorded answers. Shell commands are not compiled on Linux. Review outstanding.
