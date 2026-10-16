---
created_at: "2026-10-15T20:36:13.646672720+00:00"
id: "astromech-k3wp"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-k3wp
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-huab"
  - kind: "issue"
    id: "astromech-xf8j"
  children:
  - kind: "issue"
    id: "astromech-05bi"
  - kind: "issue"
    id: "astromech-6qk1"
  - kind: "issue"
    id: "astromech-i9w7"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-0x68"
    type: "related"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Colonisation tracker"
updated_at: "2026-10-16T19:32:22.606106233+00:00"
---

## Goal

The user sees every construction project's progress, what's left to deliver in loads for the current ship, and where to buy each commodity. This replaces Raven Colonial's Architect Tracker and EDCoPilot's tracker.

## Done when

Journal replay tests for the tracker and shopping list; recorded-response tests for where to buy. Independent review. The game check (astromech-xf8j) runs after this merges. Delivers S5.
