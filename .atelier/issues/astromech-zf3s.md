---
created_at: "2026-10-15T20:36:13.648626142+00:00"
id: "astromech-zf3s"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-zf3s
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-1per"
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-huab"
  children:
  - kind: "issue"
    id: "astromech-sd56"
  - kind: "issue"
    id: "astromech-xqdl"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Trade near me"
updated_at: "2026-10-16T20:16:03.804840811+00:00"
---

## Goal

The user can ask where to sell or buy a commodity near them, and where to sell the cargo they're carrying, in the app and by voice. This replaces Inara and Spansh for the most common trade questions; mined goods are the main case.

## Done when

Lookups against recorded Ardent responses, voice phrase tests. Independent review. The game check (astromech-1per) runs after this merges. Delivers S2.
