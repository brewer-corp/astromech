---
created_at: "2026-11-05T06:27:50.754459448+00:00"
id: "astromech-cu7d"
issue_type: "validation"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-sgwi"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-wla4"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xlbz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate the linked-places increment on the Linux build"
updated_at: "2026-11-05T06:27:50.754459448+00:00"
---

## Goal

Check the linked-places increment's outcome independently on the Linux build, against astromech-os70's Done when.

## Done when

Each claim in astromech-os70's Done when has an explicit result and a linked evidence receipt (recorded-response test runs and `scripts/ui` screenshots), and the handoff states failures, blocked checks and proof limits.

## Context

Spike astromech-yn6y (budget, recipes), decision astromech-bi94, DESIGN.md Lookup result row and Places home.
