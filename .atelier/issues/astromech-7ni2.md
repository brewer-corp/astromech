---
created_at: "2026-10-15T20:36:13.648138543+00:00"
id: "astromech-7ni2"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-7ni2
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-gtza"
  - kind: "issue"
    id: "astromech-huab"
  - kind: "issue"
    id: "astromech-w1q2"
  children:
  - kind: "issue"
    id: "astromech-7ix7"
  - kind: "issue"
    id: "astromech-h1dz"
  - kind: "issue"
    id: "astromech-ny26"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-xvov"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:56:12.326308464+00:00"
status: "done"
title: "Powerplay basics"
updated_at: "2026-10-16T21:01:45.985621615+00:00"
---

## Goal

The user sees their Powerplay standing, hears the system's Powerplay state on arrival (if enabled), and can find nearby systems for their power to work in. This replaces Inara's Powerplay pages for daily use.

## Done when

Journal replay tests for standing and arrival state; recorded-response tests for nearby targets. Independent review. The game check (astromech-w1q2) runs after this merges. Delivers S4.

## Unknowns

Spansh's Powerplay field names for system search.

## Notes

- 2026-10-16 [note] Unknowns resolved: Spansh's Powerplay field names verified live 16 Oct 2026 (see astromech-7ix7). Children ny26, h1dz, 7ix7 done on this branch; astromech-gtza (speak the arrival line; blocked by astromech-cmhv) keeps the parent open.
- 2026-10-16 [note] Correction: hearing the system's Powerplay state on arrival moved to astromech-gtza (standalone, blocked by this parent and astromech-cmhv); this parent delivered the state, summary, spoken-line formatter, app panel and voice query. The earlier note that gtza keeps this parent open is superseded: gtza was moved out so the branch could merge.

## Close Reason

Children done: Powerplay standing with rank, merits and weekly totals (ny26); system Powerplay on arrival as state, summary and spoken-line formatter, app panel and voice query (h1dz); nearby acquire/reinforce/undermine targets via verified Spansh filters (7ix7). Speaking the arrival line is astromech-gtza, which waits on the reaction engine. The game check is astromech-w1q2.
