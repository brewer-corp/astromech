---
created_at: "2026-10-15T20:30:24.598925711+00:00"
id: "astromech-shlr"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-dnh8"
  - kind: "issue"
    id: "astromech-lbv0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Construction tracker"
updated_at: "2026-10-15T20:30:24.598925711+00:00"
---

## Goal

Track every depot seen (`ColonisationConstructionDepot`: required, provided, progress, deadline), contributions (`ColonisationContribution`), claims and beacons, persisted per project.

## Done when

Replaying a fixture sequence of depot and contribution events gives the expected progress per project, and survives an app restart.

## Context

Journal only.
