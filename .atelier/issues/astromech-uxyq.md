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
updated_at: "2026-10-16T19:27:26.857232444+00:00"
---

## Goal

Confirm the sender against EDDN from the real game.

## Done when

A session against the `/test` schemas, then a live session, each with no rejections (evidence).

## Notes

- 2026-10-16 [note] Also check in real logs: whether Market/Outfitting/Shipyard.json timestamps always equal their event's, and whether Outfitting.json lists commander-specific (PowerPlay, tech-broker) modules (review of lt2b/thau, R1/R2).
