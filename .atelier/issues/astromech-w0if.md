---
created_at: "2026-10-22T21:34:32.198775344+00:00"
id: "astromech-w0if"
issue_type: "decision"
labels: []
priority: "P2"
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
status: "todo"
title: "Header: what to say when the core doesn't answer a status read"
updated_at: "2026-10-22T21:34:32.198775344+00:00"
---

## Context

Review finding R3 (astromech-lbth) on feature/astromech-jo3l; App.svelte:104 versus settings/View.svelte:77,111.

## Question

When the window's status read fails, the header says "Not running" while Settings (since astromech-cvxf) keeps the last status with "Could not refresh: the core did not answer." The window can't tell a stopped core from one that didn't answer, so the two disagree about one read. What should the header say?

## Options

- Keep "Not running": simplest; it is the one place that shows whether the core is alive, and a core that doesn't answer is, to the user, not working.
- "Not answering" (or similar) after a failed read, "Not running" only when the window knows the core is stopped (no endpoint file, or its process gone).
- Keep the last status with a stale marker, as Settings does.
