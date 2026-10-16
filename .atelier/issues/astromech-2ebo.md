---
created_at: "2026-10-16T22:18:23.052846459+00:00"
id: "astromech-2ebo"
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
title: "Cache docked-here state instead of re-reading the journal every 5 s"
updated_at: "2026-10-16T22:18:23.052846459+00:00"
---

## Goal

docked_here re-reads and re-parses the last two journal files on each call, and can briefly disagree with the ship from EngineeringCache.

## Done when

docked_here reads GameState from the cached journal state, and a test or inspection shows one source for ship and station.

## Context

Deferred from the astromech-zig3 review.
