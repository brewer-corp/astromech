---
created_at: "2026-10-15T20:36:13.652294682+00:00"
id: "astromech-0zni"
issue_type: "task"
labels:
- "size-l"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Optional EDDN listener for fresher prices"
updated_at: "2026-10-15T20:36:13.652294682+00:00"
---

## Goal

Subscribe to EDDN (ZeroMQ `tcp://eddn.edcd.io:9500`) and keep a local market cache around the commander's bubble.

## Done when

With the listener on, a price change on EDDN shows in the trade lookup within a minute.

## Context

Only if Ardent's data age turns out to be a problem.
