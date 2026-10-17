---
created_at: "2026-10-17T03:04:29.982389821+00:00"
id: "astromech-66pq"
issue_type: "task"
labels:
- "needs-human"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check the Powerplay collect and deliver rules in the real game"
updated_at: "2026-10-17T03:04:29.982389821+00:00"
---

## Goal

Confirm in the live game the Powerplay logistics rules in data/catalog/powerplay-logistics.json (astromech-bf25).

## Done when

- Each of these has been checked in game and the data file corrected where it is wrong:
  - supplies for acquisition are collected only from Fortified systems within 20 ly, or Strongholds within 30 ly;
  - reinforcement and undermining supply have no distance limit;
  - undermining data is handed in at any system the power controls;
  - the refcard caveats, which are only warnings so far (no undermining in a stronghold with a carrier; research and industrial data don't count for reinforcement).
- Every `uncertain` flag in the data file is either cleared or kept with a note.
