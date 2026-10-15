---
created_at: "2026-10-15T20:21:44.899922537+00:00"
id: "astromech-85js"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-2uji"
  - kind: "issue"
    id: "astromech-e84w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Live system view with body values"
updated_at: "2026-10-15T20:21:44.899922537+00:00"
---

## Goal

Bodies as they're scanned (FSS, `Scan`, `SAASignalsFound`): class, terraformable, landable, rings, gravity, estimated value (FSS and mapped, first-discovery bonus) and a "worth mapping" flag.

## Done when

Value tests against published examples for each body class, and a replayed system shows every body.

## Context

Value formulas are community-derived; cite them and unit-test them.
