---
created_at: "2026-10-15T20:36:13.650677537+00:00"
id: "astromech-n0qb"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-fipx"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show and refresh faction-state freshness"
updated_at: "2026-10-16T21:59:40.523523450+00:00"
---

## Goal

Show when a faction state was seen (Spansh `updated_at`), and refresh it on arrival from our own `FSDJump` faction data.

## Done when

After replaying an FSDJump fixture, the system's state and age come from the journal, not the older Spansh value.

## Context

FSDJump already carries `Factions[].ActiveStates`. Partly built.
