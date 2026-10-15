---
created_at: "2026-10-15T20:21:44.869510191+00:00"
id: "astromech-thko"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-gasi"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Neutron plotter"
updated_at: "2026-10-15T20:21:44.869510191+00:00"
---

## Goal

Plot neutron routes from inside the app, with range from the current loadout.

## Done when

A recorded `/api/route` job/poll exchange loads the route into the follower.

## Context

Spansh `/api/route` (job, then poll).
