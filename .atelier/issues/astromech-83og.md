---
created_at: "2026-10-17T19:07:41.448444054+00:00"
id: "astromech-83og"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route plotter: refresh the ship and range line after a swap or cargo change"
updated_at: "2026-10-22T20:41:36.092917981+00:00"
---

## Goal

The range line above Plot and follow on the Route page shows the ship and cargo a plot would use now, and the result says the range it was plotted with.

## Done when

The page refetches `route_plot_ship` when Plot and follow is pressed (or periodically) and the "Following ..." summary includes `plotted.range`; a component test covers a changed range.

## Context

Review finding R4 on PR #122 (astromech-94j4). `route_plot` recomputes the ship and cargo when plotting, so the plotted route is right; only the line shown before plotting can be stale.
