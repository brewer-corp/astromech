---
created_at: "2026-11-10T06:04:11.267990188+00:00"
id: "astromech-gvy0"
issue_type: "feature"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-irtr"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "The remaining pages move onto the layouts"
updated_at: "2026-11-10T06:04:11.267990188+00:00"
---

## Goal

Every page outside Controls that gets no new screen still stops sizing itself: Stored modules (three columns spread over 1,800 px today), On foot (the same), Places home, Mining, Data sharing, Diagnostics and Voice tester.

## Done when

- [ ] (provisional) Each page named is on a layout component and the type roles, and the page-sizing check is on for its module. Stored modules groups its rows by where the module is; On foot keeps its tables where rows are compared on shared columns.
- [ ] (provisional) With this merged and the screen items done, both parts of the page-sizing check are on for every file under app/src/modules/ except the Controls modules; this item enrols whatever file is still off the list.
- [ ] (provisional) Captures at 1100 x 700 and 2560 x 1440 in docs/design/.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Preliminary: planned in detail after the increment 1 checkpoint.

## Unknowns

Whether Mining stays a page or becomes part of the Now page (astromech-coq6 decides).
