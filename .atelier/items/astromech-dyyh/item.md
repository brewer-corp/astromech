---
schema: atelier.item
schema_version: 3
id: astromech-dyyh
title: 'Preview: every Outfitting search answers with the recorded 5A Frame Shift Drive stations'
status: draft
mission: null
labels:
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T15:08:59.739510266Z
---

## Goal

The browser preview's Outfitting finder shows a search for one module answered with another, which misleads anyone reading a capture of it. Searched for Pulse Laser 1F Fixed, it lists stations selling a 5A Frame Shift Drive and its side panel says "Module 5A Frame Shift Drive" (`docs/design/final/outfitting-selected-2560x1440.png`). The fake answers every module search with the one recorded reply, `app/src/preview/fixtures/stations-module.json`, at `app/src/preview/shell.ts:187`. Found by astromech-crxa's glance-test reader on 17 Nov 2026. The real app asks Spansh for the module searched, so only the preview is wrong.

## Done when

- [ ] The preview's Outfitting finder lists only stations selling the module searched for: a recorded reply for the module the captures search (`docs/design/final/plan.json` searches Pulse Laser 1F Fixed), or an empty result for a module it has no recording of, dumped as CONTRIBUTING.md says for preview fixtures.
- [ ] A browser test on the preview shows a search for a module with no recording does not list another module's stations.

## Scope

`app/src/preview/` and its fixtures. No app code.
