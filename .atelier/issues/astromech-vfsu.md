---
created_at: "2026-11-07T13:49:47.872685749+00:00"
id: "astromech-vfsu"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Main's Linux CI fails in the frontend test src/lib/palette.test.ts"
updated_at: "2026-11-07T13:49:47.872685749+00:00"
---

## Goal

The Linux CI job fails on main: `src/lib/palette.test.ts` has 1 failing test of 528 (run 42352408463 on 5aac2e85, and the same assertion on PR #217). A red main hides new failures, so find the cause and make the test pass again or fix the code it catches.

## Done when

- [ ] The cause of the `palette.test.ts` failure is named in the Report.
- [ ] The Linux CI job passes on main.
