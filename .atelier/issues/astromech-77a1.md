---
created_at: "2026-10-15T20:36:13.646466235+00:00"
id: "astromech-77a1"
issue_type: "feature"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  children:
  - kind: "issue"
    id: "astromech-2oxr"
  - kind: "issue"
    id: "astromech-6jzn"
  - kind: "issue"
    id: "astromech-lg9r"
  - kind: "issue"
    id: "astromech-nugx"
  - kind: "issue"
    id: "astromech-y916"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Arrival, docking and route callouts"
updated_at: "2026-10-15T20:36:13.671705072+00:00"
---

## Goal

On arrival, docking and while following a route, astromech says what the HUD doesn't show: notable states for the user's goals, pad position, the next scoopable star, HGE signals and route hazards. This replaces EDCoPilot's arrival and docking announcements.

## Done when

Journal replay tests per callout, and a play session in the real game. Independent review. Delivers S3, S4, S7.

## Scope

Star class, security and allegiance are on screen, so they're shown in the app, not spoken. Visit history and known biology join the arrival briefing when the history and exobiology work land.
