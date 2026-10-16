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
closed_at: "2026-10-16T19:29:25.280000251+00:00"
status: "done"
title: "EDDN journal/1"
updated_at: "2026-10-16T19:29:25.280000251+00:00"
---

## Goal

Send `journal/1` for `Docked`, `FSDJump`, `Scan`, `Location`, `SAASignalsFound` and `CarrierJump`, with the schema-specific elisions.

## Done when

A fixture of each event produces a message that validates against `journal/1`, with the elided fields absent.

## Notes

- 2026-10-16 [note] Spec read: EDCD/EDDN live 4ad669bb (16 Oct 2026). Every key journal-v1.0.json disallows (incl. Latitude/Longitude/Traits/VoucherAmount and the Factions[] keys) is elided from all six events, not only where the README lists it. An event missing a required key the README doesn't let us add (e.g. Scan without StarSystem) is not sent. Not elided because the README doesn't ask: Taxi/Multicrew/OnFoot in Location/FSDJump/CarrierJump. Wiring (observe, then journal::message, then submit) is astromech-xvjs.

## Close Reason

journal/1 SchemaSpec + trigger rule in astromech-eddn (journal.rs); each event's fixture validates against live journal-v1.0.json with elided fields asserted absent (evidence astromech-hl5i). Independent review pending (orchestrator).
