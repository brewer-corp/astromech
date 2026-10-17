---
created_at: "2026-10-17T07:38:18.826673693+00:00"
id: "astromech-3ez9"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-3ez9
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-e8ux"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Voice queries for colonisation progress, what is left and where to buy"
updated_at: "2026-10-17T07:39:53.547560207+00:00"
---

## Goal

The commander can ask by voice for a construction project's progress, what is left to deliver (in loads for the current ship), and where to buy a commodity the project still needs, e.g. "how is the build going", "what is left to deliver", "where can I buy steel for the depot". The tracker, shopping list and where-to-buy lookup exist in the app (astromech-k3wp children 6qk1, i9w7, 05bi) but have no phrase templates, so S5 of the toolbox strategy ("in the app and as a voice query"; ARCHITECTURE §18.7) is not met.

## Done when

Phrase templates for progress, remaining deliveries (loads at the current ship's capacity) and where to buy, answered from the tracker state and the same Ardent lookup the app uses; with several projects, the one the commander is docked at or last delivered to. Tests: phrase-to-command tests, journal replay for the spoken progress and remaining lines, and an answer test against the recorded Ardent responses. Independent review.

## Context

Found by the S1-S8 validation (astromech-e8ux). The plain "where can I buy {commodity}" trade query already works; this item is about the colonisation-specific answers.
