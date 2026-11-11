---
schema: atelier.item
schema_version: 3
id: astromech-rpdc
title: 'Route strip: clipboard failure as the shared Failure line'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T17:21:38.789261647Z
trivial: null
caused_by: null
created_at: 2026-11-05T17:21:38.789261647Z
---

## Goal

A failed clipboard write reads the same on the route strip as on the Route page.

## Done when

- [ ] - `app/src/modules/route/Strip.svelte` (around lines 63, 92 and 117) shows a failed clipboard write as `<Failure inline>`: red "!", the words and Try again that copies again, matching the Route page (finding astromech-rtf9). - `Strip.test.ts` covers it; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding from the 0k63 review (reviewer-fable-dfvp).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-0k63 (Route page: clipboard failure as a red ! line, off the old option cell)
