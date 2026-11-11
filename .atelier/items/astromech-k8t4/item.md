---
schema: atelier.item
schema_version: 3
id: astromech-k8t4
title: 'Mining hotspots: an ''Any ring type'' action when a reach-limited answer is empty'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T09:33:13.845723719Z
trivial: null
caused_by: null
created_at: 2026-11-05T09:33:13.845723719Z
---

## Goal

The Mining hotspots finder never shows an empty list without a way forward (DESIGN.md States: Empty is one prose line plus one action).

## Done when

- [ ] - When the request budget runs out and the ring-type filter leaves no rings ("No rings within 32 ly."), the empty line offers ANY RING TYPE, which clears the ring choice and searches again; the finder passes the action into the search kit. - A component test covers it; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding astromech-et0g from the f7i0 fix review (reviewer-sonnet-f7i0): only the ring-type check can empty a reach-limited answer, so clearing it always widens.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-51ue (Galaxy rework: route strip, Places finders, one search box)
