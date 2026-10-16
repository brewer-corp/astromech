---
created_at: "2026-10-15T20:36:13.647281475+00:00"
id: "astromech-ipb5"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-ipb5
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-2myu"
  - kind: "issue"
    id: "astromech-tqdh"
  children:
  - kind: "issue"
    id: "astromech-3tlm"
  - kind: "issue"
    id: "astromech-aonb"
  - kind: "issue"
    id: "astromech-bg0x"
  - kind: "issue"
    id: "astromech-fxp0"
  - kind: "issue"
    id: "astromech-i4gx"
  - kind: "issue"
    id: "astromech-oo8y"
  - kind: "issue"
    id: "astromech-u3f4"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-i4gx"
    type: "related"
  - kind: "issue"
    id: "astromech-u3f4"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal history and logbook"
updated_at: "2026-10-16T20:16:16.822037247+00:00"
---

## Goal

Every journal ever written is imported into a local history, and the logbook, session summary and "been here before" use it. This replaces EDSM's flight log as the user's personal record.

## Done when

Import tests on fixture logs (including a resumed import) and UI checks of the logbook. Independent review. The game check (astromech-tqdh) runs after this merges. Delivers S8, through landings per body.

## Unknowns

How long the import takes on years of logs.

## Design

[ARCHITECTURE §18.5](docs/ARCHITECTURE.md).

## Notes

- 2026-10-16 [note] astromech-2myu (visit history in the arrival briefing) moved out of this parent: it also waits on the arrival briefing (astromech-nugx, another feature), which would hold this branch. It is now standalone, blocked by this parent and nugx.
