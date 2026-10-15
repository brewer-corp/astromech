---
created_at: "2026-10-15T20:36:13.660572586+00:00"
id: "astromech-5el6"
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
title: "EDDN event schemas"
updated_at: "2026-10-15T20:36:13.660572586+00:00"
---

## Goal

Send `fssdiscoveryscan`, `fssallbodiesfound`, `fssbodysignals`, `fsssignaldiscovered` (batched per system), `navbeaconscan`, `navroute`, `scanbarycentre`, `codexentry`, `approachsettlement`, `dockinggranted`, `dockingdenied`, `fcmaterials_journal` and `blackmarket`.

## Done when

Each schema has a fixture test that validates against the live schema file.

## Context

Schema list read from `EDCD/EDDN` `schemas/` on the `live` branch, 15 Oct 2026 (17 live schemas). Unverified: `blackmarket` is still live.
