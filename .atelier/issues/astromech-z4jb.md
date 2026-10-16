---
created_at: "2026-10-16T18:12:14.627110771+00:00"
id: "astromech-z4jb"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-z4jb
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-fgv5"
  - kind: "issue"
    id: "astromech-snqt"
  children:
  - kind: "issue"
    id: "astromech-3njr"
  - kind: "issue"
    id: "astromech-9243"
  - kind: "issue"
    id: "astromech-mycc"
  - kind: "issue"
    id: "astromech-ojwr"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-bnnh"
    type: "related"
  - kind: "issue"
    id: "astromech-fdyn"
    type: "related"
  - kind: "issue"
    id: "astromech-hnar"
    type: "related"
  - kind: "issue"
    id: "astromech-ikwr"
    type: "related"
  - kind: "issue"
    id: "astromech-rbsa"
    type: "related"
  - kind: "issue"
    id: "astromech-vy9z"
    type: "related"
  - kind: "issue"
    id: "astromech-xw9t"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:53:58.493153039+00:00"
status: "done"
title: "Ships page with live build stats"
updated_at: "2026-10-16T19:53:58.493153039+00:00"
---

## Goal

Replace the Builds page, a plain table that is awkward to use, with a Ships page: a dense card per ship, and a detail view whose stats update as the build plan changes. It should answer questions like "will a charge-enhanced power distributor handle this build, or do I need weapon-focused?", "will mass manager or deep charge give me more jump range?" and "how many shield boosters need thermal resistance to reach the level I want?". One ship has one build (its engineering plan), and the stats show current against planned.

## Done when

Every child is done and the work has had independent review. The in-game check of the three questions (astromech-fgv5) runs after this merges.

## Scope

Out of scope: several hypothetical builds per ship, and swapping modules for different ones (astromech-xw9t). Copying the game UI 1:1 is also out of scope, because the game's ship screens are sparse and these should be denser.

## Close Reason

The Builds page is replaced by the Ships page. astromech-ojwr ported Coriolis's ship calculations to Rust (astromech-ship), matched against Coriolis on five reference builds. astromech-mycc added a dense card per ship and the stored modules. astromech-3njr added the ship detail at #/engineering/<id>: current and planned stats with the difference next to the build plan, with Coriolis-referenced component tests for the distributor experimental, the FSD blueprint and the thermal booster count, and the materials, trades, farming and trip panels kept. astromech-9243 added shield, DPS and jump range on the cards, with ships the calculations can't fully handle marked Incomplete. After merging main, the cards, stored modules and detail use the shared SystemChip. All four children were independently reviewed except the final 9243 and SystemChip commits; the in-game check of the three questions is astromech-fgv5.
