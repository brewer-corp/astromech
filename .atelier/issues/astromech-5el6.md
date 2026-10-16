---
created_at: "2026-10-15T20:36:13.660572586+00:00"
id: "astromech-5el6"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-uxyq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:29:25.488967331+00:00"
status: "done"
title: "EDDN event schemas"
updated_at: "2026-10-16T19:29:25.488967331+00:00"
---

## Goal

Send `fssdiscoveryscan`, `fssallbodiesfound`, `fssbodysignals`, `fsssignaldiscovered` (batched per system), `navbeaconscan`, `navroute`, `scanbarycentre`, `codexentry`, `approachsettlement`, `dockinggranted`, `dockingdenied` and `fcmaterials_journal`.

## Done when

Each schema has a fixture test that validates against the live schema file.

## Context

Schema list read from `EDCD/EDDN` `schemas/` on the `live` branch, 15 Oct 2026 (17 schema files). `blackmarket` is not sent: its README (live branch, read 16 Oct 2026) marks it deprecated, with the same data in commodity/3's `prohibited` array since September 2017.

## Notes

- 2026-10-16 [note] Spec read: EDCD/EDDN live 4ad669bb (16 Oct 2026). fsssignaldiscovered flushes on the next non-FSSSignalDiscovered event (README 'contiguous runs') or an explicit EventSchemas::flush (end of journal/shutdown); the caller must observe each event before routing it so an Odyssey run before FSDJump checks against the new system. navroute/fcmaterials_journal are built from the companion file only when its timestamp equals the announcing event's (stale-file guard, our addition), and an empty Route isn't sent. codexentry replaces any raw BodyName/BodyID with the README's Status.json + ApproachBody/Location cross-check. Events missing a schema-required key aren't sent. cross_check now also checks SystemName and System.

## Close Reason

12 event schemas as SchemaSpecs + EventSchemas router (events.rs) with fsssignaldiscovered batching and codexentry body tracking; each has a fixture test validating against its live schema file (evidence astromech-a1wb). Independent review pending (orchestrator).
