---
schema: atelier.item
schema_version: 3
id: astromech-2rar
title: 'Missions: backfill destinations from earlier journals'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T03:16:06.603030108Z
---

## Goal

Make "Mission target in this system" work for missions accepted in an earlier game session.

## Done when

- [ ] - After a restart, an FSDJump into the destination of a mission accepted in an earlier journal says "Mission target in this system." - The destination and target faction for missions in the login `Missions` list are backfilled from earlier journals' `MissionAccepted` events, or from the history store. - A test covers this. - From the review of astromech-rpsb, finding 1 (medium).
