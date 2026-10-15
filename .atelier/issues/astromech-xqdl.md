---
created_at: "2026-10-15T20:36:13.661401872+00:00"
id: "astromech-xqdl"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-05bi"
  - kind: "issue"
    id: "astromech-d1lo"
  - kind: "issue"
    id: "astromech-mcxc"
  - kind: "issue"
    id: "astromech-qw2m"
  - kind: "issue"
    id: "astromech-sd56"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Where to sell or buy a commodity near me"
updated_at: "2026-10-15T20:36:13.661401872+00:00"
---

## Goal

Best sell price within N ly, with demand, pad size and data age; and the lowest buy price, with supply.

## Done when

Recorded-response tests return the best price within the distance and filters, and exclude stations whose demand or supply is below what the user asked for.

## Context

Ardent `system/{s}/commodity/name/{c}/nearby/imports` and `.../exports`.
