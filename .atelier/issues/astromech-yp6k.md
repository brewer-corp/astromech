---
created_at: "2026-10-15T20:30:24.613903646+00:00"
id: "astromech-yp6k"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-4vb2"
  - kind: "issue"
    id: "astromech-e0iv"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Live system view with body values"
updated_at: "2026-10-15T20:30:24.613903646+00:00"
---

## Goal

Bodies as they're scanned (FSS, `Scan`, `SAASignalsFound`): class, terraformable, landable, rings, gravity, estimated value (FSS and mapped, first-discovery bonus) and a "worth mapping" flag.

## Done when

Value tests against published examples for each body class, and a replayed system shows every body.

## Context

Value formulas are community-derived; cite them and unit-test them.
