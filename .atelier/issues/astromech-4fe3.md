---
created_at: "2026-10-15T20:36:13.653923138+00:00"
id: "astromech-4fe3"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-rk4s"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:11:50.059599705+00:00"
status: "done"
title: "Exact (galaxy) plotter with refuel stops"
updated_at: "2026-10-17T19:11:50.059599705+00:00"
---

## Goal

Plot exact routes with refuel stops, using the ship build from `Loadout` (fuel tank, FSD, mass).

## Done when

A recorded exchange loads a route with refuel stops into the follower.

## Context

Spansh generic route. Unverified: the loadout input format.

## Close Reason

Exact routes plot with refuel stops from the loadout's FSD, tank and mass (input format found and documented in PLOTTING.md); a recorded /api/generic/route exchange loads a route with refuel stops into the follower. Reviewed in PR #122.
