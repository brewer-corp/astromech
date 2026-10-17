---
created_at: "2026-10-17T16:54:23.347920874+00:00"
id: "astromech-aare"
issue_type: "task"
labels:
- "alpha"
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T18:19:56.378148073+00:00"
status: "done"
title: "Merge task/astromech-3cxd (per-PR CI trim)"
updated_at: "2026-10-17T18:19:56.378148073+00:00"
---

## Goal

Land the per-PR CI trim from `task/astromech-3cxd` (one commit: Release build only on packaging changes, skip docs-only PRs). Its atelier record exists only on that branch.

## Done when

The branch is reviewed and merged to main with its record, and a docs-only PR after it skips the Release build (CI run linked as evidence).

## Close Reason

PR #61 merged into main (dd5e92c) after two independent reviews; see astromech-3cxd.
