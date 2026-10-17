---
created_at: "2026-10-15T20:36:13.661815904+00:00"
id: "astromech-mljg"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-mljg
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T23:36:22.305688530+00:00"
status: "done"
title: "Trade route finder"
updated_at: "2026-10-16T23:36:22.305688530+00:00"
---

## Goal

Single-hop and loop routes from here, given capacity, credits, pad size and max hop distance, with profit per ton and per hour, on the Trade page.

## Done when

A recorded Spansh job/poll exchange produces a route, and the Trade page shows it (component test and a UI check with `scripts/ui`).

## Context

Spansh `/api/trade/route` (job, then poll). Sending a route to the route follower is astromech-5akj (Go for every search-kit result), which waits on the follower landing (astromech-5sbu).

## Close Reason

Route mode on the Trade page: single hop from Spansh's trade router (job polled off the UI thread, 3-minute timeout) and loops with the way back from Ardent markets; profit per ton per leg and an estimated profit per hour (Spansh gives none). Recorded job/poll replay tests, component tests and a scripts/ui check. Sending a route to the follower stays with astromech-5akj.
