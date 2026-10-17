---
created_at: "2026-10-17T19:15:07.381630854+00:00"
id: "astromech-toqc"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Prospector dedupe: unparsable timestamps, reuse open_store in app.rs"
updated_at: "2026-10-17T19:15:07.381630854+00:00"
---

## Goal

Two small leftovers from the prospector dedupe review: (1) when a journal timestamp does not parse, the rock memory uses time 0, so a rock is silenced for good; speak instead or use the wall clock. (2) app.rs prospector_set and shopping_set repeat the create-folder-then-open steps that callouts::open_store now holds; use it.

## Done when

Both are tidied with a test for the unparsable timestamp.

## Context

Review findings 4 and 5 on PR #123 (astromech-u8ir), both low.
