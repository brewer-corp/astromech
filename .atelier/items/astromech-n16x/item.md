---
schema: atelier.item
schema_version: 3
id: astromech-n16x
title: 'EDDN tailer: journal order and re-read edge cases'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T20:18:55.228506869Z
---

## Goal

Edge cases in the live-journal rule (pipeline.rs) and outbox: raw file-name comparison sorts legacy 2021 names (`Journal.21…`) after new-format names; new-format local-time names may misorder across a DST change; going back to an already-opened journal replays all of it, losing lines written meanwhile; the 30-minute age limit makes retries 6–8 and the 3600 s backoff cap unreachable.

## Done when

- [ ] One parsed sort key for both journal name formats; per-journal read offsets so lines past them stay live after a bounce; MAX_ATTEMPTS and backoff consistent with the age limit; tests.

## Context

Found in review of astromech-xvjs (round 2, N2–N4). The 5-minute age guard already prevents stale sends.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-tyjz (EDDN sender)
