---
created_at: "2026-10-15T20:36:13.649455258+00:00"
id: "astromech-6qk1"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-i9w7"
  - kind: "issue"
    id: "astromech-kv9k"
  - kind: "issue"
    id: "astromech-xf8j"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Construction tracker"
updated_at: "2026-10-16T19:32:22.606107946+00:00"
---

## Goal

Track every depot seen (`ColonisationConstructionDepot`: required, provided, progress, deadline), contributions (`ColonisationContribution`), claims and beacons, persisted per project.

## Done when

Replaying a fixture sequence of depot and contribution events gives the expected progress per project, and survives an app restart.

## Context

Journal only.
