---
created_at: "2026-10-15T20:30:24.609859206+00:00"
id: "astromech-elgr"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDDN journal/1"
updated_at: "2026-10-15T20:30:24.609859206+00:00"
---

## Goal

Send `journal/1` for `Docked`, `FSDJump`, `Scan`, `Location`, `SAASignalsFound` and `CarrierJump`, with the schema-specific elisions.

## Done when

A fixture of each event produces a message that validates against `journal/1`, with the elided fields absent.
