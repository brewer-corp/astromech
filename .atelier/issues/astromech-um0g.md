---
created_at: "2026-10-15T20:36:13.664349527+00:00"
id: "astromech-um0g"
issue_type: "task"
labels:
- "size-l"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-leio"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Overlay panels"
updated_at: "2026-10-16T04:33:15.535350401+00:00"
---

## Goal

PTT and confirmation toast, then a kneeboard with prospector results, route, colonisation list and nearest service.

## Done when

Each panel renders from replayed journal data and updates as events arrive, covered by tests.

## Context

Only if the spike is a go.
