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
closed_at: "2026-10-16T19:37:20.036674183+00:00"
status: "done"
title: "Construction tracker"
updated_at: "2026-10-16T19:41:42.603185946+00:00"
---

## Goal

Track every depot seen (`ColonisationConstructionDepot`: required, provided, progress, deadline), contributions (`ColonisationContribution`), claims and beacons, persisted per project.

## Done when

Replaying a fixture sequence of depot and contribution events gives the expected progress per project, and survives an app restart.

## Context

Journal only.

## Notes

- 2026-10-16 [note] The journal's ColonisationConstructionDepot has no deadline field (EDDI reads only MarketID, ConstructionProgress, ConstructionComplete, ConstructionFailed, ResourcesRequired[]), so no deadline is tracked.

## Close Reason

astromech-colonisation tracks depots, contributions, claims and beacons per MarketID, persisted in astromech-store; replay and restart tests pass.
