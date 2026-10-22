---
created_at: "2026-10-17T23:32:41.837436940+00:00"
id: "astromech-zkz8"
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
status: "todo"
title: "Stale-line tests: assert the EDSM line alone and first-read failures on the production path"
updated_at: "2026-10-22T20:41:35.554680220+00:00"
---

## Goal

The data-sharing tests prove the EDSM stale line and the first-read failures through the app's real loaders.

## Done when

- staleTauri.test.ts asserts the EDSM section's own Could not refresh line, not any one on the page.
- First-read failures (rejected, and core_call ok false) have production-path tests for Ships, Materials, Data sharing, EDSM and Mining.

## Context

Review of astromech-spxx.
