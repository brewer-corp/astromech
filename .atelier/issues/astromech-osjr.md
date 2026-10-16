---
created_at: "2026-10-15T20:36:13.650544008+00:00"
id: "astromech-osjr"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-fipx"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T21:55:33.676453444+00:00"
status: "done"
title: "Raw material sites near me"
updated_at: "2026-10-16T21:55:33.676453444+00:00"
---

## Goal

Searchable list of crystalline shard sites, brain trees, crashed ships, geological sites and the classic raw farms, with "nearest to me" and a jump count. The "Crystalline Structures" lookup is here.

## Done when

From a fixed position, the nearest site of each kind is returned with the right distance and jump count; the voice query "nearest crystalline shards" resolves to it.

## Context

`crates/astromech-materials/src/farm.rs` has a hand-picked list without coordinates; add them. Player-visible wiki data is fine to commit (CONTRIBUTING).

## Close Reason

Nearest site of each kind (crystalline shards, brain trees, crashed ships, geological sites, known farms) from fixed positions with distance and jump count; voice 'nearest crystalline shards' resolves to it. Evidence ps8n, gjsv, mtxa, q4tg. Decision astromech-cw8i filed on the Crystalline Structures reading.
