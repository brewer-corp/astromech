---
created_at: "2026-10-17T02:22:53.714986728+00:00"
id: "astromech-1gpo"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Callout shopping list: follow spent materials and departed ships with the app closed"
updated_at: "2026-10-17T02:22:53.714986728+00:00"
---

## Goal

The core's per-ship callout shopping list uses fixed 'until' counts and drops a ship only when the Build page reports it gone. Materials spent by crafting or trading with the app closed drop below 'until' and reappear on the list; a ship sold or stored while the app is closed keeps its list indefinitely.

## Done when

The core recomputes or invalidates a ship's list from journal state (EngineerCraft/MaterialTrade/ship sold or stored), with tests for both cases.

## Context

Deferred from the astromech-77a1 fix review (findings 89pj, 3i3b).
