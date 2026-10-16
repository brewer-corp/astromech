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
status: "todo"
title: "EDDN event schemas"
updated_at: "2026-10-15T20:36:13.660572586+00:00"
---

## Goal

Send `fssdiscoveryscan`, `fssallbodiesfound`, `fssbodysignals`, `fsssignaldiscovered` (batched per system), `navbeaconscan`, `navroute`, `scanbarycentre`, `codexentry`, `approachsettlement`, `dockinggranted`, `dockingdenied` and `fcmaterials_journal`.

## Done when

Each schema has a fixture test that validates against the live schema file.

## Context

Schema list read from `EDCD/EDDN` `schemas/` on the `live` branch, 15 Oct 2026 (17 schema files). `blackmarket` is not sent: its README (live branch, read 16 Oct 2026) marks it deprecated, with the same data in commodity/3's `prohibited` array since September 2017.
