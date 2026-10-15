---
created_at: "2026-10-15T20:30:24.610809056+00:00"
id: "astromech-pwyr"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-3ufe"
  - kind: "issue"
    id: "astromech-ehhj"
  - kind: "issue"
    id: "astromech-hfjy"
  - kind: "issue"
    id: "astromech-koa0"
  - kind: "issue"
    id: "astromech-mjqs"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Where to sell or buy a commodity near me"
updated_at: "2026-10-15T20:30:24.610809056+00:00"
---

## Goal

Best sell price within N ly, with demand, pad size and data age; and the lowest buy price, with supply.

## Done when

Recorded-response tests return the best price within the distance and filters, and exclude stations whose demand or supply is below what the user asked for.

## Context

Ardent `system/{s}/commodity/name/{c}/nearby/imports` and `.../exports`.
