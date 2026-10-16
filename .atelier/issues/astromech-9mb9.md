---
created_at: "2026-10-15T20:36:13.660048378+00:00"
id: "astromech-9mb9"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-9mb9
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Open the current ship in Coriolis or EDSY, and link out to Inara/Spansh"
updated_at: "2026-10-16T18:55:36.769082024+00:00"
---

## Goal

Build Coriolis and EDSY import URLs from the current loadout, and Inara/Spansh links for any system or station. Replaces EDMC's shipyard link, and is the cheapest way to take Coriolis off the daily list.

## Done when

A fixture loadout's link opens the same build in Coriolis and in EDSY (checked by hand once, then a URL snapshot test).
