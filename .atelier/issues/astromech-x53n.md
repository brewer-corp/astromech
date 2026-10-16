---
created_at: "2026-10-15T20:36:13.655103980+00:00"
id: "astromech-x53n"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Find a station with every module a build still needs"
updated_at: "2026-10-15T20:36:13.655103980+00:00"
---

## Goal

From a Builds plan, list stations ranked by how many of the still-needed modules they sell, then by distance (Inara's "all in one station").

## Done when

For a plan needing three modules and recorded responses, a station selling all three ranks above a closer one selling two.

## Context

Cross-link with the engineering module's Builds plans.
