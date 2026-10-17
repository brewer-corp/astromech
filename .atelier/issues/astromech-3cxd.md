---
created_at: "2026-10-17T03:29:33.293913416+00:00"
id: "astromech-3cxd"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Trim per-PR CI: Release build only when packaging changes, skip docs-only, fewer pushes"
updated_at: "2026-10-17T03:29:33.293913416+00:00"
---

## Goal

Cut the time and churn of PR checks without losing the Windows signal that catches real failures.

## Done when

- Release build runs on PRs only when packaging inputs change; still on tags and manual runs.
- CI skips docs-only and atelier-record-only changes.
- AGENTS.md tells agents to push when ready for review (not every rebase) and to wait on checks with watch_pull_request instead of polling.

## Context

During the 2026-10-17 mission every push ran CI (~5 min) and Release build (~9 min, the merge gate). PRs averaged ~5 pushes (rebases), 71 of 186 PR runs were cancelled mid-run, and Release build never failed alone: both its failures duplicated the CI Windows app-shell build. The orchestrator polled gh pr checks in loops.
