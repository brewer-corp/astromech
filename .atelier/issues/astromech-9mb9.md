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
  blocks:
  - kind: "issue"
    id: "astromech-ww05"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-6yt1"
    type: "related"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-qiey"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:06:48.783645601+00:00"
status: "done"
title: "Open the current ship in Coriolis or EDSY, and link out to Inara/Spansh"
updated_at: "2026-10-16T19:06:48.783645601+00:00"
---

## Goal

Build Coriolis and EDSY import URLs from the current loadout, and Inara/Spansh links for any system or station. Replaces EDMC's shipyard link, and is the cheapest way to take Coriolis off the daily list.

## Done when

A fixture loadout's link opens the same build in Coriolis and in EDSY (checked by hand once, then a URL snapshot test).

## Close Reason

Coriolis and EDSY import links built from the raw Loadout (Rust, round-trip and snapshot tests), opened in headless Chrome in both; Spansh/Inara builders in app/src/lib/links.ts with vitest; buttons on the Builds page. Independent review outstanding.
