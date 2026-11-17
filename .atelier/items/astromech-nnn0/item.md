---
schema: atelier.item
schema_version: 3
id: astromech-nnn0
title: Ships > Suits as a loadout
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.464459239Z
---

## Goal

A suit loadout reads as a ship does: what is fitted, how far each item is engineered, what the plan adds, and what is still missing for it.

## Done when

- [ ] (provisional) Ships > Suits shows the selected loadout's suit and weapons with grade hexagons, modifications and the planned next step; the shopping list with short items marked; the other loadouts as a strip; a side panel for the selected item's next step, as `tone5-suits-2560.png`.
- [ ] (provisional) Built on a layout component and the type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440; the guide page updated.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). Preliminary: planned in detail after the increment 2 checkpoint (astromech-8e22). The page exists from astromech-5a01 with On foot's parts moved in whole.

## Unknowns

Whether the journal gives the current loadout's contents on every start or only when it changes (astromech-t8hg checks which events rewrite the on-foot files).
