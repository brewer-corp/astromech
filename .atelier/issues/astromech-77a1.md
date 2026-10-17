---
created_at: "2026-10-15T20:36:13.646466235+00:00"
id: "astromech-77a1"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-77a1
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-505u"
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
status: "review"
title: "Arrival, docking and route callouts"
updated_at: "2026-10-17T02:01:23.452515083+00:00"
---

## Goal

On arrival, docking and while following a route, astromech says what the HUD doesn't show: notable states for the user's goals, pad position, the next scoopable star, HGE signals and route hazards. This replaces EDCoPilot's arrival and docking announcements.

## Done when

Journal replay tests per callout. Independent review. The game check (astromech-505u) runs after this merges. Delivers S3, S4, S7.

## Scope

Star class, security and allegiance are on screen, so they're shown in the app, not spoken. Visit history and known biology join the arrival briefing when the history and exobiology work land.

## Notes

- 2026-10-17 [note] All five children done on feature/astromech-77a1 (nugx, lg9r, 6jzn, 2oxr, y916; evidence astromech-knhn, astromech-arti, 2oxr/6jzn records, astromech-21jd). Waiting on independent review of the branch before close and atelier merge; the game check astromech-505u follows the merge. Split out: astromech-8sx3 (bodies known to EDSM).
