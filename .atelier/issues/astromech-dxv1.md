---
created_at: "2026-10-17T22:52:51.528292386+00:00"
id: "astromech-dxv1"
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
status: "todo"
title: "EDSM: three or more poisoned events due in one pass still stop it; test the reset on stored"
updated_at: "2026-10-22T20:41:37.408800036+00:00"
---

## Goal

After astromech-ix2p, the outage guard stops a pass after three single failures in a row, so three or more poisoned events due together still hold the rest of that pass. Also, removing the 'reset on stored' line breaks no test (re-review R1).

## Done when

A test with three poisoned events separated by good ones, all due in one pass, completes the pass (and fails without the reset); and either the three-in-a-row limit is lifted without letting a sustained outage burn the queue, or the limit is documented as accepted for best-effort upload.
