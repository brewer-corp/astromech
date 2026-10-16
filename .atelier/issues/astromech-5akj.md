---
created_at: "2026-10-16T19:05:38.939338220+00:00"
id: "astromech-5akj"
issue_type: "task"
labels: []
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
status: "todo"
title: "Send a search result to the route follower with Go"
updated_at: "2026-10-16T19:05:38.939338220+00:00"
---

## Goal

In the shared search kit, **Go** sends a result to the route follower, for every lookup that uses the kit.

## Done when

Go on a faction-state result puts that system on the route follower (UI check with `scripts/ui`).

## Context

Split from astromech-pxy2: the route follower is not on main yet.
