---
created_at: "2026-10-17T03:16:06.603030108+00:00"
id: "astromech-2rar"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Missions: backfill destinations from earlier journals"
updated_at: "2026-10-17T03:16:06.603030108+00:00"
---

## Goal

Make "Mission target in this system" work for missions accepted in an earlier game session.

## Done when

- After a restart, an FSDJump into the destination of a mission accepted in an earlier journal says "Mission target in this system."
- The destination and target faction for missions in the login `Missions` list are backfilled from earlier journals' `MissionAccepted` events, or from the history store.
- A test covers this.
- From the review of astromech-rpsb, finding 1 (medium).
