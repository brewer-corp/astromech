---
created_at: "2026-10-23T01:17:57.996021169+00:00"
id: "astromech-k1fw"
issue_type: "bug"
labels:
- "size-s"
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
status: "todo"
title: "Galaxy-map route: a NavRouteClear read late can drop a newer plot"
updated_at: "2026-10-23T01:17:57.996021169+00:00"
---

## Goal

Galaxy-map plots and clears reach the route follower in the order the game made them.

## Done when

- A live `NavRoute.json` is handed to the follower when the journal's `NavRoute` line is processed (or a clear is ignored while the route file still has hops), and a test covers clear-then-plot with the journal line read late.

## Context

Review of astromech-g0hm: `NavRoute.json` is read on Status.json changes but `NavRouteClear` when its journal line is read, so a quick clear then re-plot can arrive as plotted(B) then cleared() if journal appends lag (astromech-txzq). Effect: the Route page shows no route while the game has one plotted; galaxy-map routes are silent, so nothing is spoken or copied wrongly.
