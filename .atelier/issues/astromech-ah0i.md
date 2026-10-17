---
created_at: "2026-10-15T20:36:13.660724118+00:00"
id: "astromech-ah0i"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-ah0i
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-shi5"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T04:58:22.549132434+00:00"
status: "done"
title: "EDSM journal upload and flight log"
updated_at: "2026-10-17T04:58:22.549132434+00:00"
---

## Goal

Upload journal events to EDSM (`api-journal-v1`) with the commander name and API key from Settings, honouring EDSM's discard list, and read the flight log. Keeps the user's EDSM log and map working after EDMC is gone.

## Done when

Fixture events produce requests that exclude everything on the discard list, and a live upload is accepted (evidence).

## Context

The API key goes in the OS keyring, not the config file.

## Close Reason

Done on the fixture proof: fixture events produce EDSM requests that exclude everything on the discard list (evidence recorded at 20f51b4). The live upload accepted by EDSM needs Sumeet's commander and API key and is tracked as astromech-shi5 (needs-human).
