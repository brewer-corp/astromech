---
created_at: "2026-10-16T19:41:46.292368742+00:00"
id: "astromech-0x68"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-k3wp"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Colonisation tracker: wire into the live journal (per-commander keys, load fallback, records table)"
updated_at: "2026-10-17T16:54:22.045016937+00:00"
---

## Goal

Settle three things when the colonisation tracker (astromech-colonisation) is wired into the live journal: (R5) key projects, claims and beacons by commander (ARCHITECTURE section 18.5 cmdr_fid); (R7) decide the caller's fallback when one corrupt record makes Tracker::load fail; (R6) fold the generic records table into the events table design when astromech-aonb lands, or note it in section 18.5.

## Done when

The tracker's state is keyed per commander when fed from the live journal, the load-failure fallback is decided and tested, and the records-table question is resolved or noted in ARCHITECTURE section 18.5.
