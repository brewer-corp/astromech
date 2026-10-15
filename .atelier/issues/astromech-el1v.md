---
created_at: "2026-10-15T20:36:13.659763417+00:00"
id: "astromech-el1v"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-09s3"
  - kind: "issue"
    id: "astromech-5el6"
  - kind: "issue"
    id: "astromech-ah0i"
  - kind: "issue"
    id: "astromech-lt2b"
  - kind: "issue"
    id: "astromech-thau"
  - kind: "issue"
    id: "astromech-w20r"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDDN sender core"
updated_at: "2026-10-15T20:36:13.659763417+00:00"
---

## Goal

Message builder, per-schema elision tables, a persistent outbox (SQLite) with retry and backoff, a `/test` mode, a live-vs-beta gate, and a send log in Diagnostics.

## Done when

Tests show: `*_Localised` keys and commander data are stripped; beta game versions aren't sent; a 400 or 426 isn't retried; other failures retry no sooner than 1 minute; a message over 1 MiB gzipped is dropped and logged.

## Context

EDDN sender rules (EDCD `docs/Developers.md`, read 15 Oct 2026):
- Header: `uploaderID`, `softwareName`, `softwareVersion`. `gameversion`/`gamebuild` from
  `Fileheader` (or `LoadGame`), sent even when empty.
- Strip commander-specific data and every `*_Localised` key.
- `horizons`/`odyssey` from `LoadGame`; never send them as `null`.
- Augment `StarSystem`/`SystemAddress`/`StarPos` only where the schema asks, after
  cross-checking them against the last `Location`/`FSDJump`/`CarrierJump`. Drop the message on
  a mismatch.
- Never send non-live (beta) data. Test against `/test` schemas.
- Retry no sooner than 1 minute; never retry 400 or 426. Bodies at most 1 MiB gzipped.
- "No data is better than bad data."

Write it from the spec, not from EDMC's GPL-2.0 code.
