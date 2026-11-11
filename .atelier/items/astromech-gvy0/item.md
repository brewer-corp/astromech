---
schema: atelier.item
schema_version: 3
id: astromech-gvy0
title: The remaining pages move onto the layouts
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-irtr
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T06:04:11.267990188Z
---

## Goal

Every page outside Controls that gets no new screen still stops sizing itself: Stored modules (three columns spread over 1,800 px today), On foot (the same), Places home, Mining, Data sharing, Diagnostics and Voice tester.

## Done when

- [ ] (provisional) Each page named is on a layout component and the type roles, and the page-sizing check is on for its module. Stored modules groups its rows by where the module is; On foot keeps its tables where rows are compared on shared columns.
- [ ] (provisional) With this merged and the screen items done, both parts of the page-sizing check are on for every file under app/src/modules/ except the Controls modules and the two frame parts in the route module (Strip.svelte, PlotBar.svelte); this item enrols whatever file is still off the list.
- [ ] (provisional) Captures at 1100 x 700 and 2560 x 1440 in docs/design/.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Preliminary: planned in detail after the increment 1 checkpoint.

## Unknowns

Whether Mining stays a page or becomes part of the Now page (astromech-coq6 decides).
