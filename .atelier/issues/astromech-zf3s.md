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
  - kind: "issue"
    id: "astromech-proz"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:50:11.149453734+00:00"
status: "done"
title: "Trade near me"
updated_at: "2026-10-16T21:04:20.213000838+00:00"
---

## Goal

The user can ask where to sell or buy a commodity near them, and where to sell the cargo they're carrying, in the app and by voice. This replaces Inara and Spansh for the most common trade questions; mined goods are the main case.

## Done when

Lookups against recorded Ardent responses, voice phrase tests. Independent review. The game check (astromech-1per) runs after this merges. Delivers S2.

## Notes

- 2026-10-16 [note] Trade lookups default to market data up to 30 days old (astromech_market::trade::DEFAULT_MAX_AGE_DAYS; the commander can widen it per lookup, up to 365 days). That 30-day default is a judgement call Sumeet may want to change. Review follow-ups R3/R5 filed as astromech-proz.

## Close Reason

Both children done: lookups against recorded Ardent responses and voice phrase tests pass (xqdl at 391bf16, sd56 at b7f8ebf). Done claim for review: independent review and atelier merge are outstanding; the game check astromech-1per runs after the merge.
