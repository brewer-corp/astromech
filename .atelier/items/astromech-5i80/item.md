---
schema: atelier.item
schema_version: 3
id: astromech-5i80
title: Dump eddn-log.json from a world, or correct why the drift test can't
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T21:35:38.189071110Z
---

## Goal

The preview fixture `eddn-log.json` is the real core's reply, or its `Not` reason in crates/astromech-core/tests/preview_fixtures.rs is true.

## Done when

- [ ] Either a world under `worlds/preview/` produces `eddn-log.json` and the drift test checks it, or the `Not` reason names what still stops a world from settling the EDDN log.

## Context

The reason says the EDDN outbox stamps its entries and schedules its retries on the wall clock (astromech-eddn worker.rs). Since astromech-u8ge and astromech-zgoo the worker and the pipeline stamp on the core's clock (`wall_now` is gone), so that reason no longer holds as written. World mode also switches uploads off, which may be the real reason now.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-zgoo (Clippy rejects direct time reads and sleeps; the remaining reads move to the clock)
