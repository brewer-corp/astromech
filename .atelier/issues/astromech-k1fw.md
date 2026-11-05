---
created_at: "2026-10-23T01:17:57.996021169+00:00"
id: "astromech-k1fw"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-k1fw
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-g0hm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Galaxy-map route: a NavRouteClear read late can drop a newer plot"
updated_at: "2026-11-05T01:02:32.656489479+00:00"
---

## Goal

Galaxy-map plots and clears reach the route follower in the order the game made them.

## Done when

- A live `NavRoute.json` is handed to the follower when the journal's `NavRoute` line is processed (or a clear is ignored while the route file still has hops), and a test covers clear-then-plot with the journal line read late.

## Context

Review of astromech-g0hm: `NavRoute.json` is read on Status.json changes but `NavRouteClear` when its journal line is read, so a quick clear then re-plot can arrive as plotted(B) then cleared() if journal appends lag (astromech-txzq). Effect: the Route page shows no route while the game has one plotted; galaxy-map routes are silent, so nothing is spoken or copied wrongly.

## Notes

- 2026-11-04 [decision] Ignore a clear while the route file still has hops
  Reason: Ignore a live NavRouteClear while the last NavRoute.json read (GameState::route_hops) still has hops. The game rewrites NavRoute.json empty on a clear (docs/REPORT.md), and the pipeline reads the file on Status/NavRoute.json notifications before the journal in the same batch, so hops in the state at the clear line mean a newer plot already reached the follower. One condition in the route_follow rule, no new I/O, no change to when Input::Route fires (the route briefing and fuel rules share it).
  Rejected: Hand NavRoute.json to the follower when the journal's NavRoute line is processed: needs a file read in the journal path, a timestamp check against the event (the file can lag the event, as the EDDN feed's wait shows), and re-sends a plot the follower already has, which rebuilds the galaxy-map route and resets its progress.
