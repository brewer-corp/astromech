---
created_at: "2026-10-17T19:31:03.063222350+00:00"
id: "astromech-sx84"
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
title: "atelier land fails 'review event history missing' for review records that main got through a merge"
updated_at: "2026-10-17T19:31:03.063222350+00:00"
---

## Goal

`atelier land` and `atelier merge` work on a branch that has defer or no_action dispositions, whatever the history of the older review records it carries from main.

## Done when

`atelier land` on task/astromech-8sx3 or feature/astromech-94j4 (as of 17 Oct 2026) no longer stops with "review event history missing". The fix is upstream in spadaval/atelier, with a test where a review record is introduced in a merge commit.

## Context

In `crates/atelier-app/src/review_gate.rs`, `original_independent` finds a review record's introducing commit with `introduced_query`. That is `git log --root --reverse --diff-filter=A HEAD -- <file>`, without `-m`/`--first-parent`. So a record added on main by a merge commit (for example `.atelier/reviews/astromech-0xky.md`, from merge 9e9f64f "Merge branch 'main' into task/astromech-ah0i") has no introducing commit. The `?` on the defer/no_action path in global mode then fails the whole land. 20 such records exist on main. `atelier land` on main passes, because main has no work owner. Found during astromech-94j4 and astromech-8sx3; the plotter, 8sx3 and dpoc branches are affected. It is not caused by their records.
