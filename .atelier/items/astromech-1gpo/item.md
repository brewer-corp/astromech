---
schema: atelier.item
schema_version: 3
id: astromech-1gpo
title: 'Callout shopping list: follow spent materials and departed ships with the app closed'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T02:22:53.714986728Z
---

## Goal

The core's per-ship callout shopping list uses fixed 'until' counts and drops a ship only when the Build page reports it gone. Materials spent by crafting or trading with the app closed drop below 'until' and reappear on the list; a ship sold or stored while the app is closed keeps its list indefinitely.

## Done when

- [ ] The core recomputes or invalidates a ship's list from journal state (EngineerCraft/MaterialTrade/ship sold or stored), with tests for both cases.

## Context

Deferred from the astromech-77a1 fix review (findings 89pj, 3i3b).
