---
schema: atelier.item
schema_version: 3
id: astromech-6trg
title: 'Suggestions popover: listbox roles and Tab behaviour'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T06:47:09.497179822Z
trivial: null
caused_by: null
created_at: 2026-11-05T06:47:09.497179822Z
---

## Goal

Autocomplete suggestion lists read correctly to screen readers and keep keyboard focus predictable.

## Done when

- [ ] - In `app/src/lib/ui/Suggestions.svelte` the list items carry no implicit listitem role inside the listbox (e.g. `role="presentation"` on the `li`, or options rendered directly), and options are not separate Tab stops (`tabindex="-1"`, with arrow keys moving the active option via `aria-activedescendant` on the input). - Tab from the input leaves the list closed and moves to the next control; a test covers it. - `pnpm lint && pnpm check && pnpm test` pass.

## Context

Review finding astromech-80jd (reviewer-fable-pey5) on astromech-pey5: pre-existing in all four lists Suggestions replaced; deferred out of pey5.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-pey5 (Shared controls and icons per DESIGN.md: input, dropdown, choice, buttons, badge, icon set)
