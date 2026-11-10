---
created_at: "2026-11-10T05:55:41.140290178+00:00"
id: "astromech-8wr4"
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
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Engineering > Ships as a loadout"
updated_at: "2026-11-10T05:55:41.140290178+00:00"
---

## Goal

A ship reads as its loadout: slots grouped as in the game's outfitting (Hardpoints, Utility, Core, Optional), each with class, module, engineering grade pips and blueprint; key stats as a row of hero values; the fleet as a strip. It replaces the stat boxes that are mostly dashes.

## Done when

- [ ] (provisional) Every fitted module of the preview ships appears in its group; selecting one shows what its next grade needs.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockup: docs/design/glow-up-ship.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

How the Info and Engineering sub-views (and the build planner Build.svelte) fold into one screen.
