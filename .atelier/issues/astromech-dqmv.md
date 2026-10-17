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
closed_at: "2026-10-17T19:11:49.568521587+00:00"
status: "done"
title: "Neutron plotter"
updated_at: "2026-10-17T19:11:49.568521587+00:00"
---

## Goal

Plot neutron routes from inside the app, with range from the current loadout.

## Done when

A recorded `/api/route` job/poll exchange loads the route into the follower.

## Context

Spansh `/api/route` (job, then poll).

## Close Reason

Neutron routes plot from the Route page with the range from the current loadout; a recorded /api/route job/poll exchange loads into the follower (core plotted_routes_load_into_the_follower, galaxy tests/plot.rs). Reviewed in PR #122.
