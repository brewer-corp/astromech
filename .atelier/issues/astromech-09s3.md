---
created_at: "2026-10-15T20:36:13.661131337+00:00"
id: "astromech-09s3"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-ah0i"
  - kind: "issue"
    id: "astromech-fq4o"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Settings → Data sharing"
updated_at: "2026-10-15T20:36:13.661131337+00:00"
---

## Goal

Per-service on/off, keys, status (last send, queue depth, last error) and a plain "what gets sent" text.

## Done when

Turning EDDN off stops sends (the outbox stays empty during a replay), and the status row shows the last error from a failed send.
