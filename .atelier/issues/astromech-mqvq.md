---
created_at: "2026-10-15T20:21:44.855880037+00:00"
id: "astromech-mqvq"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-11oo"
  - kind: "issue"
    id: "astromech-uh7d"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Construction tracker"
updated_at: "2026-10-15T20:21:44.855880037+00:00"
---

## Goal

Track every depot seen (`ColonisationConstructionDepot`: required, provided, progress, deadline), contributions (`ColonisationContribution`), claims and beacons, persisted per project.

## Done when

Replaying a fixture sequence of depot and contribution events gives the expected progress per project, and survives an app restart.

## Context

Journal only.
