---
created_at: "2026-10-15T20:21:44.859379094+00:00"
id: "astromech-3gkr"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Raw material sites near me"
updated_at: "2026-10-15T20:21:44.859379094+00:00"
---

## Goal

Searchable list of crystalline shard sites, brain trees, crashed ships, geological sites and the classic raw farms, with "nearest to me" and a jump count. The "Crystalline Structures" lookup is here.

## Done when

From a fixed position, the nearest site of each kind is returned with the right distance and jump count; the voice query "nearest crystalline shards" resolves to it.

## Context

`crates/astromech-materials/src/farm.rs` has a hand-picked list without coordinates; add them. Player-visible wiki data is fine to commit (CONTRIBUTING).
