---
created_at: "2026-10-15T20:36:13.655492083+00:00"
id: "astromech-1imw"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:19:22.783997810+00:00"
status: "done"
title: "Remember search filters per lookup"
updated_at: "2026-10-16T19:19:22.783997810+00:00"
---

## Goal

Each search keeps its filters between uses: max pad size from the current ship, exclude fleet carriers, exclude Odyssey settlements, max data age, max arrival distance.

## Done when

Changing a filter, closing the app and reopening it restores the filter for that lookup only; pad size follows a ship change unless the user set it.

## Context

Stored in `astromech-store` settings.

## Close Reason

Filters (pad, fleet carriers, Odyssey settlements, data age, arrival distance) are remembered per lookup in astromech-store settings; the pad follows the ship unless set. Proven by Rust store round-trip tests and a browser-fallback reload check; the real app restart via the shell is not run on Linux. Review outstanding.
