---
created_at: "2026-11-05T06:19:46.182255093+00:00"
id: "astromech-kyqw"
issue_type: "validation"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-1d92"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-2hbw"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-f7i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-ibmy"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-jp36"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-kwoi"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-rz0f"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-sgwi"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-wla4"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xlbz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate the galaxy rework on the Linux build"
updated_at: "2026-11-05T06:19:46.182255093+00:00"
---

## Goal

Check the galaxy rework's outcome independently on the Linux build, against astromech-51ue's Done when.

## Done when

Each claim in astromech-51ue's Done when has an explicit result and a linked evidence receipt (`scripts/ui` screenshots and the tests run), and the handoff states failures, blocked checks and proof limits. Windows, game and second-monitor checks are out of scope here (astromech-cxgq).

## Context

DESIGN.md (Route strip, Lookup result row, Find or plot, Places home, States), decision astromech-bi94, the `#/mockup/ds-*` mockups.
