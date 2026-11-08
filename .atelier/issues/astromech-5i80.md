---
created_at: "2026-11-07T21:35:38.189071110+00:00"
id: "astromech-5i80"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-zgoo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Dump eddn-log.json from a world, or correct why the drift test can't"
updated_at: "2026-11-07T21:35:38.189071110+00:00"
---

## Goal

The preview fixture `eddn-log.json` is the real core's reply, or its `Not` reason in crates/astromech-core/tests/preview_fixtures.rs is true.

## Done when

- [ ] Either a world under `worlds/preview/` produces `eddn-log.json` and the drift test checks it, or the `Not` reason names what still stops a world from settling the EDDN log.

## Context

The reason says the EDDN outbox stamps its entries and schedules its retries on the wall clock (astromech-eddn worker.rs). Since astromech-u8ge and astromech-zgoo the worker and the pipeline stamp on the core's clock (`wall_now` is gone), so that reason no longer holds as written. World mode also switches uploads off, which may be the real reason now.
