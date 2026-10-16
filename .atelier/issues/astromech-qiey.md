---
created_at: "2026-10-16T19:13:12.741983582+00:00"
id: "astromech-qiey"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-9mb9"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Spansh and Inara links on current location and stored stations"
updated_at: "2026-10-16T19:13:12.741983582+00:00"
---

## Goal

Offer Spansh and Inara links on the current system and station display and on stored ships' stations (StoredShips carries a market id for spanshStation). Today they appear only for ships parked elsewhere, and spanshStation/inaraStation are unused in the UI. Also test once with a large real-shaped loadout for URL length.

## Done when

Links show on the current system/station and stored ships' stations; a large real-shaped loadout has a URL-length test.
