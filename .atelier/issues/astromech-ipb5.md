---
created_at: "2026-10-15T20:36:13.647281475+00:00"
id: "astromech-ipb5"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-2myu"
  - kind: "issue"
    id: "astromech-aonb"
  - kind: "issue"
    id: "astromech-bg0x"
  - kind: "issue"
    id: "astromech-fxp0"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal history and logbook"
updated_at: "2026-10-15T20:36:13.730100607+00:00"
---

## Goal

Every journal ever written is imported into a local history, and the logbook, session summary and "been here before" use it. This replaces EDSM's flight log as the user's personal record.

## Done when

Import tests on fixture logs (including a resumed import), a timing run on Sumeet's real logs recorded as evidence, and UI checks of the logbook. Independent review. Delivers S8, through landings per body.

## Unknowns

How long the import takes on years of logs.

## Design

[ARCHITECTURE §18.5](docs/ARCHITECTURE.md).
