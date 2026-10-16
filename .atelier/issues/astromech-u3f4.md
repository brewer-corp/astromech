---
created_at: "2026-10-16T19:50:26.140217364+00:00"
id: "astromech-u3f4"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ipb5"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:01:22.423494150+00:00"
status: "done"
title: "History: ledgers follow game time, not import order"
updated_at: "2026-10-16T20:01:22.423494150+00:00"
---

## Goal

Importing an older journal after newer ones (restored backup, logs from another PC) moves a ship's type and last-flown backwards and misorders visits; rebuild_ledgers orders by id and repeats it.

## Done when

Ship updates are guarded by timestamp, last-visit lookups order by ts, and importing an older file afterwards gives the same ledgers as importing in order (test).

## Context

`crates/astromech-history`. Found in review of astromech-aonb (R2).

## Close Reason

Ledgers follow game time: ts-ordered rebuilds and lookups, ship rows guarded by event time, late older events trigger a per-commander rebuild (test: oldest file imported last equals in-order). R4: Docked.system_address used for visits. 8069aac.
