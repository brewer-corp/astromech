---
created_at: "2026-10-15T20:36:13.653800299+00:00"
id: "astromech-dqmv"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-kzfj"
  - kind: "issue"
    id: "astromech-rk4s"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Neutron plotter"
updated_at: "2026-10-15T20:36:13.653800299+00:00"
---

## Goal

Plot neutron routes from inside the app, with range from the current loadout.

## Done when

A recorded `/api/route` job/poll exchange loads the route into the follower.

## Context

Spansh `/api/route` (job, then poll).
