---
schema: atelier.item
schema_version: 3
id: astromech-azr6
title: 'Trade: show how many fetched markets passed the filters'
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T00:31:20.202543644Z
---

## Goal

Ardent returns a fixed 100 rows for galaxy-wide prices and ignores the pad; the pad (default: the ship's) is applied locally, so a large ship can see a handful of rows or none with no explanation. Prices.truncated is computed before filtering, so it is always true and never shown.

## Done when

- [ ] The Anywhere result footer or empty state says how many of the fetched markets passed the filters (e.g. '12 of 100 pass your pad filter'), truncated means something accurate, and a component test covers it.

## Context

Deferred from the astromech-d1lo review (View.svelte:94-113, trade.rs:299-321, SearchKit.svelte:114).
