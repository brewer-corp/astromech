---
created_at: "2026-10-15T20:21:44.892427908+00:00"
id: "astromech-g9hw"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Trade route finder"
updated_at: "2026-10-15T20:21:44.892427908+00:00"
---

## Goal

Single-hop and loop routes from here, given capacity, credits, pad size and max hop distance, with profit per ton and per hour. Results feed the route follower.

## Done when

A recorded Spansh job/poll exchange produces a route, and Go loads it in the follower.

## Context

Spansh `/api/trade/route` (job, then poll).
