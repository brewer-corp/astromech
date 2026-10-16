---
created_at: "2026-10-16T04:38:23.017887633+00:00"
id: "astromech-uxyq"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P0"
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
title: "Send to EDDN from a live session"
updated_at: "2026-10-16T20:18:55.664619206+00:00"
---

## Goal

Confirm the sender against EDDN from the real game.

## Done when

A session against the `/test` schemas, then a live session, each with no rejections (evidence).

## Notes

- 2026-10-16 [note] Also check in real logs: whether Market/Outfitting/Shipyard.json timestamps always equal their event's, and whether Outfitting.json lists commander-specific (PowerPlay, tech-broker) modules (review of lt2b/thau, R1/R2).
- 2026-10-16 [note] Also from review of w20r/5el6: check NavRoute.json and FCMaterials.json timestamps against their events (navroute and fcmaterials_journal require equality), and run real journal lines through the /test schemas, since every event schema sets additionalProperties false (R2, R4).
- 2026-10-16 [note] Also confirm whether journal timestamps and file names use the PC clock in UTC (the EDDN feed drops live events more than 5 minutes older than the PC clock; review of xvjs N1/N2).
