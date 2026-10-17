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
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "EDSM journal upload and flight log"
updated_at: "2026-10-17T04:40:05.291475500+00:00"
---

## Goal

Upload journal events to EDSM (`api-journal-v1`) with the commander name and API key from Settings, honouring EDSM's discard list, and read the flight log. Keeps the user's EDSM log and map working after EDMC is gone.

## Done when

Fixture events produce requests that exclude everything on the discard list, and a live upload is accepted (evidence).

## Context

The API key goes in the OS keyring, not the config file.
