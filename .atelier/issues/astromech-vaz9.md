---
created_at: "2026-11-05T15:11:03.132592413+00:00"
id: "astromech-vaz9"
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
title: "Faction states finder: check FACTION_STATES against the core's query ids"
updated_at: "2026-11-05T15:11:03.132592413+00:00"
---

## Goal

The Faction states finder and the Materials links cannot drift from the core's faction-state queries.

## Done when

- `FACTION_STATES` in app/src/lib/galaxy.ts is generated from, or checked by a test against, `astromech_galaxy::remote::QUERIES` (ids and order), e.g. a Rust test that writes or compares a fixture the app test reads.
- A new Rust id without an app entry fails a test; `pnpm test` and the Rust test pass.

## Context

Deferred finding astromech-abu3 from the kwoi review (reviewer-fable-kwoi). Same shape as astromech-z9rd (hotspot ores); one mechanism could serve both.
