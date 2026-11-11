---
schema: atelier.item
schema_version: 3
id: astromech-4odm
title: 'Find or plot: focus after a row action, live announcements, nested buttons in options'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T15:35:43.032049519Z
trivial: null
caused_by: null
created_at: 2026-11-05T15:35:43.032049519Z
---

## Goal

The header Find or plot box works well with the keyboard and a screen reader.

## Done when

- [ ] - After Add stop or Set destination from a popover row (Tab then Enter), focus returns to the input instead of dropping to the page body (finding astromech-n1xc). - FollowNote and the "Asking Spansh…" and "Nothing matches" lines are announced (`role="status"` or `aria-live="polite"`). - The nested Plot, Add stop and Set destination buttons inside listbox options are resolved: a grid pattern, or actions reached by arrow keys from the active option with only the input in the Tab order (finding astromech-hdj3). Tab from the input no longer walks up to about thirty buttons. - Tests cover focus return and keyboard reach of each action; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred findings from the 2hbw review (reviewer-sonnet-2hbw). Its decision: Add stop and Set destination stay reachable without new chords (Enter means Plot).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-2hbw (Find or plot: one header box for systems, finders and Ctrl+K)
