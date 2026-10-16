---
created_at: "2026-10-15T20:36:13.647898425+00:00"
id: "astromech-m30a"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-m30a
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-4su3"
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-huab"
  children:
  - kind: "issue"
    id: "astromech-4c6q"
  - kind: "issue"
    id: "astromech-puas"
  - kind: "issue"
    id: "astromech-tw2y"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Nearby outfitting, ships and services"
updated_at: "2026-10-16T20:16:04.662392596+00:00"
---

## Goal

From the current or a typed system, the user finds the nearest station selling a module (by group, class, rating and mount), a ship, or a service, in the app and by voice. This replaces Inara's nearest-module search for daily use.

## Done when

Lookups against recorded Spansh/Ardent responses, voice phrase tests. Independent review. The game check (astromech-4su3) runs after this merges. Delivers S1.

## Unknowns

The Spansh stations-search request shape for module and ship filters.
