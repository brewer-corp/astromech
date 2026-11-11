---
schema: atelier.item
schema_version: 3
id: astromech-dpaz
title: Exploration > System as a system tree
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
created_at: 2026-11-10T05:55:39.699712044Z
---

## Goal

The bodies of the current system read as the system is built: star, planets and moons on a spine with a body icon each and the mapped value as the one main value, the bodies worth mapping lit, and a detail panel for the selected body. It replaces the flat table.

## Done when

- [ ] (provisional) The tree renders the preview system with parents correct; selecting a body fills the panel.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockup: docs/design/glow-up-system.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

BodyView has no parent relation; the core must add each body's parents from the journal's Scan (a core task in this increment, with a refreshed fixture).
