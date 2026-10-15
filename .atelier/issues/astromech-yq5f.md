---
created_at: "2026-10-15T20:21:44.874586489+00:00"
id: "astromech-yq5f"
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
status: "todo"
title: "Remember search filters per lookup"
updated_at: "2026-10-15T20:21:44.874586489+00:00"
---

## Goal

Each search keeps its filters between uses: max pad size from the current ship, exclude fleet carriers, exclude Odyssey settlements, max data age, max arrival distance.

## Done when

Changing a filter, closing the app and reopening it restores the filter for that lookup only; pad size follows a ship change unless the user set it.

## Context

Stored in `astromech-store` settings.
