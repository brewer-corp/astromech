---
created_at: "2026-10-16T04:38:23.042533164+00:00"
id: "astromech-tqdh"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Time the logbook import on the real logs"
updated_at: "2026-10-17T21:32:14.420538427+00:00"
---

## Goal

Confirm the import is fast enough on a real journal folder.

## Done when

A full import of Sumeet's journal folder is timed and the result recorded as evidence.

## Notes

- 2026-10-16 [note] Also check in the real logs whether Odyssey taxi/dropship and multicrew Loadouts add ships to the history ships ledger (review of aonb, R8), and whether a second commander or Legacy journals are present (R1 urgency).
- 2026-10-17 [note] From the yin0 replay of the real logs: one commander, no Legacy journals (R1 not urgent on this data). No taxi or dropship Loadouts (3 BookTaxi, the Loadout events are all owned ships) and no multicrew, so R8 didn't arise; LoadGame lines naming a suit (on foot) or an SRV were not added to the ships ledger. The ships ledger has a different problem: the game reuses a traded-in ship's id for the new ship (astromech-vett).
