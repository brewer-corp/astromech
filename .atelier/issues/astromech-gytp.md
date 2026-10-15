---
created_at: "2026-10-15T20:30:24.610122647+00:00"
id: "astromech-gytp"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-rxv2"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-hw3c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDSM journal upload and flight log"
updated_at: "2026-10-15T20:30:24.610122647+00:00"
---

## Goal

Upload journal events to EDSM (`api-journal-v1`) with the commander name and API key from Settings, honouring EDSM's discard list, and read the flight log. Keeps the user's EDSM log and map working after EDMC is gone.

## Done when

Fixture events produce requests that exclude everything on the discard list, and a live upload is accepted (evidence).

## Context

The API key goes in the OS keyring, not the config file.
