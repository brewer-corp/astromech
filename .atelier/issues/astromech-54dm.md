---
created_at: "2026-10-16T22:18:23.097767574+00:00"
id: "astromech-54dm"
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
status: "backlog"
title: "Gamestate tests for docked Location and FSDJump clearing station info"
updated_at: "2026-10-16T22:18:23.097767574+00:00"
---

## Goal

No test covers Location with Docked:true filling station_info or FSDJump clearing it; landing pads stay blank after logging in docked.

## Done when

Gamestate tests cover both paths, and pads after a docked login are either filled or documented as unknown until Docked.

## Context

Deferred from the astromech-zig3 review.
