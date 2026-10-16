---
created_at: "2026-10-15T20:36:13.660439728+00:00"
id: "astromech-w20r"
issue_type: "task"
labels:
- "size-s"
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
title: "EDDN journal/1"
updated_at: "2026-10-15T20:36:13.660439728+00:00"
---

## Goal

Send `journal/1` for `Docked`, `FSDJump`, `Scan`, `Location`, `SAASignalsFound` and `CarrierJump`, with the schema-specific elisions.

## Done when

A fixture of each event produces a message that validates against `journal/1`, with the elided fields absent.
