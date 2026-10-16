---
created_at: "2026-10-16T18:12:14.627110771+00:00"
id: "astromech-z4jb"
issue_type: "feature"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-snqt"
  children:
  - kind: "issue"
    id: "astromech-3njr"
  - kind: "issue"
    id: "astromech-fgv5"
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
    id: "astromech-xw9t"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ships page with live build stats"
updated_at: "2026-10-16T18:12:14.750038265+00:00"
---

## Goal

Replace the Builds page, a plain table that is awkward to use, with a Ships page: a dense card per ship, and a detail view whose stats update as the build plan changes. It should answer questions like "will a charge-enhanced power distributor handle this build, or do I need weapon-focused?", "will mass manager or deep charge give me more jump range?" and "how many shield boosters need thermal resistance to reach the level I want?". One ship has one build (its engineering plan), and the stats show current against planned.

## Done when

Every child is done, the in-game check of the three questions (validation child) has passed, and the work has had independent review.

## Scope

Out of scope: several hypothetical builds per ship, and swapping modules for different ones (astromech-xw9t). Copying the game UI 1:1 is also out of scope, because the game's ship screens are sparse and these should be denser.
