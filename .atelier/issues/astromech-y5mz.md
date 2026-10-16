---
created_at: "2026-10-15T20:36:13.657732240+00:00"
id: "astromech-y5mz"
issue_type: "spike"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-7ck3"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Build the WebView2 cost harness"
updated_at: "2026-10-16T04:38:23.111550966+00:00"
---

## Goal

A harness that measures the app window's cost: memory idle and in use, frame-time impact on another process, and cold start, against the budgets in spike S11 (< 250 MB idle, no frame-time impact, < 2 s cold start).

## Done when

One command runs the measurements and writes a report against each budget; it runs on a dev machine without Elite (evidence).

## Context

Matters more as the window gets more modules. Spike names aren't record IDs.
