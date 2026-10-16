---
created_at: "2026-10-16T18:12:14.934041316+00:00"
id: "astromech-pxeh"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-snqt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Materials: move the detail out of the side bar"
updated_at: "2026-10-16T18:31:41.568936397+00:00"
---

## Goal

The Materials page puts all its useful detail in a narrow right side bar and leaves the bottom half of the page empty. Move the detail into the space below the material grid, at full width.

## Done when

A screenshot at the default window size shows the detail below the grid, with no empty bottom half, and the side bar removed.

## Context

`app/src/modules/materials/View.svelte`.
