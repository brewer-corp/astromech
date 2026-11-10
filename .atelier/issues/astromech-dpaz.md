---
created_at: "2026-11-10T05:55:39.699712044+00:00"
id: "astromech-dpaz"
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
title: "Exploration > System as a system tree"
updated_at: "2026-11-10T05:55:39.699712044+00:00"
---

## Goal

The bodies of the current system read as the system is built: star, planets and moons on a spine with a body icon each and the mapped value as the one main value, the bodies worth mapping lit, and a detail panel for the selected body. It replaces the flat table.

## Done when

- [ ] (provisional) The tree renders the preview system with parents correct; selecting a body fills the panel.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockup: docs/design/glow-up-system.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

BodyView has no parent relation; the core must add each body's parents from the journal's Scan (a core task in this increment, with a refreshed fixture).
