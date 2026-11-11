---
schema: atelier.item
schema_version: 3
id: astromech-uxyq
title: Send to EDDN from a live session
status: ready
mission: astromech-7q0z
labels:
- needs-human
- size-s
- validation
priority: critical
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-16T20:18:55.664619206Z
trivial: null
caused_by: null
created_at: 2026-10-16T04:38:23.017887633Z
---

## Goal

Confirm the sender against EDDN from the real game.

## Done when

- [ ] A session against the `/test` schemas, then a live session, each with no rejections (evidence).

## Notes

- 2026-10-16 [note] Also check in real logs: whether Market/Outfitting/Shipyard.json timestamps always equal their event's, and whether Outfitting.json lists commander-specific (PowerPlay, tech-broker) modules (review of lt2b/thau, R1/R2).
- 2026-10-16 [note] Also from review of w20r/5el6: check NavRoute.json and FCMaterials.json timestamps against their events (navroute and fcmaterials_journal require equality), and run real journal lines through the /test schemas, since every event schema sets additionalProperties false (R2, R4).
- 2026-10-16 [note] Also confirm whether journal timestamps and file names use the PC clock in UTC (the EDDN feed drops live events more than 5 minutes older than the PC clock; review of xvjs N1/N2).
